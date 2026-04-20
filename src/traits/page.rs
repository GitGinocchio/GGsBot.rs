use std::collections::HashMap;

use async_trait::async_trait;
use twilight_model::{application::interaction::Interaction, channel::message::{Component, Embed}, http::interaction::{InteractionResponse, InteractionResponseType}};
use worker::RouteContext;

use crate::error::InteractionError;

pub type PageMap = HashMap<String, Box<dyn Page + Send + Sync>>;

#[async_trait(?Send)]
#[allow(unused)]
pub trait Page {
    fn id(&self) -> String;

    async fn handle(
        &self,
        interaction: &Interaction, 
        ctx: &mut RouteContext<()>,
        maybe_target: Option<String>
    ) -> Result<InteractionResponse, InteractionError>;
}

#[macro_export]
macro_rules! build_pages {
    ($($command_type:ty),*) => {
        {
            #[allow(unused_mut)]
            let mut map: $crate::traits::page::PageMap = std::collections::HashMap::new();
            $(
                let page: Box<dyn $crate::traits::page::Page + Send + Sync> = 
                    Box::new(<$command_type>::default()); 
                
                map.insert(page.id(), page);
            )*
            map
        }
    };
}