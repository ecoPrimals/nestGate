// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

// **SECURITY TRAITS - CANONICAL MODERNIZED**
//! Security trait definitions for universal providers
// Security-related traits and types for universal primal integration.
// Native async traits without async_trait overhead for optimal performance.

use crate::Result;
use serde::{Deserialize, Serialize};

/// Universal security primal provider trait
/// **CANONICAL MODERNIZATION**: Native async trait without `async_trait` overhead
/// **DEPRECATED**: Consolidated into canonical SecurityProvider
///
/// # Migration Path
///
/// **Old code**:
/// ```rust,ignore
/// impl SecurityPrimalProvider for MyProvider { ... }
/// ```
///
/// **New code**:
/// ```rust,ignore
/// impl CanonicalSecurity for MyProvider { ... }
/// ```
///
/// See: `crate::traits::canonical::CanonicalSecurity`
/// Timeline: Deprecated v0.11.3 (Nov 2025), Remove v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.3",
    note = "Use crate::traits::canonical::CanonicalSecurity instead"
)]
/// SecurityPrimalProvider trait
pub trait SecurityPrimalProvider: Send + Sync {
    /// Authenticate with provided credentials
    fn authenticate(
        &self,
        credentials: &Credentials,
    ) -> impl std::future::Future<Output = Result<AuthToken>> + Send;
    /// Encrypt data with specified algorithm
    fn encrypt(
        &self,
        data: &[u8],
        algorithm: &str,
    ) -> impl std::future::Future<Output = Result<Vec<u8>>> + Send;

    /// Decrypt data with specified algorithm
    fn decrypt(
        &self,
        encrypted: &[u8],
        algorithm: &str,
    ) -> impl std::future::Future<Output = Result<Vec<u8>>> + Send;

    /// Sign data cryptographically
    fn sign_data(&self, data: &[u8])
    -> impl std::future::Future<Output = Result<Signature>> + Send;

    /// Verify cryptographic signature
    fn verify_signature(
        &self,
        data: &[u8],
        signature: &Signature,
    ) -> impl std::future::Future<Output = Result<bool>> + Send;

    /// Get signing key identifier
    fn get_key_id(&self) -> impl std::future::Future<Output = Result<String>> + Send;

    /// Hash data with specified algorithm
    fn hash_data(
        &self,
        data: &[u8],
        algorithm: &str,
    ) -> impl std::future::Future<Output = Result<Vec<u8>>> + Send;

    /// Generate secure random bytes
    fn generate_random(
        &self,
        length: usize,
    ) -> impl std::future::Future<Output = Result<Vec<u8>>> + Send;

    /// Derive key from password
    fn derive_key(
        &self,
        password: &str,
        salt: &[u8],
        iterations: u32,
    ) -> impl std::future::Future<Output = Result<Vec<u8>>> + Send;

    /// Create secure session
    fn create_session(
        &self,
        user_id: &str,
        permissions: Vec<String>,
    ) -> impl std::future::Future<Output = Result<String>> + Send;

    /// Validate session token
    fn validate_session(
        &self,
        session_token: &str,
    ) -> impl std::future::Future<Output = Result<SecurityDecision>> + Send;

    /// Evaluate boundary access for cross-domain operations
    fn evaluate_boundary_access(
        &self,
        source: &str,
        destination: &str,
    ) -> impl std::future::Future<Output = Result<SecurityDecision>> + Send;
}

/// Security decision enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Securitydecision
pub enum SecurityDecision {
    /// Allow
    Allow,
    /// Deny
    Deny,
    /// Requireadditionalauth
    RequireAdditionalAuth,
    /// Requiremfa
    RequireMFA,
    /// Rate limit exceeded - includes time in seconds to wait before retrying
    RateLimit {
        /// Number of seconds to wait before retrying
        retry_after: u64,
    },
}
/// Authentication credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Credentials
pub struct Credentials {
    /// Username
    pub username: String,
    /// Password
    pub password: String,
    /// Mfa Token
    pub mfa_token: Option<String>,
    /// Client Info
    pub client_info: Option<String>,
}
/// Authentication token
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Authtoken
pub struct AuthToken {
    /// Token
    pub token: String,
    /// Expires At
    pub expires_at: std::time::SystemTime,
    /// Permissions
    pub permissions: Vec<String>,
}
/// Cryptographic signature
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Signature
pub struct Signature {
    /// Algorithm
    pub algorithm: String,
    /// Signature
    pub signature: Vec<u8>,
    /// Key identifier
    pub key_id: Option<String>,
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;
    // Mock implementation for testing
    struct MockSecurityProvider {
        key_id: String,
    }

    impl MockSecurityProvider {
        /// Creates a new instance
        fn new() -> Self {
            Self {
                key_id: "test-key-123".to_string(),
            }
        }
    }

    #[allow(deprecated)] // Test mock using deprecated trait
    impl SecurityPrimalProvider for MockSecurityProvider {
        /// Authenticate
        async fn authenticate(&self, credentials: &Credentials) -> Result<AuthToken> {
            if credentials.username == "test_user" && credentials.password == "test_pass" {
                Ok(AuthToken {
                    token: "valid_token_123".to_string(),
                    expires_at: SystemTime::now() + std::time::Duration::from_secs(3600),
                    permissions: vec!["read".to_string(), "write".to_string()],
                })
            } else {
                Err(crate::error::NestGateError::security("Invalid credentials"))
            }
        }

        /// Encrypt
        async fn encrypt(&self, data: &[u8], algorithm: &str) -> Result<Vec<u8>> {
            if algorithm == "AES256" {
                let mut encrypted = data.to_vec();
                // Simple XOR encryption for testing
                for byte in &mut encrypted {
                    *byte ^= 0xAA;
                }
                Ok(encrypted)
            } else {
                Err(crate::error::NestGateError::security(
                    "Unsupported algorithm",
                ))
            }
        }

        /// Decrypt
        async fn decrypt(&self, encrypted: &[u8], algorithm: &str) -> Result<Vec<u8>> {
            if algorithm == "AES256" {
                let mut decrypted = encrypted.to_vec();
                // Simple XOR decryption for testing
                for byte in &mut decrypted {
                    *byte ^= 0xAA;
                }
                Ok(decrypted)
            } else {
                Err(crate::error::NestGateError::security(
                    "Unsupported algorithm",
                ))
            }
        }

        /// Sign Data
        async fn sign_data(&self, data: &[u8]) -> Result<Signature> {
            Ok(Signature {
                algorithm: "RS256".to_string(),
                signature: data.iter().map(|b| b.wrapping_add(1)).collect(),
                key_id: Some(self.key_id.clone()),
            })
        }

        /// Verify Signature
        async fn verify_signature(&self, data: &[u8], signature: &Signature) -> Result<bool> {
            if signature.algorithm == "RS256" {
                let expected: Vec<u8> = data.iter().map(|b| b.wrapping_add(1)).collect();
                Ok(signature.signature == expected)
            } else {
                Ok(false)
            }
        }

        /// Gets Key Id
        async fn get_key_id(&self) -> Result<String> {
            Ok(self.key_id.clone())
        }

        /// Hash Data
        async fn hash_data(&self, data: &[u8], algorithm: &str) -> Result<Vec<u8>> {
            match algorithm {
                "SHA256" => Ok(data.iter().map(|b| b.wrapping_mul(2)).collect()),
                _ => Err(crate::error::NestGateError::security(
                    "Unsupported hash algorithm",
                )),
            }
        }

        /// Generate Random
        async fn generate_random(&self, length: usize) -> Result<Vec<u8>> {
            Ok((0..length).map(|_| fastrand::u8(..)).collect())
        }

        /// Derive Key
        async fn derive_key(
            &self,
            password: &str,
            salt: &[u8],
            iterations: u32,
        ) -> Result<Vec<u8>> {
            let combined = format!("{password}:{salt:?}:{iterations}");
            Ok(combined.as_bytes().to_vec())
        }

        /// Creates  Session
        async fn create_session(&self, user_id: &str, permissions: Vec<String>) -> Result<String> {
            let permissions_str = permissions.join(",");
            Ok(format!(
                "session_{}_{}_{}",
                user_id, permissions_str, "token"
            ))
        }

        /// Validates  Session
        async fn validate_session(&self, session_token: &str) -> Result<SecurityDecision> {
            if session_token.starts_with("session_") {
                Ok(SecurityDecision::Allow)
            } else if session_token == "rate_limited" {
                Ok(SecurityDecision::RateLimit { retry_after: 60 })
            } else {
                Ok(SecurityDecision::Deny)
            }
        }

        /// Evaluate Boundary Access
        async fn evaluate_boundary_access(
            &self,
            source: &str,
            destination: &str,
        ) -> Result<SecurityDecision> {
            // Allow operations within same source/destination, deny cross-boundary for testing
            if source == destination {
                Ok(SecurityDecision::Allow)
            } else {
                Ok(SecurityDecision::Deny) // Test implementation denies cross-boundary access
            }
        }
    }

    #[tokio::test]
    #[allow(deprecated)]
    async fn test_authentication_success() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let provider = MockSecurityProvider::new();
        let credentials = Credentials {
            username: "test_user".to_string(),
            password: "test_pass".to_string(),
            mfa_token: None,
            client_info: None,
        };

        let result = provider.authenticate(&credentials).await;
        assert!(result.is_ok());

        let token = result?;
        assert_eq!(token.token, "valid_token_123");
        assert_eq!(token.permissions, vec!["read", "write"]);
        Ok(())
    }

    #[tokio::test]
    #[allow(deprecated)]
    async fn test_authentication_failure() {
        let provider = MockSecurityProvider::new();
        let credentials = Credentials {
            username: "wrong_user".to_string(),
            password: "wrong_pass".to_string(),
            mfa_token: None,
            client_info: None,
        };

        let result = provider.authenticate(&credentials).await;
        assert!(result.is_err());
    }

    // Note: These tests use deprecated SecurityPrimalProvider API
    // Use CanonicalSecurity trait for new code
    #[tokio::test]
    #[allow(deprecated)]
    async fn test_encryption_decryption() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let provider = MockSecurityProvider::new();
        let data = b"Hello, World!";

        let encrypted = provider
            .encrypt(data, "AES256")
            .await
            .expect("Security operation failed");
        assert_ne!(encrypted.as_slice(), data);

        let decrypted = provider.decrypt(&encrypted, "AES256").await?;
        assert_eq!(decrypted, data);
        Ok(())
    }

    #[tokio::test]
    #[allow(deprecated)]
    async fn test_signing_and_verification() -> std::result::Result<(), Box<dyn std::error::Error>>
    {
        let provider = MockSecurityProvider::new();
        let data = b"test data";

        let signature = provider.sign_data(data).await?;
        assert_eq!(signature.algorithm, "RS256");

        let is_valid = provider.verify_signature(data, &signature).await?;
        assert!(is_valid);

        // Test with different data
        let different_data = b"different data";
        let is_valid = provider
            .verify_signature(different_data, &signature)
            .await?;
        assert!(!is_valid);
        Ok(())
    }

    #[tokio::test]
    #[allow(deprecated)]
    async fn test_key_id() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let provider = MockSecurityProvider::new();
        let key_id = provider.get_key_id().await?;
        assert_eq!(key_id, "test-key-123");
        Ok(())
    }

    #[tokio::test]
    #[allow(deprecated)]
    async fn test_hash_data() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let provider = MockSecurityProvider::new();
        let data = b"test data";

        let hash = provider.hash_data(data, "SHA256").await?;
        assert_ne!(hash, data);

        // Test unsupported algorithm
        let result = provider.hash_data(data, "MD5").await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    #[allow(deprecated)] // Testing deprecated API for backwards compatibility
    async fn test_generate_random() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let provider = MockSecurityProvider::new();

        let random_data = provider.generate_random(32).await?;
        assert_eq!(random_data.len(), 32);

        // Generate another set and ensure they're different (very high probability)
        let random_data2 = provider.generate_random(32).await?;
        assert_ne!(random_data, random_data2);
        Ok(())
    }

    #[tokio::test]
    #[allow(deprecated)] // Testing deprecated API for backwards compatibility
    async fn test_derive_key() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let provider = MockSecurityProvider::new();
        let password = "test_password";
        let salt = b"salt123";
        let iterations = 1000;

        let key = provider.derive_key(password, salt, iterations).await?;
        assert!(!key.is_empty());

        // Same inputs should produce same key
        let key2 = provider.derive_key(password, salt, iterations).await?;
        assert_eq!(key, key2);

        // Different inputs should produce different keys
        let key3 = provider
            .derive_key("different_password", salt, iterations)
            .await?;
        assert_ne!(key, key3);
        Ok(())
    }

    #[tokio::test]
    #[allow(deprecated)] // Testing deprecated API for backwards compatibility
    async fn test_session_management() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let provider = MockSecurityProvider::new();
        let user_id = "user123";
        let permissions = vec!["read".to_string(), "write".to_string()];

        let session_token = provider.create_session(user_id, permissions).await?;
        assert!(session_token.contains("user123"));
        assert!(session_token.contains("read,write"));

        let decision = provider.validate_session(&session_token).await?;
        assert_eq!(decision, SecurityDecision::Allow);

        // Test rate limiting
        let decision = provider.validate_session("rate_limited").await?;
        assert_eq!(decision, SecurityDecision::RateLimit { retry_after: 60 });

        // Test invalid session
        let decision = provider.validate_session("invalid").await?;
        assert_eq!(decision, SecurityDecision::Deny);
        Ok(())
    }

    #[test]
    fn test_security_decision_serialization() -> std::result::Result<(), Box<dyn std::error::Error>>
    {
        let decision = SecurityDecision::Allow;
        let serialized = serde_json::to_string(&decision)?;
        let deserialized: SecurityDecision = serde_json::from_str(&serialized)?;
        assert_eq!(decision, deserialized);

        let rate_limit = SecurityDecision::RateLimit { retry_after: 300 };
        let serialized = serde_json::to_string(&rate_limit)?;
        let deserialized: SecurityDecision = serde_json::from_str(&serialized)?;
        assert_eq!(rate_limit, deserialized);
        Ok(())
    }

    #[test]
    fn test_credentials_serialization() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let credentials = Credentials {
            username: "test".to_string(),
            password: "pass".to_string(),
            mfa_token: Some("123456".to_string()),
            client_info: Some("mobile_app".to_string()),
        };

        let serialized = serde_json::to_string(&credentials)?;
        let deserialized: Credentials = serde_json::from_str(&serialized)?;
        assert_eq!(credentials.username, deserialized.username);
        assert_eq!(credentials.password, deserialized.password);
        assert_eq!(credentials.mfa_token, deserialized.mfa_token);
        assert_eq!(credentials.client_info, deserialized.client_info);
        Ok(())
    }

    #[test]
    fn test_auth_token_serialization() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let token = AuthToken {
            token: "test_token".to_string(),
            expires_at: SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1000),
            permissions: vec!["read".to_string()],
        };

        let serialized = serde_json::to_string(&token)?;
        let deserialized: AuthToken = serde_json::from_str(&serialized)?;
        assert_eq!(token.token, deserialized.token);
        assert_eq!(token.permissions, deserialized.permissions);
        Ok(())
    }

    #[test]
    fn test_signature_serialization() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let signature = Signature {
            algorithm: "RS256".to_string(),
            signature: vec![1, 2, 3, 4],
            key_id: Some("test-key".to_string()),
        };

        let serialized = serde_json::to_string(&signature)?;
        let deserialized: Signature = serde_json::from_str(&serialized)?;
        assert_eq!(signature.algorithm, deserialized.algorithm);
        assert_eq!(signature.signature, deserialized.signature);
        Ok(())
    }

    #[tokio::test]
    #[allow(deprecated)] // Testing deprecated API for backwards compatibility
    async fn test_boundary_access_evaluation() -> Result<()> {
        let provider = MockSecurityProvider::new();

        // Same source and destination should be allowed
        let decision = provider
            .evaluate_boundary_access("service_a", "service_a")
            .await?;
        assert_eq!(decision, SecurityDecision::Allow);

        // Cross-boundary operations should be denied in test mock implementation
        let decision = provider
            .evaluate_boundary_access("service_a", "service_b")
            .await?;
        assert_eq!(decision, SecurityDecision::Deny);

        // Different cross-service operations should also be denied in mock implementation
        let decision = provider
            .evaluate_boundary_access("service_c", "service_d")
            .await?;
        assert_eq!(decision, SecurityDecision::Deny);

        // Additional cross-boundary operations should also be denied
        let decision = provider
            .evaluate_boundary_access("service_x", "service_y")
            .await?;
        assert_eq!(decision, SecurityDecision::Deny);
        Ok(())
    }
}
