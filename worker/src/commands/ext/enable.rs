use async_trait::async_trait;
use serde_json::Value;
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
    error::Error, 
    framework::{
        discord::{
            autocomplete::Autocomplete, 
            command::{Command, CommandDataExt}, 
            option::OptionBuilder, 
            response::InteractionResponseExt
        }, 
        structs::config::extension::ExtensionConfig, 
        traits::namespaces::KvExt
    }, 
    ui::embeds::{
        default::DEFAULT_EMBED, 
        error::ERROR_EMBED
    }
};

#[derive(Default)]
pub(crate) struct Enable {}

#[async_trait(?Send)]
impl Command for Enable {
    fn name(&self) -> String {
        "enable".into()
    }

    fn description(&self) -> String {
        "Abilita un estensione del bot sul server!".into()
    }

    fn options(&self) -> Vec<CommandOption> {
        let ext = OptionBuilder::new(
            CommandOptionType::String,
            "extension",
            "L'estensione da abilitare",
        )
        .autocomplete(true)
        .required(true)
        .build();

        vec![ext]
    }

    async fn autocomplete(
        &self,
        interaction: &Interaction,
        data: &CommandData,
        ctx: &mut RouteContext<()>,
    ) -> Result<Option<InteractionResponse>, Error> {
        let guild_kv = interaction.guild_kv(&ctx.env)?;

        let mut autocomplete = Autocomplete::new();

        let user_input = match data.get_option("extension") {
            Some(CommandOptionValue::String(val)) => val.to_lowercase(),
            _ => "".to_string(),
        };

        let extensions_keys: Vec<String> = guild_kv
            .list(Some(format!("extensions:{user_input}")), Some(25), None)
            .await?
            .keys
            .iter_mut()
            .filter(|k| !k.name.ends_with(":config:pending"))
            .map(|k| k.name.clone())
            .collect();

        let extensions = guild_kv.get_json_bulk::<ExtensionConfig<Value>>(&extensions_keys).await?;

        for (ext, maybe_config) in extensions {
            let Some(config) = maybe_config else { continue; };

            let ext_name = match ext.split(":").nth(3) {
                Some(name) => name.to_string(),
                None => continue
            };

            if !config.enabled {
                autocomplete.add_choice(
                    ext_name.to_string(), 
                    CommandOptionChoiceValue::String(ext_name),
                    None
                );
            }
        }

        worker::console_debug!("choices:{:?}", autocomplete.get_choices());

        Ok(Some(autocomplete.build()))
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

        let description = if !config.enabled {
            config.set_enabled(true);
            let serialized = serde_json::to_string(&config).map_err(|e| Error::JsonFailed(e))?;
            guild_kv.put(&key, serialized, None).await?;
            format!("Extension {ext} enabled successfully!")
        } else {
            format!("Extension {ext} already enabled!")
        };

        let embed = DEFAULT_EMBED.clone().description(description).build();

        response.set_embeds(vec![embed]);

        Ok(response)
    }
}
