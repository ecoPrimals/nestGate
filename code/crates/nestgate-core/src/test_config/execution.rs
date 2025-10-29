///
/// This module provides configuration for test execution, cleanup, and reporting.
/// Consolidates: TestExecutionConfig, TestRetryConfig, CleanTestConfig
use serde::{Deserialize, Serialize};
use std::time::Duration;
// ==================== SECTION ====================

/// **Unified test execution configuration**
/// Consolidates: TestExecutionConfig, TestRetryConfig, CleanTestConfig
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestExecutionConfig {
    /// Test timeout duration
    pub timeout: Duration,
    /// Maximum test retries
    pub max_retries: u32,
    /// Retry delay between attempts
    pub retry_delay: Duration,
    /// Parallel test execution
    pub parallel_execution: bool,
    /// Maximum parallel tests
    pub max_parallel_tests: usize,
    /// Test cleanup configuration
    pub cleanup: TestCleanupConfig,
    /// Test reporting configuration
    pub reporting: TestReportingConfig,
}
impl Default for TestExecutionConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(300),
            max_retries: 3,
            retry_delay: Duration::from_millis(1000),
            parallel_execution: true,
            max_parallel_tests: 8,
            cleanup: TestCleanupConfig::default(),
            reporting: TestReportingConfig::default(),
        }
    }
}

/// **Test cleanup configuration**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCleanupConfig {
    /// Enable automatic cleanup
    pub auto_cleanup: bool,
    /// Cleanup timeout
    pub cleanup_timeout: Duration,
    /// Keep test artifacts on failure
    pub keep_on_failure: bool,
    /// Cleanup retry attempts
    pub cleanup_retries: u32,
}
impl Default for TestCleanupConfig {
    fn default() -> Self {
        Self {
            auto_cleanup: true,
            cleanup_timeout: Duration::from_secs(60),
            keep_on_failure: true,
            cleanup_retries: 2,
        }
    }
}

/// **Test reporting configuration**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestReportingConfig {
    /// Enable detailed reporting
    pub detailed_reporting: bool,
    /// Output format (json, xml, html)
    pub output_format: String,
    /// Report output directory
    pub output_directory: String,
    /// Include performance metrics
    pub include_performance: bool,
    /// Include coverage information
    pub include_coverage: bool,
}
impl Default for TestReportingConfig {
    fn default() -> Self {
        Self {
            detailed_reporting: true,
            output_format: "json".to_string(),
            output_directory: "test-reports".to_string(),
            include_performance: true,
            include_coverage: true,
        }
    }
}
