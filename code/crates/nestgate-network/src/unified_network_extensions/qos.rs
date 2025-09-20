//
// QoS and traffic management configuration.

use serde::{Deserialize, Serialize};

/// Network Quality of Service settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkQosSettings {
    /// Enable QoS
    pub enabled: bool,
    /// Default priority level
    pub default_priority: u8,
}
impl Default for NetworkQosSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            default_priority: 3,
        }
    }
}
