use async_trait::async_trait;

use crate::{discord::{
    command::{Command, CommandMap}, 
    locale::{Locale, Localization}, 
}, map};
use crate::build_commands;
mod clear;

#[derive(Default)]
pub(crate) struct Mods {

}

#[async_trait(?Send)]
impl Command for Mods {
    fn name(&self) -> Localization { 
        Localization::Map(map! {
            Locale::EnglishUS => "mods".to_string(),
            Locale::Italian => "moderatori".to_string()
        })
    }

    fn description(&self) -> Localization { "Set di comandi dedicati ai moderatori".into() }

    fn subcommands(&self) -> CommandMap {
        build_commands![clear::Clear]
    }
}