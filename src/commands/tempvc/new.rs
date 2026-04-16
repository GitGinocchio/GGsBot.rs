use async_trait::async_trait;
use twilight_model::{
    application::interaction::{
            Interaction, 
            application_command::CommandData
        }, channel::message::MessageFlags, http::interaction::{
        InteractionResponse, InteractionResponseData, InteractionResponseType 
    }
};
use worker::{RouteContext};

use crate::{
    discord::{
        command::Command, 
    }, 
    error::InteractionError, 
};

#[derive(Default)]
pub struct New;

#[async_trait(?Send)]
impl Command for New {
    fn name(&self) -> String {
        "new".into()
    }

    fn description(&self) -> String {
        "Crea un canale vocale personalizzato!".into()
    }

    async fn respond(
        &self, 
        _interaction: &Interaction, 
        _data: &CommandData, 
        _ctx: &mut RouteContext<()>
    ) -> Result<InteractionResponse, InteractionError> {
        Ok(InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(InteractionResponseData {
                content: None,
                flags: Some(MessageFlags::EPHEMERAL),
                ..Default::default()
            }),
        })
    }
}