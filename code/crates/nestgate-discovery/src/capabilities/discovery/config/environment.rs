// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use serde::{Deserialize, Serialize};

/// Environment discovery settings for capability detection
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Environmentdiscoverysettings
pub struct EnvironmentDiscoverySettings {
    /// Whether environment discovery is enabled
    pub enabled: bool,
}

impl EnvironmentDiscoverySettings {
    /// Validates the environment discovery settings
    ///
    /// # Errors
    ///
    /// Returns an error if the settings are invalid
    pub fn validate(&self) -> nestgate_types::error::Result<()> {
        Ok(())
    }
}
