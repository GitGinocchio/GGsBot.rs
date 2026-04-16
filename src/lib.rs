use std::sync::{LazyLock, atomic::{AtomicBool, Ordering}};
use reqwest::Client;
use worker::*;

mod utils;

mod discord;
use discord::bot;
use crate::discord::command::CommandMap;

mod commands;

static CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::new()
});

static COMMANDS: LazyLock<CommandMap> = LazyLock::new(|| {
    build_commands!(
        commands::hello::Hello,
        commands::version::Version,
        commands::update::Update,
        commands::mods::Mods
    )
});

static INITIALIZED: AtomicBool = AtomicBool::new(false);

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    utils::log_request(&req);
    utils::set_panic_hook();

    if !INITIALIZED.load(Ordering::SeqCst) {
        if let Err(e) = utils::update_commands(&env).await {
            worker::console_log!("Errore registrazione iniziale: {:?}", e);
        } else {
            worker::console_log!("Comandi aggiornati correttamente");
            INITIALIZED.store(true, Ordering::SeqCst);
        }
    }

    Router::new()
        .post_async("/api/interaction", |req, ctx|  async move {
            let mut app = bot::Bot::new(req, ctx);

             match app.handle_request().await {
                Ok(result) => {
                    worker::console_log!("Response : {}", serde_json::to_string_pretty(&result).unwrap());
                    Response::from_json(&result) 
                },
                Err(httperr) => {
                    worker::console_log!("Error response : {}", httperr.to_string());
                    Response::error(httperr.to_string(), httperr.status_code())
                }
            }

        })
        .run(req, env)
        .await
}
