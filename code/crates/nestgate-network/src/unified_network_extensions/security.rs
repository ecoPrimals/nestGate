//
// Network security and firewall configuration.

use serde::{Deserialize, Serialize};

/// Network security settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSecuritySettings {
    /// Enable firewall
    pub firewall_enabled: bool,
    /// Enable encryption
    pub encryption_enabled: bool,
}
impl Default for NetworkSecuritySettings {
    fn default() -> Self {
        Self {
            firewall_enabled: true,
            encryption_enabled: true,
        }
    }
}
