use std::sync::LazyLock;

use reqwest::Client;

use crate::{
    build_commands, 
    build_queue_enum, 
    build_queue_handlers, 
    build_triggers, 
    build_uihandlers, 
    commands, 
    framework::{
        discord::command::CommandMap, 
        traits::{
            queue::QueueMap, 
            trigger::TriggerMap, 
            ui::UiHandlerMap
        }
    }, 
    queues, 
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
        commands::ext::Ext
    )
});

pub static TRIGGERS: LazyLock<TriggerMap> = LazyLock::new(|| build_triggers!(
    triggers::apod::ApodTrigger
));

pub static QUEUES: LazyLock<QueueMap> = LazyLock::new(|| build_queue_handlers!(
    queues::apod::ApodQueue
));

build_queue_enum!(
    Apod => queues::apod::ApodQueueMessage
);