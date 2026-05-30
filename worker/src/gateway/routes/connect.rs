use serde_json::json;
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};
use worker::{Request, Response, Result, RouteContext, js_sys, web_sys::{self, Event, MessageEvent}};

use crate::{error::Error, gateway::{gateway::{Gateway}, queries::setup_storage}};

const DEFAULT_GATEWAY_URL: &'static str = "wss://gateway.discord.gg";

impl Gateway {
    pub async fn connect(&self, req: Request, _ctx: RouteContext<()>) -> Result<Response> {
        if !self.state.get_websockets().is_empty() {
            return Response::from_json(&serde_json::json!({
                "status": "already_connected",
                "message": "Questo Shard è già attivo."
            }));
        }

        let headers = req.headers();
        let gateway_url = headers.get("X-Gateway-Url")?.unwrap_or_else(|| DEFAULT_GATEWAY_URL.into());
        let _ = headers.get("X-Shard-Id")?.ok_or_else(|| Error::HeaderNotFound("Missing X-Shard-Id".into()))?;

        // 1. Connessione a Discord
        let url = format!("{}?v=10&encoding=json", gateway_url.replace("wss://", "https://"));
        let response = worker::Fetch::Url(url.parse()?).send().await?;
        let discord_ws = response.websocket().ok_or_else(|| Error::UpstreamError("No WS".into()))?;
        
        // Discord richiede .accept() per i socket outbound
        discord_ws.accept()?;

        // 2. Creazione Tunnel Locale (Pair)
        let pair = worker::WebSocketPair::new()?;
        let local_server = pair.server;
        let local_client = pair.client;

        // 3. Consegna il server al DO (Questo permette l'ibernazione)
        self.state.accept_web_socket(&local_server);

        // 4. Casting a web_sys nativo per gestire gli eventi in modo persistente
        let js_discord_ws: web_sys::WebSocket = discord_ws.as_ref().clone().dyn_into().map_err(|_| worker::Error::RustError("Cast err".into()))?;
        let js_local_client: web_sys::WebSocket = local_client.as_ref().clone().dyn_into().map_err(|_| worker::Error::RustError("Cast err".into()))?;

        // 5. Bridge: Discord -> DO (local_client)
        let client_clone = js_local_client.clone();
        let on_msg_discord = Closure::<dyn FnMut(MessageEvent)>::new(move |e: MessageEvent| {
            if let Some(text) = e.data().as_string() {
                let _ = client_clone.send_with_str(&text);
            }
        });
        js_discord_ws.set_onmessage(Some(on_msg_discord.as_ref().unchecked_ref()));
        on_msg_discord.forget();

        // 6. Bridge: DO (local_client) -> Discord
        let discord_clone = js_discord_ws.clone();
        let on_msg_local = Closure::<dyn FnMut(MessageEvent)>::new(move |e: MessageEvent| {
            if let Some(text) = e.data().as_string() {
                let _ = discord_clone.send_with_str(&text);
            }
        });
        js_local_client.set_onmessage(Some(on_msg_local.as_ref().unchecked_ref()));
        on_msg_local.forget();

        // 7. Chiusura speculare (indispensabile per evitare leak di socket)
        let d_ws = js_discord_ws.clone();
        let on_close_local = Closure::<dyn FnMut(Event)>::new(move |_| { let _ = d_ws.close(); });
        js_local_client.set_onclose(Some(on_close_local.as_ref().unchecked_ref()));
        on_close_local.forget();

        Response::from_json(&serde_json::json!({"status": "success"}))
    }
}