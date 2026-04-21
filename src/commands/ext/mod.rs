use async_trait::async_trait;
use twilight_model::{application::interaction::{Interaction, application_command::CommandData}, guild::Permissions, http::interaction::InteractionResponse, oauth::ApplicationIntegrationType};
use worker::RouteContext;

use crate::{build_commands, discord::command::{Command, CommandDataExt, CommandMap}, error::Error, handle_subcommands};

mod setup;
mod teardown;
mod enable;
mod disable;
mod show;

static REQUIRED_EXTENSIONS: &[&str] = &["ext", "bot"];

#[derive(Default)]
pub(crate) struct Ext {
}

#[async_trait(?Send)]
impl Command for Ext {
    fn name(&self) -> String { "ext".into() }

    fn description(&self) -> String { "Gestisci i comandi del bot!".into() }

    fn integration_types(&self) -> Vec<ApplicationIntegrationType> {
        vec![ApplicationIntegrationType::GuildInstall]
    }

    fn default_member_permissions(&self) -> Option<Permissions> {
        Some(Permissions::ADMINISTRATOR)
    }

    fn subcommands(&self) -> CommandMap {
        build_commands![
            setup::Setup,
            teardown::Teardown,
            enable::Enable,
            disable::Disable,
            show::Show
        ]
    }
}