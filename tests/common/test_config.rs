//! Centralized Test Configuration System
//!
//! This module provides a unified configuration system for tests that eliminates
//! hardcoded values and provides proper environment variable fallbacks.

use std::collections::HashMap;
use std::env;
use std::net::IpAddr;
use std::str::FromStr;
use std::time::Duration;

/// Centralized test configuration
#[derive(Debug, Clone)]
pub struct TestConfig {
    pub network: NetworkConfig,
    pub services: ServiceConfig,
    pub timeouts: TimeoutConfig,
    pub mock: MockConfig,
    pub biomeos: BioMeOSConfig,
    pub zfs: ZfsTestConfig,
}

/// Network configuration with environment variable fallbacks
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub bind_host: IpAddr,
    pub api_port: u16,
    pub websocket_port: u16,
    pub coordination_port: u16,
    pub events_port: u16,
    pub toadstool_port: u16,
    pub squirrel_port: u16,
    pub beardog_port: u16,
    pub songbird_port: u16,
}

/// Service endpoint configuration
#[derive(Debug, Clone)]
pub struct ServiceConfig {
    pub api_base_url: String,
    pub websocket_url: String,
    pub coordination_url: String,
    pub events_url: String,
    pub toadstool_url: String,
    pub squirrel_url: String,
    pub beardog_url: String,
    pub songbird_url: String,
    pub universal_storage_url: String,
}

/// Timeout configuration
#[derive(Debug, Clone)]
pub struct TimeoutConfig {
    pub test_short: Duration,
    pub test_medium: Duration,
    pub test_long: Duration,
    pub api_request: Duration,
    pub websocket_connect: Duration,
    pub zfs_operation: Duration,
    pub benchmark: Duration,
}

/// Mock configuration
#[derive(Debug, Clone)]
pub struct MockConfig {
    pub enable_realistic_delays: bool,
    pub default_error_rate: f64,
    pub max_concurrent_operations: usize,
    pub cleanup_interval: Duration,
    pub use_mock_zfs: bool,
}

/// BioMeOS integration configuration
#[derive(Debug, Clone)]
pub struct BioMeOSConfig {
    pub enable_integration: bool,
    pub coordination_endpoint: String,
    pub max_agents: usize,
    pub agent_timeout: Duration,
    pub federation_enabled: bool,
}

/// ZFS test configuration
#[derive(Debug, Clone)]
pub struct ZfsTestConfig {
    pub use_real_zfs: bool,
    pub default_pool: String,
    pub test_timeout: Duration,
    pub enable_snapshots: bool,
    pub enable_compression: bool,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            network: NetworkConfig::default(),
            services: ServiceConfig::default(),
            timeouts: TimeoutConfig::default(),
            mock: MockConfig::default(),
            biomeos: BioMeOSConfig::default(),
            zfs: ZfsTestConfig::default(),
        }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            bind_host: get_env_or_default("NESTGATE_BIND_HOST", "127.0.0.1")
                .parse()
                .unwrap_or(IpAddr::from_str("127.0.0.1").unwrap()),
            api_port: get_env_or_default("NESTGATE_API_PORT", "8080").parse().unwrap_or(8080),
            websocket_port: get_env_or_default("NESTGATE_WS_PORT", "8081").parse().unwrap_or(8081),
            coordination_port: get_env_or_default("NESTGATE_COORDINATION_PORT", "8082").parse().unwrap_or(8082),
            events_port: get_env_or_default("NESTGATE_EVENTS_PORT", "8083").parse().unwrap_or(8083),
            toadstool_port: get_env_or_default("TOADSTOOL_PORT", "9000").parse().unwrap_or(9000),
            squirrel_port: get_env_or_default("SQUIRREL_PORT", "9001").parse().unwrap_or(9001),
            beardog_port: get_env_or_default("BEARDOG_PORT", "9002").parse().unwrap_or(9002),
            songbird_port: get_env_or_default("SONGBIRD_PORT", "9003").parse().unwrap_or(9003),
        }
    }
}

impl Default for ServiceConfig {
    fn default() -> Self {
        let network = NetworkConfig::default();
        
        Self {
            api_base_url: get_env_or_default(
                "NESTGATE_API_URL",
                &format!("http://{}:{}", network.bind_host, network.api_port)
            ),
            websocket_url: get_env_or_default(
                "NESTGATE_WS_URL",
                &format!("ws://{}:{}", network.bind_host, network.websocket_port)
            ),
            coordination_url: get_env_or_default(
                "NESTGATE_COORDINATION_URL",
                &format!("http://{}:{}", network.bind_host, network.coordination_port)
            ),
            events_url: get_env_or_default(
                "NESTGATE_EVENTS_URL",
                &format!("ws://{}:{}", network.bind_host, network.events_port)
            ),
            toadstool_url: get_env_or_default(
                "TOADSTOOL_URL",
                &format!("http://{}:{}", network.bind_host, network.toadstool_port)
            ),
            squirrel_url: get_env_or_default(
                "SQUIRREL_URL",
                &format!("http://{}:{}", network.bind_host, network.squirrel_port)
            ),
            beardog_url: get_env_or_default(
                "BEARDOG_URL",
                &format!("http://{}:{}", network.bind_host, network.beardog_port)
            ),
            songbird_url: get_env_or_default(
                "SONGBIRD_URL",
                &format!("http://{}:{}", network.bind_host, network.songbird_port)
            ),
            universal_storage_url: get_env_or_default(
                "UNIVERSAL_STORAGE_URL",
                &format!("http://{}:8084", network.bind_host)
            ),
        }
    }
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            test_short: Duration::from_millis(
                get_env_or_default("TEST_SHORT_TIMEOUT_MS", "1000").parse().unwrap_or(1000)
            ),
            test_medium: Duration::from_millis(
                get_env_or_default("TEST_MEDIUM_TIMEOUT_MS", "5000").parse().unwrap_or(5000)
            ),
            test_long: Duration::from_millis(
                get_env_or_default("TEST_LONG_TIMEOUT_MS", "30000").parse().unwrap_or(30000)
            ),
            api_request: Duration::from_millis(
                get_env_or_default("API_REQUEST_TIMEOUT_MS", "5000").parse().unwrap_or(5000)
            ),
            websocket_connect: Duration::from_millis(
                get_env_or_default("WS_CONNECT_TIMEOUT_MS", "3000").parse().unwrap_or(3000)
            ),
            zfs_operation: Duration::from_millis(
                get_env_or_default("ZFS_OPERATION_TIMEOUT_MS", "10000").parse().unwrap_or(10000)
            ),
            benchmark: Duration::from_millis(
                get_env_or_default("BENCHMARK_TIMEOUT_MS", "30000").parse().unwrap_or(30000)
            ),
        }
    }
}

impl Default for MockConfig {
    fn default() -> Self {
        Self {
            enable_realistic_delays: get_env_or_default("MOCK_REALISTIC_DELAYS", "true")
                .parse()
                .unwrap_or(true),
            default_error_rate: get_env_or_default("MOCK_ERROR_RATE", "0.01")
                .parse()
                .unwrap_or(0.01),
            max_concurrent_operations: get_env_or_default("MOCK_MAX_CONCURRENT", "100")
                .parse()
                .unwrap_or(100),
            cleanup_interval: Duration::from_secs(
                get_env_or_default("MOCK_CLEANUP_INTERVAL_SEC", "60").parse().unwrap_or(60)
            ),
            use_mock_zfs: get_env_or_default("USE_MOCK_ZFS", "false")
                .parse()
                .unwrap_or(false),
        }
    }
}

impl Default for BioMeOSConfig {
    fn default() -> Self {
        let network = NetworkConfig::default();
        
        Self {
            enable_integration: get_env_or_default("BIOMEOS_INTEGRATION", "true")
                .parse()
                .unwrap_or(true),
            coordination_endpoint: get_env_or_default(
                "BIOMEOS_COORDINATION_ENDPOINT",
                &format!("http://{}:{}", network.bind_host, network.coordination_port)
            ),
            max_agents: get_env_or_default("BIOMEOS_MAX_AGENTS", "10")
                .parse()
                .unwrap_or(10),
            agent_timeout: Duration::from_secs(
                get_env_or_default("BIOMEOS_AGENT_TIMEOUT_SEC", "30").parse().unwrap_or(30)
            ),
            federation_enabled: get_env_or_default("BIOMEOS_FEDERATION", "false")
                .parse()
                .unwrap_or(false),
        }
    }
}

impl Default for ZfsTestConfig {
    fn default() -> Self {
        Self {
            use_real_zfs: get_env_or_default("USE_REAL_ZFS", "false")
                .parse()
                .unwrap_or(false),
            default_pool: get_env_or_default("ZFS_DEFAULT_POOL", "nestpool").to_string(),
            test_timeout: Duration::from_secs(
                get_env_or_default("ZFS_TEST_TIMEOUT_SEC", "30").parse().unwrap_or(30)
            ),
            enable_snapshots: get_env_or_default("ZFS_ENABLE_SNAPSHOTS", "true")
                .parse()
                .unwrap_or(true),
            enable_compression: get_env_or_default("ZFS_ENABLE_COMPRESSION", "true")
                .parse()
                .unwrap_or(true),
        }
    }
}

/// Helper function to get environment variable or default value
fn get_env_or_default(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

/// Create service endpoint map with environment variable fallbacks
pub fn create_service_endpoints() -> HashMap<String, String> {
    let config = TestConfig::default();
    let mut endpoints = HashMap::new();
    
    endpoints.insert("nestgate".to_string(), config.services.api_base_url);
    endpoints.insert("coordination".to_string(), config.services.coordination_url);
    endpoints.insert("events".to_string(), config.services.events_url);
    endpoints.insert("toadstool".to_string(), config.services.toadstool_url);
    endpoints.insert("squirrel".to_string(), config.services.squirrel_url);
    endpoints.insert("beardog".to_string(), config.services.beardog_url);
    endpoints.insert("songbird".to_string(), config.services.songbird_url);
    endpoints.insert("universal-storage".to_string(), config.services.universal_storage_url);
    
    endpoints
}

/// Create BioMeOS integration configuration
pub fn create_biomeos_integration_config() -> BioMeOSIntegrationConfig {
    let config = TestConfig::default();
    
    BioMeOSIntegrationConfig {
        enable_integration: config.biomeos.enable_integration,
        coordination_endpoint: config.biomeos.coordination_endpoint,
        max_agents: config.biomeos.max_agents,
        agent_timeout: config.biomeos.agent_timeout,
        federation_enabled: config.biomeos.federation_enabled,
        service_endpoints: create_service_endpoints(),
    }
}

/// BioMeOS integration configuration struct
#[derive(Debug, Clone)]
pub struct BioMeOSIntegrationConfig {
    pub enable_integration: bool,
    pub coordination_endpoint: String,
    pub max_agents: usize,
    pub agent_timeout: Duration,
    pub federation_enabled: bool,
    pub service_endpoints: HashMap<String, String>,
}

/// Create test-specific environment variables
pub fn setup_test_environment() -> Result<(), Box<dyn std::error::Error>> {
    // Set test-specific environment variables if not already set
    let test_vars = [
        ("NESTGATE_BIND_HOST", "127.0.0.1"),
        ("NESTGATE_API_PORT", "8080"),
        ("USE_MOCK_ZFS", "true"),
        ("MOCK_REALISTIC_DELAYS", "false"),
        ("TEST_SHORT_TIMEOUT_MS", "1000"),
        ("TEST_MEDIUM_TIMEOUT_MS", "5000"),
        ("TEST_LONG_TIMEOUT_MS", "30000"),
        ("BIOMEOS_INTEGRATION", "true"),
        ("BIOMEOS_FEDERATION", "false"),
    ];
    
    for (key, value) in test_vars.iter() {
        if env::var(key).is_err() {
            env::set_var(key, value);
        }
    }
    
    Ok(())
}

/// Cleanup test environment
pub fn cleanup_test_environment() {
    // Remove test-specific environment variables
    let test_vars = [
        "NESTGATE_BIND_HOST",
        "NESTGATE_API_PORT",
        "USE_MOCK_ZFS",
        "MOCK_REALISTIC_DELAYS",
        "TEST_SHORT_TIMEOUT_MS",
        "TEST_MEDIUM_TIMEOUT_MS",
        "TEST_LONG_TIMEOUT_MS",
        "BIOMEOS_INTEGRATION",
        "BIOMEOS_FEDERATION",
    ];
    
    for var in test_vars.iter() {
        env::remove_var(var);
    }
}

/// Create a test configuration with custom overrides
pub fn create_test_config_with_overrides(
    network_overrides: Option<NetworkConfig>,
    timeout_overrides: Option<TimeoutConfig>,
    mock_overrides: Option<MockConfig>,
) -> TestConfig {
    let mut config = TestConfig::default();
    
    if let Some(network) = network_overrides {
        config.network = network;
    }
    
    if let Some(timeouts) = timeout_overrides {
        config.timeouts = timeouts;
    }
    
    if let Some(mock) = mock_overrides {
        config.mock = mock;
    }
    
    config
}

/// Global test configuration instance
static mut GLOBAL_TEST_CONFIG: Option<TestConfig> = None;
static CONFIG_INIT: std::sync::Once = std::sync::Once::new();

/// Get or create the global test configuration
pub fn get_global_test_config() -> &'static TestConfig {
    unsafe {
        CONFIG_INIT.call_once(|| {
            setup_test_environment().expect("Failed to setup test environment");
            GLOBAL_TEST_CONFIG = Some(TestConfig::default());
        });
        GLOBAL_TEST_CONFIG.as_ref().unwrap()
    }
}

/// Utility functions for common test scenarios
pub mod utils {
    use super::*;
    
    /// Create a mock service endpoint for testing
    pub fn create_mock_service_endpoint(service_name: &str) -> String {
        let config = get_global_test_config();
        match service_name {
            "nestgate" => config.services.api_base_url.clone(),
            "coordination" => config.services.coordination_url.clone(),
            "toadstool" => config.services.toadstool_url.clone(),
            "squirrel" => config.services.squirrel_url.clone(),
            "beardog" => config.services.beardog_url.clone(),
            "songbird" => config.services.songbird_url.clone(),
            _ => format!("http://{}:8080", config.network.bind_host),
        }
    }
    
    /// Get appropriate timeout for test type
    pub fn get_test_timeout(test_type: &str) -> Duration {
        let config = get_global_test_config();
        match test_type {
            "short" => config.timeouts.test_short,
            "medium" => config.timeouts.test_medium,
            "long" => config.timeouts.test_long,
            "api" => config.timeouts.api_request,
            "websocket" => config.timeouts.websocket_connect,
            "zfs" => config.timeouts.zfs_operation,
            "benchmark" => config.timeouts.benchmark,
            _ => config.timeouts.test_medium,
        }
    }
    
    /// Check if mock mode is enabled
    pub fn is_mock_mode() -> bool {
        get_global_test_config().mock.use_mock_zfs
    }
    
    /// Get mock error rate
    pub fn get_mock_error_rate() -> f64 {
        get_global_test_config().mock.default_error_rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config_creation() {
        let config = TestConfig::default();
        
        assert_eq!(config.network.bind_host, IpAddr::from_str("127.0.0.1").unwrap());
        assert_eq!(config.network.api_port, 8080);
        assert!(config.timeouts.test_short > Duration::from_millis(0));
        assert!(config.mock.max_concurrent_operations > 0);
    }
    
    #[test]
    fn test_environment_variable_override() {
        env::set_var("NESTGATE_API_PORT", "9999");
        
        let config = TestConfig::default();
        assert_eq!(config.network.api_port, 9999);
        
        env::remove_var("NESTGATE_API_PORT");
    }
    
    #[test]
    fn test_service_endpoints_creation() {
        let endpoints = create_service_endpoints();
        
        assert!(endpoints.contains_key("nestgate"));
        assert!(endpoints.contains_key("coordination"));
        assert!(endpoints.contains_key("toadstool"));
        assert!(endpoints.contains_key("squirrel"));
        
        // All endpoints should be properly formatted URLs
        for (name, url) in endpoints.iter() {
            assert!(url.starts_with("http://") || url.starts_with("ws://"), 
                   "Invalid URL format for {}: {}", name, url);
        }
    }
    
    #[test]
    fn test_biomeos_integration_config() {
        let config = create_biomeos_integration_config();
        
        assert!(config.max_agents > 0);
        assert!(!config.coordination_endpoint.is_empty());
        assert!(!config.service_endpoints.is_empty());
    }
    
    #[test]
    fn test_test_environment_setup() {
        setup_test_environment().unwrap();
        
        assert!(env::var("NESTGATE_BIND_HOST").is_ok());
        assert!(env::var("USE_MOCK_ZFS").is_ok());
        
        cleanup_test_environment();
    }
} 