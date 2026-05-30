use serde_json::json;
use url::Url;
use worker::{Headers, Method, Request, RequestInit, Response, Result, RouteContext, wasm_bindgen_futures};
use serde::Deserialize;

use crate::{bindings::DurableObjectBinding, constants::CLIENT, error::Error};

pub static CONNECT_URL: &'static str  = "https://discord.com/api/v10/gateway/bot";

#[derive(Deserialize, Debug)]
#[allow(unused)]
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

pub async fn connect(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let token = ctx.env
        .secret("DISCORD_TOKEN")
        .map_err(|e| Error::EnvironmentVariableNotFound(e.to_string()))?
        .to_string();

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

    let namespace = ctx.env.durable_object(&DurableObjectBinding::Gateway.to_string())?;
    let gateway_url = gateway_info.url.clone();
    let total_shards = gateway_info.shards;

    worker::console_log!("[Orchestratore] Sincronizzazione automatica di {} shard richiesta da Discord...", total_shards);

    for shard_id in 0..total_shards {
        let namespace_clone = namespace.clone();
        let url_clone = gateway_url.clone();
        
        if let Ok(do_id) = namespace_clone.id_from_name(&format!("gateway-shard-{}", shard_id)) {
            if let Ok(stub) = do_id.get_stub() {
                let headers = Headers::new();
                headers
                    .set("X-Gateway-Url", &url_clone)
                    .expect("Expect Header X-Gateway-Url to be added in the headers map");
                headers
                    .set("X-Shard-Id", &shard_id.to_string())
                    .expect("Expect Header X-Shard-Id to be added in the headers map");
                
                let mut init = RequestInit::new();
                init.with_method(Method::Post);
                init.with_headers(headers);

                worker::console_debug!("[Gateway]: spawning shard {shard_id}");
                
                if let Ok(req) = Request::new_with_init("http://durableobject/connect", &init) {
                    let response = stub.fetch_with_request(req).await;
                    worker::console_debug!("[Shard]: Connect response: {response:?}");
                }
            }
        }
    }

    Response::from_json(&json!({ 
        "status": "reconciliation_started", 
        "message": "Il controllo globale e l'allineamento degli shard è stato avviato.", 
        "expected_shards": total_shards 
    }))
}