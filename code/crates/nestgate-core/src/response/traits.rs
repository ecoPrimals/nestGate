/// Response Traits Module
/// Conversion traits for transforming data into API responses
/// **PROBLEM SOLVED**: Standard conversion patterns for all response types
use std::fmt::Display;

use super::api_response::ApiResponse;
use super::error_response::{ErrorResponse, UnifiedErrorResponse};
use super::success_response::SuccessResponse;

/// Trait for converting errors to API responses
/// This trait provides convenient methods for converting Result types into structured API responses
pub trait IntoApiResponse<T> {
    /// Convert a Result into an ApiResponse
    fn into_api_response(self) -> ApiResponse<T>;

    /// Convert a Result into an ApiResponse with custom error message
    fn into_api_response_with_message(self, error_msg: &str) -> ApiResponse<T>;
}

impl<T, E: Display> IntoApiResponse<T> for std::result::Result<T, E> {
    fn into_api_response(self) -> ApiResponse<T> {
        match self {
            Ok(data) => ApiResponse::success(data),
            Err(error) => ApiResponse::error(error.to_string()),
        }
    }

    fn into_api_response_with_message(self, error_msg: &str) -> ApiResponse<T> {
        match self {
            Ok(data) => ApiResponse::success(data),
            Err(_) => ApiResponse::error(error_msg.to_string()),
        }
    }
}

/// Trait for converting data into success responses
pub trait IntoSuccessResponse {
    /// Convert data into a success response
    fn into_success_response(self, message: &str) -> SuccessResponse;
}

impl<T: serde::Serialize> IntoSuccessResponse for T {
    fn into_success_response(self, message: &str) -> SuccessResponse {
        SuccessResponse::new(message)
            .add_data("payload", serde_json::to_value(self).unwrap_or_default())
    }
}

/// Trait for converting errors into unified error responses
pub trait IntoUnifiedErrorResponse {
    /// Convert error to unified error response
    fn to_unified_error_response(&self, service: &str) -> UnifiedErrorResponse;
}

impl<E: Display> IntoUnifiedErrorResponse for E {
    fn to_unified_error_response(&self, service: &str) -> UnifiedErrorResponse {
        UnifiedErrorResponse::simple(&self.to_string(), "UNKNOWN", service)
    }
}

/// Trait for converting between different response types
pub trait ResponseConversion<T> {
    /// Convert to the target response type
    fn convert(self) -> T;
}

impl ResponseConversion<UnifiedErrorResponse> for ErrorResponse {
    fn convert(self) -> UnifiedErrorResponse {
        self.into()
    }
}

impl<T> ResponseConversion<ApiResponse<T>> for SuccessResponse {
    fn convert(self) -> ApiResponse<T> {
        ApiResponse {
            success: true,
            data: None,
            error: None,
            error_code: None,
            timestamp: self.timestamp,
            metadata: Some({
                let mut metadata = std::collections::HashMap::new();
                metadata.insert("message".to_string(), serde_json::json!(self.message));
                if let Some(data) = self.data {
                    metadata.insert("response_data".to_string(), serde_json::json!(data));
                }
                metadata
            }),
        }
    }
}

/// Trait for extracting response metadata
pub trait ResponseMetadata {
    /// Extract metadata from response
    fn extract_metadata(&self) -> std::collections::HashMap<String, serde_json::Value>;

    /// Check if response is successful
    fn is_successful(&self) -> bool;

    /// Get response timestamp
    fn get_timestamp(&self) -> chrono::DateTime<chrono::Utc>;
}

impl<T> ResponseMetadata for ApiResponse<T> {
    fn extract_metadata(&self) -> std::collections::HashMap<String, serde_json::Value> {
        let mut metadata = std::collections::HashMap::new();
        metadata.insert("success".to_string(), serde_json::json!(self.success));
        metadata.insert("timestamp".to_string(), serde_json::json!(self.timestamp));

        if let Some(error) = &self.error {
            metadata.insert("error".to_string(), serde_json::json!(error));
        }

        if let Some(error_code) = &self.error_code {
            metadata.insert("error_code".to_string(), serde_json::json!(error_code));
        }

        if let Some(response_metadata) = &self.metadata {
            for (key, value) in response_metadata {
                metadata.insert(format!("meta_{key}"), value.clone());
            }
        }

        metadata
    }

    fn is_successful(&self) -> bool {
        self.success
    }

    fn get_timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        self.timestamp
    }
}

impl ResponseMetadata for SuccessResponse {
    fn extract_metadata(&self) -> std::collections::HashMap<String, serde_json::Value> {
        let mut metadata = std::collections::HashMap::new();
        metadata.insert("success".to_string(), serde_json::json!(true));
        metadata.insert("message".to_string(), serde_json::json!(self.message));
        metadata.insert("timestamp".to_string(), serde_json::json!(self.timestamp));

        if let Some(data) = &self.data {
            for (key, value) in data {
                metadata.insert(format!("data_{key}"), value.clone());
            }
        }

        if let Some(response_metadata) = &self.metadata {
            for (key, value) in response_metadata {
                metadata.insert(format!("meta_{key}"), serde_json::json!(value));
            }
        }

        metadata
    }

    fn is_successful(&self) -> bool {
        true
    }

    fn get_timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        self.timestamp
    }
}

impl ResponseMetadata for UnifiedErrorResponse {
    fn extract_metadata(&self) -> std::collections::HashMap<String, serde_json::Value> {
        let mut metadata = std::collections::HashMap::new();
        metadata.insert("success".to_string(), serde_json::json!(false));
        metadata.insert(
            "error_code".to_string(),
            serde_json::json!(self.context.error_code),
        );
        metadata.insert(
            "service_name".to_string(),
            serde_json::json!(self.context.service_name),
        );
        metadata.insert(
            "timestamp".to_string(),
            serde_json::json!(self.context.timestamp),
        );
        metadata.insert(
            "format".to_string(),
            serde_json::json!(format!("{:?}", self.format)),
        );

        // Add context metadata
        for (key, value) in &self.context.context {
            metadata.insert(format!("context_{key}"), value.clone());
        }

        metadata
    }

    fn is_successful(&self) -> bool {
        false
    }

    fn get_timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        use chrono::{DateTime, Utc};
        DateTime::<Utc>::from(self.context.timestamp)
    }
}

/// Helper trait for chain-style response building
pub trait ResponseChaining {
    /// Add context to response (chainable)
    fn with_context_chain(self, key: &str, value: serde_json::Value) -> Self;

    /// Add metadata to response (chainable)
    fn with_metadata_chain(self, key: &str, value: &str) -> Self;

    /// Set timestamp (chainable)
    fn with_timestamp_chain(self, timestamp: chrono::DateTime<chrono::Utc>) -> Self;
}

impl ResponseChaining for UnifiedErrorResponse {
    fn with_context_chain(self, key: &str, value: serde_json::Value) -> Self {
        self.with_context(key, value)
    }

    fn with_metadata_chain(self, key: &str, value: &str) -> Self {
        self.with_context(key, serde_json::json!(value))
    }

    fn with_timestamp_chain(mut self, timestamp: chrono::DateTime<chrono::Utc>) -> Self {
        use std::time::SystemTime;
        self.context.timestamp = SystemTime::from(timestamp);
        self
    }
}

impl<T> ResponseChaining for ApiResponse<T> {
    fn with_context_chain(self, key: &str, value: serde_json::Value) -> Self {
        self.with_meta(key, value)
    }

    fn with_metadata_chain(self, key: &str, value: &str) -> Self {
        self.with_meta(key, serde_json::json!(value))
    }

    fn with_timestamp_chain(mut self, timestamp: chrono::DateTime<chrono::Utc>) -> Self {
        self.timestamp = timestamp;
        self
    }
}

impl ResponseChaining for SuccessResponse {
    fn with_context_chain(self, key: &str, value: serde_json::Value) -> Self {
        self.add_data(key, value)
    }

    fn with_metadata_chain(self, key: &str, value: &str) -> Self {
        self.add_metadata(key, value)
    }

    fn with_timestamp_chain(mut self, timestamp: chrono::DateTime<chrono::Utc>) -> Self {
        self.timestamp = timestamp;
        self
    }
}
