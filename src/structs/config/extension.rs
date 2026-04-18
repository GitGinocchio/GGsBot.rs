use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtensionConfig<T> {
    pub enabled: bool,

    #[serde(skip_serializing_if="Option::is_none")]
    pub config: Option<T>,

    pub created_at: DateTime<Utc>
}

impl<T> ExtensionConfig<T> {
    pub fn new(config: Option<T>) -> Self {
        Self {
            config: config,
            ..Default::default()
        }
    }

    #[allow(unused)]
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = value
    }
}

impl<T> Default for ExtensionConfig<T> {
    fn default() -> Self {
        Self {
            enabled: true,
            config: None,
            created_at: Utc::now()
        }
    }
}