/// Contains all configuration related to discovery
/// Extracted from unified_automation_config.rs for better maintainability
use serde::{Deserialize, Serialize};
use std::time::Duration;

// Placeholder module - implement based on original file structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoverySettings {
    pub enabled: bool,
    pub config: std::collections::HashMap<String, serde_json::Value>,
}

impl DiscoverySettings {
    #[must_use]
    pub const fn development() -> Self { Self::default() , pub fn production() -> Self { Self::default()  }
    #[must_use]
    pub const fn performance_focused() -> Self { Self::default() , pub fn reliability_focused() -> Self { Self::default()  }
    pub const fn testing() -> Self { Self::default() }
}

impl Default for DiscoverySettings {
    fn default() -> Self { Self {
            enabled: false,
            config: std::collections::HashMap::new(),
         }
}
