#![doc = "
# NestGate Core Library

The foundational crate providing core types, utilities, and abstractions for the NestGate 
Universal Storage Platform. This crate implements the Universal Primal Architecture pattern,
providing agnostic interfaces and zero-copy optimizations for high-performance storage operations.

## Key Components

- **Universal Traits**: Agnostic interfaces for storage, security, and service providers
- **Configuration Management**: Comprehensive configuration system with environment detection
- **Error Handling**: Unified error types with detailed context and retry capabilities  
- **Performance Optimization**: Memory pools, UUID caching, and zero-copy utilities
- **Security Framework**: Multi-provider authentication and authorization system
- **Type System**: Core types for storage tiers, access patterns, and system metrics

## Architecture

NestGate Core follows the Universal Primal Architecture, which provides:
- **Provider Agnosticism**: Works with any storage, security, or AI provider
- **Zero-Copy Operations**: Optimized for minimal memory allocation and copying
- **Async-First Design**: Built for high-concurrency storage operations
- **Production Readiness**: Comprehensive error handling and observability

## Performance Features

- **Memory Pools**: Reduce allocation overhead with reusable buffer pools
- **UUID Caching**: 5x performance improvement for service identification
- **Arc Patterns**: Zero-copy sharing of configuration and state objects
- **Buffer Management**: Intelligent buffer reuse for file operations

## Example Usage

```rust
use nestgate_core::{StorageTier, NestGateError, Result};
use nestgate_core::uuid_cache::get_or_create_uuid;
use nestgate_core::memory_pool::get_4kb_buffer;

// High-performance UUID generation with caching
let service_uuid = get_or_create_uuid(\"my-service\");

// Zero-copy buffer management
let mut buffer = get_4kb_buffer();
buffer.extend_from_slice(b\"data\");

// Comprehensive error handling
fn example_operation() -> Result<()> {
    // Operations that return detailed error context
    Ok(())
}
```

This crate serves as the foundation for all NestGate components and is designed for
maximum performance, reliability, and extensibility.
"]

//!
//! Core functionality and shared components for the NestGate ecosystem.
//! This crate provides the foundational building blocks used across
//! all other NestGate components.

pub mod cert;
pub mod config;
pub mod data_sources;
pub mod error;
pub mod errors;
pub mod interface;
pub mod response;
pub mod temporal_storage;
pub mod traits_root;
pub mod types;

// Additional modules referenced by other crates
pub mod biomeos;
pub mod cache;
pub mod constants;
pub mod crypto_locks;
pub mod diagnostics;
pub mod environment;
pub mod hardware_tuning;
pub mod memory_pool;
pub mod metrics;
pub mod performance;
pub mod security;
pub mod security_provider;
pub mod universal_adapter;
pub mod universal_security_client;
pub mod universal_storage;
pub mod universal_traits;
// pub mod universal_providers;  // TODO: Fix trait implementation issues
pub mod utils;
pub mod uuid_cache;
pub mod zero_copy;

// Re-export key types and traits
pub use error::{NestGateError, Result};
pub use interface::{
    UniversalProviderInterface, UniversalServiceInterface, UniversalStorageInterface,
};
pub use response::{
    ApiResponse, EmptyResponse, ErrorResponse, IntoApiResponse, ResponseBuilder, SuccessResponse,
};

// Re-export commonly used types
pub use cache::StorageTier as CacheStorageTier;
pub use types::StorageTier;

// Re-export UUID utility functions
pub use uuid_cache::{get_or_create_uuid, global_cache_statistics, preload_common_uuids};

// Re-export universal adapter types for easy access
pub use universal_adapter::{
    create_adapter_with_config, create_default_adapter, AdapterStats, DiscoveredPrimal,
    DiscoveryMethod, FallbackBehavior, UniversalAdapterConfig, UniversalPrimalAdapter,
};

// Re-export universal provider wrappers
// TODO: Fix trait implementation issues in universal_providers
// pub use universal_providers::{
//     UniversalSecurityWrapper, UniversalOrchestrationWrapper, UniversalComputeWrapper
// };

// Re-export performance optimizations
pub use memory_pool::{get_1mb_buffer, get_4kb_buffer, MemoryPool, PoolStatistics};
pub use performance::{PerformanceMetrics, PerformanceOptimizedCoordinator};

// Re-export universal traits
pub use universal_traits::*;

/// Universal result type for all NestGate operations
/// This eliminates the need for each crate to define its own Result alias
pub type NestGateResult<T> = std::result::Result<T, NestGateError>;

/// Network operation result type (for network crate operations)
pub type NetworkResult<T> = std::result::Result<T, NetworkError>;

/// API operation result type (for API crate operations)
pub type ApiResult<T> = std::result::Result<T, ApiError>;

/// ZFS operation result type (for ZFS crate operations)
pub type ZfsResult<T> = std::result::Result<T, ZfsError>;

/// MCP operation result type (for MCP crate operations)
pub type McpResult<T> = std::result::Result<T, String>;

// Re-export for convenience - MCP crate can define its own error type
pub type McpError = String;

/// Universal network error type (to be used by network crate)
#[derive(Debug, thiserror::Error)]
pub enum NetworkError {
    #[error("Service not found: {0}")]
    ServiceNotFound(String),
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Connection not found: {0}")]
    ConnectionNotFound(String),
    #[error("Discovery failed: {0}")]
    DiscoveryFailed(String),
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    #[error("Protocol error: {0}")]
    ProtocolError(String),
    #[error("Timeout error: {0}")]
    TimeoutError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<NestGateError> for NetworkError {
    fn from(err: NestGateError) -> Self {
        match err {
            NestGateError::NotFound(msg) => NetworkError::ServiceNotFound(msg),
            NestGateError::Network(msg) => NetworkError::ConnectionFailed(msg),
            NestGateError::Configuration(msg) => NetworkError::ConfigurationError(msg),
            NestGateError::Internal(msg) => NetworkError::Internal(msg),
            NestGateError::Authentication(msg) => NetworkError::Internal(format!("Auth: {}", msg)),
            NestGateError::Authorization(msg) => NetworkError::Internal(format!("Authz: {}", msg)),
            NestGateError::Validation(msg) => {
                NetworkError::Internal(format!("Validation: {}", msg))
            }
            NestGateError::Database(msg) => NetworkError::Internal(format!("Database: {}", msg)),
            NestGateError::FileSystem(msg) => {
                NetworkError::Internal(format!("FileSystem: {}", msg))
            }
            NestGateError::Serialization(msg) => {
                NetworkError::Internal(format!("Serialization: {}", msg))
            }
            NestGateError::Timeout(msg) => NetworkError::TimeoutError(msg),
            NestGateError::Io(msg) => NetworkError::Internal(format!("IO: {}", msg)),
            // Additional variants from error.rs
            NestGateError::Storage(msg) => NetworkError::Internal(format!("Storage: {}", msg)),
            NestGateError::Unauthorized(msg) => {
                NetworkError::Internal(format!("Unauthorized: {}", msg))
            }
            NestGateError::External(msg) => NetworkError::Internal(format!("External: {}", msg)),
            NestGateError::Parse(msg) => NetworkError::Internal(format!("Parse: {}", msg)),
            NestGateError::SystemError(msg) => NetworkError::Internal(format!("System: {}", msg)),
            NestGateError::InvalidInput(msg) => {
                NetworkError::Internal(format!("InvalidInput: {}", msg))
            }
            NestGateError::AuthenticationFailed => {
                NetworkError::Internal("Authentication failed".to_string())
            }
            NestGateError::SecurityModuleUnavailable => {
                NetworkError::Internal("Security module unavailable".to_string())
            }
        }
    }
}

/// Universal API error type (to be used by API crate)
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Forbidden: {0}")]
    Forbidden(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Internal server error: {0}")]
    InternalServerError(String),
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Network error: {0}")]
    NetworkError(#[from] NetworkError),
    #[error("Core error: {0}")]
    CoreError(#[from] NestGateError),
}

/// Universal ZFS error type (to be used by ZFS crate)
#[derive(Debug, thiserror::Error)]
pub enum ZfsError {
    #[error("Pool error: {0}")]
    PoolError(String),
    #[error("Dataset error: {0}")]
    DatasetError(String),
    #[error("Snapshot error: {0}")]
    SnapshotError(String),
    #[error("Migration error: {0}")]
    MigrationError(String),
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("System unavailable: {0}")]
    SystemUnavailable(String),
    #[error("Operation timed out: {0}")]
    Timeout(String),
    #[error("Resource exhausted: {0}")]
    ResourceExhausted(String),
    #[error("Permission denied: {0}")]
    PermissionError(String),
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Internal error: {0}")]
    Internal(String),
    #[error("Core error: {0}")]
    CoreError(#[from] NestGateError),
}
