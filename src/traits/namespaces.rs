use twilight_model::application::interaction::Interaction;
use worker::RouteContext;

use crate::{error::InteractionError, structs::kv::NamespacedKv};

pub static KV_BINDING: &'static str = "ggsbotkv";

pub trait InteractionKvExt {
    #[allow(unused)]
    fn user_kv(&self, ctx: &RouteContext<()>) -> Result<NamespacedKv, InteractionError>;
    #[allow(unused)]
    fn guild_kv(&self, ctx: &RouteContext<()>) -> Result<NamespacedKv, InteractionError>;
}

impl InteractionKvExt for Interaction {
    fn user_kv(&self, ctx: &RouteContext<()>) -> Result<NamespacedKv, InteractionError> {
        let user_id = self.author_id().ok_or(InteractionError::GenericError())?;
        
        let store = ctx.env.kv(KV_BINDING)
            .map_err(|e| InteractionError::WorkerError(e))?;
            
        Ok(NamespacedKv::new(store, format!("users:{}", user_id)))
    }

    fn guild_kv(&self, ctx: &RouteContext<()>) -> Result<NamespacedKv, InteractionError> {
        let Some(guild_id) = self.guild_id else {
            return Err(InteractionError::GenericError())
        };

        let store = ctx.env.kv(KV_BINDING)
            .map_err(|e| InteractionError::WorkerError(e))?;

        Ok(NamespacedKv::new(store, format!("guilds:{}", guild_id)))
    }
}