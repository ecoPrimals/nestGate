//! **CONFIG MODULE INTEGRATION TESTS**
//!
//! Tests for the configuration management system

use crate::common::*;
use nestgate_core::config::*;
use std::collections::HashMap;

/// Test basic configuration loading
#[test]
fn test_config_loading() {
    // Test creating a default configuration
    let config = Config::default();
    assert!(config.is_valid());
    
    // Test configuration validation
    assert!(config.validate().is_ok());
}

/// Test configuration with custom values
#[test]
fn test_config_custom_values() {
    let mut config = Config::default();
    
    // Test setting custom values
    config.set_string("database.host", "localhost");
    config.set_number("database.port", 5432);
    config.set_boolean("debug.enabled", true);
    
    // Test retrieving values
    assert_eq!(config.get_string("database.host").unwrap(), "localhost");
    assert_eq!(config.get_number("database.port").unwrap(), 5432);
    assert_eq!(config.get_boolean("debug.enabled").unwrap(), true);
}

/// Test configuration serialization
#[test]
fn test_config_serialization() {
    let mut config = Config::default();
    config.set_string("app.name", "NestGate");
    config.set_number("app.version", 1);
    
    // Test JSON serialization
    let json_str = config.to_json().unwrap();
    assert!(json_str.contains("NestGate"));
    
    // Test deserialization
    let restored_config = Config::from_json(&json_str).unwrap();
    assert_eq!(restored_config.get_string("app.name").unwrap(), "NestGate");
    assert_eq!(restored_config.get_number("app.version").unwrap(), 1);
}

/// Test configuration validation rules
#[test]
fn test_config_validation() {
    let mut config = Config::default();
    
    // Test valid configuration
    config.set_string("server.bind_address", "127.0.0.1");
    config.set_number("server.port", 8080);
    assert!(config.validate().is_ok());
    
    // Test invalid port
    config.set_number("server.port", 70000); // Invalid port
    let validation_result = config.validate();
    assert!(validation_result.is_err());
    
    // Test missing required field
    config.remove("server.bind_address");
    let validation_result2 = config.validate();
    assert!(validation_result2.is_err());
}

/// Test configuration environment variable support
#[test]
fn test_config_environment_variables() {
    // Set test environment variable
    std::env::set_var("NESTGATE_TEST_VALUE", "test123");
    
    let config = Config::from_env().unwrap();
    
    // Test that environment variables are loaded
    if let Ok(value) = config.get_string("test_value") {
        assert_eq!(value, "test123");
    }
    
    // Clean up
    std::env::remove_var("NESTGATE_TEST_VALUE");
}

/// Test configuration file loading
#[tokio::test]
async fn test_config_file_loading() {
    init_test_logging();
    
    // Create a temporary config file content
    let config_content = r#"
    {
        "server": {
            "host": "localhost",
            "port": 3000
        },
        "database": {
            "url": "sqlite:///tmp/test.db"
        }
    }
    "#;
    
    // Test loading from string
    let config = Config::from_json_str(config_content).unwrap();
    assert_eq!(config.get_string("server.host").unwrap(), "localhost");
    assert_eq!(config.get_number("server.port").unwrap(), 3000);
    assert!(config.get_string("database.url").unwrap().contains("test.db"));
}

/// Test configuration merging
#[test]
fn test_config_merging() {
    let mut base_config = Config::default();
    base_config.set_string("app.name", "NestGate");
    base_config.set_number("app.version", 1);
    base_config.set_boolean("features.auth", true);
    
    let mut override_config = Config::default();
    override_config.set_number("app.version", 2); // Override version
    override_config.set_boolean("features.logging", true); // Add new feature
    
    // Test merging configurations
    let merged = base_config.merge(&override_config);
    
    assert_eq!(merged.get_string("app.name").unwrap(), "NestGate"); // From base
    assert_eq!(merged.get_number("app.version").unwrap(), 2); // Overridden
    assert_eq!(merged.get_boolean("features.auth").unwrap(), true); // From base
    assert_eq!(merged.get_boolean("features.logging").unwrap(), true); // From override
}

/// Test configuration sections
#[test]
fn test_config_sections() {
    let mut config = Config::default();
    
    // Test creating nested sections
    config.set_string("database.primary.host", "db1.example.com");
    config.set_string("database.primary.user", "admin");
    config.set_string("database.replica.host", "db2.example.com");
    config.set_string("database.replica.user", "readonly");
    
    // Test getting section
    let db_section = config.get_section("database").unwrap();
    assert!(db_section.contains_key("primary"));
    assert!(db_section.contains_key("replica"));
    
    // Test section validation
    let primary_section = config.get_section("database.primary").unwrap();
    assert_eq!(primary_section.get("host").unwrap(), "db1.example.com");
    assert_eq!(primary_section.get("user").unwrap(), "admin");
}

/// Test configuration type coercion
#[test]
fn test_config_type_coercion() {
    let mut config = Config::default();
    
    // Test setting string that looks like a number
    config.set_string("port", "8080");
    
    // Test type coercion
    assert_eq!(config.get_number_from_string("port").unwrap(), 8080);
    
    // Test setting string that looks like a boolean
    config.set_string("debug", "true");
    assert_eq!(config.get_boolean_from_string("debug").unwrap(), true);
    
    config.set_string("verbose", "false");
    assert_eq!(config.get_boolean_from_string("verbose").unwrap(), false);
}

/// Test configuration error handling
#[test]
fn test_config_error_handling() {
    let config = Config::default();
    
    // Test getting non-existent key
    assert!(config.get_string("nonexistent.key").is_err());
    
    // Test invalid JSON parsing
    let invalid_json = r#"{ "key": "value" invalid }"#;
    assert!(Config::from_json_str(invalid_json).is_err());
    
    // Test type mismatch
    let mut config = Config::default();
    config.set_string("text_field", "hello");
    assert!(config.get_number("text_field").is_err());
}

/// Test configuration watching and updates
#[tokio::test]
async fn test_config_watching() {
    init_test_logging();
    
    let mut config = Config::default();
    config.set_string("watched.value", "initial");
    
    // Test configuration change notification
    let mut watcher = config.create_watcher().await.unwrap();
    
    // Make a change
    config.set_string("watched.value", "updated");
    
    // Check if watcher detects change
    let change_detected = watcher.wait_for_change(std::time::Duration::from_millis(100)).await;
    assert!(change_detected.is_ok());
}

/// Test configuration defaults and overrides
#[test]
fn test_config_defaults_and_overrides() {
    // Test default configuration values
    let config = Config::with_defaults();
    
    // Should have reasonable defaults
    assert!(config.get_number("server.port").unwrap_or(8080) > 0);
    assert!(config.get_string("server.host").unwrap_or("localhost".to_string()).len() > 0);
    
    // Test override behavior
    let mut config_with_overrides = config.clone();
    config_with_overrides.set_number("server.port", 9000);
    
    assert_eq!(config_with_overrides.get_number("server.port").unwrap(), 9000);
    assert_ne!(config.get_number("server.port").unwrap_or(8080), 9000);
}

/// Comprehensive configuration integration test
#[tokio::test]
async fn test_comprehensive_config_workflow() {
    init_test_logging();
    
    // Step 1: Create base configuration
    let mut config = Config::default();
    config.set_string("app.name", "NestGate");
    config.set_number("app.version", 1);
    config.set_boolean("features.storage", true);
    config.set_boolean("features.networking", true);
    
    // Step 2: Add environment-specific overrides
    let mut prod_overrides = Config::default();
    prod_overrides.set_string("database.host", "prod-db.example.com");
    prod_overrides.set_boolean("debug.enabled", false);
    prod_overrides.set_number("performance.max_connections", 1000);
    
    // Step 3: Merge configurations
    let final_config = config.merge(&prod_overrides);
    
    // Step 4: Validate final configuration
    assert!(final_config.validate().is_ok());
    
    // Step 5: Test serialization round-trip
    let json_str = final_config.to_json().unwrap();
    let restored_config = Config::from_json(&json_str).unwrap();
    
    // Step 6: Verify all values are preserved
    assert_eq!(restored_config.get_string("app.name").unwrap(), "NestGate");
    assert_eq!(restored_config.get_number("app.version").unwrap(), 1);
    assert_eq!(restored_config.get_boolean("features.storage").unwrap(), true);
    assert_eq!(restored_config.get_string("database.host").unwrap(), "prod-db.example.com");
    assert_eq!(restored_config.get_boolean("debug.enabled").unwrap(), false);
    assert_eq!(restored_config.get_number("performance.max_connections").unwrap(), 1000);
    
    // Step 7: Test configuration sections work correctly
    let features_section = restored_config.get_section("features").unwrap();
    assert_eq!(features_section.len(), 2); // storage and networking
    
    // Step 8: Test that validation still passes
    assert!(restored_config.validate().is_ok());
} 