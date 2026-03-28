// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

/// **PERFORMANCE CONFIGURATION**
///
/// Performance and optimization configuration types.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Performance configuration with const generics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Performance
pub struct PerformanceConfig<const MAX_CONNECTIONS: usize = 1000, const BUFFER_SIZE: usize = 65536>
{
    /// Enable performance optimizations
    pub enabled: bool,
    /// Maximum number of concurrent connections
    pub max_connections: usize,
    /// Buffer size for I/O operations
    pub buffer_size: usize,
    /// Performance settings
    pub performance_settings: HashMap<String, serde_json::Value>,
    /// Testing configuration
    pub testing: PerformanceTestingConfig,
}
/// Performance testing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for PerformanceTesting
pub struct PerformanceTestingConfig {
    /// Number of test iterations
    pub test_iterations: usize,
    /// Percentile target for measurements
    pub percentile_target: f64,
    /// Baseline timeout in seconds
    pub baseline_timeout_seconds: u64,
}
impl<const MAX_CONNECTIONS: usize, const BUFFER_SIZE: usize> Default
    for PerformanceConfig<MAX_CONNECTIONS, BUFFER_SIZE>
{
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            max_connections: MAX_CONNECTIONS,
            buffer_size: BUFFER_SIZE,
            performance_settings: HashMap::new(),
            testing: PerformanceTestingConfig::default(),
        }
    }
}

impl Default for PerformanceTestingConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            test_iterations: 10,
            percentile_target: 95.0,
            baseline_timeout_seconds: 30,
        }
    }
}
