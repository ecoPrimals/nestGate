// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

// Default configuration values and helpers

use super::*;

/// Create a default configuration with production-ready values
#[must_use]
pub fn production_defaults() -> NestGateCanonicalConfig {
    let mut config = NestGateCanonicalConfig::default();
    config.system.environment = "production".to_string();
    config.system.debug_mode = false;
    config
}
/// Create a default configuration with development values
#[must_use]
pub fn development_defaults() -> NestGateCanonicalConfig {
    let mut config = NestGateCanonicalConfig::default();
    config.system.environment = "development".to_string();
    config.system.debug_mode = true;
    config.development.enabled = true;
    config
}
