//! Test Doubles for NestGate Testing Infrastructure
//!
//! This module contains ACTUAL TEST MOCKS - not hardware abstractions.
//! These are pure test doubles used for unit and integration testing.
//!
//! ## Clear Separation of Concerns
//! - **This module**: Test infrastructure only (unit tests, integration tests)
//! - **dev_environment module**: Production-ready hardware abstractions
//!
//! ## Usage
//! These test doubles should ONLY be used in test code and NEVER in production paths.

pub mod storage_test_doubles;
pub mod network_test_doubles;
pub mod service_test_doubles;
pub mod hardware_test_doubles;

pub use storage_test_doubles::{StorageTestDouble, MockStorageForTesting};
pub use network_test_doubles::{NetworkTestDouble, MockNetworkForTesting};
pub use service_test_doubles::{ServiceTestDouble, MockServiceForTesting};
pub use hardware_test_doubles::{HardwareTestDouble, MockHardwareForTesting};

/// Test double configuration
#[derive(Debug, Clone)]
pub struct TestDoubleConfig {
    /// Whether to simulate failures
    pub simulate_failures: bool,
    /// Response delay in milliseconds
    pub response_delay_ms: u64,
    /// Whether to log all test double operations
    pub verbose_logging: bool,
}

impl Default for TestDoubleConfig {
    fn default() -> Self {
        Self {
            simulate_failures: false,
            response_delay_ms: 0,
            verbose_logging: false,
        }
    }
} 