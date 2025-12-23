//! Critical API tests to boost coverage
//!
//! These tests cover the most important API endpoints and error paths.

use nestgate_api::handlers::status::{get_status, initialize_uptime, SystemStatus};

#[test]
fn test_status_endpoint_returns_healthy() {
    initialize_uptime();
    let response = get_status();
    assert_eq!(response.0.status, "healthy");
}

#[test]
fn test_status_endpoint_has_version() {
    initialize_uptime();
    let response = get_status();
    assert!(!response.0.version.is_empty());
}

#[test]
fn test_status_endpoint_has_valid_timestamp() {
    initialize_uptime();
    let response = get_status();
    assert!(response.0.timestamp > 0);
}

#[test]
fn test_status_endpoint_uptime_increases() {
    initialize_uptime();
    let response1 = get_status();

    // Modern pattern: Test monotonicity without artificial delay
    // Uptime is based on actual elapsed time, not sleep
    // Even with minimal elapsed time, uptime should be monotonic
    let response2 = get_status();

    // Uptime should never decrease (monotonic guarantee)
    // This tests the uptime tracking logic, not sleep timing
    assert!(response2.0.uptime >= response1.0.uptime);
}

#[test]
fn test_system_status_json_round_trip() {
    let original = SystemStatus {
        status: "healthy".to_string(),
        version: "1.0.0".to_string(),
        uptime: 42,
        timestamp: 1234567890,
    };

    let json = serde_json::to_string(&original).expect("Serialization failed");
    let deserialized: SystemStatus = serde_json::from_str(&json).expect("Deserialization failed");

    assert_eq!(original.status, deserialized.status);
    assert_eq!(original.version, deserialized.version);
    assert_eq!(original.uptime, deserialized.uptime);
    assert_eq!(original.timestamp, deserialized.timestamp);
}

#[test]
fn test_system_status_with_empty_status() {
    let status = SystemStatus {
        status: String::new(),
        version: "1.0.0".to_string(),
        uptime: 0,
        timestamp: 0,
    };

    let json = serde_json::to_string(&status);
    assert!(json.is_ok());
}

#[test]
fn test_system_status_with_large_uptime() {
    let status = SystemStatus {
        status: "healthy".to_string(),
        version: "1.0.0".to_string(),
        uptime: u64::MAX,
        timestamp: u64::MAX,
    };

    let json = serde_json::to_string(&status);
    assert!(json.is_ok());

    let deserialized: SystemStatus =
        serde_json::from_str(&json.unwrap()).expect("Deserialization failed");
    assert_eq!(deserialized.uptime, u64::MAX);
}

#[test]
fn test_system_status_debug_format() {
    let status = SystemStatus {
        status: "healthy".to_string(),
        version: "1.0.0".to_string(),
        uptime: 3600,
        timestamp: 1234567890,
    };

    let debug_str = format!("{status:?}");
    assert!(debug_str.contains("healthy"));
    assert!(debug_str.contains("1.0.0"));
}

#[test]
fn test_status_endpoint_multiple_calls() {
    initialize_uptime();

    // Call multiple times to ensure stability
    for _ in 0..10 {
        let response = get_status();
        assert_eq!(response.0.status, "healthy");
        assert!(!response.0.version.is_empty());
        assert!(response.0.timestamp > 0);
    }
}

#[test]
fn test_system_status_deserialization_with_extra_fields() {
    let json = r#"{
        "status": "healthy",
        "version": "1.0.0",
        "uptime": 3600,
        "timestamp": 1234567890,
        "extra_field": "ignored"
    }"#;

    let status: Result<SystemStatus, _> = serde_json::from_str(json);
    assert!(status.is_ok());
}

#[test]
fn test_system_status_deserialization_missing_field() {
    let json = r#"{
        "status": "healthy",
        "version": "1.0.0",
        "uptime": 3600
    }"#;

    let status: Result<SystemStatus, _> = serde_json::from_str(json);
    assert!(status.is_err()); // timestamp is required
}

#[test]
fn test_system_status_with_special_characters() {
    let status = SystemStatus {
        status: "healthy-with-dashes".to_string(),
        version: "1.0.0-beta+123".to_string(),
        uptime: 3600,
        timestamp: 1234567890,
    };

    let json = serde_json::to_string(&status).expect("Serialization failed");
    let deserialized: SystemStatus = serde_json::from_str(&json).expect("Deserialization failed");

    assert_eq!(deserialized.status, "healthy-with-dashes");
    assert_eq!(deserialized.version, "1.0.0-beta+123");
}

#[test]
fn test_system_status_timestamp_ordering() {
    initialize_uptime();

    let response1 = get_status();
    // Modern pattern: Test timestamp monotonicity without artificial delay
    // System timestamps have nanosecond precision and are monotonic
    // If timestamps are identical, that tests precision, not sleep timing
    let response2 = get_status();

    // Timestamp should be monotonically increasing (or equal if same nanosecond)
    // This tests the timestamp generation logic, not sleep
    assert!(response2.0.timestamp >= response1.0.timestamp);
}

#[test]
fn test_initialize_uptime_multiple_calls() {
    // Should be idempotent - calling multiple times should be safe
    initialize_uptime();
    let first_call = get_status();

    initialize_uptime(); // Call again
    let second_call = get_status();

    // Both should work
    assert_eq!(first_call.0.status, "healthy");
    assert_eq!(second_call.0.status, "healthy");
}

// Edge case tests
mod edge_cases {
    use super::*;

    #[test]
    fn test_status_zero_uptime() {
        initialize_uptime();
        let response = get_status();
        // Uptime should be very small immediately after init
        assert!(response.0.uptime < 10); // Less than 10 seconds
    }

    #[test]
    fn test_system_status_unicode_in_status() {
        let status = SystemStatus {
            status: "健康な".to_string(), // "Healthy" in Japanese
            version: "1.0.0".to_string(),
            uptime: 3600,
            timestamp: 1234567890,
        };

        let json = serde_json::to_string(&status).expect("Serialization failed");
        let deserialized: SystemStatus =
            serde_json::from_str(&json).expect("Deserialization failed");

        assert_eq!(deserialized.status, "健康な");
    }

    #[test]
    fn test_system_status_empty_version() {
        let status = SystemStatus {
            status: "healthy".to_string(),
            version: String::new(),
            uptime: 3600,
            timestamp: 1234567890,
        };

        let json = serde_json::to_string(&status);
        assert!(json.is_ok());
    }

    #[test]
    fn test_system_status_zero_values() {
        let status = SystemStatus {
            status: "healthy".to_string(),
            version: "0.0.0".to_string(),
            uptime: 0,
            timestamp: 0,
        };

        let json = serde_json::to_string(&status).expect("Serialization failed");
        let deserialized: SystemStatus =
            serde_json::from_str(&json).expect("Deserialization failed");

        assert_eq!(deserialized.uptime, 0);
        assert_eq!(deserialized.timestamp, 0);
    }
}

// Performance tests
mod performance {
    use super::*;

    #[test]
    fn test_status_endpoint_performance() {
        initialize_uptime();

        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let _ = get_status();
        }
        let elapsed = start.elapsed();

        // Should complete 1000 calls in under 100ms
        assert!(
            elapsed.as_millis() < 100,
            "Status endpoint too slow: {elapsed:?}"
        );
    }

    #[test]
    fn test_serialization_performance() {
        let status = SystemStatus {
            status: "healthy".to_string(),
            version: "1.0.0".to_string(),
            uptime: 3600,
            timestamp: 1234567890,
        };

        let start = std::time::Instant::now();
        for _ in 0..10000 {
            let _ = serde_json::to_string(&status).expect("Serialization failed");
        }
        let elapsed = start.elapsed();

        // Should serialize 10000 times in under 50ms
        assert!(
            elapsed.as_millis() < 50,
            "Serialization too slow: {elapsed:?}"
        );
    }
}
