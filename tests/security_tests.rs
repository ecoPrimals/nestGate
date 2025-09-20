use nestgate_core::*;
use tokio;

/// Security and safety tests for NestGate
/// These tests verify security guarantees and safety properties

#[tokio::test]
async fn test_memory_safety_guarantees() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔒 Testing memory safety guarantees...");

    // Test safe string operations
    let input = "test input with special chars: <>&\"'";
    let result = safe_operations::safe_string_operation(input, "security_test")?;
    assert_eq!(result, input);

    // Test safe numeric operations with edge cases
    let max_safe = safe_operations::safe_numeric_operation(i64::MAX - 1, 1, "security_test")?;
    assert_eq!(max_safe, i64::MAX);

    // Test overflow protection
    let overflow_result = safe_operations::safe_numeric_operation(i64::MAX, 1, "security_test");
    assert!(overflow_result.is_err(), "Should detect overflow");

    println!("✅ Memory safety guarantees verified");
    Ok(())
}

#[tokio::test]
async fn test_input_validation() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔒 Testing input validation...");

    // Test SQL injection prevention
    let malicious_input = "'; DROP TABLE users; --";
    let sanitized = security::sanitize_input(malicious_input)?;
    assert!(!sanitized.contains("DROP TABLE"));
    assert!(!sanitized.contains("--"));

    // Test XSS prevention
    let xss_input = "<script>alert('xss')</script>";
    let sanitized = security::sanitize_input(xss_input)?;
    assert!(!sanitized.contains("<script>"));
    assert!(!sanitized.contains("</script>"));

    // Test path traversal prevention
    let path_traversal = "../../../etc/passwd";
    let sanitized_path = security::sanitize_path(path_traversal)?;
    assert!(!sanitized_path.contains("../"));
    assert!(!sanitized_path.contains("/etc/passwd"));

    println!("✅ Input validation tests passed");
    Ok(())
}

#[tokio::test]
async fn test_authentication_security() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔒 Testing authentication security...");

    let security_provider = security::SecurityProvider::new_test();

    // Test token generation and validation
    let user_id = "test_user_123";
    let token = security_provider.generate_test_token(user_id)?;

    // Valid token should validate
    assert!(security_provider.validate_token(&token)?);

    // Invalid token should fail
    let invalid_token = "invalid.token.here";
    assert!(!security_provider.validate_token(invalid_token)?);

    // Empty token should fail
    assert!(!security_provider.validate_token("")?);

    // Test token expiration (if implemented)
    // Note: This would require actual JWT implementation

    println!("✅ Authentication security tests passed");
    Ok(())
}

#[tokio::test]
async fn test_authorization_controls() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔒 Testing authorization controls...");

    let security_provider = security::SecurityProvider::new_test();

    // Test role-based access control
    let admin_token = security_provider.generate_test_token("admin_user")?;
    let user_token = security_provider.generate_test_token("regular_user")?;

    // Admin should have access to admin operations
    let admin_access = security_provider.check_permission(&admin_token, "admin_operation")?;
    assert!(admin_access, "Admin should have admin permissions");

    // Regular user should not have admin access
    let user_admin_access = security_provider.check_permission(&user_token, "admin_operation")?;
    assert!(
        !user_admin_access,
        "Regular user should not have admin permissions"
    );

    // Both should have access to basic operations
    let admin_basic = security_provider.check_permission(&admin_token, "basic_operation")?;
    let user_basic = security_provider.check_permission(&user_token, "basic_operation")?;
    assert!(
        admin_basic && user_basic,
        "Both should have basic permissions"
    );

    println!("✅ Authorization controls tests passed");
    Ok(())
}

#[tokio::test]
async fn test_data_encryption() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔒 Testing data encryption...");

    let security_provider = security::SecurityProvider::new_test();

    // Test sensitive data encryption
    let sensitive_data = "sensitive user information";
    let encrypted = security_provider.encrypt_data(sensitive_data.as_bytes())?;

    // Encrypted data should be different from original
    assert_ne!(encrypted, sensitive_data.as_bytes());

    // Should be able to decrypt back to original
    let decrypted = security_provider.decrypt_data(&encrypted)?;
    assert_eq!(decrypted, sensitive_data.as_bytes());

    // Test that encryption is not deterministic (different each time)
    let encrypted2 = security_provider.encrypt_data(sensitive_data.as_bytes())?;
    assert_ne!(
        encrypted, encrypted2,
        "Encryption should not be deterministic"
    );

    println!("✅ Data encryption tests passed");
    Ok(())
}

#[tokio::test]
async fn test_secure_communication() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔒 Testing secure communication...");

    let security_provider = security::SecurityProvider::new_test();

    // Test message signing
    let message = "important system message";
    let signature = security_provider.sign_message(message.as_bytes())?;

    // Should be able to verify signature
    let is_valid = security_provider.verify_signature(message.as_bytes(), &signature)?;
    assert!(is_valid, "Valid signature should verify");

    // Modified message should fail verification
    let modified_message = "modified system message";
    let is_valid_modified =
        security_provider.verify_signature(modified_message.as_bytes(), &signature)?;
    assert!(
        !is_valid_modified,
        "Modified message should fail verification"
    );

    // Invalid signature should fail
    let invalid_signature = vec![0u8; signature.len()];
    let is_valid_invalid =
        security_provider.verify_signature(message.as_bytes(), &invalid_signature)?;
    assert!(
        !is_valid_invalid,
        "Invalid signature should fail verification"
    );

    println!("✅ Secure communication tests passed");
    Ok(())
}

#[tokio::test]
async fn test_rate_limiting() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔒 Testing rate limiting...");

    let security_provider = security::SecurityProvider::new_test();
    let client_id = "test_client";
    let rate_limit = 10; // 10 requests per period

    // Should allow requests within limit
    for i in 0..rate_limit {
        let allowed = security_provider.check_rate_limit(client_id, "test_operation")?;
        assert!(allowed, "Request {} should be allowed", i + 1);
        Ok(())
    }

    // Should deny requests over limit
    let over_limit = security_provider.check_rate_limit(client_id, "test_operation")?;
    assert!(!over_limit, "Request over limit should be denied");

    println!("✅ Rate limiting tests passed");
    Ok(())
}

#[tokio::test]
async fn test_audit_logging() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔒 Testing audit logging...");

    let security_provider = security::SecurityProvider::new_test();

    // Test security event logging
    security_provider.log_security_event(
        "authentication_attempt",
        "test_user",
        "success",
        std::collections::HashMap::new(),
    )?;

    security_provider.log_security_event(
        "permission_check",
        "test_user",
        "admin_operation_denied",
        [("resource".to_string(), "admin_panel".to_string())].into(),
    )?;

    // Retrieve audit logs
    let logs = security_provider.get_audit_logs("test_user", None, None)?;
    assert!(!logs.is_empty(), "Should have audit logs");
    assert!(logs.len() >= 2, "Should have at least 2 log entries");

    // Check log content
    let auth_log = logs
        .iter()
        .find(|log| log.event_type == "authentication_attempt");
    assert!(auth_log.is_some(), "Should have authentication log");

    let perm_log = logs.iter().find(|log| log.event_type == "permission_check");
    assert!(perm_log.is_some(), "Should have permission check log");

    println!("✅ Audit logging tests passed");
    Ok(())
}

#[tokio::test]
async fn test_secure_configuration() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔒 Testing secure configuration...");

    let config = config::canonical_master::NestGateCanonicalConfig::default();

    // Test that security is enabled by default
    assert!(
        config.security.enabled,
        "Security should be enabled by default"
    );

    // Test that secure defaults are used
    assert!(
        config.network.tls_enabled,
        "TLS should be enabled by default"
    );
    assert!(
        config.security.require_authentication,
        "Authentication should be required"
    );
    assert!(
        config.security.enable_audit_logging,
        "Audit logging should be enabled"
    );

    // Test that insecure configurations are rejected
    let mut insecure_config = config.clone();
    insecure_config.security.enabled = false;

    let validation_result = config::validate_security_config(&insecure_config);
    assert!(
        validation_result.is_err(),
        "Should reject insecure configuration"
    );

    println!("✅ Secure configuration tests passed");
    Ok(())
}

#[tokio::test]
async fn test_threat_detection() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔒 Testing threat detection...");

    let security_provider = security::SecurityProvider::new_test();

    // Test suspicious activity detection
    let client_ip = "192.168.1.100";

    // Simulate multiple failed login attempts
    for _ in 0..5 {
        security_provider.record_failed_login(client_ip, "test_user")?;
        Ok(())
    }

    // Should detect suspicious activity
    let is_suspicious = security_provider.is_suspicious_activity(client_ip)?;
    assert!(
        is_suspicious,
        "Should detect suspicious activity after multiple failures"
    );

    // Test IP blocking
    let is_blocked = security_provider.is_ip_blocked(client_ip)?;
    assert!(is_blocked, "IP should be blocked after suspicious activity");

    // Test anomaly detection
    let normal_request = security::RequestPattern {
        user_agent: "Mozilla/5.0 (normal browser)".to_string(),
        request_size: 1024,
        request_frequency: 1.0, // 1 request per second
        endpoint_pattern: "/api/users".to_string(),
    };

    let anomalous_request = security::RequestPattern {
        user_agent: "curl/7.0 (automated)".to_string(),
        request_size: 1024 * 1024, // 1MB request
        request_frequency: 100.0,  // 100 requests per second
        endpoint_pattern: "/api/admin/dump".to_string(),
    };

    let normal_score = security_provider.calculate_anomaly_score(&normal_request)?;
    let anomalous_score = security_provider.calculate_anomaly_score(&anomalous_request)?;

    assert!(
        anomalous_score > normal_score,
        "Anomalous request should have higher score"
    );
    assert!(anomalous_score > 0.8, "Anomalous request should be flagged");

    println!("✅ Threat detection tests passed");
    Ok(())
}

#[tokio::test]
async fn test_comprehensive_security_suite() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔒 Running comprehensive security test suite...");

    let security_provider = security::SecurityProvider::new_test();
    let test_user = "comprehensive_test_user";

    // 1. Authentication
    let token = security_provider.generate_test_token(test_user)?;
    assert!(security_provider.validate_token(&token)?);

    // 2. Authorization
    let has_permission = security_provider.check_permission(&token, "read_operation")?;
    assert!(has_permission);

    // 3. Data protection
    let sensitive_data = "confidential information";
    let encrypted = security_provider.encrypt_data(sensitive_data.as_bytes())?;
    let decrypted = security_provider.decrypt_data(&encrypted)?;
    assert_eq!(decrypted, sensitive_data.as_bytes());

    // 4. Secure communication
    let message = "secure message";
    let signature = security_provider.sign_message(message.as_bytes())?;
    assert!(security_provider.verify_signature(message.as_bytes(), &signature)?);

    // 5. Input validation
    let malicious_input = "<script>alert('xss')</script>; DROP TABLE users;";
    let sanitized = security::sanitize_input(malicious_input)?;
    assert!(!sanitized.contains("<script>"));
    assert!(!sanitized.contains("DROP TABLE"));

    // 6. Rate limiting
    assert!(security_provider.check_rate_limit(test_user, "test_operation")?);

    // 7. Audit logging
    security_provider.log_security_event(
        "comprehensive_test",
        test_user,
        "all_tests_passed",
        std::collections::HashMap::new(),
    )?;

    let logs = security_provider.get_audit_logs(test_user, None, None)?;
    assert!(!logs.is_empty());

    println!("✅ Comprehensive security test suite passed");
    println!("🔒 Security Summary:");
    println!("   - Authentication: ✅ Verified");
    println!("   - Authorization: ✅ Verified");
    println!("   - Data Encryption: ✅ Verified");
    println!("   - Message Signing: ✅ Verified");
    println!("   - Input Sanitization: ✅ Verified");
    println!("   - Rate Limiting: ✅ Verified");
    println!("   - Audit Logging: ✅ Verified");

    Ok(())
}
