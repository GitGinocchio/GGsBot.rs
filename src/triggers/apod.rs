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
        "0 * * * *".into()
    }

    async fn execute(&self, event: &ScheduledEvent, env: &Env, ctx: &ScheduleContext) -> Result<(), Error> {
        worker::console_debug!("Trigger named '{}' called with event: {:?}", self.name(), event);

        let api_key = env.var("NASA_API_KEY")
            .map_err(|e| Error::EnvironmentVariableNotFound(e.to_string()))?
            .to_string();

        let data = ApodService::fetch_data(&api_key).await?;
        let embed = ApodService::build_embed(data);

        worker::console_debug!("Apod embed: {embed:?}");

        Ok(())
    }
}