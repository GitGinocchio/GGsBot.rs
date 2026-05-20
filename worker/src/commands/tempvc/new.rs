use async_trait::async_trait;
use twilight_model::{
    application::{command::{CommandOption, CommandOptionType}, interaction::{
        Interaction, 
        application_command::CommandData
    }}, 
    http::interaction::InteractionResponse
};
use worker::RouteContext;

use crate::{error::Error, framework::discord::{command::Command, option::OptionBuilder, response::InteractionResponseExt}};

#[derive(Default)]
pub struct New;

#[async_trait(?Send)]
impl Command for New {
    fn name(&self) -> String {
        "new".into()
    }

    fn description(&self) -> String {
        "Aggiungi un generatore di canali temporanei per questo server!".into()
    }

    fn options(&self) -> Vec<CommandOption> {
        // TODO: Inserire i channel_types dopo aver aggiunto un metodo nel builder
        // per renderlo possibile
        vec![
            OptionBuilder::new(
                CommandOptionType::Channel, 
                "channel", 
                "Canale usato per generare i canali vocali temporanei"
            )
            .build()
        ]
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
