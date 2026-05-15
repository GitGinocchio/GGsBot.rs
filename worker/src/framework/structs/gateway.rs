use serde_json::{json, Value};
use worker::{Request, Response, Result, RouteContext};



pub struct Gateway {
    ctx: RouteContext<()>
}

impl Gateway {
    pub fn new(ctx: RouteContext<()>) -> Self {
        Self { ctx: ctx }
    }

    pub async fn handle_request(&self, mut req: Request) -> Result<Response> {
        let data: Value = req.json().await?;
        worker::console_debug!("gateway data received: {data:?}");
        Response::from_json(&json!({
            "status" : "success"
        }))
    }
}