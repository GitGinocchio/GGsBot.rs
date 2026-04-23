use worker::{Env, ScheduleContext, ScheduledEvent};

use crate::TRIGGERS;




pub struct Scheduler {
    env: Env,
    ctx: ScheduleContext
}

impl Scheduler {
    pub fn new(env: Env, ctx: ScheduleContext) -> Self {
        Self { env, ctx }
    }

    pub async fn schedule(&self, event: ScheduledEvent) {
        let cron_id = event.cron();
        worker::console_log!("[ScheduledJob]: Batch started for cron expression '{}'", cron_id);

        for (name, trigger) in TRIGGERS.iter() {
            match trigger.can_run(&event, &self.env, &self.ctx).await {
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