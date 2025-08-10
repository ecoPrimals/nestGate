/// Response Builder Module
/// Utilities for building consistent HTTP responses
/// **PROBLEM SOLVED**: Centralized response building patterns
use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::Serialize;
use std::collections::HashMap;

use super::error_response::{ErrorResponseFactory, UnifiedErrorResponse};

/// Response builder for creating consistent HTTP responses
pub struct ResponseBuilder;

impl ResponseBuilder {
    /// Create an error JSON response
    pub fn error_json(message: String) -> Json<serde_json::Value> {
        Json(serde_json::json!({
            "success": false,
            "error": message,
            "timestamp": chrono::Utc::now()
        }))
    }

    /// Create a success JSON response with message
    pub fn success_json(message: String) -> Json<serde_json::Value> {
        Json(serde_json::json!({
            "success": true,
            "message": message,
            "timestamp": chrono::Utc::now()
        }))
    }

    /// Create a success JSON response with data
    pub fn success_json_with_data<T: Serialize>(
        message: String,
        data: T,
    ) -> Json<serde_json::Value> {
        Json(serde_json::json!({
            "success": true,
            "message": message,
            "data": data,
            "timestamp": chrono::Utc::now()
        }))
    }

    /// Create an error response with custom status code
    pub fn error_with_status(status: StatusCode, message: String) -> impl IntoResponse {
        (status, Self::error_json(message))
    }

    /// Create a success response with custom status code
    pub fn success_with_status(status: StatusCode, message: String) -> impl IntoResponse {
        (status, Self::success_json(message))
    }

    /// Create a service unavailable response
    pub fn service_unavailable(service: &str) -> impl IntoResponse {
        let error = ErrorResponseFactory::service_unavailable(service);
        (StatusCode::SERVICE_UNAVAILABLE, Json(error.serialize()))
    }

    /// Create an internal server error response
    pub fn internal_error(error: String) -> impl IntoResponse {
        let error_response = ErrorResponseFactory::internal_error(&error);
        error_response.into_response()
    }

    /// Create a not found response
    pub fn not_found(resource: &str) -> impl IntoResponse {
        let error = ErrorResponseFactory::not_found(resource);
        error.into_response()
    }

    /// Create a bad request response
    pub fn bad_request(message: String) -> impl IntoResponse {
        (StatusCode::BAD_REQUEST, Self::error_json(message))
    }

    /// Create an unauthorized response
    pub fn unauthorized(operation: &str) -> impl IntoResponse {
        let error = ErrorResponseFactory::unauthorized(operation);
        error.into_response()
    }

    /// Create a forbidden response
    pub fn forbidden(resource: &str) -> impl IntoResponse {
        let error = UnifiedErrorResponse::simple(
            &format!("Access to {resource} is forbidden"),
            "FORBIDDEN",
            "nestgate-core",
        );
        (StatusCode::FORBIDDEN, Json(error.serialize()))
    }

    /// Create a timeout response
    pub fn timeout(operation: &str) -> impl IntoResponse {
        let error = ErrorResponseFactory::timeout_error(operation, 30000);
        error.into_response()
    }

    /// Create a validation error response
    pub fn validation_error(field: &str, message: &str) -> impl IntoResponse {
        let error = ErrorResponseFactory::validation_error(field, message);
        error.into_response()
    }

    /// Create a created response (201)
    pub fn created(resource: &str, id: &str) -> impl IntoResponse {
        let success = super::success_response::SuccessResponseFactory::created(resource, id);
        (StatusCode::CREATED, Json(success))
    }

    /// Create an accepted response (202)
    pub fn accepted(message: &str) -> impl IntoResponse {
        (
            StatusCode::ACCEPTED,
            Self::success_json(message.to_string()),
        )
    }

    /// Create a no content response (204)
    pub fn no_content() -> impl IntoResponse {
        StatusCode::NO_CONTENT
    }

    /// Create a rate limited response (429)
    pub fn rate_limited(retry_after_seconds: u64) -> impl IntoResponse {
        let mut headers = axum::http::HeaderMap::new();
        if let Ok(header_value) = retry_after_seconds.to_string().parse() {
            headers.insert(axum::http::header::RETRY_AFTER, header_value);
        } else {
            tracing::error!(
                "Failed to parse retry_after_seconds as header value: {}",
                retry_after_seconds
            );
            // Use default of 60 seconds if parsing fails
            headers.insert(
                axum::http::header::RETRY_AFTER,
                "60".parse()
                    .unwrap_or_else(|_| axum::http::HeaderValue::from_static("60")),
            );
        }

        let error =
            UnifiedErrorResponse::simple("Rate limit exceeded", "RATE_LIMITED", "nestgate-core");

        (
            StatusCode::TOO_MANY_REQUESTS,
            headers,
            Json(error.serialize()),
        )
    }

    /// Create a maintenance response (503)
    pub fn maintenance(message: &str) -> impl IntoResponse {
        let error = UnifiedErrorResponse::simple(message, "MAINTENANCE", "nestgate-core");

        (StatusCode::SERVICE_UNAVAILABLE, Json(error.serialize()))
    }
}

/// Advanced response builder with fluent API
pub struct FluentResponseBuilder {
    status: StatusCode,
    headers: HashMap<String, String>,
    data: Option<serde_json::Value>,
    error: Option<String>,
    metadata: HashMap<String, serde_json::Value>,
}

impl FluentResponseBuilder {
    /// Create a new fluent response builder
    pub fn new() -> Self {
        Self {
            status: StatusCode::OK,
            headers: HashMap::new(),
            data: None,
            error: None,
            metadata: HashMap::new(),
        }
    }

    /// Set the HTTP status code
    pub fn status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    /// Add a header
    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    /// Set the response data
    pub fn data<T: Serialize>(mut self, data: T) -> Self {
        self.data = Some(serde_json::to_value(data).unwrap_or_default());
        self
    }

    /// Set an error message
    pub fn error(mut self, message: &str) -> Self {
        self.error = Some(message.to_string());
        self
    }

    /// Add metadata
    pub fn metadata(mut self, key: &str, value: serde_json::Value) -> Self {
        self.metadata.insert(key.to_string(), value);
        self
    }

    /// Build the final response
    pub fn build(self) -> impl IntoResponse {
        let mut response_headers = axum::http::HeaderMap::new();
        for (key, value) in self.headers {
            if let (Ok(name), Ok(val)) = (
                axum::http::HeaderName::from_bytes(key.as_bytes()),
                axum::http::HeaderValue::from_str(&value),
            ) {
                response_headers.insert(name, val);
            }
        }

        let response_body = if let Some(error) = self.error {
            serde_json::json!({
                "success": false,
                "error": error,
                "metadata": self.metadata,
                "timestamp": chrono::Utc::now()
            })
        } else {
            serde_json::json!({
                "success": true,
                "data": self.data,
                "metadata": self.metadata,
                "timestamp": chrono::Utc::now()
            })
        };

        (self.status, response_headers, Json(response_body))
    }
}

impl Default for FluentResponseBuilder {
    fn default() -> Self {
        Self::new()
    }
}
