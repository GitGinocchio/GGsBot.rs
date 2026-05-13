use async_trait::async_trait;

use crate::{
    build_commands,
    framework::discord::command::{Command, CommandMap},
};

pub mod update;
pub mod version;

#[derive(Default)]
pub(crate) struct Bot {}

#[async_trait(?Send)]
impl Command for Bot {
    fn name(&self) -> String {
        "bot".into()
    }

    fn description(&self) -> String {
        "Set di comandi relativi al bot!".into()
    }

    fn subcommands(&self) -> CommandMap {
        build_commands![update::Update, version::Version]
    }
}
