use std::time::Duration;

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

use crate::{CLIENT, discord::{command::Command, embed::{EmbedBuilder, EmbedExt}, interaction::InteractionExt, response::ResponseBuilder}, error::Error, services::apod::ApodService};

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

        let data = ApodService::fetch_data(&api_key).await?;
        let embed = ApodService::build_embed(data);

        let response = ResponseBuilder::new(InteractionResponseType::ChannelMessageWithSource)
            .embeds(vec![embed])
            .ephemeral()
            .build();

        interaction.edit(&response).await

    }
}