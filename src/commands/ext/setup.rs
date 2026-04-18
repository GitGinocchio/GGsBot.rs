use async_trait::async_trait;
use twilight_model::{application::{command::{CommandOption, CommandOptionChoiceValue, CommandOptionType}, interaction::{Interaction, application_command::{CommandData, CommandOptionValue}}}, http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType}};
use worker::RouteContext;

use crate::{
    COMMANDS, commands::ext::REQUIRED_EXTENSIONS, discord::{
        command::{
            Command, 
            CommandDataExt
        }, 
        option::{CommandOptionExt, OptionBuilder}, 
        response::InteractionResponseExt
    }, embeds::{default::DEFAULT_EMBED, error::ERROR_EMBED}, error::InteractionError, structs::config::extension::ExtensionConfig, traits::namespaces::InteractionKvExt
};

#[derive(Default)]
pub(crate) struct Setup {
}

#[async_trait(?Send)]
impl Command for Setup {
    fn name(&self) -> String { "setup".into() }

    fn description(&self) -> String { "Configura un estensione del bot sul server!".into() }

    fn options(&self) -> Vec<CommandOption> {
        let mut ext = OptionBuilder::new(CommandOptionType::String, "extension", "L'estensione da aggiungere")
            .required(true)
            .build();

        for (name, _) in COMMANDS.iter() {
            if REQUIRED_EXTENSIONS.contains(&name.as_str()) { continue };
            ext.add_choice(name, CommandOptionChoiceValue::String(name.clone()));
        }

        vec![ext]
    }

    async fn respond(
        &self, 
        interaction: &Interaction,
        data: &CommandData,
        ctx: &mut RouteContext<()>
    ) -> Result<InteractionResponse, InteractionError> {
        let guild_kv = interaction.guild_kv(ctx)?;
        let ext = match data.get_option("extension") {
            Some(CommandOptionValue::String(ext)) => Ok(ext),
            Some(_) | None => Err(InteractionError::GenericError())
        }?;

        let cmd_controller = match COMMANDS.get(ext) {
            Some(cmd) => Ok(cmd.get_controller()),
            None => Err(InteractionError::UnknownCommand(format!("Extension {ext} not found!")))
        }?;

        let mut response = InteractionResponse::new(InteractionResponseType::ChannelMessageWithSource);
        response.set_ephemeral();

        let key = format!("extensions:{ext}:config"); //guilds:{guild_id}:extensions:{ext_name}:config
        let config = guild_kv.get(&key).await.map_err(|e| InteractionError::KvError(e))?;
        
        if config.is_some() {
            let embed = ERROR_EMBED.clone()
                .description(format!("Extension {ext} is already configured for this server!"))
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
        let serialized_config = serde_json::to_string(&default_config)
            .map_err(|e| InteractionError::JsonError(e))?;

        guild_kv.put(&key, serialized_config, None)
            .await
            .map_err(|e| InteractionError::KvError(e))?;

        if let Some(controller) = cmd_controller {
            if let Some(response) = controller.on_setup(interaction, ctx).await {
                return response;
            }
        }

        let embed = DEFAULT_EMBED.clone()
            .description(format!("Extension {ext} configured successfully!"))
            .build();

        response.set_embeds(vec![embed]);

        Ok(response)
    }
}