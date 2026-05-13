use serde_json::json;
use serenity::{all::Event, async_trait};
use serenity::prelude::*;

use crate::constants::{CLIENT, HTTP_ENDPOINT};

#[derive(Default)]
pub struct RawHandler {

}

#[async_trait()]
impl RawEventHandler for RawHandler {
    async fn raw_event(&self, _ctx: Context, event: Event) {
        let payload = json!({
            "body" : event, 
            "content_type" : "json",
            "delay_seconds": 0
        });

        println!("[raw_event] sending raw_event '{:?}'", event.name());

        let response = CLIENT.post(&*HTTP_ENDPOINT)
            .json(&payload)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        println!("[raw_event] response: {response}");
    }
}