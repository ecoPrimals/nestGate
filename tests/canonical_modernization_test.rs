//! Canonical Modernization Test
//!
//! This test validates that the canonical modernization is working correctly

use nestgate_core::config::defaults::Environment;
use nestgate_core::config::unified::NestGateUnifiedConfig;

/// Test that canonical configuration works
#[tokio::test]
async fn test_canonical_config_creation() -> Result<(), Box<dyn std::error::Error>> {
    // Test default configuration creation
    let config = NestGateCanonicalUnifiedConfig::default();

    // Verify basic structure using correct field names
    assert!(
        !config.system.instance_name.is_empty(),
        "Instance name should not be empty"
    );
    assert!(
        !config.system.log_level.is_empty(),
        "Log level should not be empty"
    );

    println!("✅ Canonical configuration creation works");
    Ok(())
}

/// Test environment-specific configuration
#[tokio::test]
async fn test_environment_driven_config() -> Result<(), Box<dyn std::error::Error>> {
    // Test development environment
    let dev_config =
        nestgate_core::config::unified::create_config_for_environment(Environment::Development);
    assert!(matches!(dev_config.environment, Environment::Development));

    // Test production environment
    let prod_config =
        nestgate_core::config::unified::create_config_for_environment(Environment::Production);
    assert!(matches!(prod_config.environment, Environment::Production));

    println!("✅ Environment-driven configuration works");
    Ok(())
}

/// Test that deprecated fields are not present
#[tokio::test]
async fn test_no_deprecated_fields() -> Result<(), Box<dyn std::error::Error>> {
    let config = NestGateCanonicalUnifiedConfig::default();

    // Test that config can be serialized (validates structure)
    let serialized = serde_json::to_string(&config)?;

    // Test that serialized config doesn't contain deprecated field names
    assert!(
        !serialized.contains("\"service\":"),
        "Should not contain deprecated 'service' field"
    );
    assert!(
        !serialized.contains("\"extensions\":"),
        "Should not contain deprecated 'extensions' field"
    );

    println!("✅ No deprecated fields present");
    Ok(())
}

/// Test basic configuration validation
#[tokio::test]
async fn test_config_validation() -> Result<(), Box<dyn std::error::Error>> {
    let config = NestGateCanonicalUnifiedConfig::default();

    // Test system configuration with correct field names
    assert!(!config.system.instance_name.is_empty());
    assert!(!config.system.log_level.is_empty());
    assert!(!config.system.data_dir.as_os_str().is_empty());
    assert!(!config.system.config_dir.as_os_str().is_empty());

    println!("✅ Configuration validation passes");
    Ok(())
}
