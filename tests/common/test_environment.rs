//! **CONFIGURABLE TEST ENVIRONMENT SYSTEM**
//!
//! This module provides a centralized, configurable test environment that eliminates
//! hardcoded values and enables dynamic test configuration through environment variables.

use nestgate_core::constants::canonical::network::DEFAULT_API_PORT;
use std::collections::HashMap;
use std::env;
use std::time::Duration;
use tests::config::ConsolidatedCanonicalConfig;

/// **CONFIGURABLE TEST ENVIRONMENT**
///
/// Provides environment-driven test configuration to eliminate hardcoded values
#[derive(Debug, Clone)]
pub struct TestEnvironment {
    /// Test host configuration
    pub host: String,
    /// Test port configuration
    pub port: u16,
    /// WebSocket port for testing
    pub websocket_port: u16,
    /// API base URL for testing
    pub api_base_url: String,
    /// WebSocket base URL for testing
    pub websocket_base_url: String,
    /// Test timeouts
    pub timeouts: TestTimeouts,
    /// Test database configuration
    pub database: TestDatabaseConfig,
    /// Test storage configuration
    pub storage: TestStorageConfig,
}

/// **TEST TIMEOUT CONFIGURATION**
#[derive(Debug, Clone)]
pub struct TestTimeouts {
    /// Default test timeout
    pub default_timeout: Duration,
    /// Integration test timeout
    pub integration_timeout: Duration,
    /// Performance test timeout
    pub performance_timeout: Duration,
    /// Chaos test timeout
    pub chaos_timeout: Duration,
}

/// **TEST DATABASE CONFIGURATION**
#[derive(Debug, Clone)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust
/// // OLD (deprecated):
/// use crate::network::config::TestDatabaseConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::TestDatabaseConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
pub struct TestDatabaseConfig {
    /// Database host
    pub host: String,
    /// Database port
    pub port: u16,
    /// Database name for testing
    pub database_name: String,
    /// Test data cleanup interval
    pub cleanup_interval: Duration,
}

/// **TEST STORAGE CONFIGURATION**
#[derive(Debug, Clone)]
pub struct TestStorageConfig {
    /// Storage backend type for testing
    pub backend_type: String,
    /// Storage path for test data
    pub data_path: String,
    /// Enable cleanup after tests
    pub auto_cleanup: bool,
}

impl Default for TestEnvironment {
    fn default() -> Self {
        Self::from_environment()
    }
}

impl TestEnvironment {
    /// Create test environment from environment variables with sensible defaults
    pub fn from_environment() -> Self {
        let host = env::var("NESTGATE_TEST_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("NESTGATE_TEST_PORT")
            .unwrap_or_else(|_| DEFAULT_API_PORT.to_string())
            .parse()
            .unwrap_or(DEFAULT_API_PORT);
        let websocket_port = env::var("NESTGATE_TEST_WS_PORT")
            .unwrap_or_else(|_| (DEFAULT_API_PORT + 1).to_string())
            .parse()
            .unwrap_or(DEFAULT_API_PORT + 1);

        Self {
            api_base_url: env::var("NESTGATE_TEST_API_URL")
                .unwrap_or_else(|_| format!("http://{}:{}", host, port)),
            websocket_base_url: env::var("NESTGATE_TEST_WS_URL")
                .unwrap_or_else(|_| format!("ws://{}:{}", host, websocket_port)),
            host,
            port,
            websocket_port,
            timeouts: TestTimeouts::from_environment(),
            database: TestDatabaseConfig::from_environment(),
            storage: TestStorageConfig::from_environment(),
        }
    }

    /// Create test environment for unit tests
    pub fn unit_tests() -> Self {
        let mut env = Self::from_environment();
        env.timeouts.default_timeout = Duration::from_secs(
            env::var("NESTGATE_TEST_UNIT_TIMEOUT")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
        );
        env
    }

    /// Create test environment for integration tests
    pub fn integration_tests() -> Self {
        let mut env = Self::from_environment();
        env.timeouts.integration_timeout = Duration::from_secs(
            env::var("NESTGATE_TEST_INTEGRATION_TIMEOUT")
                .unwrap_or_else(|_| "120".to_string())
                .parse()
                .unwrap_or(120),
        );
        env
    }

    /// Create test environment for performance tests
    pub fn performance_tests() -> Self {
        let mut env = Self::from_environment();
        env.timeouts.performance_timeout = Duration::from_secs(
            env::var("NESTGATE_TEST_PERFORMANCE_TIMEOUT")
                .unwrap_or_else(|_| "300".to_string())
                .parse()
                .unwrap_or(300),
        );
        env
    }

    /// Create test environment for chaos tests
    pub fn chaos_tests() -> Self {
        let mut env = Self::from_environment();
        env.timeouts.chaos_timeout = Duration::from_secs(
            env::var("NESTGATE_TEST_CHAOS_TIMEOUT")
                .unwrap_or_else(|_| "600".to_string())
                .parse()
                .unwrap_or(600),
        );
        env
    }

    /// Get test URL with path
    pub fn get_test_url(&self, path: &str) -> String {
        format!(
            "{}/{}",
            self.api_base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        )
    }

    /// Get WebSocket test URL with path
    pub fn get_websocket_url(&self, path: &str) -> String {
        format!(
            "{}/{}",
            self.websocket_base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        )
    }

    /// Create test service address
    pub fn get_service_address(&self, service_name: &str) -> String {
        format!("{}:{}", self.host, self.port + 100) // Offset for service ports
    }

    /// Get environment variables for test setup
    pub fn as_env_vars(&self) -> HashMap<String, String> {
        let mut vars = HashMap::new();
        vars.insert("NESTGATE_TEST_HOST".to_string(), self.host.clone());
        vars.insert("NESTGATE_TEST_PORT".to_string(), self.port.to_string());
        vars.insert(
            "NESTGATE_TEST_WS_PORT".to_string(),
            self.websocket_port.to_string(),
        );
        vars.insert(
            "NESTGATE_TEST_API_URL".to_string(),
            self.api_base_url.clone(),
        );
        vars.insert(
            "NESTGATE_TEST_WS_URL".to_string(),
            self.websocket_base_url.clone(),
        );
        vars
    }
}

impl TestTimeouts {
    pub fn from_environment() -> Self {
        Self {
            default_timeout: Duration::from_secs(
                env::var("NESTGATE_TEST_DEFAULT_TIMEOUT")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()
                    .unwrap_or(30),
            ),
            integration_timeout: Duration::from_secs(
                env::var("NESTGATE_TEST_INTEGRATION_TIMEOUT")
                    .unwrap_or_else(|_| "120".to_string())
                    .parse()
                    .unwrap_or(120),
            ),
            performance_timeout: Duration::from_secs(
                env::var("NESTGATE_TEST_PERFORMANCE_TIMEOUT")
                    .unwrap_or_else(|_| "300".to_string())
                    .parse()
                    .unwrap_or(300),
            ),
            chaos_timeout: Duration::from_secs(
                env::var("NESTGATE_TEST_CHAOS_TIMEOUT")
                    .unwrap_or_else(|_| "600".to_string())
                    .parse()
                    .unwrap_or(600),
            ),
        }
    }
}

impl TestDatabaseConfig {
    pub fn from_environment() -> Self {
        let host = env::var("NESTGATE_TEST_DB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("NESTGATE_TEST_DB_PORT")
            .unwrap_or_else(|_| "5432".to_string())
            .parse()
            .unwrap_or(5432);

        Self {
            host,
            port,
            database_name: env::var("NESTGATE_TEST_DB_NAME")
                .unwrap_or_else(|_| "nestgate_test".to_string()),
            cleanup_interval: Duration::from_secs(
                env::var("NESTGATE_TEST_DB_CLEANUP_INTERVAL")
                    .unwrap_or_else(|_| "3600".to_string())
                    .parse()
                    .unwrap_or(3600),
            ),
        }
    }
}

impl TestStorageConfig {
    pub fn from_environment() -> Self {
        Self {
            backend_type: env::var("NESTGATE_TEST_STORAGE_BACKEND")
                .unwrap_or_else(|_| "memory".to_string()),
            data_path: env::var("NESTGATE_TEST_STORAGE_PATH")
                .unwrap_or_else(|_| "/tmp/nestgate_test".to_string()),
            auto_cleanup: env::var("NESTGATE_TEST_STORAGE_CLEANUP")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
        }
    }
}

/// **TEST ENVIRONMENT BUILDER**
///
/// Fluent builder for creating custom test environments
pub struct TestEnvironmentBuilder {
    host: Option<String>,
    port: Option<u16>,
    websocket_port: Option<u16>,
    api_base_url: Option<String>,
    websocket_base_url: Option<String>,
    timeouts: Option<TestTimeouts>,
}

impl TestEnvironmentBuilder {
    pub fn new() -> Self {
        Self {
            host: None,
            port: None,
            websocket_port: None,
            api_base_url: None,
            websocket_base_url: None,
            timeouts: None,
        }
    }

    pub fn host(mut self, host: &str) -> Self {
        self.host = Some(host.to_string());
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn websocket_port(mut self, port: u16) -> Self {
        self.websocket_port = Some(port);
        self
    }

    pub fn timeouts(mut self, timeouts: TestTimeouts) -> Self {
        self.timeouts = Some(timeouts);
        self
    }

    pub fn build(self) -> TestEnvironment {
        let base = TestEnvironment::from_environment();

        TestEnvironment {
            host: self.host.unwrap_or(base.host),
            port: self.port.unwrap_or(base.port),
            websocket_port: self.websocket_port.unwrap_or(base.websocket_port),
            api_base_url: self.api_base_url.unwrap_or(base.api_base_url),
            websocket_base_url: self.websocket_base_url.unwrap_or(base.websocket_base_url),
            timeouts: self.timeouts.unwrap_or(base.timeouts),
            database: base.database,
            storage: base.storage,
        }
    }
}

/// **GLOBAL TEST ENVIRONMENT**
///
/// Singleton test environment for consistent configuration across all tests
static TEST_ENV: std::sync::OnceLock<TestEnvironment> = std::sync::OnceLock::new();

/// Get the global test environment
pub fn get_test_environment() -> &'static TestEnvironment {
    TEST_ENV.get_or_init(|| TestEnvironment::from_environment())
}

/// Initialize test environment with custom configuration
pub fn init_test_environment(env: TestEnvironment) -> &'static TestEnvironment {
    TEST_ENV.get_or_init(|| env)
}

/// **TEST UTILITIES**

/// Create test URL using environment configuration
pub fn test_url(path: &str) -> String {
    get_test_environment().get_test_url(path)
}

/// Create WebSocket test URL using environment configuration
pub fn test_websocket_url(path: &str) -> String {
    get_test_environment().get_websocket_url(path)
}

/// Get test service address for a service
pub fn test_service_address(service_name: &str) -> String {
    get_test_environment().get_service_address(service_name)
}


// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
pub type TestDatabaseConfigCanonical = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using TestDatabaseConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_from_env_vars() -> Result<(), Box<dyn std::error::Error>> {
        // Test environment variable configuration
        env::set_var("NESTGATE_TEST_HOST", "testhost.example.com");
        env::set_var("NESTGATE_TEST_PORT", "9090");

        let env = TestEnvironment::from_environment();
        assert_eq!(env.host, "testhost.example.com");
        assert_eq!(env.port, 9090);

        // Cleanup
        env::remove_var("NESTGATE_TEST_HOST");
        env::remove_var("NESTGATE_TEST_PORT");
        Ok(())
    }

    #[test]
    fn test_builder_pattern() -> Result<(), Box<dyn std::error::Error>> {
        let env = TestEnvironmentBuilder::new()
            .host("custom.test.host")
            .port(7777)
            .websocket_port(7778)
            .build();

        assert_eq!(env.host, "custom.test.host");
        assert_eq!(env.port, 7777);
        assert_eq!(env.websocket_port, 7778);
        Ok(())
    }

    #[test]
    fn test_url_generation() -> Result<(), Box<dyn std::error::Error>> {
        let env = TestEnvironmentBuilder::new()
            .host("testhost")
            .port(DEFAULT_API_PORT)
            .build();

        let url = env.get_test_url("api/v1/test");
        assert!(url.contains(&format!("testhost:{}", DEFAULT_API_PORT)));
        assert!(url.contains("api/v1/test"));
        Ok(())
    }

    #[test]
    fn test_environment_variables_export() -> Result<(), Box<dyn std::error::Error>> {
        let env = TestEnvironment::from_environment();
        let vars = env.as_env_vars();

        assert!(vars.contains_key("NESTGATE_TEST_HOST"));
        assert!(vars.contains_key("NESTGATE_TEST_PORT"));
        assert!(vars.contains_key("NESTGATE_TEST_API_URL"));
        Ok(())
    }
}
