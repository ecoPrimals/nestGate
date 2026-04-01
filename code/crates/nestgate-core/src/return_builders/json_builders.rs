// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use chrono::Utc;
/// **RETURN BUILDERS - JSON BUILDERS MODULE**
/// Contains JSON and API response construction functions.
/// Extracted from the large `return_builders.rs` to achieve file size compliance.
use serde_json::{Value as JsonValue, json};
use std::collections::HashMap;
/// Build successful API response
/// **PURE FUNCTION**: API response construction with success status
/// **TESTABLE**: Can verify all field assignments without side effects
pub fn build_api_success<T>(data: T) -> crate::response::ApiResponse<T> {
    crate::response::ApiResponse {
        request_id: uuid::Uuid::new_v4().to_string(),
        status: crate::canonical_types::ResponseStatus::Success,
        success: true,
        data: Some(data),
        error: None,
        error_code: None,
        timestamp: Utc::now(),
        metadata: Some(HashMap::new()),
        processing_time_ms: 0,
    }
}
/// Build successful API response with metadata
/// **PURE FUNCTION**: API response with metadata construction
/// **TESTABLE**: Can verify metadata field assignments
pub fn build_api_success_with_metadata<T>(
    data: T,
    metadata: HashMap<String, JsonValue>,
) -> crate::response::ApiResponse<T> {
    build_api_success_with_metadata_and_request_id(data, metadata, None)
}
/// Build successful API response with metadata and request ID
/// **PURE FUNCTION**: API response with metadata and request ID construction
/// **TESTABLE**: Can verify metadata and request ID field assignments
pub fn build_api_success_with_metadata_and_request_id<T>(
    data: T,
    metadata: HashMap<String, JsonValue>,
    request_id: Option<String>,
) -> crate::response::ApiResponse<T> {
    let mut final_metadata = metadata;
    if let Some(req_id) = request_id {
        final_metadata.insert("request_id".to_string(), JsonValue::String(req_id));
    }
    crate::response::ApiResponse {
        request_id: uuid::Uuid::new_v4().to_string(),
        status: crate::canonical_types::ResponseStatus::Success,
        success: true,
        data: Some(data),
        error: None,
        error_code: None,
        timestamp: Utc::now(),
        metadata: Some(final_metadata),
        processing_time_ms: 0,
    }
}

/// Build API error response
/// **PURE FUNCTION**: API error response construction
/// **TESTABLE**: Can verify error field assignments
#[must_use]
pub fn build_api_error<T>(
    error_message: String,
    error_code: Option<String>,
) -> crate::response::ApiResponse<T> {
    crate::response::ApiResponse {
        request_id: uuid::Uuid::new_v4().to_string(),
        status: crate::canonical_types::ResponseStatus::Error,
        success: false,
        data: None,
        error: Some(error_message),
        error_code,
        timestamp: Utc::now(),
        metadata: Some(HashMap::new()),
        processing_time_ms: 0,
    }
}
/// Build JSON response with flexible structure
/// **PURE FUNCTION**: JSON response construction
/// **TESTABLE**: Can verify JSON structure field assignments
#[must_use]
pub fn build_json_response(
    success: bool,
    message: Option<String>,
    data: Option<JsonValue>,
) -> JsonValue {
    let timestamp = Utc::now().timestamp();
    let mut response = json!({
        "success": success,
        "timestamp": timestamp
    });

    if success {
        if let Some(msg) = message {
            response["message"] = JsonValue::String(msg);
        }
    } else if let Some(error_msg) = message {
        response["error"] = JsonValue::String(error_msg);
    }

    if let Some(response_data) = data {
        response["data"] = response_data;
    }

    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical_types::ResponseStatus;
    use serde_json::{Value as JsonValue, json};

    #[test]
    fn build_api_success_sets_success_status_and_data() {
        let resp = build_api_success("payload");
        assert!(resp.success);
        assert_eq!(resp.status, ResponseStatus::Success);
        assert_eq!(resp.data, Some("payload"));
        assert!(resp.error.is_none());
        assert!(resp.metadata.is_some());
        assert!(!resp.request_id.is_empty());
    }

    #[test]
    fn build_api_success_with_metadata_merges_custom_fields() {
        let mut meta = HashMap::new();
        meta.insert("k".to_string(), json!("v"));
        let resp = build_api_success_with_metadata("x", meta.clone());
        assert_eq!(resp.metadata.as_ref().unwrap().get("k"), Some(&json!("v")));
    }

    #[test]
    fn build_api_success_with_metadata_inserts_request_id_when_provided() {
        let resp = build_api_success_with_metadata_and_request_id(
            1_u8,
            HashMap::new(),
            Some("req-abc".to_string()),
        );
        assert_eq!(
            resp.metadata.as_ref().unwrap().get("request_id"),
            Some(&JsonValue::String("req-abc".to_string()))
        );
    }

    #[test]
    fn build_api_error_sets_message_and_optional_code() {
        let resp: crate::response::ApiResponse<()> =
            build_api_error("bad".to_string(), Some("E1".to_string()));
        assert!(!resp.success);
        assert_eq!(resp.status, ResponseStatus::Error);
        assert_eq!(resp.error.as_deref(), Some("bad"));
        assert_eq!(resp.error_code.as_deref(), Some("E1"));
        assert!(resp.data.is_none());
    }

    #[test]
    fn build_json_response_success_includes_message_and_optional_data() {
        let v = build_json_response(true, Some("ok".to_string()), Some(json!({"id": 7})));
        assert_eq!(v["success"], json!(true));
        assert_eq!(v["message"], json!("ok"));
        assert_eq!(v["data"], json!({"id": 7}));
        assert!(v.get("error").is_none());
    }

    #[test]
    fn build_json_response_failure_uses_error_field_not_message() {
        let v = build_json_response(false, Some("nope".to_string()), None);
        assert_eq!(v["success"], json!(false));
        assert_eq!(v["error"], json!("nope"));
        assert!(v.get("message").is_none());
    }
}
