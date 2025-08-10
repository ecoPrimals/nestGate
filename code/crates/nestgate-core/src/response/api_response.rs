/// API Response Module
/// Core ApiResponse type for standardized API communication
/// **PROBLEM SOLVED**: Centralized API response format across all NestGate services
use axum::response::{IntoResponse, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Universal API response wrapper
/// This type provides consistent response formatting across all NestGate APIs
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
    /// Create a successful response with data using standardized builder
    pub fn success(data: T) -> Self {
        crate::return_builders::build_api_success(data)
    }

    /// Create a successful response with data and metadata using standardized builder
    pub fn success_with_metadata(data: T, metadata: HashMap<String, serde_json::Value>) -> Self {
        crate::return_builders::build_api_success_with_metadata(data, metadata)
    }

    /// Create an error response using standardized builder
    pub fn error(message: String) -> Self {
        crate::return_builders::build_api_error(message, None)
    }

    /// Create an error response with error code using standardized builder
    pub fn error_with_code(message: String, code: String) -> Self {
        crate::return_builders::build_api_error(message, Some(code))
    }

    /// Create a new successful response directly
    pub fn new_success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            error_code: None,
            timestamp: chrono::Utc::now(),
            metadata: None,
        }
    }

    /// Create a new error response directly
    pub fn new_error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
            error_code: None,
            timestamp: chrono::Utc::now(),
            metadata: None,
        }
    }

    /// Create a new error response with code directly
    pub fn new_error_with_code(message: String, code: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
            error_code: Some(code),
            timestamp: chrono::Utc::now(),
            metadata: None,
        }
    }

    /// Add metadata to the response
    pub fn with_metadata(mut self, metadata: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Add a single metadata entry
    pub fn with_meta(mut self, key: &str, value: serde_json::Value) -> Self {
        let mut metadata = self.metadata.unwrap_or_default();
        metadata.insert(key.to_string(), value);
        self.metadata = Some(metadata);
        self
    }

    /// Check if the response is successful
    pub fn is_success(&self) -> bool {
        self.success
    }

    /// Check if the response is an error
    pub fn is_error(&self) -> bool {
        !self.success
    }

    /// Get the error message if present
    pub fn error_message(&self) -> Option<&str> {
        self.error.as_deref()
    }

    /// Get the error code if present
    pub fn error_code(&self) -> Option<&str> {
        self.error_code.as_deref()
    }

    /// Convert to JSON for HTTP responses
    pub fn to_json(self) -> Json<Self> {
        Json(self)
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        let status = if self.success {
            axum::http::StatusCode::OK
        } else {
            axum::http::StatusCode::BAD_REQUEST
        };

        (status, Json(self)).into_response()
    }
}

impl<T> Default for ApiResponse<T> {
    fn default() -> Self {
        Self {
            success: false,
            data: None,
            error: Some("Unknown error".to_string()),
            error_code: None,
            timestamp: chrono::Utc::now(),
            metadata: None,
        }
    }
}

/// Empty response for operations that don't return data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyResponse {
    /// Success status
    pub success: bool,
    /// Optional message
    pub message: Option<String>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

impl EmptyResponse {
    /// Create a successful empty response
    pub fn success_empty() -> Self {
        Self {
            success: true,
            message: None,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Create a successful empty response with message
    pub fn success_message(message: &str) -> Self {
        Self {
            success: true,
            message: Some(message.to_string()),
            timestamp: chrono::Utc::now(),
        }
    }

    /// Create an error empty response
    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            message: Some(message.to_string()),
            timestamp: chrono::Utc::now(),
        }
    }
}

impl IntoResponse for EmptyResponse {
    fn into_response(self) -> axum::response::Response {
        let status = if self.success {
            axum::http::StatusCode::OK
        } else {
            axum::http::StatusCode::BAD_REQUEST
        };

        (status, Json(self)).into_response()
    }
}

impl Default for EmptyResponse {
    fn default() -> Self {
        Self::success_empty()
    }
}
