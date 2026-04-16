use std::collections::HashMap;
use async_trait::async_trait;

use crate::discord::integration::ApplicationIntegrationType;
use crate::discord::interaction::*;
use crate::discord::error::InteractionError;
use crate::discord::locale::Localization;
use crate::discord::member::Member;
use crate::discord::option::{ApplicationCommandOption, ApplicationCommandOptionType};
use crate::discord::user::User;

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct CommandContext<'a> {
    pub(crate) options: Option<Vec<ApplicationCommandInteractionDataOption>>,
    pub(crate) guild_id: Option<String>,
    pub(crate) channel_id: Option<String>,
    pub(crate) user: Option<User>,
    pub(crate) member: Option<Member>,
    pub(crate) worker: &'a mut worker::RouteContext<()>,
}

#[allow(dead_code)]
impl CommandContext<'_> {
    pub fn get_subcommand_name(&self) -> Option<&str> {
        self.options.as_ref()?
            .iter()
            .find(|opt| opt.ty == ApplicationCommandOptionType::SubCommand) // Cerca il tipo SUB_COMMAND
            .map(|opt| opt.name.as_str())
    }

    pub fn create_subcommand_context(&mut self) -> CommandContext<'_> {
        let sub_options = self.options.as_ref()
            .and_then(|opts| {
                opts.iter()
                    .find(|opt| opt.ty == ApplicationCommandOptionType::SubCommand)
                    .and_then(|opt| opt.options.clone()) 
            });

        CommandContext {
            options: sub_options,
            guild_id: self.guild_id.clone(),
            channel_id: self.channel_id.clone(),
            user: self.user.clone(),
            member: self.member.clone(),
            worker: self.worker, 
        }
    }

    pub fn get_option(&self, name: &str) -> Option<&serde_json::Value> {
        match &self.options {
            Some(options) => {
                for option in options {
                    if option.name == name {
                        match option.value {
                            Some(ref value) => return Some(value),
                            None => return None
                        }
                    }
                }
                None
            },
            None => None
        }
    }

    pub fn get_env(&self, key: &str) -> Option<String> {
        self.worker.env.var(key).map(|b| b.to_string()).ok()
    }

    pub async fn kv_get(&self, namespace: &str, key: &str) -> Result<Option<String>, InteractionError> {
        let kv = self.worker.kv(namespace).map_err( |_|InteractionError::WorkerError("Bind to kv".into()))?;
        let value = kv.get(key).text().await.map_err( |_|InteractionError::WorkerError("Fetching from KV".into()))?;
        Ok(value)
    }

    pub async fn kv_put(&self, namespace: &str, key: &str, value: &str) -> Result<(), InteractionError> {
        let kv = self.worker.kv(namespace).map_err( |_|InteractionError::WorkerError("bind to kv".into()))?;
        kv.put(key, value)
        .map_err( |_|InteractionError::WorkerError("bind to KV".into()))?
        .execute()
        .await
        .map_err(|_| InteractionError::WorkerError("KV put".into()))
        ?;
        Ok(())
    }

    pub fn admin_or_bail(&self) -> Option<InteractionApplicationCommandCallbackData> {
        match &self.member {
            Some(member) => {
                if member.is_admin() {
                    None
                } else {
                    Some(InteractionApplicationCommandCallbackData {
                        content: Some("You must be an admin to use this command!".to_string()),
                        choices: None,
                        embeds: None,
                        ..Default::default()
                    })
                }
            },
            None => Some(InteractionApplicationCommandCallbackData {
                content: Some("You must use this command inside a discord server.".to_string()),
                choices: None,
                embeds: None,
                ..Default::default()
            })
        }
    }

}

#[async_trait(?Send)]
pub(crate) trait Command: Send + Sync {
    fn name(&self) -> Localization;
    fn description(&self) -> Localization;

    fn subcommands(&self) -> CommandMap { HashMap::new() }

    /// add any arguments/choices here, more info at https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-structure
    fn options(&self) -> Vec<ApplicationCommandOption> { vec![] }

    fn integration_types(&self) -> Vec<ApplicationIntegrationType> { vec![] }

    async fn respond(&self, ctx: &mut CommandContext) -> Result<InteractionApplicationCommandCallbackData, InteractionError> {
        worker::console_debug!("Context: {ctx:?}");
        
        let sub_name = ctx.get_subcommand_name()
            .ok_or(InteractionError::GenericError())?;

        let subs = self.subcommands();
        
        if let Some(sub_cmd) = subs.get(sub_name) {
            let mut sub_ctx = ctx.create_subcommand_context();
            
            return sub_cmd.respond(&mut sub_ctx).await;
        }

        // Se il comando non è nella mappa, qualcosa non va nella registrazione
        worker::console_error!("Sottocomando '{}' non trovato nella mappa di Mods", sub_name);
        Err(InteractionError::GenericError())
    }
    async fn autocomplete(&self, _ctx: &CommandContext) -> Result<Option<InteractionApplicationCommandCallbackData>, InteractionError> { Ok(None) }
}

pub struct SerializableCommand<'a>(pub &'a dyn Command);

impl<'a> serde::Serialize for SerializableCommand<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        use serde::ser::SerializeStruct;
        
        let name_loc = self.0.name();
        let desc_loc = self.0.description();

        let mut state = serializer.serialize_struct("Command", 5)?;
        
        state.serialize_field("name", &name_loc.get_default())?;
        state.serialize_field("description", &desc_loc.get_default())?;

        if let Some(map) = name_loc.get_map() {
            state.serialize_field("name_localizations", map)?;
        }
        if let Some(map) = desc_loc.get_map() {
            state.serialize_field("description_localizations", map)?;
        }
        
        let integration_types = self.0.integration_types();
        if !integration_types.is_empty() {
            state.serialize_field("integration_types", &integration_types)?;
        }

        let mut all_options = self.0.options();
        
        let subs = self.0.subcommands();
        if !subs.is_empty() {
            for sub in subs.values() {
                let sub_options = sub.options();
                all_options.push(ApplicationCommandOption {
                    ty: ApplicationCommandOptionType::SubCommand,
                    name: sub.name().get_default(),
                    description: sub.description().get_default(),
                    options: if sub_options.is_empty() { None } else { Some(sub_options) },
                    ..Default::default()
                });
            }
        }

        if !all_options.is_empty() {
            state.serialize_field("options", &all_options)?;
        }

        state.end()
    }
}

pub type CommandMap = HashMap<String, Box<dyn Command>>;

#[macro_export]
macro_rules! build_commands {
    ($($command_type:ty),*) => {
        {
            let mut map: $crate::discord::command::CommandMap = std::collections::HashMap::new();
            $(
                let cmd: Box<dyn $crate::discord::command::Command + Send + Sync> = 
                    Box::new(<$command_type>::default()); 
                
                map.insert(cmd.name().get_default(), cmd);
            )*
            map
        }
    };
}