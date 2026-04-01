// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Load balancing and routing configuration.

use serde::{Deserialize, Serialize};

/// Network routing and load balancing settings
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Networkroutingsettings
pub struct NetworkRoutingSettings {
    /// Enable load balancing
    pub load_balancing_enabled: bool,
    /// Load balancing algorithm
    pub load_balancing_algorithm: String,
}
impl Default for NetworkRoutingSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            load_balancing_enabled: true,
            load_balancing_algorithm: "round_robin".to_string(),
        }
    }
}
