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
    framework::traits::namespaces::KvExt,
    ui::embeds::{default::DEFAULT_EMBED, error::ERROR_EMBED},
};

#[derive(Default)]
pub(crate) struct Teardown;

#[async_trait(?Send)]
impl Command for Teardown {
    fn name(&self) -> String {
        "teardown".into()
    }

    fn description(&self) -> String {
        "Rimuovi un estensione del bot dal server!".into()
    }

    fn options(&self) -> Vec<CommandOption> {
        let mut ext = OptionBuilder::new(
            CommandOptionType::String,
            "extension",
            "L'estensione da rimuovere",
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

    // TODO: aggiungere un autocomplete per inserire solo i comandi che sono configurati attualmente

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

        let key = format!("extensions:{ext}:config"); //guilds:{guild_id}:extensions:{ext_name}:config
        let config = guild_kv.get(&key).await.map_err(|e| Error::KvError(e))?;

        if config.is_none() {
            let embed = ERROR_EMBED
                .clone()
                .description(format!(
                    "Extension {ext} is not configured for this server!"
                ))
                .build();

            response.set_embeds(vec![embed]);
            return Ok(response);
        }

        guild_kv.delete(&key).await.map_err(|e| Error::KvError(e))?;
        guild_kv.delete(&format!("{key}:pending")).await.map_err(|e| Error::KvError(e))?;

        if let Some(controller) = cmd_controller {
            if let Some(response) = controller.on_teardown(interaction, ctx).await {
                return response;
            }
        }

        let embed = DEFAULT_EMBED
            .clone()
            .description(format!("Extension {ext} removed successfully!"))
            .build();

        response.set_embeds(vec![embed]);

        Ok(response)
    }
}
