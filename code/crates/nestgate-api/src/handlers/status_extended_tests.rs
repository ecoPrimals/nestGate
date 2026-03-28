// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Extended tests for status handler
//!
//! Additional comprehensive tests for system status reporting.

use super::*;

#[test]
fn test_system_status_default_values() {
    initialize_uptime();
    let status = get_status();

    assert_eq!(status.0.status, "healthy");
    assert!(!status.0.version.is_empty());
    assert!(status.0.timestamp > 0);
}

#[test]
fn test_system_status_uptime_increases() {
    initialize_uptime();

    let status1 = get_status();
    // Modern pattern: Test monotonicity without artificial delay
    // Uptime is based on actual elapsed time since initialization
    let status2 = get_status();

    // Second uptime should be >= first uptime (monotonic guarantee)
    assert!(status2.0.uptime >= status1.0.uptime);
}

#[test]
fn test_system_status_timestamp_increases() {
    let status1 = get_status();
    // Modern pattern: Test timestamp monotonicity without sleep
    // System time has nanosecond precision
    let status2 = get_status();

    // Second timestamp should be >= first timestamp (monotonic guarantee)
    assert!(status2.0.timestamp >= status1.0.timestamp);
}

#[test]
fn test_system_status_version_format() {
    let status = get_status();
    let version = status.0.version;

    // Version should contain at least one dot (semantic versioning)
    assert!(version.contains('.'), "Version should follow semver format");
}

#[test]
fn test_system_status_json_structure() {
    let status = get_status();
    let json = serde_json::to_value(&status.0).expect("Should serialize");

    assert!(json.get("status").is_some());
    assert!(json.get("version").is_some());
    assert!(json.get("uptime").is_some());
    assert!(json.get("timestamp").is_some());
}

#[test]
fn test_system_status_multiple_calls() {
    // Multiple calls should work without errors
    for _ in 0..10 {
        let status = get_status();
        assert_eq!(status.0.status, "healthy");
    }
}

#[test]
fn test_system_status_concurrent() {
    use std::thread;

    let handles: Vec<_> = (0..5)
        .map(|_| {
            thread::spawn(|| {
                let status = get_status();
                assert_eq!(status.0.status, "healthy");
                assert!(!status.0.version.is_empty());
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("Thread should complete successfully");
    }
}

#[tokio::test]
async fn test_system_status_async_context() {
    let status = get_status();
    assert_eq!(status.0.status, "healthy");
}

#[test]
fn test_system_status_serialization_roundtrip() {
    let original = SystemStatus {
        status: "healthy".to_string(),
        version: "1.0.0".to_string(),
        uptime: 3600,
        timestamp: 1234567890,
    };

    let json = serde_json::to_string(&original).expect("Should serialize");
    let deserialized: SystemStatus = serde_json::from_str(&json).expect("Should deserialize");

    assert_eq!(original.status, deserialized.status);
    assert_eq!(original.version, deserialized.version);
    assert_eq!(original.uptime, deserialized.uptime);
    assert_eq!(original.timestamp, deserialized.timestamp);
}

#[test]
fn test_initialize_uptime_idempotent() {
    // Multiple initializations should be safe
    initialize_uptime();
    initialize_uptime();
    initialize_uptime();

    let status = get_status();
    assert_eq!(status.0.status, "healthy");
}

#[test]
fn test_system_status_timestamp_unix_epoch() {
    let status = get_status();

    // Timestamp should be after year 2020 (approx 1577836800)
    assert!(status.0.timestamp > 1577836800);
}
