// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// VLAN and network segmentation configuration.

use serde::{Deserialize, Serialize};

/// Network VLAN and segmentation settings
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Networkvlansettings
pub struct NetworkVlanSettings {
    /// Enable VLAN support
    pub enabled: bool,
    /// Default VLAN ID
    pub default_vlan_id: u16,
}
impl Default for NetworkVlanSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: false,
            default_vlan_id: 1,
        }
    }
}
