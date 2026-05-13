use async_trait::async_trait;
use twilight_model::{
    application::{
        command::{CommandOption, CommandOptionChoiceValue, CommandOptionType},
        interaction::{
            Interaction,
            application_command::{CommandData, CommandOptionValue},
        },
    },
    http::interaction::{InteractionResponse, InteractionResponseType},
};
use worker::RouteContext;

use crate::{
    COMMANDS,
    commands::ext::REQUIRED_EXTENSIONS,
    error::Error,
    framework::discord::{
        command::{Command, CommandDataExt},
        option::{CommandOptionExt, OptionBuilder},
        response::InteractionResponseExt,
    },
    framework::structs::config::extension::ExtensionConfig,
    framework::traits::namespaces::KvExt,
    ui::embeds::{default::DEFAULT_EMBED, error::ERROR_EMBED},
};

#[derive(Default)]
pub(crate) struct Disable {}

#[async_trait(?Send)]
impl Command for Disable {
    fn name(&self) -> String {
        "disable".into()
    }

    fn description(&self) -> String {
        "Disabilita un estensione del bot sul server!".into()
    }

    fn options(&self) -> Vec<CommandOption> {
        let mut ext = OptionBuilder::new(
            CommandOptionType::String,
            "extension",
            "L'estensione da disabilitare",
        )
        .required(true)
        .build();

        for (name, _) in COMMANDS.iter() {
            if REQUIRED_EXTENSIONS.contains(&name.as_str()) {
                continue;
            };
            ext.add_choice(name, CommandOptionChoiceValue::String(name.clone()));
        }

        vec![ext]
    }

    async fn respond(
        &self,
        interaction: &Interaction,
        data: &CommandData,
        ctx: &mut RouteContext<()>,
    ) -> Result<InteractionResponse, Error> {
        let guild_kv = interaction.guild_kv(&ctx.env)?;
        let ext = match data.get_option("extension") {
            Some(CommandOptionValue::String(ext)) => Ok(ext),
            Some(_) | None => Err(Error::InteractionFailed(
                "Missing required option 'extension'".into(),
            )),
        }?;

        let mut response =
            InteractionResponse::new(InteractionResponseType::ChannelMessageWithSource);
        response.set_ephemeral();

        let key = format!("extensions:{ext}:config"); //guilds:{guild_id}:extensions:{ext_name}:config
        let maybe_config = guild_kv.get(&key).await.map_err(|e| Error::KvError(e))?;

        let mut config: ExtensionConfig<serde_json::Value> = if let Some(serialized) = maybe_config
        {
            serde_json::from_str(&serialized).map_err(|e| Error::JsonFailed(e))?
        } else {
            let embed = ERROR_EMBED
                .clone()
                .description(format!(
                    "Extension {ext} is not configured for this server!"
                ))
                .build();

            response.set_embeds(vec![embed]);
            return Ok(response);
        };

        let description = if config.enabled {
            config.set_enabled(false);
            let serialized = serde_json::to_string(&config).map_err(|e| Error::JsonFailed(e))?;
            guild_kv.put(&key, serialized, None).await?;
            format!("Extension {ext} disabled successfully!")
        } else {
            format!("Extension {ext} already disabled!")
        };

        let embed = DEFAULT_EMBED.clone().description(description).build();

        response.set_embeds(vec![embed]);

        Ok(response)
    }
}
