//! Universal Response Utilities
//!
//! Centralized response creation and handling utilities to eliminate code duplication
//! across all NestGate crates. Provides consistent response formats for APIs.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Universal API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// Success status
    pub success: bool,
    /// Response data
    pub data: Option<T>,
    /// Error message
    pub error: Option<String>,
    /// Error code
    pub error_code: Option<String>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Additional metadata
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl<T> ApiResponse<T> {
    /// Create a successful response with data
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            error_code: None,
            timestamp: Utc::now(),
            metadata: None,
        }
    }

    /// Create a successful response with data and metadata
    pub fn success_with_metadata(data: T, metadata: HashMap<String, serde_json::Value>) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            error_code: None,
            timestamp: Utc::now(),
            metadata: Some(metadata),
        }
    }

    /// Create an error response
    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
            error_code: None,
            timestamp: Utc::now(),
            metadata: None,
        }
    }

    /// Create an error response with error code
    pub fn error_with_code(message: String, code: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
            error_code: Some(code),
            timestamp: Utc::now(),
            metadata: None,
        }
    }
}

/// Empty response for operations that don't return data
pub type EmptyResponse = ApiResponse<()>;

impl EmptyResponse {
    /// Create an empty success response
    pub fn success_empty() -> Self {
        Self::success(())
    }

    /// Create an empty success response with message
    pub fn success_message(message: &str) -> Self {
        let mut metadata = HashMap::new();
        metadata.insert("message".to_string(), serde_json::json!(message));
        Self::success_with_metadata((), metadata)
    }
}

/// Simplified error response structure for specific use cases
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// Error message
    pub message: String,
    /// Error code
    pub error_code: Option<String>,
    /// Service name that generated the error
    pub service_name: Option<String>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

impl ErrorResponse {
    /// Create a new error response
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            error_code: None,
            service_name: None,
            timestamp: Utc::now(),
        }
    }

    /// Create an error response with code
    pub fn with_code(message: &str, code: &str) -> Self {
        Self {
            message: message.to_string(),
            error_code: Some(code.to_string()),
            service_name: None,
            timestamp: Utc::now(),
        }
    }

    /// Create an error response with service name
    pub fn with_service(message: &str, service: &str) -> Self {
        Self {
            message: message.to_string(),
            error_code: None,
            service_name: Some(service.to_string()),
            timestamp: Utc::now(),
        }
    }

    /// Create an error response with both code and service
    pub fn with_code_and_service(message: &str, code: &str, service: &str) -> Self {
        Self {
            message: message.to_string(),
            error_code: Some(code.to_string()),
            service_name: Some(service.to_string()),
            timestamp: Utc::now(),
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, Json(self)).into_response()
    }
}

/// Success response structure for specific use cases
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessResponse {
    /// Success message
    pub message: String,
    /// Status indicator
    pub status: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

impl SuccessResponse {
    /// Create a new success response
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            status: "success".to_string(),
            timestamp: Utc::now(),
        }
    }
}

impl IntoResponse for SuccessResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

/// Universal response creation utilities
pub struct ResponseBuilder;

impl ResponseBuilder {
    /// Create a JSON error response for Axum handlers
    pub fn error_json(message: String) -> Json<serde_json::Value> {
        Json(serde_json::json!({
            "success": false,
            "error": message,
            "timestamp": Utc::now()
        }))
    }

    /// Create a JSON success response for Axum handlers
    pub fn success_json(message: String) -> Json<serde_json::Value> {
        Json(serde_json::json!({
            "success": true,
            "message": message,
            "timestamp": Utc::now()
        }))
    }

    /// Create a JSON success response with data
    pub fn success_json_with_data<T: Serialize>(
        message: String,
        data: T,
    ) -> Json<serde_json::Value> {
        Json(serde_json::json!({
            "success": true,
            "message": message,
            "data": data,
            "timestamp": Utc::now()
        }))
    }

    /// Create an error response with status code
    pub fn error_with_status(status: StatusCode, message: String) -> impl IntoResponse {
        (status, Self::error_json(message))
    }

    /// Create a success response with status code
    pub fn success_with_status(status: StatusCode, message: String) -> impl IntoResponse {
        (status, Self::success_json(message))
    }

    /// Create a service unavailable error response
    pub fn service_unavailable(service: &str) -> impl IntoResponse {
        Self::error_with_status(
            StatusCode::SERVICE_UNAVAILABLE,
            format!("{service} service is currently unavailable"),
        )
    }

    /// Create an internal server error response
    pub fn internal_error(error: String) -> impl IntoResponse {
        Self::error_with_status(StatusCode::INTERNAL_SERVER_ERROR, error)
    }

    /// Create a not found error response
    pub fn not_found(resource: &str) -> impl IntoResponse {
        Self::error_with_status(StatusCode::NOT_FOUND, format!("{resource} not found"))
    }

    /// Create a bad request error response
    pub fn bad_request(message: String) -> impl IntoResponse {
        Self::error_with_status(StatusCode::BAD_REQUEST, message)
    }
}

/// Trait for converting errors to API responses
pub trait IntoApiResponse<T> {
    /// Convert a Result into an ApiResponse
    fn into_api_response(self) -> ApiResponse<T>;

    /// Convert a Result into an ApiResponse with custom error message
    fn into_api_response_with_message(self, error_msg: &str) -> ApiResponse<T>;
}

impl<T, E: std::fmt::Display> IntoApiResponse<T> for Result<T, E> {
    fn into_api_response(self) -> ApiResponse<T> {
        match self {
            Ok(data) => ApiResponse::success(data),
            Err(error) => ApiResponse::error(error.to_string()),
        }
    }

    fn into_api_response_with_message(self, error_msg: &str) -> ApiResponse<T> {
        match self {
            Ok(data) => ApiResponse::success(data),
            Err(error) => ApiResponse::error(format!("{error_msg}: {error}")),
        }
    }
}
