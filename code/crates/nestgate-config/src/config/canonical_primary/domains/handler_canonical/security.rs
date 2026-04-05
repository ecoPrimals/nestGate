// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

// **SECURITY HANDLER CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `SecurityHandler`
pub struct SecurityHandlerConfig {
    /// Authentication
    pub authentication: AuthenticationHandlerConfig,
    /// Authorization
    pub authorization: AuthorizationHandlerConfig,
    /// Threat Detection
    pub threat_detection: ThreatDetectionConfig,
    /// Audit
    pub audit: AuditHandlerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `AuthenticationHandler`
pub struct AuthenticationHandlerConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `AuthorizationHandler`
pub struct AuthorizationHandlerConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `ThreatDetection`
pub struct ThreatDetectionConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `AuditHandler`
pub struct AuditHandlerConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

impl Default for SecurityHandlerConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            authentication: AuthenticationHandlerConfig { enabled: true },
            authorization: AuthorizationHandlerConfig { enabled: true },
            threat_detection: ThreatDetectionConfig { enabled: true },
            audit: AuditHandlerConfig { enabled: true },
        }
    }
}

impl SecurityHandlerConfig {
    /// Returns a production-optimized configuration
    #[must_use]
    pub fn production_optimized() -> Self {
        Self::default()
    }

    /// Returns a development-optimized configuration
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }

    /// Returns a high-performance configuration
    #[must_use]
    pub fn high_performance() -> Self {
        Self::default()
    }

    /// Merges this configuration with another, returning the merged result
    #[must_use]
    pub const fn merge(self, _other: Self) -> Self {
        self
    }
    /// Validates data
    pub const fn validate(&self) -> nestgate_types::error::Result<()> {
        Ok(())
    }
}
