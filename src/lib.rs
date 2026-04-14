use std::sync::LazyLock;
use worker::*;

mod utils;

mod discord;
use discord::bot;
use crate::discord::command::CommandMap;

mod commands;


static COMMANDS: LazyLock<CommandMap> = LazyLock::new(|| {
    build_commands!(
        commands::hello::Hello,
        commands::version::Version,
        commands::register::Register
    )
});

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    utils::log_request(&req);
    utils::set_panic_hook();

    Router::new()
        .post_async("/", |req, ctx|  async move {
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
