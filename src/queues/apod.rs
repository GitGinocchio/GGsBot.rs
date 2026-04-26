use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use worker::{Context, Env, MessageBatch, MessageExt};

use crate::{error::Error, framework::traits::queue::Queue, services::{apod::ApodService, discord::{DiscordMessagePayload, DiscordService}}};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApodQueueMessage {
    pub channel_id: String,
    pub guild_id: String
}

#[derive(Default)]
pub struct ApodQueue;

#[async_trait(?Send)]
impl Queue for ApodQueue {
    fn name(&self) -> &str {
        "ggsbotrs-apod-queue"
    }

    async fn handle(
        &self,
        batch: MessageBatch<serde_json::Value>,
        env: &Env,
        _ctx: &Context,
    ) -> Result<(), Error> {
        let discord_service = DiscordService::new(env);
        
        let apod_service = ApodService::new(env)?;
        let apod_data = apod_service.fetch_apod_with_retries(3).await?;
        apod_service.put_apod(&apod_data).await?;
        
        let apod_embed = ApodService::build_embed(apod_data);
        let apod_embed_value = serde_json::to_value(&apod_embed)?;

        for message in batch.messages()? {
            let msg_data: ApodQueueMessage = match serde_json::from_value(message.body().clone()) {
                Ok(m) => m,
                Err(e) => {
                    worker::console_error!("Errore deserializzazione messaggio: {:?}", e);
                    message.ack();
                    continue;
                }
            };

            let payload = DiscordMessagePayload {
                embeds: Some(vec![apod_embed_value.clone()]),
                ..Default::default()
            };

            match discord_service.send_guild_message(&msg_data.channel_id, &payload).await {
                Ok(_) => {
                    message.ack();
                },
                Err(e) => {
                    message.retry();
                    worker::console_error!(
                        "Fallito invio per guild {}: {:?}", 
                        msg_data.guild_id, 
                        e
                    );
                }
            }
        }

        Ok(())
    }
}
