use worker::{Env, ScheduleContext, ScheduledEvent};

use crate::{TRIGGERS, error::Error, traits::trigger::{CronSchedule, Trigger}};




pub struct Scheduler {
    env: Env,
    ctx: ScheduleContext
}

pub async fn can_run_trigger(
    trigger: &Box<dyn Trigger + Send + Sync>, 
    event: &ScheduledEvent, 
    env: &Env, 
    ctx: &ScheduleContext
) -> Result<bool, Error> {
    let current_cron = event.cron();
    let cron_ok = match trigger.cron() {
        CronSchedule::All => true,
        CronSchedule::Single(pattern) => current_cron == pattern,
        CronSchedule::Multiple(patterns) => patterns.contains(&current_cron.as_str()),
    };

    if !cron_ok { return Ok(false); }

    trigger.should_run(event, env, ctx).await
}

impl Scheduler {
    pub fn new(env: Env, ctx: ScheduleContext) -> Self {
        Self { env, ctx }
    }

    pub async fn schedule(&self, event: ScheduledEvent) {
        let cron_id = event.cron();
        worker::console_log!("[ScheduledJob]: Batch started for cron expression '{}'", cron_id);

        for (name, trigger) in TRIGGERS.iter() {
            match can_run_trigger(&trigger, &event, &self.env, &self.ctx).await {
                Ok(false) => {
                    worker::console_debug!("[ScheduledJob]: Skipping trigger '{}' (condition not met)", name);
                    continue;
                },
                Err(e) => {
                    worker::console_error!("[ScheduledJob]: Error checking condition for trigger '{}': {:?}", name, e);
                    continue;
                },
                _ => {} // Ok(true)
            }

            worker::console_log!("[ScheduledJob]: Executing trigger '{}'...", name);
            
            if let Err(e) = trigger.execute(&event, &self.env, &self.ctx).await {
                worker::console_error!("[ScheduledJob]: Trigger '{}' FAILED with error: {:?}", name, e);
                continue;
            }

            worker::console_log!("[ScheduledJob]: Trigger '{}' finished successfully.", name);
        }

        worker::console_log!("[ScheduledJob]: Batch completed successfully for '{}'", cron_id);
    }
    
}