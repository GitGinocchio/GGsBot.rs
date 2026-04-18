use async_trait::async_trait;
use twilight_model::{application::interaction::Interaction, channel::message::Component, http::interaction::InteractionResponse};
use worker::RouteContext;

use crate::error::Error;


#[async_trait(?Send)]
#[allow(unused)]
pub trait InteractiveComponent {
    // L'ID univoco che registri nella tua HashMap globale
    fn custom_id(&self) -> String;
    
    // Il testo sul bottone
    async fn label(&self) -> String;

    // Costruisce il componente Twilight
    async fn build(&self) -> Component;

    // Il callback eseguito al click
    async fn handle(
        &self, 
        interaction: &Interaction, 
        ctx: &mut RouteContext<()>
    ) -> Result<InteractionResponse, Error>;
}