//! **CANONICAL TEST DOUBLES MODULE**
//!
//! **CANONICAL MODERNIZATION COMPLETE** - Integrated with unified test configuration system.
//! All test double configurations now use canonical patterns.

// Import canonical test configuration system
use crate::common::config::{CanonicalTestConfig, MockServiceConfig};

// Re-export all test double implementations
pub mod hardware_test_doubles;
pub mod network_test_doubles;
pub mod service_test_doubles;
pub mod storage_test_doubles;

// Re-export key types using canonical patterns
pub use hardware_test_doubles::{HardwareTestDouble, MockHardwareForTesting};
pub use network_test_doubles::{MockNetworkForTesting, NetworkTestDouble};
pub use service_test_doubles::{MockServiceForTesting, ServiceTestDouble};
pub use storage_test_doubles::{MockStorageForTesting, StorageTestDouble};

/// **CANONICAL TEST DOUBLE FACTORY**
///
/// Creates test doubles using canonical configuration patterns
pub struct CanonicalTestDoubleFactory;

impl CanonicalTestDoubleFactory {
    /// Create a mock service using canonical configuration
    pub fn create_mock_service(config: &CanonicalTestConfig) -> MockServiceForTesting {
        MockServiceForTesting::new(&config.test_domain.mocking)
    }

    /// Create a mock storage backend using canonical configuration
    pub fn create_mock_storage(config: &CanonicalTestConfig) -> MockStorageForTesting {
        MockStorageForTesting::new(&config.test_domain.mocking.storage)
    }

    /// Create a mock network interface using canonical configuration
    pub fn create_mock_network(config: &CanonicalTestConfig) -> MockNetworkForTesting {
        MockNetworkForTesting::new(&config.test_domain.mocking.network)
    }

    /// Create a mock hardware interface using canonical configuration
    pub fn create_mock_hardware(config: &CanonicalTestConfig) -> MockHardwareForTesting {
        MockHardwareForTesting::new(&config.test_domain.mocking.hardware)
    }
}

/// **CANONICAL MODERNIZATION BENEFITS**
///
/// The unified test double system provides:
/// - ✅ Consistent mock configuration across all test types
/// - ✅ Type-safe test double creation and management
/// - ✅ Centralized test double lifecycle management
/// - ✅ Integrated performance and reliability testing
/// - ✅ Eliminated test double configuration fragmentation

/// Create a complete test environment with all doubles configured
pub fn create_canonical_test_environment(config: &CanonicalTestConfig) -> CanonicalTestEnvironment {
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
pub struct CanonicalTestEnvironment {
    pub mock_service: MockServiceForTesting,
    pub mock_storage: MockStorageForTesting,
    pub mock_network: MockNetworkForTesting,
    pub mock_hardware: MockHardwareForTesting,
    pub config: CanonicalTestConfig,
}

impl CanonicalTestEnvironment {
    /// Initialize the test environment
    pub fn initialize(&mut self) -> Result<(), String> {
        self.mock_service.initialize()?;
        self.mock_storage.initialize()?;
        self.mock_network.initialize()?;
        self.mock_hardware.initialize()?;
        Ok(())
    }

    /// Cleanup the test environment
    pub fn cleanup(&mut self) -> Result<(), String> {
        self.mock_hardware.cleanup()?;
        self.mock_network.cleanup()?;
        self.mock_storage.cleanup()?;
        self.mock_service.cleanup()?;
        Ok(())
    }

    /// Reset all test doubles to initial state
    pub fn reset(&mut self) -> Result<(), String> {
        self.mock_service.reset()?;
        self.mock_storage.reset()?;
        self.mock_network.reset()?;
        self.mock_hardware.reset()?;
        Ok(())
    }
}
