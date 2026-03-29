// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// **GLOBAL TEST CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for `GlobalTest`
pub struct GlobalTestConfig {
    /// Reporting
    pub reporting: TestReportingConfig,
    /// Metrics
    pub metrics: TestMetricsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `TestReporting`
pub struct TestReportingConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `TestMetrics`
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
    /// Creates a CI-optimized global test configuration
    #[must_use]
    pub fn ci_optimized() -> Self {
        Self::default()
    }
    /// Creates a development-optimized global test configuration
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }
    /// Merges this configuration with another, taking precedence
    #[must_use]
    pub const fn merge(self, _other: Self) -> Self {
        self
    }
}
