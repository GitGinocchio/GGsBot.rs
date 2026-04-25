use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use twilight_model::{application::interaction::Interaction, http::interaction::InteractionResponse, oauth::ApplicationIntegrationType};
use worker::RouteContext;

use crate::{
    build_commands, error::Error, framework::{discord::{command::{Command, CommandMap}, interaction::InteractionExt, response::{InteractionResponseExt, ResponseBuilder}}, traits::command::CommandController}
};

mod apod;

#[derive(Default, Serialize, Deserialize)]
pub struct NasaExtConfig {
    pub channel_id: Option<String>
}

#[derive(Default)]
pub(crate) struct Nasa {}

#[async_trait(?Send)]
impl CommandController for Nasa {
    async fn get_default_config(
        &self,
        _interaction: &Interaction,
        _ctx: &mut RouteContext<()>,
    ) -> Option<serde_json::Value> {
        serde_json::to_value(NasaExtConfig::default()).ok()
    }

    async fn on_setup(
        &self,
        interaction: &Interaction,
        _ctx: &mut RouteContext<()>,
    ) -> Option<Result<InteractionResponse, Error>> {
        match interaction.defer(true).await {
            Ok(_) => {},
            Err(e) => return Some(Err(e))
        };
        
        let page = crate::ui::nasa::NasaUIHandler::default();
        let response = match page.render(0, true).await {
            Ok(res) => res,
            Err(e) => return Some(Err(e)),
        };

        Some(interaction.edit(&response).await)
    }
}

#[async_trait(?Send)]
impl Command for Nasa {
    fn name(&self) -> String {
        "nasa".into()
    }

    fn description(&self) -> String {
        "Gestisci i comandi del bot!".into()
    }

    fn integration_types(&self) -> Vec<ApplicationIntegrationType> {
        vec![
            ApplicationIntegrationType::GuildInstall,
            ApplicationIntegrationType::UserInstall,
        ]
    }

    fn subcommands(&self) -> CommandMap {
        build_commands![apod::Apod]
    }

    fn get_controller(&self) -> Option<&dyn CommandController> {
        Some(self)
    }
}
