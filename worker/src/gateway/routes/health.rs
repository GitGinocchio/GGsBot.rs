use serde_json::json;
use worker::{Request, Response, Result, RouteContext};

use crate::gateway::gateway::Gateway;




impl Gateway {
    pub async fn health(&self, _req: Request, _ctx: RouteContext<()>) -> Result<Response> {
        let shard_id = self.get_shard_id()?;

        self.get_websocket()?;

        Response::from_json(&json!({
            "shard_id": shard_id,
            "status" : "CONNECTED"
        }))
    }
}