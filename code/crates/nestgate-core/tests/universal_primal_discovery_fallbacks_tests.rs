// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Comprehensive tests for universal_primal_discovery::fallbacks module
//!
//! Tests cover:
//! - All defined service fallback ports
//! - Default fallback behavior
//! - Edge cases and special inputs
//! - Service name variations

use nestgate_core::universal_primal_discovery::fallbacks::*;

// ==================== STANDARD SERVICE TESTS ====================

#[test]
fn test_api_fallback_port() {
    assert_eq!(get_fallback_port("api"), 8080);
}

#[test]
fn test_web_fallback_port() {
    assert_eq!(get_fallback_port("web"), 3000);
}

#[test]
fn test_metrics_fallback_port() {
    assert_eq!(get_fallback_port("metrics"), 9090);
}

#[test]
fn test_metrics_export_fallback_port() {
    assert_eq!(get_fallback_port("metrics_export"), 9090);
}

#[test]
fn test_nfs_fallback_port() {
    assert_eq!(get_fallback_port("nfs"), 2049);
}

#[test]
fn test_smb_fallback_port() {
    assert_eq!(get_fallback_port("smb"), 445);
}

#[test]
fn test_cifs_fallback_port() {
    assert_eq!(get_fallback_port("cifs"), 445);
}

#[test]
fn test_ftp_fallback_port() {
    assert_eq!(get_fallback_port("ftp"), 21);
}

#[test]
fn test_ssh_fallback_port() {
    assert_eq!(get_fallback_port("ssh"), 22);
}

#[test]
fn test_http_fallback_port() {
    assert_eq!(get_fallback_port("http"), 80);
}

#[test]
fn test_https_fallback_port() {
    assert_eq!(get_fallback_port("https"), 443);
}

#[test]
fn test_orchestration_fallback_port() {
    assert_eq!(get_fallback_port("orchestration"), 8081);
}

#[test]
fn test_coordination_fallback_port() {
    assert_eq!(get_fallback_port("coordination"), 8082);
}

#[test]
fn test_compute_fallback_port() {
    assert_eq!(get_fallback_port("compute"), 8083);
}

#[test]
fn test_ai_fallback_port() {
    assert_eq!(get_fallback_port("ai"), 8084);
}

#[test]
fn test_security_fallback_port() {
    assert_eq!(get_fallback_port("security"), 8085);
}

#[test]
fn test_auth_fallback_port() {
    assert_eq!(get_fallback_port("auth"), 8086);
}

// ==================== DEFAULT FALLBACK TESTS ====================

#[test]
fn test_unknown_service_default_fallback() {
    assert_eq!(get_fallback_port("unknown"), 8080);
}

#[test]
fn test_empty_string_default_fallback() {
    assert_eq!(get_fallback_port(""), 8080);
}

#[test]
fn test_random_string_default_fallback() {
    assert_eq!(get_fallback_port("random_service_xyz"), 8080);
}

#[test]
fn test_numeric_string_default_fallback() {
    assert_eq!(get_fallback_port("12345"), 8080);
}

// ==================== CASE SENSITIVITY TESTS ====================

#[test]
fn test_uppercase_api_returns_default() {
    // Function is case-sensitive, uppercase should return default
    assert_eq!(get_fallback_port("API"), 8080);
}

#[test]
fn test_mixed_case_http_returns_default() {
    assert_eq!(get_fallback_port("HTTP"), 8080);
}

#[test]
fn test_mixed_case_nfs_returns_default() {
    assert_eq!(get_fallback_port("NFS"), 8080);
}

// ==================== EDGE CASE TESTS ====================

#[test]
fn test_service_with_spaces_returns_default() {
    assert_eq!(get_fallback_port("api "), 8080);
    assert_eq!(get_fallback_port(" api"), 8080);
}

#[test]
fn test_service_with_special_chars_returns_default() {
    assert_eq!(get_fallback_port("api-service"), 8080);
    assert_eq!(get_fallback_port("api.service"), 8080);
}

#[test]
fn test_very_long_service_name() {
    let long_name = "a".repeat(1000);
    assert_eq!(get_fallback_port(&long_name), 8080);
}

// ==================== PORT VALUE VALIDATION TESTS ====================

#[test]
fn test_all_ports_are_valid() {
    // Valid ports are 1-65535
    let services = [
        "api",
        "web",
        "metrics",
        "metrics_export",
        "nfs",
        "smb",
        "cifs",
        "ftp",
        "ssh",
        "http",
        "https",
        "orchestration",
        "coordination",
        "compute",
        "ai",
        "security",
        "auth",
    ];

    for service in &services {
        let port = get_fallback_port(service);
        assert!(port > 0, "Invalid port for {}: {}", service, port);
    }
}

#[test]
fn test_well_known_ports() {
    // HTTP and HTTPS should be on standard well-known ports
    assert_eq!(get_fallback_port("http"), 80);
    assert_eq!(get_fallback_port("https"), 443);
    assert_eq!(get_fallback_port("ssh"), 22);
    assert_eq!(get_fallback_port("ftp"), 21);
}

#[test]
fn test_smb_cifs_same_port() {
    // SMB and CIFS should use the same port
    assert_eq!(get_fallback_port("smb"), get_fallback_port("cifs"));
}

#[test]
fn test_metrics_consistency() {
    // Both metrics variants should use the same port
    assert_eq!(
        get_fallback_port("metrics"),
        get_fallback_port("metrics_export")
    );
}

// ==================== ORCHESTRATION PORTS RANGE ====================

#[test]
fn test_orchestration_ports_sequential() {
    // Orchestration-related services should be in sequential range
    let orchestration = get_fallback_port("orchestration");
    let coordination = get_fallback_port("coordination");
    let compute = get_fallback_port("compute");

    assert_eq!(coordination, orchestration + 1);
    assert_eq!(compute, coordination + 1);
}

#[test]
fn test_service_ports_sequential() {
    // Service-related ports should be sequential
    let ai = get_fallback_port("ai");
    let security = get_fallback_port("security");
    let auth = get_fallback_port("auth");

    assert_eq!(security, ai + 1);
    assert_eq!(auth, security + 1);
}

// ==================== FUNCTION PROPERTIES TESTS ====================

#[test]
fn test_function_is_deterministic() {
    // Same input should always return same output
    for _ in 0..10 {
        assert_eq!(get_fallback_port("api"), 8080);
    }
}

#[test]
fn test_function_must_use_attribute() {
    // This test verifies the function returns a value (not void)
    let port = get_fallback_port("api");
    assert!(port > 0);
}
