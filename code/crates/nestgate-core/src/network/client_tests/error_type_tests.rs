// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Error Type Tests
//!
//! Auto-generated from smart refactoring of client_tests.rs
//! Sections: ERROR TYPE TESTS

use super::super::client::*;
use std::time::Duration;

// ==================== ERROR TYPE TESTS ====================
#[test]
fn test_http_client_error_connection_failed() {
    let error = HttpClientError::ConnectionFailed {
        message: "Connection refused".to_string(),
    };

    assert!(error.to_string().contains("Connection failed"));
    assert!(error.to_string().contains("Connection refused"));
}

#[test]
fn test_http_client_error_timeout() {
    let error = HttpClientError::Timeout {
        timeout: Duration::from_secs(30),
    };

    assert!(error.to_string().contains("timeout"));
}

#[test]
fn test_http_client_error_invalid_response() {
    let error = HttpClientError::InvalidResponse {
        message: "Malformed JSON".to_string(),
    };

    assert!(error.to_string().contains("Invalid response"));
    assert!(error.to_string().contains("Malformed JSON"));
}

#[test]
fn test_http_client_error_too_many_redirects() {
    let error = HttpClientError::TooManyRedirects { count: 10 };

    assert!(error.to_string().contains("Too many redirects"));
    assert!(error.to_string().contains("10"));
}
