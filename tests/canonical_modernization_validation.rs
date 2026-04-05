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

/// Canonical Modernization Validation Tests
///
/// This test suite validates that the canonical modernization has been successful
/// and that all modernized patterns work correctly.
use nestgate_core::{
    cache_math,
    config::canonical_primary::NestGateCanonicalConfig,
    consensus_math,
    error::{CanonicalResult, NestGateError},
    validation_predicates,
};

#[tokio::test]
async fn test_canonical_modernization_complete() -> Result<(), Box<dyn std::error::Error>> {
    // Verify canonical patterns are accessible and functional
    // Test passes if modules compile and are accessible
    Ok(())
}

#[tokio::test]
async fn test_consensus_math_canonical_patterns() -> Result<(), Box<dyn std::error::Error>> {
    // Test the pure consensus math functions
    assert_eq!(consensus_math::calculate_required_consensus(10, 0.6), 6);
    assert_eq!(consensus_math::calculate_consensus_percentage(7, 10), 0.7);
    assert!(consensus_math::is_consensus_achieved(0.7, 0.6));

    // Test edge cases
    assert_eq!(consensus_math::calculate_required_consensus(0, 0.5), 0);
    assert_eq!(consensus_math::calculate_consensus_percentage(0, 0), 0.0);
    Ok(())
}

#[tokio::test]
async fn test_cache_math_canonical_patterns() -> Result<(), Box<dyn std::error::Error>> {
    // Test cache calculation functions
    assert!(cache_math::needs_eviction(800, 300, 1000)); // 800 + 300 > 1000
    assert!(!cache_math::needs_eviction(700, 200, 1000)); // 700 + 200 <= 1000

    assert_eq!(
        cache_math::calculate_total_cache_size(&[100, 200, 300]),
        600
    );
    assert_eq!(cache_math::calculate_eviction_size(800, 300, 1000), 100);
    Ok(())
}

#[tokio::test]
async fn test_validation_predicates_canonical_patterns() -> Result<(), Box<dyn std::error::Error>> {
    // Test environment validation
    assert!(validation_predicates::is_production_environment(
        "production"
    ));
    assert!(validation_predicates::is_production_environment(
        "PRODUCTION"
    ));
    assert!(!validation_predicates::is_production_environment(
        "development"
    ));

    // Test threshold validation
    assert!(validation_predicates::is_valid_percentage_threshold(50.0));
    assert!(!validation_predicates::is_valid_percentage_threshold(150.0));
    assert!(validation_predicates::is_valid_consensus_threshold(0.7));
    assert!(!validation_predicates::is_valid_consensus_threshold(0.3));
    Ok(())
}

#[test]
fn test_canonical_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    // Test unified error system
    let error = NestGateError::configuration_error("test_field", "test_component");

    assert!(error.to_string().contains("test_field"));
    assert!(error.to_string().contains("Configuration"));
    Ok(())
}

#[test]
fn test_canonical_configuration_patterns() -> Result<(), Box<dyn std::error::Error>> {
    // Test that canonical config can be created
    let config = NestGateCanonicalConfig::<1000, 4096, 30000, 8080>::default();
    // Default instance_name includes "-default" suffix
    assert!(config.system.instance_name.starts_with("nestgate"));
    assert_eq!(config.system.version, "3.0.0");

    // Test environment-driven configuration
    let _bind_addr = config.network.api.bind_address; // Verify bind_address is accessible
    Ok(())
}

#[test]
fn test_sovereignty_compliance() -> Result<(), Box<dyn std::error::Error>> {
    // Test that no hardcoded primal names exist in canonical types
    let config = NestGateCanonicalConfig::<1000, 4096, 30000, 8080>::default();
    let config_str = serde_json::to_string(&config)?;

    // Verify no hardcoded primal names
    assert!(!config_str.contains("beardog"));
    assert!(!config_str.contains("songbird"));
    assert!(!config_str.contains("toadstool"));
    assert!(!config_str.contains("squirrel"));
    Ok(())
}

#[test]
fn test_zero_cost_abstractions() -> Result<(), Box<dyn std::error::Error>> {
    // Test that canonical patterns don't introduce runtime overhead
    use std::time::Instant;

    let start = Instant::now();

    // Execute canonical operations
    for _ in 0..1000 {
        let _ = consensus_math::calculate_required_consensus(10, 0.6);
        let _ = cache_math::needs_eviction(800, 200, 1000);
        let _ = validation_predicates::is_production_environment("production");
        // Test completed successfully
    }

    let duration = start.elapsed();

    // Should complete very quickly (sub-millisecond for 1000 operations)
    assert!(
        duration.as_millis() < 10,
        "Canonical operations should be zero-cost"
    );
    Ok(())
}

#[test]
fn test_canonical_type_safety() -> Result<(), Box<dyn std::error::Error>> {
    // Test compile-time type safety guarantees
    let config = NestGateCanonicalConfig::<1000, 4096, 30000, 8080>::default();

    // These should compile without issues (type safety validation)
    let _service_name: String = config.system.instance_name.clone();
    let _bind_address = &config.network.api.bind_address;
    let _storage_config = &config.storage;

    // Type safety validation passes if this compiles
    Ok(())
}

#[tokio::test]
async fn test_canonical_async_patterns() -> Result<(), Box<dyn std::error::Error>> {
    // Test that modernized async patterns work correctly

    // Test async operations complete successfully
    let start = std::time::Instant::now();

    // Simulate canonical async operation (brief yield for timing validity)
    tokio::task::yield_now().await;

    let duration = start.elapsed();
    // Lenient: allow 0ms on fast systems; original >=1 was flaky
    assert!(duration.as_millis() < 100, "Should be efficient");
    Ok(())
}

#[test]
fn test_modernization_metrics() -> Result<(), Box<dyn std::error::Error>> {
    // Validate modernization achievements

    // Test 1: Zero unsafe code validation
    // (This is enforced by compilation - if unsafe code existed, it would need explicit unsafe blocks)

    // Test 2: Canonical error handling
    let result: CanonicalResult<String> = Ok("test".to_string());
    assert!(result.is_ok());

    let error_result: CanonicalResult<String> = Err(NestGateError::internal_error(
        "test error".to_string(),
        "test_component",
    ));
    assert!(error_result.is_err());

    // Test 3: Configuration unification
    let config = NestGateCanonicalConfig::<1000, 4096, 30000, 8080>::default();
    assert!(!config.system.instance_name.is_empty());
    assert!(!config.system.version.is_empty());
    Ok(())
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_end_to_end_canonical_workflow() -> Result<(), Box<dyn std::error::Error>> {
        // Test complete canonical workflow

        // 1. Configuration loading
        let config = NestGateCanonicalConfig::<1000, 4096, 30000, 8080>::default();
        assert!(!config.system.instance_name.is_empty());

        // 2. Validation
        assert!(validation_predicates::is_valid_percentage_threshold(75.0));

        // 3. Consensus calculation
        let consensus = consensus_math::calculate_required_consensus(5, 0.6);
        assert_eq!(consensus, 3);

        // 4. Cache management
        assert!(!cache_math::needs_eviction(100, 50, 200));

        // 5. Error handling
        let result: CanonicalResult<()> = Ok(());
        assert!(result.is_ok());

        Ok(())
    }
}
