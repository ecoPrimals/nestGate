//! **NETWORK PERFORMANCE CONFIGURATION**

use serde::{Deserialize, Serialize};
use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkPerformanceConfig {
    pub optimization: NetworkOptimizationConfig,
    pub buffering: NetworkBufferingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkOptimizationConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkBufferingConfig {
    pub buffer_size: usize,
}

impl NetworkPerformanceConfig {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
} 