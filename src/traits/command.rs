use async_trait::async_trait;
use twilight_model::{application::interaction::Interaction, http::interaction::InteractionResponse};
use worker::RouteContext;

use crate::error::InteractionError;

#[async_trait(?Send)]
pub trait CommandController {
    #[allow(unused)]
    async fn get_default_config (
        &self, 
        interactio: &Interaction, 
        ctx: &mut RouteContext<()>
    ) -> Option<serde_json::Value> {
        None
    }

    #[allow(unused)]
    /// method called when this command is set up on a discord server
    async fn before_setup(&self, interaction: &Interaction, ctx: &mut RouteContext<()>) {
    }

    #[allow(unused)]
    async fn on_setup(
        &self, 
        interaction: &Interaction, 
        ctx: &mut RouteContext<()>
    ) -> Option<Result<InteractionResponse, InteractionError>> {
        None
    }

    #[allow(unused)]
    async fn after_setup(&self, interaction: &Interaction, ctx: &mut RouteContext<()>) {
    }

    /// method called when a command is removed from a discord server (act like a clean-up)
    #[allow(unused)]
    async fn before_teardown(&self, interaction: &Interaction, ctx: &mut RouteContext<()>) {}
    #[allow(unused)]
    async fn after_teardown(&self, interaction: &Interaction, ctx: &mut RouteContext<()>) {}

    /// method called when a command is enabled from a discord server
    #[allow(unused)]
    async fn on_enabled(&self, interaction: &Interaction, ctx: &mut RouteContext<()>) {}

    /// method called when a command is disabled from a discord server
    #[allow(unused)]
    async fn on_disabled(&self, interaction: &Interaction, ctx: &mut RouteContext<()>) {}

    #[allow(unused)]
    async fn on_teardown(
        &self, 
        interaction: &Interaction, 
        ctx: &mut RouteContext<()>
    ) -> Option<Result<InteractionResponse, InteractionError>> {
        None
    }
}