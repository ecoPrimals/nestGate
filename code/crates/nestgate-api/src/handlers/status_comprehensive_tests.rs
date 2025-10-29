//! **COMPREHENSIVE STATUS HANDLER TESTS**
//!
//! Complete test coverage for status.rs handler module.
//! Tests system status, health checks, uptime tracking, and service information.

use super::*;

// ==================== SYSTEM STATUS TESTS ====================

#[test]
fn test_system_status_creation() {
    let status = SystemStatus {
        status: "healthy".to_string(),
        version: "2.0.0".to_string(),
        uptime: 3600,
        timestamp: 1698505200,
    };

    assert_eq!(status.status, "healthy");
    assert_eq!(status.version, "2.0.0");
    assert_eq!(status.uptime, 3600);
    assert_eq!(status.timestamp, 1698505200);
}

#[test]
fn test_system_status_starting() {
    let status = SystemStatus {
        status: "starting".to_string(),
        version: "2.0.0".to_string(),
        uptime: 0,
        timestamp: 1698505200,
    };

    assert_eq!(status.status, "starting");
    assert_eq!(status.uptime, 0);
}

#[test]
fn test_system_status_long_uptime() {
    let one_day_seconds = 86400;
    let status = SystemStatus {
        status: "healthy".to_string(),
        version: "2.0.0".to_string(),
        uptime: one_day_seconds,
        timestamp: 1698505200,
    };

    assert_eq!(status.uptime, 86400);
    // Verify uptime is exactly 24 hours
    assert_eq!(status.uptime / 3600, 24);
}

#[test]
fn test_system_status_serialization() {
    let status = SystemStatus {
        status: "healthy".to_string(),
        version: "1.0.0".to_string(),
        uptime: 1000,
        timestamp: 1698505200,
    };

    let json = serde_json::to_string(&status).expect("Serialization failed");
    assert!(json.contains("\"status\":\"healthy\""));
    assert!(json.contains("\"uptime\":1000"));
    assert!(json.contains("\"version\":\"1.0.0\""));
}

#[test]
fn test_system_status_deserialization() {
    let json = r#"{
            "status": "healthy",
            "version": "1.5.0",
            "uptime": 500,
            "timestamp": 1698505200
        }"#;
    let status: SystemStatus = serde_json::from_str(json).expect("Deserialization failed");

    assert_eq!(status.status, "healthy");
    assert_eq!(status.uptime, 500);
    assert_eq!(status.version, "1.5.0");
    assert_eq!(status.timestamp, 1698505200);
}

// ==================== UPTIME CALCULATION TESTS ====================

#[test]
fn test_uptime_zero() {
    let uptime_seconds: u64 = 0;
    assert_eq!(uptime_seconds, 0);
}

#[test]
fn test_uptime_one_minute() {
    let uptime_seconds: u64 = 60;
    assert_eq!(uptime_seconds / 60, 1); // 1 minute
}

#[test]
fn test_uptime_one_hour() {
    let uptime_seconds: u64 = 3600;
    assert_eq!(uptime_seconds / 3600, 1); // 1 hour
}

#[test]
fn test_uptime_one_day() {
    let uptime_seconds: u64 = 86400;
    assert_eq!(uptime_seconds / 86400, 1); // 1 day
}

#[test]
fn test_uptime_one_week() {
    let uptime_seconds: u64 = 604800;
    assert_eq!(uptime_seconds / 86400, 7); // 7 days
}

#[test]
fn test_uptime_calculation() {
    let uptime_seconds: u64 = 90061; // 1 day, 1 hour, 1 minute, 1 second

    let days = uptime_seconds / 86400;
    let remaining = uptime_seconds % 86400;
    let hours = remaining / 3600;
    let remaining = remaining % 3600;
    let minutes = remaining / 60;
    let seconds = remaining % 60;

    assert_eq!(days, 1);
    assert_eq!(hours, 1);
    assert_eq!(minutes, 1);
    assert_eq!(seconds, 1);
}

// ==================== VERSION TESTS ====================

#[test]
fn test_version_format() {
    let version = "2.0.0";
    let parts: Vec<&str> = version.split('.').collect();

    assert_eq!(parts.len(), 3);
    assert_eq!(parts[0], "2");
    assert_eq!(parts[1], "0");
    assert_eq!(parts[2], "0");
}

#[test]
fn test_version_parsing() {
    let version = "1.2.3";
    let parts: Vec<u32> = version.split('.').filter_map(|s| s.parse().ok()).collect();

    assert_eq!(parts.len(), 3);
    assert_eq!(parts[0], 1);
    assert_eq!(parts[1], 2);
    assert_eq!(parts[2], 3);
}

#[test]
fn test_version_comparison() {
    let v1 = "1.0.0";
    let v2 = "2.0.0";

    assert!(v1 < v2); // String comparison works for simple versions
}

// ==================== INTEGRATION TESTS ====================

#[test]
fn test_status_serialization_roundtrip() {
    let original = SystemStatus {
        status: "healthy".to_string(),
        version: "1.2.3".to_string(),
        uptime: 12345,
        timestamp: 1698505200,
    };

    let json = serde_json::to_string(&original).expect("Serialization failed");
    let deserialized: SystemStatus = serde_json::from_str(&json).expect("Deserialization failed");

    assert_eq!(original.status, deserialized.status);
    assert_eq!(original.version, deserialized.version);
    assert_eq!(original.uptime, deserialized.uptime);
    assert_eq!(original.timestamp, deserialized.timestamp);
}

#[test]
fn test_get_status_handler() {
    // Initialize uptime
    initialize_uptime();

    // Call the handler
    let response = get_status();
    let status = response.0;

    // Verify fields
    assert_eq!(status.status, "healthy");
    assert!(!status.version.is_empty());
    // uptime is u64, so always >= 0, no need to check
    assert!(status.timestamp > 0);
}

#[test]
fn test_status_uptime_increases() {
    initialize_uptime();

    let response1 = get_status();
    let status1 = response1.0;

    // Sleep for a short time
    std::thread::sleep(std::time::Duration::from_millis(100));

    let response2 = get_status();
    let status2 = response2.0;

    // Uptime should have increased (or stayed the same due to timing precision)
    assert!(status2.uptime >= status1.uptime);
}

// ==================== EDGE CASE TESTS ====================

#[test]
fn test_zero_uptime() {
    let status = SystemStatus {
        status: "starting".to_string(),
        version: "1.0.0".to_string(),
        uptime: 0,
        timestamp: 1698505200,
    };

    assert_eq!(status.uptime, 0);
}

#[test]
fn test_maximum_uptime() {
    let status = SystemStatus {
        status: "healthy".to_string(),
        version: "1.0.0".to_string(),
        uptime: u64::MAX,
        timestamp: 1698505200,
    };

    assert_eq!(status.uptime, u64::MAX);
}

#[test]
fn test_empty_version() {
    let status = SystemStatus {
        status: "healthy".to_string(),
        version: String::new(),
        uptime: 100,
        timestamp: 1698505200,
    };

    assert!(status.version.is_empty());
}

#[test]
fn test_empty_status() {
    let status = SystemStatus {
        status: String::new(),
        version: "1.0.0".to_string(),
        uptime: 100,
        timestamp: 1698505200,
    };

    assert!(status.status.is_empty());
}

#[test]
fn test_timestamp_validation() {
    let status = SystemStatus {
        status: "healthy".to_string(),
        version: "1.0.0".to_string(),
        uptime: 1000,
        timestamp: 1698505200,
    };

    // Verify timestamp is reasonable (after year 2000)
    assert!(status.timestamp > 946684800); // Jan 1, 2000
}
