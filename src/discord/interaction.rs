use async_trait::async_trait;
use serde_json::json;
use twilight_model::{
    application::interaction::{
        Interaction, InteractionData, InteractionType
    }, 
    http::interaction::{
        InteractionResponse, InteractionResponseType
    }
};
use worker::RouteContext;

use crate::{CLIENT, COMMANDS, UIHANDLERS, discord::response::InteractionResponseExt, error::Error};

#[async_trait(?Send)]
#[allow(unused)]
pub trait InteractionExt {
    async fn handle_autocomplete(&self, ctx: &mut RouteContext<()>) -> Result<InteractionResponse, Error>;
    async fn handle_component(&self, ctx: &mut RouteContext<()>) -> Result<InteractionResponse, Error>;
    async fn handle_modal_submit(&self, ctx: &mut RouteContext<()>) -> Result<InteractionResponse, Error>;
    async fn handle_command(&self, ctx: &mut RouteContext<()>) -> Result<InteractionResponse, Error>;
    
    async fn perform(&self, ctx: &mut RouteContext<()>) -> Result<InteractionResponse, Error>;

    async fn delete(&self, ctx: &mut RouteContext<()>) -> Result<InteractionResponse, Error>;
    async fn edit(&self, response: &InteractionResponse) -> Result<InteractionResponse, Error>;
    async fn defer(&self) -> Result<(), Error>;
    async fn followup(&self) -> Result<(), Error>;

    fn is_dev(&self, ctx: &mut RouteContext<()>) -> bool;
}

#[async_trait(?Send)]
#[allow(unused)]
impl InteractionExt for Interaction {
    async fn handle_autocomplete(&self, ctx: &mut RouteContext<()>) -> Result<InteractionResponse, Error> {
        let data = match self.data.as_ref() {
            Some(InteractionData::ApplicationCommand(data)) => data,
            _ => return Err(Error::InvalidPayload("Missing autocomplete data".into())),
        };

        if let Some(command) = COMMANDS.get(data.name.as_str()) {
            match command.autocomplete(data, ctx).await {
                Ok(Some(response)) => return Ok(response),
                Ok(None) => return Err(Error::InteractionFailed("No autocomplete response supplied".into())),
                Err(e) => return Err(e),
            }
        } else {
            Err(Error::InvalidPayload(format!("Command '{}' not found", data.name)))
        }
    }

    async fn handle_component(&self, ctx: &mut RouteContext<()>) -> Result<InteractionResponse, Error> {
        let data = match self.data.as_ref() {
            Some(InteractionData::MessageComponent(data)) => data,
            _ => return Err(Error::InvalidPayload("Missing component data".into())),
        };

        if let Some((root, target)) = data.custom_id.split_once(":") && 
           let Some(component) = UIHANDLERS.get(root) {
            return component.handle(self, ctx, target.to_string()).await
        }

        Err(Error::Generic(
            "Impossibile trovare il componente richiesto".into()
        ))
    }

    async fn handle_modal_submit(&self, ctx: &mut RouteContext<()>) -> Result<InteractionResponse, Error> {
        unimplemented!()
    }

    async fn handle_command(&self, ctx: &mut RouteContext<()>) -> Result<InteractionResponse, Error> {
        let data = match self.data.as_ref() {
            Some(InteractionData::ApplicationCommand(data)) => data,
            _ => return Err(Error::InvalidPayload("Missing command data".into())),
        };

        if let Some(command) = COMMANDS.get(data.name.as_str()) {
            command.respond(self, data, ctx).await
        } else {
            Err(Error::InvalidPayload(format!("Command '{}' not found", data.name)))
        }
    }

    async fn perform(&self, ctx: &mut RouteContext<()>) -> Result<InteractionResponse, Error> {
        match self.kind {
            InteractionType::Ping => Ok(InteractionResponse { kind: InteractionResponseType::Pong, data: None }),
            InteractionType::ApplicationCommandAutocomplete => self.handle_autocomplete(ctx).await,
            InteractionType::MessageComponent => self.handle_component(ctx).await,
            InteractionType::ApplicationCommand => self.handle_command(ctx).await,
            InteractionType::ModalSubmit => self.handle_modal_submit(ctx).await,
            _ => Err(Error::InvalidPayload("Interaction type not supported".into())),
        }
    }

    async fn defer(&self) -> Result<(), Error> {
        let url = format!(
            "https://discord.com/api/v10/interactions/{}/{}/callback",
            self.id,
            self.token
        );

        let body = json!({
            "type": InteractionResponseType::DeferredUpdateMessage
        });

        let response = CLIENT.post(url)
            .json(&body)
            .send()
            .await
            .map_err(|e| Error::ReqwestError(e))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::UpstreamError("Discord: Failed to defer interaction".into()))
        }
    }

    async fn delete(&self, ctx: &mut RouteContext<()>) -> Result<InteractionResponse, Error> {
        let token = ctx.env
            .var("DISCORD_TOKEN")
            .map_err(|e| Error::EnvironmentVariableNotFound(e.to_string()))?
            .to_string();

        let Some(ref channel) = self.channel else {
            return Err(Error::InteractionFailed("Can't delete message if channel is None".to_string()))
        };

        let Some(ref message) = self.message else {
            return Err(Error::InteractionFailed("Message is missing!".into()))
        };

        let url = format!(
            "https://discord.com/api/v10/channels/{}/messages/{}",
            channel.id.get(), message.id.get()
        );

        let response = CLIENT
            .delete(url)
            .header("Authorization", format!("Bot {}", token))
            .send()
            .await
            .map_err(|e| Error::ReqwestError(e))?;

        if response.status().is_success() {
            Ok(InteractionResponse::empty())
        } else {
            Err(Error::UpstreamError(format!("Discord API error: {}", response.status()).into()))
        }
    }

    async fn edit(&self, response: &InteractionResponse) -> Result<InteractionResponse, Error> {
        let url = format!(
            "https://discord.com/api/v10/webhooks/{}/{}/messages/@original",
            self.application_id, 
            self.token
        );

        let update_response = response.as_update();
        
        let client_response = CLIENT.patch(url)
            .json(&update_response)
            .send()
            .await
            .map_err(|e| Error::ReqwestError(e))?;

        let status = client_response.status();

        if status.is_success() {
            Ok(InteractionResponse::empty())
        } else {
            let err_text = client_response.text().await.unwrap_or_default();
            Err(Error::UpstreamError(format!("Discord API error: {} - {}", status, err_text).into()))
        }
    }

    async fn followup(&self) -> Result<(), Error> {
        unimplemented!()
    }

    fn is_dev(&self, ctx: &mut RouteContext<()>) -> bool {
        let author_id = match self.author_id() {
            Some(id) => id.to_string(),
            None => return false,
        };

        let dev_id = ctx.var("DISCORD_DEVELOPER_ID")
            .map(|v| v.to_string())
            .ok();

        match dev_id {
            Some(id) => id == author_id,
            None => {
                worker::console_warn!("DISCORD_DEVELOPER_ID non configurata nel wrangler.toml");
                false
            }
        }
    }
}