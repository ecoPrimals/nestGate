// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use crate::canonical_types::ResponseStatus;
/// Response builder utilities for service responses
/// Provides convenient builders for common response patterns using canonical types
use crate::response::api_response::ApiResponse;
use std::collections::HashMap;

/// Build a successful service response using canonical types
#[must_use]
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
#[must_use]
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn build_success_response_matches_request_and_payload() {
        let r = build_success_response("rid-1".to_string(), json!({"a": 1}));
        assert_eq!(r.request_id, "rid-1");
        assert!(r.success);
        assert_eq!(r.status, ResponseStatus::Success);
        assert_eq!(r.data, Some(json!({"a": 1})));
        assert!(r.error.is_none());
    }

    #[test]
    fn build_error_response_carries_message() {
        let r = build_error_response("rid-2".to_string(), "failed".to_string());
        assert_eq!(r.request_id, "rid-2");
        assert!(!r.success);
        assert_eq!(r.status, ResponseStatus::Error);
        assert_eq!(r.error.as_deref(), Some("failed"));
        assert!(r.data.is_none());
    }
}
