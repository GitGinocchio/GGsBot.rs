use serde_json::json;
use worker::{RateLimitOutcome, Request, Response, Result, RouteContext};

use crate::framework::structs::gateway::Gateway;

pub async fn post(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let event_kind = req.headers().get("X-Event-Kind")?.unwrap_or_else(|| "unknown".to_string());

    let rate_limiter = ctx.rate_limiter("gateway-rate-limiter")?;
    let RateLimitOutcome { success } = rate_limiter.limit(event_kind).await?;

    if !success {
        let payload = json!({
            "status": "error",
            "message": "Rate limit exceeded for gateway messages"
        });

        return Response::builder()
            .with_status(429)
            .from_json(&payload)
    }

    Gateway::new(ctx).handle_request(req).await
}