// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Configuration for ZFS pool discovery, validation, and management.

use serde::{Deserialize, Serialize};

/// Pool discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::PoolDiscoveryConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::PoolDiscoveryConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for `PoolDiscovery`
pub struct PoolDiscoveryConfig {
    /// Enable automatic pool discovery
    pub auto_discovery: bool,
    /// Default pool name to use when creating pools
    pub default_pool: String,
    /// Pools to explicitly include
    pub include_pools: Vec<String>,
    /// Pools to explicitly exclude
    pub exclude_pools: Vec<String>,
    /// Discovery interval in seconds
    pub discovery_interval_seconds: u64,
    /// Validate pool health on discovery
    pub validate_health: bool,
}
impl Default for PoolDiscoveryConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            auto_discovery: true,
            default_pool: "zfspool".to_string(),
            include_pools: vec![],
            exclude_pools: vec!["rpool".to_string()], // Exclude system pool by default
            discovery_interval_seconds: 300,
            validate_health: true,
        }
    }
}

impl PoolDiscoveryConfig {
    /// Create production-optimized pool discovery configuration
    #[must_use]
    pub fn production() -> Self {
        Self {
            auto_discovery: true,
            default_pool: "zfspool".to_string(),
            include_pools: vec![],
            exclude_pools: vec!["rpool".to_string()], // Exclude system pool by default
            discovery_interval_seconds: 30,
            validate_health: true,
        }
    }
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Pooldiscoveryconfigcanonical
pub type PoolDiscoveryConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using PoolDiscoveryConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(deprecated)]
    #[test]
    fn pool_discovery_config_default_and_production() {
        let d = PoolDiscoveryConfig::default();
        assert!(d.auto_discovery);
        assert_eq!(d.default_pool, "zfspool");
        let p = PoolDiscoveryConfig::production();
        assert_eq!(p.discovery_interval_seconds, 30);
        let json = serde_json::to_string(&d).expect("serialize");
        let back: PoolDiscoveryConfig = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.exclude_pools, d.exclude_pools);
    }
}
