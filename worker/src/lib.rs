use flarecord::prelude::*;
use worker::*;

mod commands;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    let bot = Bot::new();

    Router::new()
        .post_async("/api/interaction", async |req, env| { bot.handle_interaction(req, env.env).await })
        .on_async("/api/*path", async |req, env| { bot.handle_api(req, env.env).await })
        
        .or_else_any_method_async("/*path", async |_, _| { Response::error("Method not allowed", 405)})
        .on_async("/*path", async |_, _| { Response::error("Not Found", 404)})
        
        .run(req, env)
        .await
}
