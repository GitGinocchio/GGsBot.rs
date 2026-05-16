pub mod clear;

use async_trait::async_trait;

use crate::{
    build_commands, 
    commands::mods, 
    framework::discord::command::{
        Command, 
        CommandMap
    }
};

#[derive(Default)]
pub(crate) struct Mods {}

#[async_trait(?Send)]
impl Command for Mods {
    fn name(&self) -> String {
        "mods".into()
    }

    fn description(&self) -> String {
        "Canali dedicati ai moderatori dei canali!".into()
    }

    fn subcommands(&self) -> CommandMap {
        build_commands![mods::clear::Clear]
    }
}