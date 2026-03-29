// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// Error response types and builders
// Provides unified error response structures for API endpoints

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unified error response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for UnifiedError operation
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
    pub fn simple(message: &str, code: &str, component: &str) -> Self {
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
    pub fn with_status(message: &str, code: &str, component: &str, status: u16) -> Self {
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
    pub fn to_status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl IntoResponse for UnifiedErrorResponse {
    /// Into Response
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
    pub fn bad_request(message: &str) -> UnifiedErrorResponse {
        UnifiedErrorResponse::with_status(message, "BAD_REQUEST", "nestgate-core", 400)
    }

    /// Create an internal server error
    #[must_use]
    pub fn internal(message: &str) -> UnifiedErrorResponse {
        UnifiedErrorResponse::with_status(message, "INTERNAL_ERROR", "nestgate-core", 500)
    }

    /// Create a not found error
    #[must_use]
    pub fn not_found(path: &str) -> UnifiedErrorResponse {
        UnifiedErrorResponse::with_status(
            &format!("{path} not found"),
            "NOT_FOUND",
            "nestgate-core",
            404,
        )
    }

    /// Create an unauthorized error
    #[must_use]
    pub fn unauthorized(operation: &str) -> UnifiedErrorResponse {
        UnifiedErrorResponse::with_status(
            &format!("Unauthorized to perform {operation}"),
            "UNAUTHORIZED",
            "nestgate-core",
            401,
        )
    }

    /// Create a forbidden error
    #[must_use]
    pub fn forbidden(resource: &str) -> UnifiedErrorResponse {
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
    pub fn conflict(resource: &str) -> UnifiedErrorResponse {
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
    pub fn service_unavailable(service: &str) -> UnifiedErrorResponse {
        UnifiedErrorResponse::with_status(
            &format!("Service {service} is currently unavailable"),
            "SERVICE_UNAVAILABLE",
            "nestgate-core",
            503,
        )
    }

    /// Create a timeout error
    #[must_use]
    pub fn timeout(operation: &str) -> UnifiedErrorResponse {
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
/// Response data for LegacyError operation
pub struct LegacyErrorResponse {
    /// Error
    pub error: String,
    /// Code
    pub code: Option<String>,
    /// Timestamp
    pub timestamp: String,
}
impl From<UnifiedErrorResponse> for LegacyErrorResponse {
    /// From
    fn from(unified: UnifiedErrorResponse) -> Self {
        Self {
            error: unified.message,
            code: Some(unified.code),
            timestamp: unified.timestamp,
        }
    }
}

impl IntoResponse for LegacyErrorResponse {
    /// Into Response
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::{ErrorResponseFactory, UnifiedErrorResponse};
    use axum::http::StatusCode;

    #[test]
    fn unified_error_simple_and_status() {
        let e = UnifiedErrorResponse::simple("m", "C", "comp");
        assert_eq!(e.status, 500);
        assert_eq!(e.code, "C");
        let e2 = UnifiedErrorResponse::with_status("m", "C", "comp", 418);
        assert_eq!(e2.status, 418);
    }

    #[test]
    fn to_status_code_clamps_invalid() {
        let e = UnifiedErrorResponse::with_status("x", "y", "z", 9999);
        assert_eq!(e.to_status_code(), StatusCode::INTERNAL_SERVER_ERROR);
        let ok = UnifiedErrorResponse::with_status("x", "y", "z", 200);
        assert_eq!(ok.to_status_code(), StatusCode::OK);
    }

    #[test]
    fn factory_common_errors_have_expected_codes() {
        assert_eq!(ErrorResponseFactory::bad_request("x").status, 400);
        assert_eq!(ErrorResponseFactory::internal("x").status, 500);
        assert_eq!(ErrorResponseFactory::not_found("/p").status, 404);
        assert_eq!(ErrorResponseFactory::unauthorized("op").status, 401);
        assert_eq!(ErrorResponseFactory::forbidden("r").status, 403);
        assert_eq!(ErrorResponseFactory::conflict("r").status, 409);
        assert_eq!(ErrorResponseFactory::rate_limited(Some(5)).status, 429);
        assert_eq!(ErrorResponseFactory::service_unavailable("s").status, 503);
        assert_eq!(ErrorResponseFactory::timeout("t").status, 408);
    }

    #[test]
    fn validation_error_carries_field_detail() {
        let e = ErrorResponseFactory::validation_error("f", "bad");
        assert_eq!(e.status, 400);
        let details = e.details.expect("details");
        assert_eq!(details.get("field"), Some(&serde_json::json!("f")));
    }

    #[test]
    fn legacy_from_unified_preserves_message() {
        let u = UnifiedErrorResponse::simple("msg", "CODE", "c");
        let leg = super::LegacyErrorResponse::from(u.clone());
        assert_eq!(leg.error, u.message);
        assert_eq!(leg.code, Some(u.code));
    }
}
