use std::collections::HashMap;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};

use crate::discord::interaction::*;
use crate::discord::error::InteractionError;

#[allow(dead_code)]
pub(crate) struct CommandContext<'a> {
    pub(crate) options: Option<Vec<ApplicationCommandInteractionDataOption>>,
    pub(crate) guild_id: Option<String>,
    pub(crate) channel_id: Option<String>,
    pub(crate) user: Option<User>,
    pub(crate) member: Option<Member>,
    pub(crate) worker: &'a mut worker::RouteContext<()>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ApplicationIntegrationType {
    GuildInstall,
    UserInstall
}

#[allow(dead_code)]
impl CommandContext<'_> {
    pub fn get_option(&self, name: &str) -> Option<&str> {
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
    fn name(&self) -> String;
    fn description(&self) -> String;

    /// add any arguments/choices here, more info at https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-structure
    fn options(&self) -> Option<Vec<ApplicationCommandOption>> { None }

    fn integration_types(&self) -> Option<Vec<ApplicationIntegrationType>> { None }

    async fn respond(&self, ctx: &CommandContext) -> Result<InteractionApplicationCommandCallbackData, InteractionError>;
    async fn autocomplete(&self, _ctx: &CommandContext) -> Result<Option<InteractionApplicationCommandCallbackData>, InteractionError> { Ok(None) }
}

pub struct SerializableCommand<'a>(pub &'a dyn Command);

impl<'a> serde::Serialize for SerializableCommand<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Command", 3)?;
        state.serialize_field("name", &self.0.name())?;
        state.serialize_field("description", &self.0.description())?;
        state.serialize_field("options", &self.0.options())?;
        state.serialize_field("integration_types", &self.0.integration_types())?;
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
                // Usiamo le parentesi angolari per disambiguare il tipo
                // e istanziamo la struct. Nota: funziona solo se la struct è {}
                let cmd: Box<dyn $crate::discord::command::Command + Send + Sync> = 
                    Box::new(<$command_type>::default()); 
                
                map.insert(cmd.name(), cmd);
            )*
            map
        }
    };
}