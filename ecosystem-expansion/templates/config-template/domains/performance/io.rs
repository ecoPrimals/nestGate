//! **I/O PERFORMANCE CONFIGURATION**
//!
//! I/O optimization and buffering configuration.

use serde::{Deserialize, Serialize};

use crate::{NestGateError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct IoPerformanceConfig {
    /// I/O optimization settings
    pub optimization: IoOptimizationConfig,
    
    /// Buffering configuration
    pub buffering: IoBufferingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoOptimizationConfig {
    /// Enable I/O optimization
    pub enabled: bool,
    
    /// I/O strategy
    pub strategy: IoStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IoStrategy {
    Blocking,
    NonBlocking,
    Async,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoBufferingConfig {
    /// Buffer size
    pub buffer_size: usize,
    
    /// Enable read-ahead
    pub read_ahead: bool,
}


impl Default for IoOptimizationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            strategy: IoStrategy::Async,
        }
    }
}

impl Default for IoBufferingConfig {
    fn default() -> Self {
        Self {
            buffer_size: 64 * 1024, // 64KB
            read_ahead: true,
        }
    }
}

impl IoPerformanceConfig {
    pub fn validate(&self) -> Result<()> {
        if self.buffering.buffer_size == 0 {
            return Err(NestGateError::Configuration {
                field: "io.buffering.buffer_size".to_string(),
                message: "Buffer size cannot be zero".to_string(),
                current_value: Some("0".to_string()),
                expected: Some(">0".to_string()),
                user_error: true,
            });
        }
        Ok(())
    }
} 