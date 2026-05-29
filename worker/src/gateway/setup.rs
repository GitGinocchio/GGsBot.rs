use worker::{Env, Headers, Method, Request, RequestInit, Result};

use crate::constants::DEFAULT_DISCORD_GATEWAY_URL;


pub async fn setup_shard_zero(env: Env) -> Result<()> {
    worker::console_log!("[Init] Avvio del Durable Object per lo Shard 0...");

    let namespace = env.durable_object("GATEWAY")?;

    // Identifichiamo univocamente lo Shard 0
    let object_id = namespace.id_from_name("shard-0")?;
    let stub = object_id.get_stub()?;

    let headers = Headers::new();
    headers.set("X-Gateway-Url", DEFAULT_DISCORD_GATEWAY_URL)?;
    headers.set("X-Shard-Id", "0")?;

    let mut init = RequestInit::new();
    init.with_method(Method::Post);
    init.with_headers(headers);

    let do_req = Request::new_with_init("http://durableobject/connect", &init)?;
    
    match stub.fetch_with_request(do_req).await {
        Ok(_) => worker::console_log!("[Init] Shard 0 inizializzato con successo dal ciclo di boot!"),
        Err(e) => worker::console_error!("[Init] Errore critico durante il boot dello Shard 0: {e}"),
    }

    Ok(())
}