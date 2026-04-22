use async_trait::async_trait;
use twilight_model::{
    application::interaction::{
            Interaction, 
            application_command::CommandData
        }, channel::message::MessageFlags, guild::Permissions, http::interaction::{
        InteractionResponse, InteractionResponseData, InteractionResponseType 
    }
};
use worker::RouteContext;

use crate::{
    discord::{
        command::Command, interaction::InteractionExt, response::ResponseBuilder, 
    }, 
    error::Error, 
    utils
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

    fn default_member_permissions(&self) -> Option<Permissions> {
        Some(Permissions::ADMINISTRATOR)
    }

    async fn respond(
        &self, 
        interaction: &Interaction, 
        _data: &CommandData, 
        ctx: &mut RouteContext<()>
    ) -> Result<InteractionResponse, Error> {
        interaction.defer(true).await?;

        if !interaction.is_dev(ctx) {
            return Err(Error::InteractionFailed("You must be a developer to use this command!".into()))
        }

        let response = utils::update_commands(&ctx.env).await?;
        let status = response.status().as_u16();
        
        if let Err(e) = response.error_for_status() {
            return Err(Error::ReqwestError(e));
        }

        let response = ResponseBuilder::new(InteractionResponseType::ChannelMessageWithSource)
            .content(format!("✅ Aggiornamento comandi completato! Status: **{}**", status))
            .ephemeral()
            .build();

        interaction.edit(&response).await
    }
}