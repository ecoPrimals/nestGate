//
// Error types and handling for the NestGate Data API.
// Provides clean error responses for biomeOS and other consumers.

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
    /// Core NestGate errors
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
            ApiError::Core(e) => write!(f, "Core error: {}", e),
            ApiError::Io(e) => write!(f, "I/O error: {}", e),
            ApiError::Json(e) => write!(f, "JSON error: {}", e),
            ApiError::InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
            ApiError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ApiError::Internal(msg) => write!(f, "Internal error: {}", msg),
            ApiError::ServiceUnavailable(msg) => write!(f, "Service unavailable: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ApiError::Core(e) => Some(e),
            ApiError::Io(e) => Some(e),
            ApiError::Json(e) => Some(e),
            _ => None,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_code, message) = match &self {
            ApiError::Core(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "CORE_ERROR",
                e.to_string(),
            ),
            ApiError::Io(e) => (StatusCode::INTERNAL_SERVER_ERROR, "IO_ERROR", e.to_string()),
            ApiError::Json(e) => (
                StatusCode::BAD_REQUEST,
                "JSON_ERROR",
                format!("Invalid JSON: {}", e),
            ),
            ApiError::InvalidRequest(msg) => {
                (StatusCode::BAD_REQUEST, "INVALID_REQUEST", msg.clone())
            }
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, "NOT_FOUND", msg.clone()),
            ApiError::Internal(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                msg.clone(),
            ),
            ApiError::ServiceUnavailable(msg) => (
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
        ApiError::Core(err)
    }
}

impl From<std::io::Error> for ApiError {
    fn from(err: std::io::Error) -> Self {
        ApiError::Io(err)
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        ApiError::Json(err)
    }
}
