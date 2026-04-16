// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

// Integration configuration structures

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Integration
pub struct IntegrationConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for UniversalAdapter
pub struct UniversalAdapterConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Primals
///
/// **Modern Pattern**: Use RuntimeDiscovery instead of static endpoints:
/// ```rust,ignore
/// use nestgate_core::primal_discovery::RuntimeDiscovery;
/// let discovery = RuntimeDiscovery::new().await?;
/// let security = discovery.find_security_primal().await?;
/// ```
pub struct PrimalsConfig {
    /// Ecosystem Integration Enabled
    pub ecosystem_integration_enabled: bool,
}

impl Default for IntegrationConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl Default for UniversalAdapterConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl Default for PrimalsConfig {
    /// Returns the default instance
    ///
    /// Use [`RuntimeDiscovery`](nestgate_core::primal_discovery::RuntimeDiscovery) for capability-based discovery at runtime.
    fn default() -> Self {
        if std::env::var("NESTGATE_AI_ENDPOINT").is_err() {
            tracing::debug!("No NESTGATE_AI_ENDPOINT configured - will use runtime discovery");
        }
        if std::env::var("NESTGATE_SECURITY_ENDPOINT").is_err() {
            tracing::debug!("No NESTGATE_SECURITY_ENDPOINT configured - will use runtime discovery");
        }
        Self {
            ecosystem_integration_enabled: true,
        }
    }
}
