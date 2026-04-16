use twilight_model::{application::interaction::Interaction, http::interaction::InteractionResponse};
use worker::{Request, RouteContext};

use crate::{discord::{interaction::InteractionExt, verification::verify_signature}, error::Error};

pub struct Bot {
    ctx: RouteContext<()>,
}

impl Bot {
    pub fn new(ctx: RouteContext<()>) -> Self {
        Self { ctx }
    }

    pub async fn handle(&mut self, mut req: Request) -> Result<InteractionResponse, Error> {
        let body = self.validate_signature(&mut req).await?;
        
        let interaction: Interaction = serde_json::from_str(&body)
            .map_err(Error::JsonFailed)?;

        let response = interaction.perform(&mut self.ctx).await?;
        
        Ok(response)
    }

    async fn validate_signature(&self, req: &mut Request) -> Result<String, Error> {
        let pubkey = self.ctx.var("DISCORD_PUBLIC_KEY")
            .map_err(|e| Error::EnvironmentVariableNotFound(e.to_string()))?
            .to_string();
        
        let signature = req.headers().get("x-signature-ed25519")
            .map_err(|e| Error::HeaderNotFound(e.to_string()))?
            .ok_or_else(|| Error::HeaderNotFound("x-signature-ed25519".into()))?;
        
        let timestamp = req.headers().get("x-signature-timestamp")
            .map_err(|e| Error::HeaderNotFound(e.to_string()))?
            .ok_or_else(|| Error::HeaderNotFound("x-signature-timestamp".into()))?;

        let body = req.text().await.map_err(|_| Error::InvalidPayload("Body read failed".into()))?;
        
        verify_signature(&pubkey, &signature, &timestamp, &body)
            .map_err(|e| Error::VerificationFailed(e))?;
            
        Ok(body)
    }
}