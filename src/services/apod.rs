use std::time::Duration;

use serde::Deserialize;
use twilight_model::channel::message::Embed;
use url::Url;

use crate::{CLIENT, discord::embed::EmbedExt, error::Error};

const API_URL: &'static str = "https://api.nasa.gov/planetary/apod";

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ApodMediaType {
    Video,
    Image
}

#[derive(Deserialize, Debug)]
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

pub struct ApodService;

impl ApodService {
    pub async fn fetch_data(api_key: &str) -> Result<ApodResponse, Error> {
        let response = CLIENT.get(format!("{}?api_key={}", API_URL, api_key))
            .timeout(Duration::from_secs(15))
            .send()
            .await
            .map_err(Error::ReqwestError)?
            .text()
            .await
            .map_err(Error::ReqwestError)?;

        serde_json::from_str(&response).map_err(Error::JsonFailed)
    }

    pub fn build_embed(data: ApodResponse) -> Embed {
        let mut embed = Embed::new();
        embed.set_color("#4889D8");
        embed.set_title(&data.title);
        embed.set_description(data.explanation.replace(". ", ". \n\n"));
        embed.set_footer(
            "Resource provided by NASA APOD api", 
            Some("https://api.nasa.gov/assets/img/favicons/favicon-192.png".into())
        );

        embed.set_author(
            data.copyright.unwrap_or("NASA".into()), 
            Some("https://api.nasa.gov/assets/img/favicons/favicon-192.png".into()), 
            Some("https://nasa.gov".into())
        );

        if let Some(concepts) = data.concepts {
            embed.add_field("Concepts", concepts.join(","), false);
        }

        match data.media_type { 
            ApodMediaType::Video => {
                let resource_url = convert_to_watch_url(&data.url).unwrap_or(data.url);
                embed.set_url(&resource_url);
                embed.set_video(&resource_url);
            },
            ApodMediaType::Image => {
                embed.set_url(&data.url);
                embed.set_image(&data.url);
            }
        };

        embed
    }
}