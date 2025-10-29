//! Canonical Modernization Test
//!
//! This test validates that the canonical modernization is working correctly

use nestgate_core::config::canonical_master::NestGateCanonicalConfig;
use nestgate_core::config::DeploymentEnvironment;

/// Test that canonical configuration works
#[tokio::test]
async fn test_canonical_config_creation() -> Result<(), Box<dyn std::error::Error>> {
    // Test default configuration creation
    let config = NestGateNestGateCanonicalConfig::default();

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
    let dev_config = nestgate_core::config::create_development_config();
    assert!(matches!(
        dev_config.system.environment,
        DeploymentEnvironment::Development
    ));

    // Test production environment
    let prod_config = nestgate_core::config::create_production_config();
    assert!(matches!(
        prod_config.system.environment,
        DeploymentEnvironment::Production
    ));

    println!("✅ Environment-driven configuration works");
    Ok(())
}

/// Test that deprecated fields are not present
#[tokio::test]
async fn test_no_deprecated_fields() -> Result<(), Box<dyn std::error::Error>> {
    let config = NestGateNestGateCanonicalConfig::default();

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
    let config = NestGateNestGateCanonicalConfig::default();

    // Test system configuration with correct field names
    assert!(!config.system.instance_name.is_empty());
    assert!(!config.system.log_level.is_empty());
    assert!(!config.system.data_dir.as_os_str().is_empty());
    assert!(!config.system.config_dir.as_os_str().is_empty());

    println!("✅ Configuration validation passes");
    Ok(())
}
