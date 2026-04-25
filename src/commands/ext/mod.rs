use async_trait::async_trait;
use twilight_model::{
    guild::Permissions,
    oauth::ApplicationIntegrationType,
};

use crate::{
    build_commands,
    framework::discord::command::{Command, CommandMap},
};

mod disable;
mod enable;
mod setup;
mod show;
mod teardown;

static REQUIRED_EXTENSIONS: &[&str] = &["ext", "bot"];

#[derive(Default)]
pub(crate) struct Ext {}

#[async_trait(?Send)]
impl Command for Ext {
    fn name(&self) -> String {
        "ext".into()
    }

    fn description(&self) -> String {
        "Gestisci i comandi del bot!".into()
    }

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
