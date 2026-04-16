use std::collections::HashMap;

use async_trait::async_trait;
use twilight_model::{application::interaction::{Interaction, application_command::{CommandData, CommandOptionValue}}, http::interaction::InteractionResponse, id::Id};
use worker::RouteContext;

use crate::error::InteractionError;

pub type CommandMap = HashMap<String, Box<dyn Command + Send + Sync>>;

pub trait CommandDataExt {
    fn get_option(&self, name: &str) -> Option<&CommandOptionValue>;
}

impl CommandDataExt for CommandData {
    fn get_option(&self, name: &str) -> Option<&CommandOptionValue> {
        self.options
            .iter()
            .find(|opt| opt.name == name)
            .map(|o| &o.value)
    }
}

#[async_trait(?Send)]
pub trait Command {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn autocomplete(&self) -> Option<bool> { None }
    fn subcommands(&self) -> CommandMap { HashMap::new() }

    /// add any arguments/choices here, more info at https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-structure
    fn options(&self) -> Vec<CommandOption> { vec![] }

    async fn respond(
        &self, 
        interaction: &Interaction,
        data: &CommandData,
        ctx: &mut RouteContext<()>
    ) -> Result<InteractionResponse, InteractionError>;
}

use serde::{Serialize, Serializer};
use twilight_model::application::command::{Command as DiscordCommand, CommandOption, CommandOptionType, CommandType};

pub struct SerializableCommand<'a>(pub &'a dyn Command);

impl<'a> Serialize for SerializableCommand<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut all_options = self.0.options();

        let subs = self.0.subcommands();
        if !subs.is_empty() {
            for sub in subs.values() {
                all_options.push(CommandOption {
                    kind: CommandOptionType::SubCommand,
                    name: sub.name(),
                    description: sub.description(),
                    options: Some(sub.options()), 
                    autocomplete: sub.autocomplete(),
                    channel_types: None,
                    choices: None,
                    description_localizations: None,
                    max_length: None,
                    max_value: None,
                    min_length: None,
                    min_value: None,
                    name_localizations: None,
                    required: None
                });
            }
        }

        let discord_cmd = DiscordCommand {
            name: self.0.name(),
            description: self.0.description(),
            options: all_options,
            kind: CommandType::ChatInput,
            application_id: None,
            default_member_permissions: None,
            guild_id: None,
            id: None,
            nsfw: None,
            version: Id::new(1),
            name_localizations: None,
            description_localizations: None,
            contexts: None,
            dm_permission: None,
            integration_types: None
        };

        discord_cmd.serialize(serializer)
    }
}

#[macro_export]
macro_rules! build_commands {
    ($($cmd:ty),*) => {
        {
            use std::collections::HashMap;
            use crate::discord::command::Command;

            let mut map: CommandMap = HashMap::new();
            $(
                let cmd_obj = <$cmd>::default();
                map.insert(cmd_obj.name(), Box::new(cmd_obj));
            )*
            map
        }
    };
}