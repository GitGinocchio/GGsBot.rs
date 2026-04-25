use async_trait::async_trait;
use twilight_model::{application::interaction::Interaction, channel::{ChannelType, message::{Component, Embed, component::SelectMenuType}}, http::interaction::{InteractionResponse, InteractionResponseType}};
use worker::RouteContext;

use crate::{error::Error, framework::{discord::{embed::{EmbedBuilder, EmbedExt}, response::ResponseBuilder}, traits::page::Page}, ui::embeds::default::DEFAULT_EMBED};

pub struct NasaSetupPage {
    parent_id: String
}

impl NasaSetupPage {
    pub fn new(parent_id: String) -> Self {
        Self { parent_id }
    }
}

#[async_trait(?Send)]
impl Page for NasaSetupPage {
    fn id(&self) -> String {
        format!("{}:p1", self.parent_id)
    }

    async fn render(&self) -> Result<InteractionResponse, Error> {
        let mut components: Vec<Component> = Vec::new();

        let embed = DEFAULT_EMBED.clone()
            .title("APOD Configuration")
            .description("Use this page to configure settings related to the Astronomy Picture of the Day (APOD) program by NASA.")
            .field(
                "APOD updates channel", 
                "Select the channel where Astronomy Picture of the Day updates will be sent.", 
                false
            )
            .build();

        components.push(self.select_menu(
            "channel",
            SelectMenuType::Channel, 
            false, 
            Some("APOD updates channel".into()), 
            None, 
            None, 
            Some(1), 
            Some(0), 
            Some(true), 
            Some(vec![ChannelType::GuildText, ChannelType::GuildAnnouncement])
        ));

        Ok(ResponseBuilder::new(InteractionResponseType::ChannelMessageWithSource)
            .embeds(vec![embed])
            .components(vec![self.action_row(components)])
            .build())
    }
}