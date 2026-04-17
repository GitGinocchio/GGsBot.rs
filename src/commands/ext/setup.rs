use async_trait::async_trait;
use twilight_model::{application::{command::{CommandOption, CommandOptionChoiceValue, CommandOptionType}, interaction::{Interaction, application_command::CommandData}}, http::interaction::InteractionResponse};
use worker::RouteContext;

use crate::{
    COMMANDS, 
    discord::{
        command::{
            Command, 
            CommandDataExt
        }, 
        option::CommandOptionExt
    }, 
    error::InteractionError
};

#[derive(Default)]
pub(crate) struct Setup {
}

#[async_trait(?Send)]
impl Command for Setup {
    fn name(&self) -> String { "setup".into() }

    fn description(&self) -> String { "Configura un estensione del bot!".into() }

    fn options(&self) -> Vec<CommandOption> {
        let mut ext = CommandOption::new(
            CommandOptionType::String, 
            "extension", 
            "L'estensione da aggiungere"
        );

        for (name, _) in COMMANDS.iter() {
            if name == "ext" { continue };
            ext.add_choice(name, CommandOptionChoiceValue::String(name.clone()));
        }

        vec![ext]
    }

    async fn respond(
        &self, 
        interaction: &Interaction,
        data: &CommandData,
        ctx: &mut RouteContext<()>
    ) -> Result<InteractionResponse, InteractionError> {
        let sub_name = data.get_subcommand_name().ok_or(InteractionError::GenericError())?;
        let sub_data = data.get_subcommand_data().ok_or(InteractionError::GenericError())?;

        let subs = self.subcommands();
        if let Some(sub_cmd) = subs.get(sub_name) {
            return sub_cmd.respond(interaction, &sub_data, ctx).await;
        }

        Err(InteractionError::GenericError())
    }
}