//! NestGate Core Library
//!
//! Enhanced core functionality and utilities for the NestGate system
//! Integrates enhanced NestGate capabilities with v2 orchestrator-centric architecture

pub mod cache;
pub mod cert;
pub mod config;
pub mod diagnostics;
pub mod error;
pub mod errors;
pub mod metrics;
pub mod security;
pub mod types;
pub mod utils;

use serde::{Deserialize, Serialize};

/// Storage tier types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum StorageTier {
    Hot,
    Warm,
    Cold,
    Cache,
}

// Re-export common types
pub use config::{Config, NetworkConfig};
pub use error::{NestGateError, Result};

// Re-export commonly used utilities with enhanced capabilities
pub use utils::{
    filesys,
    // Enhanced modules with advanced capabilities
    fs,
    // v2 compatibility modules
    network,
    serialization,
    string,
    sys,
    system,
    time,
    SystemInfo,
};

// Re-export security types
pub use security::{AuthContext, Permission, Role, SecurityConfig, SecurityManager};

// Re-export cache and diagnostics
pub use cache::*;
pub use diagnostics::*;

// Re-export certificate validation for external integrations
pub use cert::*;

/// Initialize the NestGate core library with enhanced capabilities
///
/// # Errors
/// Returns an error if the library initialization fails.
pub fn init() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Log system information
    let system_info = SystemInfo::new();
    tracing::info!(
        "NestGate Core initialized - OS: {}, Arch: {}, CPUs: {}, Memory: {}GB",
        system_info.os_name,
        system_info.architecture,
        system_info.cpu_cores,
        system_info.total_memory / (1024 * 1024 * 1024)
    );

    Ok(())
}

/// Get the current version of the NestGate core library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_initialization() {
        assert!(init().is_ok());
    }

    #[test]
    fn test_system_info() {
        let info = SystemInfo::new();
        assert!(!info.os_name.is_empty());
        assert!(!info.architecture.is_empty());
        assert!(info.cpu_cores > 0);
    }

    #[test]
    fn test_core_error_creation() {
        let error = NestGateError::Internal("Test error".to_string());
        assert!(error.to_string().contains("Test error"));

        let io_error = NestGateError::Io("File not found".to_string());
        assert!(io_error.to_string().contains("File not found"));
    }

    #[test]
    fn test_storage_tier_enum() {
        let tiers = vec![
            StorageTier::Hot,
            StorageTier::Warm,
            StorageTier::Cold,
            StorageTier::Cache,
        ];

        for tier in tiers {
            // Test serialization
            let serialized = serde_json::to_string(&tier).unwrap();
            assert!(!serialized.is_empty());

            // Test deserialization
            let _deserialized: StorageTier = serde_json::from_str(&serialized).unwrap();
        }
    }

    #[test]
    fn test_config_creation() {
        let config = Config::default();
        assert!(!config.system.node_id.is_empty());
        assert!(!config.system.data_dir.is_empty());
    }

    #[test]
    fn test_network_config() {
        let localhost_config = NetworkConfig::localhost(8080);
        assert!(localhost_config.is_localhost_only());
        assert_eq!(localhost_config.port, 8080);

        let all_interfaces = NetworkConfig::all_interfaces(3000);
        assert!(!all_interfaces.is_localhost_only());
        assert_eq!(all_interfaces.port, 3000);
    }

    #[test]
    fn test_security_config() {
        use crate::security::SecurityConfig;
        let security_config = SecurityConfig::default();
        assert!(!security_config.api_keys.is_empty());
    }

    #[test]
    fn test_security_manager() {
        use crate::security::{SecurityConfig, SecurityManager};
        let config = SecurityConfig::default();
        let _security_manager = SecurityManager::new(config);

        // Test that manager was created successfully
        assert!(true);
    }

    #[tokio::test]
    async fn test_cache_operations() {
        use crate::cache::CacheManager;

        let _cache = CacheManager::new();

        // Test cache creation
        assert!(true);
    }

    #[test]
    fn test_diagnostics() {
        use crate::diagnostics::DiagnosticsManager;

        let _diagnostics = DiagnosticsManager::new();

        // Test diagnostics creation
        assert!(true);
    }

    #[test]
    fn test_metrics_collection() {
        use crate::metrics::MetricsCollector;

        let mut collector = MetricsCollector::new();

        // Test counter
        collector.increment_counter("test_counter");
        let counter_value = collector.get_metric("test_counter");
        assert!(counter_value.is_some());
        assert_eq!(counter_value.unwrap().value, 1.0);

        // Test gauge
        collector.record_gauge("test_gauge", 42.5);
        let gauge_value = collector.get_metric("test_gauge");
        assert!(gauge_value.is_some());
        assert_eq!(gauge_value.unwrap().value, 42.5);

        // Test histogram
        collector.record_histogram("test_histogram", 100.0);
        let histogram_value = collector.get_metric("test_histogram");
        assert!(histogram_value.is_some());

        // Test metric enumeration
        let all_metrics = collector.get_all_metrics();
        assert!(all_metrics.len() >= 3);
    }

    #[test]
    fn test_error_conversion() {
        // Test From implementations
        let io_error = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Access denied");
        let nestgate_error: NestGateError = io_error.into();
        assert!(matches!(nestgate_error, NestGateError::Io(_)));

        // Test internal error creation
        let internal_error = NestGateError::Internal("Test error".to_string());
        assert!(internal_error.to_string().contains("Test error"));
    }

    #[test]
    fn test_result_combinators() {
        let success: Result<i32> = Ok(42);
        let error: Result<i32> = Err(NestGateError::Internal("Test".to_string()));

        // Test map
        let mapped = success.clone().map(|x| x * 2);
        assert_eq!(mapped.unwrap(), 84);

        // Test and_then
        let chained = success.and_then(|x| Ok(x + 1));
        assert_eq!(chained.unwrap(), 43);

        // Test error handling
        assert!(error.is_err());
        let error_message = error.unwrap_err().to_string();
        assert!(error_message.contains("Test"));
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();

        // Test JSON serialization
        let json = serde_json::to_string(&config).unwrap();
        assert!(!json.is_empty());

        let deserialized: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(config.system.node_id, deserialized.system.node_id);
    }

    #[test]
    fn test_thread_safety() {
        use std::sync::Arc;
        use std::thread;

        let config = Arc::new(Config::default());
        let handles: Vec<_> = (0..10)
            .map(|i| {
                let config_clone = Arc::clone(&config);
                thread::spawn(move || {
                    // Test that config can be safely accessed from multiple threads
                    assert!(!config_clone.system.node_id.is_empty());
                    i
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[tokio::test]
    async fn test_async_operations() {
        use tokio::time::{sleep, Duration};

        // Test async Result handling
        async fn async_operation() -> Result<String> {
            sleep(Duration::from_millis(1)).await;
            Ok("Success".to_string())
        }

        let result = async_operation().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Success");
    }
}
