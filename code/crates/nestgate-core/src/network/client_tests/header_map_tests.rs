// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Header Map Tests
//!
//! Auto-generated from smart refactoring of client_tests.rs
//! Sections: HEADER MAP TESTS

use super::super::client::*;
use std::collections::HashMap;

// ==================== HEADER MAP TESTS ====================
#[test]
fn test_header_map_operations() {
    let mut headers: HeaderMap = HashMap::new();

    headers.insert("content-type".to_string(), "application/json".to_string());
    headers.insert("authorization".to_string(), "Bearer token".to_string());

    assert_eq!(headers.len(), 2);
    assert!(headers.contains_key("content-type"));
    assert!(headers.contains_key("authorization"));
}

#[test]
fn test_header_map_case_sensitivity() {
    let mut headers: HeaderMap = HashMap::new();

    headers.insert("Content-Type".to_string(), "application/json".to_string());

    // HTTP headers are case-insensitive, but our map is case-sensitive
    // This is a known limitation - headers should be normalized
    assert!(headers.contains_key("Content-Type"));
    assert!(!headers.contains_key("content-type"));
}

#[test]
fn test_header_map_update() {
    let mut headers: HeaderMap = HashMap::new();

    headers.insert("x-version".to_string(), "1.0".to_string());
    assert_eq!(headers.get("x-version"), Some(&"1.0".to_string()));

    headers.insert("x-version".to_string(), "2.0".to_string());
    assert_eq!(headers.get("x-version"), Some(&"2.0".to_string()));
}
