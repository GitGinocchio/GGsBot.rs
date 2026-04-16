use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::COMMANDS;
use crate::discord::attachment::Attachment;
use crate::discord::command::CommandContext;
use crate::discord::error::{Error, InteractionError};
use crate::discord::embed::Embed;
use crate::discord::locale::Locale;
use crate::discord::member::Member;
use crate::discord::message::MessageFlags;
use crate::discord::option::{ApplicationCommandOptionChoice, ApplicationCommandOptionType};
use crate::discord::user::User;

#[derive(Deserialize_repr, Serialize)]
#[repr(u8)]
enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
    MessageComponent = 3,
    ApplicationCommandAutoComplete = 4,
    ModalSubmit = 5
}

#[allow(dead_code)]
#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub(crate) enum InteractionResponseType {
    Pong = 1,
    // Acknowledge = 2,
    // ChannelMessage = 3,
    ChannelMessageWithSource = 4,
    ACKWithSource = 5,
    AutoCompleteResult = 8
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct ApplicationCommandInteractionDataOption {
    pub(crate) name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) name_localizations: Option<HashMap<Locale, String>>,

    #[serde(rename = "type")]
    pub(crate) ty: ApplicationCommandOptionType,

    pub(crate) value: Option<serde_json::Value>,
    pub(crate) focused: Option<bool>,

    pub(crate) options: Option<Vec<ApplicationCommandInteractionDataOption>>
}

#[derive(Deserialize, Serialize)]
pub(crate) struct ApplicationCommandInteractionData {
    pub(crate) name: String,
    pub(crate) options: Option<Vec<ApplicationCommandInteractionDataOption>>
}

#[derive(Serialize, Default)]
pub(crate) struct InteractionApplicationCommandCallbackData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) content: Option<String>,

    pub(crate) choices: Option<Vec<ApplicationCommandOptionChoice>>,

    pub(crate) embeds: Option<Vec<Embed>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) flags: Option<MessageFlags>,

    // Usiamo skip_serializing_if anche per i vettori se sono vuoti
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) attachments: Vec<Attachment>,

    //#[serde(skip_serializing_if = "Option::is_none")]
    //pub(crate) poll: Option<bool>, // Nota: l'oggetto Poll di Discord è più complesso di un bool

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tts: Option<bool>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Interaction {
    #[serde(rename = "type")]
    ty: InteractionType,
    data: Option<ApplicationCommandInteractionData>,
    token: String,
    guild_id: Option<String>,
    channel_id: Option<String>,
    user: Option<User>,
    member: Option<Member>,
}

impl Interaction {
    fn data(&self) -> Result<&ApplicationCommandInteractionData, Error> {
        self
            .data
            .as_ref()
            .ok_or_else(|| Error::InvalidPayload("data not found".to_string()))
    }
}

#[derive(Serialize)]
pub struct InteractionResponse {
    #[serde(rename = "type")]
    pub(crate) ty: InteractionResponseType,
    pub(crate) data: Option<InteractionApplicationCommandCallbackData>,
}

impl Interaction {
    pub(crate) fn handle_ping(&self) -> InteractionResponse {
        InteractionResponse {
            ty: InteractionResponseType::Pong,
            data: None,
        }
    }

    pub(crate) async fn handle_command(&self, ctx: &mut worker::RouteContext<()>) -> Result<InteractionResponse, InteractionError> {
        let data = self.data().map_err(|_| InteractionError::GenericError())?;

        // Prepariamo il contesto
        let mut command_input = CommandContext {
            options: data.options.clone(),
            guild_id: self.guild_id.clone(),
            channel_id: self.channel_id.clone(),
            user: self.user.clone(),
            member: self.member.clone(),
            worker: ctx
        };

        if let Some(command) = COMMANDS.get(data.name.as_str()) {
            let response = command.respond(&mut command_input).await?;

            Ok(InteractionResponse {
                ty: InteractionResponseType::ChannelMessageWithSource,
                data: Some(response),
            })
        } else {
            Err(InteractionError::UnknownCommand(data.name.clone()))
        }
    }

    pub(crate) async fn handle_autocomplete(&self, ctx: &mut worker::RouteContext<()>) -> Result<InteractionResponse, InteractionError> {
        let data = self.data().map_err(|_| InteractionError::GenericError())?;

        let command_input = CommandContext {
            options: data.options.clone(),
            guild_id: self.guild_id.clone(),
            channel_id: self.channel_id.clone(),
            user: self.user.clone(),
            member: self.member.clone(),
            worker: ctx
        };

        if let Some(command) = COMMANDS.get(data.name.as_str()) {
            let response = command.autocomplete(&command_input).await?;

            Ok(InteractionResponse {
                ty: InteractionResponseType::AutoCompleteResult,
                data: response,
            })
        } else {
            Err(InteractionError::UnknownCommand(data.name.clone()))
        }
    }

    pub(crate) async fn perform(&self, ctx: &mut worker::RouteContext<()>) -> Result<InteractionResponse, Error> {
        match self.ty {
            InteractionType::Ping => Ok(self.handle_ping()),
            InteractionType::ApplicationCommand => self.handle_command(ctx).await.map_err(Error::InteractionFailed),
            InteractionType::ApplicationCommandAutoComplete => self.handle_autocomplete(ctx).await.map_err(Error::InteractionFailed),
            _ => Err(Error::InvalidPayload("Not implemented".into()))
        }
    }
}