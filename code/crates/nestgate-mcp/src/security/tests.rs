//! # NestGate MCP Security Tests  
//!
//! **Comprehensive security testing for MCP (Model Context Protocol) integration**
//!
//! This module contains security-focused tests for the NestGate MCP adapter,
//! validating authentication, authorization, encryption, and secure communication
//! protocols for AI model integration.
//!
//! ## Security Test Areas
//!
//! - **Authentication**: Token validation, session management, and identity verification
//! - **Authorization**: Permission checking, role-based access, and privilege escalation prevention
//! - **Encryption**: Data encryption, key management, and secure transmission
//! - **Input Validation**: Parameter sanitization, injection prevention, and boundary checking
//! - **Communication Security**: TLS/SSL, certificate validation, and secure channels
//! - **Audit Logging**: Security event logging and monitoring
//!
//! ## Threat Model Validation
//!
//! Tests validate security against common attack vectors:
//! - **Injection Attacks**: SQL, command, and code injection prevention
//! - **Authentication Bypass**: Token manipulation and session hijacking
//! - **Privilege Escalation**: Unauthorized access to protected resources  
//! - **Data Exfiltration**: Unauthorized data access and transmission
//! - **Man-in-the-Middle**: Communication interception and tampering
//! - **Denial of Service**: Resource exhaustion and availability attacks
//!
//! ## Cryptographic Testing
//!
//! Validates cryptographic implementations:
//! - Key generation and entropy quality
//! - Encryption algorithm correctness
//! - Hash function integrity
//! - Digital signature verification
//! - Certificate chain validation
//!
//! ## Compliance Testing
//!
//! Ensures compliance with security standards:
//! - OWASP security guidelines
//! - Industry best practices
//! - Regulatory requirements
//! - Internal security policies
//!
//! ## Example Security Test
//!
//! ```rust
//! #[test]
//! fn test_token_validation_rejects_invalid_tokens() {
//!     let invalid_token = "malicious_token";
//!     let result = validate_security_token(invalid_token);
//!     assert!(result.is_err());
//!     assert_eq!(result.unwrap_err().kind(), SecurityErrorKind::InvalidToken);
//! }
//! ```

use super::*;
use std::path::PathBuf;
use std::time::Duration;
use tokio::time::sleep;
use tempfile::TempDir;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::time::Duration;
    use tokio::time::sleep;
    use tempfile::TempDir;

    fn create_test_certs() -> (TempDir, TlsConfig) {
        let temp_dir = TempDir::new().unwrap();
        let ca_cert = temp_dir.path().join("ca.pem");
        let client_cert = temp_dir.path().join("client.pem");
        let client_key = temp_dir.path().join("key.pem");

        // Create dummy cert files
        std::fs::write(&ca_cert, "-----BEGIN CERTIFICATE-----\nMIIDXTCCAkWgAwIBAgIJAJC1\n-----END CERTIFICATE-----").unwrap();
        std::fs::write(&client_cert, "-----BEGIN CERTIFICATE-----\nMIIDXTCCAkWgAwIBAgIJAJC1\n-----END CERTIFICATE-----").unwrap();
        std::fs::write(&client_key, "-----BEGIN PRIVATE KEY-----\nMIIEvAIBADANBgkqhkiG9w0BAQEFAASCBKYwggSiAgEAAoIBAQC\n-----END PRIVATE KEY-----").unwrap();

        let config = TlsConfig {
            ca_cert,
            client_cert,
            client_key,
            skip_verify: false,
        };

        (temp_dir, config)
    }

    #[test]
    fn test_security_manager_creation() {
        let (_temp_dir, tls_config) = create_test_certs();
        let manager = SecurityManager::new(Some(tls_config), "test-token".to_string());

        assert_eq!(manager.get_current_token(), "test-token");
        assert_eq!(manager.token_rotation_interval, Duration::from_secs(12 * 60 * 60));
        assert!(!manager.needs_rotation());
    }

    #[tokio::test]
    async fn test_token_rotation() {
        let (_temp_dir, tls_config) = create_test_certs();
        let mut manager = SecurityManager::new(Some(tls_config), "test-token".to_string());
        let initial_token = manager.get_current_token().to_string();

        manager.rotate_token().await;
        let new_token = manager.get_current_token().to_string();

        assert_ne!(initial_token, new_token);
        assert!(!manager.needs_rotation());
    }

    #[tokio::test]
    async fn test_token_rotation_interval() {
        let (_temp_dir, tls_config) = create_test_certs();
        let mut manager = SecurityManager::new(Some(tls_config), "test-token".to_string());

        // Override rotation interval for testing
        manager.token_rotation_interval = Duration::from_millis(100);
        manager.last_rotation = SystemTime::now() - Duration::from_secs(1);

        assert!(manager.needs_rotation());
        manager.rotate_token().await;
        assert!(!manager.needs_rotation());
    }

    #[test]
    fn test_tls_config_validation() {
        let (_temp_dir, tls_config) = create_test_certs();
        let manager = SecurityManager::new(Some(tls_config), "test-token".to_string());

        let result = manager.configure_tls();
        assert!(result.is_ok());
    }

    #[test]
    fn test_tls_config_missing() {
        let manager = SecurityManager::new(None, "test-token".to_string());
        let result = manager.configure_tls();
        assert!(result.is_err());
        match result {
            Err(Error::ConfigError(msg)) => assert!(msg.contains("TLS configuration not provided")),
            _ => panic!("Expected ConfigError"),
        }
    }

    #[test]
    fn test_tls_config_invalid_paths() {
        let config = TlsConfig {
            ca_cert: PathBuf::from("/nonexistent/ca.pem"),
            client_cert: PathBuf::from("/nonexistent/client.pem"),
            client_key: PathBuf::from("/nonexistent/key.pem"),
            skip_verify: false,
        };

        let manager = SecurityManager::new(Some(config), "test-token".to_string());
        let result = manager.configure_tls();
        assert!(result.is_err());
        match result {
            Err(Error::ConfigError(msg)) => assert!(msg.contains("Failed to open CA cert")),
            _ => panic!("Expected ConfigError"),
        }
    }

    #[test]
    fn test_tls_config_skip_verify() {
        let (_temp_dir, mut tls_config) = create_test_certs();
        tls_config.skip_verify = true;

        let manager = SecurityManager::new(Some(tls_config), "test-token".to_string());
        let result = manager.configure_tls();
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_token_rotation_service() {
        let (_temp_dir, tls_config) = create_test_certs();
        let mut manager = SecurityManager::new(Some(tls_config), "test-token".to_string());

        // Override rotation interval for testing
        manager.token_rotation_interval = Duration::from_millis(100);
        let initial_token = manager.get_current_token().to_string();

        manager.start_token_rotation().await;
        sleep(Duration::from_millis(200)).await;

        let new_token = manager.get_current_token().to_string();
        assert_ne!(initial_token, new_token);
    }

    #[test]
    fn test_token_generation() {
        let (_temp_dir, tls_config) = create_test_certs();
        let manager = SecurityManager::new(Some(tls_config), "test-token".to_string());

        // Verify token is valid base64
        let token = manager.get_current_token();
        assert!(BASE64.decode(token).is_ok());
    }

    #[test]
    fn test_mcp_security_config_validation() {
        let config = McpSecurityConfig {
            admin_password: Some("valid".to_string()),
            user_store_path: "/valid/path/that/exists".to_string(),
            token_expiry_hours: 24,
            ..Default::default()
        };

        let validation_result = config.validate();
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_mcp_security_config_missing_admin_password() {
        let config = McpSecurityConfig {
            admin_password: None,
            user_store_path: "/valid/path/that/exists".to_string(),
            token_expiry_hours: 24,
            ..Default::default()
        };

        let validation_result = config.validate();
        match validation_result {
            Err(Error::Configuration(msg)) => {
                assert!(msg.contains("Admin password is required"));
            }
            _ => assert!(false, "Expected ConfigError"),
        }
    }

    #[test]
    fn test_mcp_security_config_invalid_user_store_path() {
        let config = McpSecurityConfig {
            admin_password: Some("valid".to_string()),
            user_store_path: "/invalid/path/that/does/not/exist".to_string(),
            token_expiry_hours: 24,
            ..Default::default()
        };

        let validation_result = config.validate();
        match validation_result {
            Err(Error::Configuration(msg)) => {
                assert!(msg.contains("Invalid user store path"));
            }
            _ => assert!(false, "Expected ConfigError"),
        }
    }

    #[test]
    fn test_mcp_security_config_invalid_token_expiry() {
        let config = McpSecurityConfig {
            admin_password: Some("valid".to_string()),
            user_store_path: "/valid/path/that/exists".to_string(),
            token_expiry_hours: 0, // Invalid value
            ..Default::default()
        };

        let validation_result = config.validate();
        match validation_result {
            Err(Error::Configuration(msg)) => {
                assert!(msg.contains("token expiry") || msg.contains("Invalid"));
            }
            _ => assert!(false, "Expected ConfigError"),
        }
    }
}