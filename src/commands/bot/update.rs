use async_trait::async_trait;
use twilight_model::{
    application::{
        command::{
            CommandOption, 
            CommandOptionType
        }, 
        interaction::{
            Interaction, 
            application_command::{
                CommandData, 
                CommandOptionValue
            }
        }
    },
    http::interaction::{
        InteractionResponse, 
        InteractionResponseType
    },
};
use twilight_model::http::interaction::InteractionResponseData;
use worker::RouteContext;

use crate::{
    discord::{
        command::{Command, CommandDataExt}, 
        option::{
            CommandOptionExt, 
            create_option
        }
    }, 
    error::InteractionError
};

#[derive(Default)]
pub struct Update;

#[async_trait(?Send)]
impl Command for Update {
    fn name(&self) -> String {
        "update".into()
    }

    fn description(&self) -> String {
        "Aggiorna i comandi del bot!".into()
    }

    async fn respond(
        &self, 
        interaction: &Interaction, 
        data: &CommandData, 
        _ctx: &mut RouteContext<()>
    ) -> Result<InteractionResponse, InteractionError> {

        let response = utils::update_commands(&ctx.worker.env)
            .await
            .map_err(|_e| InteractionError::GenericError())?;
        let status = response.status().as_u16();
        
        if let Err(e) = response.error_for_status() {
            return Err(InteractionError::ReqwestError(e));
        }
        
        Ok(InteractionApplicationCommandCallbackData {
            content: Some(format!("✅ Aggiornamento comandi completato! Status: **{}**", status)),
            flags: Some(MessageFlags::EPHEMERAL),
            ..Default::default()
        })
    }
}