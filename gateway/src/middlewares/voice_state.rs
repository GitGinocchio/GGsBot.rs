use dashmap::DashMap;
use twilight_model::{gateway::{event::DispatchEvent, payload::incoming::VoiceStateUpdate}, id::{Id, marker::{ChannelMarker, GuildMarker, UserMarker}}};

use crate::{dispatcher::DispatchStrategy, middleware::{EventMiddleware, MiddlewareResponse}};

#[derive(Hash, PartialEq, Eq)]
struct VoiceKey {
    guild_id: Id<GuildMarker>,
    user_id: Id<UserMarker>,
}

pub struct VoiceStateMiddleware {
    states: DashMap<VoiceKey, Id<ChannelMarker>>,
}

impl VoiceStateMiddleware {
    pub fn new() -> Self {
        Self {
            states: DashMap::new()
        }
    }
}

impl EventMiddleware for VoiceStateMiddleware {
    fn name(&self) -> &'static str { "voice-state-metadata" } 

    fn execute(&self, event: &DispatchEvent, _strategy: &DispatchStrategy) -> Result<MiddlewareResponse, anyhow::Error> {
        let mut metadata = serde_json::Value::Null;

        if let DispatchEvent::VoiceStateUpdate(box_update) = event {
            let update: &VoiceStateUpdate = box_update;
            
            let Some(guild_id) = update.guild_id else {
                return Ok(MiddlewareResponse::SendWithMetadata(metadata));
            };
            
            let key = VoiceKey {
                guild_id,
                user_id: update.user_id,
            };

            let previous_id = self.states.get(&key).map(|r| r.value().clone());

            match update.channel_id {
                Some(new_id) => { self.states.insert(key, new_id); }
                None => { self.states.remove(&key); }
            }

            metadata = serde_json::json!({
                "before_channel_id": previous_id
            });
        }

        Ok(MiddlewareResponse::SendWithMetadata(metadata))
    }
}