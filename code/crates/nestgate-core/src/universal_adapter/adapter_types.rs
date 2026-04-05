// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Core types for the universal adapter: configuration, capability metadata, and request/response DTOs.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Universal Adapter for O(1) capability-based connections
/// Replaces hardcoded primal-to-primal connections
#[derive(Debug, Clone)]
/// Universaladapter
pub struct UniversalAdapter {
    /// Adapter endpoint URL
    pub endpoint: String,
    /// Discovered capabilities from all primals
    pub capabilities: HashMap<String, CapabilityInfo>,
    /// Discovery cache
    pub discovery_cache: HashMap<String, CachedCapability>,
    /// Adapter configuration (cache/timeout settings)
    #[expect(deprecated, reason = "migration in progress")]
    /// Configuration for
    pub config: UniversalAdapterConfig,
    /// Discovery configuration (immutable, thread-safe)
    pub discovery_config: super::SharedDiscoveryConfig,
}

/// Configuration for the universal adapter
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::UniversalAdapterConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::UniversalAdapterConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for UniversalAdapter
pub struct UniversalAdapterConfig {
    /// Discovery timeout in seconds
    pub discovery_timeout: u64,
    /// Cache TTL in seconds
    pub cache_ttl: u64,
    /// Enable capability caching
    pub enable_caching: bool,
    /// Maximum concurrent discovery requests
    pub max_concurrent_discovery: usize,
}

/// Information about a capability provided by any primal
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Capabilityinfo
pub struct CapabilityInfo {
    /// Capability category (orchestration, compute, security, ai, storage, etc.)
    pub category: String,
    /// Primal provider (discovered dynamically, never hardcoded)
    pub provider: String,
    /// Capability endpoint
    pub endpoint: String,
    /// Performance tier (enterprise, `high_performance`, standard)
    pub performance_tier: String,
    /// Availability percentage
    pub availability: f64,
    /// Capability metadata
    pub metadata: HashMap<String, String>,
    /// Discovery timestamp
    pub discovered_at: SystemTime,
}

/// Cached capability information
#[derive(Debug, Clone)]
/// Cachedcapability
pub struct CachedCapability {
    /// Capability information
    pub info: CapabilityInfo,
    /// Cache timestamp
    pub cached_at: SystemTime,
    /// Cache expiry
    pub expires_at: SystemTime,
}

/// Universal adapter request for capability access
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for Capability operation
pub struct CapabilityRequest {
    /// Target capability category
    pub capability: String,
    /// Request method/operation
    pub method: String,
    /// Request parameters
    pub parameters: serde_json::Value,
    /// Request metadata
    pub metadata: HashMap<String, String>,
}

impl CapabilityRequest {
    /// Create new capability request
    #[must_use]
    pub fn new(capability: impl Into<String>, method: impl Into<String>) -> Self {
        Self {
            capability: capability.into(),
            method: method.into(),
            parameters: serde_json::Value::Null,
            metadata: HashMap::new(),
        }
    }

    /// Add parameters to the request
    #[must_use]
    pub fn with_parameters(mut self, parameters: serde_json::Value) -> Self {
        self.parameters = parameters;
        self
    }

    /// Add metadata to the request
    #[must_use]
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Create a search query for a specific capability (compatibility method)
    pub fn search(capability_type: impl Into<String>) -> Self {
        Self::new(capability_type, "search")
    }
}

/// Universal adapter response
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for Capability operation
pub struct CapabilityResponse {
    /// Response status
    pub status: String,
    /// Response data
    pub result: serde_json::Value,
    /// Response metadata
    pub metadata: HashMap<String, String>,
    /// Provider that handled the request
    pub provider: String,
    /// Request latency in milliseconds
    pub latency_ms: u64,
}

#[expect(deprecated, reason = "migration in progress")]
impl Default for UniversalAdapterConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            discovery_timeout: 30,
            cache_ttl: 300,
            enable_caching: true,
            max_concurrent_discovery: 10,
        }
    }
}

/// Primal sovereignty validation
/// Ensures no hardcoded primal-to-primal connections exist
///
/// # Errors
///
/// Returns `Err` with a message when sovereignty checks fail (currently always returns [`Ok`]).
pub const fn validate_primal_sovereignty() -> Result<(), String> {
    // This function would scan the codebase to ensure no hardcoded primal names
    // are used for direct connections - all must go through the universal adapter
    Ok(())
}

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
/// Type alias for Universaladapterconfigcanonical
pub type UniversalAdapterConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
