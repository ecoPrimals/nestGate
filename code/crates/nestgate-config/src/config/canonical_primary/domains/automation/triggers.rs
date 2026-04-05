// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// **TRIGGERS CONFIGURATION**
///
/// Event trigger settings for automation.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Triggers
pub struct TriggersConfig {
    /// Enable triggers
    pub enabled: bool,

    /// File system triggers enabled
    pub filesystem_triggers: bool,

    /// Performance triggers enabled
    pub performance_triggers: bool,

    /// Time-based triggers enabled
    pub time_triggers: bool,

    /// Custom trigger definitions
    pub custom_triggers: HashMap<String, serde_json::Value>,

    /// Trigger evaluation interval
    pub evaluation_interval: Duration,
}

impl Default for TriggersConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self::development()
    }
}

impl TriggersConfig {
    /// Creates a development-optimized configuration for event triggers
    ///
    /// Returns a `TriggersConfig` with triggers disabled and slower evaluation intervals
    /// suitable for development environments.
    #[must_use]
    pub fn development() -> Self {
        Self {
            enabled: false,
            filesystem_triggers: false,
            performance_triggers: false,
            time_triggers: false,
            custom_triggers: HashMap::new(),
            evaluation_interval: Duration::from_secs(60),
        }
    }

    /// Creates a production-hardened configuration for event triggers
    ///
    /// Returns a `TriggersConfig` with all trigger types enabled and fast evaluation
    /// intervals for production workloads.
    #[must_use]
    pub fn production() -> Self {
        Self {
            enabled: true,
            filesystem_triggers: true,
            performance_triggers: true,
            time_triggers: true,
            custom_triggers: HashMap::new(),
            evaluation_interval: Duration::from_secs(10),
        }
    }
}
