use std::sync::{LazyLock, atomic::{AtomicBool, Ordering}};
use reqwest::Client;
use worker::*;

use crate::discord::{bot::Bot, command::{CommandMap, SerializableCommand}};

mod utils;
mod commands;
mod discord;
mod error;
mod traits;
mod structs;
mod components;
mod embeds;


static CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::new()
});

static COMPONENTS: LazyLock<()> = LazyLock::new(|| {
    ()
});

static COMMANDS: LazyLock<CommandMap> = LazyLock::new(|| {
    build_commands!(
        commands::hello::Hello,
        commands::bot::Bot,
        commands::ext::Ext
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
        .get_async("/api/commands", |_req, _ctx| async move {
            let commands: Vec<_> = COMMANDS.values()
                .map(|cmd| SerializableCommand(cmd.as_ref()))
                .collect();

            Response::from_json(&commands)
        })
        .post_async("/api/interaction", |req, ctx| async move {
            match Bot::new(ctx).handle(req).await {
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
