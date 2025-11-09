// **PERFORMANCE ENVIRONMENT CONFIGURATION**

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceEnvironmentConfig {
    pub environment: String,
    pub overrides: HashMap<String, serde_json::Value>,
    pub feature_flags: HashMap<String, bool>,
    pub debug: Option<PerformanceDebugConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceDebugConfig {
    pub enabled: bool,
    pub log_level: String,
}
