use async_trait::async_trait;
use twilight_model::{application::interaction::Interaction, http::interaction::{InteractionResponse, InteractionResponseType}};
use worker::RouteContext;

use crate::{components::navbar::NavBar, discord::{component::CustomComponent, response::ResponseBuilder}, error::InteractionError, traits::page::Page};


#[derive(Default)]
pub struct HelloSetupPage {}

#[async_trait(?Send)]
impl Page for HelloSetupPage {
    fn id(&self) -> String {
        "hello".into()
    }

    async fn handle(
        &self,
        interaction: &Interaction, 
        ctx: &mut RouteContext<()>,
        maybe_target: Option<String>
    ) -> Result<InteractionResponse, InteractionError> {
        worker::console_debug!("target: {maybe_target:?}");
        let mut navbar = NavBar::new(self.id(), None);

        // Base case, first handling
        let Some(target) = maybe_target else {
            let component = navbar.build();

            return Ok(ResponseBuilder::new(InteractionResponseType::ChannelMessageWithSource)
                .content(format!("Pagina numero: 0"))
                .components(vec![component])
                .build())
        };

        let parts: Vec<&str> = target.split(':').collect();

        match parts.as_slice() {
            ["nav",page, "next"] => {
                let page: u8 = page.parse().map_err(|_e| InteractionError::GenericError())?;
                navbar.set_page(page);
                let next_page = navbar.next_page();

                let component = navbar.build();
                return Ok(ResponseBuilder::new(InteractionResponseType::UpdateMessage)
                    .components(vec![component])
                    .content(format!("Pagina numero: {next_page}"))
                    .build());
            },
            ["nav",page, "back"] => {
                let page: u8 = page.parse().map_err(|_e| InteractionError::GenericError())?;
                navbar.set_page(page);
                let prev_page = navbar.prev_page();

                let component = navbar.build();
                return Ok(ResponseBuilder::new(InteractionResponseType::UpdateMessage)
                    .components(vec![component])
                    .content(format!("Pagina numero: {prev_page}"))
                    .build());
            },
            default => {
                worker::console_debug!("unexpected target: {default:?}")
            }

        }

        Ok(ResponseBuilder::new(InteractionResponseType::ChannelMessageWithSource)
            .content("Interaction received!")
            .build())
    }
}