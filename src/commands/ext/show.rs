use async_trait::async_trait;
use twilight_model::{application::{command::{CommandOption, CommandOptionChoiceValue, CommandOptionType}, interaction::{Interaction, application_command::CommandData}}, channel::message::Embed, http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType}};
use worker::RouteContext;

use crate::{
    COMMANDS, discord::{
        command::Command, 
        embed::{EmbedBuilder, EmbedExt}
    }, embeds::default::DEFAULT_EMBED, error::{Error, InteractionError}, traits::namespaces::InteractionKvExt
};

#[derive(Default)]
pub(crate) struct Show {
}

#[async_trait(?Send)]
impl Command for Show {
    fn name(&self) -> String { "show".into() }

    fn description(&self) -> String { "Mostra le estensioni abilitate/disabilitate o disponibili!".into() }

    async fn respond(
        &self, 
        _interaction: &Interaction,
        _data: &CommandData,
        _ctx: &mut RouteContext<()>
    ) -> Result<InteractionResponse, InteractionError> {
        /*
        let guild_kv = interaction.guild_kv(ctx)?;
        let commands = guild_kv.list(Some("commands".into()), None, None)
            .await
            .map_err(|e| InteractionError::KvError(e))?;
        */

        let mut description = String::new();

        for (name, _) in COMMANDS.iter() {
            description.push_str(&format!("- *{name}*\n"))
        }

        let embed = DEFAULT_EMBED.clone()
            .title("GGsBot Extensions")
            .field("All extensions", description, false)
            .build();
        
        Ok(InteractionResponse { 
            kind: InteractionResponseType::ChannelMessageWithSource, 
            data: Some(InteractionResponseData {
                embeds: Some(vec![embed]),
                ..Default::default()
            })
        })
    }
}