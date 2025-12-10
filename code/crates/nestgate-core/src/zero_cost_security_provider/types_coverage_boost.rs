// Strategic tests to boost security provider types coverage from 44.55% to 80%+
// Focus: Validation, error paths, edge cases, and boundary conditions

#[cfg(test)]
mod security_types_coverage_tests {
    use super::super::types::*;
    use std::time::Duration;

    // ==================== CREDENTIALS TESTS ====================

    #[test]
    fn test_credentials_empty_username() {
        let creds = ZeroCostCredentials::new_password(String::new(), "password".to_string());
        assert!(!creds.is_valid()); // Empty username should be invalid
    }

    #[test]
    fn test_credentials_empty_password() {
        let creds = ZeroCostCredentials::new_password("username".to_string(), String::new());
        assert!(!creds.is_valid()); // Empty password should be invalid
    }

    #[test]
    fn test_credentials_both_empty() {
        let creds = ZeroCostCredentials::new_password(String::new(), String::new());
        assert!(!creds.is_valid());
    }

    #[test]
    fn test_credentials_token_creation() {
        let creds = ZeroCostCredentials::new_token("user".to_string(), "token123".to_string());
        assert_eq!(creds.auth_method, AuthMethod::Token);
        assert!(creds.is_valid());
    }

    #[test]
    fn test_credentials_certificate_creation() {
        let creds =
            ZeroCostCredentials::new_certificate("user".to_string(), "cert_data".to_string());
        assert_eq!(creds.auth_method, AuthMethod::Certificate);
        assert!(creds.is_valid());
    }

    #[test]
    fn test_credentials_with_metadata() {
        let creds = ZeroCostCredentials::new_password("user".to_string(), "pass".to_string())
            .with_metadata("key1".to_string(), "value1".to_string())
            .with_metadata("key2".to_string(), "value2".to_string());

        assert_eq!(creds.metadata.get("key1"), Some(&"value1".to_string()));
        assert_eq!(creds.metadata.get("key2"), Some(&"value2".to_string()));
    }

    #[test]
    fn test_credentials_metadata_overwrite() {
        let creds = ZeroCostCredentials::new_password("user".to_string(), "pass".to_string())
            .with_metadata("key".to_string(), "value1".to_string())
            .with_metadata("key".to_string(), "value2".to_string());

        assert_eq!(creds.metadata.get("key"), Some(&"value2".to_string()));
    }

    #[test]
    fn test_credentials_special_characters() {
        let special_username = "user@domain.com";
        let special_password = "p@$$w0rd!#$%";
        let creds = ZeroCostCredentials::new_password(
            special_username.to_string(),
            special_password.to_string(),
        );
        assert!(creds.is_valid());
    }

    #[test]
    fn test_credentials_very_long_username() {
        let long_username = "a".repeat(10000);
        let creds = ZeroCostCredentials::new_password(long_username.clone(), "pass".to_string());
        assert_eq!(creds.username, long_username);
        assert!(creds.is_valid());
    }

    #[test]
    fn test_credentials_unicode() {
        let creds = ZeroCostCredentials::new_password("用户".to_string(), "密码".to_string());
        assert!(creds.is_valid());
    }

    // ==================== AUTH TOKEN TESTS ====================

    #[test]
    fn test_auth_token_zero_expiry() {
        let token = ZeroCostAuthToken::new(
            "token".to_string(),
            "user".to_string(),
            vec![],
            Duration::from_secs(0),
        );
        // Token with zero expiry should be immediately expired
        assert!(token.is_expired());
    }

    #[test]
    fn test_auth_token_far_future_expiry() {
        let token = ZeroCostAuthToken::new(
            "token".to_string(),
            "user".to_string(),
            vec![],
            Duration::from_secs(365 * 24 * 3600), // 1 year
        );
        assert!(!token.is_expired());
    }

    #[test]
    fn test_auth_token_empty_permissions() {
        let token = ZeroCostAuthToken::new(
            "token".to_string(),
            "user".to_string(),
            vec![],
            Duration::from_secs(3600),
        );
        assert!(!token.has_permission("any_permission"));
    }

    #[test]
    fn test_auth_token_multiple_permissions() {
        let token = ZeroCostAuthToken::new(
            "token".to_string(),
            "user".to_string(),
            vec![
                "read".to_string(),
                "write".to_string(),
                "delete".to_string(),
            ],
            Duration::from_secs(3600),
        );
        assert!(token.has_permission("read"));
        assert!(token.has_permission("write"));
        assert!(token.has_permission("delete"));
        assert!(!token.has_permission("admin"));
    }

    #[test]
    fn test_auth_token_case_sensitive_permissions() {
        let token = ZeroCostAuthToken::new(
            "token".to_string(),
            "user".to_string(),
            vec!["Read".to_string()],
            Duration::from_secs(3600),
        );
        assert!(token.has_permission("Read"));
        assert!(!token.has_permission("read")); // Case sensitive
    }

    #[test]
    fn test_auth_token_empty_token_value() {
        let token = ZeroCostAuthToken::new(
            String::new(),
            "user".to_string(),
            vec!["read".to_string()],
            Duration::from_secs(3600),
        );
        assert!(!token.is_expired());
    }

    #[test]
    fn test_auth_token_empty_user_id() {
        let token = ZeroCostAuthToken::new(
            "token".to_string(),
            String::new(),
            vec!["read".to_string()],
            Duration::from_secs(3600),
        );
        assert!(!token.is_expired());
    }

    #[test]
    fn test_auth_token_duplicate_permissions() {
        let token = ZeroCostAuthToken::new(
            "token".to_string(),
            "user".to_string(),
            vec!["read".to_string(), "read".to_string(), "read".to_string()],
            Duration::from_secs(3600),
        );
        assert!(token.has_permission("read"));
    }

    // ==================== SIGNATURE TESTS ====================

    #[test]
    fn test_signature_empty_algorithm() {
        let sig = ZeroCostSignature::new(
            String::new(),
            "signature_data".to_string(),
            "key_id".to_string(),
        );
        assert!(!sig.is_valid()); // Empty algorithm should be invalid
    }

    #[test]
    fn test_signature_empty_data() {
        let sig = ZeroCostSignature::new(
            "ECDSA-P256".to_string(),
            String::new(),
            "key_id".to_string(),
        );
        assert!(!sig.is_valid()); // Empty signature data should be invalid
    }

    #[test]
    fn test_signature_empty_key_id() {
        let sig = ZeroCostSignature::new(
            "ECDSA-P256".to_string(),
            "signature_data".to_string(),
            String::new(),
        );
        assert!(!sig.is_valid()); // Empty key ID should be invalid
    }

    #[test]
    fn test_signature_all_empty() {
        let sig = ZeroCostSignature::new(String::new(), String::new(), String::new());
        assert!(!sig.is_valid());
    }

    #[test]
    fn test_signature_various_algorithms() {
        let algorithms = vec![
            "ECDSA-P256",
            "ECDSA-P384",
            "RSA-PSS-2048",
            "RSA-PSS-4096",
            "Ed25519",
            "HMAC-SHA256",
        ];

        for algo in algorithms {
            let sig = ZeroCostSignature::new(
                algo.to_string(),
                "signature_data".to_string(),
                "key_id".to_string(),
            );
            assert!(sig.is_valid());
            assert_eq!(sig.algorithm, algo);
        }
    }

    #[test]
    fn test_signature_very_long_data() {
        let long_data = "a".repeat(100000);
        let sig = ZeroCostSignature::new(
            "ECDSA-P256".to_string(),
            long_data.clone(),
            "key_id".to_string(),
        );
        assert!(sig.is_valid());
        assert_eq!(sig.signature, long_data);
    }

    #[test]
    fn test_signature_base64_data() {
        let base64_data = "SGVsbG8gV29ybGQh"; // "Hello World!" in base64
        let sig = ZeroCostSignature::new(
            "ECDSA-P256".to_string(),
            base64_data.to_string(),
            "key_id".to_string(),
        );
        assert!(sig.is_valid());
    }

    #[test]
    fn test_signature_special_characters_in_key_id() {
        let sig = ZeroCostSignature::new(
            "ECDSA-P256".to_string(),
            "signature_data".to_string(),
            "key-id-with-dashes_and_underscores.123".to_string(),
        );
        assert!(sig.is_valid());
    }

    // ==================== SECURITY CONTEXT TESTS ====================

    #[test]
    fn test_security_context_minimal() {
        let ctx = SecurityContext::new("user123".to_string(), "session456".to_string());
        assert_eq!(ctx.user_id, "user123");
        assert_eq!(ctx.session_id, "session456");
        assert!(ctx.client_ip.is_none());
        assert!(ctx.user_agent.is_none());
    }

    #[test]
    fn test_security_context_with_client_info() {
        let ctx = SecurityContext::new("user".to_string(), "session".to_string()).with_client_info(
            Some("192.168.1.1".to_string()),
            Some("Mozilla/5.0".to_string()),
        );

        assert_eq!(ctx.client_ip, Some("192.168.1.1".to_string()));
        assert_eq!(ctx.user_agent, Some("Mozilla/5.0".to_string()));
    }

    #[test]
    fn test_security_context_with_partial_client_info() {
        let ctx1 = SecurityContext::new("user".to_string(), "session".to_string())
            .with_client_info(Some("192.168.1.1".to_string()), None);
        assert_eq!(ctx1.client_ip, Some("192.168.1.1".to_string()));
        assert_eq!(ctx1.user_agent, None);

        let ctx2 = SecurityContext::new("user".to_string(), "session".to_string())
            .with_client_info(None, Some("Mozilla/5.0".to_string()));
        assert_eq!(ctx2.client_ip, None);
        assert_eq!(ctx2.user_agent, Some("Mozilla/5.0".to_string()));
    }

    #[test]
    fn test_security_context_with_metadata() {
        let ctx = SecurityContext::new("user".to_string(), "session".to_string())
            .with_metadata("region".to_string(), "us-east-1".to_string())
            .with_metadata("device".to_string(), "mobile".to_string());

        assert_eq!(ctx.metadata.get("region"), Some(&"us-east-1".to_string()));
        assert_eq!(ctx.metadata.get("device"), Some(&"mobile".to_string()));
    }

    #[test]
    fn test_security_context_request_id_uniqueness() {
        let ctx1 = SecurityContext::new("user".to_string(), "session".to_string());
        let ctx2 = SecurityContext::new("user".to_string(), "session".to_string());

        // Request IDs should be unique (UUIDs)
        assert_ne!(ctx1.request_id, ctx2.request_id);
    }

    #[test]
    fn test_security_context_empty_ids() {
        let ctx = SecurityContext::new(String::new(), String::new());
        assert_eq!(ctx.user_id, "");
        assert_eq!(ctx.session_id, "");
    }

    #[test]
    fn test_security_context_ipv6() {
        let ctx = SecurityContext::new("user".to_string(), "session".to_string()).with_client_info(
            Some("2001:0db8:85a3:0000:0000:8a2e:0370:7334".to_string()),
            None,
        );

        assert!(ctx.client_ip.is_some());
    }

    #[test]
    fn test_security_context_complex_user_agent() {
        let complex_ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36";
        let ctx = SecurityContext::new("user".to_string(), "session".to_string())
            .with_client_info(None, Some(complex_ua.to_string()));

        assert_eq!(ctx.user_agent, Some(complex_ua.to_string()));
    }

    #[test]
    fn test_security_context_chained_builders() {
        let ctx = SecurityContext::new("user".to_string(), "session".to_string())
            .with_client_info(Some("192.168.1.1".to_string()), Some("Mozilla".to_string()))
            .with_metadata("key1".to_string(), "value1".to_string())
            .with_metadata("key2".to_string(), "value2".to_string());

        assert!(ctx.client_ip.is_some());
        assert!(ctx.user_agent.is_some());
        assert_eq!(ctx.metadata.len(), 2);
    }

    // ==================== AUTH METHOD TESTS ====================

    #[test]
    fn test_auth_method_default() {
        let method: AuthMethod = Default::default();
        assert_eq!(method, AuthMethod::Password);
    }

    #[test]
    fn test_auth_method_equality() {
        assert_eq!(AuthMethod::Password, AuthMethod::Password);
        assert_eq!(AuthMethod::Token, AuthMethod::Token);
        assert_eq!(AuthMethod::Certificate, AuthMethod::Certificate);

        assert_ne!(AuthMethod::Password, AuthMethod::Token);
        assert_ne!(AuthMethod::Token, AuthMethod::Certificate);
    }

    #[test]
    fn test_auth_method_clone() {
        let method = AuthMethod::Token;
        let cloned = method.clone();
        assert_eq!(method, cloned);
    }
}
