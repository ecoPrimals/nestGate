//
// ZFS-specific configuration structures extracted from the monolithic domain_configs.rs
// for better maintainability and focused responsibility.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// ZFS domain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsDomainConfig {
    pub pools: Vec<String>,
    pub compression: String,
    pub deduplication: bool,
    pub snapshots_enabled: bool,
    pub snapshot_interval: Duration,
    pub performance_tuning: ZfsPerformanceConfig,
}

/// ZFS performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsPerformanceConfig {
    pub recordsize: String,
    pub atime: bool,
    pub sync: String,
    pub cache_mode: String,
}

impl Default for ZfsDomainConfig {
    fn default() -> Self {
        Self {
            pools: vec!["rpool".to_string()],
            compression: "lz4".to_string(),
            deduplication: false,
            snapshots_enabled: true,
            snapshot_interval: Duration::from_secs(3600), // 1 hour
            performance_tuning: ZfsPerformanceConfig::default(),
        }
    }
}

impl Default for ZfsPerformanceConfig {
    fn default() -> Self {
        Self {
            recordsize: "128K".to_string(),
            atime: false,
            sync: "standard".to_string(),
            cache_mode: "all".to_string(),
        }
    }
}
