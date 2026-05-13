use serde_json::Value;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::constants::{CLIENT, HTTP_ENDPOINT};

#[derive(Default)]
pub struct MessageHandler {

}

#[async_trait()]
impl EventHandler for MessageHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot { return; }

        let request = CLIENT
            .post(&*HTTP_ENDPOINT)
            .json(&msg)
            .send()
            .await;

        match request {
            Ok(res) => {
                if let Ok(response_json) = res.json::<Value>().await {
                    println!("Successo: {response_json:?}");
                } else {
                    eprintln!("Errore nel parsing del JSON dal Worker");
                }
            }
            Err(e) => eprintln!("Errore nell'invio al Worker: {e}"),
        }

        println!("ctx: {ctx:?}");
        println!("msg: {msg:?}");
    }
}