use std::future::Future;
// Test common utilities and helpers
// **MODERNIZED**: Removed async_trait dependency for zero-cost async
use nestgate_core::canonical_modernization::UnifiedCapabilityType;
use nestgate_core::error::Result;
use nestgate_core::{UnifiedServiceState, UniversalService};

// Re-export configuration modules
pub mod config;
pub use config::UnifiedTestConfig;

// Test utilities
// REMOVED: helpers module - use test_helpers directly
// pub mod helpers;
pub mod builders; // ✅ NEW: Test data builders
pub mod mocks;
pub mod templates; // ✅ NEW: Test templates and macros
pub mod test_config;
pub mod test_doubles;
pub mod test_error_handling;
pub mod test_service_manager; // ✅ NEW: Dynamic test port allocation
pub mod utils;

// Re-export commonly used test utilities
pub use mocks::{MockServiceRegistry, MockStorageService, MockUniversalService};
pub use test_helpers::{TestHelpers, TestSetup};
pub use test_service_manager::{
    get_test_api_endpoint, get_test_endpoint, TestConfig as DynamicTestConfig, TestServiceManager,
}; // ✅ NEW: Export test service manager
pub use utils::TestUtils;

// Test configuration types from canonical system
pub use config::UnifiedTestConfig as TestConfig;
pub use nestgate_core::config::canonical_master::NestGateCanonicalConfig as CleanTestConfig;

/// **TEST ENVIRONMENT SYSTEM**
pub mod test_environment;
pub use test_environment::{
    get_test_environment, init_test_environment, test_service_address, test_url,
    test_websocket_url, TestDatabaseConfig, TestEnvironment, TestEnvironmentBuilder,
    TestStorageConfig, TestTimeouts,
};

/// Simple test service for mocking
#[derive(Clone, Debug)]
pub struct SimpleTestService {
    pub name: String,
}

impl SimpleTestService {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

/// **ZERO-COST ASYNC**: Native async implementation without async_trait overhead
impl UniversalService for SimpleTestService {
    type Config = String;
    type Health = bool;

    fn initialize(
        &mut self,
        _config: Self::Config,
    ) -> impl std::future::Future<Output = Result<()>> + Send {
        async move { Ok(()) }
    }

    fn start(&mut self) -> impl std::future::Future<Output = Result<()>> + Send {
        async move { Ok(()) }
    }

    fn stop(&mut self) -> impl std::future::Future<Output = Result<()>> + Send {
        async move { Ok(()) }
    }

    fn status(&self) -> impl std::future::Future<Output = UnifiedServiceState> + Send {
        async move { UnifiedServiceState::Running }
    }

    fn health(&self) -> impl std::future::Future<Output = Result<Self::Health>> + Send {
        async move { Ok(true) }
    }

    fn service_id(&self) -> &str {
        &self.name
    }

    fn service_type(&self) -> UnifiedCapabilityType {
        UnifiedCapabilityType::Storage
    }

    fn handle_request<'a>(
        &self,
        _request: &'a str,
    ) -> impl std::future::Future<Output = Result<String>> + Send {
        async move { Ok("test_response".to_string()) }
    }

    fn capabilities<'a>(&self) -> impl std::future::Future<Output = Result<Vec<String>>> + Send {
        async move { Ok(vec!["test".to_string()]) }
    }
}
