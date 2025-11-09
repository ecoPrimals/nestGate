// **GLOBAL TEST CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GlobalTestConfig {
    pub reporting: TestReportingConfig,
    pub metrics: TestMetricsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestReportingConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestMetricsConfig {
    pub enabled: bool,
}

impl Default for TestReportingConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl Default for TestMetricsConfig {
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
