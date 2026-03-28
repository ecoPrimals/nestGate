// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// Simplified, unified middleware configuration using canonical patterns

use nestgate_core::config::canonical_primary::NestGateCanonicalConfig;

/// Middleware configuration type alias
pub type MiddlewareConfig = NestGateCanonicalConfig;
/// Create default middleware configuration
#[must_use]
pub fn create_default_config() -> MiddlewareConfig {
    NestGateCanonicalConfig::default()
}
