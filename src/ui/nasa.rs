use async_trait::async_trait;
use serde::Serialize;
use twilight_model::{
    application::interaction::{Interaction, message_component::MessageComponentInteractionData},
    http::interaction::InteractionResponse,
};
use worker::RouteContext;

use crate::{
    commands::nasa::NasaExtConfig, error::Error, framework::{discord::{interaction::InteractionExt, response::InteractionResponseExt}, structs::config::extension::ExtensionConfig, traits::{component::CustomComponent, namespaces::KvExt, page::Page, ui::UiHandler}}, ui::components::navbar::SetupExtNavBar
};

#[derive(Default)]
pub struct NasaUIHandler;

impl NasaUIHandler {
    pub async fn render(&self, num_page: u8, disable_confirm: bool) -> Result<InteractionResponse, Error> {
        let mut navbar = SetupExtNavBar::new(self.id(), None, Some(0), disable_confirm);
        navbar.set_page(num_page);

        let page = crate::ui::pages::nasa::NasaSetupPage::new(self.id().to_string());

        let mut response = page.render().await?;
        response.push_component(navbar.build());

        Ok(response)
    }
}

#[async_trait(?Send)]
impl UiHandler for NasaUIHandler {
    fn id(&self) -> &str {
        "nasa"
    }

    async fn handle(
        &self,
        interaction: &Interaction,
        ctx: &mut RouteContext<()>,
        data: &Box<MessageComponentInteractionData>,
        target: String,
    ) -> Result<InteractionResponse, Error> {
        worker::console_debug!("target: {target:?}");
        interaction.defer(true).await?;

        let parts: Vec<&str> = target.split(':').collect();

        let mut render_page = false;
        let mut disable_confirm = true;

        match parts.as_slice() {
            ["p1", "channel"] => {
                let guild_kv = interaction.guild_kv(&ctx.env)?;

                // TODO: Gestire il caso in cui non venga trovvato :pending
                if let Some(ext_data) = guild_kv.get("extensions:nasa:config:pending").await? {
                    let mut ext_config: ExtensionConfig<NasaExtConfig> = serde_json::from_str(&ext_data)?;
                    let nasa_config = ext_config.config.get_or_insert_default();

                    nasa_config.channel_id = data.values.first().cloned();

                    if nasa_config.channel_id.is_some() {
                        disable_confirm = false;
                    }

                    let serialized = serde_json::to_string(&ext_config)?;
                    guild_kv.put("extensions:nasa:config:pending", serialized, Some(3600)).await?;
                }

                render_page = true;
            },
            ["nav", _, "confirm"] => {
                let guild_kv = interaction.guild_kv(&ctx.env)?;
                guild_kv.duplicate(
                    "extensions:nasa:config:pending", 
                    "extensions:nasa:config", 
                    None
                ).await?;
                return interaction.delete(ctx).await;
            },
            ["nav", _, "cancel"] => {
                return interaction.delete(ctx).await;
            }
            _ => {},
        };

        if render_page {
            let response = self.render(0, disable_confirm).await?;
            return interaction.edit(&response).await;
        }

        interaction.edit(&InteractionResponse::empty()).await
    }
}
