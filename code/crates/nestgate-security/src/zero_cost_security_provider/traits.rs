// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::types::{ZeroCostAuthToken, ZeroCostCredentials, ZeroCostSignature};
/// **ZERO-COST SECURITY PROVIDER TRAITS**
///
/// This module contains the core security provider trait definitions
/// using zero-cost abstractions for maximum performance.
///
use nestgate_types::Result;
// ==================== SECTION ====================

/// **Zero-cost security provider trait**
///
/// High-performance security provider using zero-cost abstractions:
/// - Native async methods (no Future boxing)
/// - Compile-time specialization through const generics
/// - Direct method dispatch (no vtable overhead)
/// - Memory-efficient security operations
///
/// # Deprecation & Migration
///
/// **DEPRECATED**: Zero-cost patterns now integrated into canonical `SecurityProvider`
///
/// **Old code**:
/// ```rust,ignore
/// impl ZeroCostSecurityProvider for MyProvider {
///     type Config = MyConfig;
///     // ...
/// }
/// ```
///
/// **New code**:
/// ```text
/// impl SecurityProvider for MyProvider {
///     // No Config type needed - passed to methods directly
///     // Native async (RPITIT) provides zero-cost abstraction
/// }
/// ```
///
/// The canonical `CanonicalSecurity` trait includes all zero-cost optimizations
/// through native async (`impl Future`) without the complexity of associated types.
///
/// **Timeline**: Deprecated v0.11.3 (Nov 2025), Remove v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.3",
    note = "Use nestgate_core::traits::canonical::CanonicalSecurity instead"
)]
/// `ZeroCostSecurityProvider` trait
pub trait ZeroCostSecurityProvider: Send + Sync + 'static {
    /// Security provider configuration type
    type Config: Clone + Send + Sync + 'static;
    /// Security health information type
    type Health: Clone + Send + Sync + 'static;

    /// Security metrics type
    type Metrics: Clone + Send + Sync + 'static;

    // ==================== AUTHENTICATION OPERATIONS (Native Async) ====================

    /// Authenticate user credentials - native async, no boxing overhead
    fn authenticate(
        &self,
        credentials: &ZeroCostCredentials,
    ) -> impl std::future::Future<Output = Result<ZeroCostAuthToken>> + Send;

    /// Validate authentication token - native async
    fn validate_token(&self, token: &str)
    -> impl std::future::Future<Output = Result<bool>> + Send;

    /// Refresh authentication token - native async
    fn refresh_token(
        &self,
        token: &str,
    ) -> impl std::future::Future<Output = Result<ZeroCostAuthToken>> + Send;

    /// Revoke authentication token - native async
    fn revoke_token(&self, token: &str) -> impl std::future::Future<Output = Result<()>> + Send;

    // ==================== ENCRYPTION OPERATIONS (Native Async) ====================

    /// Encrypt data - native async
    fn encrypt(
        &self,
        data: &[u8],
        algorithm: &str,
    ) -> impl std::future::Future<Output = Result<Vec<u8>>> + Send;

    /// Decrypt data - native async
    fn decrypt(
        &self,
        encrypted: &[u8],
        algorithm: &str,
    ) -> impl std::future::Future<Output = Result<Vec<u8>>> + Send;

    // ==================== SIGNING OPERATIONS (Native Async) ====================

    /// Sign data - native async
    fn sign_data(
        &self,
        data: &[u8],
    ) -> impl std::future::Future<Output = Result<ZeroCostSignature>> + Send;

    /// Verify signature - native async
    fn verify_signature(
        &self,
        data: &[u8],
        signature: &ZeroCostSignature,
    ) -> impl std::future::Future<Output = Result<bool>> + Send;

    // ==================== PROVIDER MANAGEMENT (Direct Access) ====================

    /// Get provider key ID - direct method call
    fn get_key_id(&self) -> String;

    /// Get supported algorithms - direct method call
    fn supported_algorithms(&self) -> Vec<String>;

    /// Check if algorithm is supported - direct method call
    fn supports_algorithm(&self, algorithm: &str) -> bool;

    /// Get provider health - native async
    fn health_check(&self) -> impl std::future::Future<Output = Self::Health> + Send;

    /// Get security metrics - native async
    fn get_metrics(&self) -> impl std::future::Future<Output = Self::Metrics> + Send;

    // ==================== CONFIGURATION (Direct Access) ====================

    /// Get current configuration - direct access
    fn current_config(&self) -> &Self::Config;

    /// Update configuration - native async
    fn update_config(
        &mut self,
        config: Self::Config,
    ) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Validate configuration - native async with default implementation
    fn validate_config(
        &self,
        _config: &Self::Config,
    ) -> impl std::future::Future<Output = Result<()>> + Send {
        async move {
            // Default implementation accepts all configs
            // Override in implementations that need validation
            Ok(())
        }
    }
}

/// **Authentication provider trait**
/// Specialized trait for authentication operations
pub trait AuthenticationProvider: Send + Sync {
    /// Authenticate user with credentials
    fn authenticate(
        &self,
        credentials: &ZeroCostCredentials,
    ) -> impl std::future::Future<Output = Result<ZeroCostAuthToken>> + Send;
    /// Validate token
    fn validate_token(&self, token: &str)
    -> impl std::future::Future<Output = Result<bool>> + Send;

    /// Refresh token
    fn refresh_token(
        &self,
        token: &str,
    ) -> impl std::future::Future<Output = Result<ZeroCostAuthToken>> + Send;

    /// Revoke token
    fn revoke_token(&self, token: &str) -> impl std::future::Future<Output = Result<()>> + Send;
}

/// **Encryption provider trait**
/// Specialized trait for encryption operations
pub trait EncryptionProvider: Send + Sync {
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

    /// Get supported encryption algorithms
    fn supported_algorithms(&self) -> Vec<String>;

    /// Check if algorithm is supported
    fn supports_algorithm(&self, algorithm: &str) -> bool {
        self.supported_algorithms().contains(&algorithm.to_string())
    }
}

/// **Signing provider trait**
/// Specialized trait for digital signing operations
pub trait SigningProvider: Send + Sync {
    /// Sign data
    fn sign_data(
        &self,
        data: &[u8],
    ) -> impl std::future::Future<Output = Result<ZeroCostSignature>> + Send;
    /// Verify signature
    fn verify_signature(
        &self,
        data: &[u8],
        signature: &ZeroCostSignature,
    ) -> impl std::future::Future<Output = Result<bool>> + Send;

    /// Get signing key ID
    fn get_key_id(&self) -> String;

    /// Get supported signing algorithms
    fn supported_algorithms(&self) -> Vec<String>;
}

/// **Security health provider trait**
/// Specialized trait for security health monitoring
/// **DEPRECATED**: Health monitoring integrated into canonical security
#[deprecated(
    since = "0.9.0",
    note = "Use nestgate_core::traits::canonical::CanonicalSecurity health_check method"
)]
/// `SecurityHealthProvider` trait
pub trait SecurityHealthProvider: Send + Sync {
    /// Health information type
    type Health: Clone + Send + Sync + 'static;
    /// Get security health status
    fn health_check(&self) -> impl std::future::Future<Output = Self::Health> + Send;

    /// Check if security provider is healthy
    fn is_healthy(&self) -> impl std::future::Future<Output = bool> + Send;
}

/// **Security metrics provider trait**
/// Specialized trait for security metrics collection
/// **DEPRECATED**: Metrics integrated into canonical security
#[deprecated(
    since = "0.9.0",
    note = "Use nestgate_core::traits::canonical::CanonicalSecurity metrics methods"
)]
/// `SecurityMetricsProvider` trait
pub trait SecurityMetricsProvider: Send + Sync {
    /// Metrics type
    type Metrics: Clone + Send + Sync + 'static;
    /// Get security metrics
    fn get_metrics(&self) -> impl std::future::Future<Output = Self::Metrics> + Send;

    /// Reset metrics
    fn reset_metrics(&self) -> impl std::future::Future<Output = Result<()>> + Send;
}

#[cfg(test)]
mod tests {
    use super::*;
    // Remove unused imports
    // use crate::zero_cost_security_provider::types::AuthMethod;
    // use std::collections::HashMap;

    // Mock implementation for testing
    struct MockSecurityProvider {
        config: String,
    }

    impl ZeroCostSecurityProvider for MockSecurityProvider {
        /// Type alias for Config
        type Config = String;
        /// Type alias for Health
        type Health = bool;
        /// Type alias for Metrics
        type Metrics = u64;

        /// Authenticate
        async fn authenticate(
            &self,
            _credentials: &ZeroCostCredentials,
        ) -> Result<ZeroCostAuthToken> {
            Ok(ZeroCostAuthToken::new(
                "mock-token".to_string(),
                "user123".to_string(),
                vec!["read".to_string()],
                std::time::Duration::from_secs(3600),
            ))
        }

        /// Validates  Token
        async fn validate_token(&self, _token: &str) -> Result<bool> {
            Ok(true)
        }

        /// Refresh Token
        async fn refresh_token(&self, _token: &str) -> Result<ZeroCostAuthToken> {
            Ok(ZeroCostAuthToken::new(
                "refreshed-token".to_string(),
                "user123".to_string(),
                vec!["read".to_string()],
                std::time::Duration::from_secs(3600),
            ))
        }

        /// Revoke Token
        async fn revoke_token(&self, _token: &str) -> Result<()> {
            Ok(())
        }

        /// Encrypt
        async fn encrypt(&self, data: &[u8], _algorithm: &str) -> Result<Vec<u8>> {
            Ok(data.to_vec()) // Mock encryption
        }

        /// Decrypt
        async fn decrypt(&self, encrypted: &[u8], _algorithm: &str) -> Result<Vec<u8>> {
            Ok(encrypted.to_vec()) // Mock decryption
        }

        /// Sign Data
        async fn sign_data(&self, _data: &[u8]) -> Result<ZeroCostSignature> {
            Ok(ZeroCostSignature::new(
                "ECDSA-P256".to_string(),
                "mock-signature".to_string(),
                "key123".to_string(),
            ))
        }

        /// Verify Signature
        async fn verify_signature(
            &self,
            _data: &[u8],
            _signature: &ZeroCostSignature,
        ) -> Result<bool> {
            Ok(true)
        }

        /// Gets Key Id
        fn get_key_id(&self) -> String {
            "mock-key-123".to_string()
        }

        /// Supported Algorithms
        fn supported_algorithms(&self) -> Vec<String> {
            vec!["AES-256-GCM".to_string(), "ECDSA-P256".to_string()]
        }

        /// Supports Algorithm
        fn supports_algorithm(&self, algorithm: &str) -> bool {
            self.supported_algorithms().contains(&algorithm.to_string())
        }

        /// Health Check
        async fn health_check(&self) -> Self::Health {
            true
        }

        /// Gets Metrics
        async fn get_metrics(&self) -> Self::Metrics {
            42
        }

        /// Current Config
        fn current_config(&self) -> &Self::Config {
            &self.config
        }

        /// Updates  Config
        async fn update_config(&mut self, _config: Self::Config) -> Result<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_mock_security_provider() -> Result<()> {
        let provider = MockSecurityProvider {
            config: "mock-config".to_string(),
        };

        // Test authentication - using canonical error handling
        let credentials =
            ZeroCostCredentials::new_password("testuser".to_string(), "testpass".to_string());
        let token = provider.authenticate(&credentials).await?;
        assert_eq!(token.user_id, "user123");

        // Test token validation - using canonical error handling
        assert!(provider.validate_token("test-token").await?);

        // Test encryption - using canonical error handling
        let data = b"test data";
        let encrypted = provider.encrypt(data, "AES-256-GCM").await?;
        let decrypted = provider.decrypt(&encrypted, "AES-256-GCM").await?;
        assert_eq!(data, decrypted.as_slice());

        // Test signing - using canonical error handling
        let signature = provider.sign_data(data).await?;
        assert!(provider.verify_signature(data, &signature).await?);

        // Test metadata
        assert_eq!(provider.get_key_id(), "mock-key-123");
        assert!(provider.supports_algorithm("AES-256-GCM"));
        assert!(!provider.supports_algorithm("UNKNOWN"));

        // Test health and metrics
        assert!(provider.health_check().await);
        assert_eq!(provider.get_metrics().await, 42);

        Ok(())
    }
}
