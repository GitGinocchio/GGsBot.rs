use reqwest::Response;
use worker::{Result, Date, Env, Request, console_log};
use cfg_if::cfg_if;

use crate::{CLIENT, COMMANDS, discord::{command::SerializableCommand, error::{Error, InteractionError}}};

cfg_if! {
    // https://github.com/rustwasm/console_error_panic_hook#readme
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

pub fn is_dev(env: &Env) -> bool {
    match env.var("WORKER_ENV") {
        Ok(v) => v.to_string() == "dev",
        Err(_) => false, // fallback a production se non definito
    }
}

pub fn log_request(req: &Request) {
    let cf = req.cf();

    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        cf.and_then(|cf| cf.coordinates()).unwrap_or_default(),
        cf.and_then(|cf| cf.region()).unwrap_or("unknown region".into())
    );
}

pub async fn update_commands(env: &Env) -> Result<Response, Error> {
    let to_register: Vec<_> = COMMANDS.values()
        .map(|cmd| SerializableCommand(cmd.as_ref()))
        .collect();

    let app_id = env
        .var("DISCORD_APPLICATION_ID")
        .map_err(|e| InteractionError::WorkerError(e))?
        .to_string();

    let token = env
        .var("DISCORD_TOKEN")
        .map_err(|e| InteractionError::WorkerError(e))?
        .to_string();

    let url = format!("https://discord.com/api/v10/applications/{}/commands", app_id);

    let serialized_commands = serde_json::to_string(&to_register)
        .map_err(|_e| InteractionError::GenericError())?;
    worker::console_log!{"Sending  : {}", serialized_commands};

    CLIENT
        .put(url)
        .header("Authorization", format!("Bot {}", token))
        .header("Content-Type", "application/json")
        .body(serialized_commands) 
        .send()
        .await
        .map_err(|_e| Error::InteractionFailed(InteractionError::GenericError()))
}