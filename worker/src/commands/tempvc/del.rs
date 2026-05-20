use async_trait::async_trait;
use twilight_model::{
    application::interaction::{
        Interaction, 
        application_command::CommandData
    }, 
    http::interaction::InteractionResponse
};
use worker::RouteContext;

use crate::{
    error::Error, 
    framework::{
        discord::{
            command::Command, 
            response::InteractionResponseExt
        }
    }
};

#[derive(Default)]
pub struct Del;

#[async_trait(?Send)]
impl Command for Del {
    fn name(&self) -> String {
        "del".into()
    }

    fn description(&self) -> String {
        "Rimuovi un generatore di canali temporanei!".into()
    }

    async fn respond(
        &self,
        _interaction: &Interaction,
        _data: &CommandData,
        _ctx: &mut RouteContext<()>,
    ) -> Result<InteractionResponse, Error> {
        Ok(InteractionResponse::empty())
    }
}
