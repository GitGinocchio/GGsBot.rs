use async_trait::async_trait;
use twilight_model::{
    application::{
        command::{CommandOption, CommandOptionType},
        interaction::{
            Interaction, InteractionContextType, application_command::{CommandData, CommandOptionValue}
        },
    },
    http::interaction::{InteractionResponse, InteractionResponseType},
};
use worker::RouteContext;

use crate::{
    error::Error,
    framework::discord::{
        command::{Command, CommandDataExt}, interaction::InteractionExt, option::OptionBuilder, response::ResponseBuilder
    }, services::discord::DiscordService,
};

#[derive(Default)]
pub struct Clear;

#[async_trait(?Send)]
impl Command for Clear {
    fn name(&self) -> String {
        "clear".into()
    }

    fn description(&self) -> String {
        "Elimina i messaggi da una chat!".into()
    }

    fn interaction_contexts(&self) -> Vec<InteractionContextType> {
        vec![InteractionContextType::Guild]
    }

    fn options(&self) -> Vec<CommandOption> {
        vec![
            OptionBuilder::new(CommandOptionType::Integer, "amount", "Il numero di messaggi da eliminare")
                .required(false)
                .build()
        ]
    }

    async fn respond(
        &self,
        interaction: &Interaction,
        data: &CommandData,
        ctx: &mut RouteContext<()>,
    ) -> Result<InteractionResponse, Error> {
        interaction.defer(true).await?;

        let amount = match data.get_option("amount") {
            Some(CommandOptionValue::Integer(value)) => *value,
            _ => 10,
        };

        if amount < 1 || amount > 100 {
            let response = ResponseBuilder::new(InteractionResponseType::ChannelMessageWithSource)
                .content("⚠️ Puoi eliminare solo un numero di messaggi compreso tra 1 e 100.")
                .ephemeral()
                .build();

            return interaction.edit(&response).await;
        }

        let channel_id = interaction
            .channel
            .as_ref()
            .map(|c| c.id)
            .ok_or_else(|| Error::InteractionFailed("Impossibile determinare il canale.".into()))?
            .get();

        let discord = DiscordService::new(&ctx.env)?;
        let num_deleted = discord.delete_messages(&channel_id.to_string(), amount as u8).await?;

        let response = ResponseBuilder::new(InteractionResponseType::ChannelMessageWithSource)
            .content(format!("🗑️ Eliminati **{}** messaggi.", num_deleted))
            .ephemeral()
            .build();

        interaction.edit(&response).await
    }
}
