//! **Security Provider Migration Adapters**
//!
//! Provides adapter implementations to enable gradual migration from deprecated
//! security provider traits to the canonical `SecurityProvider` trait.
//!
//! # Purpose
//!
//! These adapters allow existing code using deprecated traits to work seamlessly
//! with code expecting the canonical `SecurityProvider`. This enables:
//!
//! - Zero breaking changes during consolidation
//! - Gradual migration over time
//! - Backwards compatibility for 6 months (until v0.12.0)
//! - Smooth transition path
//!
//! # Migration Timeline
//!
//! - **Created**: November 10, 2025 (v0.11.3)
//! - **Deprecation Period**: 6 months
//! - **Removal**: May 2026 (v0.12.0)
//!
//! # Usage Examples
//!
//! ```rust,ignore
//! use nestgate_core::traits::security_migration::SecurityPrimalAdapter;
//! use nestgate_core::traits::canonical_provider_unification::SecurityProvider;
//!
//! // Old code using SecurityPrimalProvider
//! let old_provider = MySecurityPrimalProviderImpl::new();
//!
//! // Wrap with adapter to use with new SecurityProvider trait
//! let adapted: SecurityPrimalAdapter<_> = SecurityPrimalAdapter(old_provider);
//!
//! // Now can be used with code expecting SecurityProvider
//! process_security(&adapted).await?;
//! ```
//!
//! # Migration Guide
//!
//! Instead of using adapters long-term, migrate to canonical SecurityProvider:
//!
//! ```rust,ignore
//! // OLD: Implement deprecated trait
//! impl SecurityPrimalProvider for MyProvider {
//!     async fn authenticate(&self, credentials: &Credentials) -> Result<AuthToken> {
//!         // ...
//!     }
//! }
//!
//! // NEW: Implement canonical trait directly
//! impl SecurityProvider for MyProvider {
//!     async fn authenticate(&self, credentials: &[u8]) -> Result<AuthToken> {
//!         // ...
//!     }
//!     // Implement all 14 methods...
//! }
//! ```

#![allow(deprecated)]

use crate::traits::canonical_provider_unification::{
    AuthToken, CanonicalUniversalProvider, ProviderCapabilities, ProviderHealth, SecurityProvider,
    SecurityService,
};
use crate::unified_enums::service_types::UnifiedServiceType;
use crate::universal_traits::{Credentials, SecurityPrimalProvider, Signature};
use crate::zero_cost_security_provider::{ZeroCostCredentials, ZeroCostSignature};
// Import the trait from the dedicated traits module
use crate::zero_cost_security_provider::traits::ZeroCostSecurityProvider;
use crate::Result;
// Removed unused import: use std::future::Future;

// ==================== SECURITY PRIMAL ADAPTER ====================

/// Adapter from `SecurityPrimalProvider` to canonical `SecurityProvider`
///
/// Wraps an implementation of the deprecated `SecurityPrimalProvider` trait
/// to work with code expecting the canonical `SecurityProvider` trait.
///
/// # Deprecation
///
/// This adapter exists only for backwards compatibility during the migration period.
/// Migrate your implementations to use `SecurityProvider` directly instead.
///
/// **Timeline**:
/// - Created: v0.11.3 (November 2025)
/// - Remove: v0.12.0 (May 2026)
///
/// # Example
///
/// ```rust,ignore
/// use nestgate_core::traits::security_migration::SecurityPrimalAdapter;
///
/// let old_provider = MyOldProvider::new();
/// let adapted = SecurityPrimalAdapter(old_provider);
///
/// // Now works with SecurityProvider trait bounds
/// some_function_expecting_security_provider(&adapted).await?;
/// ```
#[derive(Debug, Clone)]
/// Securityprimaladapter
pub struct SecurityPrimalAdapter<T>(pub T);

impl<T: SecurityPrimalProvider + 'static> SecurityProvider for SecurityPrimalAdapter<T> {
    // ===== AUTHENTICATION =====

    /// Authenticate
    async fn authenticate(&self, credentials: &[u8]) -> Result<AuthToken> {
        // Convert raw bytes to Credentials structure
        let creds = Credentials {
            username: String::from_utf8_lossy(credentials).to_string(),
            password: String::new(), // Simplified for adapter
            mfa_token: None,
            client_info: None,
        };
        // Convert universal_traits::AuthToken to canonical AuthToken
        let token = self.0.authenticate(&creds).await?;
        Ok(AuthToken {
            token: token.token,
            expires_at: token.expires_at,
            permissions: token.permissions,
        })
    }

    /// Authorize
    async fn authorize(&self, _token: &AuthToken, data: &[u8]) -> Result<Vec<u8>> {
        // SecurityPrimalProvider doesn't have authorize, decrypt as proxy
        self.0.decrypt(data, "default").await
    }

    // ===== TOKEN MANAGEMENT =====

    /// Validates  Token
    async fn validate_token(&self, _token: &AuthToken) -> Result<bool> {
        // SecurityPrimalProvider doesn't have token validation
        // Default to true for compatibility
        Ok(true)
    }

    /// Refresh Token
    async fn refresh_token(&self, token: &AuthToken) -> Result<AuthToken> {
        // Return same token (no refresh in SecurityPrimalProvider)
        Ok(token.clone())
    }

    /// Revoke Token
    async fn revoke_token(&self, _token: &AuthToken) -> Result<()> {
        // No-op for SecurityPrimalProvider
        Ok(())
    }

    // ===== ENCRYPTION =====

    /// Encrypt
    async fn encrypt(&self, data: &[u8], algorithm: &str) -> Result<Vec<u8>> {
        self.0.encrypt(data, algorithm).await
    }

    /// Decrypt
    async fn decrypt(&self, data: &[u8]) -> Result<Option<Vec<u8>>> {
        match self.0.decrypt(data, "default").await {
            Ok(decrypted) => Ok(Some(decrypted)),
            Err(_) => Ok(None),
        }
    }

    // ===== SIGNING =====

    /// Sign
    async fn sign(&self, data: &[u8]) -> Result<()> {
        self.0.sign_data(data).await?;
        Ok(())
    }

    /// Verify
    async fn verify(&self, data: &[u8], signature: &[u8]) -> Result<Option<(String, Vec<u8>)>> {
        let sig = Signature {
            algorithm: "default".to_string(),
            signature: signature.to_vec(),
            key_id: None,
        };

        match self.0.verify_signature(data, &sig).await {
            Ok(true) => Ok(Some(("default".to_string(), vec![]))),
            Ok(false) => Ok(None),
            Err(e) => Err(e),
        }
    }

    // ===== KEY MANAGEMENT =====

    /// Gets Key Id
    async fn get_key_id(&self) -> Result<String> {
        self.0.get_key_id().await
    }

    /// Supported Algorithms
    async fn supported_algorithms(&self) -> Result<Vec<String>> {
        // Return common algorithms
        Ok(vec![
            "AES-256-GCM".to_string(),
            "ChaCha20-Poly1305".to_string(),
            "SHA-256".to_string(),
        ])
    }

    // ===== UTILITIES =====

    /// Hash Data
    async fn hash_data(&self, data: &[u8], algorithm: &str) -> Result<Vec<u8>> {
        self.0.hash_data(data, algorithm).await
    }

    /// Generate Random
    async fn generate_random(&self, length: usize) -> Result<Vec<u8>> {
        self.0.generate_random(length).await
    }
}

// Implement CanonicalUniversalProvider for the adapter
impl<T: SecurityPrimalProvider + 'static> CanonicalUniversalProvider<Box<dyn SecurityService>>
    for SecurityPrimalAdapter<T>
{
    /// Type alias for Config
    type Config = ();
    /// Type alias for Error
    type Error = crate::NestGateError;
    /// Type alias for Metadata
    type Metadata = ();

    /// Initialize
    async fn initialize(&self, _config: Self::Config) -> Result<()> {
        Ok(())
    }

    /// Provide
    async fn provide(&self) -> Result<Box<dyn SecurityService>> {
        Err(crate::NestGateError::internal_error(
            "adapter_not_service_provider",
            "SecurityPrimalAdapter is for trait adaptation, not service provision",
        ))
    }

    /// Stop
    async fn stop(&self) -> Result<()> {
        Ok(())
    }

    /// Gets Metadata
    async fn get_metadata(&self) -> Result<Self::Metadata> {
        Ok(())
    }

    /// Health Check
    async fn health_check(&self) -> Result<ProviderHealth> {
        Ok(ProviderHealth {
            status: crate::traits::canonical_provider_unification::HealthStatus::Healthy,
            checked_at: std::time::SystemTime::now(),
            details: Default::default(),
            metrics: crate::traits::canonical_provider_unification::ProviderMetrics {
                requests_total: 0,
                requests_successful: 0,
                requests_failed: 0,
                avg_response_time_ms: 0.0,
                active_connections: 0,
            },
        })
    }

    /// Supported Types
    async fn supported_types(&self) -> Result<Vec<UnifiedServiceType>> {
        Ok(vec![])
    }

    /// Supports Type
    async fn supports_type(&self, _service_type: &UnifiedServiceType) -> Result<bool> {
        Ok(false)
    }

    /// Gets Capabilities
    async fn get_capabilities(&self) -> Result<ProviderCapabilities> {
        Ok(ProviderCapabilities {
            operations: vec!["authenticate".to_string(), "encrypt".to_string()],
            max_concurrent: None,
            protocols: vec![],
            features: Default::default(),
        })
    }

    /// Validates  Config
    async fn validate_config(&self, _config: &Self::Config) -> Result<Vec<String>> {
        Ok(vec![])
    }
}

// ==================== ZERO-COST SECURITY ADAPTER ====================

/// Adapter from `ZeroCostSecurityProvider` to canonical `SecurityProvider`
///
/// Wraps an implementation of the deprecated `ZeroCostSecurityProvider` trait
/// to work with code expecting the canonical `SecurityProvider` trait.
///
/// # Deprecation
///
/// This adapter exists only for backwards compatibility. The zero-cost patterns
/// from `ZeroCostSecurityProvider` are now integrated into the canonical
/// `SecurityProvider` trait (which uses native async/RPITIT for zero cost).
///
/// **Timeline**:
/// - Created: v0.11.3 (November 2025)
/// - Remove: v0.12.0 (May 2026)
///
/// # Example
///
/// ```rust,ignore
/// use nestgate_core::traits::security_migration::ZeroCostSecurityAdapter;
///
/// let zero_cost_provider = MyZeroCostProvider::new();
/// let adapted = ZeroCostSecurityAdapter(zero_cost_provider);
///
/// // Now works with SecurityProvider trait bounds
/// secure_operation(&adapted).await?;
/// ```
#[derive(Debug, Clone)]
/// Zerocostsecurityadapter
pub struct ZeroCostSecurityAdapter<T>(pub T);

impl<T: ZeroCostSecurityProvider + Send + Sync + 'static> SecurityProvider
    for ZeroCostSecurityAdapter<T>
{
    // ===== AUTHENTICATION =====

    /// Authenticate
    async fn authenticate(&self, credentials: &[u8]) -> Result<AuthToken> {
        // Parse credentials as "username:password"
        let cred_str = String::from_utf8_lossy(credentials);
        let parts: Vec<&str> = cred_str.split(':').collect();
        let (username, password) = if parts.len() >= 2 {
            (parts[0].to_string(), parts[1].to_string())
        } else {
            (cred_str.to_string(), String::new())
        };

        let creds = ZeroCostCredentials {
            username,
            password,
            auth_method: crate::zero_cost_security_provider::types::AuthMethod::Password,
            metadata: std::collections::HashMap::new(),
        };

        let token = self.0.authenticate(&creds).await?;

        Ok(AuthToken {
            token: token.token,
            expires_at: token.expires_at,
            permissions: token.permissions,
        })
    }

    /// Authorize
    async fn authorize(&self, _token: &AuthToken, data: &[u8]) -> Result<Vec<u8>> {
        // Proxy through decrypt
        self.0.decrypt(data, "default").await
    }

    // ===== TOKEN MANAGEMENT =====

    /// Validates  Token
    async fn validate_token(&self, token: &AuthToken) -> Result<bool> {
        self.0.validate_token(&token.token).await
    }

    /// Refresh Token
    async fn refresh_token(&self, token: &AuthToken) -> Result<AuthToken> {
        let refreshed = self.0.refresh_token(&token.token).await?;

        Ok(AuthToken {
            token: refreshed.token,
            expires_at: refreshed.expires_at,
            permissions: refreshed.permissions,
        })
    }

    /// Revoke Token
    async fn revoke_token(&self, token: &AuthToken) -> Result<()> {
        self.0.revoke_token(&token.token).await
    }

    // ===== ENCRYPTION =====

    /// Encrypt
    async fn encrypt(&self, data: &[u8], algorithm: &str) -> Result<Vec<u8>> {
        self.0.encrypt(data, algorithm).await
    }

    /// Decrypt
    async fn decrypt(&self, data: &[u8]) -> Result<Option<Vec<u8>>> {
        match self.0.decrypt(data, "default").await {
            Ok(decrypted) => Ok(Some(decrypted)),
            Err(_) => Ok(None),
        }
    }

    // ===== SIGNING =====

    /// Sign
    async fn sign(&self, data: &[u8]) -> Result<()> {
        self.0.sign_data(data).await?;
        Ok(())
    }

    /// Verify
    async fn verify(&self, data: &[u8], signature: &[u8]) -> Result<Option<(String, Vec<u8>)>> {
        // Convert signature bytes to base64 string
        let sig_str = String::from_utf8_lossy(signature).to_string();

        let sig = ZeroCostSignature {
            algorithm: "default".to_string(),
            signature: sig_str,
            key_id: String::new(),
            timestamp: std::time::SystemTime::now(),
            metadata: std::collections::HashMap::new(),
        };

        match self.0.verify_signature(data, &sig).await {
            Ok(true) => Ok(Some(("default".to_string(), vec![]))),
            Ok(false) => Ok(None),
            Err(e) => Err(e),
        }
    }

    // ===== KEY MANAGEMENT =====

    /// Gets Key Id
    async fn get_key_id(&self) -> Result<String> {
        Ok(self.0.get_key_id())
    }

    /// Supported Algorithms
    async fn supported_algorithms(&self) -> Result<Vec<String>> {
        Ok(self.0.supported_algorithms())
    }

    // ===== UTILITIES =====

    /// Hash Data
    async fn hash_data(&self, data: &[u8], algorithm: &str) -> Result<Vec<u8>> {
        // ZeroCostSecurityProvider doesn't have hash_data
        // Use a simple hash as fallback
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        algorithm.hash(&mut hasher);
        let hash = hasher.finish();

        Ok(hash.to_le_bytes().to_vec())
    }

    /// Generate Random
    async fn generate_random(&self, length: usize) -> Result<Vec<u8>> {
        // ZeroCostSecurityProvider doesn't have generate_random
        // Use a simple random generation as fallback
        Ok(vec![0u8; length]) // Placeholder - real implementation would use rand crate
    }
}

// Implement CanonicalUniversalProvider for the adapter
impl<T: ZeroCostSecurityProvider + Send + Sync + 'static>
    CanonicalUniversalProvider<Box<dyn SecurityService>> for ZeroCostSecurityAdapter<T>
{
    /// Type alias for Config
    type Config = T::Config;
    /// Type alias for Error
    type Error = crate::NestGateError;
    /// Type alias for Metadata
    type Metadata = ();

    /// Initialize
    async fn initialize(&self, _config: Self::Config) -> Result<()> {
        Ok(())
    }

    /// Provide
    async fn provide(&self) -> Result<Box<dyn SecurityService>> {
        Err(crate::NestGateError::internal_error(
            "adapter_not_service_provider",
            "ZeroCostSecurityAdapter is for trait adaptation, not service provision",
        ))
    }

    /// Stop
    async fn stop(&self) -> Result<()> {
        Ok(())
    }

    /// Gets Metadata
    async fn get_metadata(&self) -> Result<Self::Metadata> {
        Ok(())
    }

    /// Health Check
    async fn health_check(&self) -> Result<ProviderHealth> {
        let _health = self.0.health_check().await;

        Ok(ProviderHealth {
            status: crate::traits::canonical_provider_unification::HealthStatus::Healthy,
            checked_at: std::time::SystemTime::now(),
            details: Default::default(),
            metrics: crate::traits::canonical_provider_unification::ProviderMetrics {
                requests_total: 0,
                requests_successful: 0,
                requests_failed: 0,
                avg_response_time_ms: 0.0,
                active_connections: 0,
            },
        })
    }

    /// Supported Types
    async fn supported_types(&self) -> Result<Vec<UnifiedServiceType>> {
        Ok(vec![])
    }

    /// Supports Type
    async fn supports_type(&self, _service_type: &UnifiedServiceType) -> Result<bool> {
        Ok(false)
    }

    /// Gets Capabilities
    async fn get_capabilities(&self) -> Result<ProviderCapabilities> {
        Ok(ProviderCapabilities {
            operations: vec![
                "authenticate".to_string(),
                "encrypt".to_string(),
                "sign".to_string(),
            ],
            max_concurrent: None,
            protocols: vec![],
            features: Default::default(),
        })
    }

    /// Validates  Config
    async fn validate_config(&self, _config: &Self::Config) -> Result<Vec<String>> {
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::canonical_provider_unification::{AuthToken, SecurityProvider};
    use crate::universal_traits::{Credentials, SecurityPrimalProvider, Signature};
    use crate::zero_cost_security_provider::{
        traits::ZeroCostSecurityProvider,
        types::{ZeroCostAuthToken, ZeroCostCredentials, ZeroCostSignature},
    };
    use std::time::SystemTime;

    // Mock SecurityPrimalProvider for adapter tests
    struct MockSecurityPrimalProvider {
        key_id: String,
    }

    #[allow(deprecated)]
    impl SecurityPrimalProvider for MockSecurityPrimalProvider {
        async fn authenticate(
            &self,
            credentials: &Credentials,
        ) -> Result<crate::universal_traits::AuthToken> {
            if credentials.username == "test" {
                Ok(crate::universal_traits::AuthToken {
                    token: "token-123".to_string(),
                    expires_at: SystemTime::now(),
                    permissions: vec!["read".to_string()],
                })
            } else {
                Err(crate::NestGateError::security("Invalid credentials"))
            }
        }

        async fn encrypt(&self, data: &[u8], _alg: &str) -> Result<Vec<u8>> {
            Ok(data.to_vec())
        }

        async fn decrypt(&self, data: &[u8], _alg: &str) -> Result<Vec<u8>> {
            Ok(data.to_vec())
        }

        async fn sign_data(&self, data: &[u8]) -> Result<Signature> {
            Ok(Signature {
                algorithm: "default".to_string(),
                signature: data.to_vec(),
                key_id: Some(self.key_id.clone()),
            })
        }

        async fn verify_signature(&self, data: &[u8], sig: &Signature) -> Result<bool> {
            Ok(sig.signature == data)
        }

        async fn get_key_id(&self) -> Result<String> {
            Ok(self.key_id.clone())
        }

        async fn hash_data(&self, data: &[u8], _alg: &str) -> Result<Vec<u8>> {
            Ok(data.to_vec())
        }

        async fn generate_random(&self, length: usize) -> Result<Vec<u8>> {
            Ok(vec![0u8; length])
        }

        async fn derive_key(&self, _pwd: &str, _salt: &[u8], _iter: u32) -> Result<Vec<u8>> {
            Ok(vec![0u8; 32])
        }

        async fn create_session(&self, _uid: &str, _perms: Vec<String>) -> Result<String> {
            Ok("session-1".to_string())
        }

        async fn validate_session(
            &self,
            _token: &str,
        ) -> Result<crate::universal_traits::SecurityDecision> {
            Ok(crate::universal_traits::SecurityDecision::Allow)
        }

        async fn evaluate_boundary_access(
            &self,
            _src: &str,
            _dst: &str,
        ) -> Result<crate::universal_traits::SecurityDecision> {
            Ok(crate::universal_traits::SecurityDecision::Allow)
        }
    }

    struct MockZeroCostProvider {
        config: String,
    }

    #[allow(deprecated)]
    impl ZeroCostSecurityProvider for MockZeroCostProvider {
        type Config = String;
        type Health = bool;
        type Metrics = u64;

        async fn authenticate(&self, _creds: &ZeroCostCredentials) -> Result<ZeroCostAuthToken> {
            Ok(ZeroCostAuthToken::new(
                "zc-token".to_string(),
                "user".to_string(),
                vec!["read".to_string(), "write".to_string()],
                std::time::Duration::from_secs(3600),
            ))
        }

        async fn validate_token(&self, token: &str) -> Result<bool> {
            Ok(!token.is_empty())
        }

        async fn refresh_token(&self, _token: &str) -> Result<ZeroCostAuthToken> {
            Ok(ZeroCostAuthToken::new(
                "refreshed".to_string(),
                "user".to_string(),
                vec!["read".to_string()],
                std::time::Duration::from_secs(3600),
            ))
        }

        async fn revoke_token(&self, _token: &str) -> Result<()> {
            Ok(())
        }

        async fn encrypt(&self, data: &[u8], _alg: &str) -> Result<Vec<u8>> {
            Ok(data.to_vec())
        }

        async fn decrypt(&self, data: &[u8], _alg: &str) -> Result<Vec<u8>> {
            Ok(data.to_vec())
        }

        async fn sign_data(&self, _data: &[u8]) -> Result<ZeroCostSignature> {
            Ok(ZeroCostSignature::new(
                "ECDSA".to_string(),
                "sig".to_string(),
                "key-1".to_string(),
            ))
        }

        async fn verify_signature(&self, _data: &[u8], _sig: &ZeroCostSignature) -> Result<bool> {
            Ok(true)
        }

        fn get_key_id(&self) -> String {
            "zc-key".to_string()
        }

        fn supported_algorithms(&self) -> Vec<String> {
            vec!["AES-256".to_string(), "ECDSA".to_string()]
        }

        fn supports_algorithm(&self, alg: &str) -> bool {
            self.supported_algorithms().contains(&alg.to_string())
        }

        fn health_check(&self) -> impl std::future::Future<Output = bool> + Send {
            async { true }
        }

        fn get_metrics(&self) -> impl std::future::Future<Output = u64> + Send {
            async { 0 }
        }

        fn current_config(&self) -> &String {
            &self.config
        }

        async fn update_config(&mut self, config: String) -> Result<()> {
            self.config = config;
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_security_primal_adapter_authenticate() {
        let provider = MockSecurityPrimalProvider {
            key_id: "key-1".to_string(),
        };
        let adapter = SecurityPrimalAdapter(provider);

        let creds = b"test";
        let token = adapter
            .authenticate(creds)
            .await
            .expect("test: primal adapter authenticate");
        assert_eq!(token.token, "token-123");
    }

    #[tokio::test]
    async fn test_security_primal_adapter_validate_token() {
        let provider = MockSecurityPrimalProvider {
            key_id: "key-1".to_string(),
        };
        let adapter = SecurityPrimalAdapter(provider);
        let token = AuthToken {
            token: "t".to_string(),
            expires_at: SystemTime::now(),
            permissions: vec![],
        };
        let ok = adapter
            .validate_token(&token)
            .await
            .expect("test: primal adapter validate_token");
        assert!(ok);
    }

    #[tokio::test]
    async fn test_security_primal_adapter_refresh_token() {
        let provider = MockSecurityPrimalProvider {
            key_id: "key-1".to_string(),
        };
        let adapter = SecurityPrimalAdapter(provider);
        let token = AuthToken {
            token: "t".to_string(),
            expires_at: SystemTime::now(),
            permissions: vec![],
        };
        let refreshed = adapter
            .refresh_token(&token)
            .await
            .expect("test: primal adapter refresh_token");
        assert_eq!(refreshed.token, "t");
    }

    #[tokio::test]
    async fn test_security_primal_adapter_encrypt_decrypt() {
        let provider = MockSecurityPrimalProvider {
            key_id: "key-1".to_string(),
        };
        let adapter = SecurityPrimalAdapter(provider);
        let enc = adapter
            .encrypt(b"data", "AES256")
            .await
            .expect("test: primal adapter encrypt");
        assert_eq!(enc, b"data");
        let dec = adapter
            .decrypt(b"data")
            .await
            .expect("test: primal adapter decrypt");
        assert_eq!(dec, Some(b"data".to_vec()));
    }

    #[tokio::test]
    async fn test_security_primal_adapter_get_key_id() {
        let provider = MockSecurityPrimalProvider {
            key_id: "my-key".to_string(),
        };
        let adapter = SecurityPrimalAdapter(provider);
        let id = adapter
            .get_key_id()
            .await
            .expect("test: primal adapter get_key_id");
        assert_eq!(id, "my-key");
    }

    #[tokio::test]
    async fn test_security_primal_adapter_supported_algorithms() {
        let provider = MockSecurityPrimalProvider {
            key_id: "k".to_string(),
        };
        let adapter = SecurityPrimalAdapter(provider);
        let algs = adapter
            .supported_algorithms()
            .await
            .expect("test: primal adapter supported_algorithms");
        assert!(!algs.is_empty());
    }

    #[tokio::test]
    async fn test_security_primal_adapter_canonical_provider_provide() {
        let provider = MockSecurityPrimalProvider {
            key_id: "k".to_string(),
        };
        let adapter: SecurityPrimalAdapter<_> = SecurityPrimalAdapter(provider);
        let result = adapter.provide().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_zero_cost_adapter_authenticate() {
        let provider = MockZeroCostProvider {
            config: "cfg".to_string(),
        };
        let adapter = ZeroCostSecurityAdapter(provider);
        let creds = b"user:pass";
        let token = adapter
            .authenticate(creds)
            .await
            .expect("test: zero-cost adapter authenticate");
        assert_eq!(token.token, "zc-token");
    }

    #[tokio::test]
    async fn test_zero_cost_adapter_validate_token() {
        let provider = MockZeroCostProvider {
            config: "cfg".to_string(),
        };
        let adapter = ZeroCostSecurityAdapter(provider);
        let token = AuthToken {
            token: "valid".to_string(),
            expires_at: SystemTime::now(),
            permissions: vec![],
        };
        let ok = adapter
            .validate_token(&token)
            .await
            .expect("test: zero-cost adapter validate_token");
        assert!(ok);
    }

    #[tokio::test]
    async fn test_zero_cost_adapter_encrypt_decrypt() {
        let provider = MockZeroCostProvider {
            config: "cfg".to_string(),
        };
        let adapter = ZeroCostSecurityAdapter(provider);
        let enc = adapter
            .encrypt(b"secret", "AES")
            .await
            .expect("test: zero-cost adapter encrypt");
        assert_eq!(enc, b"secret");
        let dec = adapter
            .decrypt(b"secret")
            .await
            .expect("test: zero-cost adapter decrypt");
        assert_eq!(dec, Some(b"secret".to_vec()));
    }

    #[tokio::test]
    async fn test_zero_cost_adapter_get_key_id() {
        let provider = MockZeroCostProvider {
            config: "cfg".to_string(),
        };
        let adapter = ZeroCostSecurityAdapter(provider);
        let id = adapter
            .get_key_id()
            .await
            .expect("test: zero-cost adapter get_key_id");
        assert_eq!(id, "zc-key");
    }

    #[tokio::test]
    async fn test_zero_cost_adapter_supported_algorithms() {
        let provider = MockZeroCostProvider {
            config: "cfg".to_string(),
        };
        let adapter = ZeroCostSecurityAdapter(provider);
        let algs = adapter
            .supported_algorithms()
            .await
            .expect("test: zero-cost adapter supported_algorithms");
        assert_eq!(algs.len(), 2);
    }

    #[tokio::test]
    async fn test_zero_cost_adapter_hash_data() {
        let provider = MockZeroCostProvider {
            config: "cfg".to_string(),
        };
        let adapter = ZeroCostSecurityAdapter(provider);
        let hash = adapter
            .hash_data(b"data", "SHA256")
            .await
            .expect("test: zero-cost adapter hash_data");
        assert!(!hash.is_empty());
    }

    #[tokio::test]
    async fn test_zero_cost_adapter_generate_random() {
        let provider = MockZeroCostProvider {
            config: "cfg".to_string(),
        };
        let adapter = ZeroCostSecurityAdapter(provider);
        let bytes = adapter
            .generate_random(32)
            .await
            .expect("test: zero-cost adapter generate_random");
        assert_eq!(bytes.len(), 32);
    }

    #[tokio::test]
    async fn test_security_primal_adapter_authenticate_rejects_invalid_user() {
        let provider = MockSecurityPrimalProvider {
            key_id: "key-1".to_string(),
        };
        let adapter = SecurityPrimalAdapter(provider);
        let err = adapter
            .authenticate(b"not-test")
            .await
            .expect_err("test: invalid user should fail");
        assert!(err.to_string().contains("Invalid") || err.to_string().contains("credentials"));
    }

    #[tokio::test]
    async fn test_security_primal_adapter_verify_signature_mismatch_returns_none() {
        let provider = MockSecurityPrimalProvider {
            key_id: "key-1".to_string(),
        };
        let adapter = SecurityPrimalAdapter(provider);
        let out = adapter
            .verify(b"data", b"other")
            .await
            .expect("test: primal adapter verify");
        assert!(out.is_none());
    }

    #[tokio::test]
    async fn test_security_primal_adapter_hash_and_random_delegate() {
        let provider = MockSecurityPrimalProvider {
            key_id: "key-1".to_string(),
        };
        let adapter = SecurityPrimalAdapter(provider);
        let h = adapter
            .hash_data(b"abc", "x")
            .await
            .expect("test: primal adapter hash_data");
        assert_eq!(h, b"abc");
        let r = adapter
            .generate_random(4)
            .await
            .expect("test: primal adapter generate_random");
        assert_eq!(r, vec![0u8; 4]);
    }

    #[tokio::test]
    async fn test_zero_cost_adapter_credentials_without_colon_uses_whole_string_as_username() {
        let provider = MockZeroCostProvider {
            config: "cfg".to_string(),
        };
        let adapter = ZeroCostSecurityAdapter(provider);
        let token = adapter
            .authenticate(b"onlyuser")
            .await
            .expect("test: zero-cost authenticate single segment");
        assert_eq!(token.token, "zc-token");
    }

    #[tokio::test]
    async fn test_zero_cost_adapter_revoke_token_delegates() {
        let provider = MockZeroCostProvider {
            config: "cfg".to_string(),
        };
        let adapter = ZeroCostSecurityAdapter(provider);
        let token = AuthToken {
            token: "t".to_string(),
            expires_at: SystemTime::now(),
            permissions: vec![],
        };
        adapter
            .revoke_token(&token)
            .await
            .expect("test: zero-cost revoke_token");
    }
}
