// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Error types and handling for the NestGate Data API.
// Provides clean error responses for management and other consumers.

//! Error module

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

// CANONICAL MODERNIZATION: Use canonical error system directly
// Removed fragmented Result type alias
pub use nestgate_core::error::Result;

/// Main API error type.
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    /// Core `NestGate` errors.
    #[error("Core error: {0}")]
    Core(#[from] nestgate_core::error::NestGateError),
    /// I/O errors.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    /// JSON serialization errors.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    /// Invalid request data.
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    /// Resource not found.
    #[error("Not found: {0}")]
    NotFound(String),
    /// Internal server error.
    #[error("Internal error: {0}")]
    Internal(String),
    /// Service unavailable.
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
}
/// API error response format
#[derive(Debug, Serialize, Deserialize)]
/// Response data for Error operation
pub struct ErrorResponse {
    /// Error message
    pub error: String,
    /// Error code for programmatic handling
    pub code: String,
    /// Additional error details
    pub details: Option<serde_json::Value>,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl IntoResponse for ApiError {
    /// Into Response
    fn into_response(self) -> Response {
        let (status, error_code, message) = match &self {
            Self::Core(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "CORE_ERROR",
                e.to_string(),
            ),
            Self::Io(e) => (StatusCode::INTERNAL_SERVER_ERROR, "IO_ERROR", e.to_string()),
            Self::Json(e) => (
                StatusCode::BAD_REQUEST,
                "JSON_ERROR",
                format!("Invalid JSON: {e}"),
            ),
            Self::InvalidRequest(msg) => (StatusCode::BAD_REQUEST, "INVALID_REQUEST", msg.clone()),
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, "NOT_FOUND", msg.clone()),
            Self::Internal(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                msg.clone(),
            ),
            Self::ServiceUnavailable(msg) => (
                StatusCode::SERVICE_UNAVAILABLE,
                "SERVICE_UNAVAILABLE",
                msg.clone(),
            ),
        };

        let error_response = ErrorResponse {
            error: message,
            code: error_code.to_string(),
            details: None,
            timestamp: chrono::Utc::now(),
        };

        (status, Json(error_response)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as _;

    #[test]
    fn test_error_response_structure() {
        let error = ErrorResponse {
            error: "Test error".into(),
            code: "TEST_ERROR".into(),
            details: None,
            timestamp: chrono::Utc::now(),
        };

        assert_eq!(error.error, "Test error");
        assert_eq!(error.code, "TEST_ERROR");
        assert!(error.details.is_none());
    }

    #[test]
    fn test_error_response_serialization() {
        let error = ErrorResponse {
            error: "Test error".into(),
            code: "TEST_ERROR".into(),
            details: None,
            timestamp: chrono::Utc::now(),
        };

        let serialized = serde_json::to_string(&error);
        assert!(serialized.is_ok(), "ErrorResponse should serialize");

        let json = serialized.expect("Operation failed");
        assert!(json.contains("\"error\":\"Test error\""));
        assert!(json.contains("\"code\":\"TEST_ERROR\""));
    }

    #[test]
    fn test_api_error_display_invalid_request() {
        let error = ApiError::InvalidRequest("Missing field".into());
        let display = format!("{error}");
        assert_eq!(display, "Invalid request: Missing field");
    }

    #[test]
    fn test_api_error_display_not_found() {
        let error = ApiError::NotFound("Resource not found".into());
        let display = format!("{error}");
        assert_eq!(display, "Not found: Resource not found");
    }

    #[test]
    fn test_api_error_display_internal() {
        let error = ApiError::Internal("Internal error occurred".into());
        let display = format!("{error}");
        assert_eq!(display, "Internal error: Internal error occurred");
    }

    #[test]
    fn test_api_error_display_service_unavailable() {
        let error = ApiError::ServiceUnavailable("Service is down".into());
        let display = format!("{error}");
        assert_eq!(display, "Service unavailable: Service is down");
    }

    #[test]
    fn test_api_error_from_io() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let api_error = ApiError::from(io_error);

        match api_error {
            ApiError::Io(_) => {
                // Success
            }
            _ => panic!("Expected ApiError::Io"),
        }
    }

    #[test]
    fn test_api_error_from_json() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let api_error = ApiError::from(json_err);

        match api_error {
            ApiError::Json(_) => {
                // Success
            }
            _ => panic!("Expected ApiError::Json"),
        }
    }

    #[test]
    fn round5_api_error_from_core_display() {
        let core = nestgate_core::NestGateError::internal_error("inner", "test");
        let err = ApiError::from(core);
        let s = err.to_string();
        assert!(s.contains("Core error"));
        assert!(s.contains("inner"));
    }

    #[test]
    fn round5_api_error_json_variant_display() {
        let json_err = serde_json::from_str::<serde_json::Value>("not-json").unwrap_err();
        let err = ApiError::Json(json_err);
        assert!(err.to_string().starts_with("JSON error"));
    }

    #[test]
    fn round5_error_response_serde_roundtrip() {
        let er = ErrorResponse {
            error: "e".into(),
            code: "c".into(),
            details: Some(serde_json::json!({"k": 1})),
            timestamp: chrono::Utc::now(),
        };
        let json = serde_json::to_string(&er).unwrap();
        let back: ErrorResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(back.error, er.error);
        assert_eq!(back.code, er.code);
    }

    #[test]
    fn api_error_into_response_status_core() {
        let core = nestgate_core::NestGateError::internal_error("inner", "test");
        let resp = ApiError::from(core).into_response();
        assert_eq!(resp.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn api_error_into_response_status_json() {
        let json_err = serde_json::from_str::<serde_json::Value>("not-json").unwrap_err();
        let resp = ApiError::from(json_err).into_response();
        assert_eq!(resp.status(), axum::http::StatusCode::BAD_REQUEST);
    }

    #[test]
    fn api_error_source_for_core_io_json() {
        let core = nestgate_core::NestGateError::internal_error("x", "y");
        assert!(ApiError::from(core).source().is_some());
        let io = std::io::Error::other("io");
        assert!(ApiError::from(io).source().is_some());
        let je = serde_json::from_str::<serde_json::Value>("{").unwrap_err();
        assert!(ApiError::from(je).source().is_some());
    }
}
