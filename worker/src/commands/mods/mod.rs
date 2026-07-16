use flarecord::models::command::SubcommandType;
use flarecord::prelude::*;
use flarecord::command;

use crate::commands::mods::clear::Clear;

mod clear;

#[command]
impl Command for Mods {
    fn name(&self) -> String {
        "mods".into()
    }

    fn description(&self) -> String {
        "Set of commands used for chat moderation".into()
    }

    fn subcommands(&self) -> Vec<SubcommandType> {
        vec![
            Clear.into_subcommand()
        ]
    }
}