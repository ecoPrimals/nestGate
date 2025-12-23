//! Error Path Coverage Expansion Tests
//!
//! Adding targeted tests for error handling paths to increase coverage
//! from 73.49% toward 90% target.

use nestgate_core::config::environment::{ConfigError, EnvironmentConfig, Port};
use std::str::FromStr;

// ==================== PORT ERROR PATH TESTS ====================

#[test]
fn test_port_zero_returns_error() {
    let result = Port::new(0);
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("0"));
    }
}

#[test]
fn test_port_reserved_range_errors() {
    // Test several reserved ports
    for port in [0, 1, 20, 80, 443, 1023] {
        let result = Port::new(port);
        assert!(result.is_err(), "Port {} should be rejected", port);
    }
}

#[test]
fn test_port_valid_range_succeeds() {
    // Test valid ports
    for port in [1024, 3000, 8080, 9090, 65535] {
        let result = Port::new(port);
        assert!(result.is_ok(), "Port {} should be valid", port);
    }
}

#[test]
fn test_port_from_str_invalid_formats() {
    let invalid_inputs = vec![
        "", " ", "abc", "12.34", "0x1234", "-1", "65536", "100000", "8080 ", " 8080", "80 80",
        "port",
    ];

    for input in invalid_inputs {
        let result = Port::from_str(input);
        assert!(result.is_err(), "Input '{}' should be invalid", input);
    }
}

#[test]
fn test_port_from_str_valid_formats() {
    let valid_inputs = vec![
        ("1024", 1024),
        ("3000", 3000),
        ("8080", 8080),
        ("65535", 65535),
    ];

    for (input, expected) in valid_inputs {
        let result = Port::from_str(input);
        assert!(result.is_ok(), "Input '{}' should be valid", input);
        assert_eq!(result.unwrap().get(), expected);
    }
}

// ==================== CONFIG ERROR PATH TESTS ====================

#[test]
fn test_config_error_display() {
    let error = ConfigError::MissingEnvVar("TEST_VAR".to_string());
    let display = format!("{}", error);
    assert!(display.contains("TEST_VAR"));
    assert!(display.contains("not found"));
}

#[test]
fn test_config_error_invalid_port_display() {
    let error = ConfigError::InvalidPort(80);
    let display = format!("{}", error);
    assert!(display.contains("80"));
}

#[test]
fn test_config_error_invalid_display() {
    let error = ConfigError::Invalid("test message".to_string());
    let display = format!("{}", error);
    assert!(display.contains("test message"));
}

// ==================== ENVIRONMENT CONFIG ERROR TESTS ====================

#[test]
fn test_environment_config_default_is_valid() {
    let config = EnvironmentConfig::default();

    // Verify defaults are sensible
    assert!(config.network.port.get() >= 1024);
    // Note: Port is u16, so it's always <= 65535 (no need to check)
    assert!(!config.network.host.is_empty());
}

#[test]
fn test_environment_config_bind_address() {
    let config = EnvironmentConfig::default();
    let addr = config.bind_address().expect("Should parse bind address");

    // Should return valid socket address
    assert!(addr.port() >= 1024);
}

// ==================== CONCURRENT ACCESS TESTS ====================

#[test]
fn test_port_is_send() {
    fn assert_send<T: Send>() {}
    assert_send::<Port>();
}

#[test]
fn test_port_is_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<Port>();
}

#[test]
fn test_config_is_send() {
    fn assert_send<T: Send>() {}
    assert_send::<EnvironmentConfig>();
}

#[test]
fn test_config_is_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<EnvironmentConfig>();
}

// ==================== EDGE CASE TESTS ====================

#[test]
fn test_port_boundary_values() {
    // Test exact boundaries
    assert!(Port::new(1023).is_err());
    assert!(Port::new(1024).is_ok());
    assert!(Port::new(65535).is_ok());
}

#[test]
fn test_port_equality() {
    let port1 = Port::new(8080).unwrap();
    let port2 = Port::new(8080).unwrap();
    assert_eq!(port1, port2);
}

#[test]
fn test_port_inequality() {
    let port1 = Port::new(8080).unwrap();
    let port2 = Port::new(9090).unwrap();
    assert_ne!(port1, port2);
}

#[test]
fn test_port_clone() {
    let port1 = Port::new(8080).unwrap();
    // Port implements Copy, so we can just copy it directly
    let port2 = port1;
    assert_eq!(port1, port2);
}

#[test]
fn test_port_copy() {
    let port1 = Port::new(8080).unwrap();
    let port2 = port1; // Copy, not move
    assert_eq!(port1, port2); // Both still valid
}

// ==================== SERIALIZATION TESTS ====================

#[test]
fn test_port_serialization() {
    let port = Port::new(8080).unwrap();
    let json = serde_json::to_string(&port).unwrap();
    assert!(json.contains("8080"));
}

#[test]
fn test_port_deserialization() {
    let json = "8080";
    let port: Port = serde_json::from_str(json).unwrap();
    assert_eq!(port.get(), 8080);
}

#[test]
fn test_config_clone() {
    let config1 = EnvironmentConfig::default();
    let config2 = config1.clone();
    assert_eq!(config1.network.port, config2.network.port);
}

// ==================== DEBUG FORMATTING TESTS ====================

#[test]
fn test_port_debug_format() {
    let port = Port::new(8080).unwrap();
    let debug = format!("{:?}", port);
    assert!(debug.contains("Port") || debug.contains("8080"));
}

#[test]
fn test_config_debug_format() {
    let config = EnvironmentConfig::default();
    let debug = format!("{:?}", config);
    assert!(!debug.is_empty());
}

// ==================== INTEGRATION TESTS ====================

#[test]
fn test_full_config_workflow() {
    // Create config
    let config = EnvironmentConfig::default();

    // Clone it
    let config_copy = config.clone();

    // Get bind address
    let addr = config_copy
        .bind_address()
        .expect("Should parse bind address");

    // Verify it's valid
    assert!(addr.port() >= 1024);
}

#[test]
fn test_multiple_configs_independent() {
    let config1 = EnvironmentConfig::default();
    let config2 = EnvironmentConfig::default();

    // They should have same defaults but be independent
    assert_eq!(config1.network.port, config2.network.port);
}

// ==================== STRESS TESTS ====================

#[test]
fn test_many_port_creations() {
    // Create many ports to test performance
    for i in 1024..2024 {
        let port = Port::new(i).unwrap();
        assert_eq!(port.get(), i);
    }
}

#[test]
fn test_many_config_creations() {
    // Create many configs to test cloning
    let mut configs = Vec::new();
    for _ in 0..100 {
        configs.push(EnvironmentConfig::default());
    }
    assert_eq!(configs.len(), 100);
}
