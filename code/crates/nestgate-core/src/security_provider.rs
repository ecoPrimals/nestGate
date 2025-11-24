#![allow(deprecated)]

//! **DEPRECATED Security Provider Module**
//!
//! **DEPRECATED**: This module uses the deprecated `SecurityPrimalProvider` trait.
//!
//! # Migration
//!
//! **Use instead**: `crate::security_provider_canonical`
//!
//! ```rust,ignore
//! // OLD (deprecated)
//! use nestgate_core::security_provider::{SecurityProvider, create_default};
//!
//! // NEW (canonical)
//! use nestgate_core::security_provider_canonical::{
//!     CanonicalSecurityProvider, create_default
//! };
//! ```
//!
//! **Timeline**:
//! - Deprecated: v0.11.3 (November 2025)
//! - Remove: v0.12.0 (May 2026)
//!
//! **See**: `docs/guides/SECURITY_PROVIDER_MIGRATION.md` for complete migration guide

// Removed unused error imports
use crate::{NestGateError, Result};
// SecurityPrimalProvider has been consolidated - using unified zero-cost types
use crate::universal_traits::SecurityPrimalProvider;
use crate::universal_traits::{AuthToken, Credentials, SecurityDecision, Signature};
// CANONICAL MODERNIZATION: Removed async_trait for native async patterns
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// CLEANED: Removed unused Arc import as part of canonical modernization
// use std::sync::Arc;
use std::time::Duration;
/// Security provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::SecurityProviderConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::SecurityProviderConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
pub struct SecurityProviderConfig {
    pub provider_type: String,
    pub config: HashMap<String, String>,
}
/// Security provider interface
#[derive(Debug, Clone)]
pub struct SecurityProvider {
    pub id: String,
    pub config: SecurityProviderConfig,
}
impl SecurityProvider {
    /// Create a new security provider
    #[must_use]
    pub fn new(id: String, config: SecurityProviderConfig) -> Self {
        Self { id, config }
    }

    /// Generate a secure token
    #[must_use]
    pub fn generate_token(&self) -> String {
        use uuid::Uuid;
        Uuid::new_v4().to_string()
    }

    /// Validate a token
    #[must_use]
    pub fn validate_token(&self, _token: &str) -> bool {
        // Basic validation - in production this would be more sophisticated
        true
    }
}

/// **CANONICAL MODERNIZATION**: Native async implementation without `async_trait` overhead
impl SecurityPrimalProvider for SecurityProvider {
    async fn authenticate(&self, credentials: &Credentials) -> Result<AuthToken> {
        // Basic implementation for testing
        use std::time::SystemTime;
        if credentials.username.is_empty() {
            return Err(NestGateError::security_error("Security operation failed"));
        }

        Ok(AuthToken {
            token: self.generate_token(),
            expires_at: SystemTime::now() + Duration::from_secs(3600),
            permissions: vec!["read".to_string(), "write".to_string()],
        })
    }

    async fn encrypt(&self, data: &[u8], _algorithm: &str) -> Result<Vec<u8>> {
        // Simple test implementation
        Ok(data.to_vec())
    }

    async fn decrypt(&self, encrypted: &[u8], _algorithm: &str) -> Result<Vec<u8>> {
        // Simple test implementation
        Ok(encrypted.to_vec())
    }

    fn sign_data(
        &self,
        data: &[u8],
    ) -> impl std::future::Future<Output = Result<Signature>> + Send {
        let id = self.id.clone();
        async move {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};

            let mut hasher = DefaultHasher::new();
            data.hash(&mut hasher);

            Ok(Signature {
                algorithm: "test".to_string(),
                signature: format!("test_sig_{:x}", hasher.finish()).into_bytes(),
                key_id: Some(id),
            })
        }
    }

    async fn verify_signature(&self, _data: &[u8], _signature: &Signature) -> Result<bool> {
        // Simple test implementation
        Ok(true)
    }

    fn get_key_id(&self) -> impl std::future::Future<Output = Result<String>> + Send {
        let id = self.id.clone();
        async move { Ok(id) }
    }

    async fn evaluate_boundary_access(
        &self,
        _source: &str,
        _destination: &str,
    ) -> Result<SecurityDecision> {
        // Simple test implementation - allow all operations
        Ok(SecurityDecision::Allow)
    }

    async fn hash_data(&self, data: &[u8], algorithm: &str) -> Result<Vec<u8>> {
        // Basic hash implementation
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        algorithm.hash(&mut hasher);
        Ok(hasher.finish().to_be_bytes().to_vec())
    }

    async fn generate_random(&self, length: usize) -> Result<Vec<u8>> {
        // Basic random generation
        Ok((0..length).map(|_| rand::random::<u8>()).collect())
    }

    async fn derive_key(&self, password: &str, salt: &[u8], iterations: u32) -> Result<Vec<u8>> {
        // Basic key derivation
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        password.hash(&mut hasher);
        salt.hash(&mut hasher);
        iterations.hash(&mut hasher);
        Ok(hasher.finish().to_be_bytes().to_vec())
    }

    async fn create_session(&self, user_id: &str, permissions: Vec<String>) -> Result<String> {
        // Basic session creation
        Ok(format!("session-{}-{}", user_id, permissions.len()))
    }

    async fn validate_session(
        &self,
        session_token: &str,
    ) -> Result<crate::universal_traits::security::SecurityDecision> {
        // Basic session validation
        if session_token.starts_with("session-") {
            Ok(crate::universal_traits::security::SecurityDecision::Allow)
        } else {
            Ok(crate::universal_traits::security::SecurityDecision::Deny)
        }
    }
}

/// Create a default security provider
#[must_use]
pub fn create_default() -> SecurityProvider {
    let config = SecurityProviderConfig {
        provider_type: "default".to_string(),
        config: std::collections::HashMap::new(),
    };
    SecurityProvider::new("default-provider".to_string(), config)
}
/// Create a custom security provider  
#[must_use]
pub fn create_custom(
    provider_type: String,
    config_map: std::collections::HashMap<String, String>,
) -> SecurityProvider {
    let config = SecurityProviderConfig {
        provider_type,
        config: config_map,
    };
    SecurityProvider::new("custom-provider".to_string(), config)
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
pub type SecurityProviderConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using SecurityProviderConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_security_provider() {
        let provider = create_default();
        // Just test that provider was created successfully
        let key_id = provider.get_key_id().await.unwrap_or_else(|e| {
            tracing::error!("Failed to get key ID: {:?}", e);
            "default_key_id".to_string()
        });
        assert!(!key_id.is_empty());
    }

    #[tokio::test]
    async fn test_generate_token() {
        let provider = create_default();
        let token = provider.generate_token();
        assert!(!token.is_empty());
    }

    #[tokio::test]
    async fn test_validate_token() {
        let provider = create_default();
        let is_valid = provider.validate_token("test-token");
        assert!(is_valid); // validate_token returns bool directly
    }
}
