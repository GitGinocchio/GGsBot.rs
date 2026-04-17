use async_trait::async_trait;
use twilight_model::{application::interaction::{Interaction, application_command::CommandData}, http::interaction::InteractionResponse};
use worker::RouteContext;

use crate::{build_commands, discord::command::{Command, CommandDataExt, CommandMap}, error::InteractionError};

mod setup;
mod show;

#[derive(Default)]
pub(crate) struct Ext {
}

#[async_trait(?Send)]
impl Command for Ext {
    fn name(&self) -> String { "ext".into() }

    fn description(&self) -> String { "Gestisci i comandi del bot!".into() }

    fn subcommands(&self) -> CommandMap {
        build_commands![
            setup::Setup,
            show::Show
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