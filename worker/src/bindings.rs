use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[allow(unused)]
pub static KV_BINDING: &'static str = "ggsbotkv";

#[derive(Debug, Serialize, Deserialize, Display, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[allow(unused)]
pub enum QueueBinding {
    #[serde(rename = "GATEWAY_QUEUE")]
    #[strum(serialize = "GATEWAY_QUEUE")]
    Gateway,

    #[serde(rename = "TASKS_QUEUE")]
    #[strum(serialize = "TASKS_QUEUE")]
    Tasks,
}

#[derive(Debug, Serialize, Deserialize, Display, PartialEq)]
#[allow(unused)]
pub enum DurableObjectBinding {
    #[serde(rename = "GATEWAY")]
    #[strum(serialize = "GATEWAY")]
    Gateway 
}

#[derive(Debug, Serialize, Deserialize, Display, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[allow(unused)]
pub enum RatelimiterBinding {
    #[serde(rename = "gateway-rate-limiter")]
    #[strum(serialize = "gateway-rate-limiter")]
    Gateway
}