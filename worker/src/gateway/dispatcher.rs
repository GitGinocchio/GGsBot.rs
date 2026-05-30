use std::collections::HashMap;

use serde_json::{json, Value};
use twilight_model::gateway::event::DispatchEvent;
use worker::{Env, Response, Result, Storage};

use crate::{constants::COMMANDS};

pub struct Dispatcher {
    storage: Storage,
    env: Env 
}

impl Dispatcher {
    pub fn new(storage: Storage, env: Env) -> Self {
        Self {
            storage,
            env
        }
    }

    pub async fn dispatch(&self, event: DispatchEvent) -> Result<Response> {
        let tasks = COMMANDS.iter().filter_map(|(name, command)| {
            let handlers = command.get_events()?;
            
            if !handlers.responds_to(event.kind()) {
                return None;
            }

            let name = name.clone();
            let event = event.clone();

            Some(async move {
                let result = handlers.dispatch(self.env.clone(), event, Value::Null).await;
                (name, result)
            })
        });

        let results = futures::future::join_all(tasks).await;

        let mut responses: HashMap<String, Value> = HashMap::new();
        for (name, res) in results {
            match res {
                Ok(val) => {
                    responses.insert(name, val);
                }
                Err(e) => {
                    worker::console_error!("Errore nel comando {}: {:?}", name, e);
                    responses.insert(name, json!({ "error": e.to_string() }));
                }
            }
        }

        let payload = json!({
            "status" : "success",
            "responses" : responses
        });

        worker::console_debug!("[gateway: {:?}] response: {}", event.kind(), payload);

        Response::from_json(&payload)
    }
}