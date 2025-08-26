//
// VLAN and network segmentation configuration.

use serde::{Deserialize, Serialize};

/// Network VLAN and segmentation settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkVlanSettings {
    /// Enable VLAN support
    pub enabled: bool,
    /// Default VLAN ID
    pub default_vlan_id: u16,
}

impl Default for NetworkVlanSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            default_vlan_id: 1,
        }
    }
}
