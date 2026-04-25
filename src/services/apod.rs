use std::time::Duration;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use twilight_model::channel::message::Embed;
use url::Url;
use worker::Env;

use crate::{
    CLIENT,
    error::Error,
    framework::{discord::embed::EmbedExt, traits::namespaces::KV_BINDING},
};

const API_URL: &'static str = "https://api.nasa.gov/planetary/apod";

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ApodMediaType {
    Video,
    Image,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(unused)]
pub struct ApodResponse {
    title: String,
    explanation: String,
    media_type: ApodMediaType,
    url: String,
    hdurl: Option<String>,
    service_version: String,
    date: chrono::NaiveDate,
    concepts: Option<Vec<String>>,
    thumbnail_url: Option<String>,
    copyright: Option<String>,
}

fn convert_to_watch_url(nasa_embed_url: &str) -> Option<String> {
    let parsed_url = Url::parse(nasa_embed_url).ok()?;
    let path_segments: Vec<&str> = parsed_url.path_segments()?.collect();
    if path_segments.len() >= 2 && path_segments[0] == "embed" {
        let video_id = path_segments[1];
        return Some(format!("https://www.youtube.com/watch?v={}", video_id));
    }

    None
}

pub struct ApodService {
    api_key: String,
    env: Env,
}

impl ApodService {
    pub fn new(env: &Env) -> Result<Self, Error> {
        let api_key = env.var("NASA_API_KEY")?.to_string();

        Ok(Self {
            api_key,
            env: env.clone(),
        })
    }

    async fn fetch_apod_with_retries(&self, max_attempts: u32) -> Result<ApodResponse, Error> {
        let mut attempts = 0;
        while attempts < max_attempts {
            match self.fetch_data().await {
                Ok(data) => return Ok(data),
                Err(e) => {
                    attempts += 1;
                    worker::console_warn!("Tentativo {} fallito: {:?}. Riprovo...", attempts, e);
                    worker::Delay::from(Duration::from_secs(5 * attempts as u64)).await;
                }
            }
        }
        Err(Error::Generic(
            "NASA API irraggiungibile dopo N tentativi".into(),
        ))
    }

    async fn fetch_data(&self) -> Result<ApodResponse, Error> {
        let response = CLIENT
            .get(format!("{}?api_key={}", API_URL, self.api_key))
            .timeout(Duration::from_secs(15))
            .send()
            .await
            .map_err(Error::ReqwestError)?
            .text()
            .await
            .map_err(Error::ReqwestError)?;

        serde_json::from_str(&response).map_err(Error::JsonFailed)
    }

    pub async fn get_apod(&self) -> Result<ApodResponse, Error> {
        let kv = self.env.kv(KV_BINDING)?;

        let date = Utc::now().format("%Y-%m-%d").to_string();

        if let Some(cached_data) = kv.get(&format!("nasa:apod:{date}")).json::<ApodResponse>().await? {
            return Ok(cached_data);
        }

        let apod_data = self.fetch_apod_with_retries(3).await?;
        kv.put(&format!("nasa:apod:{date}"), serde_json::to_string(&apod_data)?)?
            .expiration_ttl(86400)
            .execute()
            .await?;

        Ok(apod_data)
    }

    pub fn build_embed(data: ApodResponse) -> Embed {
        let mut embed = Embed::new();
        embed.set_color("#4889D8");
        embed.set_title(&data.title);
        embed.set_description(data.explanation.replace(". ", ". \n\n"));
        embed.set_footer(
            "Resource provided by NASA APOD api",
            Some("https://api.nasa.gov/assets/img/favicons/favicon-192.png".into()),
        );

        embed.set_author(
            data.copyright.unwrap_or("NASA".into()),
            Some("https://api.nasa.gov/assets/img/favicons/favicon-192.png".into()),
            Some("https://nasa.gov".into()),
        );

        if let Some(concepts) = data.concepts {
            embed.add_field("Concepts", concepts.join(","), false);
        }

        match data.media_type {
            ApodMediaType::Video => {
                let resource_url = convert_to_watch_url(&data.url).unwrap_or(data.url);
                embed.set_url(&resource_url);
                embed.set_video(&resource_url);
            }
            ApodMediaType::Image => {
                embed.set_url(&data.url);
                embed.set_image(&data.url);
            }
        };

        embed
    }
}
