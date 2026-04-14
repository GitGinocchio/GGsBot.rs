use crate::utils;
use crate::discord::interaction::InteractionApplicationCommandCallbackData;
use crate::discord::error::InteractionError;
use crate::discord::command::{ApplicationIntegrationType, Command, CommandContext};
use crate::discord::message::MessageFlags;

use async_trait::async_trait;

#[derive(Default)]
pub(crate) struct Update {}

#[async_trait(?Send)]
impl Command for Update {
    fn name(&self) -> String { "update".to_string() }
    fn description(&self) -> String { "Aggiorna i comandi globali su Discord".to_string() }

    fn integration_types(&self) -> Option<Vec<ApplicationIntegrationType>> {
        Some(vec![ApplicationIntegrationType::GuildInstall])
    }

    async fn respond(&self, ctx: &CommandContext) -> Result<InteractionApplicationCommandCallbackData, InteractionError> {
        if let Some(bail) = ctx.admin_or_bail() {
            return Ok(bail);
        }

        let response = utils::update_commands(&ctx.worker.env)
            .await
            .map_err(|_e| InteractionError::GenericError())?;
        let status = response.status().as_u16();
        
        if let Err(e) = response.error_for_status() {
            return Err(InteractionError::ReqwestError(e));
        }
        
        Ok(InteractionApplicationCommandCallbackData {
            content: Some(format!("✅ Aggiornamento comandi completato! Status: **{}**", status)),
            flags: Some(MessageFlags::EPHEMERAL),
            ..Default::default()
        })
    }
}