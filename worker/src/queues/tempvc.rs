use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use twilight_model::{id::{Id, marker::ChannelMarker}};
use worker::{Env};

use crate::{
    error::Error, framework::traits::queue::MessageHandler, services::discord::DiscordService 
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TempvcDeleteChannelMessage {
    pub channel_id: String,
}

pub struct TempvcDeleteChannelHandler {
    discord: DiscordService
}

#[async_trait(?Send)]
impl MessageHandler for TempvcDeleteChannelHandler {
    type Payload = TempvcDeleteChannelMessage;

    async fn setup(env: &Env) -> Result<Self, Error> {
        Ok(Self {
            discord: DiscordService::new(env)?
        })
    }

    async fn handle(&self, payload: &Self::Payload) -> Result<(), Error> {
        let channel_id: Id<ChannelMarker> = payload.channel_id.parse()?;
        self.discord.delete_channel(channel_id).await?;

        Ok(())
    }
}