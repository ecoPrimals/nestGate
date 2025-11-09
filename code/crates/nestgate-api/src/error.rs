//
// Error types and handling for the NestGate Data API.
// Provides clean error responses for management and other consumers.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::fmt;

// CANONICAL MODERNIZATION: Use canonical error system directly
// Removed fragmented Result type alias
pub use nestgate_core::error::Result;

/// Main API error type
#[derive(Debug)]
pub enum ApiError {
    /// Core `NestGate` errors
    Core(nestgate_core::error::NestGateError),
    /// I/O errors
    Io(std::io::Error),
    /// JSON serialization errors
    Json(serde_json::Error),
    /// Invalid request data
    InvalidRequest(String),
    /// Resource not found
    NotFound(String),
    /// Internal server error
    Internal(String),
    /// Service unavailable
    ServiceUnavailable(String),
}
/// API error response format
#[derive(Debug, Serialize, Deserialize)]
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
impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Core(e) => write!(f, "Core error: {e}"),
            Self::Io(e) => write!(f, "I/O error: {e}"),
            Self::Json(e) => write!(f, "JSON error: {e}"),
            Self::InvalidRequest(msg) => write!(f, "Invalid request: {msg}"),
            Self::NotFound(msg) => write!(f, "Not found: {msg}"),
            Self::Internal(msg) => write!(f, "Internal error: {msg}"),
            Self::ServiceUnavailable(msg) => write!(f, "Service unavailable: {msg}"),
        }
    }
}

impl std::error::Error for ApiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Core(e) => Some(e),
            Self::Io(e) => Some(e),
            Self::Json(e) => Some(e),
            _ => None,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_code, message) = match &self {
            Self::Core(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "CORE_ERROR",
                e.to_string(),
            ),
            Self::Io(e) => (StatusCode::INTERNAL_SERVER_ERROR, "IO_ERROR", e.to_string()),
            Self::Json(_e) => (
                StatusCode::BAD_REQUEST,
                "JSON_ERROR",
                "Invalid JSON: self.base_url".to_string(),
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

// Conversion implementations
impl From<nestgate_core::error::NestGateError> for ApiError {
    fn from(err: nestgate_core::error::NestGateError) -> Self {
        Self::Core(err)
    }
}

impl From<std::io::Error> for ApiError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_response_structure() {
        let error = ErrorResponse {
            error: "Test error".to_string(),
            code: "TEST_ERROR".to_string(),
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
            error: "Test error".to_string(),
            code: "TEST_ERROR".to_string(),
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
        let error = ApiError::InvalidRequest("Missing field".to_string());
        let display = format!("{}", error);
        assert_eq!(display, "Invalid request: Missing field");
    }

    #[test]
    fn test_api_error_display_not_found() {
        let error = ApiError::NotFound("Resource not found".to_string());
        let display = format!("{}", error);
        assert_eq!(display, "Not found: Resource not found");
    }

    #[test]
    fn test_api_error_display_internal() {
        let error = ApiError::Internal("Internal error occurred".to_string());
        let display = format!("{}", error);
        assert_eq!(display, "Internal error: Internal error occurred");
    }

    #[test]
    fn test_api_error_display_service_unavailable() {
        let error = ApiError::ServiceUnavailable("Service is down".to_string());
        let display = format!("{}", error);
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
}
