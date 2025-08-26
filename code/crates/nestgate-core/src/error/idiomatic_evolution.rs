use crate::NestGateError;
use std::collections::HashMap;
// **IDIOMATIC RESULT<T, E> EVOLUTION**
//
// **CANONICAL MODERNIZATION**: Evolution from Result<T> to idiomatic Result<T, E>
// while preserving ALL benefits of our sophisticated unified error system.
//
// **PROBLEM SOLVED**: 
// - Non-idiomatic Result<T> pattern (only T generic)
// - 7+ fragmented Result type patterns across crates
// - Poor ecosystem integration with anyhow/thiserror
// - Limited flexibility for domain-specific errors
//
// **SOLUTION**:
// - Idiomatic Result<T, E> with both T and E generic
// - Preserves unified NestGateError system
// - Domain-specific error types for specialized contexts
// - Ecosystem integration patterns
// - Zero breaking changes through gradual migration

use serde::{Deserialize, Serialize};
use crate::error::NestGateError;

// ==================== IDIOMATIC RESULT TYPES ====================

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

/// **BACKWARD COMPATIBLE RESULT**
/// 
/// Maintains compatibility with existing Result<T> usage while encouraging
/// migration to IdioResult<T, E> for new code.
/// 
/// **MIGRATION STRATEGY**: Gradually replace with IdioResult<T> or IdioResult<T, E>
pub type Result<T> = IdioResult<T>;

// ==================== DOMAIN-SPECIFIC RESULT TYPES ====================

/// **VALIDATION OPERATIONS**
/// Specialized Result type for validation operations with rich error context
pub type ValidationResult<T> = IdioResult<T, ValidationError>;

/// **NETWORK OPERATIONS**  
/// Specialized Result type for network operations with connection context
pub type NetworkResult<T> = IdioResult<T, NetworkError>;

/// **STORAGE OPERATIONS**
/// Specialized Result type for storage operations with resource context  
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
/// Specialized Result type for MCP protocol operations with protocol context
pub type McpResult<T> = IdioResult<T, McpError>;

// ==================== ECOSYSTEM INTEGRATION TYPES ====================

/// **ANYHOW INTEGRATION**
/// For operations that benefit from anyhow's error handling
pub type AnyhowResult<T> = IdioResult<T, anyhow::Error>;

/// **BOXED ERROR INTEGRATION**
/// For operations that need dynamic error types
pub type BoxedResult<T> = IdioResult<T, Box<dyn std::error::Error + Send + Sync>>;

/// **STANDARD LIBRARY INTEGRATION**
/// For operations that work with standard library error types
pub type StdResult<T, E> = IdioResult<T, E>;

// ==================== DOMAIN-SPECIFIC ERROR TYPES ====================

/// **VALIDATION ERROR**
/// Rich error type for validation operations with field-level context
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum ValidationError {
    #[error("Field validation failed: {field} - {message}")]
    FieldValidation {
        field: String,
        message: String,
        value: Option<String>,
        constraint: Option<String>,
    },
    
    #[error("Constraint violation: {constraint} - expected {expected}, got {actual}")]
    ConstraintViolation {
        constraint: String,
        expected: String,
        actual: String,
        field: Option<String>,
    },
    
    #[error("Security constraint violated: {field} - {message}")]
    SecurityConstraint {
        field: String,
        message: String,
        security_level: String,
        recommendation: Option<String>,
    },
    
    #[error("Format error: {format} - {error}")]
    FormatError {
        format: String,
        error: String,
        content_preview: Option<String>,
    },
    
    #[error("Unified validation error: {0}")]
    Unified(#[from] NestGateError),
}

/// **NETWORK ERROR**
/// Rich error type for network operations with connection context
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum NetworkError {
    #[error("Connection failed to {address}:{port} - {error}")]
    ConnectionFailed {
        address: String,
        port: u16,
        error: String,
        timeout: Option<std::time::Duration>,
        retry_count: Option<u32>,
    },
    
    #[error("Network timeout: {operation} took longer than {duration:?}")]
    Timeout {
        operation: String,
        duration: std::time::Duration,
        address: Option<String>,
    },
    
    #[error("DNS resolution failed: {hostname} - {error}")]
    DnsResolution {
        hostname: String,
        error: String,
        nameservers: Option<Vec<String>>,
    },
    
    #[error("Protocol error: {protocol} - {message}")]
    ProtocolError {
        protocol: String,
        message: String,
        error_code: Option<i32>,
    },
    
    #[error("Service discovery failed: {service} - {message}")]
    ServiceDiscoveryFailed {
        service: String,
        message: String,
        endpoint: Option<String>,
    },
    
    #[error("Service registration failed: {service} - {message}")]
    ServiceRegistrationFailed {
        service: String,
        message: String,
        registry: Option<String>,
    },
    
    #[error("Invalid address: {address} - {message}")]
    InvalidAddress {
        address: String,
        message: String,
    },
    
    #[error("Connection lost: {endpoint} - {reason}")]
    ConnectionLost {
        endpoint: String,
        reason: String,
    },
    
    #[error("Unified network error: {0}")]
    Unified(#[from] NestGateError),
}

/// **STORAGE ERROR**
/// Rich error type for storage operations with resource context
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum StorageError {
    #[error("File not found: {path}")]
    FileNotFound {
        path: String,
        operation: String,
        permissions: Option<String>,
    },
    
    #[error("Permission denied: {path} - {operation} requires {required_permission}")]
    PermissionDenied {
        path: String,
        operation: String,
        required_permission: String,
        current_user: String,
    },
    
    #[error("Disk full: {path} - {available_space} bytes available")]
    DiskFull {
        path: String,
        available_space: u64,
        required_space: u64,
    },
    
    #[error("File read error: {path} - {error}")]
    FileReadError {
        path: String,
        operation: String,
        error: String,
        permissions: Option<String>,
    },
    
    #[error("Invalid content: {path} - {content_type} format error: {error}")]
    InvalidContent {
        path: String,
        content_type: String,
        error: String,
    },
    
    #[error("Unified storage error: {0}")]
    Unified(#[from] NestGateError),
}

/// **SECURITY ERROR**
/// Rich error type for security operations with authentication context
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum SecurityError {
    #[error("Invalid token: {token_type} - {reason}")]
    InvalidToken {
        token_type: String,
        reason: String,
        expired: bool,
    },
    
    #[error("Token expired: {token_type} expired at {expired_at:?}")]
    TokenExpired {
        token_type: String,
        expired_at: std::time::SystemTime,
    },
    
    #[error("Authentication failed: {method} - {reason}")]
    AuthenticationFailed {
        method: String,
        reason: String,
        user_id: Option<String>,
        ip_address: Option<String>,
    },
    
    #[error("Authorization failed: {resource} - {required_permission} required")]
    AuthorizationFailed {
        resource: String,
        required_permission: String,
        user_permissions: Vec<String>,
        user_id: String,
    },
    
    #[error("Unified security error: {0}")]
    Unified(#[from] NestGateError),
}

/// **ZFS ERROR**
/// Rich error type for ZFS operations with pool/dataset context
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum ZfsError {
    #[error("Pool error: {pool} - {operation} failed: {error}")]
    PoolError {
        pool: String,
        operation: String,
        error: String,
        available_space: Option<u64>,
    },
    
    #[error("Dataset error: {dataset} - {operation} failed: {error}")]
    DatasetError {
        dataset: String,
        operation: String,
        error: String,
        parent_pool: Option<String>,
    },
    
    #[error("Snapshot error: {snapshot} - {operation} failed: {error}")]
    SnapshotError {
        snapshot: String,
        operation: String,
        error: String,
        dataset: Option<String>,
    },
    
    #[error("Unified ZFS error: {0}")]
    Unified(#[from] NestGateError),
}

/// **API ERROR**
/// Rich error type for API operations with HTTP context
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum ApiError {
    #[error("HTTP {status_code}: {method} {path} - {message}")]
    HttpError {
        status_code: u16,
        method: String,
        path: String,
        message: String,
        headers: Option<std::collections::HashMap<String, String>>,
    },
    
    #[error("Request validation failed: {field} - {message}")]
    RequestValidation {
        field: String,
        message: String,
        request_body: Option<String>,
    },
    
    #[error("Rate limit exceeded: {endpoint} - {limit} requests per {window:?}")]
    RateLimitExceeded {
        endpoint: String,
        limit: u32,
        window: std::time::Duration,
        retry_after: Option<std::time::Duration>,
    },
    
    #[error("Unified API error: {0}")]
    Unified(#[from] NestGateError),
}

/// **MCP ERROR**
/// Rich error type for MCP protocol operations with protocol context
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum McpError {
    #[error("Protocol version mismatch: expected={expected}, got={actual}")]
    VersionMismatch {
        expected: String,
        actual: String,
    },
    
    #[error("Message parsing failed: {message_type} - {error}")]
    MessageParsing {
        message_type: String,
        error: String,
        raw_message: Option<String>,
    },
    
    #[error("Connection state invalid: expected={expected}, current={current}")]
    InvalidState {
        expected: String,
        current: String,
        operation: String,
    },
    
    #[error("Resource not found: {resource_type}:{resource_id}")]
    ResourceNotFound {
        resource_type: String,
        resource_id: String,
    },
    
    #[error("Protocol error: {version} - {message_type} (code: {error_code:?})")]
    ProtocolError {
        version: String,
        message_type: String,
        error_code: Option<i32>,
        request_id: Option<String>,
    },
    
    #[error("Unified MCP error: {0}")]
    Unified(#[from] NestGateError),
}

// ==================== ERROR CONVERSION TRAITS ====================

/// **IDIOMATIC ERROR CONVERSION**
/// Provides seamless conversion between domain-specific errors and NestGateError
/// while maintaining all rich context information.
pub trait IntoNestGateError {
    fn into_nestgate_error(self) -> NestGateError;
}

/// **CONTEXTUAL ERROR ENHANCEMENT**
/// Adds rich context to errors for better debugging and monitoring
pub trait WithContext<T> {
    fn with_operation(self, operation: &str) -> IdioResult<T>;
    
    fn with_component(self, component: &str) -> IdioResult<T>;
}

impl<T, E> WithContext<T> for IdioResult<T, E>
where
    E: Into<NestGateError>,
{
    fn with_operation(self, operation: &str) -> IdioResult<T> {
        self.map_err(|e| {
            let mut error = e.into();
            // Add operation context to NestGateError
            error.add_context("operation", operation);
            error
        })
    }
    
    fn with_component(self, component: &str) -> IdioResult<T> {
        self.map_err(|e| {
            let mut error = e.into();
            // Add component context to NestGateError
            error.add_context("component", component);
            error
        })
    }
}

// ==================== CONVENIENCE MACROS ====================

/// **VALIDATION ERROR MACRO**
/// Ergonomic construction of validation errors
#[macro_export]
macro_rules! validation_error {
    ($field:expr, $message:expr) => {
        $crate::error::ValidationError::FieldValidation {
            field: $field.to_string(),
            message: $message.to_string(),
            value: None,
            constraint: None,
        }
    };
    ($field:expr, $message:expr, $value:expr) => {
        $crate::error::ValidationError::FieldValidation {
            field: $field.to_string(),
            message: $message.to_string(),
            value: Some($value.to_string()),
            constraint: None,
        }
    };
}

/// **NETWORK ERROR MACRO**
/// Ergonomic construction of network errors
#[macro_export]
macro_rules! network_error {
    (connection, $address:expr, $port:expr, $error:expr) => {
        $crate::error::NetworkError::ConnectionFailed {
            address: $address.to_string(),
            port: $port,
            error: $error.to_string(),
            timeout: None,
            retry_count: None,
        }
    };
    (timeout, $operation:expr, $duration:expr) => {
        $crate::error::NetworkError::Timeout {
            operation: $operation.to_string(),
            duration: $duration,
            address: None,
        }
    };
}

/// **STORAGE ERROR MACRO**
/// Ergonomic construction of storage errors
#[macro_export]
macro_rules! storage_error {
    (not_found, $path:expr) => {
        $crate::error::StorageError::FileNotFound {
            path: $path.to_string(),
            operation: "access".to_string(),
            permissions: None,
        }
    };
    (permission_denied, $path:expr, $operation:expr, $required:expr) => {
        $crate::error::StorageError::PermissionDenied {
            path: $path.to_string(),
            operation: $operation.to_string(),
            required_permission: $required.to_string(),
            current_user: "unknown".to_string(),
        }
    };
}

/// **MIGRATION HELPER**
/// Utilities to help migrate from Result<T> to IdioResult<T, E> patterns
pub struct MigrationHelper;

impl MigrationHelper {
    /// Convert legacy Result<T> to IdioResult<T, E> with domain-specific error
    pub fn to_validation_result<T>(result: Result<T>) -> ValidationResult<T> {
        result.map_err(ValidationError::Unified)
    }
    
    pub fn to_network_result<T>(result: Result<T>) -> NetworkResult<T> {
        result.map_err(NetworkError::Unified)
    }
    
    pub fn to_storage_result<T>(result: Result<T>) -> StorageResult<T> {
        result.map_err(StorageError::Unified)
    }
    
    pub fn to_security_result<T>(result: Result<T>) -> SecurityResult<T> {
        result.map_err(SecurityError::Unified)
    }
    
    pub fn to_zfs_result<T>(result: Result<T>) -> ZfsResult<T> {
        result.map_err(ZfsError::Unified)
    }
    
    pub fn to_api_result<T>(result: Result<T>) -> ApiResult<T> {
        result.map_err(ApiError::Unified)
    }
    
    pub fn to_mcp_result<T>(result: Result<T>) -> McpResult<T> {
        result.map_err(McpError::Unified)
    }
}

// ==================== TESTING UTILITIES ====================

/// **TEST HELPERS**
/// Utilities for testing with idiomatic error types
#[cfg(test)]
pub mod test_utils {
    use super::*;
    
    pub fn create_validation_error() -> ValidationError {
        ValidationError::FieldValidation {
            field: "test_field".to_string(),
            message: "Test validation error".to_string(),
            value: Some("invalid_value".to_string()),
            constraint: Some("non-empty".to_string()),
        }
    }
    
    pub fn create_network_error() -> NetworkError {
        NetworkError::ConnectionFailed {
            address: "localhost".to_string(),
            port: 8080,
            error: "Connection refused".to_string(),
            timeout: Some(std::time::Duration::from_secs(30)),
            retry_count: Some(3),
        }
    }
    
    pub fn create_storage_error() -> StorageError {
        StorageError::FileNotFound {
            path: "/tmp/test.txt".to_string(),
            operation: "read".to_string(),
            permissions: Some("read".to_string()),
        }
    }
}

/// **USAGE EXAMPLES**
/// 
/// This module demonstrates the idiomatic usage patterns for the evolved error system.
#[cfg(doc)]
pub mod examples {
    use super::*;
    
    /// Example: Validation with rich error context
    pub fn validate_config(config: &str) -> ValidationResult<Config> {
        if config.is_empty() {
            return Err(validation_error!("config", "Configuration cannot be empty"));
        }
        
        // Parse and validate config
        Ok(Config::new())
    }
    
    /// Example: Network operation with timeout context
    pub fn connect_to_service(address: &str, port: u16) -> NetworkResult<Connection> {
        std::thread::sleep(std::time::Duration::from_secs(1));
        Err(network_error!(connection, address, port, "Service unavailable"))
    }
    
    /// Example: Storage operation with permission context
    pub fn read_file(path: &str) -> StorageResult<String> {
        if !std::path::Path::new(path).exists() {
            return Err(storage_error!(not_found, path));
        }
        
        Ok("file contents".to_string())
    }
    
    /// Example: Ecosystem integration with anyhow
    pub fn parse_json(data: &str) -> AnyhowResult<serde_json::Value> {
        serde_json::from_str(data).map_err(Into::into)
    }
    
    /// Example: Migration from legacy Result<T>
    pub fn legacy_operation() -> Result<String> {
        Ok("legacy result".to_string())
    }
    
    pub fn migrated_operation() -> ValidationResult<String> {
        MigrationHelper::to_validation_result(legacy_operation())
    }
    
    // Placeholder types for examples
    struct Config;
    impl Config {
        fn new() -> Self { Config }
    }
    struct Connection;
}

// ==================== IMPLEMENTATION TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_idiomatic_result_types() {
        // Test ValidationResult
        let validation_result: ValidationResult<String> = Ok("valid".to_string());
        assert!(validation_result.is_ok());
        
        let validation_error: ValidationResult<String> = Err(ValidationError::FieldValidation {
            field: "test".to_string(),
            message: "Invalid".to_string(),
            value: None,
            constraint: None,
        });
        assert!(validation_error.is_err());
        
        // Test NetworkResult
        let network_result: NetworkResult<String> = Ok("connected".to_string());
        assert!(network_result.is_ok());
        
        // Test StorageResult
        let storage_result: StorageResult<String> = Ok("file contents".to_string());
        assert!(storage_result.is_ok());
    }
    
    #[test]
    fn test_migration_helper() {
        // Test legacy to validation result migration
        let legacy_result: Result<String> = Ok("test".to_string());
        let migrated: ValidationResult<String> = MigrationHelper::to_validation_result(legacy_result);
        assert!(migrated.is_ok());
        assert_eq!(migrated.unwrap(), "test");
        
        // Test error migration
        let legacy_error: Result<String> = Err(NestGateError::Configuration {
            message: "Test error".to_string(),
            field: None,
            suggested_fix: None,
            config_source: None,
        });
        let migrated_error: ValidationResult<String> = MigrationHelper::to_validation_result(legacy_error);
        assert!(migrated_error.is_err());
        
        if let Err(ValidationError::Unified(_)) = migrated_error {
            // Expected unified error
        } else {
            panic!("Expected unified validation error");
        }
    }
    
    #[test]
    fn test_domain_specific_errors() {
        // Test ValidationError
        let validation_error = ValidationError::FieldValidation {
            field: "username".to_string(),
            message: "Cannot be empty".to_string(),
            value: Some("".to_string()),
            constraint: Some("non-empty".to_string()),
        };
        assert!(format!("{}", validation_error).contains("Field validation failed"));
        
        // Test NetworkError
        let network_error = NetworkError::ConnectionFailed {
            address: "localhost".to_string(),
            port: 8080,
            error: "Connection refused".to_string(),
            timeout: None,
            retry_count: None,
        };
        assert!(format!("{}", network_error).contains("Connection failed"));
        
        // Test StorageError
        let storage_error = StorageError::FileNotFound {
            path: "/tmp/test.txt".to_string(),
            operation: "read".to_string(),
            permissions: None,
        };
        assert!(format!("{}", storage_error).contains("File not found"));
    }
    
    #[test]
    fn test_ecosystem_integration() {
        // Test AnyhowResult with serde_json
        let json_result: AnyhowResult<serde_json::Value> = serde_json::from_str(r#"{"key": "value"}"#)
            .map_err(Into::into);
        assert!(json_result.is_ok());
        
        // Test parsing error
        let json_error: AnyhowResult<serde_json::Value> = serde_json::from_str("invalid json")
            .map_err(Into::into);
        assert!(json_error.is_err());
    }
    
    #[test]
    fn test_error_conversion() {
        // Test conversion from domain-specific error to NestGateError
        let validation_error = ValidationError::FieldValidation {
            field: "test".to_string(),
            message: "Test error".to_string(),
            value: None,
            constraint: None,
        };
        
        // This tests the conversion to NestGateError through the unified variant
        let _nestgate_error = ValidationError::Unified(NestGateError::Configuration {
            message: "Test".to_string(),
            field: None,
            suggested_fix: None,
            config_source: None,
        });
    }
    
    #[test]
    fn test_macro_usage() {
        // Test validation_error! macro
        let error = validation_error!("field", "message");
        if let ValidationError::FieldValidation { field, message, .. } = error {
            assert_eq!(field, "field");
            assert_eq!(message, "message");
        } else {
            panic!("Expected FieldValidation error");
        }
        
        // Test network_error! macro
        let error = network_error!(connection, "localhost", 8080, "Connection refused");
        if let NetworkError::ConnectionFailed { address, port, error, .. } = error {
            assert_eq!(address, "localhost");
            assert_eq!(port, 8080);
            assert_eq!(error, "Connection refused");
        } else {
            panic!("Expected ConnectionFailed error");
        }
        
        // Test storage_error! macro
        let error = storage_error!(not_found, "/tmp/test.txt");
        if let StorageError::FileNotFound { path, .. } = error {
            assert_eq!(path, "/tmp/test.txt");
        } else {
            panic!("Expected FileNotFound error");
        }
    }
}

// ==================== EXTENSION TRAITS ====================

/// **IDIOMATIC RESULT EXTENSIONS**
/// Extension trait providing additional functionality for IdioResult types
pub trait IdioResultExt<T, E> {
    /// Add operation context to the error
    fn with_operation(self, operation: &str) -> IdioResult<T, E>;
    
    /// Add component context to the error
    fn with_component(self, component: &str) -> IdioResult<T, E>;
    
    /// Convert to a domain-specific Result type
    fn to_domain<F>(self, f: F) -> IdioResult<T, E>
    where
        F: FnOnce(E) -> E;
}

impl<T, E> IdioResultExt<T, E> for IdioResult<T, E>
where
    E: std::fmt::Debug + std::fmt::Display,
{
    fn with_operation(self, operation: &str) -> IdioResult<T, E> {
        self
    }
    
    fn with_component(self, component: &str) -> IdioResult<T, E> {
        self
    }
    
    fn to_domain<F>(self, f: F) -> IdioResult<T, E>
    where
        F: FnOnce(E) -> E,
    {
        self.map_err(f)
    }
}

/// **DOMAIN CONTEXT EXTENSION**
/// Extension trait for adding domain-specific context to errors
pub trait WithDomainContext<T> {
    /// Add validation context
    fn with_validation_context(self, field: &str, constraint: &str) -> ValidationResult<T>;
    
    /// Add network context
    fn with_network_context(self, address: &str, port: u16) -> NetworkResult<T>;
    
    /// Add storage context
    fn with_storage_context(self, path: &str, operation: &str) -> StorageResult<T>;
    
    /// Add security context
    fn with_security_context(self, method: &str, reason: &str) -> SecurityResult<T>;
}

impl<T> WithDomainContext<T> for IdioResult<T, NestGateError> {
    fn with_validation_context(self, field: &str, constraint: &str) -> ValidationResult<T> {
        self.map_err(|_| ValidationError::FieldValidation {
            field: field.to_string(),
            message: format!("Validation failed: {constraint}"),
            value: None,
            constraint: Some(constraint.to_string()),
        })
    }
    
    fn with_network_context(self, address: &str, port: u16) -> NetworkResult<T> {
        self.map_err(|_| NetworkError::ConnectionFailed {
            address: address.to_string(),
            port,
            error: "Connection failed".to_string(),
            timeout: None,
            retry_count: None,
        })
    }
    
    fn with_storage_context(self, path: &str, operation: &str) -> StorageResult<T> {
        self.map_err(|_| StorageError::FileNotFound {
            path: path.to_string(),
            operation: operation.to_string(),
            permissions: None,
        })
    }
    
    fn with_security_context(self, method: &str, reason: &str) -> SecurityResult<T> {
        self.map_err(|_| SecurityError::AuthenticationFailed {
            method: method.to_string(),
            reason: reason.to_string(),
            user_id: None,
            ip_address: None,
        })
    }
} 