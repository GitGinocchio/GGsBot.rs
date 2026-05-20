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
    commands::ext::REQUIRED_EXTENSIONS, 
    constants::COMMANDS, 
    error::Error, framework::{
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
pub(crate) struct Setup {}

#[async_trait(?Send)]
impl Command for Setup {
    fn name(&self) -> String {
        "setup".into()
    }

    fn description(&self) -> String {
        "Configura un estensione del bot sul server!".into()
    }

    fn options(&self) -> Vec<CommandOption> {
        let ext = OptionBuilder::new(
            CommandOptionType::String,
            "extension",
            "L'estensione da aggiungere",
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

        let keys_to_check: Vec<String> = COMMANDS
            .iter()
            .filter(|(name, command)| {
                command.get_controller().is_some() &&
                !REQUIRED_EXTENSIONS.contains(&name.as_str()) && 
                if !user_input.is_empty() { name.to_lowercase().contains(&user_input) } else { true }
            })
            .map(|(name, _)| format!("extensions:{name}"))
            .collect();

        let results = guild_kv
            .get_json_bulk::<ExtensionConfig<Value>>(&keys_to_check)
            .await?;

        for name in keys_to_check {
            let is_selectable = match results.get(&name) {
                Some(Some(_)) => false,
                Some(None) => true,
                None => true
            };

            let ext_name = match name.split(":").nth(3) {
                Some(name) => name.to_string(),
                None => continue
            };

            if is_selectable {
                let was_added = autocomplete.add_choice(
                    ext_name.clone(),
                    CommandOptionChoiceValue::String(ext_name),
                    None,
                );

                if !was_added {
                    break;
                }
            }
        }

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

        let cmd_controller = match COMMANDS.get(ext) {
            Some(cmd) => Ok(cmd.get_controller()),
            None => Err(Error::InteractionFailed(
                "Command has no CommandController trait!".into(),
            )),
        }?;

        let mut response =
            InteractionResponse::new(InteractionResponseType::ChannelMessageWithSource);
        response.set_ephemeral();

        let existing_key = format!("extensions:{ext}:config"); //guilds:{guild_id}:extensions:{ext_name}:config
        let config = guild_kv.get(&existing_key).await.map_err(|e| Error::KvError(e))?;

        if config.is_some() {
            let embed = ERROR_EMBED
                .clone()
                .description(format!(
                    "Extension {ext} is already configured for this server!"
                ))
                .build();

            response.set_embeds(vec![embed]);
            return Ok(response);
        }

        let config = if let Some(controller) = cmd_controller {
            controller.get_default_config(interaction, ctx).await
        } else {
            None
        };

        let default_config = ExtensionConfig::new(config);
        let serialized_config =
            serde_json::to_string(&default_config).map_err(|e| Error::JsonFailed(e))?;

        let pending_key = format!("extensions:{ext}:config:pending"); //guilds:{guild_id}:extensions:{ext_name}:config:pending
        guild_kv
            .put(&pending_key, serialized_config, Some(3600))
            .await
            .map_err(|e| Error::KvError(e))?;

        if let Some(controller) = cmd_controller {
            if let Some(response) = controller.on_setup(interaction, ctx).await {
                return response;
            }
        }

        let embed = DEFAULT_EMBED
            .clone()
            .description(format!("Extension {ext} configured successfully!"))
            .build();

        response.set_embeds(vec![embed]);

        Ok(response)
    }
}
