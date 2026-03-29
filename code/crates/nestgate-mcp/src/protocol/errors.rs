// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

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
                Self::network_error(format!("MCP protocol error: {message}"))
            }
            McpProtocolError::ConnectionError { message } => {
                Self::network_error(format!("MCP connection error: {message}"))
            }
            McpProtocolError::MessageParsingError { message } => {
                Self::network_error(format!("MCP parsing error: {message}"))
            }
            McpProtocolError::AuthenticationError { message } => {
                Self::security_authentication_failed("mcp", format!("MCP auth error: {message}"))
            }
            McpProtocolError::SessionError { message } => {
                Self::network_error(format!("MCP session error: {message}"))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_payload_round_trips_json() {
        let mut details = HashMap::new();
        details.insert("k".to_string(), "v".to_string());
        let p = ErrorPayload {
            error_code: "E".to_string(),
            error_message: "msg".to_string(),
            details,
            timestamp: std::time::SystemTime::UNIX_EPOCH,
        };
        let json = serde_json::to_string(&p).unwrap();
        let back: ErrorPayload = serde_json::from_str(&json).unwrap();
        assert_eq!(back.error_code, "E");
        assert_eq!(back.error_message, "msg");
        assert_eq!(back.details.get("k").map(String::as_str), Some("v"));
    }

    #[test]
    fn acknowledgment_payload_round_trips() {
        let a = AcknowledmentPayload {
            ack_type: AcknowledmentType::Processed,
            message: "done".to_string(),
        };
        let json = serde_json::to_string(&a).unwrap();
        let back: AcknowledmentPayload = serde_json::from_str(&json).unwrap();
        assert!(matches!(back.ack_type, AcknowledmentType::Processed));
        assert_eq!(back.message, "done");
    }

    #[test]
    fn mcp_protocol_error_display_and_constructors() {
        let e = McpProtocolError::protocol_error("p");
        assert!(e.to_string().contains("Protocol error"));
        let e = McpProtocolError::connection_error("c");
        assert!(e.to_string().contains("Connection error"));
        let e = McpProtocolError::message_parsing_error("m");
        assert!(e.to_string().contains("Message parsing"));
        let e = McpProtocolError::authentication_error("a");
        assert!(e.to_string().contains("Authentication error"));
        let e = McpProtocolError::session_error("s");
        assert!(e.to_string().contains("Session error"));
    }

    #[test]
    fn mcp_protocol_error_maps_to_nestgate_error_by_variant() {
        let cases: Vec<(McpProtocolError, fn(&str) -> bool)> = vec![
            (McpProtocolError::protocol_error("x"), |s| {
                s.contains("MCP protocol error")
            }),
            (McpProtocolError::connection_error("x"), |s| {
                s.contains("MCP connection error")
            }),
            (McpProtocolError::message_parsing_error("x"), |s| {
                s.contains("MCP parsing error")
            }),
            (McpProtocolError::authentication_error("x"), |s| {
                s.contains("MCP auth error")
            }),
            (McpProtocolError::session_error("x"), |s| {
                s.contains("MCP session error")
            }),
        ];
        for (err, pred) in cases {
            let ng: NestGateError = err.into();
            let msg = ng.to_string();
            assert!(pred(&msg), "unexpected message: {msg}");
        }
    }
}
