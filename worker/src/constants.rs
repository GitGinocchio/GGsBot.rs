use std::sync::LazyLock;

use reqwest::Client;

use crate::{
    build_commands, 
    build_triggers, 
    build_uihandlers, 
    commands, 
    framework::{
        discord::command::CommandMap, 
        traits::{
            trigger::TriggerMap, 
            ui::UiHandlerMap
        }
    },
    triggers, 
    ui
};

pub static CLIENT: LazyLock<Client> = LazyLock::new(|| Client::new());

pub static UIHANDLERS: LazyLock<UiHandlerMap> = LazyLock::new(|| build_uihandlers!(
    ui::nasa::NasaUIHandler
));

pub static COMMANDS: LazyLock<CommandMap> = LazyLock::new(|| {
    build_commands!(
        commands::hello::Hello,
        commands::nasa::Nasa,
        commands::bot::Bot,
        commands::mods::Mods,
        commands::tempvc::Tempvc,

        commands::ext::Ext
    )
});

pub static TRIGGERS: LazyLock<TriggerMap> = LazyLock::new(|| build_triggers!(
    triggers::apod::ApodTrigger
));