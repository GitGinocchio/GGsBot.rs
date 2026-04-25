use crate::error::Error;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};

pub(crate) fn verify_signature(
    public_key_hex: &str,
    signature_hex: &str,
    timestamp: &str,
    body: &str,
) -> Result<(), Error> {
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
