use async_trait::async_trait;
use twilight_model::{
    application::{command::CommandOption, interaction::{
        Interaction, 
        application_command::{CommandData, CommandOptionValue}
    }}, channel::{ChannelType, message::Embed}, http::interaction::{InteractionResponse, InteractionResponseType}
};
use worker::RouteContext;

use crate::{
    commands::tempvc::TempvcExtConfig, error::Error, framework::{discord::{
            command::{Command, CommandDataExt}, interaction::InteractionExt, option::OptionBuilder, response::ResponseBuilder
        }, structs::config::extension::ExtensionConfig, traits::namespaces::KvExt}, ui::embeds::{default::DEFAULT_EMBED, error::ERROR_EMBED}
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

    fn options(&self) -> Vec<CommandOption> {
        vec![
            OptionBuilder::channel("channel", "Canale usato per generare i canali vocali temporanei")
                .channel_types(vec![ChannelType::GuildVoice, ChannelType::GuildStageVoice])
                .required(true)
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

        let channel = match data.get_option("channel") {
            Some(CommandOptionValue::Channel(channel)) => channel.get(),
            Some(_) | None => return Err(Error::InteractionFailed("Missing required option 'channel'".into()))
        };

        let guild_kv = interaction.guild_kv(&ctx.env)?;
        let key = format!("extensions:tempvc:config");

        let mut extension = guild_kv
            .get_json::<ExtensionConfig<TempvcExtConfig>>(&key)
            .await?
            .unwrap_or_default();

        let channels = &mut extension
            .config
            .get_or_insert_default()
            .channels;

        let string_channel = channel.to_string();
        let embed: Embed;

        if !channels.contains(&string_channel) {
            embed = ERROR_EMBED.clone()
                .description("This channel is not set as generator channel!")
                .build();
        } else {
            channels.retain(|c| c != &string_channel);

            guild_kv.put(&key, serde_json::to_string(&extension)?, None).await?;
            
            embed = DEFAULT_EMBED.clone()
                .description("Channel successfully removed from config!")
                .build();
        }

        let response = ResponseBuilder::new(InteractionResponseType::ChannelMessageWithSource)
            .embeds(vec![embed])
            .build();

        Ok(interaction.edit(&response).await?)
    }
}
