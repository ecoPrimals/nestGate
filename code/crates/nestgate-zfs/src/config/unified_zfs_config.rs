use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Unified ZFS configuration using canonical patterns
/// **MIGRATED**: Now imports from the canonical ZFS configuration
use std::time::Duration;

pub use crate::canonical_zfs_config::{ZfsConfig, ZfsExtensions};

/// Simplified ZFS extensions for canonical config (internal definition)
#[derive(Serialize, Deserialize)]
pub struct InternalZfsExtensions {
    /// ZFS pool settings
    pub pools: ZfsPoolSettings,
    /// Dataset settings
    pub datasets: ZfsDatasetSettings,
    /// Snapshot settings
    pub snapshots: ZfsSnapshotSettings,
}
/// ZFS pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsPoolSettings {
    pub auto_discovery: bool,
    pub health_check_interval: Duration,
    pub default_properties: HashMap<String, String>,
}
impl Default for ZfsPoolSettings {
    fn default() -> Self {
        Self {
            auto_discovery: true,
            health_check_interval: Duration::from_secs(300), // 5 minutes
            default_properties: HashMap::new(),
        }
    }
}

/// ZFS dataset configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsDatasetSettings {
    pub compression: String,
    pub deduplication: bool,
    pub quota_gb: Option<u64>,
}
impl Default for ZfsDatasetSettings {
    fn default() -> Self {
        Self {
            compression: "lz4".to_string(),
            deduplication: false,
            quota_gb: None,
        }
    }
}

/// ZFS snapshot configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsSnapshotSettings {
    pub frequency: Duration,
    pub retention_days: u32,
    pub auto_cleanup: bool,
}
impl Default for ZfsSnapshotSettings {
    fn default() -> Self {
        Self {
            frequency: Duration::from_secs(24 * 3600), // daily
            retention_days: 30,
            auto_cleanup: true,
        }
    }
}

/// ZFS configuration factory methods
pub mod zfs_config_factory {
    use super::ZfsConfig;
    /// Create base ZFS configuration
    #[must_use]
    pub fn zfs_default() -> ZfsConfig {
        // Note: instance_name field not available in current SystemConfig
        ZfsConfig::default()
    }

    /// Create development ZFS configuration
    #[must_use]
    pub fn zfs_development() -> ZfsConfig {
        // Note: deployment_environment field not available in current EnvironmentConfig
        zfs_default()
    }

    /// Create production ZFS configuration
    #[must_use]
    pub fn zfs_production() -> ZfsConfig {
        // Note: deployment_environment field not available in current EnvironmentConfig
        zfs_default()
    }

    /// Alias for `zfs_development` for compatibility
    #[must_use]
    pub fn development() -> ZfsConfig {
        zfs_development()
    }

    /// Alias for `zfs_production` for compatibility
    #[must_use]
    pub fn production() -> ZfsConfig {
        zfs_production()
    }
}
