use chrono::{DateTime, TimeDelta, Utc};
use serde::Serialize;
use serde_json::{Value, json};
use twilight_model::channel::Message;
use worker::Env;

use crate::{constants::CLIENT, error::Error};

pub const DISCORD_API_ENDPOINT: &'static str = "https://discord.com/api/v10";

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
    env: Env,
    token: String
}

#[allow(unused)]
impl DiscordService {
    pub fn new(env: &Env) -> Result<Self, Error> {
        let token = env.var("DISCORD_TOKEN")?
            .to_string();

        Ok(Self { 
            env: env.clone(),
            token: token
        })
    }

    pub fn fetch_guilds(&self) {
        unimplemented!()
    }

    pub async fn send_guild_message(&self, channel_id: &str, payload: &DiscordMessagePayload) -> Result<(), Error> {
        let url = format!("{}/channels/{}/messages",DISCORD_API_ENDPOINT, channel_id);

        let response = CLIENT
            .post(url)
            .header("Authorization", format!("Bot {}", self.token))
            .json(payload)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            worker::console_error!("Discord API Error {}: {}", status, body);
            Err(Error::UpstreamError(format!("Discord error: {}", status).into()))
        }
    }

    pub async fn fetch_messages(&self, channel_id: &str, amount: u8) -> Result<Vec<Message>, Error> {
        let messages: Vec<Message> = CLIENT
            .get(format!("{}/channels/{}/messages?limit={}",DISCORD_API_ENDPOINT, channel_id, amount))
            .header("Authorization", format!("Bot {}", self.token))
            .send()
            .await?
            .json()
            .await?;

        Ok(messages)
    }

    pub async fn delete_messages_bulk(&self, channel_id: &str, amount: u8) -> Result<usize, Error> {
        if amount < 2 || amount > 100 {
            return Err(Error::InteractionFailed("Il bulk delete richiede tra i 2 e i 100 messaggi.".into()));
        }

        let messages = self.fetch_messages(channel_id, amount).await?;

        if messages.is_empty() {
            return Ok(0);
        }

        let now = Utc::now();
        let two_weeks_limit = TimeDelta::days(14);

        let message_ids: Vec<String> = messages
            .into_iter()
            .filter(|msg| {
                if let Ok(date) = DateTime::from_timestamp_secs(msg.timestamp.as_secs()).ok_or(None::<DateTime<Utc>>) {
                    (now - date) < two_weeks_limit
                } else {
                    false
                }
            })
            .map(|msg| msg.id.get().to_string())
            .collect();

        if message_ids.len() < 2 {
            return Err(Error::InteractionFailed("I messaggi trovati sono troppo vecchi (>14 giorni) o insufficienti per il bulk delete.".into()));
        }

        let payload = json!({ "messages": message_ids });

        let response: Value = CLIENT
            .post(format!("{}/channels/{}/messages/bulk-delete",DISCORD_API_ENDPOINT, channel_id))
            .header("Authorization", format!("Bot {}", self.token))
            .json(&payload)
            .send()
            .await?
            .json()
            .await?;

        worker::console_debug!("[delete_bulk] response: {response:?}");

        Ok(message_ids.iter().count())
    }
}