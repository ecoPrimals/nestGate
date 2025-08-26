//
// Configuration for ZFS pool discovery, validation, and management.

use serde::{Deserialize, Serialize};

/// Pool discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    fn default() -> Self {
        Self {
            auto_discovery: true,
            default_pool: "nestpool".to_string(),
            include_pools: vec![],
            exclude_pools: vec!["rpool".to_string()], // Exclude system pool by default
            discovery_interval_seconds: 300,
            validate_health: true,
        }
    }
}

impl PoolDiscoveryConfig {
    /// Create production-optimized pool discovery configuration
    pub fn production() -> Self {
        Self {
            auto_discovery: true,
            default_pool: "nestpool".to_string(),
            include_pools: vec![],
            exclude_pools: vec!["rpool".to_string()], // Exclude system pool by default
            discovery_interval_seconds: 30,
            validate_health: true,
        }
    }
}
