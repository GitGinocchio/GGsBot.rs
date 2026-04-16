use async_trait::async_trait;
use twilight_model::{
    application::interaction::{
            Interaction, 
            application_command::CommandData
        }, channel::message::MessageFlags, http::interaction::{
        InteractionResponse, InteractionResponseData, InteractionResponseType 
    }
};
use worker::RouteContext;

use crate::{
    discord::{
        command::Command, interaction::InteractionExt, 
    }, 
    error::InteractionError, utils
};

#[derive(Default)]
pub struct Update;

#[async_trait(?Send)]
impl Command for Update {
    fn name(&self) -> String {
        "update".into()
    }

    fn description(&self) -> String {
        "Aggiorna i comandi del bot!".into()
    }

    async fn respond(
        &self, 
        interaction: &Interaction, 
        _data: &CommandData, 
        ctx: &mut RouteContext<()>
    ) -> Result<InteractionResponse, InteractionError> {
        if !interaction.is_dev(ctx) {
            return Err(InteractionError::GenericError())
        }

        let response = utils::update_commands(&ctx.env)
            .await
            .map_err(|_e| InteractionError::GenericError())?;
        let status = response.status().as_u16();
        
        if let Err(e) = response.error_for_status() {
            return Err(InteractionError::ReqwestError(e));
        }

        Ok(InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(InteractionResponseData {
                content: Some(format!("✅ Aggiornamento comandi completato! Status: **{}**", status)),
                flags: Some(MessageFlags::EPHEMERAL),
                ..Default::default()
            }),
        })
    }
}