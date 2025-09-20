// **ERROR CREATION MACROS**
//! Macros functionality and utilities.
// Modern, idiomatic error creation macros for unified error handling.

/// Create a network error with rich context
#[macro_export]
macro_rules! network_error {
    (connection, $address:expr, $port:expr, $error:expr) => {
        $crate::error::idiomatic::NetworkError::ConnectionFailed {
            address: $address.to_string(),
            port: $port,
            error: $error.to_string(),
            timeout: None,
        }
    };
    
    (timeout, $url:expr, $timeout:expr) => {
        $crate::error::idiomatic::NetworkError::Timeout {
            url: $url.to_string(),
            timeout: $timeout,
            method: None,
        }
    };
}
/// Create a storage error with rich context
#[macro_export]
macro_rules! storage_error {
    (not_found, $path:expr) => {
        $crate::error::idiomatic::StorageError::FileNotFound {
            path: $path.to_string(),
        }
    };
    
    (permission_denied, $path:expr, $operation:expr) => {
        $crate::error::idiomatic::StorageError::PermissionDenied {
            path: $path.to_string(),
            operation: Some($operation.to_string()),
            required_permissions: None,
        }
    };
    
    (disk_full, $path:expr, $available:expr) => {
        $crate::error::idiomatic::StorageError::DiskFull {
            path: $path.to_string(),
            available: $available,
            required: None,
        }
    };
}
/// Create a validation error with rich context
#[macro_export]
macro_rules! validation_error {
    (field, $field:expr, $message:expr) => {
        $crate::error::idiomatic::ValidationError::FieldValidation {
            field: Some($field.to_string()),
            message: $message.to_string(),
            constraint: None,
        }
    };
    
    (schema, $schema:expr, $message:expr) => {
        $crate::error::idiomatic::ValidationError::SchemaValidation {
            schema: $schema.to_string(),
            message: $message.to_string(),
            path: None,
        }
    };
}
/// Create a security error with rich context
#[macro_export]
macro_rules! security_error {
    (auth_failed, $principal:expr) => {
        $crate::error::idiomatic::SecurityError::AuthenticationFailed {
            principal: Some($principal.to_string()),
            method: None,
        }
    };
    
    (auth_denied, $operation:expr, $principal:expr) => {
        $crate::error::idiomatic::SecurityError::AuthorizationDenied {
            operation: $operation.to_string(),
            principal: Some($principal.to_string()),
            required_permissions: None,
        }
    };
    
    (token_expired, $token_type:expr) => {
        $crate::error::idiomatic::SecurityError::TokenExpired {
            token_type: $token_type.to_string(),
            expiry: None,
        }
    };
}
/// Create a ZFS error with rich context
#[macro_export]
macro_rules! zfs_error {
    (pool_not_found, $pool:expr) => {
        $crate::error::idiomatic::ZfsError::PoolNotFound {
            pool: $pool.to_string(),
            available_pools: None,
        }
    };
    
    (dataset_creation_failed, $dataset:expr, $error:expr) => {
        $crate::error::idiomatic::ZfsError::DatasetCreationFailed {
            dataset: $dataset.to_string(),
            error: $error.to_string(),
            parent_pool: None,
        }
    };
    
    (snapshot_failed, $operation:expr, $target:expr, $error:expr) => {
        $crate::error::idiomatic::ZfsError::SnapshotFailed {
            operation: $operation.to_string(),
            target: $target.to_string(),
            error: $error.to_string(),
        }
    };
}
/// Create an API error with rich context
#[macro_export]
macro_rules! api_error {
    (http, $status_code:expr, $method:expr, $path:expr, $message:expr) => {
        $crate::error::idiomatic::ApiError::HttpError {
            status_code: $status_code,
            method: $method.to_string(),
            path: $path.to_string(),
            message: $message.to_string(),
            headers: None,
        }
    };
    
    (validation, $field:expr, $message:expr) => {
        $crate::error::idiomatic::ApiError::RequestValidation {
            field: $field.to_string(),
            message: $message.to_string(),
            request_body: None,
        }
    };
    
    (rate_limit, $endpoint:expr, $limit:expr, $window:expr) => {
        $crate::error::idiomatic::ApiError::RateLimitExceeded {
            endpoint: $endpoint.to_string(),
            limit: $limit,
            window: $window,
            retry_after: None,
        }
    };
}
/// Create an MCP error with rich context
#[macro_export]
macro_rules! mcp_error {
    (version_mismatch, $expected:expr, $current:expr) => {
        $crate::error::idiomatic::McpError::VersionMismatch {
            expected: $expected.to_string(),
            currentvalue: $current.to_string(),
        }
    };
    
    (message_parsing, $message_type:expr, $error:expr) => {
        $crate::error::idiomatic::McpError::MessageParsing {
            message_type: $message_type.to_string(),
            error: $error.to_string(),
            raw_message: None,
        }
    };
    
    (invalid_state, $expected:expr, $current:expr) => {
        $crate::error::idiomatic::McpError::InvalidState {
            expected: $expected.to_string(),
            current: $current.to_string(),
        }
    };
    
    (resource_not_found, $resource_type:expr, $resource_id:expr) => {
        $crate::error::idiomatic::McpError::ResourceNotFound {
            resource_type: $resource_type.to_string(),
            resource_id: $resource_id.to_string(),
        }
    };
}
// Note: Macros are automatically available when this module is imported 