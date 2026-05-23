use std::collections::HashMap;

use serde::{Deserialize, de::DeserializeSeed};
use serde_json::{json, Value};
use twilight_model::gateway::event::{DispatchEvent, DispatchEventWithTypeDeserializer, EventType};
use worker::{Request, Response, Result, RouteContext};

use crate::constants::COMMANDS;

#[derive(Deserialize, Clone, Debug)]
pub struct DispatcherEnvelope {
    event: Value,
    kind: EventType,

    #[serde(default)]
    metadata: Value
}

pub struct Gateway {
    ctx: RouteContext<()>
}

impl Gateway {
    pub fn new(ctx: RouteContext<()>) -> Self {
        Self { ctx: ctx }
    }

    pub async fn handle_request(&self, mut req: Request) -> Result<Response> {
        let data: DispatcherEnvelope = req.json().await?;

        let event_name = data.kind
            .name()
            .ok_or(worker::Error::from("Could not get event kind name"))?;

        let deserializer = DispatchEventWithTypeDeserializer::new(event_name);
        let event: DispatchEvent = deserializer
            .deserialize(data.event)
            .map_err(|e| worker::Error::from(format!("Errore deserializzazione: {}", e)))?;

        let tasks = COMMANDS.iter().filter_map(|(name, command)| {
            let handlers = command.get_events()?;
            
            if !handlers.responds_to(data.kind) {
                return None;
            }

            let name = name.clone();
            let event = event.clone();
            let metadata = data.metadata.clone();
            let ctx = &self.ctx;

            Some(async move {
                let result = handlers.dispatch(ctx, event, metadata).await;
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

        worker::console_debug!("[gateway: {event_name}] response: {payload}");

        Response::from_json(&payload)
    }
}