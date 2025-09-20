use super::api_response::ApiResponse;
use super::error_response::{LegacyErrorResponse, UnifiedErrorResponse};
use super::success_response::SuccessResponse;
/// Response Traits Module
/// Conversion traits for transforming data into API responses
/// **PROBLEM SOLVED**: Standard conversion patterns for all response types
use std::fmt::Display;

/// Trait for converting errors to API responses
/// This trait provides convenient methods for converting Result types into structured API responses
pub trait IntoApiResponse<T> {
    /// Convert a Result into an `ApiResponse`
    fn into_api_response(self) -> ApiResponse<T>;
    /// Convert a Result into an `ApiResponse` with custom error message
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
impl ResponseConversion<UnifiedErrorResponse> for LegacyErrorResponse {
    fn convert(self) -> UnifiedErrorResponse {
        UnifiedErrorResponse {
            message: self.error,
            code: self.code.unwrap_or_else(|| "LEGACY_ERROR".to_string()),
            component: "legacy".to_string(),
            status: 500,
            details: None,
            timestamp: self.timestamp,
            correlation_id: None,
        }
    }
}

impl<T> ResponseConversion<ApiResponse<T>> for SuccessResponse {
    fn convert(self) -> ApiResponse<T> {
        ApiResponse {
            request_id: uuid::Uuid::new_v4().to_string(),
            status: crate::canonical_types::ResponseStatus::Success,
            success: true,
            data: None,
            error: None,
            error_code: None,
            timestamp: chrono::DateTime::parse_from_rfc3339(&self.timestamp)
                .unwrap_or_else(|_| chrono::Utc::now().into())
                .with_timezone(&chrono::Utc),
            metadata: Some({
                let mut metadata = std::collections::HashMap::new();
                metadata.insert("message".to_string(), serde_json::json!(self.message));
                if let Some(data) = self.data {
                    metadata.insert("response_data".to_string(), serde_json::json!(data));
                }
                metadata
            }),
            processing_time_ms: 0,
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

        for (key, value) in &self.metadata {
            metadata.insert(format!("meta_{key}"), value.clone());
        }

        metadata
    }

    fn is_successful(&self) -> bool {
        true
    }

    fn get_timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::parse_from_rfc3339(&self.timestamp)
            .unwrap_or_else(|_| chrono::Utc::now().into())
            .with_timezone(&chrono::Utc)
    }
}

impl ResponseMetadata for UnifiedErrorResponse {
    fn extract_metadata(&self) -> std::collections::HashMap<String, serde_json::Value> {
        let mut metadata = std::collections::HashMap::new();
        metadata.insert("success".to_string(), serde_json::json!(false));
        metadata.insert(
            "error_code".to_string(),
            serde_json::json!(self.code), // Use direct field access
        );
        metadata.insert(
            "service_name".to_string(),
            serde_json::json!(self.component), // Use direct field access
        );
        metadata.insert(
            "timestamp".to_string(),
            serde_json::json!(self.timestamp), // Use direct field access
        );
        metadata.insert(
            "status".to_string(),
            serde_json::json!(self.status), // Use available field
        );

        // Add details metadata if available
        if let Some(details) = &self.details {
            for (key, value) in details {
                metadata.insert(format!("detail_{key}"), value.clone());
            }
        }

        metadata
    }

    fn is_successful(&self) -> bool {
        false
    }

    fn get_timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        use chrono::{DateTime, Utc};
        // Parse timestamp string to DateTime
        self.timestamp
            .parse::<DateTime<Utc>>()
            .unwrap_or_else(|_| Utc::now())
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
    fn with_context_chain(self, _key: &str, _value: serde_json::Value) -> Self {
        // UnifiedErrorResponse doesn't have a with_context method, so we'll just return self
        self
    }

    fn with_metadata_chain(self, _key: &str, _value: &str) -> Self {
        // UnifiedErrorResponse doesn't have a context field, so we'll just return self
        self
    }

    fn with_timestamp_chain(mut self, timestamp: chrono::DateTime<chrono::Utc>) -> Self {
        self.timestamp = timestamp.to_rfc3339();
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
        self.add_metadata(key, value.into())
    }

    fn with_timestamp_chain(mut self, timestamp: chrono::DateTime<chrono::Utc>) -> Self {
        self.timestamp = timestamp.to_rfc3339();
        self
    }
}
