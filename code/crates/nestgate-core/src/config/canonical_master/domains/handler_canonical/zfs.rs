// **ZFS HANDLER CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsHandlerConfig {
    pub pool: PoolHandlerConfig,
    pub dataset: DatasetHandlerConfig,
    pub snapshot: SnapshotHandlerConfig,
    pub backup: BackupHandlerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolHandlerConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetHandlerConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotHandlerConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupHandlerConfig {
    pub enabled: bool,
}

impl Default for ZfsHandlerConfig {
    fn default() -> Self {
        Self {
            pool: PoolHandlerConfig { enabled: true },
            dataset: DatasetHandlerConfig { enabled: true },
            snapshot: SnapshotHandlerConfig { enabled: true },
            backup: BackupHandlerConfig { enabled: true },
        }
    }
}

impl ZfsHandlerConfig {
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
    pub const fn merge(self, _other: Self) -> Self {
        self
    }
    pub const fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}
