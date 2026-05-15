use worker::{Request, Result, Response, RouteContext};

use crate::{constants::COMMANDS, framework::discord::command::SerializableCommand};

pub async fn get(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let commands: Vec<_> = COMMANDS
        .values()
        .map(|cmd| SerializableCommand(cmd.as_ref()))
        .collect();

    Response::from_json(&commands)
}