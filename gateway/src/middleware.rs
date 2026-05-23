
use serde_json::Value;
use twilight_model::gateway::event::DispatchEvent;

use crate::dispatcher::DispatchStrategy;

pub trait EventMiddleware: Send + Sync {
    fn name(&self) -> &'static str;
    fn execute(&self, event: &DispatchEvent, strategy: &DispatchStrategy) -> Result<Value, anyhow::Error>;
}