
use serde_json::Value;
use twilight_gateway::EventTypeFlags;
use twilight_model::gateway::event::DispatchEvent;

use crate::{constants::MIDDLEWARES, dispatcher::DispatchStrategy};

#[derive(Clone, Debug)]
pub enum MiddlewareResponse {
    Send,
    SendWithMetadata(Value),
    Discard,
}

pub trait EventMiddleware: Send + Sync {
    fn name(&self) -> &'static str;
    fn execute(&self, event: &DispatchEvent, strategy: &DispatchStrategy) -> Result<MiddlewareResponse, anyhow::Error>;
}

pub fn get_middlewares(event_flag: EventTypeFlags) -> impl Iterator<Item = &'static Box<dyn EventMiddleware>> {
    MIDDLEWARES
        .iter()
        .filter(move |(flags, _)| flags.contains(event_flag))
        .map(|(_, middleware)| middleware)
}