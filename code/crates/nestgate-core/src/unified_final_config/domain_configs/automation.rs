//
// Automation-specific configuration structures extracted from the monolithic NestGateCanonicalConfig.rs
// for better maintainability and focused responsibility.

use serde::{Deserialize, Serialize};

/// Automation domain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationDomainConfig {
    pub enabled: bool,
    pub scheduling_enabled: bool,
    pub ml_predictions: bool,
    pub auto_scaling: bool,
    pub maintenance_windows: Vec<String>,
}
impl Default for AutomationDomainConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            scheduling_enabled: true,
            ml_predictions: false,
            auto_scaling: false,
            maintenance_windows: vec!["02:00-04:00".to_string(), "14:00-16:00".to_string()],
        }
    }
}
