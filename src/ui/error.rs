use async_trait::async_trait;
use twilight_model::{application::interaction::Interaction, http::interaction::{InteractionResponse, InteractionResponseType}};
use worker::RouteContext;

use crate::{
    ui::components::navbar::NavBar,
    discord::{
        interaction::InteractionExt, 
        response::ResponseBuilder
    }, 
    error::Error,
    traits::{
        component::CustomComponent, 
        ui::UiHandler
    }
};


#[derive(Default)]
pub struct HelloUiHandler {}

impl HelloUiHandler {
    pub async fn render(&self, page: u8) -> InteractionResponse {
        let mut navbar = NavBar::new(self.id(), None, Some(5));
        navbar.set_page(page);
        let component = navbar.build();

        ResponseBuilder::new(InteractionResponseType::ChannelMessageWithSource)
            .components(vec![component])
            .content(format!("Pagina numero: {page}"))
            .build()
    }
}

#[async_trait(?Send)]
impl UiHandler for HelloUiHandler {
    fn id(&self) -> String {
        "hello".into()
    }

    async fn handle(
        &self,
        interaction: &Interaction, 
        ctx: &mut RouteContext<()>,
        target: String
    ) -> Result<InteractionResponse, Error> {
        worker::console_debug!("target: {target:?}");
        interaction.defer(true).await?;

        let parts: Vec<&str> = target.split(':').collect();

        let new_page = match parts.as_slice() {
            ["nav", page, "next"] => page.parse::<u8>().unwrap_or(0) + 1,
            ["nav", page, "back"] => page.parse::<u8>().unwrap_or(0).saturating_sub(1),
            ["nav", _, "cancel"] => {
                return interaction.delete(ctx).await;
            },
            _ => 0,
        };

        let response = self.render(new_page).await;
        interaction.edit(&response).await
    }
}