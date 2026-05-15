use worker::*;

use crate::{
    constants::QueueMessage,
    framework::structs::{
        queue::QueueProcessor, 
        scheduler::Scheduler
    }, 
    routes::api
};

mod constants;
mod commands;
mod error;
mod framework;
mod queues;
mod services;
mod triggers;
mod routes;
mod ui;
mod utils;

#[event(queue)]
pub async fn on_queue(
    batch: MessageBatch<QueueMessage>,
    env: Env,
    ctx: Context,
) -> Result<()> {
    QueueProcessor::new(env, ctx).process(batch).await
}

#[event(scheduled)]
pub async fn scheduled(event: ScheduledEvent, env: Env, ctx: ScheduleContext) {
    Scheduler::new(env, ctx).schedule(event).await;
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    utils::log_request(&req);
    utils::set_panic_hook();

    Router::new()
        .post_async("/api/interaction", |req, ctx| api::interaction::post(req, ctx))
        .post_async("/api/gateway", |req, ctx| api::gateway::post(req, ctx))
        .get_async("/api/commands", |req, ctx| api::commands::get(req, ctx))
        .run(req, env)
        .await
}
