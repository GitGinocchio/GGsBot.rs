use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use twilight_model::{application::interaction::Interaction, gateway::payload::incoming::VoiceStateUpdate, http::interaction::InteractionResponse};
use worker::RouteContext;

use crate::{
    build_commands, commands::tempvc, error::Error, framework::{discord::command::{Command, CommandMap}, traits::{command::{CommandController, CommandEvents}, namespaces::KvExt}}
};

mod new;
mod del;

#[derive(Default)]
pub(crate) struct Tempvc {}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TempvcExtConfig {
    channels: Vec<String>
}

#[async_trait(?Send)]
impl CommandController for Tempvc {
    async fn get_default_config(&self, _interaction: &Interaction, _ctx: &mut RouteContext<()>) -> Option<Value> {
        serde_json::to_value(TempvcExtConfig::default()).ok()
    }

    async fn on_setup(
        &self,
        interaction: &Interaction,
        ctx: &mut RouteContext<()>,
    ) -> Option<Result<InteractionResponse, Error>> {
        let guild_kv = match interaction.guild_kv(&ctx.env) {
            Ok(kv) => kv,
            Err(e) => return Some(Err(e))
        };

        let outcome = guild_kv.duplicate(
            &format!("extensions:{}:config:pending", self.name()),
            &format!("extensions:{}:config", self.name()),
            None
        ).await;

        match outcome {
            Ok(_) => {},
            Err(e) => return Some(Err(Error::KvError(e)))
        };

        None
    }
}

#[async_trait(?Send)]
impl CommandEvents for Tempvc {
    async fn on_voice_state_update(&self, _ctx: &RouteContext<()>, state: VoiceStateUpdate) -> Result<Value, Error> {
        worker::console_debug!("[tempvc-events] Received voice_state_update: {state:?}");
        Ok(Value::Null)
    }
}

#[async_trait(?Send)]
impl Command for Tempvc {
    fn name(&self) -> String {
        "tempvc".into()
    }

    fn description(&self) -> String {
        "Crea canali vocali personalizzati per te!".into()
    }

    fn subcommands(&self) -> CommandMap {
        build_commands![tempvc::new::New, tempvc::del::Del]
    }

    fn get_controller(&self) -> Option<&dyn CommandController> {
        Some(self)
    }
}
