// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    unused,
    dead_code,
    deprecated,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::restriction,
    clippy::cargo
)]

//! Canonical Modernization Test
//!
//! This test validates that the canonical modernization is working correctly

use nestgate_core::config::DeploymentEnvironment;
use nestgate_core::config::canonical_primary::NestGateCanonicalConfig;

/// Test that canonical configuration works
#[tokio::test]
async fn test_canonical_config_creation() -> Result<(), Box<dyn std::error::Error>> {
    // Test default configuration creation with explicit type parameters
    let config: NestGateCanonicalConfig = NestGateCanonicalConfig::default();

    // Verify basic structure using correct field names
    assert!(
        !config.system.instance_name.is_empty(),
        "Instance name should not be empty"
    );
    // LogLevel is an enum, not a string - just verify it exists
    // by accessing it (compilation ensures it's valid)
    let _log_level = &config.system.log_level;

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
    let config: NestGateCanonicalConfig = NestGateCanonicalConfig::default();

    // Test that config can be serialized (validates structure)
    let serialized = serde_json::to_string(&config)?;

    // NOTE: "service" field is actually part of the canonical system config, not deprecated
    // It refers to ServiceConfig which is canonical
    // Only check for truly deprecated fields like "extensions"
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
    let config: NestGateCanonicalConfig = NestGateCanonicalConfig::default();

    // Test system configuration with correct field names
    assert!(!config.system.instance_name.is_empty());
    // LogLevel is an enum, not a string - just verify it exists
    let _log_level = &config.system.log_level;
    assert!(!config.system.data_dir.as_os_str().is_empty());
    assert!(!config.system.config_dir.as_os_str().is_empty());

    println!("✅ Configuration validation passes");
    Ok(())
}
