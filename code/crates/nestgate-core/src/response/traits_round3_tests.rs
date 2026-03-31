// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Round 3 coverage for `response/traits.rs` conversions and metadata.

use crate::response::api_response::ApiResponse;
use crate::response::error_response::{LegacyErrorResponse, UnifiedErrorResponse};
use crate::response::success_response::SuccessResponse;
use crate::response::traits::{
    IntoApiResponse, IntoSuccessResponse, IntoUnifiedErrorResponse, ResponseChaining,
    ResponseConversion, ResponseMetadata,
};
use serde_json::json;

#[test]
fn into_api_response_ok_and_err() {
    let ok: ApiResponse<i32> = Ok::<i32, std::convert::Infallible>(7).into_api_response();
    assert!(ok.success);
    let err: ApiResponse<i32> =
        Result::<i32, std::io::Error>::Err(std::io::Error::other("boom")).into_api_response();
    assert!(!err.success);

    let masked: ApiResponse<i32> = Result::<i32, std::io::Error>::Err(std::io::Error::other("x"))
        .into_api_response_with_message("user-visible");
    assert_eq!(masked.error.as_deref(), Some("user-visible"));
}

#[test]
fn into_success_response_serializes_payload() {
    let v = json!({"a": 1});
    let s: SuccessResponse = v.into_success_response("ok");
    assert_eq!(s.message, "ok");
    assert!(s.data.is_some());
}

#[test]
fn into_unified_error_response_display() {
    let e = std::io::Error::other("io fail");
    let u = e.to_unified_error_response("svc");
    assert_eq!(u.component, "svc");
    assert!(!u.message.is_empty());
}

#[test]
fn legacy_error_response_converts_to_unified() {
    let leg = LegacyErrorResponse {
        error: "bad".into(),
        code: Some("E1".into()),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    let u: UnifiedErrorResponse = leg.convert();
    assert_eq!(u.message, "bad");
    assert_eq!(u.code, "E1");
}

#[test]
fn success_response_converts_to_api_response_metadata() {
    let s = SuccessResponse::new("done").add_data("k", json!("v"));
    let a: ApiResponse<()> = s.convert();
    assert!(a.success);
    assert!(a.metadata.is_some());
}

#[test]
fn response_metadata_api_and_success_and_unified() {
    let api = ApiResponse::success(42i32);
    assert!(api.is_successful());
    assert!(!api.extract_metadata().is_empty());

    let su = SuccessResponse::new("m");
    assert!(su.is_successful());
    assert!(su.extract_metadata().contains_key("message"));

    let ue = UnifiedErrorResponse::simple("e", "C", "comp");
    assert!(!ue.is_successful());
    let _ = ue.get_timestamp();
}

#[test]
fn response_chaining_unified_and_api() {
    let ts = chrono::DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
        .unwrap()
        .with_timezone(&chrono::Utc);
    let ue = UnifiedErrorResponse::simple("e", "C", "c").with_timestamp_chain(ts);
    assert!(ue.timestamp.contains("2024"));

    let api = ApiResponse::success(1u8).with_timestamp_chain(ts);
    assert_eq!(api.timestamp, ts);
}
