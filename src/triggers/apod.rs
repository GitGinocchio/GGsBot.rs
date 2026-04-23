use async_trait::async_trait;
use worker::{Env, ScheduleContext, ScheduledEvent};

use crate::{error::Error, services::apod::ApodService, traits::trigger::{CronSchedule, Trigger}};

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

    async fn should_run(&self, event: &ScheduledEvent, env: &Env, ctx: &ScheduleContext) -> Result<bool, Error> {
        Ok(true)
    }

    async fn execute(&self, event: &ScheduledEvent, env: &Env, ctx: &ScheduleContext) -> Result<(), Error> {
        let api_key = env.var("NASA_API_KEY")
            .map_err(|e| Error::EnvironmentVariableNotFound(e.to_string()))?
            .to_string();

        let apod_data = ApodService::fetch_apod_with_retries(&api_key, 3).await?;

        worker::console_debug!("{apod_data:?}");

        Ok(())
    }
}