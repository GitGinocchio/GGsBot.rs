use async_trait::async_trait;
use serde_json::Value;
use twilight_model::{
    application::interaction::Interaction, 
    gateway::{
        event::{DispatchEvent, EventType}, 
        payload::incoming::*
    }, 
    http::interaction::InteractionResponse
};
use worker::{Env, RouteContext};

use crate::error::Error;

#[async_trait(?Send)]
pub trait CommandController {
    #[allow(unused)]
    async fn get_default_config(
        &self,
        interaction: &Interaction,
        ctx: &mut RouteContext<()>,
    ) -> Option<serde_json::Value> {
        None
    }

    #[allow(unused)]
    /// method called when this command is set up on a discord server
    async fn before_setup(&self, interaction: &Interaction, ctx: &mut RouteContext<()>) {}

    #[allow(unused)]
    async fn on_setup(
        &self,
        interaction: &Interaction,
        ctx: &mut RouteContext<()>,
    ) -> Option<Result<InteractionResponse, Error>> {
        None
    }

    #[allow(unused)]
    async fn after_setup(&self, interaction: &Interaction, ctx: &mut RouteContext<()>) {}

    /// method called when a command is removed from a discord server (act like a clean-up)
    #[allow(unused)]
    async fn before_teardown(&self, interaction: &Interaction, ctx: &mut RouteContext<()>) {}
    #[allow(unused)]
    async fn after_teardown(&self, interaction: &Interaction, ctx: &mut RouteContext<()>) {}

    /// method called when a command is enabled from a discord server
    #[allow(unused)]
    async fn on_enabled(&self, interaction: &Interaction, ctx: &mut RouteContext<()>) {}

    /// method called when a command is disabled from a discord server
    #[allow(unused)]
    async fn on_disabled(&self, interaction: &Interaction, ctx: &mut RouteContext<()>) {}

    #[allow(unused)]
    async fn on_teardown(
        &self,
        interaction: &Interaction,
        ctx: &mut RouteContext<()>,
    ) -> Option<Result<InteractionResponse, Error>> {
        None
    }
}

#[async_trait(?Send)]
pub trait CommandEvents {
    fn responds_to(&self, _event_type: EventType) -> bool {
        false
    }

    async fn on_message_create(&self, _env: Env, _payload: MessageCreate, _metadata: Value) -> Result<Value, Error> { Ok(Value::Null) }
    async fn on_message_update(&self, _env: Env, _payload: MessageUpdate, _metadata: Value) -> Result<Value, Error> { Ok(Value::Null) }
    async fn on_message_delete(&self, _env: Env, _payload: MessageDelete, _metadata: Value) -> Result<Value, Error> { Ok(Value::Null) }
    async fn on_message_delete_bulk(&self, _env: Env, _payload: MessageDeleteBulk, _metadata: Value) -> Result<Value, Error> { Ok(Value::Null) }

    async fn on_voice_state_update(&self, _env: Env, _payload: VoiceStateUpdate, _metadata: Value) -> Result<Value, Error> { Ok(Value::Null) }

    async fn dispatch(&self, env: Env, event: DispatchEvent, metadata: Value) -> Result<Value, Error> {
        match event {
            DispatchEvent::MessageCreate(m) => self.on_message_create(env, *m, metadata).await,
            DispatchEvent::MessageUpdate(m) => self.on_message_update(env, *m, metadata).await,
            DispatchEvent::MessageDelete(m) => self.on_message_delete(env, m, metadata).await,
            DispatchEvent::MessageDeleteBulk(m) => self.on_message_delete_bulk(env, m, metadata).await,
            DispatchEvent::VoiceStateUpdate(s) => self.on_voice_state_update(env, *s, metadata).await,
            e => {
                worker::console_warn!("Unhandled event: {e:?}");
                Ok(Value::Null)
            }
        }
    }
}