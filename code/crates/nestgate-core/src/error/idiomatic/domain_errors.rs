// **DOMAIN-SPECIFIC ERROR TYPES**
//! Domain Errors functionality and utilities.
// Rich error types for specific domains with contextual information.

use serde::{Deserialize, Serialize,
        };
use crate::error::NestGateError;
use std::collections::HashMap;

/// **VALIDATION ERROR**
/// Rich error type for validation operations with field context
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum ValidationError {
    #[error("Field validation failed: {fiel,
            d:?,
        } - {message,
        }")]
    FieldValidation {
        field: Option<String>,
        message: String,
        constraint: Option<String>,
    },
    
    #[error("Schema validation failed: {schema,
        } - {message,
        }")]
    SchemaValidation {
        schema: String,
        message: String,
        path: Option<String>,
    },
    
    #[error("Unified validation error: {0,
        }")]
    Unified(#[from] NestGateError),
}
/// **NETWORK ERROR**
/// Rich error type for network operations with connection context
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum NetworkError {
    #[error("Connection failed: {address,
        }:{port,
        } - {error,
        }")]
    ConnectionFailed {
        address: String,
        port: u16,
        error: String,
        timeout: Option<std::tim,
            e::Duration>,
    },
    
    #[error("Request timeout: {url,
        } after {timeout:?,
        }")]
    Timeout {
        url: String,
        timeout: std::tim,
            e::Duration,
        method: Option<String>,
    },
    
    #[error("Unified network error: {0,
        }")]
    Unified(#[from] NestGateError),
}
/// **STORAGE ERROR**
/// Rich error type for storage operations with file/database context
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum StorageError {
    #[error("File not found: {path,
        }")]
    FileNotFound {
        path: String,
        operation: Option<String>,
    },
    
    #[error("Permission denied: {path,
        } - {operation:?,
        }")]
    PermissionDenied {
        path: String,
        operation: Option<String>,
        required_permissions: Option<String>,
    },
    
    #[error("Disk full: {path,
        } - {available,
        } bytes available")]
    DiskFull {
        path: String,
        available: u64,
        required: Option<u64>,
    },
    
    #[error("Unified storage error: {0,
        }")]
    Unified(#[from] NestGateError),
}
/// **SECURITY ERROR**
/// Rich error type for security operations with authentication context
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum SecurityError {
    #[error("Authentication failed: {principa,
            l:?,
        }")]
    AuthenticationFailed {
        principal: Option<String>,
        method: Option<String>,
    },
    
    #[error("Authorization denied: {operation,
        } for {principal:?,
        }")]
    AuthorizationDenied {
        operation: String,
        principal: Option<String>,
        required_permissions: Option<Vec<String>>,
    },
    
    #[error("Token expired: {token_type,
        } expired at {expiry:?,
        }")]
    TokenExpired {
        token_type: String,
        expiry: Option<std::tim,
            e::SystemTime>,
    },
    
    #[error("Unified security error: {0,
        }")]
    Unified(#[from] NestGateError),
}
/// **ZFS ERROR**
/// Rich error type for ZFS operations with pool/dataset context
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum ZfsError {
    #[error("Pool not found: {pool,
        }")]
    PoolNotFound {
        pool: String,
        available_pools: Option<Vec<String>>,
    },
    
    #[error("Dataset creation failed: {dataset,
        } - {error,
        }")]
    DatasetCreationFailed {
        dataset: String,
        error: String,
        parent_pool: Option<String>,
    },
    
    #[error("Snapshot operation failed: {operation,
        } on {target,
        } - {error,
        }")]
    SnapshotFailed {
        operation: String,
        target: String,
        error: String,
    },
    
    #[error("Unified ZFS error: {0,
        }")]
    Unified(#[from] NestGateError),
}
/// **API ERROR**
/// Rich error type for API operations with HTTP context
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum ApiError {
    #[error("HTTP {status_code,
        }: {method,
        } {path,
        } - {message,
        }")]
    HttpError {
        status_code: u16,
        method: String,
        path: String,
        message: String,
        headers: Option<HashMap<String, String>>,
    },
    
    #[error("Request validation failed: {field,
        } - {message,
        }")]
    RequestValidation {
        field: String,
        message: String,
        request_body: Option<String>,
    },
    
    #[error("Rate limit exceeded: {endpoint,
        } - {limit,
        } requests per {window:?,
        }")]
    RateLimitExceeded {
        endpoint: String,
        limit: u32,
        window: std::tim,
            e::Duration,
        retry_after: Option<std::tim,
            e::Duration>,
    },
    
    #[error("Unified API error: {0,
        }")]
    Unified(#[from] NestGateError),
}
/// **MCP ERROR**
/// Rich error type for MCP protocol operations with protocol context
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum McpError {
    #[error("Protocol version mismatch: expected={expected,
        }, got={currentvalue,
        }")]
    VersionMismatch {
        expected: String,
        currentvalue: String,
    },
    
    #[error("Message parsing failed: {message_type,
        } - {error,
        }")]
    MessageParsing {
        message_type: String,
        error: String,
        raw_message: Option<String>,
    },
    
    #[error("Connection state invalid: expected={expected,
        }, current={current,
        }")]
    InvalidState {
        expected: String,
        current: String,
    },
    
    #[error("Resource not found: {resource_type,
        }:{resource_id,
        }")]
    ResourceNotFound {
        resource_type: String,
        resource_id: String,
    },
    
    #[error("Protocol error: {version,
        } - {message_type,
        } (code: {error_cod,
            e:?,
        })")]
    ProtocolError {
        version: String,
        message_type: String,
        error_code: Option<i32>,
        request_id: Option<String>,
    },
    
    #[error("Unified MCP error: {0,
        }")]
    Unified(#[from] NestGateError),
}
/// **TESTING ERROR**
/// Rich error type for testing operations with test context
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum TestingError {
    #[error("Test assertion failed: {test_name,
        } - {message,
        }")]
    AssertionFailed {
        test_name: String,
        message: Stringactua,
            l: Option<String>,
    },
    
    #[error("Test setup failed: {test_name,
        } - {error,
        }")]
    SetupFailed {
        test_name: String,
        error: String,
        setup_step: Option<String>,
    },
    
    #[error("Unified testing error: {0,
        }")]
    Unified(#[from] NestGateError),
}
/// **PERFORMANCE ERROR**
/// Rich error type for performance operations with metric context
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum PerformanceError {
    #[error("Performance threshold exceeded: {metric,
        } = {value,
        } > {threshold,
        }")]
    ThresholdExceeded {
        metric: String,
        value: f64,
        threshold: f64,
        unit: Option<String>,
    },
    
    #[error("Benchmark failed: {benchmark,
        } - {error,
        }")]
    BenchmarkFailed {
        benchmark: String,
        error: String,
        duration: Option<std::tim,
            e::Duration>,
    },
    
    #[error("Unified performance error: {0,
        }")]
    Unified(#[from] NestGateError),
}
/// **HANDLER ERROR**
/// Rich error type for handler operations with request context
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum HandlerError {
    #[error("Handler execution failed: {handler,
        } - {error,
        }")]
    ExecutionFailed {
        handler: String,
        error: String,
        request_id: Option<String>,
    },
    
    #[error("Handler not found: {path,
        } {method,
        }")]
    NotFound {
        path: String,
        method: String,
    },
    
    #[error("Unified handler error: {0,
        }")]
    Unified(#[from] NestGateError),
}
/// **SERIALIZATION ERROR**
/// Rich error type for serialization operations with format context
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum SerializationError {
    #[error("Serialization failed: {format,
        } - {error,
        }")]
    SerializationFailed {
        format: String,
        error: String,
        data_type: Option<String>,
    },
    
    #[error("Deserialization failed: {format,
        } - {error,
        }")]
    DeserializationFailed {
        format: String,
        error: String,
        expected_type: Option<String>,
    },
    
    #[error("Unified serialization error: {0,
        }")]
    Unified(#[from] NestGateError),
}
/// **DATABASE ERROR**
/// Rich error type for database operations with query context
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum DatabaseError {
    #[error("Query execution failed: {query,
        } - {error,
        }")]
    QueryFailed {
        query: String,
        error: String,
        execution_time: Option<std::tim,
            e::Duration>,
    },
    
    #[error("Connection failed: {database,
        } - {error,
        }")]
    ConnectionFailed {
        database: String,
        error: String,
        host: Option<String>,
    },
    
    #[error("Transaction failed: {transaction_id,
        } - {error,
        }")]
    TransactionFailed {
        transaction_id: String,
        error: String,
        rollback_successful: Option<bool>,
    },
    
    #[error("Unified database error: {0,
        }")]
    Unified(#[from] NestGateError),
}
/// **CACHE ERROR**
/// Rich error type for cache operations with key context
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum CacheError {
    #[error("Cache miss: {key,
        }")]
    Miss {
        key: String,
        cache_type: Option<String>,
    },
    
    #[error("Cache write failed: {key,
        } - {error,
        }")]
    WriteFailed {
        key: String,
        error: String,
        ttl: Option<std::tim,
            e::Duration>,
    },
    
    #[error("Cache eviction: {key,
        } evicted due to {reason,
        }")]
    Evicted {
        key: String,
        reason: String,
    },
    
    #[error("Unified cache error: {0,
        }")]
    Unified(#[from] NestGateError),
}
/// **WORKFLOW ERROR**
/// Rich error type for workflow operations with step context
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum WorkflowError {
    #[error("Workflow step failed: {workflow_id,
        } step {step,
        } - {error,
        }")]
    StepFailed {
        workflow_id: String,
        step: String,
        error: String,
        step_index: Option<u32>,
    },
    
    #[error("Workflow timeout: {workflow_id,
        } exceeded {timeout:?,
        }")]
    Timeout {
        workflow_id: String,
        timeout: std::tim,
            e::Duration,
        current_step: Option<String>,
    },
    
    #[error("Workflow dependency failed: {dependency,
        } required by {workflow_id,
        }")]
    DependencyFailed {
        workflow_id: String,
        dependency: String,
        error: String,
    },
    
    #[error("Workflow state error: {workflow_id,
        } in invalid state {state,
        }")]
    InvalidState {
        workflow_id: String,
        state: String,
        expected_states: Vec<String>,
    },
    
    #[error("Unified workflow error: {0,
        }")]
    Unified(#[from] NestGateError),
}
/// **MONITORING ERROR**
/// Rich error type for monitoring operations with metric context
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum MonitoringError {
    #[error("Metric collection failed: {metric,
        } - {error,
        }")]
    CollectionFailed {
        metric: String,
        error: String,
        timestamp: Option<std::tim,
            e::SystemTime>,
    },
    
    #[error("Alert threshold breached: {metric,
        } = {value,
        } {operator,
        } {threshold,
        }")]
    ThresholdBreached {
        metric: String,
        value: f64,
        operator: String,
        threshold: f64,
        severity: Option<String>,
    },
    
    #[error("Monitoring system unavailable: {system,
        } - {error,
        }")]
    SystemUnavailable {
        system: String,
        error: String,
        retry_after: Option<std::tim,
            e::Duration>,
    },
    
    #[error("Unified monitoring error: {0,
        }")]
    Unified(#[from] NestGateError),
} 