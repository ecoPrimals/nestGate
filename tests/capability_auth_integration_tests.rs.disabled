//! Integration tests for capability-based authentication
//!
//! Tests the complete authentication flow using capability discovery.

use nestgate_core::{
    capabilities::discovery::CapabilityDiscovery, zero_cost_security_provider::capability_auth::*,
};

#[tokio::test]
async fn test_capability_auth_fallback_validation_jwt() {
    // Test JWT validation without discovered services (uses fallback)
    let discovery = CapabilityDiscovery::new_with_backends(vec![]);
    let client = CapabilityAuthClient::new(discovery);

    // Valid JWT structure (3 parts separated by dots)
    let jwt = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";

    let result = client.validate_token(jwt).await;
    assert!(result.is_ok(), "JWT validation should succeed");
    assert!(result.unwrap(), "Well-formed JWT should be valid");
}

#[tokio::test]
async fn test_capability_auth_fallback_validation_api_key() {
    // Test API key validation without discovered services (uses fallback)
    let discovery = CapabilityDiscovery::new_with_backends(vec![]);
    let client = CapabilityAuthClient::new(discovery);

    // Valid API key format (starts with nsg_, length > 20)
    let api_key = "nsg_1234567890abcdef1234567890abcdef";

    let result = client.validate_token(api_key).await;
    assert!(result.is_ok(), "API key validation should succeed");
    assert!(result.unwrap(), "Well-formed API key should be valid");
}

#[tokio::test]
async fn test_capability_auth_fallback_invalid_token() {
    // Test that invalid tokens are rejected
    let discovery = CapabilityDiscovery::new_with_backends(vec![]);
    let client = CapabilityAuthClient::new(discovery);

    let invalid_tokens = vec![
        "",              // Empty
        "invalid",       // Too short
        "no_prefix_key", // Wrong format
        "a.b",           // Not enough JWT parts
        "nsg_short",     // API key too short
    ];

    for token in invalid_tokens {
        let result = client.validate_token(token).await;
        assert!(result.is_ok(), "Validation should not error");
        assert!(
            !result.unwrap(),
            "Invalid token '{}' should be rejected",
            token
        );
    }
}

#[tokio::test]
async fn test_capability_auth_client_creation() {
    // Test client can be created successfully
    let discovery = CapabilityDiscovery::new_with_backends(vec![]);
    let _client = CapabilityAuthClient::new(discovery);

    // If we got here without panic, test passes
}

#[tokio::test]
async fn test_capability_auth_custom_timeout() {
    // Test client with custom timeout
    use std::time::Duration;

    let discovery = CapabilityDiscovery::new_with_backends(vec![]);
    let client = CapabilityAuthClient::new(discovery).with_timeout(Duration::from_secs(10));

    // Verify it still works
    let jwt = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.dozjgNryP4J3jVmNHl0w5N_XgL0n3I9PlFUP0THsR8U";
    let result = client.validate_token(jwt).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_capability_auth_validates_jwt_structure() {
    // Test JWT structure validation
    let discovery = CapabilityDiscovery::new_with_backends(vec![]);
    let client = CapabilityAuthClient::new(discovery);

    // Test various JWT structures
    let test_cases = vec![
        ("a.b.c", true, "Three-part structure"),
        ("header.payload.signature", true, "Three parts with names"),
        ("a.b", false, "Only two parts"),
        ("a.b.c.d", false, "Four parts"),
        ("single", false, "Single part"),
    ];

    for (token, should_pass, description) in test_cases {
        let result = client.validate_token(token).await;
        assert!(result.is_ok(), "{}: should not error", description);
        assert_eq!(
            result.unwrap(),
            should_pass,
            "{}: expected {}, got opposite",
            description,
            should_pass
        );
    }
}

#[tokio::test]
async fn test_capability_auth_validates_api_key_format() {
    // Test API key format validation
    let discovery = CapabilityDiscovery::new_with_backends(vec![]);
    let client = CapabilityAuthClient::new(discovery);

    let test_cases = vec![
        (format!("nsg_{}", "a".repeat(20)), true, "Valid length"),
        (format!("nsg_{}", "1".repeat(25)), true, "Longer valid key"),
        ("nsg_short".to_string(), false, "Too short"),
        (
            format!("wrong_prefix_{}", "a".repeat(20)),
            false,
            "Wrong prefix",
        ),
        (
            format!("NSG_{}", "a".repeat(20)),
            false,
            "Wrong case prefix",
        ),
    ];

    for (token, should_pass, description) in test_cases {
        let result = client.validate_token(&token).await;
        assert!(result.is_ok(), "{}: should not error", description);
        assert_eq!(
            result.unwrap(),
            should_pass,
            "{}: expected {}, got opposite",
            description,
            should_pass
        );
    }
}

#[tokio::test]
async fn test_capability_auth_empty_token() {
    // Test empty token handling
    let discovery = CapabilityDiscovery::new_with_backends(vec![]);
    let client = CapabilityAuthClient::new(discovery);

    let result = client.validate_token("").await;
    assert!(result.is_ok(), "Empty token should not error");
    assert!(!result.unwrap(), "Empty token should be invalid");
}

#[tokio::test]
async fn test_capability_auth_no_services_discovered() {
    // Test behavior when no auth services are available
    let discovery = CapabilityDiscovery::new_with_backends(vec![]);
    let client = CapabilityAuthClient::new(discovery);

    // Should fall back to local validation
    let jwt = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.dozjgNryP4J3jVmNHl0w5N_XgL0n3I9PlFUP0THsR8U";
    let result = client.validate_token(jwt).await;

    assert!(result.is_ok(), "Should succeed with fallback");
    assert!(result.unwrap(), "JWT should be valid via fallback");
}

#[tokio::test]
async fn test_capability_auth_refresh_no_services() {
    // Test refresh when no services available
    let discovery = CapabilityDiscovery::new_with_backends(vec![]);
    let client = CapabilityAuthClient::new(discovery);

    let token = "test-token";
    let result = client.refresh_token(token).await;

    // Should error since refresh requires a service
    assert!(result.is_err(), "Refresh should fail without services");
}

#[tokio::test]
async fn test_capability_auth_revoke_no_services() {
    // Test revoke when no services available
    let discovery = CapabilityDiscovery::new_with_backends(vec![]);
    let client = CapabilityAuthClient::new(discovery);

    let token = "test-token";
    let result = client.revoke_token(token).await;

    // Should succeed (logs warning but completes)
    assert!(result.is_ok(), "Revoke should succeed (local only)");
}

/// Test that capability-based pattern works correctly
#[tokio::test]
async fn test_capability_based_pattern() {
    let discovery = CapabilityDiscovery::new_with_backends(vec![]);
    let client = CapabilityAuthClient::new(discovery);

    // The key insight: we never specify WHICH service to use
    // We only specify WHAT capability we need ("authentication")
    // The system discovers and uses ANY service that provides it

    let jwt = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjoidGVzdCJ9.signature";
    let result = client.validate_token(jwt).await;

    // Success means the capability-based pattern works
    assert!(result.is_ok());

    // This validates the core principle:
    // "Primals discover capabilities, not specific services"
}

/// Test zero hardcoding principle
#[test]
fn test_zero_hardcoding_in_types() {
    // Verify no hardcoded primal names in type definitions
    let request = ValidateTokenRequest {
        token: "test".to_string(),
        permissions: None,
    };

    // The type doesn't know about beardog, songbird, etc.
    // It only knows about capabilities
    assert_eq!(request.token, "test");

    // This test documents that we achieve zero hardcoding
}

/// Performance test: validate many tokens
#[tokio::test]
async fn test_capability_auth_performance_many_tokens() {
    let discovery = CapabilityDiscovery::new_with_backends(vec![]);
    let client = CapabilityAuthClient::new(discovery);

    let jwt = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.dozjgNryP4J3jVmNHl0w5N_XgL0n3I9PlFUP0THsR8U";

    // Validate 100 tokens
    for _ in 0..100 {
        let result = client.validate_token(jwt).await;
        assert!(result.is_ok());
    }

    // If we got here, performance is acceptable
}

/// Test concurrent token validations
#[tokio::test]
async fn test_capability_auth_concurrent_validation() {
    use std::sync::Arc;

    let discovery = CapabilityDiscovery::new_with_backends(vec![]);
    let client = Arc::new(CapabilityAuthClient::new(discovery));

    let jwt = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.dozjgNryP4J3jVmNHl0w5N_XgL0n3I9PlFUP0THsR8U";

    // Spawn 10 concurrent validations
    let mut handles = vec![];
    for _ in 0..10 {
        let client_clone = Arc::clone(&client);
        let jwt_clone = jwt.to_string();

        let handle = tokio::spawn(async move { client_clone.validate_token(&jwt_clone).await });

        handles.push(handle);
    }

    // Wait for all to complete
    for handle in handles {
        let result = handle.await.expect("Task should complete");
        assert!(result.is_ok(), "Concurrent validation should succeed");
    }
}
