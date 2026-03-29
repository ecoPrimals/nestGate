// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Round 5: error constructors, `Display`, and `From` conversion branches.

use nestgate_types::error::NestGateError;
use nestgate_types::error::ResultExt;
use std::time::Duration;

#[test]
fn display_configuration_error() {
    let e = NestGateError::configuration_error("field", "bad");
    assert!(e.to_string().contains("Configuration"));
}

#[test]
fn display_api_error() {
    let e = NestGateError::api_error("fail");
    assert!(e.to_string().contains("API"));
}

#[test]
fn display_storage_error() {
    let e = NestGateError::storage_error("io");
    assert!(e.to_string().contains("Storage"));
}

#[test]
fn display_security_error() {
    let e = NestGateError::security_error("denied");
    assert!(e.to_string().contains("Security"));
}

#[test]
fn display_network_error() {
    let e = NestGateError::network_error("timeout");
    assert!(e.to_string().contains("Network"));
}

#[test]
fn display_validation_error() {
    let e = NestGateError::validation_error("invalid");
    assert!(e.to_string().contains("Validation"));
}

#[test]
fn display_timeout_error() {
    let e = NestGateError::timeout_error("op", Duration::from_millis(1));
    assert!(e.to_string().contains("Timeout"));
}

#[test]
fn api_not_found_branch() {
    let e = NestGateError::not_found("resource");
    let s = e.to_string();
    assert!(s.contains("Not found") || s.contains("404"));
}

#[test]
fn api_service_unavailable_branch() {
    let e = NestGateError::service_unavailable("backend");
    assert!(e.to_string().contains("unavailable") || e.to_string().contains("503"));
}

#[test]
fn api_with_status_branch() {
    let e = NestGateError::api_with_status("x", 418);
    assert!(e.to_string().contains("API"));
}

#[test]
fn api_with_context_branch() {
    let e = NestGateError::api_with_context("m", Some(400), None, Some("/p".into()));
    assert!(!e.to_string().is_empty());
}

#[test]
fn invalid_input_with_field_branch() {
    let e = NestGateError::invalid_input_with_field("f", "msg");
    assert!(e.to_string().contains("Validation") || e.to_string().contains("Invalid"));
}

#[test]
fn config_helpers_branch() {
    let a = NestGateError::config("x");
    let b = NestGateError::config_with_field("f", "m");
    assert_ne!(a.to_string(), b.to_string());
}

#[test]
fn from_io_error_branch() {
    let io = std::io::Error::new(std::io::ErrorKind::NotFound, "nope");
    let e: NestGateError = io.into();
    assert!(e.to_string().contains("I/O") || e.to_string().contains("Internal"));
}

#[test]
fn from_serde_json_error_branch() -> std::result::Result<(), &'static str> {
    let Err(err) = serde_json::from_str::<serde_json::Value>("not json") else {
        return Err("expected JSON parse failure");
    };
    let e: NestGateError = err.into();
    assert!(e.to_string().contains("JSON") || e.to_string().contains("Validation"));
    Ok(())
}

#[test]
fn from_string_branch() {
    let e: NestGateError = "hello".to_string().into();
    assert!(!e.to_string().is_empty());
}

#[test]
fn from_str_slice_branch() {
    let e: NestGateError = "slice".into();
    assert!(!e.to_string().is_empty());
}

#[test]
fn result_ext_to_canonical_ok() {
    let r: std::result::Result<i32, std::io::Error> = Ok(3);
    assert_eq!(r.to_canonical().ok(), Some(3));
}

#[test]
fn result_ext_to_canonical_err_io() {
    let r: std::result::Result<(), std::io::Error> = Err(std::io::Error::other("x"));
    assert!(r.to_canonical().is_err());
}

#[test]
fn configuration_error_detailed_branch() {
    let e = NestGateError::configuration_error_detailed(
        "f",
        "m",
        Some("c".into()),
        Some("e".into()),
        true,
    );
    assert!(e.to_string().contains("Configuration"));
}

#[test]
fn api_error_detailed_branch() {
    let e =
        NestGateError::api_error_detailed("m", Some(500), Some("rid".into()), Some("/a".into()));
    assert!(e.to_string().contains("API"));
}

#[test]
fn storage_error_detailed_branch() {
    let e = NestGateError::storage_error_detailed("m", Some("read".into()));
    assert!(e.to_string().contains("Storage"));
}

#[test]
fn network_error_detailed_branch() {
    let e = NestGateError::network_error_detailed("m", Some("op".into()), Some("e".into()));
    assert!(e.to_string().contains("Network"));
}

#[test]
fn validation_error_detailed_branch() {
    let e = NestGateError::validation_error_detailed(
        "m",
        Some("f".into()),
        Some("e".into()),
        Some("a".into()),
    );
    assert!(e.to_string().contains("Validation"));
}

#[test]
fn network_connection_failed_branch() {
    let e = NestGateError::network_connection_failed("127.0.0.1", 9, "refused");
    assert!(e.to_string().contains("Connection") || e.to_string().contains("Network"));
}

#[test]
fn network_timeout_helper_branch() {
    let e = NestGateError::network_timeout("http://x", Duration::from_secs(1));
    assert!(e.to_string().contains("timeout") || e.to_string().contains("Timeout"));
}

#[test]
fn storage_not_found_helper_branch() {
    let e = NestGateError::storage_not_found("/no/such");
    assert!(e.to_string().contains("not found") || e.to_string().contains("Storage"));
}

#[test]
fn storage_permission_denied_exercise() {
    let e = NestGateError::storage_permission_denied("/p", "read");
    assert!(e.to_string().contains("permission") || e.to_string().contains("Storage"));
}

#[test]
fn migrate_result_passthrough() {
    let r: nestgate_types::error::Result<i32> = Ok(1);
    assert_eq!(nestgate_types::error::migrate_result(r).ok(), Some(1));
}

#[test]
fn system_internal_external_validation_aliases() {
    let system_err = NestGateError::system("m", "c");
    let internal_err = NestGateError::internal("i");
    let internal_comp = NestGateError::internal_with_component("i", "comp");
    let internal_det = NestGateError::internal_error("i", "comp");
    let external_err = NestGateError::external_service_unavailable("svc", "down");
    let validation_err = NestGateError::validation("v");
    for err in [
        system_err,
        internal_err,
        internal_comp,
        internal_det,
        external_err,
        validation_err,
    ] {
        assert!(!err.to_string().is_empty());
    }
}

#[test]
fn from_anyhow_error_branch() {
    let e: NestGateError = anyhow::anyhow!("boom").into();
    assert!(e.to_string().contains("External") || e.to_string().contains("anyhow"));
}

#[test]
fn storage_disk_full_branch() {
    let e = NestGateError::storage_disk_full("/mnt", 100, 0);
    assert!(e.to_string().contains("Disk") || e.to_string().contains("Resource"));
}

#[test]
fn auth_security_authorization_branches() {
    let a = NestGateError::auth("bad");
    let s = NestGateError::security("sec");
    let z = NestGateError::authorization("no", "user1");
    for x in [a, s, z] {
        assert!(x.to_string().contains("Security"));
    }
}

#[test]
fn automation_branches() {
    let a = NestGateError::automation("a");
    let b = NestGateError::automation_operation("b", Some("t".into()));
    assert!(a.to_string().contains("Automation"));
    assert!(b.to_string().contains("Automation"));
}

#[test]
fn io_error_helper_branch() {
    let e = NestGateError::io_error("read failed");
    let s = e.to_string();
    assert!(s.contains("System") && s.contains("read failed"));
}

#[test]
fn api_helpers_round5() {
    let a = NestGateError::api("m");
    let b = NestGateError::invalid_input_with_field("f", "bad");
    assert_ne!(a.to_string(), b.to_string());
}

#[test]
fn result_ext_with_context_maps_err() {
    let r: std::result::Result<(), &str> = Err("e");
    let mapped = r.with_context(|| "ctx".to_string());
    assert!(mapped.is_err());
}

#[test]
fn validation_message_roundtrip_non_empty() {
    let e = NestGateError::validation_error("must be positive");
    assert!(!e.to_string().is_empty());
}

#[test]
fn network_error_and_storage_error_distinct() {
    let n = NestGateError::network_error("n");
    let s = NestGateError::storage_error("s");
    assert_ne!(n.to_string(), s.to_string());
}

#[test]
fn timeout_error_contains_operation() {
    let e = NestGateError::timeout_error("read", Duration::from_secs(2));
    assert!(e.to_string().contains("read"));
}
