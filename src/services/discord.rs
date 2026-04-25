use serde::Serialize;
use serde_json::Value;
use twilight_model::{channel::Message, http::interaction::{InteractionResponse, InteractionResponseData}};
use worker::Env;

use crate::{CLIENT, error::Error};

#[derive(Serialize, Default, Debug)]
pub struct DiscordMessagePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
}

#[allow(unused)]
pub struct DiscordService {
    env: Env
}

#[allow(unused)]
impl DiscordService {
    pub fn new(env: &Env) -> Self {
        Self { env: env.clone() }
    }

    pub fn fetch_guilds(&self) {
        unimplemented!()
    }

    pub async fn send_guild_message(&self, channel_id: &str, payload: &DiscordMessagePayload) -> Result<(), Error> {
        let token = self.env.var("DISCORD_TOKEN")?
            .to_string();

        let url = format!("https://discord.com/api/v10/channels/{}/messages", channel_id);

        let response = CLIENT
            .post(url)
            .header("Authorization", format!("Bot {}", token))
            .json(payload)
            .send()
            .await
            .map_err(|e| Error::ReqwestError(e))?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            worker::console_error!("Discord API Error {}: {}", status, body);
            Err(Error::UpstreamError(format!("Discord error: {}", status).into()))
        }
    }
}