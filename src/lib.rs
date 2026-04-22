use std::sync::{LazyLock, atomic::{AtomicBool, Ordering}};
use reqwest::Client;
use worker::*;

use crate::{
    discord::{
        bot::Bot, 
        command::{
            CommandMap, 
            SerializableCommand
        }, 
    }, 
    traits::{
        ui::UiHandlerMap
    }
};

mod utils;
mod commands;
mod discord;
mod error;
mod traits;
mod structs;
mod ui;


static CLIENT: LazyLock<Client> = LazyLock::new(|| Client::new());

static UIHANDLERS: LazyLock<UiHandlerMap> = LazyLock::new(|| build_ui_handlers!(
    ui::hello::HelloUiHandler
));

static COMMANDS: LazyLock<CommandMap> = LazyLock::new(|| build_commands!(
    commands::hello::Hello,
    commands::nasa::Nasa,
    commands::bot::Bot,
    commands::ext::Ext
));

#[event(scheduled)]
pub async fn scheduled(_event: ScheduledEvent, _env: Env, _ctx: ScheduleContext) {
    worker::console_debug!("scheduled event triggered!");
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    utils::log_request(&req);
    utils::set_panic_hook();

    Router::new()
        .post_async("/api/interaction", async |req, ctx| Bot::new(ctx).handle(req).await)
        .get_async("/api/commands", |_req, _ctx| async move {
            let commands: Vec<_> = COMMANDS.values()
                .map(|cmd| SerializableCommand(cmd.as_ref()))
                .collect();

            Response::from_json(&commands)
        })
        .run(req, env)
        .await
}
