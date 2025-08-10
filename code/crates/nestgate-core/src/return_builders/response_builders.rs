/// **RETURN BUILDERS - RESPONSE BUILDERS MODULE**
/// Contains service response construction functions for success and error cases.
/// Extracted from the large return_builders.rs to achieve file size compliance.
use chrono::Utc;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::time::Duration;

use crate::traits::{UniversalResponseStatus, UniversalServiceRequest, UniversalServiceResponse};

/// Build a successful service response using canonical types
pub fn build_success_response(
    request_id: String,
    data: serde_json::Value,
) -> UniversalServiceResponse {
    UniversalServiceResponse {
        request_id,
        status: UniversalResponseStatus::Success,
        data: Some(data),
        error: None,
        metadata: std::collections::HashMap::new(),
    }
}

/// Build an error service response using canonical types
pub fn build_error_response(
    error_code: String,
    error_message: String,
    _operation: String,
) -> UniversalServiceResponse {
    UniversalServiceResponse {
        request_id: uuid::Uuid::new_v4().to_string(),
        status: UniversalResponseStatus::Error,
        data: None,
        error: Some(error_message),
        metadata: {
            let mut metadata = std::collections::HashMap::new();
            metadata.insert("error_code".to_string(), error_code);
            metadata
        },
    }
}

/// Build service successful response with timing
/// **PURE FUNCTION**: Service response with duration tracking
/// **TESTABLE**: Can verify duration calculations and field consistency
pub fn build_service_success(
    request_id: String,
    payload: JsonValue,
    duration: Duration,
) -> crate::services::native_async_final_services::ServiceResponse {
    let timestamp = Utc::now();
    let processing_time = duration.as_millis() as u32;

    crate::services::native_async_final_services::ServiceResponse {
        success: true,
        data: serde_json::to_vec(&payload).unwrap_or_default(),
        request_id: Some(request_id),
        status: crate::traits::UniversalResponseStatus::Success,
        headers: HashMap::new(),
        payload,
        timestamp: timestamp.timestamp() as u64,
        duration,
        processing_time: processing_time.into(),
        tags: HashMap::new(),
        error_details: None,
        correlation_id: None,
        trace_id: None,
    }
}

/// Build service error response with error details
/// **PURE FUNCTION**: Service error response construction
/// **TESTABLE**: Can verify error status field assignments
pub fn build_service_error(
    request_id: String,
    code: u16,
    message: String,
    duration: Duration,
) -> crate::services::native_async_final_services::ServiceResponse {
    let timestamp = Utc::now();
    let processing_time = duration.as_millis() as u32;

    crate::services::native_async_final_services::ServiceResponse {
        success: false,
        data: Vec::new(),
        request_id: Some(request_id),
        status: crate::traits::UniversalResponseStatus::Error,
        headers: HashMap::new(),
        payload: JsonValue::Null,
        timestamp: timestamp.timestamp() as u64,
        duration,
        processing_time: processing_time.into(),
        tags: HashMap::new(),
        error_details: Some(format!("{code}: {message}")),
        correlation_id: None,
        trace_id: None,
    }
}

/// Build standardized response with metadata
/// **PURE FUNCTION**: Standardized response construction
/// **TESTABLE**: Can verify metadata field assignments
pub fn build_standardized_response(
    success: bool,
    message: String,
    data: Option<JsonValue>,
    request_id: String,
) -> crate::traits::UniversalServiceResponse {
    crate::traits::UniversalServiceResponse {
        request_id,
        status: if success {
            crate::traits::UniversalResponseStatus::Success
        } else {
            crate::traits::UniversalResponseStatus::Error
        },
        data,
        error: if !success { Some(message) } else { None },
        metadata: HashMap::new(),
    }
}

/// Build service response with custom headers
/// **PURE FUNCTION**: Service response with header customization
/// **TESTABLE**: Can verify header field assignments
pub fn build_service_response_with_headers(
    request_id: String,
    success: bool,
    payload: JsonValue,
    headers: HashMap<String, String>,
    duration: Duration,
) -> crate::services::native_async_final_services::ServiceResponse {
    let timestamp = Utc::now();
    let processing_time = duration.as_millis() as u32;

    crate::services::native_async_final_services::ServiceResponse {
        success,
        data: if success {
            serde_json::to_vec(&payload).unwrap_or_default()
        } else {
            Vec::new()
        },
        request_id: Some(request_id),
        status: if success {
            crate::traits::UniversalResponseStatus::Success
        } else {
            crate::traits::UniversalResponseStatus::Error
        },
        headers,
        payload,
        timestamp: timestamp.timestamp() as u64,
        duration,
        processing_time: processing_time.into(),
        tags: HashMap::new(),
        error_details: if !success {
            Some("Request failed".to_string())
        } else {
            None
        },
        correlation_id: None,
        trace_id: None,
    }
}

/// Build service request with metadata
/// **PURE FUNCTION**: Service request construction
/// **TESTABLE**: Can verify request field assignments
pub fn build_service_request(
    _operation: String,
    parameters: HashMap<String, serde_json::Value>,
    payload: Option<JsonValue>,
) -> UniversalServiceRequest {
    let mut params = parameters;
    if let Some(p) = payload {
        params.insert("payload".to_string(), p);
    }

    UniversalServiceRequest {
        request_id: uuid::Uuid::new_v4().to_string(),
        operation: "test_operation".to_string(),
        parameters: HashMap::new(),
        metadata: HashMap::new(),
    }
}
