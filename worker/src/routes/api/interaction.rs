use worker::{Request, Result, Response, RouteContext};
use crate::framework::discord::bot::Bot;

pub async fn post(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    Bot::new(ctx).handle(req).await
}