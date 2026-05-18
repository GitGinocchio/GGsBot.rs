use std::collections::HashMap;

use serde::{Deserialize, de::DeserializeSeed};
use serde_json::{json, Value};
use twilight_model::gateway::event::{DispatchEvent, DispatchEventWithTypeDeserializer, EventType};
use worker::{Request, Response, Result, RouteContext};

use crate::constants::COMMANDS;

#[derive(Deserialize, Clone, Debug)]
pub struct DispatcherEnvelope {
    event: Value,
    kind: EventType
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

        let mut responses: HashMap<String, Value> = HashMap::new();

        for (name, command) in COMMANDS.iter() {
            let handlers = match command.get_events() {
                Some(events) => events,
                None => continue
            };

            if !handlers.responds_to(data.kind) {
                continue;
            }

            let response = handlers.dispatch(&self.ctx, event.clone()).await?;
            responses.insert(name.clone(), response);
        }

        Response::from_json(&json!({
            "status" : "success",
            "responses" : responses
        }))
    }
}