use crate::discord::interaction::{
    InteractionApplicationCommandCallbackData, 
};
use crate::discord::locale::{Locale, Localization};
use crate::discord::message::MessageFlags;
use crate::map;
use crate::utils::is_dev;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use worker::WorkerVersionMetadata;


use crate::discord::error::InteractionError;
use crate::discord::command::{Command, CommandContext};

#[derive(Default)]
pub(crate) struct Version {}

#[async_trait(?Send)]
impl Command for Version {
    fn name(&self) -> Localization {
        Localization::Map(map! {
            Locale::EnglishUS => "version".to_string(),
            Locale::Italian => "versione".to_string()
        })
    }
    fn description(&self) -> Localization { "Returns the bot version!".into() }

    async fn respond(&self, ctx: &mut CommandContext) -> Result<InteractionApplicationCommandCallbackData, InteractionError> {
        let metadata: WorkerVersionMetadata = ctx.worker.env.get_binding::<WorkerVersionMetadata>("metadata")?;
        let mut lines = Vec::new();

        if !is_dev(&ctx.worker.env) {
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

        Ok(InteractionApplicationCommandCallbackData {
            content: Some(message),
            flags: Some(MessageFlags::EPHEMERAL),
            ..Default::default()
        })
    }
}

