use std::num::ParseIntError;

use hex::FromHexError;

#[allow(unused)]
#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("Environment variable '{0}' not found.")]
    EnvironmentVariableNotFound(String),

    #[error("Header '{0}' not found.")]
    HeaderNotFound(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Failed to parse integer:")]
    ParseIntError(#[from] ParseIntError),

    #[error("Failed to parse from hex.")]
    ParseHexFailed(#[from] FromHexError),

    #[error("Invalid public key or signature format.")]
    CryptoError(#[from] ed25519_dalek::SignatureError),

    #[error("JSON error: {0}")]
    JsonFailed(#[from] serde_json::Error),

    #[error("Invalid payload: {0}")]
    InvalidPayload(String),

    #[error("Verification failed: {0}")]
    VerificationFailed(String),

    #[error("Interaction failed: {0}")]
    InteractionFailed(String),

    #[error("Cloudflare worker error: {0}")]
    WorkerError(#[from] worker::Error),

    #[error("Cloudflare worker kv error: {0}")]
    KvError(#[from] worker::KvError),

    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Error communicating with {0}")]
    UpstreamError(String),

    #[error("Error: {0}")]
    Generic(String)
}

impl Error {
    pub fn status_code(&self) -> u16 {
        match self {
            Self::VerificationFailed(_) => 401,
            Self::HeaderNotFound(_) | Self::InvalidPayload(_) | Self::EnvironmentVariableNotFound(_) => 400,
            Self::JsonFailed(_) => 400,
            _ => 500
        }
    }
}