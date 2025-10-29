// **STORAGE PERFORMANCE CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoragePerformanceConfig {
    pub optimization: PerformanceOptimizationConfig,
    pub io_optimization: IOOptimizationConfig,
    pub compression: CompressionConfig,
    pub deduplication: DeduplicationConfig,
    pub tuning: TuningConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceOptimizationConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IOOptimizationConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionConfig {
    pub enabled: bool,
    pub algorithm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeduplicationConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuningConfig {
    pub auto_tune: bool,
}

impl Default for StoragePerformanceConfig {
    fn default() -> Self {
        Self {
            optimization: PerformanceOptimizationConfig { enabled: true },
            io_optimization: IOOptimizationConfig { enabled: true },
            compression: CompressionConfig {
                enabled: true,
                algorithm: "lz4".to_string(),
            },
            deduplication: DeduplicationConfig { enabled: false },
            tuning: TuningConfig { auto_tune: true },
        }
    }
}

impl StoragePerformanceConfig {
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
