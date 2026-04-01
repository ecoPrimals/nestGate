// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Connection error tests

use super::super::connection::ConnectionError;

#[test]
fn test_connection_error_creation() {
    let error = ConnectionError::Timeout("Connection timed out after 30s".to_string());
    assert!(format!("{error:?}").contains("Timeout"));
}

#[test]
fn test_connection_error_types() {
    let timeout = ConnectionError::Timeout("timeout".to_string());
    let refused = ConnectionError::Refused("refused".to_string());
    let network = ConnectionError::Network("network".to_string());

    // Verify they're different error types
    assert!(std::mem::discriminant(&timeout) != std::mem::discriminant(&refused));
    assert!(std::mem::discriminant(&refused) != std::mem::discriminant(&network));
}

#[test]
fn test_timeout_error_display() {
    let error = ConnectionError::Timeout("Operation timed out after 30s".to_string());
    let error_string = format!("{error}");

    assert!(error_string.contains("timeout"));
    assert!(error_string.contains("30s"));
}

#[test]
fn test_refused_error_display() {
    let error = ConnectionError::Refused("Connection refused by server".to_string());
    let error_string = format!("{error}");

    assert!(error_string.contains("refused"));
}

#[test]
fn test_network_error_display() {
    let error = ConnectionError::Network("Network unreachable".to_string());
    let error_string = format!("{error}");

    assert!(error_string.contains("Network"));
    assert!(error_string.contains("unreachable"));
}

#[test]
fn test_auth_error_display() {
    let error = ConnectionError::Auth("Invalid credentials".to_string());
    let error_string = format!("{error}");

    assert!(error_string.contains("Authentication") || error_string.contains("failed"));
}

#[test]
fn test_too_many_failures_error() {
    let error = ConnectionError::TooManyFailures { consecutive: 5 };
    let error_string = format!("{error}");

    assert!(error_string.contains('5'));
    assert!(error_string.contains("failures"));
}

#[test]
fn test_error_debug_format() {
    let errors = vec![
        ConnectionError::Timeout("test".to_string()),
        ConnectionError::Refused("test".to_string()),
        ConnectionError::Network("test".to_string()),
        ConnectionError::Auth("test".to_string()),
        ConnectionError::TooManyFailures { consecutive: 3 },
    ];

    for error in errors {
        let debug = format!("{error:?}");
        assert!(!debug.is_empty());
    }
}
