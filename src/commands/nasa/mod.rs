use async_trait::async_trait;
use twilight_model::{guild::Permissions, oauth::ApplicationIntegrationType};

use crate::{build_commands, discord::command::{Command, CommandMap}};

mod apod;

#[derive(Default)]
pub(crate) struct Nasa {
}

#[async_trait(?Send)]
impl Command for Nasa {
    fn name(&self) -> String { "nasa".into() }

    fn description(&self) -> String { "Gestisci i comandi del bot!".into() }

    fn integration_types(&self) -> Vec<ApplicationIntegrationType> {
        vec![ApplicationIntegrationType::GuildInstall, ApplicationIntegrationType::UserInstall]
    }

    fn subcommands(&self) -> CommandMap {
        build_commands![
            apod::Apod
        ]
    }
}