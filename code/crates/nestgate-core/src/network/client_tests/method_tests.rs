// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Method Tests
//!
//! Auto-generated from smart refactoring of client_tests.rs
//! Sections: METHOD TESTS, METHOD COMPREHENSIVE TESTS

use super::super::client::*;

// ==================== METHOD TESTS ====================
#[test]
fn test_method_is_safe_get() {
    assert!(Method::Get.is_safe());
}

#[test]
fn test_method_is_safe_head() {
    assert!(Method::Head.is_safe());
}

#[test]
fn test_method_is_safe_options() {
    assert!(Method::Options.is_safe());
}

#[test]
fn test_method_is_not_safe_post() {
    assert!(!Method::Post.is_safe());
}

#[test]
fn test_method_is_not_safe_put() {
    assert!(!Method::Put.is_safe());
}

#[test]
fn test_method_is_not_safe_delete() {
    assert!(!Method::Delete.is_safe());
}

#[test]
fn test_method_can_have_body_post() {
    assert!(Method::Post.can_have_body());
}

#[test]
fn test_method_can_have_body_put() {
    assert!(Method::Put.can_have_body());
}

#[test]
fn test_method_can_have_body_patch() {
    assert!(Method::Patch.can_have_body());
}

#[test]
fn test_method_cannot_have_body_get() {
    assert!(!Method::Get.can_have_body());
}

#[test]
fn test_method_cannot_have_body_delete() {
    assert!(!Method::Delete.can_have_body());
}

#[test]
fn test_method_serialization() {
    let method = Method::Post;
    let json = serde_json::to_string(&method);
    assert!(json.is_ok());
}

// ==================== METHOD COMPREHENSIVE TESTS ====================
#[test]
fn test_all_methods_safe_unsafe() {
    assert!(Method::Get.is_safe());
    assert!(Method::Head.is_safe());
    assert!(Method::Options.is_safe());

    assert!(!Method::Post.is_safe());
    assert!(!Method::Put.is_safe());
    assert!(!Method::Delete.is_safe());
    assert!(!Method::Patch.is_safe());
}

#[test]
fn test_all_methods_body_capability() {
    assert!(!Method::Get.can_have_body());
    assert!(!Method::Head.can_have_body());
    assert!(!Method::Delete.can_have_body());
    assert!(!Method::Options.can_have_body());

    assert!(Method::Post.can_have_body());
    assert!(Method::Put.can_have_body());
    assert!(Method::Patch.can_have_body());
}
