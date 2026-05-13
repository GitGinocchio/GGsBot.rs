use async_trait::async_trait;
use twilight_model::{
    application::{
        command::{CommandOption, CommandOptionType},
        interaction::{
            Interaction,
            application_command::{CommandData, CommandOptionValue},
        },
    },
    http::interaction::{InteractionResponse, InteractionResponseType},
};
use worker::RouteContext;

use crate::{
    error::Error,
    framework::discord::{
        command::{Command, CommandDataExt},
        option::OptionBuilder,
        response::ResponseBuilder,
    },
};

#[derive(Default)]
pub struct Hello;

/*
#[async_trait(?Send)]
impl CommandController for Hello {
    async fn on_setup(
        &self,
        interaction: &Interaction,
        ctx: &mut RouteContext<()>
    ) -> Option<Result<InteractionResponse, Error>> {
        let page = crate::ui::hello::HelloUiHandler::default();
        Some(Ok(page.render(0).await))
    }
}
*/

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
                .build(),
        ]
    }

    async fn respond(
        &self,
        interaction: &Interaction,
        data: &CommandData,
        _ctx: &mut RouteContext<()>,
    ) -> Result<InteractionResponse, Error> {
        let target_id = match data.get_option("user") {
            Some(CommandOptionValue::User(user)) => Some(user.get()),
            Some(_) | None => None,
        };

        let message = match target_id {
            Some(id) => format!("Ciao <@{}>! 👋", id),
            None => {
                let author = interaction
                    .author()
                    .ok_or(Error::InteractionFailed("No author field present!".into()))?;
                format!("Ciao <@{}>, come stai? 👋", author.id)
            }
        };

        Ok(
            ResponseBuilder::new(InteractionResponseType::ChannelMessageWithSource)
                .content(message)
                .build(),
        )
    }
}
