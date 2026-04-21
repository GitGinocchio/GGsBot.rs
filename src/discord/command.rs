use std::collections::HashMap;
use serde::{Serialize, Serializer};
use twilight_model::{application::command::{Command as DiscordCommand, CommandOption, CommandOptionType, CommandType}, guild::Permissions, oauth::ApplicationIntegrationType};
use async_trait::async_trait;
use twilight_model::{application::interaction::{Interaction, application_command::{CommandData, CommandOptionValue}}, http::interaction::InteractionResponse, id::Id};
use worker::RouteContext;

use crate::{error::Error, traits::command::CommandController};
use crate::handle_subcommands;

pub type CommandMap = HashMap<String, Box<dyn Command + Send + Sync>>;

pub trait CommandDataExt {
    fn get_option(&self, name: &str) -> Option<&CommandOptionValue>;
    fn get_subcommand_name(&self) -> Option<&str>;
    fn get_subcommand_data(&self) -> Option<CommandData>;
}

impl CommandDataExt for CommandData {
    fn get_option(&self, name: &str) -> Option<&CommandOptionValue> {
        self.options
            .iter()
            .find(|opt| opt.name == name)
            .map(|o| &o.value)
    }

    fn get_subcommand_name(&self) -> Option<&str> {
        self.options.iter().find_map(|opt| {
            match opt.value {
                CommandOptionValue::SubCommand(_) | CommandOptionValue::SubCommandGroup(_) => {
                    Some(opt.name.as_str())
                }
                _ => None,
            }
        })
    }

    fn get_subcommand_data(&self) -> Option<CommandData> {
        self.options.iter().find_map(|opt| {
            if let CommandOptionValue::SubCommand(sub_options) = &opt.value {
                let mut sub_data = self.clone();
                sub_data.options = sub_options.clone();
                Some(sub_data)
            } else {
                None
            }
        })
    }
}

#[async_trait(?Send)]
pub trait Command {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn subcommands(&self) -> CommandMap { HashMap::new() }

    /// add any arguments/choices here, more info at https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-structure
    fn options(&self) -> Vec<CommandOption> { vec![] }

    fn integration_types(&self) -> Vec<ApplicationIntegrationType> { vec![] }

    fn default_member_permissions(&self) -> Option<Permissions> { None }
    
    async fn respond(
        &self, 
        interaction: &Interaction,
        data: &CommandData,
        ctx: &mut RouteContext<()>
    ) -> Result<InteractionResponse, Error> {
        handle_subcommands!(self, data, interaction, ctx)
    }

    #[allow(unused_variables)]
    async fn autocomplete(
        &self, 
        data: &CommandData, 
        ctx: &mut RouteContext<()>
    ) -> Result<Option<InteractionResponse>, Error> { Ok(None) }

    #[allow(unused)]
    fn get_controller(&self) -> Option<&dyn CommandController> { None }
}

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
                    autocomplete: None,
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

        let itypes = self.0.integration_types();

        let discord_cmd = DiscordCommand {
            name: self.0.name(),
            description: self.0.description(),
            options: all_options,
            kind: CommandType::ChatInput,
            application_id: None,
            default_member_permissions: self.0.default_member_permissions(),
            guild_id: None,
            id: None,
            nsfw: None,
            version: Id::new(1),
            name_localizations: None,
            description_localizations: None,
            contexts: None,
            #[allow(deprecated)]
            dm_permission: None,
            integration_types: if itypes.is_empty() { None } else { Some(itypes) }
        };

        discord_cmd.serialize(serializer)
    }
}

#[macro_export]
macro_rules! build_commands {
    ($($command_type:ty),*) => {
        {
            #[allow(unused_mut)]
            let mut map: $crate::discord::command::CommandMap = std::collections::HashMap::new();
            $(
                let cmd: Box<dyn $crate::discord::command::Command + Send + Sync> = 
                    Box::new(<$command_type>::default()); 
                
                map.insert(cmd.name(), cmd);
            )*
            map
        }
    };
}

#[macro_export]
macro_rules! handle_subcommands {
    ($self:expr, $data:expr, $interaction:expr, $ctx:expr) => {
        {
            let sub_name = $data.get_subcommand_name().ok_or(Error::Generic("Could not get subcommand_name".into()))?;
            let sub_data = $data.get_subcommand_data().ok_or(Error::Generic("Could not get subcommand_data".into()))?;

            let subs = $self.subcommands();
            if let Some(sub_cmd) = subs.get(sub_name) {
                return sub_cmd.respond($interaction, &sub_data, $ctx).await;
            }

            Err(Error::Generic("No response was created for this subcommand!".into()))
        }
    };
}