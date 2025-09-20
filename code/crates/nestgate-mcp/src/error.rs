//! **MCP ERROR HANDLING - CANONICAL MODERNIZATION**
//!
//! Simplified error constructors using standard NestGateError types

use nestgate_core::error::{ExternalErrorDetails, InternalErrorDetails};
use nestgate_core::NestGateError;
use std::collections::HashMap;
use std::time::SystemTime;

// Simple MCP error constructors using standard error types

pub const fn mcp_connection_error(message: &str) -> NestGateError {
    NestGateError::Internal(Box::new(InternalErrorDetails {
        message: format!("MCP Connection Error: {message}"),
        component: "nestgate-mcp".to_string(),
        location: Some("connection".to_string()),
        is_bug: false,
        context: None,
    }))
}

pub const fn protocol_error(message: &str, method: Option<&str>) -> NestGateError {
    NestGateError::Internal(Box::new(InternalErrorDetails {
        message: format!("MCP Protocol Error: {message} (method: {method:?})"),
        component: "nestgate-mcp".to_string(),
        location: method.map(|m| m.to_string()),
        is_bug: false,
        context: None,
    }))
}

pub const fn method_error(message: &str, method: &str) -> NestGateError {
    NestGateError::Internal(Box::new(InternalErrorDetails {
        message: format!("MCP Method Error: {message} (method: {method})"),
        component: "nestgate-mcp".to_string(),
        location: Some(method.to_string()),
        is_bug: false,
        context: None,
    }))
}

pub const fn session_error(message: &str, session_id: &str) -> NestGateError {
    NestGateError::Internal(Box::new(InternalErrorDetails {
        message: format!("MCP Session Error: {message} (session: {session_id})"),
        component: "nestgate-mcp".to_string(),
        location: Some(session_id.to_string()),
        is_bug: false,
        context: None,
    }))
}

pub const fn serialization_error(message: &str) -> NestGateError {
    NestGateError::Internal(Box::new(InternalErrorDetails {
        message: format!("MCP Serialization Error: {message}"),
        component: "nestgate-mcp".to_string(),
        location: Some("serialization".to_string()),
        is_bug: false,
        context: None,
    }))
}

pub const fn transport_error(message: &str) -> NestGateError {
    NestGateError::External(Box::new(ExternalErrorDetails {
        message: format!("MCP Transport Error: {message}"),
        service: "mcp-transport".to_string(),
        retryable: true,
        context: None,
    }))
}

// Extension trait for MCP-specific error handling
pub trait McpErrorExt {
    fn extract_mcp_context(&self) -> Option<String>;
    fn extract_session_id(&self) -> Option<String>;
    fn extract_method(&self) -> Option<String>;
}

impl McpErrorExt for NestGateError {
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

    fn extract_session_id(&self) -> Option<String> {
        match self {
            NestGateError::Internal(details) => details.location.clone(),
            _ => None,
        }
    }

    fn extract_method(&self) -> Option<String> {
        match self {
            NestGateError::Internal(details) => details.location.clone(),
            _ => None,
        }
    }
}

// Helper functions
pub const fn extract_mcp_context(error: &NestGateError) -> Option<String> {
    error.extract_mcp_context()
}

pub const fn extract_session_id(error: &NestGateError) -> Option<String> {
    error.extract_session_id()
}

pub const fn extract_method(error: &NestGateError) -> Option<String> {
    error.extract_method()
}

// Simplified data structures for MCP context (if needed)
#[derive(Debug, Clone)]
pub struct McpErrorData {
    pub message_type: String,
    pub protocol_version: Option<String>,
    pub message_id: Option<String>,
    pub session_info: Option<McpSessionInfo>,
    pub transport_info: Option<McpTransportInfo>,
}

#[derive(Debug, Clone)]
pub struct McpSessionInfo {
    pub session_id: String,
    pub client_info: Option<HashMap<String, String>>,
    pub server_info: Option<HashMap<String, String>>,
    pub established_at: SystemTime,
    pub message_count: u64,
}

#[derive(Debug, Clone)]
pub struct McpTransportInfo {
    pub transport_type: String,
    pub local_endpoint: Option<String>,
    pub remote_endpoint: Option<String>,
    pub connection_id: Option<String>,
    pub established_at: SystemTime,
}
