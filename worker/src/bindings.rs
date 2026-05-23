use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[allow(unused)]
pub static KV_NAMESPACE_BINDING: &'static str = "ggsbotkv";

#[derive(Debug, Serialize, Deserialize, Display, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[allow(unused)]
pub enum QueueBinding {
    #[serde(rename = "ggsbotrs-gateway-queue")]
    #[strum(serialize = "ggsbotrs-gateway-queue")]
    Gateway,

    #[serde(rename = "ggsbotrs-tasks-queue")]
    #[strum(serialize = "ggsbotrs-tasks-queue")]
    Tasks,
}

#[derive(Debug, Serialize, Deserialize, Display, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[allow(unused)]
pub enum RatelimiterBinding {
    #[serde(rename = "gateway-rate-limiter")]
    #[strum(serialize = "gateway-rate-limiter")]
    Gateway
}