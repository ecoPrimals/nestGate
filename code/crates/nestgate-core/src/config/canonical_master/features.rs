// Features configuration structures

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturesConfig {
    pub enabled: bool,
    pub enable_auto_scaling: bool,
    pub enable_load_balancing: bool,
    pub enable_monitoring: bool,
    pub enable_metrics: bool,
    pub enable_tracing: bool,
}

impl Default for FeaturesConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            enable_auto_scaling: false,
            enable_load_balancing: false,
            enable_monitoring: true,
            enable_metrics: true,
            enable_tracing: true,
        }
    }
}
