// **CORE RESULT TYPES**
//! Result Types functionality and utilities.
// Idiomatic Result<T, E> types with domain-specific specializations.

use crate::error::NestGateError;
use super::domain_errors::*;

/// **CANONICAL IDIOMATIC RESULT**
/// 
/// This is the primary Result type that should be used throughout the codebase.
/// Both T and E are generic for maximum idiomaticity and ecosystem integration.
/// 
/// **USAGE PATTERNS**:
/// ```rust
/// // Unified error (most common)
/// fn operation() -> IdioResult<Data>                    // Uses NestGateError default
/// 
/// // Domain-specific error  
/// fn validate() -> IdioResult<Config, ValidationError>  // Specific error type
/// 
/// // Ecosystem integration
/// fn parse() -> IdioResult<Value, serde_json::Error>    // External error type
/// ```
pub type IdioResult<T, E = NestGateError> = std::result::Result<T, E>;
/// **VALIDATION OPERATIONS**
/// Specialized Result type for validation operations with rich error context
pub type ValidationResult<T> = IdioResult<T, ValidationError>;
/// **NETWORK OPERATIONS**  
/// Specialized Result type for network operations with connection context
pub type NetworkResult<T> = IdioResult<T, NetworkError>;
/// **STORAGE OPERATIONS**
/// Specialized Result type for storage operations with file/database context
pub type StorageResult<T> = IdioResult<T, StorageError>;
/// **SECURITY OPERATIONS**
/// Specialized Result type for security operations with authentication context
pub type SecurityResult<T> = IdioResult<T, SecurityError>;
/// **ZFS OPERATIONS**
/// Specialized Result type for ZFS operations with pool/dataset context
pub type ZfsResult<T> = IdioResult<T, ZfsError>;
/// **API OPERATIONS**
/// Specialized Result type for API operations with HTTP context
pub type ApiResult<T> = IdioResult<T, ApiError>;
/// **MCP PROTOCOL OPERATIONS**
/// Specialized Result type for MCP operations with protocol context
pub type McpResult<T> = IdioResult<T, McpError>;
/// **TESTING OPERATIONS**
/// Specialized Result type for testing operations with test context
pub type TestingResult<T> = IdioResult<T, TestingError>;
/// **PERFORMANCE OPERATIONS**
/// Specialized Result type for performance operations with metric context
pub type PerformanceResult<T> = IdioResult<T, PerformanceError>;
/// **HANDLER OPERATIONS**
/// Specialized Result type for handler operations with request context
pub type HandlerResult<T> = IdioResult<T, HandlerError>;
/// **SERIALIZATION OPERATIONS**
/// Specialized Result type for serialization operations with format context
pub type SerializationResult<T> = IdioResult<T, SerializationError>;
/// **DATABASE OPERATIONS**
/// Specialized Result type for database operations with query context
pub type DatabaseResult<T> = IdioResult<T, DatabaseError>;
/// **CACHE OPERATIONS**
/// Specialized Result type for cache operations with key context
pub type CacheResult<T> = IdioResult<T, CacheError>;
/// **WORKFLOW OPERATIONS**
/// Specialized Result type for workflow operations with step context
pub type WorkflowResult<T> = IdioResult<T, WorkflowError>;
/// **MONITORING OPERATIONS**
/// Specialized Result type for monitoring operations with metric context
pub type MonitoringResult<T> = IdioResult<T, MonitoringError>; 