use chrono::{DateTime, TimeDelta, Utc};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::Serialize;
use serde_json::{Value, json};
use twilight_model::{
    channel::{
        Channel, 
        ChannelType, 
        Message,
    }, 
    guild::Permissions, 
    http::permission_overwrite::{
        PermissionOverwrite,
        PermissionOverwriteType
    }, 
    id::{Id, marker::{ChannelMarker, GenericMarker, GuildMarker, UserMarker}}
};
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


// TODO: Spostare questo in framework
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

    pub async fn send_guild_message(&self, channel_id: Id<ChannelMarker>, payload: &DiscordMessagePayload) -> Result<(), Error> {
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

    pub async fn fetch_messages(&self, channel_id: Id<ChannelMarker>, amount: u8) -> Result<Vec<Message>, Error> {
        let messages: Vec<Message> = CLIENT
            .get(format!("{}/channels/{}/messages?limit={}",DISCORD_API_ENDPOINT, channel_id, amount))
            .header("Authorization", format!("Bot {}", self.token))
            .send()
            .await?
            .json()
            .await?;

        Ok(messages)
    }

    pub async fn delete_messages(&self, channel_id: Id<ChannelMarker>, amount: u8) -> Result<usize, Error> {
        if amount < 1 || amount > 100 {
            return Err(Error::InteractionFailed("Il delete richiede tra 1 e i 100 messaggi.".into()));
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

        if message_ids.is_empty() {
            return Err(Error::InteractionFailed("I messaggi trovati sono troppo vecchi (>14 giorni) o insufficienti per il bulk delete.".into()));
        }

        if message_ids.len() == 1 {
            let message_id = &message_ids[0];
            let response = CLIENT
                .delete(format!("{}/channels/{}/messages/{}", DISCORD_API_ENDPOINT, channel_id, message_id))
                .header("Authorization", format!("Bot {}", self.token))
                .send()
                .await?;

            worker::console_debug!("[delete_single] response status: {:?}", response.status());
        } else {
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
        }

        Ok(message_ids.len())
    }

    pub async fn create_channel(&self, 
        guild_id: Id<GuildMarker>, 
        name: String, 
        kind: ChannelType, 
        parent_id: Option<Id<GenericMarker>>, 
        position: Option<u16>
    ) -> Result<Channel, Error> {
        let url = format!("{}/guilds/{}/channels", DISCORD_API_ENDPOINT, guild_id);

        let payload = json!({
            "name": name,
            "type": kind,
            "parent_id": parent_id.map(|id| id.get().to_string()),
            "position": position
        });

        let response: Channel = CLIENT
            .post(&url)
            .header(AUTHORIZATION, format!("Bot {}", self.token))
            .header(CONTENT_TYPE, "application/json")
            .json(&payload)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    pub async fn delete_channel(&self, 
        channel_id: Id<ChannelMarker>
    ) -> Result<(), Error> {
        let url = format!("{}/channels/{}", DISCORD_API_ENDPOINT, channel_id);

        let response = CLIENT
            .delete(&url)
            .header(AUTHORIZATION, format!("Bot {}", self.token))
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::Generic(format!("Errore eliminazione canale: {}", error_text)))
        }
    }

    pub async fn move_member(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
        channel_id: Id<ChannelMarker>
    ) -> Result<(), Error> {
        let url = format!(
            "{}/guilds/{}/members/{}", 
            DISCORD_API_ENDPOINT, 
            guild_id, 
            user_id
        );

        let payload = json!({
            "channel_id": Some(channel_id.get().to_string()),
        });

        let response = CLIENT
            .patch(&url)
            .header(AUTHORIZATION, format!("Bot {}", self.token))
            .header(CONTENT_TYPE, "application/json")
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let error_text = response.text().await.unwrap_or_default();
            worker::console_error!("Errore spostamento membro: {}", error_text);
            Err(Error::UpstreamError(format!("Discord API Error: {}", error_text)))
        }
    }

    pub async fn get_channel(&self, channel_id: Id<ChannelMarker>) -> Result<Channel, Error> {
        let url = format!("{}/channels/{}",DISCORD_API_ENDPOINT, channel_id);

        let channel: Channel = CLIENT
            .get(&url)
            .header(AUTHORIZATION, format!("Bot {}", self.token))
            .send()
            .await?
            .json()
            .await?;

        Ok(channel)
    }

    pub async fn set_permissions(
        &self, 
        channel_id: Id<ChannelMarker>, 
        user_id: Id<UserMarker>, 
        allow: Option<Permissions>, 
        deny: Option<Permissions>,
        kind: PermissionOverwriteType
    ) -> Result<(), Error> {
        let url = format!("{}/channels/{}/permissions/{}",DISCORD_API_ENDPOINT, channel_id, user_id);

        let payload = PermissionOverwrite {
            allow, 
            deny,
            id: user_id.cast(),
            kind
        };

        let response = CLIENT.put(&url)
            .header(AUTHORIZATION, format!("Bot {}", self.token))
            .header(CONTENT_TYPE, "application/json")
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        }
        else {
            Err(Error::Generic(format!("Errore Discord: {}", response.text().await?).into()))
        }
    }
}