use twilight_model::gateway::{event::DispatchEvent};

use crate::{constants::BOT_ID, dispatcher::DispatchStrategy, middleware::{EventMiddleware, MiddlewareResponse}, traits::dispatch_event::DispatchEventExt};

pub struct DiscardSelfEventsMiddleware {
}

impl DiscardSelfEventsMiddleware {
    pub fn new() -> Self {
        Self {}
    }
}

impl EventMiddleware for DiscardSelfEventsMiddleware {
    fn name(&self) -> &'static str { "discard-self-events-middleware" } 

    fn execute(&self, event: &DispatchEvent, _strategy: &DispatchStrategy) -> Result<MiddlewareResponse, anyhow::Error> {
        if let Some(user_id) = event.user_id() && user_id.to_string() == *BOT_ID {
            Ok(MiddlewareResponse::Discard)
        } else {
            Ok(MiddlewareResponse::Send)
        }
    }
}