use twilight_model::application::interaction::Interaction;
use worker::{Request, Response, RouteContext};

use crate::{
    error::Error,
    framework::discord::{interaction::InteractionExt, verification::verify_signature},
};

pub struct Bot {
    ctx: RouteContext<()>,
}

impl Bot {
    pub fn new(ctx: RouteContext<()>) -> Self {
        Self { ctx }
    }

    pub async fn handle(&mut self, mut req: Request) -> worker::Result<Response> {
        let ray_id = req
            .headers()
            .get("cf-ray")?
            .unwrap_or_else(|| "unknown".to_string());

        let body = match self.validate_signature(&mut req).await {
            Err(e) => return Response::from_json(&e.as_interaction(&ray_id)),
            Ok(b) => b,
        };

        let interaction: Interaction = match serde_json::from_str(&body).map_err(Error::JsonFailed)
        {
            Err(e) => return Response::from_json(&e.as_interaction(&ray_id)),
            Ok(i) => i,
        };

        worker::console_debug!("[RayID: {ray_id}] Interaction started");
        match interaction.perform(&mut self.ctx).await {
            Ok(response) => {
                worker::console_log!(
                    "[RayID: {ray_id}] Response : {}",
                    serde_json::to_string_pretty(&response).unwrap()
                );
                Response::from_json(&response)
            }
            Err(e) => {
                worker::console_error!("[RayID: {ray_id}] Error: {e:?}");
                let response = e.as_interaction(&ray_id);
                match interaction.edit(&response).await {
                    Ok(r) => return Response::from_json(&r),
                    Err(e) => worker::console_warn!("[RayID: {ray_id}] Error (edit message): {e}"),
                }
                Response::from_json(&response)
            }
        }
    }

    async fn validate_signature(&self, req: &mut Request) -> Result<String, Error> {
        let pubkey = self
            .ctx
            .var("DISCORD_PUBLIC_KEY")
            .map_err(|e| Error::EnvironmentVariableNotFound(e.to_string()))?
            .to_string();

        let signature = req
            .headers()
            .get("x-signature-ed25519")
            .map_err(|e| Error::HeaderNotFound(e.to_string()))?
            .ok_or_else(|| Error::HeaderNotFound("x-signature-ed25519".into()))?;

        let timestamp = req
            .headers()
            .get("x-signature-timestamp")
            .map_err(|e| Error::HeaderNotFound(e.to_string()))?
            .ok_or_else(|| Error::HeaderNotFound("x-signature-timestamp".into()))?;

        let body = req
            .text()
            .await
            .map_err(|_| Error::InvalidPayload("Body read failed".into()))?;

        verify_signature(&pubkey, &signature, &timestamp, &body)
            .map_err(|e| Error::VerificationFailed(e.to_string()))?;

        Ok(body)
    }
}
