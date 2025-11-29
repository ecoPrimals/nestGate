// **PERFORMANCE TEST CONFIGURATION**

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for PerformanceTest
pub struct PerformanceTestConfig {
    /// Benchmark
    pub benchmark: BenchmarkConfig,
    /// Profiling
    pub profiling: ProfilingConfig,
    /// Metrics
    pub metrics: MetricsTestConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Benchmark
pub struct BenchmarkConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Timeout
    pub timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Profiling
pub struct ProfilingConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Timeout
    pub timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for MetricsTest
pub struct MetricsTestConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Timeout
    pub timeout: Duration,
}

impl Default for BenchmarkConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            timeout: Duration::from_secs(120),
        }
    }
}

impl Default for ProfilingConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            timeout: Duration::from_secs(120),
        }
    }
}

impl Default for MetricsTestConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            timeout: Duration::from_secs(60),
        }
    }
}

impl PerformanceTestConfig {
    #[must_use]
    pub fn ci_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn merge(self, _other: Self) -> Self {
        self
    }
}
