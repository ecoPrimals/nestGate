//! **MEMORY PERFORMANCE CONFIGURATION**
//!
//! Memory optimization and garbage collection configuration.

use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::{NestGateError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct MemoryPerformanceConfig {
    /// Memory pool configuration
    pub pool: MemoryPoolConfig,
    
    /// Garbage collection settings
    pub gc: GarbageCollectionConfig,
    
    /// Memory monitoring
    pub monitoring: MemoryMonitoringConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryPoolConfig {
    /// Initial pool size
    pub initial_size: usize,
    
    /// Maximum pool size
    pub max_size: usize,
    
    /// Pool growth factor
    pub growth_factor: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GarbageCollectionConfig {
    /// Enable garbage collection
    pub enabled: bool,
    
    /// GC interval
    pub interval: Duration,
    
    /// GC threshold
    pub threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMonitoringConfig {
    /// Enable memory monitoring
    pub enabled: bool,
    
    /// Memory usage threshold
    pub usage_threshold: f64,
}


impl Default for MemoryPoolConfig {
    fn default() -> Self {
        Self {
            initial_size: 1024 * 1024, // 1MB
            max_size: 1024 * 1024 * 1024, // 1GB
            growth_factor: 2.0,
        }
    }
}

impl Default for GarbageCollectionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(60),
            threshold: 0.8,
        }
    }
}

impl Default for MemoryMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            usage_threshold: 0.8,
        }
    }
}

impl MemoryPerformanceConfig {
    pub fn validate(&self) -> Result<()> {
        if self.pool.max_size < self.pool.initial_size {
            return Err(NestGateError::Configuration {
                field: "memory.pool.max_size".to_string(),
                message: "Maximum pool size cannot be less than initial size".to_string(),
                current_value: Some(self.pool.max_size.to_string()),
                expected: Some(format!(">={}", self.pool.initial_size)),
                user_error: true,
            });
        }
        Ok(())
    }
} 