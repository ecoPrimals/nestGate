// **ZFS STORAGE CONFIGURATION**

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsStorageConfig {
    pub enabled: bool,
    pub pools: Vec<ZfsPoolConfig>,
    pub datasets: ZfsDatasetConfig,
    pub snapshots: ZfsSnapshotConfig,
    pub maintenance: ZfsMaintenanceConfig,
    pub performance: ZfsPerformanceConfig,
    pub security: ZfsSecurityConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsPoolConfig {
    pub name: String,
    pub devices: Vec<String>,
    pub redundancy: ZfsRedundancy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsDatasetConfig {
    pub auto_create: bool,
    pub compression: ZfsCompression,
    pub deduplication: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsSnapshotConfig {
    pub enabled: bool,
    pub interval: Duration,
    pub retention: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsMaintenanceConfig {
    pub scrub_interval: Duration,
    pub auto_scrub: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsPerformanceConfig {
    pub arc_size: Option<u64>,
    pub prefetch: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsSecurityConfig {
    pub encryption: bool,
    pub key_location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZfsCompression {
    Off,
    Lzjb,
    Gzip,
    Zle,
    Lz4,
    Zstd,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZfsRedundancy {
    None,
    Mirror,
    RaidZ1,
    RaidZ2,
    RaidZ3,
}

impl Default for ZfsDatasetConfig {
    fn default() -> Self {
        Self {
            auto_create: true,
            compression: ZfsCompression::Lz4,
            deduplication: false,
        }
    }
}

impl Default for ZfsSnapshotConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(3600), // 1 hour
            retention: 24,                       // 24 snapshots
        }
    }
}

impl Default for ZfsMaintenanceConfig {
    fn default() -> Self {
        Self {
            scrub_interval: Duration::from_secs(7 * 24 * 3600), // 1 week
            auto_scrub: true,
        }
    }
}

impl Default for ZfsPerformanceConfig {
    fn default() -> Self {
        Self {
            arc_size: None,
            prefetch: true,
        }
    }
}

impl ZfsStorageConfig {
    #[must_use]
    pub const fn production_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn development_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn high_performance() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn cloud_native() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn merge(self, _other: Self) -> Self {
        self
    }
    pub const fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}
