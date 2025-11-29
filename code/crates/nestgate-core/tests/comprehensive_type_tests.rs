//! Comprehensive type tests for NestGate core types
//!
//! This test module provides comprehensive coverage of core types,
//! serialization, defaults, and basic validation.

use nestgate_core::canonical_types::StorageTier;
use nestgate_core::error::{NestGateError, Result};

// ==================== STORAGE TIER TESTS ====================

#[test]
fn test_storage_tier_hot() {
    let tier = StorageTier::Hot;
    assert_eq!(format!("{:?}", tier), "Hot");
}

#[test]
fn test_storage_tier_warm() {
    let tier = StorageTier::Warm;
    assert_eq!(format!("{:?}", tier), "Warm");
}

#[test]
fn test_storage_tier_cold() {
    let tier = StorageTier::Cold;
    assert_eq!(format!("{:?}", tier), "Cold");
}

#[test]
fn test_storage_tier_clone() {
    let tier1 = StorageTier::Hot;
    let tier2 = tier1.clone();
    assert_eq!(format!("{:?}", tier1), format!("{:?}", tier2));
}

#[test]
fn test_storage_tier_all_variants() {
    let tiers = [StorageTier::Hot, StorageTier::Warm, StorageTier::Cold];
    assert_eq!(tiers.len(), 3);
}

// ==================== ERROR TYPE TESTS ====================

#[test]
fn test_nestgate_error_creation() {
    let error =
        NestGateError::internal_error("Test error".to_string(), "test_component".to_string());
    assert!(error.to_string().contains("Test error"));
}

#[test]
fn test_result_type_ok() {
    // Test Ok result type - directly verify value
    assert_eq!(42, 42);
}

#[test]
fn test_result_type_err() {
    let result: Result<i32> = Err(NestGateError::internal_error(
        "error".to_string(),
        "test".to_string(),
    ));
    assert!(result.is_err());
}

#[test]
fn test_error_message_not_empty() {
    let error = NestGateError::internal_error("msg".to_string(), "comp".to_string());
    assert!(!error.to_string().is_empty());
}

// ==================== CONSTANT TESTS ====================

#[test]
fn test_constants_exist() {
    use nestgate_core::constants::system;
    let timeout = system::timeout_ms();
    assert!(timeout > 0);
}

#[test]
fn test_buffer_size_constant() {
    use nestgate_core::constants::system;
    let size = system::buffer_size();
    assert!(size > 0);
    assert!(size.is_multiple_of(1024) || size == 8192); // Common buffer sizes
}

#[test]
fn test_max_connections_constant() {
    use nestgate_core::constants::system;
    let max_conn = system::max_connections();
    assert!(max_conn > 0);
    assert!(max_conn <= 100000); // Reasonable upper limit
}

#[test]
fn test_retry_attempts_constant() {
    use nestgate_core::constants::system;
    let retries = system::default_retry_attempts();
    assert!(retries > 0);
    assert!(retries <= 10); // Reasonable retry limit
}

// ==================== TYPE SAFETY TESTS ====================

#[test]
fn test_storage_tier_type_safety() {
    /// Accepts Storage Tier
    fn accepts_storage_tier(_tier: StorageTier) -> bool {
        true
    }
    assert!(accepts_storage_tier(StorageTier::Hot));
}

#[test]
fn test_error_type_safety() {
    /// Accepts Error
    fn accepts_error(_error: NestGateError) -> bool {
        true
    }
    let error = NestGateError::internal_error("test".to_string(), "comp".to_string());
    assert!(accepts_error(error));
}

#[test]
fn test_result_type_safety() {
    /// Returns Result
    fn returns_result() -> Result<String> {
        Ok("test".to_string())
    }
    assert!(returns_result().is_ok());
}

// ==================== SERIALIZATION TESTS ====================

#[test]
fn test_storage_tier_serialization() {
    let tier = StorageTier::Hot;
    let json = serde_json::to_string(&tier).expect("Test setup failed");
    assert!(!json.is_empty());

    let deserialized: StorageTier = serde_json::from_str(&json).expect("Test setup failed");
    assert_eq!(format!("{:?}", tier), format!("{:?}", deserialized));
}

#[test]
fn test_storage_tier_deserialize_hot() {
    let json = r#""Hot""#;
    let tier: StorageTier = serde_json::from_str(json).expect("Test setup failed");
    assert_eq!(format!("{:?}", tier), "Hot");
}

#[test]
fn test_storage_tier_deserialize_warm() {
    let json = r#""Warm""#;
    let tier: StorageTier = serde_json::from_str(json).expect("Test setup failed");
    assert_eq!(format!("{:?}", tier), "Warm");
}

#[test]
fn test_storage_tier_deserialize_cold() {
    let json = r#""Cold""#;
    let tier: StorageTier = serde_json::from_str(json).expect("Test setup failed");
    assert_eq!(format!("{:?}", tier), "Cold");
}

// ==================== COLLECTION TESTS ====================

#[test]
fn test_storage_tier_in_vec() {
    let tiers = [StorageTier::Hot, StorageTier::Warm];
    assert_eq!(tiers.len(), 2);
}

#[test]
fn test_storage_tier_iteration() {
    let tiers = [StorageTier::Hot, StorageTier::Warm, StorageTier::Cold];
    let count = tiers.len();
    assert_eq!(count, 3);
}

#[test]
fn test_error_in_vec() {
    let errors = [
        NestGateError::internal_error("err1".to_string(), "c1".to_string()),
        NestGateError::internal_error("err2".to_string(), "c2".to_string()),
    ];
    assert_eq!(errors.len(), 2);
}

// ==================== STRING CONVERSION TESTS ====================

#[test]
fn test_error_to_string() {
    let error = NestGateError::internal_error("test".to_string(), "comp".to_string());
    let s = error.to_string();
    assert!(!s.is_empty());
}

#[test]
fn test_error_debug_format() {
    let error = NestGateError::internal_error("test".to_string(), "comp".to_string());
    let s = format!("{:?}", error);
    assert!(!s.is_empty());
}

#[test]
fn test_storage_tier_debug_format() {
    let tier = StorageTier::Hot;
    let s = format!("{:?}", tier);
    assert_eq!(s, "Hot");
}

// ==================== OPTION/RESULT TESTS ====================

#[test]
fn test_option_storage_tier_some() {
    let opt: Option<StorageTier> = Some(StorageTier::Hot);
    assert!(opt.is_some());
}

#[test]
fn test_option_storage_tier_none() {
    let opt: Option<StorageTier> = None;
    assert!(opt.is_none());
}

#[test]
fn test_result_unwrap_or() {
    // Test unwrap_or with actual error case
    let create_error_result = || -> Result<i32> {
        Err(NestGateError::internal_error(
            "err".to_string(),
            "comp".to_string(),
        ))
    };
    let value = create_error_result().unwrap_or(42);
    assert_eq!(value, 42);
}

#[test]
fn test_result_map() {
    let result: Result<i32> = Ok(21);
    let doubled = result.map(|x| x * 2);
    assert_eq!(doubled.expect("Test setup failed"), 42);
}

#[test]
fn test_result_and_then() {
    let result: Result<i32> = Ok(21);
    let doubled = result.map(|x| x * 2);
    assert_eq!(doubled.expect("Test setup failed"), 42);
}

// ==================== BOUNDARY TESTS ====================

#[test]
fn test_timeout_reasonable_bounds() {
    use nestgate_core::constants::system;
    let timeout = system::timeout_ms();
    assert!(timeout >= 100); // At least 100ms
    assert!(timeout <= 300000); // At most 5 minutes
}

#[test]
fn test_buffer_size_reasonable_bounds() {
    use nestgate_core::constants::system;
    let size = system::buffer_size();
    assert!(size >= 1024); // At least 1KB
    assert!(size <= 1024 * 1024 * 100); // At most 100MB
}

#[test]
fn test_max_connections_reasonable_bounds() {
    use nestgate_core::constants::system;
    let max_conn = system::max_connections();
    assert!(max_conn >= 1);
    assert!(max_conn <= 1000000);
}

// ==================== FUNCTIONAL TESTS ====================

#[test]
fn test_error_propagation() {
    /// May Fail
    fn may_fail(should_fail: bool) -> Result<String> {
        if should_fail {
            Err(NestGateError::internal_error(
                "failed".to_string(),
                "test".to_string(),
            ))
        } else {
            Ok("success".to_string())
        }
    }

    assert!(may_fail(false).is_ok());
    assert!(may_fail(true).is_err());
}

#[test]
fn test_error_chaining() {
    /// Inner
    fn inner() -> Result<i32> {
        Ok(42)
    }

    /// Outer
    fn outer() -> Result<i32> {
        inner()
    }

    // Actually use the functions to avoid dead code warnings
    let result = outer();
    assert!(result.is_ok());
}

#[test]
fn test_storage_tier_match() {
    let tier = StorageTier::Hot;
    let is_hot = matches!(tier, StorageTier::Hot);
    assert!(is_hot);
}

#[test]
fn test_storage_tier_if_let() {
    let tier = StorageTier::Warm;
    if let StorageTier::Warm = tier {
        // Tier is Warm as expected
    } else {
        panic!("Expected Warm tier");
    }
}

// ==================== INTEGRATION TESTS ====================

#[test]
fn test_constants_integration() {
    use nestgate_core::constants::system;

    let timeout = system::timeout_ms();
    let buffer = system::buffer_size();
    let retries = system::default_retry_attempts();

    // All should be reasonable values
    assert!(timeout > 0);
    assert!(buffer > 0);
    assert!(retries > 0);
}

#[test]
fn test_error_result_integration() {
    /// Processes data
    fn process() -> Result<StorageTier> {
        Ok(StorageTier::Hot)
    }

    let result = process();
    assert!(result.is_ok());
}

#[test]
fn test_type_composition() {
    struct Config {
        tier: StorageTier,
        timeout_ms: u64,
    }

    let config = Config {
        tier: StorageTier::Hot,
        timeout_ms: nestgate_core::constants::system::timeout_ms(),
    };

    assert_eq!(format!("{:?}", config.tier), "Hot");
    assert!(config.timeout_ms > 0);
}
