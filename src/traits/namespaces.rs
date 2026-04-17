use twilight_model::application::interaction::Interaction;
use worker::RouteContext;

use crate::{error::InteractionError, structs::kv::NamespacedKv};

pub static KV_BINDING: &'static str = "ggsbotkv";

pub trait InteractionKvExt {
    fn user_kv(&self, ctx: &RouteContext<()>) -> Result<NamespacedKv, InteractionError>;
    fn guild_kv(&self, ctx: &RouteContext<()>) -> Result<NamespacedKv, InteractionError>;
}

impl InteractionKvExt for Interaction {
    fn user_kv(&self, ctx: &RouteContext<()>) -> Result<NamespacedKv, InteractionError> {
        let user_id = self.author_id().ok_or(InteractionError::GenericError())?;
        
        let store = ctx.env.kv(KV_BINDING)
            .map_err(|e| InteractionError::WorkerError(e))?;
            
        Ok(NamespacedKv::new(store, format!("user:{}", user_id)))
    }

    fn guild_kv(&self, ctx: &RouteContext<()>) -> Result<NamespacedKv, InteractionError> {
        let Some(guild_id) = self.guild_id else {
            return Err(InteractionError::GenericError())
        };

        let store = ctx.env.kv(KV_BINDING)
            .map_err(|e| InteractionError::WorkerError(e))?;

        Ok(NamespacedKv::new(store, format!("guild:{}", guild_id)))
    }
}