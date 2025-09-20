// **PERFORMANCE TEST CONFIGURATION**

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceTestConfig {
    pub benchmark: BenchmarkConfig,
    pub profiling: ProfilingConfig,
    pub metrics: MetricsTestConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    pub enabled: bool,
    pub timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilingConfig {
    pub enabled: bool,
    pub timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsTestConfig {
    pub enabled: bool,
    pub timeout: Duration,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            timeout: Duration::from_secs(120),
        }
    }
}

impl Default for ProfilingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            timeout: Duration::from_secs(120),
        }
    }
}

impl Default for MetricsTestConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            timeout: Duration::from_secs(60),
        }
    }
}

impl PerformanceTestConfig {
    #[must_use]
    pub const fn ci_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn development_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn merge(self, _other: Self) -> Self {
        self
    }
}
