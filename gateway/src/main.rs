use twilight_gateway::{Shard, ShardId, StreamExt};

pub mod constants;
pub mod dispatcher;

use constants::{INTENTS, DISCORD_TOKEN};
use twilight_model::gateway::event::DispatchEvent;

use crate::constants::{DISPATCHER, WANTED_EVENTS};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let mut shard = Shard::new(
        ShardId::ONE, 
        DISCORD_TOKEN.clone(), 
        INTENTS
    );

    println!("📡 Avvio del gateway (Twilight) in corso...");

    while let Some(item) = shard.next_event(WANTED_EVENTS.clone()).await {
        let event = match item {
                Ok(event) => event,
                Err(source) => {
                    eprintln!("⚠️ Errore socket: {:?}", source);
                    continue;
                }
            };

        if let Ok(dispatch_event) = DispatchEvent::try_from(event.clone()) {
            tokio::spawn(async move {
                if let Err(e) = DISPATCHER.dispatch(&dispatch_event).await {
                    eprintln!("❌ Dispatch error: {}", e);
                }
            }); 
        } else {
            println!("Evento ricevuto non di tipo DispatchEvent: {event:?}");
        }
    }

    Ok(())
}