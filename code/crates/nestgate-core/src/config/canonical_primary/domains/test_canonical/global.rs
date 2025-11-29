// **GLOBAL TEST CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for GlobalTest
pub struct GlobalTestConfig {
    /// Reporting
    pub reporting: TestReportingConfig,
    /// Metrics
    pub metrics: TestMetricsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for TestReporting
pub struct TestReportingConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for TestMetrics
pub struct TestMetricsConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

impl Default for TestReportingConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl Default for TestMetricsConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl GlobalTestConfig {
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
