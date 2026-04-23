use std::collections::HashMap;

use async_trait::async_trait;
use worker::{Env, ScheduleContext, ScheduledEvent};

use crate::error::Error;

pub type TriggerMap = HashMap<String, Box<dyn Trigger + Send + Sync>>;

#[macro_export]
macro_rules! build_triggers {
    ($($command_type:ty),*) => {
        {
            #[allow(unused_mut)]
            let mut map: $crate::traits::trigger::TriggerMap = std::collections::HashMap::new();
            $(
                let handler: Box<dyn $crate::traits::trigger::Trigger + Send + Sync> = 
                    Box::new(<$command_type>::default()); 
                
                map.insert(handler.name().into(), handler);
            )*
            map
        }
    };
}

pub enum CronSchedule {
    All,
    Single(&'static str),
    Multiple(&'static [&'static str]),
}

impl From<&'static str> for CronSchedule {
    fn from(s: &'static str) -> Self {
        CronSchedule::Single(s)
    }
}

impl From<&'static [&'static str]> for CronSchedule {
    fn from(s: &'static [&'static str]) -> Self {
        CronSchedule::Multiple(s)
    }
}

impl Default for CronSchedule {
    fn default() -> Self {
        CronSchedule::All
    }
}

#[async_trait(?Send)]
#[allow(unused)]
pub trait Trigger {
    fn name(&self) -> &str;

    fn cron(&self) -> CronSchedule { CronSchedule::All }

    async fn should_run(&self, _event: &ScheduledEvent, _env: &Env, _ctx: &ScheduleContext) -> Result<bool, Error> {
        Ok(true)
    }

    async fn execute(&self, event: &ScheduledEvent, env: &Env, ctx: &ScheduleContext) -> Result<(), Error>;
}