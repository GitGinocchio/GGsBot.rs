use async_trait::async_trait;
use chrono::{Duration, Utc};

use crate::{CLIENT, 
    discord::{
        command::{
            Command, 
            CommandContext
        }, error::InteractionError, interaction::InteractionApplicationCommandCallbackData, locale::{Locale, Localization}, message::{
            Message, 
            MessageFlags
        }, option::{ApplicationCommandOption, ApplicationCommandOptionType}
    }, map
};

#[derive(Default)]
pub(crate) struct Clear {}

#[async_trait(?Send)]
impl Command for Clear {
    fn name(&self) -> Localization { 
        Localization::Map(map! {
            Locale::EnglishUS => "clear".to_string(),
            Locale::Italian => "pulisci".to_string()
        })
    }
    fn description(&self) -> Localization { "Elimina un numero specificato di messaggi".into() }

    fn options(&self) -> Vec<ApplicationCommandOption> {
        vec![
            ApplicationCommandOption {
                name: "amount".into(),
                description: "The amount of messages to delete".into(),
                ty: ApplicationCommandOptionType::Integer,
                required: Some(false),
                ..Default::default()
            }
        ]
    }

    async fn respond(&self, ctx: &mut CommandContext) -> Result<InteractionApplicationCommandCallbackData, InteractionError> {
        let amount = ctx.get_option("amount")
            .map(|v| v.as_u64())
            .flatten()
            .unwrap_or(100)
            .clamp(2, 100);

        let channel_id = ctx.channel_id.as_ref().ok_or_else(|| InteractionError::GenericError())?;
        let token = ctx.get_env("DISCORD_TOKEN").ok_or_else(|| InteractionError::GenericError())?;

        let messages_url = format!("https://discord.com/api/v10/channels/{}/messages?limit={}", channel_id, amount);
        let messages: Vec<Message> = CLIENT.get(messages_url)
            .header("Authorization", format!("Bot {}", token))
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| InteractionError::UpstreamError(e.to_string()))?
            .json()
            .await
            .map_err(|_e| InteractionError::GenericError())?;

        let message_ids: Vec<&str> = messages.iter()
            .filter(|m| m.timestamp > Utc::now() - Duration::days(14))
            .map(|m| m.id.as_ref())
            .collect();

        if message_ids.is_empty() {
            return Ok(InteractionApplicationCommandCallbackData {
                content: Some("⚠️ Non ho trovato messaggi da eliminare. Nota: non posso eliminare messaggi più vecchi di 14 giorni.".into()),
                flags: Some(MessageFlags::EPHEMERAL),
                ..Default::default()
            });
        }

        let bulk_url = format!("https://discord.com/api/v10/channels/{}/messages/bulk-delete", channel_id);
        let bulk_data = serde_json::json!({ "messages": message_ids });

        CLIENT.post(bulk_url)
            .header("Authorization", format!("Bot {}", token))
            .header("Content-Type", "application/json")
            .json(&bulk_data)
            .send()
            .await
            .map_err(|e| InteractionError::UpstreamError(e.to_string()))?;

        Ok(InteractionApplicationCommandCallbackData {
            content: Some(format!("🗑️ Eliminati **{}** messaggi.", message_ids.len())),
            flags: Some(MessageFlags::EPHEMERAL),
            ..Default::default()
        })
    }
}