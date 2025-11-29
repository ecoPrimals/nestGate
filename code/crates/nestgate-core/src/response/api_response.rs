/// API Response Module
/// Core `ApiResponse` type for standardized API communication
/// **PROBLEM SOLVED**: Centralized API response format across all `NestGate` services
use axum::response::{IntoResponse, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
/// Universal API response wrapper
/// This type provides consistent response formatting across all `NestGate` APIs
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for Api operation
pub struct ApiResponse<T> {
    /// Request ID for tracing
    pub request_id: String,
    /// Response status
    pub status: crate::canonical_types::ResponseStatus,
    /// Success status (for backward compatibility)
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
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
}
impl<T> ApiResponse<T> {
    /// Create a successful response with data using standardized builder
    #[must_use]
    pub fn success(data: T) -> Self {
        Self {
            request_id: Uuid::new_v4().to_string(),
            status: crate::canonical_types::ResponseStatus::Success,
            success: true,
            data: Some(data),
            error: None,
            error_code: None,
            timestamp: chrono::Utc::now(),
            metadata: Some(HashMap::new()),
            processing_time_ms: 0,
        }
    }

    /// Create a successful response with data and metadata using standardized builder
    pub fn success_with_metadata(data: T, metadata: HashMap<String, serde_json::Value>) -> Self {
        Self {
            request_id: Uuid::new_v4().to_string(),
            status: crate::canonical_types::ResponseStatus::Success,
            success: true,
            data: Some(data),
            error: None,
            error_code: None,
            timestamp: chrono::Utc::now(),
            metadata: Some(metadata),
            processing_time_ms: 0,
        }
    }

    /// Create an error response using standardized builder
    #[must_use]
    pub fn error(message: String) -> Self {
        Self {
            request_id: Uuid::new_v4().to_string(),
            status: crate::canonical_types::ResponseStatus::Error,
            success: false,
            data: None,
            error: Some(message),
            error_code: None,
            timestamp: chrono::Utc::now(),
            metadata: Some(HashMap::new()),
            processing_time_ms: 0,
        }
    }

    /// Create an error response with error code using standardized builder
    #[must_use]
    pub fn error_with_code(message: String, code: String) -> Self {
        Self {
            request_id: Uuid::new_v4().to_string(),
            status: crate::canonical_types::ResponseStatus::Error,
            success: false,
            data: None,
            error: Some(message),
            error_code: Some(code.clone()),
            timestamp: chrono::Utc::now(),
            metadata: Some({
                let mut meta = HashMap::new();
                meta.insert("error_code".to_string(), serde_json::Value::String(code));
                meta
            }),
            processing_time_ms: 0,
        }
    }

    /// Create a new successful response directly
    pub fn new_success(data: T) -> Self {
        Self {
            request_id: Uuid::new_v4().to_string(),
            status: crate::canonical_types::ResponseStatus::Success,
            success: true,
            data: Some(data),
            error: None,
            error_code: None,
            timestamp: chrono::Utc::now(),
            metadata: None,
            processing_time_ms: 0,
        }
    }

    /// Create a new error response directly
    #[must_use]
    pub fn new_error(message: String) -> Self {
        Self {
            request_id: Uuid::new_v4().to_string(),
            status: crate::canonical_types::ResponseStatus::Error,
            success: false,
            data: None,
            error: Some(message),
            error_code: None,
            timestamp: chrono::Utc::now(),
            metadata: None,
            processing_time_ms: 0,
        }
    }

    /// Create a new error response with code directly
    #[must_use]
    pub fn new_error_with_code(message: String, code: String) -> Self {
        Self {
            request_id: Uuid::new_v4().to_string(),
            status: crate::canonical_types::ResponseStatus::Error,
            success: false,
            data: None,
            error: Some(message),
            error_code: Some(code),
            timestamp: chrono::Utc::now(),
            metadata: None,
            processing_time_ms: 0,
        }
    }

    /// Add metadata to the response
    #[must_use]
    pub fn with_metadata(mut self, metadata: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Add a single metadata entry
    #[must_use]
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
    /// Into Response
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
    /// Returns the default instance
    fn default() -> Self {
        Self {
            request_id: Uuid::new_v4().to_string(),
            status: crate::canonical_types::ResponseStatus::Error,
            success: false,
            data: None,
            error: Some("Unknown error".to_string()),
            error_code: None,
            timestamp: chrono::Utc::now(),
            metadata: None,
            processing_time_ms: 0,
        }
    }
}

/// Empty response for operations that don't return data
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for Empty operation
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
    #[must_use]
    pub fn success_empty() -> Self {
        Self {
            success: true,
            message: None,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Create a successful empty response with message
    #[must_use]
    pub fn success_message(message: &str) -> Self {
        Self {
            success: true,
            message: Some(message.to_string()),
            timestamp: chrono::Utc::now(),
        }
    }

    /// Create an error empty response
    #[must_use]
    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            message: Some(message.to_string()),
            timestamp: chrono::Utc::now(),
        }
    }
}

impl IntoResponse for EmptyResponse {
    /// Into Response
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
    /// Returns the default instance
    fn default() -> Self {
        Self::success_empty()
    }
}
