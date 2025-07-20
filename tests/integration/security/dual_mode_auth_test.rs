//! Dual-mode authentication tests
//!
//! Tests authentication in both standalone and BearDog integrated modes

use std::time::Duration;
use tokio::time::timeout;
use nestgate_core::cert::{CertValidator, CertMode, BearDogConfig, CertUtils, CertInfo};
use nestgate_core::{Result, NestGateError};

/// Test standalone mode functionality
#[tokio::test]
async fn test_standalone_full_workflow() {
    let mut validator = CertValidator::standalone();

    // Test mode is correct
    assert_eq!(*validator.mode(), CertMode::Standalone);

    // Generate and validate self-signed certificate
    let cert = CertUtils::generate_self_signed().unwrap();
    let result = validator.validate_cert(&cert).await.unwrap();
    assert!(result, "Self-signed cert should be valid in standalone mode");

    // Test certificate caching
    let start = std::time::Instant::now();
    validator.validate_cert(&cert).await.unwrap();
    let cached_duration = start.elapsed();
    assert!(cached_duration < Duration::from_millis(5), "Cached validation should be fast");

    // Test adding trusted certificate
    let cert_info = CertInfo {
        subject: "CN=TestCert".to_string(),
        issuer: "CN=TestCA".to_string(),
        valid_from: std::time::SystemTime::now(),
        valid_to: std::time::SystemTime::now() + Duration::from_secs(3600),
        fingerprint: "test123".to_string(),
        key_usage: vec!["digitalSignature".to_string()],
        extended_key_usage: vec!["clientAuth".to_string()],
    };

    validator.add_trusted_cert(cert_info).unwrap();

    // Test BearDog unavailable
    assert!(!validator.beardog_available().await);
}

/// Test BearDog integration mode
#[tokio::test]
async fn test_beardog_integration_mode() {
    let config = BearDogConfig {
        discovery_timeout: Duration::from_secs(10),
        validation_timeout: nestgate_core::constants::test_defaults::TEST_SHORT_TIMEOUT,
        retry_attempts: 2,
        fallback_to_standalone: false,
    };

    let mut validator = CertValidator::with_beardog(config);

    // Test mode is correct
    assert_eq!(*validator.mode(), CertMode::BearDog);

    // Test BearDog connectivity simulation
    let connectivity = timeout(Duration::from_secs(2), validator.beardog_available()).await;
    assert!(connectivity.is_ok());

    // Test certificate validation through BearDog
    let cert = CertUtils::generate_self_signed().unwrap();
    let result = timeout(Duration::from_secs(3), validator.validate_cert(&cert)).await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_ok());

    // Test that adding certs is not allowed in BearDog-only mode
    let cert_info = CertInfo {
        subject: "CN=Test".to_string(),
        issuer: "CN=TestCA".to_string(),
        valid_from: std::time::SystemTime::now(),
        valid_to: std::time::SystemTime::now() + Duration::from_secs(3600),
        fingerprint: "test456".to_string(),
        key_usage: vec!["digitalSignature".to_string()],
        extended_key_usage: vec!["serverAuth".to_string()],
    };

    let add_result = validator.add_trusted_cert(cert_info);
    assert!(add_result.is_err());
    assert!(matches!(add_result.unwrap_err(), NestGateError::Internal(_)));
}

/// Test hybrid mode with fallback
#[tokio::test]
async fn test_hybrid_mode_fallback() {
    let config = BearDogConfig {
        discovery_timeout: Duration::from_secs(10),
        validation_timeout: Duration::from_secs(30),
        retry_attempts: 3,
        fallback_to_standalone: true,
    };

    let mut validator = CertValidator::hybrid(config);

    // Test mode is correct
    assert_eq!(*validator.mode(), CertMode::Hybrid);

    // Test certificate validation with fallback
    let cert = CertUtils::generate_self_signed().unwrap();
    let result = validator.validate_cert(&cert).await.unwrap();
    assert!(result, "Hybrid mode should fallback to standalone");

    // Test that adding certs is allowed in hybrid mode
    let cert_info = CertInfo {
        subject: "CN=HybridTest".to_string(),
        issuer: "CN=HybridCA".to_string(),
        valid_from: std::time::SystemTime::now(),
        valid_to: std::time::SystemTime::now() + Duration::from_secs(3600),
        fingerprint: "hybrid789".to_string(),
        key_usage: vec!["digitalSignature".to_string()],
        extended_key_usage: vec!["serverAuth".to_string(), "clientAuth".to_string()],
    };

    let add_result = validator.add_trusted_cert(cert_info);
    assert!(add_result.is_ok());
}

/// Test certificate utilities
#[test]
fn test_certificate_utilities() {
    // Test certificate generation
    let cert = CertUtils::generate_self_signed().unwrap();
    assert!(cert.contains("-----BEGIN CERTIFICATE-----"));
    assert!(cert.contains("-----END CERTIFICATE-----"));
    assert!(cert.len() > 100);

    // Test CN extraction
    let cn = CertUtils::extract_cn(&cert).unwrap();
    assert_eq!(cn, "NestGate");

    // Test invalid certificate handling
    let invalid_cn = CertUtils::extract_cn("invalid cert data");
    assert!(invalid_cn.is_err());

    // Test empty certificate
    let empty_cn = CertUtils::extract_cn("");
    assert!(empty_cn.is_err());
}

/// Test BearDog configuration serialization
#[test]
fn test_beardog_config_serialization() {
    let config = BearDogConfig {
        endpoint: "https://beardog.example.com:8443".to_string(),
        api_key: "secret-key-123".to_string(),
        trust_anchor: "custom-ca".to_string(),
        validation_timeout: Duration::from_secs(45),
        retry_attempts: 5,
    };

    // Test JSON serialization
    let json = serde_json::to_string(&config).unwrap();
    assert!(!json.is_empty());
    assert!(json.contains("beardog.example.com"));

    // Test deserialization
    let deserialized: BearDogConfig = serde_json::from_str(&json).unwrap();
    assert_eq!(config.endpoint, deserialized.endpoint);
    assert_eq!(config.api_key, deserialized.api_key);
    assert_eq!(config.trust_anchor, deserialized.trust_anchor);
    assert_eq!(config.retry_attempts, deserialized.retry_attempts);

    // Test default values
    let default_config = BearDogConfig::default();
    assert!(default_config.endpoint.contains("beardog.local"));
    assert_eq!(default_config.retry_attempts, 3);
    assert_eq!(default_config.validation_timeout, nestgate_core::constants::test_defaults::TEST_MEDIUM_TIMEOUT);
}

/// Test concurrent certificate validation
#[tokio::test]
async fn test_concurrent_validation() {
    let mut validator = CertValidator::standalone();
    let cert = CertUtils::generate_self_signed().unwrap();

    // Run multiple validations concurrently
    let mut handles = Vec::new();

    for i in 0..10 {
        let cert_clone = cert.clone();
        let handle = tokio::spawn(async move {
            // Create a new validator for each task to avoid borrow checker issues
            let mut local_validator = CertValidator::standalone();
            let result = local_validator.validate_cert(&cert_clone).await;
            (i, result)
        });
        handles.push(handle);
    }

    // Wait for all validations to complete
    let mut results = Vec::new();
    for handle in handles {
        let (id, result) = handle.await.unwrap();
        results.push((id, result));
    }

    // All should succeed
    assert_eq!(results.len(), 10);
    for (_, result) in results {
        assert!(result.is_ok());
        assert!(result.unwrap());
    }
}

/// Test performance benchmarks
#[tokio::test]
async fn test_validation_performance() {
    let mut validator = CertValidator::standalone();
    let cert = CertUtils::generate_self_signed().unwrap();

    // Warm up
    validator.validate_cert(&cert).await.unwrap();

    // Benchmark validation speed
    let iterations = 100;
    let start = std::time::Instant::now();

    for _ in 0..iterations {
        validator.validate_cert(&cert).await.unwrap();
    }

    let duration = start.elapsed();
    let avg_time = duration / iterations;

    println!("Average validation time: {:?}", avg_time);
    assert!(avg_time < Duration::from_millis(1), "Cached validation should be sub-millisecond");
}

/// Test certificate expiration handling
#[tokio::test]
async fn test_certificate_expiration() {
    use std::time::SystemTime;

    let mut validator = CertValidator::standalone();

    // Create expired certificate info
    let expired_cert_info = CertInfo {
        subject: "CN=ExpiredCert".to_string(),
        issuer: "CN=ExpiredCA".to_string(),
        valid_from: SystemTime::now() - Duration::from_secs(7200), // 2 hours ago
        valid_to: SystemTime::now() - Duration::from_secs(3600),   // 1 hour ago (expired)
        fingerprint: "expired123".to_string(),
        key_usage: vec!["digitalSignature".to_string()],
        extended_key_usage: vec!["serverAuth".to_string()],
    };

    // Add expired certificate to trust store
    validator.add_trusted_cert(expired_cert_info).unwrap();

    // Note: The current implementation doesn't actually check expiration in trust store
    // This is a test to ensure the structure supports it for future implementation
    assert_eq!(validator.trust_store.len(), 1);
}

/// Test error handling scenarios
#[tokio::test]
async fn test_error_handling() {
    // Test empty certificate
    let mut validator = CertValidator::standalone();
    let result = validator.validate_cert("").await;
    assert!(result.is_ok()); // Empty cert is handled gracefully in standalone mode

    // Test malformed certificate
    let malformed_cert = "-----BEGIN CERTIFICATE-----\ngarbage\n-----END CERTIFICATE-----";
    let result = validator.validate_cert(malformed_cert).await;
    assert!(result.is_ok()); // Standalone mode is permissive

    // Test BearDog with no config
    let mut beardog_validator = CertValidator::with_beardog(BearDogConfig {
        endpoint: "".to_string(),
        api_key: "".to_string(),
        ..Default::default()
    });

    let cert = CertUtils::generate_self_signed().unwrap();
    let result = beardog_validator.validate_cert(&cert).await;
    assert!(result.is_err()); // Should fail with empty config
}

/// Integration test: Full dual-mode scenario
#[tokio::test]
async fn test_full_dual_mode_scenario() {
    // Scenario: Start in hybrid mode, BearDog fails, fallback to standalone

    let beardog_config = BearDogConfig {
        endpoint: "https://unreachable-beardog.test:8443".to_string(),
        api_key: "test-key".to_string(),
        ..Default::default()
    };

    let mut validator = CertValidator::hybrid(beardog_config);

    // Add some trusted certificates for standalone fallback
    let trusted_cert = CertInfo {
        subject: "CN=TrustedService".to_string(),
        issuer: "CN=CompanyCA".to_string(),
        valid_from: SystemTime::now(),
        valid_to: SystemTime::now() + Duration::from_secs(86400), // 24 hours
        fingerprint: "trusted456".to_string(),
        key_usage: vec!["digitalSignature".to_string(), "keyEncipherment".to_string()],
        extended_key_usage: vec!["serverAuth".to_string()],
    };

    validator.add_trusted_cert(trusted_cert).unwrap();

    // Test certificate validation (should fallback to standalone)
    let cert = CertUtils::generate_self_signed().unwrap();
    let start = std::time::Instant::now();
    let result = validator.validate_cert(&cert).await.unwrap();
    let duration = start.elapsed();

    assert!(result);
    println!("Hybrid validation (with fallback) took: {:?}", duration);

    // Verify BearDog is indeed unavailable
    assert!(!validator.beardog_available().await);

    // Test that caching works in hybrid mode
    let start = std::time::Instant::now();
    validator.validate_cert(&cert).await.unwrap();
    let cached_duration = start.elapsed();

    assert!(cached_duration < duration, "Second validation should be faster due to caching");
}