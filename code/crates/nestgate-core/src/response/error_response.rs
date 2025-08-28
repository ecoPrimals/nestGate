/// Error Response Module
/// Unified error response handling and formatting
/// **PROBLEM SOLVED**: Consistent error response patterns across all APIs
use axum::response::{IntoResponse, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

// **MIGRATED**: Using canonical error context instead of deprecated unified_types
use crate::error::context::ErrorContext as UnifiedErrorContext;
use crate::NestGateError;

/// Unified error response with multiple formatting options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedErrorResponse {
    /// Error context with rich information
    pub context: UnifiedErrorContext,
    /// Response format preference (simple or detailed)
    pub format: ErrorResponseFormat,
}

/// Response format preference (simple or detailed)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorResponseFormat {
    /// Simple format for public APIs
    Simple,
    /// Detailed format for internal services and debugging
    Detailed,
    /// Statistical format for monitoring and analytics
    Statistics,
}

impl UnifiedErrorResponse {
    pub fn simple(message: &str, error_code: &str, service_name: &str) -> Self {
        Self {
            context: UnifiedErrorContext {
                error_id: uuid::Uuid::new_v4().to_string(),
                component: service_name.to_string(),
                operation: "error_response".to_string(),
                timestamp: std::time::SystemTime::now(),
                metadata: {
                    let mut map = std::collections::HashMap::new();
                    map.insert("message".to_string(), message.to_string());
                    map.insert("error_code".to_string(), error_code.to_string());
                    map
                },
                stack_trace: None,
                related_errors: Vec::new(),
                retry_info: None,
                recovery_suggestions: Vec::new(),
                performance_metrics: None,
                environment: None,
            },
            format: ErrorResponseFormat::Simple,
        }
    }

    pub fn detailed(context: UnifiedErrorContext) -> Self {
        Self {
            context,
            format: ErrorResponseFormat::Detailed,
        }
    }

    pub fn statistics(context: UnifiedErrorContext) -> Self {
        Self {
            context,
            format: ErrorResponseFormat::Statistics,
        }
    }

    /// Serialize the response according to its format
    pub fn serialize(&self) -> serde_json::Value {
        match self.format {
            ErrorResponseFormat::Simple => {
                serde_json::json!({
                    "error": self.context.metadata.get("message").unwrap_or(&"Unknown error".to_string()),
                    "code": self.context.metadata.get("error_code").unwrap_or(&"UNKNOWN".to_string()),
                    "service": self.context.component
                })
            }
            ErrorResponseFormat::Detailed => {
                serde_json::to_value(&self.context).unwrap_or_default()
            }
            ErrorResponseFormat::Statistics => {
                serde_json::json!({
                    "error_id": self.context.error_id,
                    "component": self.context.component,
                    "operation": self.context.operation,
                    "timestamp": self.context.timestamp,
                    "metadata": self.context.metadata
                })
            }
        }
    }

    /// Set the format type
    pub fn with_format(mut self, format: ErrorResponseFormat) -> Self {
        self.format = format;
        self
    }

    /// Add additional context
    pub fn with_context(mut self, key: &str, value: serde_json::Value) -> Self {
        self.context.metadata.insert(key.to_string(), value.to_string());
        self
    }
}

impl IntoResponse for UnifiedErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let status = match self.context.error_id.as_str() { // PEDANTIC: Fixed from error_code to error_id
            "VALIDATION_ERROR" => 400,
            "AUTHENTICATION_ERROR" => 401,
            "PERMISSION_DENIED" => 403,
            "NOT_FOUND" => 404,
            "TIMEOUT" => 408,
            "RATE_LIMIT" => 429,
            "INTERNAL_ERROR" => 500,
            "SERVICE_UNAVAILABLE" => 503,
            _ => 500,
        };

        (status, Json(self.serialize())).into_response()
    }
}

/// **DEPRECATED** - Legacy error response structure for backward compatibility
/// Use UnifiedErrorResponse for new implementations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// Error message
    pub error: String,
    /// Error code
    pub code: Option<String>,
    /// Service name that generated the error
    pub service: Option<String>,
    /// Timestamp when error occurred
    pub timestamp: DateTime<Utc>,
    /// Additional error context
    pub context: Option<HashMap<String, serde_json::Value>>,
}

impl From<ErrorResponse> for UnifiedErrorResponse {
    fn from(legacy: ErrorResponse) -> Self {
        let context = UnifiedErrorContext {
            error_id: uuid::Uuid::new_v4().to_string(),
            component: legacy.service.as_deref().unwrap_or("unknown").to_string(),
            operation: "legacy_conversion".to_string(),
            timestamp: std::time::SystemTime::now(),
            metadata: {
                let mut map = std::collections::HashMap::new();
                map.insert("message".to_string(), legacy.error);
                map.insert("error_code".to_string(), legacy.code.unwrap_or("UNKNOWN".to_string()));
                map
            },
            stack_trace: None,
            related_errors: Vec::new(),
            retry_info: None,
            recovery_suggestions: Vec::new(),
            performance_metrics: None,
            environment: None,
        };

        Self {
            context,
            format: ErrorResponseFormat::Simple,
        }
    }
}

impl ErrorResponse {
    /// Create a new error response
    pub fn new(message: &str) -> Self {
        Self {
            error: message.to_string(),
            code: None,
            service: None,
            timestamp: chrono::Utc::now(),
            context: None,
        }
    }

    /// Create an error response with error code
    pub fn with_code(message: &str, code: &str) -> Self {
        Self {
            error: message.to_string(),
            code: Some(code.to_string()),
            service: None,
            timestamp: chrono::Utc::now(),
            context: None,
        }
    }

    /// Create an error response with service name
    pub fn with_service(message: &str, service: &str) -> Self {
        Self {
            error: message.to_string(),
            code: None,
            service: Some(service.to_string()),
            timestamp: chrono::Utc::now(),
            context: None,
        }
    }

    /// Add context to the error response
    pub fn with_context(mut self, key: &str, value: serde_json::Value) -> Self {
        let mut context = self.context.unwrap_or_default();
        context.insert(key.to_string(), value);
        self.context = Some(context);
        self
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::BAD_REQUEST, Json(self)).into_response()
    }
}

/// Common error response factory methods
pub struct ErrorResponseFactory;

impl ErrorResponseFactory {
    /// Create a not found error
    pub fn not_found(resource: &str) -> UnifiedErrorResponse {
        UnifiedErrorResponse::simple(
            &format!("{resource} not found"),
            "NOT_FOUND",
            "nestgate-core",
        )
    }

    /// Create an unauthorized error
    pub fn unauthorized(operation: &str) -> UnifiedErrorResponse {
        UnifiedErrorResponse::simple(
            &format!("Unauthorized to perform {operation}"),
            "UNAUTHORIZED",
            "nestgate-core",
        )
    }

    /// Create a validation error
    pub fn validation_error(field: &str, message: &str) -> UnifiedErrorResponse {
        let mut response = UnifiedErrorResponse::simple(
            &format!("Validation failed for field '{field}': {message}"),
            "INVALID_INPUT",
            "nestgate-core",
        );
        response = response.with_context("field", serde_json::json!(field));
        response = response.with_context("validation_message", serde_json::json!(message));
        response
    }

    /// Create a service unavailable error
    pub fn service_unavailable(service: &str) -> UnifiedErrorResponse {
        UnifiedErrorResponse::simple(
            &format!("Service '{service}' is currently unavailable"),
            "SERVICE_UNAVAILABLE",
            "nestgate-core",
        )
    }

    /// Create an internal server error
    pub fn internal_error(message: &str) -> UnifiedErrorResponse {
        UnifiedErrorResponse::simple(message, "INTERNAL_ERROR", "nestgate-core")
    }

    /// Create a timeout error
    pub fn timeout_error(operation: &str, timeout_ms: u64) -> UnifiedErrorResponse {
        let mut response = UnifiedErrorResponse::simple(
            &format!("Operation '{operation}' timed out after {timeout_ms}ms"),
            "TIMEOUT",
            "nestgate-core",
        );
        response = response.with_context("operation", serde_json::json!(operation));
        response = response.with_context("timeout_ms", serde_json::json!(timeout_ms));
        response
    }
}
