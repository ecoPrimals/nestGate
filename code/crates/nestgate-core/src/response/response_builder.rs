// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// Response builder utilities
// Provides convenient methods for building HTTP responses

//! Response Builder module

use super::error_response::ErrorResponseFactory;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// CLEANED: Removed unused import as part of canonical modernization
// use super::success_response::SuccessResponseFactory;

/// Response builder for creating standardized HTTP responses
pub struct ResponseBuilder;
impl ResponseBuilder {
    /// Create a JSON success response
    pub fn success<T: Serialize>(data: T) -> impl IntoResponse {
        (StatusCode::OK, Json(data))
    }

    /// Create a created response with location
    pub fn created<T: Serialize>(data: T, location: Option<&str>) -> impl IntoResponse {
        let mut headers = axum::http::HeaderMap::new();
        if let Some(location) = location {
            if let Ok(locationvalue) = axum::http::HeaderValue::from_str(location) {
                headers.insert(axum::http::header::LOCATION, locationvalue);
            }
        }
        (StatusCode::CREATED, headers, Json(data))
    }

    /// Create an accepted response for async operations
    pub fn accepted<T: Serialize>(data: T) -> impl IntoResponse {
        (StatusCode::ACCEPTED, Json(data))
    }

    /// Create a no content response
    #[must_use]
    pub fn no_content() -> impl IntoResponse {
        StatusCode::NO_CONTENT
    }

    /// Create a not modified response
    #[must_use]
    pub fn not_modified() -> impl IntoResponse {
        StatusCode::NOT_MODIFIED
    }

    /// Create a bad request response
    #[must_use]
    pub fn bad_request(message: &str) -> impl IntoResponse {
        let error = ErrorResponseFactory::bad_request(message);
        error.into_response()
    }

    /// Create an unauthorized response
    #[must_use]
    pub fn unauthorized(operation: &str) -> impl IntoResponse {
        let error = ErrorResponseFactory::unauthorized(operation);
        error.into_response()
    }

    /// Create a forbidden response
    #[must_use]
    pub fn forbidden(resource: &str) -> impl IntoResponse {
        let error = ErrorResponseFactory::forbidden(resource);
        error.into_response()
    }

    /// Create a not found response
    #[must_use]
    pub fn not_found(path: &str) -> impl IntoResponse {
        let error = ErrorResponseFactory::not_found(path);
        error.into_response()
    }

    /// Create a conflict response
    #[must_use]
    pub fn conflict(resource: &str) -> impl IntoResponse {
        let error = ErrorResponseFactory::conflict(resource);
        error.into_response()
    }

    /// Create a validation error response
    #[must_use]
    pub fn validation_error(field: &str, message: &str) -> impl IntoResponse {
        let error = ErrorResponseFactory::validation_error(field, message);
        error.into_response()
    }

    /// Create a rate limited response
    #[must_use]
    pub fn rate_limited(retry_after: Option<u64>) -> impl IntoResponse {
        let error = ErrorResponseFactory::rate_limited(retry_after);
        error.into_response()
    }

    /// Create an internal server error response
    #[must_use]
    pub fn internal(message: &str) -> impl IntoResponse {
        let error = ErrorResponseFactory::internal(message);
        error.into_response()
    }

    /// Create a service unavailable response
    #[must_use]
    pub fn service_unavailable(service: &str) -> impl IntoResponse {
        let error = ErrorResponseFactory::service_unavailable(service);
        error.into_response()
    }

    /// Create a timeout response
    #[must_use]
    pub fn timeout(operation: &str) -> impl IntoResponse {
        let error = ErrorResponseFactory::timeout(operation);
        error.into_response()
    }

    /// Create a paginated response
    #[must_use]
    pub fn paginated<T: Serialize>(
        data: Vec<T>,
        page: u32,
        per_page: u32,
        total: u64,
    ) -> impl IntoResponse {
        let response = PaginatedResponse {
            data,
            pagination: PaginationMetadata {
                page,
                per_page,
                total,
                total_pages: (total as f64 / f64::from(per_page)).ceil() as u32,
            },
        };
        (StatusCode::OK, Json(response))
    }

    /// Create a response with custom headers
    pub fn with_headers<T: Serialize>(
        data: T,
        status: StatusCode,
        headers: HashMap<String, String>,
    ) -> impl IntoResponse {
        let mut header_map = axum::http::HeaderMap::new();
        for (key, value) in headers {
            if let (Ok(header_name), Ok(headervalue)) = (
                axum::http::HeaderName::from_bytes(key.as_bytes()),
                axum::http::HeaderValue::from_str(&value),
            ) {
                header_map.insert(header_name, headervalue);
            }
        }
        (status, header_map, Json(data))
    }

    /// Create a response with cache headers
    pub fn with_cache<T: Serialize>(
        data: T,
        max_age: u32,
        etag: Option<&str>,
    ) -> impl IntoResponse {
        let mut headers = axum::http::HeaderMap::new();

        // Set Cache-Control header
        if let Ok(cachevalue) = axum::http::HeaderValue::from_str(&format!("max-age={max_age}")) {
            headers.insert(axum::http::header::CACHE_CONTROL, cachevalue);
        }

        // Set ETag if provided
        if let Some(etag) = etag {
            if let Ok(etagvalue) = axum::http::HeaderValue::from_str(&format!("\"{etag}\"")) {
                headers.insert(axum::http::header::ETAG, etagvalue);
            }
        }

        (StatusCode::OK, headers, Json(data))
    }

    /// Create a streaming response placeholder
    #[must_use]
    pub fn stream_placeholder() -> impl IntoResponse {
        (
            StatusCode::OK,
            "Streaming response would be implemented here",
        )
    }
}

/// Paginated response structure
#[derive(Debug, Serialize, Deserialize)]
/// Response data for Paginated operation
pub struct PaginatedResponse<T> {
    /// Data
    pub data: Vec<T>,
    /// Pagination
    pub pagination: PaginationMetadata,
}
/// Pagination metadata
#[derive(Debug, Serialize, Deserialize)]
/// Paginationmetadata
pub struct PaginationMetadata {
    /// Page
    pub page: u32,
    /// Per Page
    pub per_page: u32,
    /// Total
    pub total: u64,
    /// Total Pages
    pub total_pages: u32,
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_builder_success() {
        let data = serde_json::json!({"message": "success"});
        let _response = ResponseBuilder::success(data);
        // Response building test would require axum test utilities
        // This is a placeholder for the actual test implementation
    }

    #[test]
    fn test_paginated_response() {
        let data = vec!["item1", "item2", "item3"];
        let _response = ResponseBuilder::paginated(data, 1, 10, 3);
        // Response building test would require axum test utilities
        // This is a placeholder for the actual test implementation
    }

    #[test]
    fn test_error_responses() {
        let _bad_request = ResponseBuilder::bad_request("Invalid input");
        let _not_found = ResponseBuilder::not_found("/api/test");
        let _unauthorized = ResponseBuilder::unauthorized("read");
        // Response building tests would require axum test utilities
        // These are placeholders for actual test implementations
    }
}
