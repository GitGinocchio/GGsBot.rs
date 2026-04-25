use std::collections::HashMap;

use async_trait::async_trait;
use twilight_model::{
    application::interaction::{Interaction, message_component::MessageComponentInteractionData}, http::interaction::InteractionResponse,
};
use worker::RouteContext;

use crate::{error::Error, framework::discord::command::Command};

pub type UiHandlerMap = HashMap<String, Box<dyn UiHandler + Send + Sync>>;

#[async_trait(?Send)]
#[allow(unused)]
pub trait UiHandler {
    fn id(&self) -> &str;

    async fn handle(
        &self,
        interaction: &Interaction,
        ctx: &mut RouteContext<()>,
        data: &Box<MessageComponentInteractionData>,
        target: String,
    ) -> Result<InteractionResponse, Error>;
}

#[macro_export]
macro_rules! build_uihandlers {
    ($($command_type:ty),*) => {
        {
            #[allow(unused_mut)]
            let mut map: $crate::framework::traits::ui::UiHandlerMap = std::collections::HashMap::new();
            $(
                let handler: Box<dyn $crate::framework::traits::ui::UiHandler + Send + Sync> =
                    Box::new(<$command_type>::default());

                map.insert(handler.id().into(), handler);
            )*
            map
        }
    };
}
