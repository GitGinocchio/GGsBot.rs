use std::sync::atomic::AtomicBool;

use futures::StreamExt;
use serde::{Deserialize, de::DeserializeSeed};
use serde_json::json;
use twilight_model::gateway::{event::{GatewayEvent, GatewayEventDeserializer}, payload::outgoing::Heartbeat as HeartbeatPayload};
use worker::{DurableObject, Env, Request, Response, Result, State, Storage, WebSocket, WebsocketEvent, durable_object, wasm_bindgen::prelude::wasm_bindgen, wasm_bindgen_futures};

use crate::{constants::CLIENT, error::Error, gateway::queries::{get_last_sequence, save_last_sequence, setup_storage}};

pub static CONNECT_URL: &'static str  = "https://discord.com/api/v10/gateway/bot";

#[derive(Deserialize, Debug)]
struct SessionStartLimit {
    total: u32,
    remaining: u32,
    reset_after: u64,
    max_concurrency: u32,
}

#[derive(Deserialize, Debug)]
struct GatewayBotResponse {
    url: String,
    shards: u32,
    session_start_limit: SessionStartLimit,
}

#[durable_object]
pub struct Gateway {
    state: State,
    env: Env,
    is_connected: AtomicBool
}

impl DurableObject for Gateway {
    fn new(state: State, env: Env) -> Self {
        Self {
            state,
            env,
            is_connected: AtomicBool::new(false)
        }
    }

    async fn fetch(&self, _req: Request) -> Result<Response> {
        if self.is_connected.load(std::sync::atomic::Ordering::Relaxed) {
            worker::console_log!("[DO Shard] Richiesta ricevuta, ma sono già connesso al Gateway di Discord. Ignoro.");
            return Response::from_json(&serde_json::json!({
                "status": "already_connected",
                "message": "Questo Shard è già attivo e in ascolto."
            }));
        }

        let storage = self.state.storage();
        setup_storage(&storage).await?;

        let token = self.env
            .secret("DISCORD_TOKEN")
            .map_err(|e| Error::EnvironmentVariableNotFound(e.to_string()))?
            .to_string();

        let ws = connect(token).await?;

        let env = self.env.clone();

        // WARN: Qui se andasse in errore il thread non lo sapremmo mai...
        wasm_bindgen_futures::spawn_local(async move {
            if let Err(e) = handle_events(ws, env, storage).await {
                worker::console_error!("Error handling events from the discord gateway: {e}");
            }
        });

        self.is_connected.store(true, std::sync::atomic::Ordering::Relaxed);

        Response::from_json(&json!({
            "status" : "success",
            "message" : "Connected to the discord gateway"
        }))
    }
}

pub async fn connect(token: String) -> Result<WebSocket> {
    // TODO: Spostare questa parte e fare in modo che al primo avvio vengano creati
    // N Durable Object quanti sono quelli richiesti da Discord dopo questa response
    let gateway_info: GatewayBotResponse = CLIENT.get(CONNECT_URL)
        .header("Authorization", &format!("Bot {}", token))
        .send()
        .await
        .map_err(|e| Error::ReqwestError(e))?
        .json()
        .await
        .map_err(|e| Error::ReqwestError(e))?;

    worker::console_debug!("Discord Gateway INFO: {gateway_info:?}");

    if gateway_info.session_start_limit.remaining == 0 {
        worker::console_error!("Session start limit reached!");
        return Err(Error::UpstreamError(format!("Discord session start limit reached!")).into())
    }

    let url = format!("{}?v=10&encoding=json", gateway_info.url);
    
    let headers = worker::Headers::new();
    headers.set("Upgrade", "websocket")?;
    headers.set("Connection", "Upgrade")?;

    let mut init = worker::RequestInit::new();
    init.with_method(worker::Method::Get);
    init.with_headers(headers);

    let request = worker::Request::new_with_init(&url, &init)?;

    let response = worker::Fetch::Request(request)
        .send()
        .await?;

    let ws = response.websocket().ok_or_else(|| {
        Error::UpstreamError(format!("Websocket was not provided in the response"))
    })?;

    ws.accept()?;

    Ok(ws)
}

pub async fn handle_events(ws: WebSocket, _env: Env, storage: Storage) -> Result<()> {
    let mut stream = ws.events()?;
    
    while let Some(maybe_event) = stream.next().await {
        let event = maybe_event.or_else(|e| {
            Err(Error::Generic(format!("Error receiving ws gateway event: {e:?}")))
        })?;

        let gateway_event = match event {
            WebsocketEvent::Message(message) => {
                let payload = message.text().ok_or_else(|| {
                    Error::InvalidPayload(format!("Invalid ws event payload: {message:?}"))
                })?;
                
                let deserializer = GatewayEventDeserializer::from_json(&payload).ok_or_else(|| {
                    Error::Generic(format!("Error instantiating gataway event deserializer"))
                })?;

                let mut json_deserializer = serde_json::Deserializer::from_str(&payload);
                let gateway_event: GatewayEvent = deserializer
                    .deserialize(&mut json_deserializer)
                    .map_err(|e| Error::Generic(format!("Errore deserializzazione GatewayEvent: {e}")))?;

                gateway_event
            },
            WebsocketEvent::Close(close) => {
                worker::console_debug!("Webscoket connection closed: code={}, reason={}, was_clean={}", 
                    close.code(), 
                    close.reason(), 
                    close.was_clean()
                );
                return Ok(());
            }
        };

        // TODO: Terminare l'implementazione

        match gateway_event {
            GatewayEvent::Dispatch(seq, event) => {
                save_last_sequence(&storage, seq)?;

                worker::console_debug!("Received event: {event:?}");
        
                // TODO: Collegare questo evento al Gateway (che dovra' chiamarsi GatewayDispatcher)

            },
            GatewayEvent::Heartbeat => {
                let last_seq = get_last_sequence(&storage)?;
                let heartbeat = HeartbeatPayload::new(Some(last_seq));
                let payload = serde_json::to_string(&heartbeat)?;
                ws.send_with_str(payload)?;
            },
            GatewayEvent::Hello(hello) => {
                worker::console_debug!("Received hello: {hello:?}");
            },
            GatewayEvent::HeartbeatAck => {

            },
            GatewayEvent::Reconnect => {

            },
            GatewayEvent::InvalidateSession(resumable) => {
                worker::console_error!("[DO] Session invalidated! Resumable: {}", resumable);
                if !resumable {
                    //storage.delete("last_sequence").await?;
                    //storage.delete("session_id").await?;
                }
                ws.close(Some(1000), Some("Session invalidated"))?;
            }
        }
    }

    Ok(())
}