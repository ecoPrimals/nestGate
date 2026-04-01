// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Scheme Tests
//!
//! Auto-generated from smart refactoring of client_tests.rs
//! Sections: SCHEME TESTS, SCHEME COMPREHENSIVE TESTS

use super::super::client::*;

// ==================== SCHEME TESTS ====================
#[test]
fn test_scheme_http_display() {
    let scheme = Scheme::Http;
    assert_eq!(format!("{scheme}"), "http");
}

#[test]
fn test_scheme_https_display() {
    let scheme = Scheme::Https;
    assert_eq!(format!("{scheme}"), "https");
}

#[test]
fn test_scheme_equality() {
    assert_eq!(Scheme::Http, Scheme::Http);
    assert_eq!(Scheme::Https, Scheme::Https);
    assert_ne!(Scheme::Http, Scheme::Https);
}

// ==================== SCHEME COMPREHENSIVE TESTS ====================
#[test]
fn test_scheme_serialization() {
    let http = Scheme::Http;
    let https = Scheme::Https;

    let json_http = serde_json::to_string(&http);
    let json_https = serde_json::to_string(&https);

    assert!(json_http.is_ok());
    assert!(json_https.is_ok());
}

#[test]
fn test_scheme_clone() {
    let original = Scheme::Http;
    let cloned = original;

    assert_eq!(original, cloned);
}
