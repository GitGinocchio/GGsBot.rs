use async_trait::async_trait;
use twilight_model::{
    application::interaction::{Interaction, application_command::CommandData},
    http::interaction::{InteractionResponse, InteractionResponseType},
    oauth::ApplicationIntegrationType,
};
use worker::RouteContext;

use crate::{
    error::Error,
    framework::{
        discord::{
            command::Command, 
            interaction::InteractionExt, 
            response::ResponseBuilder,
        }
    },
    services::apod::ApodService,
};

#[derive(Default)]
pub(crate) struct Apod {}

#[async_trait(?Send)]
impl Command for Apod {
    fn name(&self) -> String {
        "apod".into()
    }

    fn description(&self) -> String {
        "Astronomy picture of the day!".into()
    }

    fn integration_types(&self) -> Vec<ApplicationIntegrationType> {
        vec![
            ApplicationIntegrationType::GuildInstall,
            ApplicationIntegrationType::UserInstall,
        ]
    }

    async fn respond(
        &self,
        interaction: &Interaction,
        _data: &CommandData,
        ctx: &mut RouteContext<()>,
    ) -> Result<InteractionResponse, Error> {
        interaction.defer(true).await?;

        let service = ApodService::new(&ctx.env)?;
        let data = service.get_apod().await?;
        let embed = ApodService::build_embed(data);

        let response = ResponseBuilder::new(InteractionResponseType::ChannelMessageWithSource)
            .embeds(vec![embed])
            .ephemeral()
            .build();

        interaction.edit(&response).await
    }
}
