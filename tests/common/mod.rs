pub use nestgate_core::unified_enums::UnifiedServiceType;
pub use nestgate_core::unified_types::UnifiedConfig;
/// Clean, rebuilt common test infrastructure
/// Eliminates duplicate definitions and import conflicts
// Re-export core types we need for testing - avoid conflicts by being explicit
pub use nestgate_core::{NestGateError, Result};

use serde::{Deserialize, Serialize};
use std::time::Duration;

// Declare submodules
pub mod config;
pub mod helpers;
pub mod mocks;

// Re-export key utilities for convenience
pub use config::CompleteTestConfig;
pub use helpers::{TestHelpers, TestSetup};
pub use mocks::{MockServiceRegistry, MockStorageService, MockUniversalService};

/// Clean test configuration without conflicts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanTestConfig {
    pub name: String,
    pub timeout: Duration,
    pub max_concurrent: usize,
    pub enable_chaos: bool,
}

impl Default for CleanTestConfig {
    fn default() -> Self {
        Self {
            name: "default_test".to_string(),
            timeout: Duration::from_secs(30),
            max_concurrent: 10,
            enable_chaos: false,
        }
    }
}

/// Simple test service for mocking
#[derive(Debug, Clone)]
pub struct SimpleTestService {
    pub service_type: UnifiedServiceType,
    pub name: String,
    pub enabled: bool,
}

impl SimpleTestService {
    pub fn new(service_type: UnifiedServiceType, name: String) -> Self {
        Self {
            service_type,
            name,
            enabled: true,
        }
    }

    pub fn storage(name: String) -> Self {
        Self::new(UnifiedServiceType::Storage, name)
    }

    pub fn network(name: String) -> Self {
        Self::new(UnifiedServiceType::Network, name)
    }

    pub fn security(name: String) -> Self {
        Self::new(UnifiedServiceType::Security, name)
    }
}

/// Test result tracking
#[derive(Debug, Clone)]
pub struct TestResult {
    pub test_name: String,
    pub success: bool,
    pub duration: Duration,
    pub error: Option<String>,
}

impl TestResult {
    pub fn success(test_name: String, duration: Duration) -> Self {
        Self {
            test_name,
            success: true,
            duration,
            error: None,
        }
    }

    pub fn failure(test_name: String, duration: Duration, error: String) -> Self {
        Self {
            test_name,
            success: false,
            duration,
            error: Some(error),
        }
    }
}

/// Test utilities
pub struct TestUtils;

impl TestUtils {
    /// Create a test configuration with custom timeout
    pub fn config_with_timeout(timeout_secs: u64) -> CleanTestConfig {
        CleanTestConfig {
            timeout: Duration::from_secs(timeout_secs),
            ..Default::default()
        }
    }

    /// Generate a unique test name
    pub fn unique_test_name(prefix: &str) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        format!("{}_{}", prefix, timestamp)
    }

    /// Create a simple unified config for testing
    pub fn simple_unified_config() -> UnifiedConfig {
        UnifiedConfig::default()
    }

    /// Wait for a condition with timeout
    pub async fn wait_for_condition<F, Fut>(
        condition: F,
        timeout: Duration,
        check_interval: Duration,
    ) -> Result<()>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = bool>,
    {
        let start = std::time::Instant::now();

        while start.elapsed() < timeout {
            if condition().await {
                return Ok(());
            }
            tokio::time::sleep(check_interval).await;
        }

        Err(NestGateError::internal_error(
            "Condition not met within timeout".to_string(),
            "test_utils".to_string(),
        ))
    }
}
