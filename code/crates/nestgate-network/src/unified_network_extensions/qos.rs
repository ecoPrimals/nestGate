// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// QoS and traffic management configuration.

use serde::{Deserialize, Serialize};

/// Network Quality of Service settings
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Networkqossettings
pub struct NetworkQosSettings {
    /// Enable `QoS`
    pub enabled: bool,
    /// Default priority level
    pub default_priority: u8,
}
impl Default for NetworkQosSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: false,
            default_priority: 3,
        }
    }
}
