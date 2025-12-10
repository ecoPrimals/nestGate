#![allow(deprecated)]
// **CANONICAL PROVIDER UNIFICATION**
//! Trait definitions and implementations.
// This module provides the unified provider pattern that replaces all
//! scattered provider interfaces with a single canonical provider trait.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;
use std::time::SystemTime;

use crate::unified_enums::service_types::UnifiedServiceType;
use crate::Result;

/// **DEPRECATED**: Extended provider trait with lifecycle management
///
/// **Use `crate::traits::canonical::CanonicalProvider` instead.**
///
/// This trait provided additional lifecycle methods (initialize, stop, validate_config)
/// beyond the core CanonicalProvider interface. The core methods have been consolidated
/// into `canonical::CanonicalProvider`.
///
/// # Migration Guide
///
/// Most features from `CanonicalUniversalProvider` are available in the canonical version:
///
/// **Core Methods** (in canonical):
/// - `provide()` - Service provisioning ✅
/// - `configure()` - Configuration ✅
/// - `metadata()` - Provider metadata ✅
/// - `health_check()` - Health status ✅
/// - `capabilities()` - Provider capabilities ✅
///
/// **Extended Methods** (not in canonical - add if needed):
/// - `initialize()` - Call in constructor
/// - `stop()` - Call in Drop impl
/// - `validate_config()` - Call before configure()
/// - `supported_types()` - Add to capabilities()
///
/// **BEFORE** (this deprecated version):
/// ```rust,ignore
/// impl CanonicalUniversalProvider<MyService> for MyProvider {
///     fn initialize(&self, config: Config) -> impl Future { ... }
///     fn provide(&self) -> impl Future { ... }
///     fn stop(&self) -> impl Future { ... }
/// }
/// ```
///
/// **AFTER** (canonical version):
/// ```rust,ignore
/// impl CanonicalProvider<MyService> for MyProvider {
///     fn provide(&self, config: Config) -> impl Future { ... }
///     fn configure(&mut self, config: Config) -> impl Future { ... }
///     // Add init/stop logic in constructor/Drop if needed
/// }
/// ```
///
/// # Timeline
///
/// - **Deprecated**: November 10, 2025 (v0.11.2)
/// - **Removal**: May 2026 (v0.12.0)
///
/// This replaces ALL scattered provider interfaces
#[deprecated(
    since = "0.11.2",
    note = "Use crate::traits::canonical::CanonicalProvider instead. \
            CanonicalUniversalProvider had extended lifecycle methods (initialize, stop) \
            that can be handled in constructors and Drop implementations. \
            Core provider functionality is in the canonical CanonicalProvider trait. \
            Migration: Replace with CanonicalProvider, move init/stop to constructor/Drop. \
            Target removal: v0.12.0 (May 2026). \
            See: CANONICAL_PROVIDER_COMPARISON.md for detailed migration guide."
)]
/// CanonicalUniversalProvider trait
pub trait CanonicalUniversalProvider<T>: Send + Sync + 'static {
    /// Provider configuration type
    type Config: Clone + Send + Sync + 'static;
    /// Provider error type
    type Error: Send + Sync + std::error::Error + 'static;
    /// Provider metadata type
    type Metadata: Clone + Send + Sync + 'static;
    /// Initialize provider with configuration
    fn initialize(&self, config: Self::Config) -> impl Future<Output = Result<()>> + Send;

    /// Provide service instance
    fn provide(&self) -> impl Future<Output = Result<T>> + Send;

    /// Stop provider
    fn stop(&self) -> impl Future<Output = Result<()>> + Send;

    /// Get provider metadata
    fn get_metadata(&self) -> impl Future<Output = Result<Self::Metadata>> + Send;

    /// Get provider health status
    fn health_check(&self) -> impl Future<Output = Result<ProviderHealth>> + Send;

    /// Get supported service types
    fn supported_types(&self) -> impl Future<Output = Result<Vec<UnifiedServiceType>>> + Send;

    /// Check if provider supports a service type
    fn supports_type(
        &self,
        service_type: &UnifiedServiceType,
    ) -> impl Future<Output = Result<bool>> + Send;

    /// Get provider capabilities
    fn get_capabilities(&self) -> impl Future<Output = Result<ProviderCapabilities>> + Send;

    /// Validate configuration
    fn validate_config(
        &self,
        config: &Self::Config,
    ) -> impl Future<Output = Result<Vec<String>>> + Send;
}

/// Provider health status
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Providerhealth
pub struct ProviderHealth {
    /// Overall health status
    pub status: HealthStatus,
    /// Health check timestamp
    pub checked_at: SystemTime,
    /// Detailed health information
    pub details: HashMap<String, String>,
    /// Performance metrics
    pub metrics: ProviderMetrics,
}
/// Health status enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Status values for Health
pub enum HealthStatus {
    /// Healthy
    Healthy,
    /// Degraded
    Degraded,
    /// Unhealthy
    Unhealthy,
    /// Unknown
    Unknown,
}
/// Provider capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Providercapabilities
pub struct ProviderCapabilities {
    /// Supported operations
    pub operations: Vec<String>,
    /// Maximum concurrent requests
    pub max_concurrent: Option<u32>,
    /// Supported protocols
    pub protocols: Vec<String>,
    /// Feature flags
    pub features: HashMap<String, bool>,
}
/// Provider performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Providermetrics
pub struct ProviderMetrics {
    /// Request count
    pub requests_total: u64,
    /// Success count
    pub requests_successful: u64,
    /// Error count
    pub requests_failed: u64,
    /// Average response time (ms)
    pub avg_response_time_ms: f64,
    /// Current active connections
    pub active_connections: u32,
}
/// **Canonical Security Provider Trait**
///
/// **DEPRECATED**: Use `CanonicalSecurity` from `canonical_unified_traits.rs` instead.
///
/// **Migration Path**:
/// - Replace `impl SecurityProvider` with `impl CanonicalSecurity`
/// - `CanonicalSecurity` has identical 14 methods with enhanced documentation
/// - Same native async (RPITIT) performance characteristics
/// - Target removal: v0.12.0 (May 2026)
///
/// **Why Deprecated**:
/// - Duplicate of `CanonicalSecurity` with identical functionality
/// - `CanonicalSecurity` is the single source of truth for security traits
/// - Part of trait unification initiative (November 2025)
///
/// Unified security provider interface that consolidates all security operations.
/// This trait replaces the following deprecated traits:
/// - `SecurityPrimalProvider` (universal_traits/security.rs)
/// - `ZeroCostSecurityProvider` (zero_cost_security_provider/traits.rs)
/// - `NativeAsyncSecurityProvider` (traits/native_async.rs)
/// - `AuthenticationProvider`, `EncryptionProvider`, `SigningProvider` (specialized traits)
///
/// **Consolidated**: November 10, 2025 - Provider Trait Consolidation Phase 2A
///
/// All methods use native async (RPITIT) for zero-cost abstractions.
/// Extends `CanonicalUniversalProvider` for health checks, metrics, and configuration.
#[deprecated(
    since = "0.11.2",
    note = "Use CanonicalSecurity from canonical_unified_traits instead. \
            SecurityProvider is now a duplicate. Migration: Replace impl SecurityProvider \
            with impl CanonicalSecurity. Target removal: v0.12.0 (May 2026)."
)]
/// SecurityProvider trait
pub trait SecurityProvider: CanonicalUniversalProvider<Box<dyn SecurityService>> {
    // ==================== AUTHENTICATION OPERATIONS ====================

    /// Authenticate user with credentials
    ///
    /// # Arguments
    /// * `credentials` - User credentials (username, password, etc.)
    ///
    /// # Returns
    /// * `Ok(AuthToken)` - Authentication token on success
    /// * `Err(NestGateError)` - Authentication failure
    fn authenticate(&self, credentials: &[u8]) -> impl Future<Output = Result<AuthToken>> + Send;

    /// Authorize access to resource
    ///
    /// # Arguments
    /// * `token` - Authentication token
    /// * `data` - Resource data to authorize
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` - Authorized data
    /// * `Err(NestGateError)` - Authorization failure
    fn authorize(
        &self,
        token: &AuthToken,
        data: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>>> + Send;

    // ==================== TOKEN MANAGEMENT ====================

    /// Validate authentication token
    ///
    /// **NEW**: Consolidated from ZeroCostSecurityProvider
    ///
    /// # Arguments
    /// * `token` - Token to validate
    ///
    /// # Returns
    /// * `Ok(true)` - Token is valid
    /// * `Ok(false)` - Token is invalid or expired
    /// * `Err(NestGateError)` - Validation error
    fn validate_token(&self, token: &AuthToken) -> impl Future<Output = Result<bool>> + Send;

    /// Refresh authentication token
    ///
    /// **NEW**: Consolidated from ZeroCostSecurityProvider
    ///
    /// # Arguments
    /// * `token` - Token to refresh
    ///
    /// # Returns
    /// * `Ok(AuthToken)` - New token with extended expiry
    /// * `Err(NestGateError)` - Refresh failure
    fn refresh_token(&self, token: &AuthToken) -> impl Future<Output = Result<AuthToken>> + Send;

    /// Revoke authentication token
    ///
    /// **NEW**: Consolidated from ZeroCostSecurityProvider
    ///
    /// # Arguments
    /// * `token` - Token to revoke
    ///
    /// # Returns
    /// * `Ok(())` - Token successfully revoked
    /// * `Err(NestGateError)` - Revocation failure
    fn revoke_token(&self, token: &AuthToken) -> impl Future<Output = Result<()>> + Send;

    // ==================== ENCRYPTION OPERATIONS ====================

    /// Encrypt data with specified algorithm
    ///
    /// **NEW**: Missing from original SecurityProvider (had decrypt but not encrypt!)
    /// Consolidated from SecurityPrimalProvider and EncryptionProvider
    ///
    /// # Arguments
    /// * `data` - Data to encrypt
    /// * `algorithm` - Encryption algorithm (e.g., "AES-256-GCM", "ChaCha20-Poly1305")
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` - Encrypted data
    /// * `Err(NestGateError)` - Encryption failure
    fn encrypt(&self, data: &[u8], algorithm: &str)
        -> impl Future<Output = Result<Vec<u8>>> + Send;

    /// Decrypt data
    ///
    /// # Arguments
    /// * `data` - Encrypted data to decrypt
    ///
    /// # Returns
    /// * `Ok(Some(Vec<u8>))` - Decrypted data
    /// * `Ok(None)` - Decryption failed (invalid data or key)
    /// * `Err(NestGateError)` - Decryption error
    fn decrypt(&self, data: &[u8]) -> impl Future<Output = Result<Option<Vec<u8>>>> + Send;

    // ==================== SIGNING OPERATIONS ====================

    /// Sign data cryptographically
    ///
    /// # Arguments
    /// * `data` - Data to sign
    ///
    /// # Returns
    /// * `Ok(())` - Data signed successfully
    /// * `Err(NestGateError)` - Signing failure
    fn sign(&self, data: &[u8]) -> impl Future<Output = Result<()>> + Send;

    /// Verify cryptographic signature
    ///
    /// # Arguments
    /// * `data` - Original data
    /// * `signature` - Signature to verify
    ///
    /// # Returns
    /// * `Ok(Some((algorithm, key_id)))` - Signature valid, returns algorithm and key ID
    /// * `Ok(None)` - Signature invalid
    /// * `Err(NestGateError)` - Verification error
    #[allow(clippy::type_complexity)]
    fn verify(
        &self,
        data: &[u8],
        signature: &[u8],
    ) -> impl Future<Output = Result<Option<(String, Vec<u8>)>>> + Send;

    // ==================== KEY MANAGEMENT ====================

    /// Get signing key identifier
    ///
    /// **NEW**: Consolidated from SecurityPrimalProvider
    ///
    /// # Returns
    /// * `Ok(String)` - Key ID used for signing
    /// * `Err(NestGateError)` - Key retrieval failure
    fn get_key_id(&self) -> impl Future<Output = Result<String>> + Send;

    /// Get supported encryption/signing algorithms
    ///
    /// **NEW**: Consolidated from ZeroCostSecurityProvider
    ///
    /// # Returns
    /// * `Ok(Vec<String>)` - List of supported algorithm names
    /// * `Err(NestGateError)` - Query failure
    fn supported_algorithms(&self) -> impl Future<Output = Result<Vec<String>>> + Send;

    // ==================== UTILITY OPERATIONS ====================

    /// Hash data with specified algorithm
    ///
    /// **NEW**: Consolidated from SecurityPrimalProvider
    ///
    /// # Arguments
    /// * `data` - Data to hash
    /// * `algorithm` - Hash algorithm (e.g., "SHA-256", "SHA-512", "BLAKE3")
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` - Hash digest
    /// * `Err(NestGateError)` - Hashing failure
    fn hash_data(
        &self,
        data: &[u8],
        algorithm: &str,
    ) -> impl Future<Output = Result<Vec<u8>>> + Send;

    /// Generate cryptographically secure random bytes
    ///
    /// **NEW**: Consolidated from SecurityPrimalProvider
    ///
    /// # Arguments
    /// * `length` - Number of random bytes to generate
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` - Random bytes
    /// * `Err(NestGateError)` - Generation failure
    fn generate_random(&self, length: usize) -> impl Future<Output = Result<Vec<u8>>> + Send;
}

/// **Storage Provider Trait**
///
/// **DEPRECATED**: Use `CanonicalStorage` from `canonical_unified_traits.rs` instead.
///
/// **Migration Path**:
/// - Replace `impl StorageProvider` with `impl CanonicalStorage`
/// - `CanonicalStorage` has 17+ comprehensive methods vs 4 basic methods here
/// - Supports advanced features: batch operations, metadata, streaming, backend management
/// - Same native async (RPITIT) performance characteristics
/// - Target removal: v0.12.0 (May 2026)
///
/// **Why Deprecated**:
/// - `StorageProvider` has only 4 basic methods (store, retrieve, delete, list)
/// - `CanonicalStorage` has 17+ methods covering all storage use cases
/// - `CanonicalStorage` is the single source of truth for storage traits
/// - Part of trait unification initiative (November 2025)
///
/// **Method Mapping**:
/// - `StorageProvider::store()` → `CanonicalStorage::write()`
/// - `StorageProvider::retrieve()` → `CanonicalStorage::read()`
/// - `StorageProvider::delete()` → `CanonicalStorage::delete()`
/// - `StorageProvider::list()` → `CanonicalStorage::list()`
///
/// Plus 13+ additional methods in `CanonicalStorage`:
/// - Metadata operations (get/set)
/// - Batch operations (batch_read/write/delete)
/// - Advanced operations (copy, move, usage_stats)
/// - Backend management (initialize, shutdown, capabilities)
/// - Optional features (snapshots, streaming)
///
/// # Example Migration
/// ```ignore
/// // OLD (deprecated):
/// impl StorageProvider for MyStorage {
///     async fn store(&self, key: &str, data: &[u8]) -> Result<()> { ... }
///     async fn retrieve(&self, key: &str) -> Result<Option<Vec<u8>>> { ... }
///     async fn delete(&self, key: &str) -> Result<()> { ... }
///     async fn list(&self, prefix: Option<&str>) -> Result<Vec<String>> { ... }
/// }
///
/// // NEW (canonical):
/// impl CanonicalStorage for MyStorage {
///     type Item = Vec<u8>;
///     type Key = String;
///     type Metadata = HashMap<String, String>;
///     type BackendConfig = MyConfig;
///     
///     async fn write(&self, key: Self::Key, item: Self::Item) -> Result<(), Self::Error> { ... }
///     async fn read(&self, key: Self::Key) -> Result<Option<Self::Item>, Self::Error> { ... }
///     async fn delete(&self, key: Self::Key) -> Result<bool, Self::Error> { ... }
///     async fn list(&self, prefix: Option<Self::Key>) -> Result<Vec<Self::Key>, Self::Error> { ... }
///     // ... plus 13+ additional methods
/// }
/// ```
#[deprecated(
    since = "0.11.2",
    note = "Use CanonicalStorage from canonical_unified_traits instead. \
            StorageProvider has only 4 methods vs 17+ in CanonicalStorage. \
            Migration: Replace impl StorageProvider with impl CanonicalStorage. \
            Target removal: v0.12.0 (May 2026)."
)]
/// StorageProvider trait
pub trait StorageProvider: CanonicalUniversalProvider<Box<dyn StorageService>> {
    /// Store data
    ///
    /// **DEPRECATED**: Use `CanonicalStorage::write()` instead
    fn store(&self, key: &str, data: &[u8]) -> impl Future<Output = Result<()>> + Send;

    /// Retrieve data
    ///
    /// **DEPRECATED**: Use `CanonicalStorage::read()` instead
    fn retrieve(&self, key: &str) -> impl Future<Output = Result<Option<Vec<u8>>>> + Send;

    /// Delete data
    ///
    /// **DEPRECATED**: Use `CanonicalStorage::delete()` instead
    fn delete(&self, key: &str) -> impl Future<Output = Result<()>> + Send;

    /// List keys
    ///
    /// **DEPRECATED**: Use `CanonicalStorage::list()` instead
    fn list(&self, prefix: Option<&str>) -> impl Future<Output = Result<Vec<String>>> + Send;
}

/// **Network Provider Trait**
///
/// **DEPRECATED**: Use `CanonicalNetwork` from `canonical_unified_traits.rs` instead.
///
/// **Migration Path**:
/// - Replace `impl NetworkProvider` with `impl CanonicalNetwork`
/// - `CanonicalNetwork` has 9 comprehensive methods vs 4 basic methods here
/// - Supports advanced features: request/response handling, connection management, streaming
/// - Same native async (RPITIT) performance characteristics
/// - Target removal: v0.12.0 (May 2026)
///
/// **Why Deprecated**:
/// - `NetworkProvider` has only 4 basic methods (send, receive, connect, disconnect)
/// - `CanonicalNetwork` has 9 methods covering all network use cases
/// - `CanonicalNetwork` is the single source of truth for network traits
/// - Part of trait unification initiative (November 2025)
///
/// **Method Mapping**:
/// - `NetworkProvider::send()` → `CanonicalNetwork::send()` (identical)
/// - `NetworkProvider::receive()` → `CanonicalNetwork::receive()` (identical)
/// - `NetworkProvider::connect()` → `CanonicalNetwork::connect()` (identical)
/// - `NetworkProvider::disconnect()` → `CanonicalNetwork::disconnect()` (identical)
///
/// Plus 5+ additional methods in `CanonicalNetwork`:
/// - Typed request/response handling (`handle_request`)
/// - Connection status queries (`connection_status`, `list_connections`)
/// - Optional stream support (`open_stream`, `close_stream`)
///
/// # Example Migration
/// ```ignore
/// // OLD (deprecated):
/// impl NetworkProvider for MyNetwork {
///     async fn send(&self, destination: &str, data: &[u8]) -> Result<()> { ... }
///     async fn receive(&self, timeout_ms: u64) -> Result<Option<Vec<u8>>> { ... }
///     async fn connect(&self, endpoint: &str) -> Result<ConnectionHandle> { ... }
///     async fn disconnect(&self, handle: ConnectionHandle) -> Result<()> { ... }
/// }
///
/// // NEW (canonical):
/// impl CanonicalNetwork for MyNetwork {
///     type Request = NetworkRequest;
///     type Response = NetworkResponse;
///     
///     async fn send(&self, destination: &str, data: &[u8]) -> Result<(), Self::Error> { ... }
///     async fn receive(&self, timeout_ms: u64) -> Result<Option<Vec<u8>>, Self::Error> { ... }
///     async fn connect(&self, endpoint: &str) -> Result<ConnectionHandle, Self::Error> { ... }
///     async fn disconnect(&self, handle: ConnectionHandle) -> Result<(), Self::Error> { ... }
///     
///     // Plus 5+ additional methods:
///     async fn handle_request(&self, request: Self::Request) -> Result<Self::Response, Self::Error> { ... }
///     async fn connection_status(&self, handle: ConnectionHandle) -> Result<ConnectionStatus, Self::Error> { ... }
///     async fn list_connections(&self) -> Result<Vec<ConnectionHandle>, Self::Error> { ... }
///     // open_stream and close_stream have default implementations
/// }
/// ```
#[deprecated(
    since = "0.11.2",
    note = "Use CanonicalNetwork from canonical_unified_traits instead. \
            NetworkProvider has only 4 methods vs 9 in CanonicalNetwork. \
            Migration: Replace impl NetworkProvider with impl CanonicalNetwork. \
            Target removal: v0.12.0 (May 2026)."
)]
/// NetworkProvider trait
pub trait NetworkProvider: CanonicalUniversalProvider<Box<dyn NetworkService>> {
    /// Send data
    ///
    /// **DEPRECATED**: Use `CanonicalNetwork::send()` instead
    fn send(&self, destination: &str, data: &[u8]) -> impl Future<Output = Result<()>> + Send;

    /// Receive data
    ///
    /// **DEPRECATED**: Use `CanonicalNetwork::receive()` instead
    fn receive(&self, timeout_ms: u64) -> impl Future<Output = Result<Option<Vec<u8>>>> + Send;

    /// Connect to endpoint
    ///
    /// **DEPRECATED**: Use `CanonicalNetwork::connect()` instead
    fn connect(&self, endpoint: &str) -> impl Future<Output = Result<ConnectionHandle>> + Send;

    /// Disconnect from endpoint
    ///
    /// **DEPRECATED**: Use `CanonicalNetwork::disconnect()` instead
    fn disconnect(&self, handle: ConnectionHandle) -> impl Future<Output = Result<()>> + Send;
}

/// Service trait definitions
/// **DEPRECATED**: Use canonical security trait instead
#[deprecated(
    since = "0.9.0",
    note = "Use crate::traits::canonical::CanonicalSecurity instead"
)]
/// SecurityService trait
pub trait SecurityService: Send + Sync {}
/// **DEPRECATED**: Duplicate trait - use canonical storage system
#[deprecated(
    since = "0.9.0",
    note = "Use crate::traits::canonical::CanonicalStorage instead"
)]
/// StorageService trait
pub trait StorageService: Send + Sync {}

/// **DEPRECATED**: Use canonical network trait instead
#[deprecated(
    since = "0.11.2",
    note = "Use crate::traits::canonical::CanonicalNetwork instead. \
            NetworkService is a marker trait - migrate to CanonicalNetwork for full functionality. \
            Target removal: v0.12.0 (May 2026)."
)]
/// NetworkService trait
pub trait NetworkService: Send + Sync {}

/// **DEPRECATED**: Use canonical service traits instead
#[deprecated(
    since = "0.11.2",
    note = "Use crate::traits::canonical::CanonicalService instead. \
            CacheService is a marker trait - migrate to CanonicalService with cache-specific types. \
            Target removal: v0.12.0 (May 2026)."
)]
/// CacheService trait
pub trait CacheService: Send + Sync {}
/// Authentication token
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Authtoken
pub struct AuthToken {
    /// Token
    pub token: String,
    /// Expires At
    pub expires_at: SystemTime,
    /// Permissions
    pub permissions: Vec<String>,
}
/// Connection handle
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Connectionhandle
pub struct ConnectionHandle {
    /// Unique identifier
    pub id: String,
    /// Endpoint
    pub endpoint: String,
    /// Status
    pub status: ConnectionStatus,
}
/// Connection status
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Status values for Connection
pub enum ConnectionStatus {
    /// Connected
    Connected,
    /// Connecting
    Connecting,
    /// Disconnected
    Disconnected,
    /// Error variant containing error message
    Error(String),
}
// Default implementations
impl Default for ProviderHealth {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            status: HealthStatus::Unknown,
            checked_at: SystemTime::now(),
            details: HashMap::new(),
            metrics: ProviderMetrics::default(),
        }
    }
}

impl Default for ProviderCapabilities {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            operations: Vec::new(),
            max_concurrent: Some(100),
            protocols: Vec::new(),
            features: HashMap::new(),
        }
    }
}

impl Default for ProviderMetrics {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            requests_total: 0,
            requests_successful: 0,
            requests_failed: 0,
            avg_response_time_ms: 0.0,
            active_connections: 0,
        }
    }
}
