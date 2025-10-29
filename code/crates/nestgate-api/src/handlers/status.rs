use axum::Json;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::info;
// Removed unused tracing import

#[cfg(test)]
#[path = "status_comprehensive_tests.rs"]
mod status_comprehensive_tests;

#[derive(Debug, Serialize, Deserialize)]
/// System status information
pub struct SystemStatus {
    /// Current system status
    pub status: String,
    /// System version
    pub version: String,
    /// System uptime in seconds
    pub uptime: u64,
    /// Current timestamp
    pub timestamp: u64,
}
static START_TIME: std::sync::OnceLock<SystemTime> = std::sync::OnceLock::new();

/// Initialize system uptime tracking
pub fn initialize_uptime() {
    START_TIME.set(SystemTime::now()).ok();
}
/// Get system status handler
pub fn get_status() -> Json<SystemStatus> {
    info!("Status endpoint called");
    let start_time = START_TIME.get().copied().unwrap_or_else(SystemTime::now);
    let uptime = SystemTime::now()
        .duration_since(start_time)
        .unwrap_or_default()
        .as_secs();

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    Json(SystemStatus {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime,
        timestamp,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_status_structure() {
        let status = SystemStatus {
            status: "healthy".to_string(),
            version: "1.0.0".to_string(),
            uptime: 3600,
            timestamp: 1234567890,
        };

        assert_eq!(status.status, "healthy");
        assert_eq!(status.version, "1.0.0");
        assert_eq!(status.uptime, 3600);
        assert_eq!(status.timestamp, 1234567890);
    }

    #[test]
    fn test_system_status_serialization() {
        let status = SystemStatus {
            status: "healthy".to_string(),
            version: "1.0.0".to_string(),
            uptime: 3600,
            timestamp: 1234567890,
        };

        let serialized = serde_json::to_string(&status);
        assert!(serialized.is_ok(), "SystemStatus should serialize");

        let json = serialized.unwrap();
        assert!(json.contains("\"status\":\"healthy\""));
        assert!(json.contains("\"version\":\"1.0.0\""));
        assert!(json.contains("\"uptime\":3600"));
    }

    #[test]
    fn test_system_status_deserialization() {
        let json = r#"{
            "status": "healthy",
            "version": "1.0.0",
            "uptime": 3600,
            "timestamp": 1234567890
        }"#;

        let status: std::result::Result<SystemStatus, _> = serde_json::from_str(json);
        assert!(status.is_ok(), "SystemStatus should deserialize");

        let status = status.unwrap();
        assert_eq!(status.status, "healthy");
        assert_eq!(status.version, "1.0.0");
    }

    #[test]
    fn test_initialize_uptime() {
        // Initialize uptime tracking
        initialize_uptime();

        // START_TIME should be set
        assert!(
            START_TIME.get().is_some(),
            "START_TIME should be initialized"
        );
    }

    #[test]
    fn test_get_status_returns_healthy() {
        // Initialize uptime
        initialize_uptime();

        let status_response = get_status();
        let status = status_response.0;

        assert_eq!(status.status, "healthy");
        assert!(!status.version.is_empty(), "Version should not be empty");
        assert!(status.timestamp > 0, "Timestamp should be positive");
    }
}
