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
    // CLEANED: Removed unused Result import as part of canonical modernization
    // use crate::Result;
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

#[cfg(test)]
/// Testing utilities for response types.
///
/// This module provides test helpers, builders, and mock responses
/// for comprehensive testing of API response patterns.
pub mod testing {
    use super::*;

    /// Create a test API response with mock data
    pub fn mock_api_response<T: serde::Serialize>(data: T) -> ApiResponse<T> {
        ApiResponse::new_success(data)
    }

    /// Create a test error response
    pub fn mock_error_response(message: &str) -> UnifiedErrorResponse {
        UnifiedErrorResponse::simple(message, "TEST_ERROR", "test-service")
    }

    /// Create a test success response
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
