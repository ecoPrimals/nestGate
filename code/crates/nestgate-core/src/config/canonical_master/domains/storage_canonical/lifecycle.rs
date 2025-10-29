// **STORAGE LIFECYCLE CONFIGURATION**

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageLifecycleConfig {
    pub data_lifecycle: DataLifecycleConfig,
    pub retention: RetentionConfig,
    pub archival: ArchivalConfig,
    pub purging: PurgingConfig,
    pub compliance: ComplianceStorageConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataLifecycleConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionConfig {
    pub enabled: bool,
    pub duration: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchivalConfig {
    pub enabled: bool,
    pub after: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurgingConfig {
    pub enabled: bool,
    pub after: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceStorageConfig {
    pub enabled: bool,
}

impl Default for StorageLifecycleConfig {
    fn default() -> Self {
        Self {
            data_lifecycle: DataLifecycleConfig { enabled: true },
            retention: RetentionConfig {
                enabled: true,
                duration: Duration::from_secs(365 * 24 * 3600),
            },
            archival: ArchivalConfig {
                enabled: false,
                after: Duration::from_secs(90 * 24 * 3600),
            },
            purging: PurgingConfig {
                enabled: false,
                after: Duration::from_secs(7 * 365 * 24 * 3600),
            },
            compliance: ComplianceStorageConfig { enabled: false },
        }
    }
}

impl StorageLifecycleConfig {
    #[must_use]
    pub fn production_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn high_performance() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn cloud_native() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn merge(self, _other: Self) -> Self {
        self
    }
    pub fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}
