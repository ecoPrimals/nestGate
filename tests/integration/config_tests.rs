//! **CONFIG MODULE INTEGRATION TESTS**
//!
//! Tests for the configuration management system

use crate::common::*;
use nestgate_core::config::*;
use std::collections::HashMap;

/// Test basic configuration loading
#[test]
async fn test_config_loading() -> Result<(), Box<dyn std::error::Error>> {
    // Test creating a default configuration
    let config = Config::default();
    assert!(config.is_valid());
    
    // Test configuration validation
    assert!(config.validate().is_ok());
    Ok(())
}

/// Test configuration with custom values
#[test]
async fn test_config_custom_values() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::default();
    
    // Test setting custom values
    config.set_string("database.host", nestgate_core::constants::TEST_HOSTNAME);
    config.set_number("database.port", 5432);
    config.set_boolean("debug.enabled", true);
    
    // Test retrieving values
    assert_eq!(config.get_string("database.host")?, nestgate_core::constants::TEST_HOSTNAME);
    assert_eq!(config.get_number("database.port")?, 5432);
    assert_eq!(config.get_boolean("debug.enabled")?, true);
    Ok(())
}

/// Test configuration serialization
#[test]
async fn test_config_serialization() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::default();
    config.set_string("app.name", "NestGate");
    config.set_number("app.version", 1);
    
    // Test JSON serialization
    let json_str = config.to_json()?;
    assert!(json_str.contains("NestGate"));
    
    // Test deserialization
    let restored_config = Config::from_json(&json_str)?;
    assert_eq!(restored_config.get_string("app.name")?, "NestGate");
    assert_eq!(restored_config.get_number("app.version")?, 1);
    Ok(())
}

/// Test configuration validation rules
#[test]
async fn test_config_validation() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::default();
    
    // Test valid configuration using environment-driven values
    let test_env = crate::common::get_test_environment();
    config.set_string("server.bind_address", &test_env.host);
    config.set_number("server.port", test_env.port as i64);
    assert!(config.validate().is_ok());
    
    // Test invalid port
    config.set_number("server.port", 70000); // Invalid port
    let validation_result = config.validate();
    assert!(validation_result.is_err());
    
    // Test missing required field
    config.remove("server.bind_address");
    let validation_result2 = config.validate();
    assert!(validation_result2.is_err());
    Ok(())
}

/// Test configuration environment variable support
#[test]
async fn test_config_environment_variables() -> Result<(), Box<dyn std::error::Error>> {
    // Set test environment variable
    std::env::set_var("NESTGATE_TEST_VALUE", "test123");
    
    let config = Config::from_env()?;
    
    // Test that environment variables are loaded
    if let Ok(value) = config.get_string("test_value") {
        assert_eq!(value, "test123");
    Ok(())
    }
    
    // Clean up
    std::env::remove_var("NESTGATE_TEST_VALUE");
    Ok(())
}

/// Test configuration file loading
#[tokio::test]
async fn test_config_file_loading() -> Result<(), Box<dyn std::error::Error>> {
    init_test_logging();
    
    // Create a temporary config file content
    let config_content = r#"
    {
        "server": {
            "host": nestgate_core::constants::TEST_HOSTNAME,
            "port": 3000
        },
        "database": {
            "url": "sqlite:///tmp/test.db"
    Ok(())
        }
    Ok(())
    }
    "#;
    
    // Test loading from string
    let config = Config::from_json_str(config_content)?;
    assert_eq!(config.get_string("server.host")?, nestgate_core::constants::TEST_HOSTNAME);
    assert_eq!(config.get_number("server.port")?, 3000);
    assert!(config.get_string("database.url")?.contains("test.db"));
    Ok(())
}

/// Test configuration merging
#[test]
async fn test_config_merging() -> Result<(), Box<dyn std::error::Error>> {
    let mut base_config = Config::default();
    base_config.set_string("app.name", "NestGate");
    base_config.set_number("app.version", 1);
    base_config.set_boolean("features.auth", true);
    
    let mut override_config = Config::default();
    override_config.set_number("app.version", 2); // Override version
    override_config.set_boolean("features.logging", true); // Add new feature
    
    // Test merging configurations
    let merged = base_config.merge(&override_config);
    
    assert_eq!(merged.get_string("app.name")?, "NestGate"); // From base
    assert_eq!(merged.get_number("app.version")?, 2); // Overridden
    assert_eq!(merged.get_boolean("features.auth")?, true); // From base
    assert_eq!(merged.get_boolean("features.logging")?, true); // From override
    Ok(())
}

/// Test configuration sections
#[test]
async fn test_config_sections() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::default();
    
    // Test creating nested sections
    config.set_string("database.primary.host", "db1.example.com");
    config.set_string("database.primary.user", "admin");
    config.set_string("database.replica.host", "db2.example.com");
    config.set_string("database.replica.user", "readonly");
    
    // Test getting section
    let db_section = config.get_section("database")?;
    assert!(db_section.contains_key("primary"));
    assert!(db_section.contains_key("replica"));
    
    // Test section validation
    let primary_section = config.get_section("database.primary")?;
    assert_eq!(primary_section.get("host")?, "db1.example.com");
    assert_eq!(primary_section.get("user")?, "admin");
    Ok(())
}

/// Test configuration type coercion
#[test]
async fn test_config_type_coercion() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::default();
    
    // Test setting string that looks like a number
    config.set_string("port", "8080");
    
    // Test type coercion
    assert_eq!(config.get_number_from_string("port")?, 8080);
    
    // Test setting string that looks like a boolean
    config.set_string("debug", "true");
    assert_eq!(config.get_boolean_from_string("debug")?, true);
    
    config.set_string("verbose", "false");
    assert_eq!(config.get_boolean_from_string("verbose")?, false);
    Ok(())
}

/// Test configuration error handling
#[test]
async fn test_config_error_handling() -> Result<(), Box<dyn std::error::Error>> {
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
    Ok(())
}

/// Test configuration watching and updates
#[tokio::test]
async fn test_config_watching() -> Result<(), Box<dyn std::error::Error>> {
    init_test_logging();
    
    let mut config = Config::default();
    config.set_string("watched.value", "initial");
    
    // Test configuration change notification
    let mut watcher = config.create_watcher().await?;
    
    // Make a change
    config.set_string("watched.value", "updated");
    
    // Check if watcher detects change
    let change_detected = watcher.wait_for_change(std::time::Duration::from_millis(100)).await;
    assert!(change_detected.is_ok());
    Ok(())
}

/// Test configuration defaults and overrides
#[test]
fn test_config_defaults_and_overrides() -> Result<(), Box<dyn std::error::Error>> {
    // Test default configuration values
    let config = Config::with_defaults();
    
    // Should have reasonable defaults
    assert!(config.get_number("server.port").unwrap_or(8080) > 0);
    assert!(config.get_string("server.host").unwrap_or(nestgate_core::constants::TEST_HOSTNAME.to_string()).len() > 0);
    
    // Test override behavior
    let mut config_with_overrides = config.clone();
    config_with_overrides.set_number("server.port", 9000);
    
    assert_eq!(config_with_overrides.get_number("server.port")?, 9000);
    assert_ne!(config.get_number("server.port").unwrap_or(8080), 9000);
    Ok(())
}

/// Comprehensive configuration integration test
#[tokio::test]
async fn test_comprehensive_config_workflow() -> Result<(), Box<dyn std::error::Error>> {
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
    let json_str = final_config.to_json()?;
    let restored_config = Config::from_json(&json_str)?;
    
    // Step 6: Verify all values are preserved
    assert_eq!(restored_config.get_string("app.name")?, "NestGate");
    assert_eq!(restored_config.get_number("app.version")?, 1);
    assert_eq!(restored_config.get_boolean("features.storage")?, true);
    assert_eq!(restored_config.get_string("database.host")?, "prod-db.example.com");
    assert_eq!(restored_config.get_boolean("debug.enabled")?, false);
    assert_eq!(restored_config.get_number("performance.max_connections")?, 1000);
    
    // Step 7: Test configuration sections work correctly
    let features_section = restored_config.get_section("features")?;
    assert_eq!(features_section.len(), 2); // storage and networking
    
    // Step 8: Test that validation still passes
    assert!(restored_config.validate().is_ok());
    Ok(())
} 