use serde::{de::DeserializeSeed};
use twilight_model::gateway::{event::{GatewayEvent, GatewayEventDeserializer}, payload::outgoing::Heartbeat as HeartbeatPayload};
use worker::{Date, DateInit::Millis, DurableObject, Env, Request, Response, Result, Router, ScheduledTime, State, Storage, WebSocket, WebSocketIncomingMessage, WebsocketEvent, durable_object, wasm_bindgen::prelude::wasm_bindgen, wasm_bindgen_futures};

use crate::{error::Error, gateway::{dispatcher::Dispatcher, queries::{delete_session, get_heartbeat_ack, get_heartbeat_interval, get_last_sequence, save_heartbeat_interval, save_last_sequence, set_heartbeat_ack}}};

#[durable_object]
pub struct Gateway {
    pub (crate) state: State,
    pub (crate) env: Env,
    pub (crate) dispatcher: Dispatcher
}

impl Gateway {
    pub fn get_shard_id(&self) -> Result<u32> {
        let do_name = self.state.id().name()
            .ok_or_else(|| worker::Error::from("Impossibile recuperare il nome del DO"))?;

        let shard_number_str = do_name.split('-').last()
            .ok_or_else(|| worker::Error::from("Formato nome DO invalido"))?;

        let shard_id = shard_number_str.parse::<u32>()
            .map_err(|_| worker::Error::from("Impossibile convertire lo shard ID in numero"))?;

        Ok(shard_id)
    }

    pub fn get_websocket(&self) -> Result<WebSocket> {
        let shard_id = self.get_shard_id()?;

        self.state
            .get_websockets()
            .first()
            .ok_or(Error::Generic(format!("Shard {shard_id} is not connected with discord via websocket!")).into())
            .cloned()
    }
}

impl DurableObject for Gateway {
    fn new(state: State, env: Env) -> Self {
        Self {
            dispatcher: Dispatcher::new(state.storage(), env.clone()),
            state,
            env,
        }
    }

    async fn fetch(&self, req: Request) -> Result<Response> {
        Router::new()
            .post_async("/connect", |req, ctx| self.connect(req, ctx))
            .get_async("/health", |req, ctx| self.health(req, ctx))
            .run(req, self.env.clone()).await
    }

    async fn websocket_message(&self, ws: WebSocket, msg: WebSocketIncomingMessage) -> Result<()> {
        let WebSocketIncomingMessage::String(msg) = msg else {
            return Err(Error::UpstreamError("Received a binary message!".into()).into());
        };
        
        let deserializer = GatewayEventDeserializer::from_json(&msg).ok_or_else(|| {
            Error::Generic(format!("Error instantiating gataway event deserializer"))
        })?;

        let mut json_deserializer = serde_json::Deserializer::from_str(&msg);
        let gateway_event: GatewayEvent = deserializer
            .deserialize(&mut json_deserializer)
            .map_err(|e| Error::Generic(format!("Errore deserializzazione GatewayEvent: {e}")))?;

        let shard_id = self.get_shard_id()?;
        let storage = self.state.storage();

        match gateway_event {
            GatewayEvent::Dispatch(seq, event) => {
                save_last_sequence(&storage, shard_id, seq)?;

                worker::console_debug!("Received event: {event:?}");
                let response = self.dispatcher.dispatch(event).await?;

                worker::console_debug!("[Gateway]: Dispatcher response: {response:?}");
            },
            GatewayEvent::Heartbeat => {
                let last_seq = get_last_sequence(&storage, shard_id)?;
                let heartbeat = HeartbeatPayload::new(Some(last_seq));
                let payload = serde_json::to_string(&heartbeat)?;
                ws.send_with_str(payload)?;
            },
            GatewayEvent::Hello(hello) => {
                worker::console_debug!("Received hello: {hello:?}");
                save_heartbeat_interval(&storage, shard_id, hello.heartbeat_interval)?;
                
                let date = Date::new(Millis(Date::now().as_millis() + hello.heartbeat_interval));
                storage.set_alarm(ScheduledTime::new(date.into())).await?;
            },
            GatewayEvent::HeartbeatAck => {
                worker::console_debug!("Heartbeat ACK ricevuto con successo da Discord.");
                set_heartbeat_ack(&storage, shard_id, true)?;
            },
            GatewayEvent::Reconnect => {
                let ws = self.get_websocket()?;
                ws.close(Some(4000), Some("Reconnect requested"))?;
            },
            GatewayEvent::InvalidateSession(resumable) => {
                worker::console_error!("[DO] Session invalidated! Resumable: {}", resumable);
                if !resumable {
                    delete_session(&storage, shard_id)?;
                }
                ws.close(Some(1000), Some("Session invalidated"))?;
            }
        }

        Ok(())
    }

    async fn websocket_close(&self, _ws: WebSocket, code: usize, reason: String, was_clean: bool) -> Result<()> {
        worker::console_debug!("Webscoket connection closed: code={}, reason={}, was_clean={}", 
            code, 
            reason, 
            was_clean
        );
        return Ok(());
    }

    async fn websocket_error(&self, _ws: WebSocket, error: worker::Error) -> Result<()> {
        worker::console_error!("[Gateway] Websocket error: {error:?}");
        Ok(())
    }

    async fn alarm(&self) -> Result<Response> {
        let storage = self.state.storage();
        let shard_id = self.get_shard_id()?;

        if !get_heartbeat_ack(&storage, shard_id)? {
            worker::console_warn!("Lo Shard {} non ha ricevuto l'ACK precedente. Connessione zombie, chiudo!", shard_id);

            if let Ok(ws) = self.get_websocket() {
                ws.close(Some(4000), Some("Heartbeat ACK missed"))?;
            }
            
            return Response::ok("");
        }

        let Ok(ws) = self.get_websocket() else {
            worker::console_warn!(
                "[Alarm Shard {}] L'allarme è scattato ma non ho trovato nessuna WebSocket attiva nei tag.", 
                shard_id
            );

            return Response::ok("");
        };

        set_heartbeat_ack(&storage, shard_id, false)?;

        let last_seq = get_last_sequence(&storage, shard_id)?;
        let heartbeat = HeartbeatPayload::new(if last_seq != 0 { Some(last_seq) } else { None });
        let payload = serde_json::to_string(&heartbeat)?;
        ws.send_with_str(payload)?;

        let interval = get_heartbeat_interval(&storage, shard_id)?;

        if interval > 0 {
            let date = Date::new(Millis(Date::now().as_millis() + interval));
            storage.set_alarm(ScheduledTime::new(date.into())).await?;
        }

        Response::empty()
    }
}