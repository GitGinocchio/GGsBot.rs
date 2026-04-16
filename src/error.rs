use crate::discord::verification::VerificationError;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("Environment variable '{0}' not found.")]
    EnvironmentVariableNotFound(String),

    #[error("Header '{0}' not found.")]
    HeaderNotFound(String),

    #[error("JSON error: {0}")]
    JsonFailed(#[from] serde_json::Error),

    #[error("Invalid payload: {0}")]
    InvalidPayload(String),

    #[error("Verification failed: {0}")]
    VerificationFailed(#[from] VerificationError),

    #[error("Interaction failed: {0}")]
    InteractionFailed(#[from] InteractionError)
}

impl Error {
    pub fn status_code(&self) -> u16 {
        match self {
            Self::VerificationFailed(_) => 401,
            Self::HeaderNotFound(_) | Self::InvalidPayload(_) | Self::EnvironmentVariableNotFound(_) => 400,
            
            Self::JsonFailed(_) => 400,

            Self::InteractionFailed(i_err) => i_err.status_code(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum InteractionError {
    #[error("Error communicating with {0}")]
    #[allow(dead_code)]
    UpstreamError(String),

    #[error("Command not found: {0}")]
    #[allow(dead_code)]
    UnknownCommand(String),

    #[error("Something went wrong")]
    GenericError(),

    #[error("Cloudflare worker error: {0}")]
    WorkerError(#[from] worker::Error),

    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error)
}

impl InteractionError {
    pub fn status_code(&self) -> u16 {
        match self {
            Self::UnknownCommand(_) => 404,
            Self::UpstreamError(_) => 502,
            _ => 500, // GenericError e WorkerError
        }
    }
}