use std::str::FromStr;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use twilight_model::{id::Id};
use worker::{Env};

use crate::{
    error::Error,
    framework::traits::queue::MessageHandler, 
    services::{
        apod::ApodService, 
        discord::{
            DiscordMessagePayload, 
            DiscordService
        }
    }
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApodQueueMessage {
    pub channel_id: String,
    pub guild_id: String
}

pub struct ApodMessageHandler {
    pub discord: DiscordService,
    pub apod_embed: serde_json::Value,
}

#[async_trait(?Send)]
impl MessageHandler for ApodMessageHandler {
    type Payload = ApodQueueMessage;

    async fn setup(env: &Env) -> Result<Self, Error> {
        let discord_service = DiscordService::new(env)?;
        let apod_service = ApodService::new(env)?;
        
        let apod_data = apod_service.fetch_apod_with_retries(5).await?;
        apod_service.put_apod(&apod_data).await?;

        let apod_embed = ApodService::build_embed(apod_data);
        let apod_embed_value = serde_json::to_value(&apod_embed)?;

        Ok(Self { 
            discord: discord_service,
            apod_embed: apod_embed_value
        })
    }

    async fn handle(&self, payload: &Self::Payload) -> Result<(), Error> {
        let base_payload = DiscordMessagePayload {
            embeds: Some(vec![self.apod_embed.clone()]),
            ..Default::default()
        };

        let channel_id = Id::from_str(&payload.channel_id)
            .map_err(|e| Error::ParseIntError(e))?;

        self.discord.send_guild_message(channel_id, &base_payload).await
    }
}