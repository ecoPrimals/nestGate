//! Canonical Unified Traits for `NestGate`
//! This module provides the unified trait system that consolidates all `NestGate` functionality.
//! Trait definitions and implementations.
//! These traits use `impl Future` returns which may trigger `clippy::type_complexity` warnings
//! but represent the modern async Rust patterns and are more efficient than boxed futures.

#![allow(clippy::type_complexity)]

use std::future::Future;

// **CANONICAL UNIFIED TRAIT SYSTEM**
// This is THE single source of truth for ALL traits across NestGate,
// replacing and consolidating 50+ scattered trait definitions.

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

// Removed unused imports for pedantic perfection
use crate::unified_enums::service_types::UnifiedServiceType;

// ==================== THE CANONICAL SERVICE TRAIT ====================

/// **THE** canonical service trait that replaces ALL service traits
/// This is the single source of truth for all `NestGate` services
pub trait CanonicalService: Send + Sync + 'static {
    /// Service configuration type
    type Config: Clone + Send + Sync + 'static;

    /// Service health status type
    type Health: Clone + Send + Sync + 'static;

    /// Service metrics type
    type Metrics: Clone + Send + Sync + 'static;

    /// Service error type
    type Error: Send + Sync + std::error::Error + 'static;
    // ==================== CORE SERVICE OPERATIONS ====================

    /// Start the service - native async
    fn start(&self) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    /// Stop the service - native async
    fn stop(&self) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    /// Check service health - native async
    fn is_healthy(
        &self,
    ) -> impl Future<Output = std::result::Result<Self::Health, Self::Error>> + Send;

    /// Get service metrics - native async
    fn get_metrics(
        &self,
    ) -> impl Future<Output = std::result::Result<Self::Metrics, Self::Error>> + Send;

    /// Get service capabilities - native async
    fn capabilities(
        &self,
    ) -> impl Future<Output = std::result::Result<ServiceCapabilities, Self::Error>> + Send;

    /// Validate configuration - native async
    fn validate_config(
        &self,
        config: &Self::Config,
    ) -> impl Future<Output = std::result::Result<Vec<String>, Self::Error>> + Send;

    // ==================== ADDITIONAL SERVICE METHODS ====================

    /// Get service identifier - PEDANTIC ADDITION
    fn service_id(&self) -> &str {
        "unknown"
    }

    /// Get service type - PEDANTIC ADDITION
    fn service_type(&self) -> UnifiedServiceType {
        UnifiedServiceType::Generic
    }

    /// Initialize service with config - PEDANTIC ADDITION
    fn initialize(
        &self,
        config: Self::Config,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send {
        async move {
            let _ = config; // Use config parameter
            Ok(())
        }
    }

    /// Health check method - PEDANTIC ADDITION
    fn health_check(
        &self,
    ) -> impl Future<Output = std::result::Result<Self::Health, Self::Error>> + Send {
        async move {
            // PEDANTIC: Use is_healthy method instead of default()
            self.is_healthy().await
        }
    }

    /// Shutdown method - PEDANTIC ADDITION  
    fn shutdown(&self) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send {
        async move {
            // Default graceful shutdown
            Ok(())
        }
    }

    /// Restart method - PEDANTIC ADDITION
    fn restart(&self) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send {
        async move {
            // Default restart implementation
            Ok(())
        }
    }

    /// Update configuration method - PEDANTIC ADDITION
    fn update_config(
        &self,
        _config: Self::Config,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send {
        async move {
            // Default config update implementation
            Ok(())
        }
    }
}

// ==================== THE CANONICAL PROVIDER TRAIT ====================

/// **THE** canonical provider trait that replaces ALL provider traits
/// This is the single source of truth for all `NestGate` providers
pub trait CanonicalProvider<T>: Send + Sync + 'static {
    /// Provider configuration type
    type Config: Clone + Send + Sync + 'static;

    /// Provider error type
    type Error: Send + Sync + std::error::Error + 'static;

    /// Provider metadata type
    type Metadata: Clone + Send + Sync + 'static;
    // ==================== CORE PROVIDER OPERATIONS ====================

    /// Provide service instance - native async
    fn provide(
        &self,
        config: Self::Config,
    ) -> impl Future<Output = std::result::Result<T, Self::Error>> + Send;

    /// Configure provider - native async
    fn configure(
        &mut self,
        config: Self::Config,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    /// Get provider metadata - native async
    fn metadata(
        &self,
    ) -> impl Future<Output = std::result::Result<Self::Metadata, Self::Error>> + Send;

    /// Health check - native async
    fn health_check(
        &self,
    ) -> impl Future<Output = std::result::Result<ProviderHealth, Self::Error>> + Send;

    /// Get provider capabilities - native async
    fn capabilities(
        &self,
    ) -> impl Future<Output = std::result::Result<ProviderCapabilities, Self::Error>> + Send;
}

// ==================== CANONICAL STORAGE TRAIT ====================

/// **THE** canonical storage trait that replaces ALL storage traits
///
/// This trait consolidates and replaces:
/// - `StorageProvider` (from `canonical_provider_unification.rs`) ✨ **Deprecated Nov 9, 2025**
/// - `UnifiedStorageBackend` (from `unified_storage_traits.rs`)
/// - `CanonicalStorageBackend` (from `canonical_storage.rs`)  
/// - `StorageBackend` (from backends/mod.rs)
/// - `ZeroCostStorageProvider` (from `migration/storage_adapters.rs`)
/// - `NativeAsyncStorageProvider` (from `migration/storage_adapters.rs`)
/// - `ZeroCostUnifiedStorageBackend` (from `zero_cost_unified_storage_traits.rs`)
/// - `EnterpriseStorageCapabilities` (from enterprise/traits.rs)
///
/// **ENHANCED**: November 9, 2025 - Comprehensive storage interface (17+ methods)
/// **PERFORMANCE**: Native async throughout - zero `async_trait` overhead
/// **COMPLETENESS**: Covers all storage operations (CRUD, metadata, batch, lifecycle)
/// **UNIFICATION**: Single source of truth for all storage implementations
///
/// # Consolidated Methods (17+ total)
/// - **Core operations (5)**: read, write, delete, list, exists
/// - **Metadata (2)**: get_metadata, set_metadata
/// - **Batch operations (3)**: batch_read, batch_write, batch_delete
/// - **Advanced (3)**: copy, move_item, usage_stats
/// - **Backend management (4)**: backend_type, capabilities, initialize, shutdown
/// - **Optional features (3)**: create_snapshot, stream_read, stream_write
///
/// # Type Safety
/// Generic over `Item`, `Key`, `Metadata`, and `BackendConfig` types:
/// - Flexible type system supports any storage backend (filesystem, object store, database, etc.)
/// - Type-safe operations prevent common storage bugs
/// - Zero-cost abstractions with compile-time optimization
///
/// # Performance Characteristics
/// - **Native async (RPITIT)**: Zero `async_trait` overhead
/// - **Batch operations**: Efficient multi-item operations
/// - **Stream support**: Large data handling without memory bloat
/// - **Backend abstraction**: Zero-cost dispatch for different backends
///
/// # Example Implementation
/// ```ignore
/// impl CanonicalStorage for MyStorageBackend {
///     type Item = Vec<u8>;
///     type Key = String;
///     type Metadata = HashMap<String, String>;
///     type BackendConfig = MyConfig;
///     
///     async fn read(&self, key: Self::Key) -> Result<Option<Self::Item>, Self::Error> {
///         // Read from storage backend
///         Ok(Some(vec![1, 2, 3]))
///     }
///     
///     async fn write(&self, key: Self::Key, item: Self::Item) -> Result<(), Self::Error> {
///         // Write to storage backend
///         Ok(())
///     }
///     
///     // ... implement remaining methods
/// }
/// ```
pub trait CanonicalStorage: CanonicalService {
    /// Storage item type - can be bytes, structured data, or custom types
    type Item: Clone + Send + Sync + 'static;

    /// Storage key type - flexible key system (string, path, custom)
    type Key: Clone + Send + Sync + 'static;

    /// Storage metadata type - extensible metadata system
    type Metadata: Clone + Send + Sync + 'static;

    /// Backend-specific configuration type
    type BackendConfig: Clone + Send + Sync + 'static;

    // ==================== CORE STORAGE OPERATIONS ====================

    /// Read data from storage - native async
    fn read(
        &self,
        key: Self::Key,
    ) -> impl Future<Output = std::result::Result<Option<Self::Item>, Self::Error>> + Send;

    /// Write data to storage - native async
    fn write(
        &self,
        key: Self::Key,
        item: Self::Item,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    /// Delete data from storage - native async
    fn delete(
        &self,
        key: Self::Key,
    ) -> impl Future<Output = std::result::Result<bool, Self::Error>> + Send;

    /// List storage keys with optional prefix - native async
    fn list(
        &self,
        prefix: Option<Self::Key>,
    ) -> impl Future<Output = std::result::Result<Vec<Self::Key>, Self::Error>> + Send;

    /// Check if key exists - native async
    fn exists(
        &self,
        key: Self::Key,
    ) -> impl Future<Output = std::result::Result<bool, Self::Error>> + Send;

    // ==================== METADATA OPERATIONS ====================

    /// Get metadata for a storage item - native async
    fn get_metadata(
        &self,
        key: Self::Key,
    ) -> impl Future<Output = std::result::Result<Option<Self::Metadata>, Self::Error>> + Send;

    /// Set metadata for a storage item - native async
    fn set_metadata(
        &self,
        key: Self::Key,
        metadata: Self::Metadata,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    // ==================== BATCH OPERATIONS ====================

    /// Batch read multiple items - native async
    fn batch_read(
        &self,
        keys: Vec<Self::Key>,
    ) -> impl Future<Output = std::result::Result<Vec<(Self::Key, Option<Self::Item>)>, Self::Error>>
           + Send;

    /// Batch write multiple items - native async
    fn batch_write(
        &self,
        items: Vec<(Self::Key, Self::Item)>,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    /// Batch delete multiple items - native async
    fn batch_delete(
        &self,
        keys: Vec<Self::Key>,
    ) -> impl Future<Output = std::result::Result<Vec<(Self::Key, bool)>, Self::Error>> + Send;

    // ==================== ADVANCED OPERATIONS ====================

    /// Copy item to new location - native async
    fn copy(
        &self,
        source: Self::Key,
        destination: Self::Key,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    /// Move item to new location - native async  
    fn move_item(
        &self,
        source: Self::Key,
        destination: Self::Key,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    /// Get storage usage statistics - native async
    fn usage_stats(
        &self,
    ) -> impl Future<Output = std::result::Result<StorageUsageStats, Self::Error>> + Send;

    // ==================== BACKEND MANAGEMENT ====================

    /// Get backend type identifier
    fn backend_type(&self) -> StorageBackendType;

    /// Get backend capabilities
    fn capabilities(&self) -> Vec<StorageCapability>;

    /// Initialize backend with configuration - native async
    fn initialize_backend(
        &mut self,
        config: Self::BackendConfig,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    /// Shutdown backend gracefully - native async
    fn shutdown_backend(
        &mut self,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    // ==================== OPTIONAL ADVANCED FEATURES ====================

    /// Create a snapshot (if supported) - native async
    fn create_snapshot(
        &self,
        name: &str,
    ) -> impl Future<Output = std::result::Result<Option<SnapshotInfo>, Self::Error>> + Send {
        async move {
            let _ = name;
            Ok(None) // Default: not supported
        }
    }

    /// Stream read large data (if supported) - native async  
    fn stream_read(
        &self,
        key: Self::Key,
    ) -> impl Future<Output = std::result::Result<Option<DataStream>, Self::Error>> + Send {
        async move {
            let _ = key;
            Ok(None) // Default: not supported
        }
    }

    /// Stream write large data (if supported) - native async
    fn stream_write(
        &self,
        key: Self::Key,
    ) -> impl Future<Output = std::result::Result<Option<WriteStream>, Self::Error>> + Send {
        async move {
            let _ = key;
            Ok(None) // Default: not supported
        }
    }
}

// ==================== CANONICAL NETWORK TRAIT ====================

/// **THE** canonical network trait that replaces ALL network service traits
///
/// This trait consolidates and replaces:
/// - `NetworkProvider` (from `canonical_provider_unification.rs`) ✨ **Deprecated Nov 9, 2025**
/// - `NetworkService` (various service traits)
/// - `ZeroCostNetworkProvider` (if any)
/// - Other network-related provider traits
///
/// **ENHANCED**: November 9, 2025 - Comprehensive network interface (9 methods)
/// **PERFORMANCE**: Native async throughout - zero `async_trait` overhead
/// **COMPLETENESS**: Covers all network operations (messaging, connections, streaming)
/// **UNIFICATION**: Single source of truth for all network implementations
///
/// # Consolidated Methods (9 total)
/// - **Messaging (3)**: send, receive, handle_request
/// - **Connection management (4)**: connect, disconnect, connection_status, list_connections
/// - **Stream support (2)**: open_stream, close_stream
///
/// # Type Safety
/// Generic over `Request` and `Response` types:
/// - Flexible type system supports any network protocol
/// - Type-safe request/response handling
/// - Zero-cost abstractions with compile-time optimization
///
/// # Performance Characteristics
/// - **Native async (RPITIT)**: Zero `async_trait` overhead
/// - **Connection pooling**: Efficient connection reuse
/// - **Stream support**: Large data transfer without memory bloat
/// - **Protocol abstraction**: Zero-cost dispatch for different protocols
///
/// # Example Implementation
/// ```ignore
/// impl CanonicalNetwork for MyNetworkBackend {
///     type Request = Vec<u8>;
///     type Response = Vec<u8>;
///     
///     async fn send(&self, destination: &str, data: &[u8]) -> Result<(), Self::Error> {
///         // Send data over network
///         Ok(())
///     }
///     
///     async fn connect(&self, endpoint: &str) -> Result<ConnectionHandle, Self::Error> {
///         // Establish connection
///         Ok(ConnectionHandle::new())
///     }
///     
///     // ... implement remaining methods
/// }
/// ```
pub trait CanonicalNetwork: CanonicalService {
    /// Request type
    type Request: Clone + Send + Sync + 'static;

    /// Response type
    type Response: Clone + Send + Sync + 'static;

    // ==================== MESSAGING OPERATIONS ====================

    /// Send data to destination - native async
    ///
    /// **Consolidated from**: NetworkProvider::send
    ///
    /// # Arguments
    /// * `destination` - Target endpoint/address
    /// * `data` - Data to send
    ///
    /// # Returns
    /// * `Ok(())` - Data sent successfully
    /// * `Err(Self::Error)` - Send failure
    fn send(
        &self,
        destination: &str,
        data: &[u8],
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    /// Receive data with timeout - native async
    ///
    /// **Consolidated from**: NetworkProvider::receive
    ///
    /// # Arguments
    /// * `timeout_ms` - Timeout in milliseconds (0 = no timeout)
    ///
    /// # Returns
    /// * `Ok(Some(data))` - Data received
    /// * `Ok(None)` - Timeout or no data available
    /// * `Err(Self::Error)` - Receive failure
    fn receive(
        &self,
        timeout_ms: u64,
    ) -> impl Future<Output = std::result::Result<Option<Vec<u8>>, Self::Error>> + Send;

    /// Handle network request - native async
    ///
    /// High-level request/response pattern for typed network operations
    ///
    /// # Arguments
    /// * `request` - Network request to handle
    ///
    /// # Returns
    /// * `Ok(response)` - Request handled successfully
    /// * `Err(Self::Error)` - Request handling failure
    fn handle_request(
        &self,
        request: Self::Request,
    ) -> impl Future<Output = std::result::Result<Self::Response, Self::Error>> + Send;

    // ==================== CONNECTION MANAGEMENT ====================

    /// Establish connection to endpoint - native async
    ///
    /// **Consolidated from**: NetworkProvider::connect
    ///
    /// # Arguments
    /// * `endpoint` - Endpoint to connect to (URL, address, etc.)
    ///
    /// # Returns
    /// * `Ok(handle)` - Connection established
    /// * `Err(Self::Error)` - Connection failure
    fn connect(
        &self,
        endpoint: &str,
    ) -> impl Future<Output = std::result::Result<ConnectionHandle, Self::Error>> + Send;

    /// Close connection - native async
    ///
    /// **Consolidated from**: NetworkProvider::disconnect
    ///
    /// # Arguments
    /// * `handle` - Connection handle to close
    ///
    /// # Returns
    /// * `Ok(())` - Connection closed successfully
    /// * `Err(Self::Error)` - Disconnect failure
    fn disconnect(
        &self,
        handle: ConnectionHandle,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    /// Get connection status - native async
    ///
    /// # Arguments
    /// * `handle` - Connection handle to query
    ///
    /// # Returns
    /// * `Ok(status)` - Connection status
    /// * `Err(Self::Error)` - Query failure
    fn connection_status(
        &self,
        handle: ConnectionHandle,
    ) -> impl Future<Output = std::result::Result<ConnectionStatus, Self::Error>> + Send;

    /// List all active connections - native async
    ///
    /// # Returns
    /// * `Ok(handles)` - List of active connection handles
    /// * `Err(Self::Error)` - Query failure
    fn list_connections(
        &self,
    ) -> impl Future<Output = std::result::Result<Vec<ConnectionHandle>, Self::Error>> + Send;

    // ==================== OPTIONAL STREAM SUPPORT ====================

    /// Open bidirectional stream (if supported) - native async
    ///
    /// # Arguments
    /// * `endpoint` - Endpoint to stream to/from
    ///
    /// # Returns
    /// * `Ok(Some(handle))` - Stream opened
    /// * `Ok(None)` - Streams not supported
    /// * `Err(Self::Error)` - Stream open failure
    fn open_stream(
        &self,
        endpoint: &str,
    ) -> impl Future<Output = std::result::Result<Option<ConnectionHandle>, Self::Error>> + Send
    {
        async move {
            let _ = endpoint;
            Ok(None) // Default: not supported
        }
    }

    /// Close stream (if supported) - native async
    ///
    /// # Arguments
    /// * `handle` - Stream handle to close
    ///
    /// # Returns
    /// * `Ok(())` - Stream closed
    /// * `Err(Self::Error)` - Stream close failure
    fn close_stream(
        &self,
        handle: ConnectionHandle,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send {
        async move {
            let _ = handle;
            Ok(()) // Default: no-op
        }
    }
}

// ==================== CANONICAL SECURITY TRAIT ====================

/// **THE** canonical security trait that replaces ALL security service traits
///
/// This trait consolidates and replaces:
/// - `SecurityProvider` (from `canonical_provider_unification.rs`)
/// - `ZeroCostSecurityProvider` (multiple versions)
/// - `SecurityPrimalProvider` (from `universal_traits/security.rs`)
/// - `NativeAsyncSecurityProvider` (multiple versions)
///
/// **ENHANCED**: Now includes comprehensive security operations (14 methods)
/// **PERFORMANCE**: Native async throughout - zero overhead
/// **COMPLETENESS**: All security operations in single canonical location
///
/// # Consolidated Methods (November 9, 2025)
/// - Authentication (4 methods)
/// - Token management (4 methods)
/// - Encryption (4 methods)
/// - Signing/verification (2 methods)
/// - Key management (3 methods)
/// - Utilities (2 methods)
pub trait CanonicalSecurity: CanonicalService {
    /// Authentication token type
    type Token: Clone + Send + Sync + 'static;

    /// User identity type
    type Identity: Clone + Send + Sync + 'static;

    // ==================== AUTHENTICATION OPERATIONS ====================

    /// Authenticate user with credentials
    ///
    /// # Arguments
    /// * `credentials` - User credentials for authentication
    ///
    /// # Returns
    /// * `Ok(Token)` - Authentication token on success
    /// * `Err(Self::Error)` - Authentication failure
    ///
    /// # Example
    /// ```ignore
    /// let token = security.authenticate(credentials).await?;
    /// ```
    fn authenticate(
        &self,
        credentials: SecurityCredentials,
    ) -> impl Future<Output = std::result::Result<Self::Token, Self::Error>> + Send;

    /// Authorize access to resource
    ///
    /// Validates token and checks permissions for requested resource.
    ///
    /// # Arguments
    /// * `token` - Authentication token to validate
    /// * `resource` - Resource identifier being accessed
    ///
    /// # Returns
    /// * `Ok(Identity)` - Authorized user identity
    /// * `Err(Self::Error)` - Authorization failure
    fn authorize(
        &self,
        token: &Self::Token,
        resource: &str,
    ) -> impl Future<Output = std::result::Result<Self::Identity, Self::Error>> + Send;

    // ==================== TOKEN MANAGEMENT ====================

    /// Validate authentication token
    ///
    /// **Consolidated from**: ZeroCostSecurityProvider
    ///
    /// # Arguments
    /// * `token` - Token to validate
    ///
    /// # Returns
    /// * `Ok(Identity)` - Valid token, returns associated identity
    /// * `Err(Self::Error)` - Token invalid, expired, or validation error
    fn validate_token(
        &self,
        token: Self::Token,
    ) -> impl Future<Output = std::result::Result<Self::Identity, Self::Error>> + Send;

    /// Refresh authentication token
    ///
    /// **Consolidated from**: ZeroCostSecurityProvider, SecurityProvider
    ///
    /// # Arguments
    /// * `token` - Token to refresh
    ///
    /// # Returns
    /// * `Ok(Token)` - New token with extended expiry
    /// * `Err(Self::Error)` - Refresh failure (token expired or invalid)
    fn refresh_token(
        &self,
        token: &Self::Token,
    ) -> impl Future<Output = std::result::Result<Self::Token, Self::Error>> + Send;

    /// Revoke authentication token
    ///
    /// **Consolidated from**: All security providers
    ///
    /// # Arguments
    /// * `token` - Token to revoke
    ///
    /// # Returns
    /// * `Ok(())` - Token successfully revoked
    /// * `Err(Self::Error)` - Revocation failure
    fn revoke_token(
        &self,
        token: Self::Token,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    /// List all active tokens for user
    ///
    /// **Consolidated from**: SecurityProvider
    ///
    /// # Arguments
    /// * `identity` - User identity to query
    ///
    /// # Returns
    /// * `Ok(Vec<Token>)` - List of active tokens
    /// * `Err(Self::Error)` - Query failure
    fn list_active_tokens(
        &self,
        identity: &Self::Identity,
    ) -> impl Future<Output = std::result::Result<Vec<Self::Token>, Self::Error>> + Send;

    // ==================== ENCRYPTION OPERATIONS ====================

    /// Encrypt data with specified algorithm
    ///
    /// **Consolidated from**: ZeroCostSecurityProvider, EncryptionProvider
    ///
    /// # Arguments
    /// * `data` - Data to encrypt
    /// * `algorithm` - Encryption algorithm (e.g., "AES-256-GCM", "ChaCha20-Poly1305")
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` - Encrypted data
    /// * `Err(Self::Error)` - Encryption failure
    fn encrypt(
        &self,
        data: &[u8],
        algorithm: &str,
    ) -> impl Future<Output = std::result::Result<Vec<u8>, Self::Error>> + Send;

    /// Decrypt data
    ///
    /// **Consolidated from**: ZeroCostSecurityProvider, EncryptionProvider
    ///
    /// # Arguments
    /// * `encrypted_data` - Encrypted data to decrypt
    /// * `algorithm` - Decryption algorithm
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` - Decrypted data
    /// * `Err(Self::Error)` - Decryption failure
    fn decrypt(
        &self,
        encrypted_data: &[u8],
        algorithm: &str,
    ) -> impl Future<Output = std::result::Result<Vec<u8>, Self::Error>> + Send;

    /// Get encryption key for algorithm
    ///
    /// **Consolidated from**: SecurityProvider
    ///
    /// # Arguments
    /// * `algorithm` - Algorithm name
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` - Encryption key
    /// * `Err(Self::Error)` - Key retrieval failure
    fn get_encryption_key(
        &self,
        algorithm: &str,
    ) -> impl Future<Output = std::result::Result<Vec<u8>, Self::Error>> + Send;

    /// Rotate encryption key
    ///
    /// **Consolidated from**: SecurityProvider
    ///
    /// # Arguments
    /// * `algorithm` - Algorithm to rotate key for
    ///
    /// # Returns
    /// * `Ok(())` - Key rotated successfully
    /// * `Err(Self::Error)` - Rotation failure
    fn rotate_encryption_key(
        &self,
        algorithm: &str,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    // ==================== SIGNING OPERATIONS ====================

    /// Sign data cryptographically
    ///
    /// **Consolidated from**: ZeroCostSecurityProvider, SigningProvider
    ///
    /// # Arguments
    /// * `data` - Data to sign
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` - Digital signature
    /// * `Err(Self::Error)` - Signing failure
    fn sign(
        &self,
        data: &[u8],
    ) -> impl Future<Output = std::result::Result<Vec<u8>, Self::Error>> + Send;

    /// Verify cryptographic signature
    ///
    /// **Consolidated from**: ZeroCostSecurityProvider, SigningProvider
    ///
    /// # Arguments
    /// * `data` - Original data
    /// * `signature` - Signature to verify
    ///
    /// # Returns
    /// * `Ok(true)` - Signature valid
    /// * `Ok(false)` - Signature invalid
    /// * `Err(Self::Error)` - Verification error
    fn verify(
        &self,
        data: &[u8],
        signature: &[u8],
    ) -> impl Future<Output = std::result::Result<bool, Self::Error>> + Send;

    // ==================== KEY MANAGEMENT ====================

    /// Get signing key identifier
    ///
    /// **Consolidated from**: ZeroCostSecurityProvider
    ///
    /// # Returns
    /// * `Ok(String)` - Key ID used for signing
    /// * `Err(Self::Error)` - Key retrieval failure
    fn get_key_id(&self) -> impl Future<Output = std::result::Result<String, Self::Error>> + Send;

    /// Get supported algorithms
    ///
    /// **Consolidated from**: ZeroCostSecurityProvider, EncryptionProvider
    ///
    /// # Returns
    /// * `Ok(Vec<String>)` - List of supported algorithm names
    /// * `Err(Self::Error)` - Query failure
    fn supported_algorithms(
        &self,
    ) -> impl Future<Output = std::result::Result<Vec<String>, Self::Error>> + Send;

    // ==================== UTILITY OPERATIONS ====================

    /// Generate cryptographically secure random data
    ///
    /// **Consolidated from**: ZeroCostSecurityProvider
    ///
    /// # Arguments
    /// * `length` - Number of random bytes to generate
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` - Random bytes
    /// * `Err(Self::Error)` - Generation failure
    fn generate_random(
        &self,
        length: usize,
    ) -> impl Future<Output = std::result::Result<Vec<u8>, Self::Error>> + Send;

    /// Hash data with specified algorithm
    ///
    /// **Consolidated from**: ZeroCostSecurityProvider (optional)
    ///
    /// # Arguments
    /// * `data` - Data to hash
    /// * `algorithm` - Hash algorithm (e.g., "SHA-256", "SHA-512", "BLAKE3")
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` - Hash digest
    /// * `Err(Self::Error)` - Hashing failure
    ///
    /// # Default Implementation
    /// Implementations may provide default hash algorithm if None specified
    fn hash_data(
        &self,
        data: &[u8],
        algorithm: Option<&str>,
    ) -> impl Future<Output = std::result::Result<Vec<u8>, Self::Error>> + Send
    where
        Self::Error: From<crate::NestGateError>,
    {
        // Default implementation can be overridden
        async move {
            let _ = (data, algorithm);
            Err(Self::Error::from(crate::NestGateError::not_implemented(
                "hash_data not implemented for this security provider",
            )))
        }
    }
}

// ==================== ADDITIONAL CANONICAL TRAITS ====================

/// MCP protocol trait
pub trait CanonicalMcp: CanonicalService {}
/// Automation trait
pub trait CanonicalAutomation: CanonicalService {}
/// Zero-cost service marker
pub trait ZeroCostService: CanonicalService {}
/// Service factory
pub trait CanonicalServiceFactory<T: CanonicalService> {
    fn create_service(
        &self,
        config: T::Config,
    ) -> impl Future<Output = std::result::Result<T, crate::NestGateError>> + Send;
}
/// Provider factory
pub trait CanonicalProviderFactory<T, P: CanonicalProvider<T>> {
    fn create_provider(
        &self,
        config: P::Config,
    ) -> impl Future<Output = std::result::Result<P, crate::NestGateError>> + Send;
}
// ==================== SUPPORTING TYPES ====================

/// Service capabilities
#[derive(Debug, Clone, Default, Serialize, Deserialize)] // PEDANTIC: Added Default derive
pub struct ServiceCapabilities {
    pub can_scale: bool,
    pub can_migrate: bool,
    pub can_backup: bool,
    pub supported_protocols: Vec<String>,
}
/// Provider health status
#[derive(Debug, Clone, Serialize, Deserialize)] // PEDANTIC: Added Default derive
pub struct ProviderHealth {
    pub is_healthy: bool,
    pub last_check: SystemTime,
    pub health: String,
}
impl Default for ProviderHealth {
    fn default() -> Self {
        Self {
            is_healthy: false,
            last_check: SystemTime::now(),
            health: String::new(),
        }
    }
}

/// Provider capabilities
#[derive(Debug, Clone, Serialize, Deserialize)] // PEDANTIC: Added Serialize/Deserialize derives
pub struct ProviderCapabilities {
    pub supported_types: Vec<UnifiedServiceType>,
    pub max_instances: Option<u32>,
}
/// Storage usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageUsageStats {
    /// Total storage capacity in bytes
    pub total_capacity: u64,
    /// Used storage in bytes
    pub used_space: u64,
    /// Available storage in bytes
    pub available_space: u64,
    /// Number of stored items
    pub item_count: u64,
    /// Last updated timestamp
    pub last_updated: std::time::SystemTime,
}
/// Connection handle
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ConnectionHandle(pub u64);
/// Connection status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Active,
    Idle,
    Closed,
    Error(String),
}
/// Health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}
/// Security credentials
#[derive(Debug, Clone)]
pub struct SecurityCredentials {
    pub username: String,
    pub password: String,
}
/// Cron schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CronSchedule {
    pub expression: String,
}
/// Schedule ID
#[derive(Debug, Clone, Serialize, Deserialize)] // PEDANTIC: Added Serialize/Deserialize derives
pub struct ScheduleId {
    pub id: String,
}
/// Schedule info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleInfo {
    pub id: ScheduleId,
    pub schedule: CronSchedule,
    pub next_run: Option<SystemTime>,
}

// ==================== CANONICAL STORAGE TYPES ====================

/// Storage backend type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StorageBackendType {
    /// Local filesystem storage
    FileSystem,
    /// In-memory storage
    Memory,
    /// Object storage (S3-compatible)
    ObjectStorage,
    /// Block storage
    BlockStorage,
    /// Network filesystem (NFS, SMB, etc.)
    NetworkFileSystem,
    /// ZFS storage
    Zfs,
    /// Distributed storage
    Distributed,
    /// Custom storage type
    Custom(String),
}

/// Storage capability enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StorageCapability {
    /// Basic CRUD operations
    BasicOperations,
    /// Batch operations support
    BatchOperations,
    /// Metadata operations
    Metadata,
    /// Streaming data support
    Streaming,
    /// Snapshot support
    Snapshots,
    /// Atomic operations
    Atomic,
    /// Versioning support
    Versioning,
    /// Encryption support
    Encryption,
    /// Compression support
    Compression,
    /// Replication support
    Replication,
    /// Custom capability
    Custom(String),
}

/// Snapshot information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotInfo {
    /// Snapshot identifier
    pub id: String,
    /// Snapshot name
    pub name: String,
    /// Creation timestamp
    pub created_at: std::time::SystemTime,
    /// Snapshot size in bytes
    pub size: u64,
}

/// Data stream for reading large objects
pub struct DataStream {
    // Implementation would contain actual stream
    _phantom: std::marker::PhantomData<()>,
}

/// Write stream for writing large objects
pub struct WriteStream {
    // Implementation would contain actual stream
    _phantom: std::marker::PhantomData<()>,
}
