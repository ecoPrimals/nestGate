// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Timeout Edge Cases Tests
//!
//! Auto-generated from smart refactoring of client_tests.rs
//! Sections: TIMEOUT EDGE CASES

use super::super::client::*;

// ==================== TIMEOUT EDGE CASES ====================
#[test]
fn test_timeout_very_large() {
    let timeout = TimeoutMs::new(u64::MAX);
    assert!(timeout.as_duration().as_millis() > 0);
}

#[test]
fn test_timeout_millisecond_precision() {
    let timeout1 = TimeoutMs::new(1);
    let timeout2 = TimeoutMs::new(2);

    assert!(timeout2 > timeout1);
    assert_eq!(timeout2.as_millis() - timeout1.as_millis(), 1);
}
