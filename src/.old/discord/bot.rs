//use crate::discord::interaction::{InteractionResponse, Interaction};
use crate::{discord::error::Error, traits::interaction::InteractionExt};
use crate::discord::verification::verify_signature;
use worker::{Request, RouteContext};

use twilight_model::{application::interaction::Interaction, http::interaction::InteractionResponse};


pub struct Bot {
    req: Request, 
    ctx: RouteContext<()>
}

impl Bot {

    pub fn new(req: Request, ctx: RouteContext<()>) -> Bot {
        Self {
            req, 
            ctx
        }
    }

    fn var(&self, key: &str) -> Result<String, Error> {
        match self.ctx.var(key) {
            Ok(var) =>  Ok(var.to_string()),
            Err(_) =>  Err(Error::EnvironmentVariableNotFound(key.to_string()))
        }

    }
    fn header(&self, key:&str) -> Result<String, Error> {
        match  self.req.headers().get(key) {
            Ok(val) => val.ok_or_else(|| Error::HeaderNotFound(key.to_string())),
            Err(_) => Err(Error::HeaderNotFound(key.to_string()))
        }
    }

    async fn validate_signature(&mut self) -> Result<String, Error> {
        let pubkey = self.var("DISCORD_PUBLIC_KEY")?;
        let signature = self.header("x-signature-ed25519")?;
        let timestamp = self.header("x-signature-timestamp")?;

        let body = self.req.text().await.map_err(|_| Error::InvalidPayload("".into()))?;
        verify_signature(&pubkey, &signature, &timestamp, &body).map_err(Error::VerificationFailed)?;
        Ok(body)
    }

    pub async fn handle_request(&mut self) -> Result<InteractionResponse, Error> {
        let body = self.validate_signature().await?;
        
        let interaction = serde_json::from_str::<Interaction>(&body)
            .map_err(Error::JsonFailed)?;

        worker::console_log!{"Request parsed : {}", serde_json::to_string_pretty(&interaction).unwrap()};
        let response = interaction.perform(&mut self.ctx).await?;
        
        Ok(response)

    }

}