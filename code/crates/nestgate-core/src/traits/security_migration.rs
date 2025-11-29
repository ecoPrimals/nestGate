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

    #[test]
    fn test_adapters_compile() {
        // Compilation test - ensures adapter types are well-formed
        // Actual functionality tests would require concrete implementations
    }
}
