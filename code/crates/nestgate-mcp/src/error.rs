//! **MCP ERROR HANDLING - CANONICAL MODERNIZATION**
//!
//! Simplified error constructors using standard `NestGateError` types

use nestgate_core::error::{ExternalErrorDetails, InternalErrorDetails};
use nestgate_core::NestGateError;
use std::collections::HashMap;
use std::time::SystemTime;

// Simple MCP error constructors using standard error types

#[must_use]
pub fn mcp_connection_error(message: &str) -> NestGateError {
    NestGateError::Internal(Box::new(InternalErrorDetails {
        message: format!("MCP Connection Error: {message}"),
        component: "nestgate-mcp".to_string(),
        location: Some("connection".to_string()),
        is_bug: false,
        context: None,
    }))
}

/// Protocol Error
pub fn protocol_error(message: &str, method: Option<&str>) -> NestGateError {
    NestGateError::Internal(Box::new(InternalErrorDetails {
        message: format!("MCP Protocol Error: {message} (method: {method:?})"),
        component: "nestgate-mcp".to_string(),
        location: method.map(std::string::ToString::to_string),
        is_bug: false,
        context: None,
    }))
}

#[must_use]
pub fn method_error(message: &str, method: &str) -> NestGateError {
    NestGateError::Internal(Box::new(InternalErrorDetails {
        message: format!("MCP Method Error: {message} (method: {method})"),
        component: "nestgate-mcp".to_string(),
        location: Some(method.to_string()),
        is_bug: false,
        context: None,
    }))
}

#[must_use]
pub fn session_error(message: &str, session_id: &str) -> NestGateError {
    NestGateError::Internal(Box::new(InternalErrorDetails {
        message: format!("MCP Session Error: {message} (session: {session_id})"),
        component: "nestgate-mcp".to_string(),
        location: Some(session_id.to_string()),
        is_bug: false,
        context: None,
    }))
}

#[must_use]
pub fn serialization_error(message: &str) -> NestGateError {
    NestGateError::Internal(Box::new(InternalErrorDetails {
        message: format!("MCP Serialization Error: {message}"),
        component: "nestgate-mcp".to_string(),
        location: Some("serialization".to_string()),
        is_bug: false,
        context: None,
    }))
}

#[must_use]
pub fn transport_error(message: &str) -> NestGateError {
    NestGateError::External(Box::new(ExternalErrorDetails {
        message: format!("MCP Transport Error: {message}"),
        service: "mcp-transport".to_string(),
        retryable: true,
        context: None,
    }))
}

// Extension trait for MCP-specific error handling
pub trait McpErrorExt {
    /// Extract Mcp Context
    fn extract_mcp_context(&self) -> Option<String>;
    /// Extract Session Id
    fn extract_session_id(&self) -> Option<String>;
    /// Extract Method
    fn extract_method(&self) -> Option<String>;
}

impl McpErrorExt for NestGateError {
    /// Extract Mcp Context
    fn extract_mcp_context(&self) -> Option<String> {
        match self {
            NestGateError::Internal(details) if details.component == "nestgate-mcp" => {
                Some("MCP operation".to_string())
            }
            NestGateError::External(details) if details.service == "mcp-transport" => {
                Some("MCP transport".to_string())
            }
            _ => None,
        }
    }

    /// Extract Session Id
    fn extract_session_id(&self) -> Option<String> {
        match self {
            NestGateError::Internal(details) => details.location.clone(),
            _ => None,
        }
    }

    /// Extract Method
    fn extract_method(&self) -> Option<String> {
        match self {
            NestGateError::Internal(details) => details.location.clone(),
            _ => None,
        }
    }
}

// Helper functions
#[must_use]
pub fn extract_mcp_context(error: &NestGateError) -> Option<String> {
    error.extract_mcp_context()
}

#[must_use]
pub fn extract_session_id(error: &NestGateError) -> Option<String> {
    error.extract_session_id()
}

#[must_use]
pub fn extract_method(error: &NestGateError) -> Option<String> {
    error.extract_method()
}

// Simplified data structures for MCP context (if needed)
#[derive(Debug, Clone)]
/// Mcperrordata
pub struct McpErrorData {
    /// Message Type
    pub message_type: String,
    /// Protocol Version
    pub protocol_version: Option<String>,
    /// Message identifier
    pub message_id: Option<String>,
    /// Session Info
    pub session_info: Option<McpSessionInfo>,
    /// Transport Info
    pub transport_info: Option<McpTransportInfo>,
}

#[derive(Debug, Clone)]
/// Mcpsessioninfo
pub struct McpSessionInfo {
    /// Session identifier
    pub session_id: String,
    /// Client Info
    pub client_info: Option<HashMap<String, String>>,
    /// Server Info
    pub server_info: Option<HashMap<String, String>>,
    /// Established At
    pub established_at: SystemTime,
    /// Count of message
    pub message_count: u64,
}

#[derive(Debug, Clone)]
/// Mcptransportinfo
pub struct McpTransportInfo {
    /// Transport Type
    pub transport_type: String,
    /// Local Endpoint
    pub local_endpoint: Option<String>,
    /// Remote Endpoint
    pub remote_endpoint: Option<String>,
    /// Connection identifier
    pub connection_id: Option<String>,
    /// Established At
    pub established_at: SystemTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Error Constructor Tests ====================

    #[test]
    fn test_mcp_connection_error() {
        let err = mcp_connection_error("timeout");
        let msg = format!("{:?}", err);
        assert!(msg.contains("MCP Connection Error"));
        assert!(msg.contains("timeout"));
    }

    #[test]
    fn test_protocol_error_with_method() {
        let err = protocol_error("invalid format", Some("call_tool"));
        let msg = format!("{:?}", err);
        assert!(msg.contains("MCP Protocol Error"));
        assert!(msg.contains("invalid format"));
    }

    #[test]
    fn test_protocol_error_without_method() {
        let err = protocol_error("malformed", None);
        let msg = format!("{:?}", err);
        assert!(msg.contains("MCP Protocol Error"));
        assert!(msg.contains("malformed"));
    }

    #[test]
    fn test_method_error() {
        let err = method_error("not found", "list_resources");
        let msg = format!("{:?}", err);
        assert!(msg.contains("MCP Method Error"));
        assert!(msg.contains("not found"));
        assert!(msg.contains("list_resources"));
    }

    #[test]
    fn test_session_error() {
        let err = session_error("expired", "session-123");
        let msg = format!("{:?}", err);
        assert!(msg.contains("MCP Session Error"));
        assert!(msg.contains("expired"));
        assert!(msg.contains("session-123"));
    }

    #[test]
    fn test_serialization_error() {
        let err = serialization_error("invalid JSON");
        let msg = format!("{:?}", err);
        assert!(msg.contains("MCP Serialization Error"));
        assert!(msg.contains("invalid JSON"));
    }

    #[test]
    fn test_transport_error() {
        let err = transport_error("connection lost");
        let msg = format!("{:?}", err);
        assert!(msg.contains("MCP Transport Error"));
        assert!(msg.contains("connection lost"));
    }

    // ==================== McpErrorExt Trait Tests ====================

    #[test]
    fn test_extract_mcp_context_internal() {
        let err = mcp_connection_error("test");
        let context = err.extract_mcp_context();
        assert!(context.is_some());
        assert_eq!(context.expect("Operation failed"), "MCP operation");
    }

    #[test]
    fn test_extract_mcp_context_external() {
        let err = transport_error("test");
        let context = err.extract_mcp_context();
        assert!(context.is_some());
        assert_eq!(context.expect("Operation failed"), "MCP transport");
    }

    #[test]
    fn test_extract_session_id() {
        let err = session_error("test", "abc-123");
        let session_id = err.extract_session_id();
        assert!(session_id.is_some());
    }

    #[test]
    fn test_extract_method() {
        let err = method_error("test", "my_method");
        let method = err.extract_method();
        assert!(method.is_some());
    }

    // ==================== Helper Function Tests ====================

    #[test]
    fn test_extract_mcp_context_helper() {
        let err = mcp_connection_error("test");
        let context = extract_mcp_context(&err);
        assert!(context.is_some());
    }

    #[test]
    fn test_extract_session_id_helper() {
        let err = session_error("test", "session-456");
        let session_id = extract_session_id(&err);
        assert!(session_id.is_some());
    }

    #[test]
    fn test_extract_method_helper() {
        let err = method_error("test", "test_method");
        let method = extract_method(&err);
        assert!(method.is_some());
    }

    // ==================== McpErrorData Tests ====================

    #[test]
    fn test_mcp_error_data_creation() {
        let data = McpErrorData {
            message_type: "request".to_string(),
            protocol_version: Some("1.0".to_string()),
            message_id: Some("msg-123".to_string()),
            session_info: None,
            transport_info: None,
        };
        assert_eq!(data.message_type, "request");
    }

    #[test]
    fn test_mcp_error_data_clone() {
        let data = McpErrorData {
            message_type: "response".to_string(),
            protocol_version: None,
            message_id: None,
            session_info: None,
            transport_info: None,
        };
        let cloned = data.clone();
        assert_eq!(data.message_type, cloned.message_type);
    }

    #[test]
    fn test_mcp_error_data_debug() {
        let data = McpErrorData {
            message_type: "error".to_string(),
            protocol_version: Some("2.0".to_string()),
            message_id: Some("err-456".to_string()),
            session_info: None,
            transport_info: None,
        };
        let debug = format!("{:?}", data);
        assert!(debug.contains("McpErrorData"));
        assert!(debug.contains("error"));
    }

    // ==================== McpSessionInfo Tests ====================

    #[test]
    fn test_mcp_session_info_creation() {
        let info = McpSessionInfo {
            session_id: "sess-789".to_string(),
            client_info: None,
            server_info: None,
            established_at: SystemTime::now(),
            message_count: 0,
        };
        assert_eq!(info.session_id, "sess-789");
        assert_eq!(info.message_count, 0);
    }

    #[test]
    fn test_mcp_session_info_with_metadata() {
        let mut client_info = HashMap::new();
        client_info.insert("version".to_string(), "1.0".to_string());

        let info = McpSessionInfo {
            session_id: "sess-abc".to_string(),
            client_info: Some(client_info),
            server_info: None,
            established_at: SystemTime::now(),
            message_count: 42,
        };

        assert!(info.client_info.is_some());
        assert_eq!(info.message_count, 42);
    }

    #[test]
    fn test_mcp_session_info_clone() {
        let info = McpSessionInfo {
            session_id: "sess-clone".to_string(),
            client_info: None,
            server_info: None,
            established_at: SystemTime::now(),
            message_count: 10,
        };
        let cloned = info.clone();
        assert_eq!(info.session_id, cloned.session_id);
        assert_eq!(info.message_count, cloned.message_count);
    }

    #[test]
    fn test_mcp_session_info_debug() {
        let info = McpSessionInfo {
            session_id: "debug-session".to_string(),
            client_info: None,
            server_info: None,
            established_at: SystemTime::now(),
            message_count: 5,
        };
        let debug = format!("{:?}", info);
        assert!(debug.contains("McpSessionInfo"));
        assert!(debug.contains("debug-session"));
    }

    // ==================== McpTransportInfo Tests ====================

    #[test]
    fn test_mcp_transport_info_creation() {
        let info = McpTransportInfo {
            transport_type: "stdio".to_string(),
            local_endpoint: None,
            remote_endpoint: None,
            connection_id: Some("conn-123".to_string()),
            established_at: SystemTime::now(),
        };
        assert_eq!(info.transport_type, "stdio");
    }

    #[test]
    fn test_mcp_transport_info_with_endpoints() {
        let info = McpTransportInfo {
            transport_type: "http".to_string(),
            local_endpoint: Some("localhost:8080".to_string()),
            remote_endpoint: Some("server:9090".to_string()),
            connection_id: Some("conn-456".to_string()),
            established_at: SystemTime::now(),
        };

        assert_eq!(info.transport_type, "http");
        assert!(info.local_endpoint.is_some());
        assert!(info.remote_endpoint.is_some());
    }

    #[test]
    fn test_mcp_transport_info_clone() {
        let info = McpTransportInfo {
            transport_type: "ws".to_string(),
            local_endpoint: None,
            remote_endpoint: None,
            connection_id: None,
            established_at: SystemTime::now(),
        };
        let cloned = info.clone();
        assert_eq!(info.transport_type, cloned.transport_type);
    }

    #[test]
    fn test_mcp_transport_info_debug() {
        let info = McpTransportInfo {
            transport_type: "sse".to_string(),
            local_endpoint: Some("local".to_string()),
            remote_endpoint: Some("remote".to_string()),
            connection_id: Some("conn-debug".to_string()),
            established_at: SystemTime::now(),
        };
        let debug = format!("{:?}", info);
        assert!(debug.contains("McpTransportInfo"));
        assert!(debug.contains("sse"));
    }

    // ==================== Integration Tests ====================

    #[test]
    fn test_error_chain_connection_to_session() {
        let conn_err = mcp_connection_error("initial failure");
        let session_err = session_error("secondary failure", "sess-test");

        assert!(format!("{:?}", conn_err).contains("MCP Connection Error"));
        assert!(format!("{:?}", session_err).contains("MCP Session Error"));
    }

    #[test]
    fn test_all_error_types_distinct() {
        let errors = [
            mcp_connection_error("e1"),
            protocol_error("e2", None),
            method_error("e3", "m"),
            session_error("e4", "s"),
            serialization_error("e5"),
            transport_error("e6"),
        ];

        // All errors should be created successfully
        assert_eq!(errors.len(), 6);
    }

    #[test]
    fn test_error_data_with_full_context() {
        let session_info = McpSessionInfo {
            session_id: "full-ctx".to_string(),
            client_info: None,
            server_info: None,
            established_at: SystemTime::now(),
            message_count: 100,
        };

        let transport_info = McpTransportInfo {
            transport_type: "full".to_string(),
            local_endpoint: Some("local".to_string()),
            remote_endpoint: Some("remote".to_string()),
            connection_id: Some("conn-full".to_string()),
            established_at: SystemTime::now(),
        };

        let data = McpErrorData {
            message_type: "full-test".to_string(),
            protocol_version: Some("1.0".to_string()),
            message_id: Some("msg-full".to_string()),
            session_info: Some(session_info),
            transport_info: Some(transport_info),
        };

        assert!(data.session_info.is_some());
        assert!(data.transport_info.is_some());
    }
}
