// SPDX-License-Identifier: AGPL-3.0-only
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
/// ⚠️ **EVOLUTION NOTE**: This config is being evolved from hardcoded endpoints
/// to pure runtime discovery. The endpoint fields are deprecated and will be
/// removed in v0.14.0 (Q2 2026).
///
/// **Modern Pattern**: Use RuntimeDiscovery instead:
/// ```rust,ignore
/// use nestgate_core::primal_discovery::RuntimeDiscovery;
/// let discovery = RuntimeDiscovery::new().await?;
/// let security = discovery.find_security_primal().await?;
/// ```
pub struct PrimalsConfig {
    /// Intelligence AI Endpoint (DEPRECATED - use RuntimeDiscovery)
    ///
    /// This field will be removed in v0.14.0. Use runtime discovery instead.
    #[deprecated(since = "0.12.0", note = "Use RuntimeDiscovery::find_ai_primal() instead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intelligence_ai_endpoint: Option<String>,
    
    /// Security Endpoint (DEPRECATED - use RuntimeDiscovery)
    ///
    /// This field will be removed in v0.14.0. Use runtime discovery instead.
    #[deprecated(since = "0.12.0", note = "Use RuntimeDiscovery::find_security_primal() instead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_security_endpoint: Option<String>,
    
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
    /// **EVOLUTION**: No more hardcoded endpoints! Use RuntimeDiscovery for
    /// pure capability-based discovery at runtime.
    ///
    /// **Backward Compatibility**: Still checks env vars if set, but no hardcoded fallbacks.
    fn default() -> Self {
        // Check for backward-compatible env vars but don't hardcode fallbacks
        let intelligence_endpoint = std::env::var("NESTGATE_AI_ENDPOINT").ok();
        let security_endpoint = std::env::var("NESTGATE_SECURITY_ENDPOINT").ok();
        
        if intelligence_endpoint.is_none() {
            tracing::debug!(
                "No NESTGATE_AI_ENDPOINT configured - will use runtime discovery"
            );
        }
        
        if security_endpoint.is_none() {
            tracing::debug!(
                "No NESTGATE_SECURITY_ENDPOINT configured - will use runtime discovery"
            );
        }
        
        Self {
            intelligence_ai_endpoint: intelligence_endpoint,
            security_security_endpoint: security_endpoint,
            ecosystem_integration_enabled: true,
        }
    }
}
