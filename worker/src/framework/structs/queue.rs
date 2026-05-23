use serde::{Deserialize, Serialize};
use worker::*;

use crate::{
    error::Error,
    framework::traits::queue::MessageHandler, queues::{apod::{ApodMessageHandler, ApodQueueMessage}, tempvc::{TempvcDeleteChannelHandler, TempvcDeleteChannelMessage}}
};

macro_rules! define_queue_message {
    ($name:ident { $($variant:ident($payload:ty)),* $(,)? }) => {
        #[derive(Debug, Serialize, Deserialize, Clone)]
        #[serde(tag = "type", content = "data")]
        #[serde(rename_all = "kebab-case")]
        pub enum $name {
            $($variant($payload)),*
        }

        $(
            // Conversione per il valore: impl TryFrom<QueueMessage> for ApodQueueMessage
            impl TryFrom<$name> for $payload {
                type Error = Error;

                fn try_from(value: $name) -> Result<Self, Self::Error> {
                    match value {
                        $name::$variant(data) => Ok(data),
                        #[allow(unreachable_patterns)]
                        _ => Err(Error::Generic(format!(
                            "Expected variant {} from {}, but found another", 
                            stringify!($variant), 
                            stringify!($name)
                        ))),
                    }
                }
            }

            // Conversione per il riferimento: impl TryFrom<&QueueMessage> for &ApodQueueMessage
            impl<'a> TryFrom<&'a $name> for &'a $payload {
                type Error = Error;

                fn try_from(value: &'a $name) -> Result<Self, Self::Error> {
                    match value {
                        $name::$variant(data) => Ok(data),
                        #[allow(unreachable_patterns)]
                        _ => Err(Error::Generic(format!(
                            "Expected variant {} from {}, but found another", 
                            stringify!($variant), 
                            stringify!($name)
                        ))),
                    }
                }
            }
        )*
    };
}

define_queue_message!(QueueMessage {
    Apod(ApodQueueMessage),
    Tempvc(TempvcDeleteChannelMessage)
});

impl QueueMessage {
    pub async fn dispatch_batch(
        &self, 
        batch: &[Message<QueueMessage>], 
        env: &Env
    ) -> Result<(), Error> {
        match self {
            QueueMessage::Apod(_) => self.run_batch::<ApodMessageHandler>(batch, env).await,
            QueueMessage::Tempvc(_) => self.run_batch::<TempvcDeleteChannelHandler>(batch, env).await
        }
    }

    /// Motore interno che orchestra il batch per un tipo specifico di Handler
    async fn run_batch<H>(&self, batch: &[Message<QueueMessage>], env: &Env) -> Result<(), Error> 
    where 
        H: MessageHandler,
        for<'a> &'a H::Payload: TryFrom<&'a QueueMessage, Error = Error>
    {
        // Setup dell'handler specifico (es. ApodHandler)
        let handler = H::setup(env).await?;
        
        let tasks = batch.iter().map(|msg| async {
            let Ok(payload) = msg.body().try_into() else {
                msg.ack();
                return Err(Error::InvalidPayload(format!("Invalid payload for queue message!")))
            };
        
            match handler.handle(payload).await {
                Ok(_) => {
                    msg.ack();
                    Ok(())
                },
                Err(e) => {
                    msg.retry();
                    Err(e)
                }
            }
        });

        futures::future::join_all(tasks).await;
        Ok(())
    }
}

#[allow(unused)]
pub struct QueueProcessor {
    env: Env,
    ctx: Context,
}

impl QueueProcessor {
    pub fn new(env: Env, ctx: Context) -> Self {
        Self { env, ctx }
    }

    pub async fn process(&self, batch: MessageBatch<QueueMessage>) -> Result<()> {
        let messages = batch.messages()?;
        
        if let Some(first_msg) = messages.first() {
            first_msg.body().dispatch_batch(&messages, &self.env).await?;
        }

        Ok(())
    }
}