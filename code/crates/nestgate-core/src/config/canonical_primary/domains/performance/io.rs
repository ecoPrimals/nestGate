//! I/O performance configuration module
//! Provides unified I/O optimization and storage performance settings.

use serde::{Deserialize, Serialize};

use crate::{NestGateError, Result};

/// I/O performance configuration for optimizing disk and storage operations.
///
/// Controls I/O strategies, buffering, and read-ahead for maximum throughput.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for IoPerformance
pub struct IoPerformanceConfig {
    /// I/O optimization settings for strategy selection.
    pub optimization: IoOptimizationConfig,

    /// Buffering configuration for I/O operations.
    pub buffering: IoBufferingConfig,
}

/// I/O optimization configuration for selecting I/O strategies.
///
/// Determines whether I/O operations are blocking, non-blocking, or asynchronous.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for IoOptimization
pub struct IoOptimizationConfig {
    /// Whether I/O optimization is enabled.
    pub enabled: bool,

    /// I/O execution strategy.
    pub strategy: IoStrategy,
}

/// I/O execution strategy.
///
/// Determines how I/O operations are performed.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Iostrategy
pub enum IoStrategy {
    /// Blocking I/O - thread waits for completion.
    Blocking,
    /// Non-blocking I/O - returns immediately.
    NonBlocking,
    /// Asynchronous I/O - uses async runtime.
    Async,
}

/// I/O buffering configuration for optimizing read/write operations.
///
/// Controls buffer sizes and read-ahead behavior.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for IoBuffering
pub struct IoBufferingConfig {
    /// Buffer size in bytes (default: 64KB).
    pub buffer_size: usize,

    /// Whether to enable read-ahead prefetching.
    pub read_ahead: bool,
}

impl Default for IoOptimizationConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            strategy: IoStrategy::Async,
        }
    }
}

impl Default for IoBufferingConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            buffer_size: 64 * 1024, // 64KB
            read_ahead: true,
        }
    }
}

impl IoPerformanceConfig {
    /// Validate I/O performance configuration.
    ///
    /// Ensures buffer sizes are non-zero and strategies are valid.
    ///
    /// # Errors
    ///
    /// Returns an error if buffer size is zero.
    pub fn validate(&self) -> Result<()> {
        if self.buffering.buffer_size == 0 {
            return Err(NestGateError::configuration_error_detailed(
                "io.buffering.buffer_size".to_string(),
                "Buffer size cannot be zero".to_string(),
                Some("0".to_string()),
                Some(">0".to_string()),
                true,
            ));
        }
        Ok(())
    }
}
