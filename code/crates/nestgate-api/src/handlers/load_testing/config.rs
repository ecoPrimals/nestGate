//! Load testing configuration types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Load testing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust
/// // OLD (deprecated):
/// use crate::network::config::LoadTestConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::LoadTestConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
pub struct LoadTestConfig {
    /// Test duration in seconds
    pub duration_seconds: u64,
    /// Number of concurrent users
    pub concurrent_users: u32,
    /// Requests per second per user
    pub requests_per_second: f64,
    /// Test scenario type
    pub scenario: TestScenario,
    /// Target endpoints to test
    pub endpoints: Vec<String>,
    /// Test data parameters
    pub test_data: TestDataConfig,
    /// Performance thresholds
    pub thresholds: PerformanceThresholds,
}

/// Test scenario types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestScenario {
    /// **CONSTANT LOAD**
    ///
    /// Configuration for constant load testing.
    ConstantLoad,
    /// **RAMP CONFIGURATION**
    ///
    /// Configuration for gradual load increase testing.
    Ramp {
        /// Starting number of concurrent users
        start_users: u32,
        /// Ending number of concurrent users
        end_users: u32,
        /// Duration for ramping up load in seconds
        ramp_duration_seconds: u64,
    },
    /// **SPIKE CONFIGURATION**
    ///
    /// Configuration for sudden load spike testing.
    Spike {
        /// Baseline number of users before spike
        baseline_users: u32,
        /// Peak number of users during spike
        spike_users: u32,
        /// Duration of the spike in seconds
        spike_duration_seconds: u64,
    },
    /// **STEP CONFIGURATION**
    ///
    /// Configuration for stepped load increase testing.
    Step {
        /// Maximum number of users to reach
        max_users: u32,
        /// Number of users to add in each step
        step_users: u32,
        /// Duration of each step in seconds
        step_duration_seconds: u64,
    },
}

/// Load test parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadTestParameters {
    /// Load test configuration settings
    pub config: LoadTestConfig,
    /// Timestamp when the test was started, if running
    pub started_at: Option<std::time::SystemTime>,
    /// Unique identifier for this test run
    pub test_id: String,
}

/// Test data configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust
/// // OLD (deprecated):
/// use crate::network::config::TestDataConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::TestDataConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
pub struct TestDataConfig {
    /// Request payload size in bytes
    pub payload_size_bytes: usize,
    /// Response size expectations
    pub expected_response_size_bytes: Option<usize>,
    /// Custom headers to include
    pub custom_headers: HashMap<String, String>,
    /// Request body template
    pub body_template: Option<String>,
}

/// Performance thresholds for pass/fail criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    /// Maximum acceptable average response time (ms)
    pub max_avg_response_time_ms: f64,
    /// Maximum acceptable 95th percentile response time (ms)
    pub max_p95_response_time_ms: f64,
    /// Minimum acceptable success rate (0.0 - 1.0)
    pub min_success_rate: f64,
    /// Maximum acceptable error rate (0.0 - 1.0)
    pub max_error_rate: f64,
}

/// **LOAD TEST EXECUTION**
///
/// Represents an active load test execution with configuration and timing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadTestExecution {
    /// Load test configuration parameters
    pub config: LoadTestConfig,
    /// Timestamp when the test execution started
    pub started_at: Option<std::time::SystemTime>,
    /// Unique identifier for this test execution
    pub test_id: String,
}

impl Default for LoadTestConfig {
    fn default() -> Self {
        Self {
            duration_seconds: 60,
            concurrent_users: 10,
            requests_per_second: 1.0,
            scenario: TestScenario::ConstantLoad,
            endpoints: vec!["/health".to_string()],
            test_data: TestDataConfig::default(),
            thresholds: PerformanceThresholds::default(),
        }
    }
}

impl Default for TestDataConfig {
    fn default() -> Self {
        Self {
            payload_size_bytes: 1024,
            expected_response_size_bytes: None,
            custom_headers: HashMap::new(),
            body_template: None,
        }
    }
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            max_avg_response_time_ms: 1000.0,
            max_p95_response_time_ms: 2000.0,
            min_success_rate: 0.95,
            max_error_rate: 0.05,
        }
    }
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
pub type LoadTestConfigCanonical = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using LoadTestConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.


// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
pub type TestDataConfigCanonical = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using TestDataConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

