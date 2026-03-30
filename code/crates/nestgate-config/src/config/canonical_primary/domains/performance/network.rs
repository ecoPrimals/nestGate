// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// **NETWORK PERFORMANCE CONFIGURATION**

use nestgate_types::error::Result;
use serde::{Deserialize, Serialize};

/// Network performance configuration for optimizing network I/O.
///
/// Controls network-specific optimizations and buffering strategies.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for `NetworkPerformance`
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
/// Configuration for `NetworkOptimization`
pub struct NetworkOptimizationConfig {
    /// Whether network optimization is enabled.
    pub enabled: bool,
}

/// Network buffering configuration.
///
/// Controls buffer sizes for network operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for `NetworkBuffering`
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
    pub const fn validate(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn network_performance_validate_serde() {
        let c = NetworkPerformanceConfig::default();
        c.validate().expect("validate");
        let s = serde_json::to_string(&c).expect("to_string");
        let _: NetworkPerformanceConfig = serde_json::from_str(&s).expect("from_str");
    }
}
