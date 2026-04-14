use crate::COMMANDS;
use crate::discord::interaction::{
    InteractionApplicationCommandCallbackData
};
use crate::discord::error::InteractionError;
use crate::discord::command::{Command, CommandContext, SerializableCommand};

use async_trait::async_trait;

#[derive(Default)]
pub(crate) struct Register {}

#[async_trait(?Send)]
impl Command for Register {
    fn name(&self) -> String { "register".to_string() }
    fn description(&self) -> String { "Aggiorna i comandi globali su Discord".to_string() }

    async fn respond(&self, ctx: &CommandContext) -> Result<InteractionApplicationCommandCallbackData, InteractionError> {
        if let Some(bail) = ctx.admin_or_bail() {
            return Ok(bail);
        }
        
        let to_register: Vec<_> = COMMANDS.values()
            .map(|cmd| SerializableCommand(cmd.as_ref()))
            .collect();

        let client = reqwest::Client::new();
        let app_id = ctx.worker.env.var("DISCORD_APPLICATION_ID")?.to_string();
        let token = ctx.worker.env.var("DISCORD_TOKEN")?.to_string();
        let url = format!("https://discord.com/api/v10/applications/{}/commands", app_id);

        let serialized_commands = serde_json::to_string(&to_register)
            .map_err(|_e| InteractionError::GenericError())?;
        worker::console_log!{"Sending  : {}", serialized_commands};

        let response = client
            .put(url)
            .header("Authorization", format!("Bot {}", token))
            .header("Content-Type", "application/json")
            .body(serialized_commands) 
            .send()
            .await
            .map_err(|e| InteractionError::UpstreamError(e.to_string()))?;

        let status = response.status();
        
        Ok(InteractionApplicationCommandCallbackData {
            content: Some(format!("✅ Registrazione completata! Status: **{}**", status)),
            ..Default::default()
        })
    }
}