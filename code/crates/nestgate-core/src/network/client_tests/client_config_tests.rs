//! Client Config Tests
//!
//! Auto-generated from smart refactoring of client_tests.rs
//! Sections: CLIENT CONFIG TESTS, CLIENT CONFIG COMPREHENSIVE TESTS

use super::super::client::*;
use std::time::Duration;

// ==================== CLIENT CONFIG TESTS ====================
#[test]
fn test_client_config_default() {
    let config = ClientConfig::<30000>::default();

    assert_eq!(config.timeout.as_duration(), Duration::from_millis(30000));
    assert_eq!(config.max_connections, 100);
    assert_eq!(config.max_connections_per_host, 10);
    assert!(config.enable_compression);
    assert!(config.follow_redirects);
    assert_eq!(config.max_redirects, 5);
}

#[test]
fn test_client_config_custom_timeout() {
    let config = ClientConfig::<60000>::default();
    assert_eq!(config.timeout.as_duration(), Duration::from_millis(60000));
}

#[test]
fn test_client_config_serialization() {
    let config = ClientConfig::<30000>::default();
    let json = serde_json::to_string(&config);
    assert!(json.is_ok());
}

// ==================== CLIENT CONFIG COMPREHENSIVE TESTS ====================
#[test]
fn test_client_config_custom_values() {
    let mut config = ClientConfig::<30000>::default();
    config.max_connections = 200;
    config.max_connections_per_host = 20;
    config.enable_compression = false;
    config.follow_redirects = false;
    config.max_redirects = 10;

    assert_eq!(config.max_connections, 200);
    assert_eq!(config.max_connections_per_host, 20);
    assert!(!config.enable_compression);
    assert!(!config.follow_redirects);
    assert_eq!(config.max_redirects, 10);
}

#[test]
fn test_client_config_user_agent() {
    let config = ClientConfig::<30000>::default();
    assert!(config.user_agent.contains("NestGate"));
}

#[test]
fn test_client_config_timeout_variations() {
    let config_5s = ClientConfig::<5000>::default();
    let config_30s = ClientConfig::<30000>::default();
    let config_60s = ClientConfig::<60000>::default();

    assert_eq!(config_5s.timeout.as_duration(), Duration::from_secs(5));
    assert_eq!(config_30s.timeout.as_duration(), Duration::from_secs(30));
    assert_eq!(config_60s.timeout.as_duration(), Duration::from_secs(60));
}
