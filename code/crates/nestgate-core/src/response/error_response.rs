// Error response types and builders
// Provides unified error response structures for API endpoints

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unified error response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedErrorResponse {
    /// Error message for display
    pub message: String,

    /// Machine-readable error code
    pub code: String,

    /// Component that generated the error
    pub component: String,

    /// HTTP status code
    pub status: u16,

    /// Additional error details
    pub details: Option<HashMap<String, serde_json::Value>>,

    /// Timestamp of error occurrence
    pub timestamp: String,

    /// Request correlation ID
    pub correlation_id: Option<String>,
}
impl UnifiedErrorResponse {
    /// Create a simple error response
    #[must_use]
    pub const fn simple(message: &str, code: &str, component: &str) -> Self {
        Self {
            message: message.to_string(),
            code: code.to_string(),
            component: component.to_string(),
            status: 500,
            details: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
            correlation_id: None,
        }
    }

    /// Create an error response with status code
    #[must_use]
    pub const fn with_status(message: &str, code: &str, component: &str, status: u16) -> Self {
        Self {
            message: message.to_string(),
            code: code.to_string(),
            component: component.to_string(),
            status,
            details: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
            correlation_id: None,
        }
    }

    /// Add details to the error response
    #[must_use]
    pub fn with_details(mut self, details: HashMap<String, serde_json::Value>) -> Self {
        self.details = Some(details);
        self
    }

    /// Add correlation ID to the error response
    #[must_use]
    pub fn with_correlation_id(mut self, correlation_id: String) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }

    /// Add context information to the error response
    #[must_use]
    pub fn with_context(mut self, key: &str, value: serde_json::Value) -> Self {
        if self.details.is_none() {
            self.details = Some(HashMap::new());
        }
        if let Some(ref mut details) = self.details {
            details.insert(key.to_string(), value);
        }
        self
    }

    /// Add context chain information to the error response
    #[must_use]
    pub fn with_context_chain(self, key: &str, value: serde_json::Value) -> Self {
        // Removed mut - delegating to with_context
        self.with_context(key, value)
    }

    /// Convert to HTTP status code
    #[must_use]
    pub const fn to_status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl IntoResponse for UnifiedErrorResponse {
    fn into_response(self) -> Response {
        let status_code = self.to_status_code();
        (status_code, Json(self)).into_response()
    }
}

/// Error response factory for common error types
pub struct ErrorResponseFactory;
impl ErrorResponseFactory {
    /// Create a bad request error
    #[must_use]
    pub const fn bad_request(message: &str) -> UnifiedErrorResponse {
        UnifiedErrorResponse::with_status(message, "BAD_REQUEST", "nestgate-core", 400)
    }

    /// Create an internal server error
    #[must_use]
    pub const fn internal(message: &str) -> UnifiedErrorResponse {
        UnifiedErrorResponse::with_status(message, "INTERNAL_ERROR", "nestgate-core", 500)
    }

    /// Create a not found error
    #[must_use]
    pub const fn not_found(path: &str) -> UnifiedErrorResponse {
        UnifiedErrorResponse::with_status(
            &format!("{path} not found"),
            "NOT_FOUND",
            "nestgate-core",
            404,
        )
    }

    /// Create an unauthorized error
    #[must_use]
    pub const fn unauthorized(operation: &str) -> UnifiedErrorResponse {
        UnifiedErrorResponse::with_status(
            &format!("Unauthorized to perform {operation}"),
            "UNAUTHORIZED",
            "nestgate-core",
            401,
        )
    }

    /// Create a forbidden error
    #[must_use]
    pub const fn forbidden(resource: &str) -> UnifiedErrorResponse {
        UnifiedErrorResponse::with_status(
            &format!("Access forbidden to {resource}"),
            "FORBIDDEN",
            "nestgate-core",
            403,
        )
    }

    /// Create a validation error
    #[must_use]
    pub fn validation_error(field: &str, message: &str) -> UnifiedErrorResponse {
        let mut details = HashMap::new();
        details.insert("field".to_string(), serde_json::json!(field));

        UnifiedErrorResponse::with_status(
            &format!("Validation failed for {field}: {message}"),
            "VALIDATION_ERROR",
            "nestgate-core",
            400,
        )
        .with_details(details)
    }

    /// Create a conflict error
    #[must_use]
    pub const fn conflict(resource: &str) -> UnifiedErrorResponse {
        UnifiedErrorResponse::with_status(
            &format!("Conflict with existing {resource}"),
            "CONFLICT",
            "nestgate-core",
            409,
        )
    }

    /// Create a rate limit error
    #[must_use]
    pub fn rate_limited(retry_after: Option<u64>) -> UnifiedErrorResponse {
        let mut details = HashMap::new();
        if let Some(retry_after) = retry_after {
            details.insert(
                "retry_after_seconds".to_string(),
                serde_json::json!(retry_after),
            );
        }

        UnifiedErrorResponse::with_status(
            "Rate limit exceeded",
            "RATE_LIMITED",
            "nestgate-core",
            429,
        )
        .with_details(details)
    }

    /// Create a service unavailable error
    #[must_use]
    pub const fn service_unavailable(service: &str) -> UnifiedErrorResponse {
        UnifiedErrorResponse::with_status(
            &format!("Service {service} is currently unavailable"),
            "SERVICE_UNAVAILABLE",
            "nestgate-core",
            503,
        )
    }

    /// Create a timeout error
    #[must_use]
    pub const fn timeout(operation: &str) -> UnifiedErrorResponse {
        UnifiedErrorResponse::with_status(
            &format!("Operation {operation} timed out"),
            "TIMEOUT",
            "nestgate-core",
            408,
        )
    }
}

/// Legacy error response for backward compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyErrorResponse {
    pub error: String,
    pub code: Option<String>,
    pub timestamp: String,
}
impl From<UnifiedErrorResponse> for LegacyErrorResponse {
    fn from(unified: UnifiedErrorResponse) -> Self {
        Self {
            error: unified.message,
            code: Some(unified.code),
            timestamp: unified.timestamp,
        }
    }
}

impl IntoResponse for LegacyErrorResponse {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_error_response() {
        let error = UnifiedErrorResponse::simple("Test error", "TEST_ERROR", "test-component");
        assert_eq!(error.message, "Test error");
        assert_eq!(error.code, "TEST_ERROR");
        assert_eq!(error.component, "test-component");
        assert_eq!(error.status, 500);
    }

    #[test]
    fn test_error_response_with_status() {
        let error = UnifiedErrorResponse::with_status("Bad request", "BAD_REQUEST", "test", 400);
        assert_eq!(error.status, 400);
        assert_eq!(error.to_status_code(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_error_factory_methods() {
        let bad_request = ErrorResponseFactory::bad_request("Invalid input");
        assert_eq!(bad_request.status, 400);
        assert_eq!(bad_request.code, "BAD_REQUEST");

        let not_found = ErrorResponseFactory::not_found("/api/test");
        assert_eq!(not_found.status, 404);
        assert_eq!(not_found.code, "NOT_FOUND");
    }

    #[test]
    fn test_validation_error_with_details() {
        let error =
            ErrorResponseFactory::validation_error("username", "must be at least 3 characters");
        assert_eq!(error.status, 400);
        assert!(error.details.is_some());

        let details = error.details.unwrap_or_default();
        assert_eq!(details["field"], serde_json::json!("username"));
    }

    #[test]
    fn test_legacy_error_conversion() {
        let unified = UnifiedErrorResponse::simple("Test", "TEST", "component");
        let legacy: LegacyErrorResponse = unified.into();
        assert_eq!(legacy.error, "Test");
        assert_eq!(legacy.code, Some("TEST".to_string()));
    }
}
