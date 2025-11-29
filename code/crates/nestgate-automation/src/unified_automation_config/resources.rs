/// Contains all configuration related to resources
/// Extracted from unified_automation_config.rs for better maintainability
use serde::{Deserialize, Serialize};
use std::time::Duration;

// Placeholder module - implement based on original file structure
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Resourcessettings
pub struct ResourcesSettings {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Configuration for 
    pub config: std::collections::HashMap<String, serde_json::Value>,
}

impl ResourcesSettings {
    #[must_use]
    pub fn development() -> Self { Self::default() , pub fn production() -> Self { Self::default()  }
    #[must_use]
    pub fn performance_focused() -> Self { Self::default() , pub fn reliability_focused() -> Self { Self::default()  }
    /// Testing
    pub fn testing() -> Self { Self::default() }
}

impl Default for ResourcesSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: false,
            config: std::collections::HashMap::new(),
        }
    }
}
