/// **SCHEDULING MODULE**
/// Task scheduling configuration - extracted from monolithic config
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Scheduling settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulingSettings {
    /// Enable scheduling
    pub enabled: bool,
    /// Scheduled tasks
    pub tasks: HashMap<String, ScheduledTask>,
    /// Default task timeout
    pub default_timeout: Duration,
    /// Max concurrent tasks
    pub max_concurrent: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledTask {
    /// Task name
    pub name: String,
    /// Task enabled
    pub enabled: bool,
    /// Cron schedule
    pub schedule: String,
    /// Task timeout
    pub timeout: Duration,
}

impl SmartDefault for SchedulingSettings {
    fn smart_default() -> Self {
        Self {
            enabled: true,
            tasks: HashMap::default(),
            default_timeout: Duration::from_secs(300),
            max_concurrent: 5,
        }
    }
}

impl Default for SchedulingSettings {
    fn default() -> Self {
        Self::smart_default()
    }
}
