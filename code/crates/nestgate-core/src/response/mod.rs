// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

// Response Module System
// This module system breaks down the large response.rs file into manageable,
//! focused modules while maintaining the unified response architecture.
// **ACHIEVEMENT**: Reduces file sizes to <400 lines while preserving functionality
// Core response modules
/// AI-first response types optimized for human-centric interactions and clarity.
pub mod ai_first_response;
/// Standard API response wrappers for JSON and HTTP responses.
pub mod api_response;
/// Comprehensive tests for API responses
#[cfg(test)]
mod api_response_tests;
/// Comprehensive error response handling with context and suggestions.
pub mod error_response;
/// Fluent builder pattern for constructing complex responses.
pub mod response_builder;
/// Comprehensive tests for response builders
#[cfg(test)]
mod response_builder_tests;
/// Success response types with metadata and status information.
pub mod success_response;
/// Response conversion traits for seamless type transformations.
pub mod traits;
#[cfg(test)]
mod traits_coverage_tests;
// Re-export all types for backward compatibility and ease of use
pub use ai_first_response::{
    AIErrorCategory, AIFirstError, AIFirstResponse, AIResponseMetadata, CacheInfo, DataQuality,
    ErrorSeverity, HumanInteractionContext, HumanPriority, IntoAIFirstResponse, PerformanceMetrics,
    ResourceUsage, RetryStrategy, SuggestedAction, UIHints,
};
pub use api_response::{ApiResponse, EmptyResponse};
pub use error_response::{ErrorResponseFactory, LegacyErrorResponse, UnifiedErrorResponse};
pub use response_builder::ResponseBuilder;
pub use success_response::{SuccessResponse, SuccessResponseFactory};
pub use traits::{
    IntoApiResponse, IntoSuccessResponse, IntoUnifiedErrorResponse, ResponseChaining,
    ResponseConversion, ResponseMetadata,
};

// ==================== SECTION ====================

/// Response utility functions for common operations like pagination and batching.
pub mod utils {
    use super::{ApiResponse, UnifiedErrorResponse};
    use std::collections::HashMap;
    /// Create a paginated response wrapper
    #[must_use]
    pub fn paginated_response<T: serde::Serialize>(
        data: Vec<T>,
        page: usize,
        page_size: usize,
        total_count: usize,
    ) -> ApiResponse<serde_json::Value> {
        let total_pages = total_count.div_ceil(page_size);
        let has_next = page < total_pages;
        let has_previous = page > 1;

        let pagination_info = serde_json::json!({
            "items": data,
            "pagination": {
                "page": page,
                "page_size": page_size,
                "total_count": total_count,
                "total_pages": total_pages,
                "has_next": has_next,
                "has_previous": has_previous
            }
        });

        ApiResponse::success(pagination_info)
    }

    /// Create a batch operation response
    #[must_use]
    pub fn batch_response<T: serde::Serialize>(
        successful: Vec<T>,
        failed: Vec<(String, String)>, // (item_id, error_message)
    ) -> ApiResponse<serde_json::Value> {
        let batch_info = serde_json::json!({
            "successful": successful,
            "failed": failed,
            "summary": {
                "successful_count": successful.len(),
                "failed_count": failed.len(),
                "total_count": successful.len() + failed.len()
            }
        });

        if failed.is_empty() {
            ApiResponse::success(batch_info)
        } else {
            ApiResponse::new_error_with_code(
                format!("Batch operation completed with {} failures", failed.len()),
                "PARTIAL_SUCCESS".to_string(),
            )
            .with_meta("batch_results", batch_info)
        }
    }

    /// Create a health check response
    #[must_use]
    pub fn health_check_response(
        service: &str,
        status: &str,
        checks: HashMap<String, bool>,
    ) -> ApiResponse<serde_json::Value> {
        let all_healthy = checks.values().all(|&v| v);
        let health_data = serde_json::json!({
            "service": service,
            "status": status,
            "checks": checks,
            "overall_health": if all_healthy { "healthy" } else { "unhealthy" },
            "timestamp": chrono::Utc::now()
        });

        if all_healthy {
            ApiResponse::success(health_data)
        } else {
            ApiResponse::new_error_with_code(
                format!("{service} health check failed"),
                "HEALTH_CHECK_FAILED".to_string(),
            )
            .with_meta("health_details", health_data)
        }
    }

    /// Create a validation response with field-specific errors
    #[must_use]
    pub fn validation_response(field_errors: HashMap<String, Vec<String>>) -> UnifiedErrorResponse {
        let error_count = field_errors.values().map(std::vec::Vec::len).sum::<usize>();
        let message = format!(
            "Validation failed with {} errors across {} items",
            error_count,
            field_errors.len()
        );

        let mut response =
            UnifiedErrorResponse::simple(&message, "VALIDATION_ERROR", "nestgate-core");
        response = response.with_context("field_errors", serde_json::json!(field_errors));
        response = response.with_context("error_count", serde_json::json!(error_count));
        response = response.with_context("field_count", serde_json::json!(field_errors.len()));
        response
    }

    /// Create a rate limit response with retry information
    #[must_use]
    pub fn rate_limit_response(
        limit: u64,
        remaining: u64,
        reset_time: chrono::DateTime<chrono::Utc>,
    ) -> UnifiedErrorResponse {
        let mut response =
            UnifiedErrorResponse::simple("Rate limit exceeded", "RATE_LIMITED", "nestgate-core");
        response = response.with_context("rate_limit", serde_json::json!(limit));
        response = response.with_context("remaining", serde_json::json!(remaining));
        response = response.with_context("reset_time", serde_json::json!(reset_time));
        response = response.with_context(
            "retry_after_seconds",
            serde_json::json!((reset_time - chrono::Utc::now()).num_seconds()),
        );
        response
    }
}

// ==================== SECTION ====================

/// Middleware utilities for response processing including headers and transformations.
pub mod middleware {
    use axum::response::Response;

    /// Add correlation ID to response headers
    #[must_use]
    pub fn add_correlation_id(mut response: Response, correlation_id: &str) -> Response {
        if let Ok(headervalue) = correlation_id.parse() {
            response
                .headers_mut()
                .insert("X-Correlation-ID", headervalue);
        } else {
            // Use a safe default correlation ID if parsing fails
            if let Ok(defaultvalue) = "unknown".parse() {
                response
                    .headers_mut()
                    .insert("X-Correlation-ID", defaultvalue);
            }
        }
        response
    }

    /// Add standard security headers to response
    #[must_use]
    pub fn add_security_headers(mut response: Response) -> Response {
        let headers = response.headers_mut();

        // Use safe header insertion with fallback on parse errors
        if let Ok(value) = "nosniff".parse() {
            headers.insert("X-Content-Type-Options", value);
        }
        if let Ok(value) = "DENY".parse() {
            headers.insert("X-Frame-Options", value);
        }
        if let Ok(value) = "1; mode=block".parse() {
            headers.insert("X-XSS-Protection", value);
        }
        if let Ok(value) = "max-age=31536000; includeSubDomains".parse() {
            headers.insert("Strict-Transport-Security", value);
        }

        response
    }

    /// Add caching headers to response
    #[must_use]
    pub fn add_cache_headers(mut response: Response, max_age_seconds: u64) -> Response {
        let cachevalue = format!("max-age={max_age_seconds}, public");
        if let Ok(headervalue) = cachevalue.parse() {
            response.headers_mut().insert("Cache-Control", headervalue);
        }
        response
    }

    /// Add no-cache headers to response
    #[must_use]
    pub fn add_no_cache_headers(mut response: Response) -> Response {
        let headers = response.headers_mut();

        // Safe header insertion with fallback on parse errors
        if let Ok(value) = "no-cache, no-store, must-revalidate".parse() {
            headers.insert("Cache-Control", value);
        }
        if let Ok(value) = "no-cache".parse() {
            headers.insert("Pragma", value);
        }
        if let Ok(value) = "0".parse() {
            headers.insert("Expires", value);
        }

        response
    }
}

// ==================== SECTION ====================

/// Validation utilities for responses ensuring data integrity and consistency.
pub mod validation {
    use super::{ApiResponse, SuccessResponse, UnifiedErrorResponse};
    /// Validate API response structure
    pub fn validate_api_response<T>(response: &ApiResponse<T>) -> std::result::Result<(), String> {
        if response.success && response.data.is_none() {
            return Err("Successful response must have data".to_string());
        }

        if !response.success && response.error.is_none() {
            return Err("Failed response must have error message".to_string());
        }

        if response.success && response.error.is_some() {
            return Err("Successful response cannot have error message".to_string());
        }
        Ok(())
    }

    /// Validate unified error response structure
    pub fn validate_unified_error_response(
        response: &mut UnifiedErrorResponse,
    ) -> std::result::Result<(), String> {
        if response.component.is_empty() {
            // PEDANTIC: Fixed from service_name to component
            response.component = "unknown".to_string();
        }
        Ok(())
    }

    /// Validate success response structure
    pub fn validate_success_response(
        response: &SuccessResponse,
    ) -> std::result::Result<(), String> {
        if response.message.is_empty() {
            return Err("Success response must have message".to_string());
        }
        Ok(())
    }
}

// ==================== SECTION ====================

#[cfg(any(test, feature = "dev-stubs"))]
/// Testing utilities for response types.
///
/// This module provides test helpers, builders, and mock responses
/// for comprehensive testing of API response patterns.
pub mod testing {
    use super::{ApiResponse, SuccessResponse, UnifiedErrorResponse};

    /// Create a test API response with mock data
    pub fn mock_api_response<T: serde::Serialize>(data: T) -> ApiResponse<T> {
        ApiResponse::new_success(data)
    }

    /// Create a test error response
    #[must_use]
    pub fn mock_error_response(message: &str) -> UnifiedErrorResponse {
        UnifiedErrorResponse::simple(message, "TEST_ERROR", "test-service")
    }

    /// Create a test success response
    #[must_use]
    pub fn mock_success_response(message: &str) -> SuccessResponse {
        SuccessResponse::new(message)
    }

    /// Assert response is successful
    pub fn assert_response_success<T>(response: &ApiResponse<T>) {
        assert!(
            response.success,
            "Response should be successful: {:?}",
            response.error
        );
    }

    /// Assert response is error
    pub fn assert_response_error<T>(response: &ApiResponse<T>) {
        assert!(!response.success, "Response should be error");
        assert!(
            response.error.is_some(),
            "Error response should have error message"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::middleware;
    use super::utils;
    use super::validation;
    use super::{ApiResponse, SuccessResponse, UnifiedErrorResponse};
    use axum::body::Body;
    use axum::response::Response;
    use chrono::Utc;
    use std::collections::HashMap;

    fn sample_api_response_invalid_success_missing_data() -> ApiResponse<i32> {
        ApiResponse {
            request_id: "rid".to_string(),
            status: crate::canonical_types::ResponseStatus::Success,
            success: true,
            data: None,
            error: None,
            error_code: None,
            timestamp: Utc::now(),
            metadata: None,
            processing_time_ms: 0,
        }
    }

    #[test]
    fn utils_paginated_response_includes_pagination() {
        let items = vec![serde_json::json!({"n": 1}), serde_json::json!({"n": 2})];
        let resp = utils::paginated_response(items, 1, 10, 23);
        assert!(resp.success);
        let data = resp.data.expect("test: paginated data");
        let total_pages = data["pagination"]["total_pages"]
            .as_u64()
            .expect("test: total_pages number");
        assert_eq!(total_pages, 3);
    }

    #[test]
    fn utils_batch_response_all_success() {
        let resp = utils::batch_response(vec![serde_json::json!({"a": 1})], vec![]);
        assert!(resp.success);
    }

    #[test]
    fn utils_batch_response_partial_failure() {
        let resp = utils::batch_response(
            vec![serde_json::json!(true)],
            vec![("id1".to_string(), "boom".to_string())],
        );
        assert!(!resp.success);
        assert!(
            resp.error
                .expect("test: batch error msg")
                .contains("failures")
        );
    }

    #[test]
    fn utils_health_check_response_healthy_vs_unhealthy() {
        let mut ok = HashMap::new();
        ok.insert("disk".to_string(), true);
        let good = utils::health_check_response("svc", "up", ok);
        assert!(good.success);

        let mut bad = HashMap::new();
        bad.insert("disk".to_string(), false);
        let poor = utils::health_check_response("svc", "degraded", bad);
        assert!(!poor.success);
    }

    #[test]
    fn utils_validation_response_sets_context() {
        let mut fe = HashMap::new();
        fe.insert("email".to_string(), vec!["invalid".to_string()]);
        let u = utils::validation_response(fe);
        assert_eq!(u.code, "VALIDATION_ERROR");
        let details = u.details.expect("test: details");
        assert!(details.contains_key("field_errors"));
    }

    #[test]
    fn utils_rate_limit_response_has_retry_context() {
        let reset = Utc::now() + chrono::Duration::seconds(30);
        let u = utils::rate_limit_response(100, 0, reset);
        let details = u.details.expect("test: rate details");
        assert!(details.contains_key("rate_limit"));
        assert!(details.contains_key("retry_after_seconds"));
    }

    #[test]
    fn validation_api_response_errors() {
        let err =
            validation::validate_api_response(&sample_api_response_invalid_success_missing_data());
        assert!(
            err.expect_err("test: invalid success")
                .contains("must have data")
        );

        let invalid_fail = ApiResponse::<()> {
            request_id: "r".to_string(),
            status: crate::canonical_types::ResponseStatus::Error,
            success: false,
            data: None,
            error: None,
            error_code: None,
            timestamp: Utc::now(),
            metadata: None,
            processing_time_ms: 0,
        };
        let e2 = validation::validate_api_response(&invalid_fail);
        assert!(
            e2.expect_err("test: failed without error message")
                .contains("must have error message")
        );
    }

    #[test]
    fn validation_api_response_success_and_conflicting_error() {
        let ok = ApiResponse::success(1_i32);
        validation::validate_api_response(&ok).expect("test: valid success");

        let mut conflict = ApiResponse::success(1_i32);
        conflict.error = Some("oops".to_string());
        let e = validation::validate_api_response(&conflict);
        assert!(e.expect_err("test: conflict").contains("cannot have error"));
    }

    #[test]
    fn validation_unified_error_fills_empty_component() {
        let mut u = UnifiedErrorResponse::simple("m", "C", "");
        validation::validate_unified_error_response(&mut u).expect("test: validate unified");
        assert_eq!(u.component, "unknown");
    }

    #[test]
    fn validation_success_response_requires_message() {
        let bad = SuccessResponse {
            message: String::new(),
            data: None,
            metadata: HashMap::new(),
            timestamp: Utc::now().to_rfc3339(),
            correlation_id: None,
        };
        assert!(
            validation::validate_success_response(&bad)
                .expect_err("test: empty message")
                .contains("must have message")
        );
        let good = SuccessResponse::new("ok");
        validation::validate_success_response(&good).expect("test: good success");
    }

    #[test]
    fn middleware_add_correlation_id_and_security_headers() {
        let base = Response::new(Body::empty());
        let with_id = middleware::add_correlation_id(base, "abc-123");
        assert!(with_id.headers().get("X-Correlation-ID").is_some());

        let secured = middleware::add_security_headers(Response::new(Body::empty()));
        assert!(secured.headers().get("X-Content-Type-Options").is_some());

        let cached = middleware::add_cache_headers(Response::new(Body::empty()), 60);
        assert!(cached.headers().get("Cache-Control").is_some());

        let no_cache = middleware::add_no_cache_headers(Response::new(Body::empty()));
        assert!(no_cache.headers().get("Cache-Control").is_some());
    }
}
