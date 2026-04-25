use twilight_model::application::interaction::Interaction;
use worker::Env;

use crate::{error::Error, framework::structs::kv::NamespacedKv};

pub static KV_BINDING: &'static str = "ggsbotkv";

pub trait KvExt {
    #[allow(unused)]
    fn user_kv(&self, env: &Env) -> Result<NamespacedKv, Error>;
    #[allow(unused)]
    fn guild_kv(&self, env: &Env) -> Result<NamespacedKv, Error>;
}

impl KvExt for Interaction {
    fn user_kv(&self, env: &Env) -> Result<NamespacedKv, Error> {
        let user_id = self
            .author_id()
            .ok_or(Error::Generic("Missing author id".into()))?;

        let store = env
            .kv(KV_BINDING)
            .map_err(|e| Error::EnvironmentVariableNotFound(e.to_string()))?;

        Ok(NamespacedKv::new(store, format!("users:{}", user_id)))
    }

    fn guild_kv(&self, env: &Env) -> Result<NamespacedKv, Error> {
        let Some(guild_id) = self.guild_id else {
            return Err(Error::InteractionFailed("Missing guild_id".into()));
        };

        let store = env
            .kv(KV_BINDING)
            .map_err(|e| Error::EnvironmentVariableNotFound(e.to_string()))?;

        Ok(NamespacedKv::new(store, format!("guilds:{}", guild_id)))
    }
}
