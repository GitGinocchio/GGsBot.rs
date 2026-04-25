use std::num::ParseIntError;

use hex::FromHexError;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};

use crate::{
    framework::discord::response::ResponseBuilder,
    ui::embeds::error::{BUG_ICON, ERROR_EMBED},
};

#[allow(unused)]
#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("Environment variable '{0}' not found.")]
    EnvironmentVariableNotFound(String),

    #[error("Header '{0}' not found.")]
    HeaderNotFound(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Failed to parse integer: {0:?}")]
    ParseIntError(#[from] ParseIntError),

    #[error("Failed to parse from hex: {0:?}")]
    ParseHexFailed(#[from] FromHexError),

    #[error("Invalid public key or signature format: {0:?}")]
    CryptoError(#[from] ed25519_dalek::SignatureError),

    #[error("JSON error: {0:?}")]
    JsonFailed(#[from] serde_json::Error),

    #[error("Invalid payload: {0}")]
    InvalidPayload(String),

    #[error("Verification failed: {0}")]
    VerificationFailed(String),

    #[error("Interaction failed: {0}")]
    InteractionFailed(String),

    #[error("Cloudflare worker error: {0:?}")]
    WorkerError(#[from] worker::Error),

    #[error("Cloudflare worker kv error: {0:?}")]
    KvError(#[from] worker::KvError),

    #[error("Reqwest error: {0:?}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Error communicating with {0}")]
    UpstreamError(String),

    #[error("Error: {0}")]
    Generic(String),
}

impl From<Error> for worker::Error {
    fn from(err: Error) -> Self {
        worker::Error::from(err.to_string())
    }
}

#[allow(unused)]
impl Error {
    pub fn status_code(&self) -> u16 {
        match self {
            Self::VerificationFailed(_) => 401,
            Self::HeaderNotFound(_)
            | Self::InvalidPayload(_)
            | Self::EnvironmentVariableNotFound(_) => 400,
            Self::JsonFailed(_) => 400,
            _ => 500,
        }
    }

    pub fn as_interaction(&self, ray_id: &str) -> InteractionResponse {
        let (title, description) = match self {
            _ => ("Ops!", "Something went wrong!"),
        };

        let embed = ERROR_EMBED.clone()
            .title(title)
            .description(description)
            .field(
                "Need help?", 
                "This error has been logged, if you need additional support you can write to a moderator by sending the code below",
                false
            )
            .footer(
                format!("Error code: {ray_id}"), 
                Some(BUG_ICON.into())
            )
            .build();

        ResponseBuilder::new(InteractionResponseType::ChannelMessageWithSource)
            .embeds(vec![embed])
            .components(vec![])
            .ephemeral()
            .build()
    }
}
