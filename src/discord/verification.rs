use std::convert::TryInto as _;

use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use hex::FromHexError;

#[derive(Debug, thiserror::Error)]
pub(crate) enum VerificationError {
    #[error("Failed to parse from hex.")]
    ParseHexFailed(#[from] FromHexError),

    #[error("Invalid public key or signature format.")]
    CryptoError(#[from] ed25519_dalek::SignatureError),
}

pub(crate) fn verify_signature(
    public_key_hex: &str,
    signature_hex: &str,
    timestamp: &str,
    body: &str,
) -> Result<(), VerificationError> {
    let public_key_bytes = hex::decode(public_key_hex)?;

    let public_key_array: [u8; 32] = public_key_bytes
        .try_into()
        .map_err(|_| ed25519_dalek::SignatureError::new())?;
    
    let verifying_key = VerifyingKey::from_bytes(&public_key_array)?;

    let signature_bytes = hex::decode(signature_hex)?;
    let signature = Signature::from_slice(&signature_bytes)?;

    let message = format!("{}{}", timestamp, body);

    verifying_key.verify(message.as_bytes(), &signature)?;

    Ok(())
}