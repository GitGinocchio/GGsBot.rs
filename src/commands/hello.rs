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
use worker::RouteContext;

use crate::{
    discord::{
        command::{Command, CommandDataExt}, 
        option::OptionBuilder, 
        response::ResponseBuilder
    }, 
    error::InteractionError,
    traits::{command::CommandController, page::Page}
};

#[derive(Default)]
pub struct Hello;

#[async_trait(?Send)]
impl CommandController for Hello {
    async fn on_setup(
        &self, 
        interaction: &Interaction, 
        ctx: &mut RouteContext<()>
    ) -> Option<Result<InteractionResponse, InteractionError>> {
        let page = crate::pages::hello::HelloSetupPage::default();
        Some(page.handle(interaction, ctx, None).await)
    }
}

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
            OptionBuilder::new(CommandOptionType::User, "user", "L'utente da salutare")
                .required(false)
                .build()
        ]
    }

    fn get_controller(&self) -> Option<&dyn CommandController> {
        Some(self)
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

        Ok(ResponseBuilder::new(InteractionResponseType::ChannelMessageWithSource)
            .content(message)
            .build())
    }
}
