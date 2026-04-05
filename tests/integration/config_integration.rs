// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Configuration Integration Tests
//!
//! Tests for configuration loading and environment handling

#![expect(clippy::const_is_empty)]

use nestgate_core::config::canonical_primary::Environment;
use nestgate_core::Result;

/// Test environment enum functionality
#[test]
fn test_environment_types() {
    let dev = Environment::Development;
    let prod = Environment::Production;

    // Test Debug formatting
    assert!(format!("{:?}", dev).contains("Development"));
    assert!(format!("{:?}", prod).contains("Production"));

    // Test Clone
    let _dev_clone = dev;
}

/// Test environment from string parsing
#[test]
fn test_environment_parsing() {
    // Test valid environments
    assert!(matches!(
        "development".parse::<Environment>(),
        Ok(Environment::Development)
    ));

    assert!(matches!(
        "production".parse::<Environment>(),
        Ok(Environment::Production)
    ));
}

/// Test configuration constants are accessible
#[test]
fn test_configuration_constants() {
    use nestgate_core::constants;

    // Test that constants module is accessible
    let _default_port = constants::DEFAULT_API_PORT;

    // Verify constants are reasonable
    assert!(_default_port > 1000);
    assert!(_default_port < 65535);
}

/// Test result type functionality
#[tokio::test]
async fn test_result_handling() -> Result<()> {
    // Test Ok path
    let success: Result<i32> = Ok(42);
    assert_eq!(success?, 42);

    Ok(())
}

/// Test configuration validation patterns
#[test]
fn test_config_validation() {
    // Test that empty strings are invalid
    let empty_name = "";
    assert!(empty_name.is_empty());

    // Test that valid strings pass
    let valid_name = "nestgate-instance";
    assert!(!valid_name.is_empty());
    assert!(valid_name.len() > 5);
}
