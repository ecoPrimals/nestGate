// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **CANONICAL TEST DOUBLES MODULE**
//!
//! **CANONICAL MODERNIZATION COMPLETE** - Integrated with unified test configuration system.
//! All test double configurations now use canonical patterns.

// Import canonical test configuration system (only available with dev-stubs feature)
// Use the correct type from nestgate-core directly
#[cfg(feature = "dev-stubs")]
use nestgate_core::config::canonical_primary::domains::test_canonical::CanonicalTestConfigs;

// Re-export all test double implementations.
// These are test utility libraries — many building blocks defined for future test scenarios.
#[allow(dead_code)]
pub mod hardware_test_doubles;
#[allow(dead_code)]
pub mod network_test_doubles;
#[allow(dead_code)]
pub mod service_test_doubles;
#[allow(dead_code)]
pub mod storage_test_doubles;

// Re-export mock types for ergonomic access
pub use hardware_test_doubles::MockHardwareForTesting;
pub use network_test_doubles::MockNetworkForTesting;
pub use service_test_doubles::MockServiceForTesting;
pub use storage_test_doubles::MockStorageForTesting;

// Re-export key types using canonical patterns

/// **COMPATIBILITY ALIAS** for test doubles migrating from old patterns
/// This type will be phased out as all test doubles adopt canonical patterns
#[derive(Debug, Clone, Default)]
pub struct TestDoubleConfig {
    pub enabled: bool,
    pub simulate_failures: bool,
    pub response_delay_ms: u64,
    pub verbose_logging: bool,
}

impl TestDoubleConfig {
    pub fn new() -> Self {
        Self {
            enabled: true,
            simulate_failures: false,
            response_delay_ms: 0,
            verbose_logging: false,
        }
    }
}

/// **COMPATIBILITY ALIAS** - Mock service config
/// Use CanonicalTestConfig in new code
pub type MockServiceConfig = TestDoubleConfig;

/// **CANONICAL TEST DOUBLE FACTORY**
///
/// Creates test doubles using canonical configuration patterns
#[cfg(feature = "dev-stubs")]
pub struct CanonicalTestDoubleFactory;

#[cfg(feature = "dev-stubs")]
impl CanonicalTestDoubleFactory {
    /// Create a mock service using canonical configuration
    pub fn create_mock_service(_config: &CanonicalTestConfigs) -> MockServiceForTesting {
        MockServiceForTesting::new()
    }

    /// Create a mock storage backend using canonical configuration
    pub fn create_mock_storage(_config: &CanonicalTestConfigs) -> MockStorageForTesting {
        MockStorageForTesting::new()
    }

    /// Create a mock network interface using canonical configuration
    pub fn create_mock_network(_config: &CanonicalTestConfigs) -> MockNetworkForTesting {
        MockNetworkForTesting::new()
    }

    /// Create a mock hardware interface using canonical configuration
    pub fn create_mock_hardware(_config: &CanonicalTestConfigs) -> MockHardwareForTesting {
        MockHardwareForTesting::new()
    }
}

/// Create a complete test environment with all doubles configured.
///
/// The unified test double system provides:
/// - Consistent mock configuration across all test types
/// - Type-safe test double creation and management
/// - Centralized test double lifecycle management
/// - Integrated performance and reliability testing
#[cfg(feature = "dev-stubs")]
pub fn create_canonical_test_environment(
    config: &CanonicalTestConfigs,
) -> CanonicalTestEnvironment {
    CanonicalTestEnvironment {
        mock_service: CanonicalTestDoubleFactory::create_mock_service(config),
        mock_storage: CanonicalTestDoubleFactory::create_mock_storage(config),
        mock_network: CanonicalTestDoubleFactory::create_mock_network(config),
        mock_hardware: CanonicalTestDoubleFactory::create_mock_hardware(config),
        config: config.clone(),
    }
}

/// **CANONICAL TEST ENVIRONMENT**
///
/// Complete test environment with all necessary test doubles
#[cfg(feature = "dev-stubs")]
pub struct CanonicalTestEnvironment {
    pub mock_service: MockServiceForTesting,
    pub mock_storage: MockStorageForTesting,
    pub mock_network: MockNetworkForTesting,
    pub mock_hardware: MockHardwareForTesting,
    pub config: CanonicalTestConfigs,
}

#[cfg(feature = "dev-stubs")]
impl CanonicalTestEnvironment {
    /// Initialize the test environment
    pub fn initialize(&mut self) -> Result<(), String> {
        self.mock_service
            .initialize()
            .map_err(|e| format!("Service initialization failed: {}", e))?;
        self.mock_storage
            .initialize()
            .map_err(|e| format!("Storage initialization failed: {}", e))?;
        self.mock_network
            .initialize()
            .map_err(|e| format!("Network initialization failed: {}", e))?;
        self.mock_hardware
            .initialize()
            .map_err(|e| format!("Hardware initialization failed: {}", e))?;
        Ok(())
    }

    /// Cleanup the test environment
    pub fn cleanup(&mut self) -> Result<(), String> {
        self.mock_hardware
            .cleanup()
            .map_err(|e| format!("Hardware cleanup failed: {}", e))?;
        self.mock_network
            .cleanup()
            .map_err(|e| format!("Network cleanup failed: {}", e))?;
        self.mock_storage
            .cleanup()
            .map_err(|e| format!("Storage cleanup failed: {}", e))?;
        self.mock_service
            .cleanup()
            .map_err(|e| format!("Service cleanup failed: {}", e))?;
        Ok(())
    }

    /// Reset all test doubles to initial state
    pub fn reset(&mut self) -> Result<(), String> {
        self.mock_service
            .reset()
            .map_err(|e| format!("Service reset failed: {}", e))?;
        self.mock_storage
            .reset()
            .map_err(|e| format!("Storage reset failed: {}", e))?;
        self.mock_network
            .reset()
            .map_err(|e| format!("Network reset failed: {}", e))?;
        self.mock_hardware
            .reset()
            .map_err(|e| format!("Hardware reset failed: {}", e))?;
        Ok(())
    }
}
