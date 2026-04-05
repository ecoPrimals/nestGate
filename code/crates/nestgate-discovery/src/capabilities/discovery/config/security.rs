// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use serde::{Deserialize, Serialize};

/// Security discovery settings for capability detection
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Securitydiscoverysettings
pub struct SecurityDiscoverySettings {
    /// Whether security discovery is enabled
    pub enabled: bool,
}

impl SecurityDiscoverySettings {
    /// Validates the security discovery settings
    ///
    /// # Errors
    ///
    /// Returns an error if the settings are invalid
    pub fn validate(&self) -> nestgate_types::error::Result<()> {
        Ok(())
    }
}
