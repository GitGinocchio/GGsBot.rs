use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::discord::{channel::ChannelType, locale::Locale};


#[derive(Serialize_repr, Deserialize_repr, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
/// https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-type
pub(crate) enum ApplicationCommandOptionType {
    SubCommand = 1,
    SubCommandGroup = 2,
    String = 3,
    Integer = 4,
    Boolean = 5,
    User = 6,
    Channel = 7,
    Role = 8,
    Mentionable = 9,
    Number = 10,
    Attachment = 11
}

impl Default for ApplicationCommandOptionType {
    fn default() -> Self { Self::String }
}

#[derive(Deserialize, Serialize, Clone, Default)]
/// https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-structure
pub(crate) struct ApplicationCommandOption {
    pub(crate) name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) name_localizations: Option<HashMap<Locale, String>>,

    pub(crate) description: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) description_localizations: Option<HashMap<Locale, String>>,

    #[serde(rename = "type")]
    pub(crate) ty: ApplicationCommandOptionType,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) choices: Option<Vec<ApplicationCommandOptionChoice>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) options: Option<Vec<ApplicationCommandOption>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) channel_types: Option<Vec<ChannelType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) min_value: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) max_value: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) min_length: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) max_length: Option<u64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) autocomplete: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) required: Option<bool>
}

#[derive(Deserialize, Serialize, Clone, Debug)]
/// https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-choice-structure
pub(crate) struct ApplicationCommandOptionChoice {
    pub(crate) name: String,
    pub(crate) value: String

}