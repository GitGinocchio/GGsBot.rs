use async_trait::async_trait;
use chrono::Utc;
use serde_json::json;
use worker::{Env, ScheduleContext, ScheduledEvent};

use crate::{
    commands::nasa::NasaExtConfig, error::Error, framework::{structs::config::extension::ExtensionConfig, traits::{
        namespaces::KV_BINDING,
        trigger::{CronSchedule, Trigger},
    }}, services::apod::ApodService
};

#[derive(Default)]
pub struct ApodTrigger;

#[async_trait(?Send)]
impl Trigger for ApodTrigger {
    fn name(&self) -> &str {
        "nasa-apod-trigger"
    }

    fn cron(&self) -> CronSchedule {
        "0 7 * * *".into()
    }

    async fn should_run(
        &self,
        _event: &ScheduledEvent,
        _env: &Env,
        _ctx: &ScheduleContext,
    ) -> Result<bool, Error> {
        Ok(true)
    }

    async fn execute(
        &self,
        _event: &ScheduledEvent,
        env: &Env,
        _ctx: &ScheduleContext,
    ) -> Result<(), Error> {
        let kv = env.kv(KV_BINDING)?;
        let queue = env.queue("APOD_QUEUE")?;
        let mut cursor: Option<String> = None;

        loop {
            let list = kv.list()
                .prefix("guilds:".to_string())
                .cursor(cursor.clone().unwrap_or_default())
                .execute()
                .await?;

            for key in list.keys {
                if !key.name.ends_with(":nasa:config") { continue; }

                let Some(guild_id) = key.name.split(":").nth(1) else {
                    continue;
                };

                let ext = match kv.get(&key.name).json::<ExtensionConfig<NasaExtConfig>>().await? {
                    Some(data) => data,
                    None => continue
                };

                if !ext.enabled { continue };
                let Some(config) = ext.config else {
                    continue;
                };

                let Some(channel_id) = config.channel_id else {
                    continue;
                };

                let message = json!({
                    "guild_id" : guild_id,
                    "channel_id": channel_id
                });

                queue.send(message).await?;
            }

            if list.list_complete {
                break;
            }
            cursor = list.cursor;
        }

        Ok(())
    }
}
