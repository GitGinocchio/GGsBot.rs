use async_trait::async_trait;
use twilight_model::{
    application::{command::{CommandOption}, interaction::{
        Interaction, 
        application_command::{CommandData, CommandOptionValue}
    }}, channel::ChannelType, http::interaction::{InteractionResponse, InteractionResponseType}
};
use worker::RouteContext;

use crate::{
    commands::tempvc::TempvcExtConfig, 
    error::Error, 
    framework::{discord::{
            command::{
                Command, 
                CommandDataExt
            }, interaction::InteractionExt, option::{OptionBuilder}, response::ResponseBuilder
        }, structs::config::extension::ExtensionConfig, traits::namespaces::KvExt}, 
    ui::embeds::{
        default::DEFAULT_EMBED
    }
};

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

        let generators = &mut extension
            .config
            .get_or_insert_default()
            .generators;

        let channel_string = channel.to_string();

        if generators.contains(&channel_string) {
            return Err(Error::InteractionFailed(format!("This channel is already set as channel generator!")));
        }

        generators.push(channel_string);

        guild_kv
            .put(&key, serde_json::to_string(&extension)?, None)
            .await?;

        let embed = DEFAULT_EMBED.clone()
            .description("Channel set as a voice channel generator!")
            .build();

        let response = ResponseBuilder::new(InteractionResponseType::ChannelMessageWithSource)
            .embeds(vec![embed])
            .ephemeral()
            .build();

        Ok(interaction.edit(&response).await?)
    }
}
