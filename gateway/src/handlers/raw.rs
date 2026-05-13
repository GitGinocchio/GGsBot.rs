use serde_json::json;
use serenity::{all::Event, async_trait};
use serenity::prelude::*;

use crate::constants::{DISPATCHER};
#[derive(Default)]
pub struct RawHandler {

}

#[async_trait()]
impl RawEventHandler for RawHandler {
    async fn raw_event(&self, _ctx: Context, event: Event) {
        println!("[raw_event] sending raw_event '{:?}'", event.name());

        let response = DISPATCHER
            .send_event(&json!(event))
            .await
            .unwrap();
        

        println!("[raw_event] response: {response}");
    }
}