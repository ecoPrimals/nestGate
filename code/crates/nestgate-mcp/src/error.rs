//
// This module provides MCP-specific error handling that integrates seamlessly
// with the canonical NestGateError system. All MCP errors are now represented
// as NestGateError::Mcp variants with rich context and recovery suggestions.

// McpErrorData moved to unified error system
use nestgate_core::error::McpErrorData;
use nestgate_core::error::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

// ==================== SECTION ====================

/// Canonical error creation helpers for MCP operations
pub struct McpErrorBuilder;

impl McpErrorBuilder {
    /// Create a canonical MCP connection error
    pub fn connection_error(message: &str, endpoint: Option<&str>) -> NestGateError {
        let mut context = HashMap::new();
        context.insert("error_type".to_string(), "ConnectionError".to_string());
        if let Some(ep) = endpoint {
            context.insert("endpoint".to_string(), ep.to_string());
        }
        context.insert(
            "recovery_suggestions".to_string(),
            "Check network connectivity; Verify MCP server is running; Check firewall settings"
                .to_string(),
        );

        NestGateError::Mcp {
            message: message.to_string(),
            operation: "connection".to_string(),
            connection_id: None,
            protocol_version: Some("1.0".to_string()),
            mcp_data: Some(McpErrorData {
                message_type: "connection_error".to_string(),
                protocol_version: "1.0".to_string(),
                connection_state: "disconnected".to_string(),
                sequence_number: None,
            }),
            context: None,
        }
    }

    /// Create a canonical MCP protocol error
    pub fn protocol_error(message: &str, method: Option<&str>) -> NestGateError {
        let mut context = HashMap::new();
        context.insert("error_type".to_string(), "ProtocolError".to_string());
        if let Some(m) = method {
            context.insert("method".to_string(), m.to_string());
        }
        context.insert("recovery_suggestions".to_string(), 
            "Check MCP protocol version compatibility; Verify message format; Review MCP specification".to_string());

        NestGateError::Mcp {
            message: message.to_string(),
            operation: "protocol".to_string(),
            connection_id: None,
            protocol_version: Some("1.0".to_string()),
            mcp_data: Some(McpErrorData {
                message_type: "protocol_error".to_string(),
                protocol_version: "1.0".to_string(),
                connection_state: "connected".to_string(),
                sequence_number: None,
            }),
            context: None,
        }
    }

    /// Create a canonical MCP authentication error
    pub fn authentication_error(message: &str, endpoint: Option<&str>) -> NestGateError {
        let mut context = HashMap::new();
        context.insert("error_type".to_string(), "AuthenticationError".to_string());
        if let Some(ep) = endpoint {
            context.insert("endpoint".to_string(), ep.to_string());
        }
        context.insert(
            "recovery_suggestions".to_string(),
            "Check authentication credentials; Verify API keys are valid; Check token expiration"
                .to_string(),
        );

        NestGateError::Mcp {
            message: message.to_string(),
            operation: "authentication".to_string(),
            connection_id: None,
            protocol_version: Some("1.0".to_string()),
            mcp_data: Some(McpErrorData {
                message_type: "authentication_error".to_string(),
                protocol_version: "1.0".to_string(),
                connection_state: "connected".to_string(),
                sequence_number: None,
            }),
            context: None,
        }
    }

    /// Create a canonical MCP timeout error
    pub fn timeout_error(
        message: &str,
        method: Option<&str>,
        timeout_ms: Option<u64>,
    ) -> NestGateError {
        let mut context = HashMap::new();
        context.insert("error_type".to_string(), "TimeoutError".to_string());
        if let Some(m) = method {
            context.insert("method".to_string(), m.to_string());
        }
        if let Some(ms) = timeout_ms {
            context.insert("timeout_ms".to_string(), ms.to_string());
        }
        context.insert(
            "recovery_suggestions".to_string(),
            "Increase timeout duration; Check server performance; Verify network latency"
                .to_string(),
        );

        NestGateError::Mcp {
            message: message.to_string(),
            operation: "timeout".to_string(),
            connection_id: None,
            protocol_version: Some("1.0".to_string()),
            mcp_data: Some(McpErrorData {
                message_type: "timeout_error".to_string(),
                protocol_version: "1.0".to_string(),
                connection_state: "connected".to_string(),
                sequence_number: None,
            }),
            context: None,
        }
    }

    /// Create a canonical MCP validation error
    pub fn validation_error(message: &str, field: Option<&str>) -> NestGateError {
        let mut context = HashMap::new();
        context.insert("error_type".to_string(), "ValidationError".to_string());
        if let Some(f) = field {
            context.insert("field".to_string(), f.to_string());
        }
        context.insert(
            "recovery_suggestions".to_string(),
            "Check input parameters; Verify data format; Review field requirements".to_string(),
        );

        NestGateError::Mcp {
            message: message.to_string(),
            operation: "validation".to_string(),
            connection_id: None,
            protocol_version: Some("1.0".to_string()),
            mcp_data: Some(McpErrorData {
                message_type: "validation_error".to_string(),
                protocol_version: "1.0".to_string(),
                connection_state: "connected".to_string(),
                sequence_number: None,
            }),
            context: None,
        }
    }
}

// ==================== SECTION ====================
// These are kept for internal compatibility but not exposed in public API

/// Internal error type enumeration for backward compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorType {
    ConnectionError,
    ProtocolError,
    AuthenticationError,
    TimeoutError,
    ValidationError,
    InternalError,
    Network,
    Auth,
    Authorization,
    NotFound,
    InvalidRequest,
    ServerError,
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorType::ConnectionError => write!(f, "ConnectionError"),
            ErrorType::ProtocolError => write!(f, "ProtocolError"),
            ErrorType::AuthenticationError => write!(f, "AuthenticationError"),
            ErrorType::TimeoutError => write!(f, "TimeoutError"),
            ErrorType::ValidationError => write!(f, "ValidationError"),
            ErrorType::InternalError => write!(f, "InternalError"),
            ErrorType::Network => write!(f, "Network"),
            ErrorType::Auth => write!(f, "Auth"),
            ErrorType::Authorization => write!(f, "Authorization"),
            ErrorType::NotFound => write!(f, "NotFound"),
            ErrorType::InvalidRequest => write!(f, "InvalidRequest"),
            ErrorType::ServerError => write!(f, "ServerError"),
        }
    }
}

// ==================== SECTION ====================

/// Convert legacy error type to canonical MCP error
pub fn create_mcp_error(error_type: ErrorType, message: String) -> NestGateError {
    let mut context = HashMap::new();
    context.insert("error_type".to_string(), error_type.to_string());

    let (operation, recovery_suggestions) = match &error_type {
        ErrorType::ConnectionError => ("connection", "Check network connectivity; Verify MCP server is running; Check firewall settings"),
        ErrorType::ProtocolError => ("protocol", "Check MCP protocol version compatibility; Verify message format; Review MCP specification"),
        ErrorType::AuthenticationError | ErrorType::Auth => ("authentication", "Check authentication credentials; Verify API keys are valid; Check token expiration"),
        ErrorType::Authorization => ("authorization", "Check user permissions; Verify access rights; Contact administrator if needed"),
        ErrorType::TimeoutError => ("timeout", "Increase timeout duration; Check server performance; Verify network latency"),
        ErrorType::ValidationError => ("validation", "Check input parameters; Verify data format; Review field requirements"),
        ErrorType::Network => ("network", "Check network connection; Verify DNS resolution; Check proxy settings"),
        ErrorType::NotFound => ("not_found", "Check resource path; Verify resource exists; Check permissions"),
        ErrorType::InvalidRequest => ("invalid_request", "Review request format; Check required parameters; Verify API documentation"),
        ErrorType::ServerError | ErrorType::InternalError => ("server_error", "Retry the operation; Check server logs; Contact support if problem persists"),
    };

    context.insert(
        "recovery_suggestions".to_string(),
        recovery_suggestions.to_string(),
    );

    NestGateError::Mcp {
        message: format!("MCP {} operation failed", operation),
        operation: operation.to_string(),
        connection_id: None,
        protocol_version: Some("1.0".to_string()),
        mcp_data: Some(McpErrorData {
            message_type: operation.to_string(),
            protocol_version: "1.0".to_string(),
            connection_state: "connected".to_string(),
            sequence_number: None,
        }),
        context: None,
    }
}

/// Create an internal MCP error
pub fn internal_error(message: String) -> NestGateError {
    create_mcp_error(ErrorType::InternalError, message)
}

/// Create a network MCP error
pub fn network_error(err: impl fmt::Display) -> NestGateError {
    create_mcp_error(ErrorType::Network, format!("Network error: {err}"))
}

/// Create an authentication MCP error
pub fn authentication_error(message: String) -> NestGateError {
    create_mcp_error(ErrorType::AuthenticationError, message)
}

/// Create an authorization MCP error
pub fn authorization_error(message: String) -> NestGateError {
    create_mcp_error(ErrorType::Authorization, message)
}

/// Create a connection MCP error
pub fn connection_error(message: String) -> NestGateError {
    create_mcp_error(ErrorType::ConnectionError, message)
}

/// Create a protocol MCP error
pub fn protocol_error(message: String) -> NestGateError {
    create_mcp_error(ErrorType::ProtocolError, message)
}

/// Create a timeout MCP error
pub fn timeout_error(message: String) -> NestGateError {
    create_mcp_error(ErrorType::TimeoutError, message)
}

/// Create a validation MCP error
pub fn validation_error(message: String) -> NestGateError {
    create_mcp_error(ErrorType::ValidationError, message)
}

/// Create a not found MCP error
pub fn not_found_error(message: String) -> NestGateError {
    create_mcp_error(ErrorType::NotFound, message)
}

/// Create an invalid request MCP error
pub fn invalid_request_error(message: String) -> NestGateError {
    create_mcp_error(ErrorType::InvalidRequest, message)
}

/// Create a server MCP error
pub fn server_error(message: String) -> NestGateError {
    create_mcp_error(ErrorType::ServerError, message)
}

// ==================== SECTION ====================

/// Convert any MCP operation result to canonical Result<T>
pub fn to_canonical_result<T, E: Into<NestGateError>>(
    result: std::result::Result<T, E>,
) -> Result<T> {
    result.map_err(|e| e.into())
}

/// Create a canonical MCP error with full context
pub fn create_contextual_error(
    error_type: ErrorType,
    message: String,
    endpoint: Option<String>,
    method: Option<String>,
    request_id: Option<String>,
) -> NestGateError {
    let mut context = HashMap::new();
    context.insert("error_type".to_string(), error_type.to_string());

    if let Some(ep) = endpoint {
        context.insert("endpoint".to_string(), ep);
    }
    if let Some(m) = method {
        context.insert("method".to_string(), m);
    }

    let (operation, recovery_suggestions) = match &error_type {
        ErrorType::ConnectionError => ("connection", "Check network connectivity; Verify MCP server is running; Check firewall settings"),
        ErrorType::ProtocolError => ("protocol", "Check MCP protocol version compatibility; Verify message format; Review MCP specification"),
        ErrorType::AuthenticationError | ErrorType::Auth => ("authentication", "Check authentication credentials; Verify API keys are valid; Check token expiration"),
        ErrorType::TimeoutError => ("timeout", "Increase timeout duration; Check server performance; Verify network latency"),
        ErrorType::ValidationError => ("validation", "Check input parameters; Verify data format; Review field requirements"),
        _ => ("general", "Check MCP server status; Verify configuration; Retry the operation"),
    };

    context.insert(
        "recovery_suggestions".to_string(),
        recovery_suggestions.to_string(),
    );

    NestGateError::Mcp {
        message: format!("MCP {} operation failed", operation),
        operation: operation.to_string(),
        connection_id: request_id.clone(),
        protocol_version: Some("1.0".to_string()),
        mcp_data: Some(McpErrorData {
            message_type: operation.to_string(),
            protocol_version: "1.0".to_string(),
            connection_state: "connected".to_string(),
            sequence_number: request_id.and_then(|id| id.parse().ok()),
        }),
        context: None,
    }
}
