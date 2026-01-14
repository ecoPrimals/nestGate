//! **ERROR & ACKNOWLEDGMENT TYPES**
//!
//! Error payloads, acknowledgments, and protocol error types.

use nestgate_core::error::NestGateError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Error Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorPayload {
    /// Error Code
    pub error_code: String,
    /// Error Message
    pub error_message: String,
    /// Details
    pub details: HashMap<String, String>,
    /// Timestamp
    pub timestamp: std::time::SystemTime,
}

/// Acknowledgment Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcknowledmentPayload {
    /// Ack Type
    pub ack_type: AcknowledmentType,
    /// Message
    pub message: String,
}

/// Acknowledgment Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AcknowledmentType {
    /// Received
    Received,
    /// Processed
    Processed,
    /// Completed
    Completed,
    /// Failed
    Failed,
}

/// MCP protocol-specific error types with rich context
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum McpProtocolError {
    #[error("Protocol error: {message}")]
    ProtocolError { message: String },

    #[error("Connection error: {message}")]
    ConnectionError { message: String },

    #[error("Message parsing error: {message}")]
    MessageParsingError { message: String },

    #[error("Authentication error: {message}")]
    AuthenticationError { message: String },

    #[error("Session error: {message}")]
    SessionError { message: String },
}

impl From<McpProtocolError> for NestGateError {
    fn from(err: McpProtocolError) -> Self {
        match err {
            McpProtocolError::ProtocolError { message } => {
                NestGateError::simple(format!("Protocol error: {}", message))
            }
            McpProtocolError::ConnectionError { message } => {
                NestGateError::network_error("mcp_connection", message)
            }
            McpProtocolError::MessageParsingError { message } => {
                NestGateError::simple(format!("Parsing error: {}", message))
            }
            McpProtocolError::AuthenticationError { message } => {
                NestGateError::simple(format!("Auth error: {}", message))
            }
            McpProtocolError::SessionError { message } => {
                NestGateError::simple(format!("Session error: {}", message))
            }
        }
    }
}

impl McpProtocolError {
    /// Create protocol error
    #[must_use]
    pub fn protocol_error(message: impl Into<String>) -> Self {
        Self::ProtocolError {
            message: message.into(),
        }
    }

    /// Create connection error
    #[must_use]
    pub fn connection_error(message: impl Into<String>) -> Self {
        Self::ConnectionError {
            message: message.into(),
        }
    }

    /// Create message parsing error
    #[must_use]
    pub fn message_parsing_error(message: impl Into<String>) -> Self {
        Self::MessageParsingError {
            message: message.into(),
        }
    }

    /// Create authentication error
    #[must_use]
    pub fn authentication_error(message: impl Into<String>) -> Self {
        Self::AuthenticationError {
            message: message.into(),
        }
    }

    /// Create session error
    #[must_use]
    pub fn session_error(message: impl Into<String>) -> Self {
        Self::SessionError {
            message: message.into(),
        }
    }
}
