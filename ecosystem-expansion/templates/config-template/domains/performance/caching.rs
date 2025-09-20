//! **CACHING PERFORMANCE CONFIGURATION**

use serde::{Deserialize, Serialize};
use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CachePerformanceConfig {
    pub optimization: CacheOptimizationConfig,
    pub warming: CacheWarmingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheOptimizationConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheWarmingConfig {
    pub enabled: bool,
    pub strategy: WarmingStrategy,
    pub batch_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum WarmingStrategy {
    #[default]
    Eager,
    Lazy,
    Predictive,
}

impl CachePerformanceConfig {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
} 