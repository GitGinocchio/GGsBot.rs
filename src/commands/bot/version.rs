use async_trait::async_trait;
use chrono::{DateTime, Utc};
use twilight_model::{
    application::interaction::{
            Interaction, 
            application_command::CommandData
        }, channel::message::MessageFlags, http::interaction::{
        InteractionResponse, InteractionResponseData, InteractionResponseType 
    }
};
use worker::{RouteContext, WorkerVersionMetadata};

use crate::{
    discord::{
        command::Command, 
    }, 
    error::InteractionError, 
    utils::is_dev
};

#[derive(Default)]
pub struct Version;

#[async_trait(?Send)]
impl Command for Version {
    fn name(&self) -> String {
        "version".into()
    }

    fn description(&self) -> String {
        "Mostra la version del bot!".into()
    }

    async fn respond(
        &self, 
        _interaction: &Interaction, 
        _data: &CommandData, 
        ctx: &mut RouteContext<()>
    ) -> Result<InteractionResponse, InteractionError> {
        let metadata: WorkerVersionMetadata = ctx.env.get_binding::<WorkerVersionMetadata>("metadata")?;
        let mut lines = Vec::new();

        if !is_dev(&ctx.env) {
            lines.push(format!("**🆔 ID:** `{}`", metadata.id()));
        }

        if !metadata.tag().is_empty() {
            lines.push(format!("**🏷️ Tag:** `{}`", metadata.tag()));
        }

        let timestamp_str = metadata.timestamp();
        if let Ok(dt) = timestamp_str.parse::<DateTime<Utc>>() {
            let unix_secs = dt.timestamp(); 
            
            lines.push(format!("**⏰ Built at:** <t:{}:R>", unix_secs));
        } else {
            lines.push(format!("**⏰ Built at:** {}", timestamp_str));
        }

        let message = lines.join("\n");

        Ok(InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(InteractionResponseData {
                content: Some(message),
                flags: Some(MessageFlags::EPHEMERAL),
                ..Default::default()
            }),
        })
    }
}