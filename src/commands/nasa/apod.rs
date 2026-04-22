use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use twilight_model::{
    application::interaction::{
        Interaction, 
        application_command::CommandData
    }, channel::{Attachment, message::Embed}, guild::Permissions, http::interaction::{
        InteractionResponse, 
        InteractionResponseType
    }, oauth::ApplicationIntegrationType, util::Timestamp
};
use url::Url;
use worker::RouteContext;

use crate::{CLIENT, discord::{command::Command, embed::{EmbedBuilder, EmbedExt}, interaction::InteractionExt, response::ResponseBuilder}, error::Error};

const API_URL: &'static str = "https://api.nasa.gov/planetary/apod";

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum ApodMediaType {
    Video,
    Image
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
struct ApodResponse {
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

#[derive(Default)]
pub(crate) struct Apod {}

#[async_trait(?Send)]
impl Command for Apod {
    fn name(&self) -> String { "apod".into() }

    fn description(&self) -> String { "Astronomy picture of the day!".into() }

    fn integration_types(&self) -> Vec<ApplicationIntegrationType> {
        vec![ApplicationIntegrationType::GuildInstall, ApplicationIntegrationType::UserInstall]
    }

    async fn respond(
        &self, 
        interaction: &Interaction,
        _data: &CommandData,
        ctx: &mut RouteContext<()>
    ) -> Result<InteractionResponse, Error> {
        interaction.defer(true).await?;

        let api_key = ctx.var("NASA_API_KEY")
            .map_err(|e| Error::EnvironmentVariableNotFound(e.to_string()))?
            .to_string();

        let response = CLIENT.get(format!("{}?api_key={}", API_URL, api_key))
            .send()
            .await
            .map_err(|e| Error::ReqwestError(e))?
            .text()
            .await
            .map_err(|e| Error::ReqwestError(e))?;

        let response: ApodResponse = serde_json::from_str(&response)
            .map_err(|e| Error::JsonFailed(e))?;

        let mut embed = Embed::new();
        embed.set_color("#4889D8");
        embed.set_title(&response.title);
        embed.set_description(response.explanation.replace(". ", ". \n\n"));
        embed.set_footer(
            "Resource provided by NASA APOD api", 
            Some("https://api.nasa.gov/assets/img/favicons/favicon-192.png".into())
        );

        embed.set_author(
            response.copyright.unwrap_or("NASA".into()), 
            Some("https://api.nasa.gov/assets/img/favicons/favicon-192.png".into()), 
            Some("https://nasa.gov".into())
        );

        if let Some(concepts) = response.concepts {
            embed.add_field("Concepts", concepts.join(","), false);
        }

        match response.media_type { 
            ApodMediaType::Video => {
                let resource_url = convert_to_watch_url(&response.url).unwrap_or(response.url);
                embed.set_url(&resource_url);
                embed.set_video(&resource_url);
            },
            ApodMediaType::Image => {
                embed.set_url(&response.url);
                embed.set_image(&response.url);
            }
        };

        let response = ResponseBuilder::new(InteractionResponseType::ChannelMessageWithSource)
            .embeds(vec![embed])
            .ephemeral()
            .build();

        interaction.edit(&response).await

    }
}