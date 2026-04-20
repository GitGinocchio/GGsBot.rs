use async_trait::async_trait;
use twilight_model::{
    application::interaction::{
        Interaction, InteractionData, InteractionType
    }, 
    http::interaction::{
        InteractionResponse, InteractionResponseType
    }
};
use worker::RouteContext;

use crate::{COMMANDS, PAGES, error::{Error, InteractionError}};

#[async_trait(?Send)]
#[allow(unused)]
pub trait InteractionExt {
    async fn handle_autocomplete(&self, ctx: &mut RouteContext<()>) -> Result<InteractionResponse, Error>;
    async fn handle_component(&self, ctx: &mut RouteContext<()>) -> Result<InteractionResponse, Error>;
    async fn handle_modal_submit(&self, ctx: &mut RouteContext<()>) -> Result<InteractionResponse, Error>;
    async fn handle_command(&self, ctx: &mut RouteContext<()>) -> Result<InteractionResponse, Error>;
    
    async fn perform(&self, ctx: &mut RouteContext<()>) -> Result<InteractionResponse, Error>;

    async fn defer(&self) -> Result<(), Error>;

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
                Ok(None) => return Err(Error::InteractionFailed(crate::error::InteractionError::GenericError())),
                Err(e) => return Err(Error::InteractionFailed(e)),
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
           let Some(component) = PAGES.get(root) {
            return component.handle(self, ctx, Some(target.to_string()))
                .await
                .map_err(|e| Error::InteractionFailed(e));
        }

        Err(Error::InteractionFailed(InteractionError::UnknownComponent(
            "Impossibile trovare il componente richiesto".into()
        )))
    }

    async fn handle_modal_submit(&self, ctx: &mut RouteContext<()>) -> Result<InteractionResponse, Error> {
        unimplemented!()
    }

    async fn defer(&self) -> Result<(), Error> {
        unimplemented!()
    }

    async fn handle_command(&self, ctx: &mut RouteContext<()>) -> Result<InteractionResponse, Error> {
        let data = match self.data.as_ref() {
            Some(InteractionData::ApplicationCommand(data)) => data,
            _ => return Err(Error::InvalidPayload("Missing command data".into())),
        };

        if let Some(command) = COMMANDS.get(data.name.as_str()) {
            command.respond(self, data, ctx)
                .await
                .map_err(|e| Error::InteractionFailed(e))
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