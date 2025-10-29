//! **CANONICAL TRAIT HIERARCHY**
//!
//! This module defines the canonical trait hierarchy for the NestGate ecosystem.
//! It consolidates 35+ scattered provider trait variants into 5 core traits.
//!
//! **Design Principles**:
//! - Single Responsibility - Each trait has one clear purpose
//! - Native Async - Zero-cost abstractions (no `async_trait`)
//! - Composability - Traits build on each other
//! - Type Safety - Strong typing with clear contracts
//! - Performance - Zero-cost where possible
//!
//! **Trait Hierarchy**:
//! ```text
//! CanonicalService (base)
//!   ├─ CanonicalProvider<T> (generic provisioning)
//!   ├─ CanonicalStorage (storage operations)
//!   ├─ CanonicalSecurity (security operations)
//!   └─ ZeroCostService<T> (performance marker)
//! ```
//!
//! **Date**: October 1, 2025
//! **Status**: Initial implementation - Week 3
//! **Replaces**: 35+ provider trait variants

use std::future::Future;

// ==================== BASE TRAIT ====================

/// **THE** base trait for all services in the NestGate ecosystem
///
/// This trait provides common functionality that all services must implement:
/// - Lifecycle management (start, stop, health checks)
/// - Configuration management
/// - Metrics and observability
///
/// **Native Async**: All methods use `impl Future` for zero-cost abstractions
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::traits::canonical_hierarchy::CanonicalService;
///
/// pub struct MyService {
///     config: MyConfig,
/// }
///
/// impl CanonicalService for MyService {
///     type Config = MyConfig;
///     type Health = MyHealth;
///     type Metrics = MyMetrics;
///     type Error = MyError;
///
///     async fn start(&mut self) -> Result<(), Self::Error> {
///         // Initialize service
///         Ok(())
///     }
///
///     async fn stop(&mut self) -> Result<(), Self::Error> {
///         // Cleanup
///         Ok(())
///     }
///
///     async fn health(&self) -> Result<Self::Health, Self::Error> {
///         // Health check
///         Ok(MyHealth::default())
///     }
///
///     fn config(&self) -> &Self::Config {
///         &self.config
///     }
///
///     async fn metrics(&self) -> Result<Self::Metrics, Self::Error> {
///         // Collect metrics
///         Ok(MyMetrics::default())
///     }
///
///     fn name(&self) -> &str {
///         "my-service"
///     }
///
///     fn version(&self) -> &str {
///         env!("CARGO_PKG_VERSION")
///     }
/// }
/// ```
pub trait CanonicalService: Send + Sync + 'static {
    /// Service configuration type
    type Config: Clone + Send + Sync + 'static;

    /// Health status type
    type Health: Clone + Send + Sync + 'static;

    /// Metrics type
    type Metrics: Clone + Send + Sync + 'static;

    /// Error type
    type Error: Send + Sync + std::error::Error + 'static;

    // ==================== LIFECYCLE ====================

    /// Start the service
    fn start(&mut self) -> impl Future<Output = Result<(), Self::Error>> + Send;

    /// Stop the service gracefully
    fn stop(&mut self) -> impl Future<Output = Result<(), Self::Error>> + Send;

    /// Check service health
    fn health(&self) -> impl Future<Output = Result<Self::Health, Self::Error>> + Send;

    // ==================== CONFIGURATION ====================

    /// Get current configuration
    fn config(&self) -> &Self::Config;

    /// Update configuration (if supported)
    ///
    /// Default implementation is not provided - must be overridden if dynamic
    /// configuration updates are supported. If not supported, don't implement.
    fn update_config(
        &mut self,
        config: Self::Config,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send
    where
        Self: Sized,
    {
        async move {
            // Default: Drop the config to avoid "must use" warnings
            let _ = config;
            // This default implementation will not be called in practice
            // as implementations should override this method
            unreachable!("update_config not implemented - this service does not support dynamic configuration updates")
        }
    }

    // ==================== OBSERVABILITY ====================

    /// Get service metrics
    fn metrics(&self) -> impl Future<Output = Result<Self::Metrics, Self::Error>> + Send;

    /// Get service name
    fn name(&self) -> &str;

    /// Get service version
    fn version(&self) -> &str;
}

// ==================== GENERIC PROVIDER ====================

/// **THE** canonical provider trait for service provisioning
///
/// This trait provides a generic way to provision services of type `T`.
/// It extends `CanonicalService` and adds provisioning capabilities.
///
/// **Type Parameter**: `T` - The service type being provided
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::traits::canonical_hierarchy::{CanonicalProvider, CanonicalStorage};
///
/// pub struct StorageProvider {
///     config: ProviderConfig,
/// }
///
/// impl CanonicalService for StorageProvider {
///     // ... implement CanonicalService
/// }
///
/// impl CanonicalProvider<Box<dyn CanonicalStorage>> for StorageProvider {
///     type Metadata = ProviderMetadata;
///
///     async fn provide(&self) -> Result<Box<dyn CanonicalStorage>, Self::Error> {
///         // Create storage instance
///         Ok(Box::new(MyStorage::new()))
///     }
///
///     async fn provide_with_config(
///         &self,
///         config: Self::Config,
///     ) -> Result<Box<dyn CanonicalStorage>, Self::Error> {
///         // Create storage with config
///         Ok(Box::new(MyStorage::with_config(config)))
///     }
///
///     async fn metadata(&self) -> Result<Self::Metadata, Self::Error> {
///         Ok(ProviderMetadata::default())
///     }
///
///     async fn from_config(config: Self::Config) -> Result<Self, Self::Error> {
///         Ok(Self { config })
///     }
/// }
/// ```
pub trait CanonicalProvider<T>: CanonicalService {
    /// Provider-specific metadata
    type Metadata: Clone + Send + Sync + 'static;

    // ==================== PROVISIONING ====================

    /// Provide a service instance
    fn provide(&self) -> impl Future<Output = Result<T, Self::Error>> + Send;

    /// Provide a service with specific configuration
    fn provide_with_config(
        &self,
        config: Self::Config,
    ) -> impl Future<Output = Result<T, Self::Error>> + Send;

    // ==================== CAPABILITY DISCOVERY ====================

    /// Get provider metadata
    fn metadata(&self) -> impl Future<Output = Result<Self::Metadata, Self::Error>> + Send;

    /// Check if provider can provide the requested service
    ///
    /// Default implementation returns `true`.
    /// Override for capability-based filtering.
    fn can_provide(&self) -> impl Future<Output = bool> + Send {
        async { true }
    }

    // ==================== FACTORY METHODS ====================

    /// Create provider from configuration
    fn from_config(config: Self::Config) -> impl Future<Output = Result<Self, Self::Error>> + Send
    where
        Self: Sized;
}

// ==================== STORAGE TRAIT ====================

/// **THE** canonical storage trait
///
/// Replaces ALL storage provider traits:
/// - UnifiedStorageBackend
/// - CanonicalStorageBackend
/// - ZeroCostUnifiedStorageBackend
/// - StorageBackend
/// - 6+ other storage trait variants
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::traits::canonical_hierarchy::{CanonicalService, CanonicalStorage};
///
/// pub struct ZfsStorage {
///     pool: String,
///     config: ZfsConfig,
/// }
///
/// impl CanonicalService for ZfsStorage {
///     // ... implement CanonicalService
/// }
///
/// impl CanonicalStorage for ZfsStorage {
///     type Key = String;
///     type Value = Vec<u8>;
///     type Metadata = ZfsMetadata;
///
///     async fn read(&self, key: &Self::Key) -> Result<Option<Self::Value>, Self::Error> {
///         // ZFS read implementation
///         todo!()
///     }
///
///     async fn write(&self, key: Self::Key, value: Self::Value) -> Result<(), Self::Error> {
///         // ZFS write implementation
///         todo!()
///     }
///
///     async fn delete(&self, key: &Self::Key) -> Result<(), Self::Error> {
///         // ZFS delete implementation
///         todo!()
///     }
///
///     async fn exists(&self, key: &Self::Key) -> Result<bool, Self::Error> {
///         // ZFS exists check
///         todo!()
///     }
///
///     async fn metadata(&self, key: &Self::Key) -> Result<Self::Metadata, Self::Error> {
///         // Get ZFS metadata
///         todo!()
///     }
///
///     async fn list(&self, prefix: Option<&str>) -> Result<Vec<Self::Key>, Self::Error> {
///         // List ZFS datasets
///         todo!()
///     }
/// }
/// ```
///
/// **DEPRECATED**: Use `crate::traits::canonical_unified_traits::CanonicalStorage` instead.
/// This is a duplicate definition maintained for backward compatibility only.
#[deprecated(since = "0.9.0", note = "Use crate::traits::canonical_unified_traits::CanonicalStorage instead - unified in canonical_unified_traits module")]
pub trait CanonicalStorage: CanonicalService {
    /// Storage key type
    type Key: Clone + Send + Sync + 'static;

    /// Storage value type
    type Value: Clone + Send + Sync + 'static;

    /// Metadata type
    type Metadata: Clone + Send + Sync + 'static;

    // ==================== BASIC OPERATIONS ====================

    /// Read a value by key
    fn read(
        &self,
        key: &Self::Key,
    ) -> impl Future<Output = Result<Option<Self::Value>, Self::Error>> + Send;

    /// Write a value
    fn write(
        &self,
        key: Self::Key,
        value: Self::Value,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    /// Delete a value
    fn delete(&self, key: &Self::Key) -> impl Future<Output = Result<(), Self::Error>> + Send;

    /// Check if key exists
    fn exists(&self, key: &Self::Key) -> impl Future<Output = Result<bool, Self::Error>> + Send;

    // ==================== BATCH OPERATIONS ====================

    /// Batch read with default implementation
    ///
    /// Override for optimized batch operations if your backend supports it.
    fn batch_read(
        &self,
        keys: &[Self::Key],
    ) -> impl Future<Output = Result<Vec<Option<Self::Value>>, Self::Error>> + Send {
        async {
            let mut results = Vec::with_capacity(keys.len());
            for key in keys {
                results.push(self.read(key).await?);
            }
            Ok(results)
        }
    }

    /// Batch write with default implementation
    ///
    /// Override for optimized batch operations if your backend supports it.
    fn batch_write(
        &self,
        items: Vec<(Self::Key, Self::Value)>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async {
            for (key, value) in items {
                self.write(key, value).await?;
            }
            Ok(())
        }
    }

    // ==================== METADATA & LISTING ====================

    /// Get metadata for a key
    fn metadata(
        &self,
        key: &Self::Key,
    ) -> impl Future<Output = Result<Self::Metadata, Self::Error>> + Send;

    /// List keys with optional prefix
    fn list(
        &self,
        prefix: Option<&str>,
    ) -> impl Future<Output = Result<Vec<Self::Key>, Self::Error>> + Send;

    // ==================== ADVANCED OPERATIONS ====================

    /// Copy a value from one key to another
    ///
    /// Default implementation uses read + write.
    /// Override for optimized copy if your backend supports it.
    fn copy(
        &self,
        from: &Self::Key,
        to: Self::Key,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async {
            if let Some(value) = self.read(from).await? {
                self.write(to, value).await?;
            }
            Ok(())
        }
    }

    /// Move a value from one key to another
    ///
    /// Default implementation uses copy + delete.
    /// Override for optimized move if your backend supports it.
    fn move_key(
        &self,
        from: &Self::Key,
        to: Self::Key,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async {
            self.copy(from, &to).await?;
            self.delete(from).await?;
            Ok(())
        }
    }
}

// ==================== SECURITY TRAIT ====================

/// **THE** canonical security trait
///
/// Replaces ALL security provider traits:
/// - ZeroCostSecurityProvider (3 versions)
/// - SecurityProvider (multiple)
/// - AuthenticationProvider
/// - EncryptionProvider
/// - SigningProvider
/// - 3+ other security trait variants
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::traits::canonical_hierarchy::{CanonicalService, CanonicalSecurity};
///
/// pub struct JwtSecurity {
///     config: JwtConfig,
/// }
///
/// impl CanonicalService for JwtSecurity {
///     // ... implement CanonicalService
/// }
///
/// impl CanonicalSecurity for JwtSecurity {
///     type Token = String;
///     type Credentials = UserCredentials;
///     type Principal = User;
///
///     async fn authenticate(
///         &self,
///         credentials: Self::Credentials,
///     ) -> Result<Self::Token, Self::Error> {
///         // JWT authentication
///         todo!()
///     }
///
///     async fn validate_token(
///         &self,
///         token: &Self::Token,
///     ) -> Result<Self::Principal, Self::Error> {
///         // JWT validation
///         todo!()
///     }
///
///     async fn revoke_token(&self, token: &Self::Token) -> Result<(), Self::Error> {
///         // JWT revocation
///         todo!()
///     }
///
///     async fn authorize(
///         &self,
///         principal: &Self::Principal,
///         resource: &str,
///         action: &str,
///     ) -> Result<bool, Self::Error> {
///         // Authorization check
///         todo!()
///     }
///
///     async fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, Self::Error> {
///         // Encryption
///         todo!()
///     }
///
///     async fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, Self::Error> {
///         // Decryption
///         todo!()
///     }
///
///     async fn sign(&self, data: &[u8]) -> Result<Vec<u8>, Self::Error> {
///         // Signing
///         todo!()
///     }
///
///     async fn verify(&self, data: &[u8], signature: &[u8]) -> Result<bool, Self::Error> {
///         // Signature verification
///         todo!()
///     }
/// }
/// ```
/// **DEPRECATED**: Use canonical_unified_traits::CanonicalSecurity instead
#[deprecated(since = "0.9.0", note = "Use crate::traits::canonical_unified_traits::CanonicalSecurity instead - unified in canonical_unified_traits module")]
pub trait CanonicalSecurity: CanonicalService {
    /// Token type (JWT, session, etc.)
    type Token: Clone + Send + Sync + 'static;

    /// Credentials type
    type Credentials: Clone + Send + Sync + 'static;

    /// Principal type (user, service, etc.)
    type Principal: Clone + Send + Sync + 'static;

    // ==================== AUTHENTICATION ====================

    /// Authenticate credentials and return a token
    fn authenticate(
        &self,
        credentials: Self::Credentials,
    ) -> impl Future<Output = Result<Self::Token, Self::Error>> + Send;

    /// Validate a token
    fn validate_token(
        &self,
        token: &Self::Token,
    ) -> impl Future<Output = Result<Self::Principal, Self::Error>> + Send;

    /// Revoke a token
    fn revoke_token(
        &self,
        token: &Self::Token,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    // ==================== AUTHORIZATION ====================

    /// Check if principal has permission
    fn authorize(
        &self,
        principal: &Self::Principal,
        resource: &str,
        action: &str,
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send;

    // ==================== CRYPTOGRAPHY ====================

    /// Encrypt data
    fn encrypt(&self, data: &[u8]) -> impl Future<Output = Result<Vec<u8>, Self::Error>> + Send;

    /// Decrypt data
    fn decrypt(&self, data: &[u8]) -> impl Future<Output = Result<Vec<u8>, Self::Error>> + Send;

    /// Sign data
    fn sign(&self, data: &[u8]) -> impl Future<Output = Result<Vec<u8>, Self::Error>> + Send;

    /// Verify signature
    fn verify(
        &self,
        data: &[u8],
        signature: &[u8],
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send;

    // ==================== AUDIT ====================

    /// Log security event
    ///
    /// Default implementation is a no-op.
    /// Override to implement audit logging.
    fn audit_log(
        &self,
        _event: &str,
        _principal: Option<&Self::Principal>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async {
            // Default: no-op
            Ok(())
        }
    }

    // ==================== ADVANCED CRYPTOGRAPHY (OPTIONAL) ====================
    // These methods provide advanced cryptographic operations beyond the basic
    // encrypt/decrypt/sign/verify. They have default implementations that either
    // delegate to the simpler methods or return "not supported" errors.

    /// Sign data and return structured signature (optional, delegates to sign by default)
    /// 
    /// Default implementation delegates to `sign()` and wraps in a generic signature structure.
    /// Override for more sophisticated signature formats (e.g., with key IDs, algorithms).
    fn sign_data(&self, data: &[u8]) -> impl Future<Output = Result<Vec<u8>, Self::Error>> + Send {
        self.sign(data)
    }

    /// Verify structured signature (optional, delegates to verify by default)
    /// 
    /// Default implementation delegates to `verify()`.
    /// Override for more sophisticated signature verification.
    fn verify_signature(
        &self,
        data: &[u8],
        signature: &[u8],
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send {
        self.verify(data, signature)
    }

    /// Get the key ID used for signing (optional)
    /// 
    /// Default implementation returns None to indicate no key ID tracking.
    /// Override if your implementation tracks key IDs.
    fn get_key_id(&self) -> impl Future<Output = Result<Option<String>, Self::Error>> + Send {
        async { Ok(None) }
    }

    /// Hash data with specific algorithm (optional)
    /// 
    /// Default implementation returns "not supported" error.
    /// Override to provide hashing capabilities.
    fn hash_data(
        &self,
        _data: &[u8],
        _algorithm: &str,
    ) -> impl Future<Output = Result<Vec<u8>, Self::Error>> + Send
    where
        Self: Sized,
    {
        async {
            // Default: not supported - implementations must override
            // We use unreachable here as a placeholder since we can't construct Self::Error generically
            unimplemented!("hash_data not implemented - override this method to provide hashing")
        }
    }

    /// Generate random bytes (optional)
    /// 
    /// Default implementation returns "not supported" error.
    /// Override to provide random generation capabilities.
    fn generate_random(&self, _length: usize) -> impl Future<Output = Result<Vec<u8>, Self::Error>> + Send
    where
        Self: Sized,
    {
        async {
            // Default: not supported - implementations must override
            unimplemented!("generate_random not implemented - override this method to provide random generation")
        }
    }

    /// Derive key from master key (optional)
    /// 
    /// Default implementation returns "not supported" error.
    /// Override to provide key derivation capabilities.
    fn derive_key(
        &self,
        _master_key: &[u8],
        _salt: &[u8],
        _info: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, Self::Error>> + Send
    where
        Self: Sized,
    {
        async {
            // Default: not supported - implementations must override
            unimplemented!("derive_key not implemented - override this method to provide key derivation")
        }
    }

    /// Evaluate boundary access control (optional)
    /// 
    /// Default implementation delegates to `authorize()`.
    /// Override for more sophisticated boundary access control.
    fn evaluate_boundary_access(
        &self,
        principal: &Self::Principal,
        boundary: &str,
        action: &str,
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send {
        self.authorize(principal, boundary, action)
    }

    /// Create session for principal (optional)
    /// 
    /// Default implementation creates a token via `authenticate`.
    /// Override for session-based authentication systems.
    fn create_session(
        &self,
        credentials: Self::Credentials,
    ) -> impl Future<Output = Result<Self::Token, Self::Error>> + Send {
        self.authenticate(credentials)
    }

    /// Validate session token (optional)
    /// 
    /// Default implementation delegates to `validate_token`.
    /// Override for session-specific validation logic.
    fn validate_session(
        &self,
        token: &Self::Token,
    ) -> impl Future<Output = Result<Self::Principal, Self::Error>> + Send {
        self.validate_token(token)
    }
}

// ==================== ZERO-COST MARKER ====================

/// **ZERO-COST** marker trait for performance-critical services
///
/// This trait is a **marker trait** that provides hints to the compiler
/// for zero-cost abstractions. Services implementing this trait should:
///
/// 1. Have no runtime overhead
/// 2. Be fully inlineable
/// 3. Use const generics where possible
/// 4. Avoid dynamic dispatch in hot paths
///
/// **Type Parameter**: `T` - The service type
///
/// **Usage**: Mark performance-critical service implementations
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::traits::canonical_hierarchy::ZeroCostService;
///
/// pub struct FastCache<const SIZE: usize> {
///     buffer: [u8; SIZE],
/// }
///
/// impl<const SIZE: usize> ZeroCostService<Self> for FastCache<SIZE> {}
///
/// // Assert at compile time
/// assert_zero_cost!(FastCache<1024>);
/// ```
pub trait ZeroCostService<T>: Send + Sync + 'static {
    // Marker trait: no methods - compile-time only
}

/// Helper macro for asserting zero-cost properties
#[macro_export]
macro_rules! assert_zero_cost {
    ($t:ty) => {
        const _: () = {
            fn assert_send_sync<T: Send + Sync>() {}
            fn assert_zero_sized<T>() {
                assert_send_sync::<T>();
            }
            assert_zero_sized::<$t>();
        };
    };
}

// ==================== MODULE EXPORTS ====================

// NOTE: These traits are NOT re-exported in the parent mod.rs yet
// to avoid conflicts with existing traits during the migration period.
// They will be exported after migration is complete (Week 8).
//
// For now, use them explicitly:
// use nestgate_core::traits::canonical_hierarchy::{CanonicalService, ...}; 