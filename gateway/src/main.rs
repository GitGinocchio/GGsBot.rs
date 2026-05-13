use serenity::prelude::*;

pub mod handlers;
pub mod constants;
pub mod dispatcher;

use handlers::message::MessageHandler;
use handlers::raw::RawHandler;

use constants::INTENTS;
use constants::DISCORD_TOKEN;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let mut client = Client::builder(&*DISCORD_TOKEN, INTENTS)
        .raw_event_handler(RawHandler::default())
        .await
        .expect("Errore durante la creazione del client Discord");

    println!("📡 Avvio del gateway in corso...");
    if let Err(why) = client.start().await {
        eprintln!("❌ Errore fatale del client: {:?}", why);
    }
}