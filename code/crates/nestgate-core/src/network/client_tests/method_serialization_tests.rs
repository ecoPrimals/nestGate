// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Method Serialization Tests
//!
//! Auto-generated from smart refactoring of client_tests.rs
//! Sections: METHOD SERIALIZATION TESTS

use super::super::client::*;

// ==================== METHOD SERIALIZATION TESTS ====================
#[test]
fn test_method_deserialization() {
    let json = r#""Post""#;
    let method: Result<Method, _> = serde_json::from_str(json);
    assert!(method.is_ok());
    assert_eq!(method.expect("Network operation failed"), Method::Post);
}

#[test]
fn test_all_methods_serialization_roundtrip() {
    let methods = [
        Method::Get,
        Method::Post,
        Method::Put,
        Method::Delete,
        Method::Patch,
        Method::Head,
        Method::Options,
    ];

    for method in &methods {
        let json = serde_json::to_string(method).expect("Network operation failed");
        let deserialized: Method = serde_json::from_str(&json).expect("Network operation failed");
        assert_eq!(*method, deserialized);
    }
}
