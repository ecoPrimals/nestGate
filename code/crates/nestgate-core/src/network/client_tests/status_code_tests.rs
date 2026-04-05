// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Status Code Tests
//!
//! Auto-generated from smart refactoring of client_tests.rs
//! Sections: STATUS CODE TESTS, STATUS CODE COMPREHENSIVE TESTS

use super::super::client::*;

// ==================== STATUS CODE TESTS ====================
#[test]
fn test_status_code_ok() {
    assert_eq!(StatusCode::OK.as_u16(), 200);
}

#[test]
fn test_status_code_created() {
    assert_eq!(StatusCode::CREATED.as_u16(), 201);
}

#[test]
fn test_status_code_bad_request() {
    assert_eq!(StatusCode::BAD_REQUEST.as_u16(), 400);
}

#[test]
fn test_status_code_not_found() {
    assert_eq!(StatusCode::NOT_FOUND.as_u16(), 404);
}

#[test]
fn test_status_code_internal_server_error() {
    assert_eq!(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), 500);
}

#[test]
fn test_status_code_is_success_200() {
    assert!(StatusCode::OK.is_success());
}

#[test]
fn test_status_code_is_success_201() {
    assert!(StatusCode::CREATED.is_success());
}

#[test]
fn test_status_code_is_not_success_400() {
    assert!(!StatusCode::BAD_REQUEST.is_success());
}

#[test]
fn test_status_code_is_error_400() {
    assert!(StatusCode::BAD_REQUEST.is_error());
}

#[test]
fn test_status_code_is_error_500() {
    assert!(StatusCode::INTERNAL_SERVER_ERROR.is_error());
}

#[test]
fn test_status_code_is_not_error_200() {
    assert!(!StatusCode::OK.is_error());
}

#[test]
fn test_status_code_custom() {
    let status = StatusCode::new(418); // I'm a teapot
    assert_eq!(status.as_u16(), 418);
    assert!(status.is_error());
}

// ==================== STATUS CODE COMPREHENSIVE TESTS ====================
#[test]
fn test_status_code_1xx_informational() {
    let codes = [
        StatusCode::new(100), // Continue
        StatusCode::new(101), // Switching Protocols
        StatusCode::new(102), // Processing
    ];

    for code in &codes {
        assert!(!code.is_success());
        assert!(!code.is_error());
    }
}

#[test]
fn test_status_code_2xx_success() {
    let codes = [
        StatusCode::new(200), // OK
        StatusCode::new(201), // Created
        StatusCode::new(202), // Accepted
        StatusCode::new(204), // No Content
        StatusCode::new(206), // Partial Content
    ];

    for code in &codes {
        assert!(code.is_success());
        assert!(!code.is_error());
    }
}

#[test]
fn test_status_code_3xx_redirection() {
    let codes = [
        StatusCode::new(301), // Moved Permanently
        StatusCode::new(302), // Found
        StatusCode::new(304), // Not Modified
        StatusCode::new(307), // Temporary Redirect
    ];

    for code in &codes {
        assert!(!code.is_success());
        assert!(!code.is_error());
    }
}

#[test]
fn test_status_code_4xx_client_errors() {
    let codes = [
        StatusCode::new(400), // Bad Request
        StatusCode::new(401), // Unauthorized
        StatusCode::new(403), // Forbidden
        StatusCode::new(404), // Not Found
        StatusCode::new(429), // Too Many Requests
    ];

    for code in &codes {
        assert!(!code.is_success());
        assert!(code.is_error());
    }
}

#[test]
fn test_status_code_5xx_server_errors() {
    let codes = [
        StatusCode::new(500), // Internal Server Error
        StatusCode::new(501), // Not Implemented
        StatusCode::new(502), // Bad Gateway
        StatusCode::new(503), // Service Unavailable
        StatusCode::new(504), // Gateway Timeout
    ];

    for code in &codes {
        assert!(!code.is_success());
        assert!(code.is_error());
    }
}
