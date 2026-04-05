// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Timeout Tests
//!
//! Auto-generated from smart refactoring of client_tests.rs
//! Sections: TIMEOUT TESTS, TIMEOUT COMPREHENSIVE TESTS

use super::super::client::*;
use std::time::Duration;

// ==================== TIMEOUT TESTS ====================
#[test]
fn test_timeout_new() {
    let timeout = TimeoutMs::new(5000);
    assert_eq!(timeout.as_duration(), Duration::from_millis(5000));
}

#[test]
fn test_timeout_as_duration() {
    let timeout = TimeoutMs::new(1000);
    assert_eq!(timeout.as_duration(), Duration::from_secs(1));
}

#[test]
fn test_timeout_zero() {
    let timeout = TimeoutMs::new(0);
    assert_eq!(timeout.as_duration(), Duration::from_millis(0));
}

#[test]
fn test_timeout_large() {
    let timeout = TimeoutMs::new(60000);
    assert_eq!(timeout.as_duration(), Duration::from_secs(60));
}

// ==================== TIMEOUT COMPREHENSIVE TESTS ====================
#[test]
fn test_timeout_various_durations() {
    let timeouts = vec![
        (100, Duration::from_millis(100)),
        (1000, Duration::from_secs(1)),
        (5000, Duration::from_secs(5)),
        (30000, Duration::from_secs(30)),
        (60000, Duration::from_secs(60)),
    ];

    for (ms, expected_duration) in timeouts {
        let timeout = TimeoutMs::new(ms);
        assert_eq!(timeout.as_duration(), expected_duration);
    }
}
