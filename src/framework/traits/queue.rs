use async_trait::async_trait;
use std::collections::HashMap;
use worker::{Context, Env, MessageBatch};

use crate::error::Error;

pub type QueueMap = HashMap<String, Box<dyn Queue + Send + Sync>>;

#[macro_export]
macro_rules! build_queue_handlers {
    ($($handler_type:ty),*) => {
        {
            #[allow(unused_mut)]
            let mut map: $crate::framework::traits::queue::QueueMap = std::collections::HashMap::new();
            $(
                let handler: Box<dyn $crate::framework::traits::queue::Queue + Send + Sync> =
                    Box::new(<$handler_type>::default());

                map.insert(handler.name().into(), handler);
            )*
            map
        }
    };
}

#[allow(unused)]
#[async_trait(?Send)]
pub trait Queue {
    fn name(&self) -> &str;

    async fn handle(
        &self,
        batch: MessageBatch<serde_json::Value>,
        env: &Env,
        ctx: &Context,
    ) -> Result<(), Error>;
}
