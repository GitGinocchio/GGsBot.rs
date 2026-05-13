use async_trait::async_trait;

use crate::{
    build_commands,
    commands::tempvc,
    framework::discord::command::{Command, CommandMap},
};

mod new;

#[derive(Default)]
pub(crate) struct Tempvc {}

#[async_trait(?Send)]
impl Command for Tempvc {
    fn name(&self) -> String {
        "tempvc".into()
    }

    fn description(&self) -> String {
        "Crea canali vocali personalizzati per te!".into()
    }

    fn subcommands(&self) -> CommandMap {
        build_commands![tempvc::new::New]
    }
}
