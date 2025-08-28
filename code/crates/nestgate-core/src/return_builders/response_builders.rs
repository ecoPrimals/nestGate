/// Response builder utilities for service responses
/// Provides convenient builders for common response patterns using canonical types

use crate::response::api_response::ApiResponse;
use crate::canonical_types::ResponseStatus;
use std::collections::HashMap;

/// Build a successful service response using canonical types
pub fn build_success_response(
    request_id: String,
    data: serde_json::Value,
) -> ApiResponse<serde_json::Value> {
    ApiResponse {
        request_id,
        status: ResponseStatus::Success,
        success: true,
        data: Some(data),
        error: None,
        error_code: None,
        timestamp: chrono::Utc::now(),
        metadata: Some(HashMap::new()),
        processing_time_ms: 0,
    }
}

/// Build an error response
pub fn build_error_response(
    request_id: String,
    error_message: String,
) -> ApiResponse<serde_json::Value> {
    ApiResponse {
        request_id,
        status: ResponseStatus::Error,
        success: false,
        data: None,
        error: Some(error_message),
        error_code: None,
        timestamp: chrono::Utc::now(),
        metadata: Some(HashMap::new()),
        processing_time_ms: 0,
    }
}
