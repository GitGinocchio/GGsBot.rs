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

use crate::{COMMANDS, error::Error};

#[async_trait(?Send)]
pub trait InteractionExt {
    async fn handle_command(&self, ctx: &mut RouteContext<()>) -> Result<InteractionResponse, Error>;
    async fn perform(&self, ctx: &mut RouteContext<()>) -> Result<InteractionResponse, Error>;

    fn is_dev(&self, ctx: &mut RouteContext<()>) -> bool;
}

#[async_trait(?Send)]
impl InteractionExt for Interaction {
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
            InteractionType::Ping => Ok(InteractionResponse {
                kind: InteractionResponseType::Pong,
                data: None
            }),
            InteractionType::ApplicationCommand => self.handle_command(ctx).await,
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