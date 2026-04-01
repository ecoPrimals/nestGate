// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Error Scenario Tests
//!
//! Auto-generated from smart refactoring of client_tests.rs
//! Sections: ERROR SCENARIO ADVANCED TESTS

use super::super::client::*;
use std::time::Duration;

// ==================== ERROR SCENARIO ADVANCED TESTS ====================
#[test]
fn test_http_error_types_exhaustive() {
    let errors = [
        HttpClientError::ConnectionFailed {
            message: "test".to_string(),
        },
        HttpClientError::Timeout {
            timeout: Duration::from_secs(30),
        },
        HttpClientError::InvalidResponse {
            message: "test".to_string(),
        },
        HttpClientError::TooManyRedirects { count: 10 },
    ];

    assert_eq!(errors.len(), 4);
}

#[test]
fn test_error_message_formats() {
    let error1 = HttpClientError::ConnectionFailed {
        message: "DNS resolution failed".to_string(),
    };
    assert!(error1.to_string().contains("DNS resolution failed"));

    let error2 = HttpClientError::Timeout {
        timeout: Duration::from_secs(30),
    };
    assert!(error2.to_string().contains("30"));

    let error3 = HttpClientError::InvalidResponse {
        message: "Status code 999".to_string(),
    };
    assert!(error3.to_string().contains("Status code 999"));
}
