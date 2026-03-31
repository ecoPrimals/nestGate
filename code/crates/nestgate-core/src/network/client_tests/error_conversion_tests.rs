// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Error Conversion Tests
//!
//! Auto-generated from smart refactoring of client_tests.rs
//! Sections: ERROR CONVERSION TESTS

use super::super::client::*;
use crate::error::NestGateError;

// ==================== ERROR CONVERSION TESTS ====================
#[test]
fn test_http_client_error_to_nestgate_error_conversion() {
    let http_error = HttpClientError::ConnectionFailed {
        message: "test".to_string(),
    };

    let nestgate_error: NestGateError = http_error.into();
    assert!(format!("{nestgate_error:?}").contains("test"));
}
