use async_trait::async_trait;
use worker::{Env};

use crate::{error::Error, framework::structs::queue::QueueMessage};



#[async_trait(?Send)]
#[allow(unused)]
pub trait MessageHandler: Sized {
    type Payload: TryFrom<QueueMessage, Error = Error>;

    async fn setup(env: &Env) -> Result<Self, Error>;
    async fn handle(&self, payload: &Self::Payload) -> Result<(), Error>;
}