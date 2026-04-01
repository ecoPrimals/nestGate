// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Integration Tests
//!
//! Auto-generated from smart refactoring of client_tests.rs
//! Sections: INTEGRATION TESTS

use super::super::client::*;

// ==================== INTEGRATION TESTS ====================
#[test]
fn test_port_in_endpoint() {
    let port = Port::new(3000).expect("Network operation failed");
    let endpoint = Endpoint::http("api.example.com".to_string(), port);

    assert_eq!(endpoint.url(""), "http://api.example.com:3000");
}

#[test]
fn test_multiple_ports() {
    let ports: Vec<crate::Result<Port>> = vec![
        Port::new(0),
        Port::new(80),
        Port::new(443),
        Port::new(8080),
        Port::new(65535),
    ];

    assert!(ports[0].is_err()); // 0 is invalid
    assert!(ports[1].is_ok());
    assert!(ports[2].is_ok());
    assert!(ports[3].is_ok());
    assert!(ports[4].is_ok());
}

#[test]
fn test_all_methods() {
    let methods = [
        Method::Get,
        Method::Post,
        Method::Put,
        Method::Delete,
        Method::Patch,
        Method::Head,
        Method::Options,
    ];

    assert_eq!(methods.len(), 7);
}

#[test]
fn test_status_code_ranges() {
    let codes = [
        StatusCode::new(200),
        StatusCode::new(201),
        StatusCode::new(299),
        StatusCode::new(300),
        StatusCode::new(399),
        StatusCode::new(400),
        StatusCode::new(404),
        StatusCode::new(500),
    ];

    assert!(codes[0].is_success()); // 200
    assert!(codes[1].is_success()); // 201
    assert!(codes[2].is_success()); // 299
    assert!(!codes[3].is_success()); // 300
    assert!(!codes[4].is_success()); // 399
    assert!(codes[5].is_error()); // 400
    assert!(codes[6].is_error()); // 404
    assert!(codes[7].is_error()); // 500
}
