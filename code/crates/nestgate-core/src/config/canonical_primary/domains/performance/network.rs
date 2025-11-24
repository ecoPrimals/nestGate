// **NETWORK PERFORMANCE CONFIGURATION**

use crate::Result;
use serde::{Deserialize, Serialize};

/// Network performance configuration for optimizing network I/O.
///
/// Controls network-specific optimizations and buffering strategies.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkPerformanceConfig {
    /// Network optimization settings.
    pub optimization: NetworkOptimizationConfig,
    /// Network buffering configuration.
    pub buffering: NetworkBufferingConfig,
}

/// Network optimization configuration.
///
/// Enables network-specific performance optimizations.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkOptimizationConfig {
    /// Whether network optimization is enabled.
    pub enabled: bool,
}

/// Network buffering configuration.
///
/// Controls buffer sizes for network operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkBufferingConfig {
    /// Network buffer size in bytes.
    pub buffer_size: usize,
}

impl NetworkPerformanceConfig {
    /// Validate network performance configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if validation fails.
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}
