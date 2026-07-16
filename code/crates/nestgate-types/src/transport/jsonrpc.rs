// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Canonical JSON-RPC 2.0 error types.
//!
//! Single source of truth for JSON-RPC error codes and error construction.
//! Replaces 6+ independent `JsonRpcError` structs scattered across crates.

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt;

/// Standard JSON-RPC 2.0 error codes.
///
/// Codes -32700 through -32600 are reserved by the JSON-RPC specification.
/// Custom codes use -32604 and below.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i32)]
pub enum JsonRpcErrorCode {
    /// Invalid JSON was received (-32700).
    ParseError = -32700,
    /// The JSON sent is not a valid Request object (-32600).
    InvalidRequest = -32600,
    /// The method does not exist or is not available (-32601).
    MethodNotFound = -32601,
    /// Invalid method parameter(s) (-32602).
    InvalidParams = -32602,
    /// Internal JSON-RPC error (-32603).
    InternalError = -32603,
    /// BTSP authentication required (-32604, nestGate extension).
    AuthRequired = -32604,
}

impl JsonRpcErrorCode {
    /// Numeric code value.
    #[must_use]
    pub const fn code(self) -> i32 {
        self as i32
    }

    /// Default human-readable message for this code.
    #[must_use]
    pub const fn default_message(self) -> &'static str {
        match self {
            Self::ParseError => "Parse error",
            Self::InvalidRequest => "Invalid Request",
            Self::MethodNotFound => "Method not found",
            Self::InvalidParams => "Invalid params",
            Self::InternalError => "Internal error",
            Self::AuthRequired => "BTSP authentication required",
        }
    }
}

impl fmt::Display for JsonRpcErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.default_message(), self.code())
    }
}

/// Canonical JSON-RPC 2.0 error object.
///
/// Serializes to the standard `{"code": -326xx, "message": "...", "data": ...}` shape.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    /// Error code.
    pub code: i32,
    /// Human-readable error message.
    pub message: Cow<'static, str>,
    /// Additional error data (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl JsonRpcError {
    /// Create an error from a typed code with default message.
    #[must_use]
    pub const fn from_code(code: JsonRpcErrorCode) -> Self {
        Self {
            code: code.code(),
            message: Cow::Borrowed(code.default_message()),
            data: None,
        }
    }

    /// Create an error from a typed code with a custom message.
    #[must_use]
    pub fn with_message(code: JsonRpcErrorCode, message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            code: code.code(),
            message: message.into(),
            data: None,
        }
    }

    /// Create an error with attached data.
    #[must_use]
    pub fn with_data(
        code: JsonRpcErrorCode,
        message: impl Into<Cow<'static, str>>,
        data: serde_json::Value,
    ) -> Self {
        Self {
            code: code.code(),
            message: message.into(),
            data: Some(data),
        }
    }

    /// Parse error (-32700).
    #[must_use]
    pub const fn parse_error() -> Self {
        Self::from_code(JsonRpcErrorCode::ParseError)
    }

    /// Invalid request (-32600).
    #[must_use]
    pub const fn invalid_request() -> Self {
        Self::from_code(JsonRpcErrorCode::InvalidRequest)
    }

    /// Method not found (-32601).
    #[must_use]
    pub const fn method_not_found() -> Self {
        Self::from_code(JsonRpcErrorCode::MethodNotFound)
    }

    /// Invalid params (-32602).
    #[must_use]
    pub fn invalid_params(detail: impl Into<Cow<'static, str>>) -> Self {
        Self::with_message(JsonRpcErrorCode::InvalidParams, detail)
    }

    /// Internal error (-32603) with a detail message.
    #[must_use]
    pub fn internal(detail: impl Into<Cow<'static, str>>) -> Self {
        Self::with_message(JsonRpcErrorCode::InternalError, detail)
    }

    /// Internal error (-32603) with the default message.
    #[must_use]
    pub const fn internal_error() -> Self {
        Self::from_code(JsonRpcErrorCode::InternalError)
    }

    /// BTSP authentication required (-32604).
    #[must_use]
    pub const fn auth_required() -> Self {
        Self::from_code(JsonRpcErrorCode::AuthRequired)
    }
}

impl fmt::Display for JsonRpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "JSON-RPC error {}: {}", self.code, self.message)
    }
}

impl std::error::Error for JsonRpcError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_code_values() {
        assert_eq!(JsonRpcErrorCode::ParseError.code(), -32700);
        assert_eq!(JsonRpcErrorCode::InvalidRequest.code(), -32600);
        assert_eq!(JsonRpcErrorCode::MethodNotFound.code(), -32601);
        assert_eq!(JsonRpcErrorCode::InvalidParams.code(), -32602);
        assert_eq!(JsonRpcErrorCode::InternalError.code(), -32603);
        assert_eq!(JsonRpcErrorCode::AuthRequired.code(), -32604);
    }

    #[test]
    fn error_code_default_messages() {
        assert_eq!(JsonRpcErrorCode::ParseError.default_message(), "Parse error");
        assert_eq!(JsonRpcErrorCode::MethodNotFound.default_message(), "Method not found");
    }

    #[test]
    fn error_code_display() {
        let display = format!("{}", JsonRpcErrorCode::InternalError);
        assert!(display.contains("-32603"));
        assert!(display.contains("Internal error"));
    }

    #[test]
    fn parse_error_factory() {
        let e = JsonRpcError::parse_error();
        assert_eq!(e.code, -32700);
        assert_eq!(e.message, "Parse error");
        assert!(e.data.is_none());
    }

    #[test]
    fn invalid_params_with_detail() {
        let e = JsonRpcError::invalid_params("Missing 'key' parameter");
        assert_eq!(e.code, -32602);
        assert!(e.message.contains("Missing"));
    }

    #[test]
    fn internal_error_with_detail() {
        let e = JsonRpcError::internal(format!("handler failed: {}", "timeout"));
        assert_eq!(e.code, -32603);
        assert!(e.message.contains("timeout"));
    }

    #[test]
    fn with_data_attaches_json() {
        let data = serde_json::json!({"field": "name"});
        let e = JsonRpcError::with_data(
            JsonRpcErrorCode::InvalidParams,
            "validation failed",
            data.clone(),
        );
        assert_eq!(e.code, -32602);
        assert_eq!(e.data, Some(data));
    }

    #[test]
    fn serde_roundtrip() {
        let e = JsonRpcError::parse_error();
        let json = serde_json::to_string(&e).expect("serialize");
        let back: JsonRpcError = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.code, -32700);
        assert_eq!(back.message, "Parse error");
    }

    #[test]
    fn display_format() {
        let e = JsonRpcError::method_not_found();
        let s = e.to_string();
        assert!(s.contains("-32601"));
        assert!(s.contains("Method not found"));
    }

    #[test]
    fn auth_required_custom_code() {
        let e = JsonRpcError::auth_required();
        assert_eq!(e.code, -32604);
        assert_eq!(e.message, "BTSP authentication required");
    }
}
