// **CACHING PERFORMANCE CONFIGURATION**

use crate::Result;
use serde::{Deserialize, Serialize};

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
    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        pub const fn validate(&self) -> Result<()>  {
        Ok(())
    }
}
