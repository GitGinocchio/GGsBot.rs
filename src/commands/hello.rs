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
pub struct Hello;

#[async_trait(?Send)]
impl Command for Hello {
    fn name(&self) -> String {
        "hello".into()
    }

    fn description(&self) -> String {
        "Saluta qualcuno nella chat!".into()
    }

    fn options(&self) -> Vec<CommandOption> {
        vec![
            create_option(CommandOptionType::User, "user", "L'utente da salutare")
                .required(false)
        ]
    }

    async fn respond(
        &self, 
        interaction: &Interaction, 
        data: &CommandData, 
        _ctx: &mut RouteContext<()>
    ) -> Result<InteractionResponse, InteractionError> {
        let target_id = match data.get_option("user") {
            Some(CommandOptionValue::User(user)) => Some(user.get()),
            Some(_) | None => None
        };

        let message = match target_id {
            Some(id) => format!("Ciao <@{}>! 👋", id),
            None => {
                let author = interaction.author()
                    .ok_or(InteractionError::GenericError())?;
                format!("Ciao <@{}>, come stai? 👋", author.id)
            }
        };

        Ok(InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(InteractionResponseData {
                content: Some(message),
                ..Default::default()
            }),
        })
    }
}
