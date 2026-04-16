use async_trait::async_trait;
use twilight_model::{application::interaction::{Interaction, application_command::CommandData}, http::interaction::InteractionResponse};
use worker::RouteContext;

use crate::{build_commands, commands::tempvc, discord::command::{Command, CommandDataExt, CommandMap}, error::InteractionError};

mod new;

#[derive(Default)]
pub(crate) struct Tempvc {
}

#[async_trait(?Send)]
impl Command for Tempvc {
    fn name(&self) -> String { "tempvc".into() }

    fn description(&self) -> String { "Crea canali vocali personalizzati per te!".into() }

    fn subcommands(&self) -> CommandMap {
        build_commands![
            tempvc::new::New
        ]
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