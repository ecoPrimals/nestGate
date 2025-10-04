// **STORAGE CACHING CONFIGURATION**

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageCachingConfig {
    pub enabled: bool,
    pub cache: CacheConfig,
    pub eviction: CacheEvictionPolicy,
    pub consistency: CacheConsistencyConfig,
    pub performance: CachePerformanceConfig,
    pub monitoring: CacheMonitoringConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub size: u64,
    pub ttl: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheEvictionPolicy {
    Lru,
    Lfu,
    Fifo,
    Random,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConsistencyConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachePerformanceConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMonitoringConfig {
    pub enabled: bool,
}

impl Default for StorageCachingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cache: CacheConfig {
                size: 1024 * 1024 * 1024,
                ttl: Duration::from_secs(3600),
            },
            eviction: CacheEvictionPolicy::Lru,
            consistency: CacheConsistencyConfig { enabled: true },
            performance: CachePerformanceConfig { enabled: true },
            monitoring: CacheMonitoringConfig { enabled: false },
        }
    }
}

impl StorageCachingConfig {
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
