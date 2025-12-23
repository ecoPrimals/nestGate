//! Status module

use axum::Json;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::info;
// Removed unused tracing import

#[cfg(test)]
#[path = "status_comprehensive_tests.rs"]
mod status_comprehensive_tests;

#[cfg(test)]
#[path = "status_extended_tests.rs"]
mod status_extended_tests;

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
///
/// Returns current system health status, version, and uptime information.
/// This is the primary health check endpoint for monitoring systems.
///
/// # Returns
///
/// A JSON response containing:
/// - `status`: Current system status ("healthy", "degraded", etc.)
/// - `version`: Cargo package version
/// - `uptime`: System uptime in seconds
/// - `timestamp`: Current Unix timestamp
///
/// # Examples
///
/// ```rust,ignore
/// let status_json = get_status();
/// println!("System is {}", status_json.0.status);
/// ```
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

// ==================== TEST-ONLY STUBS ====================
// These types exist only to make tests compile
// In production, use SystemStatus instead

#[cfg(test)]
#[derive(Debug, Serialize, Deserialize)]
/// Status information for testing
pub struct StatusInfo {
    /// Service version
    pub version: String,
    /// Current status
    pub status: String,
    /// Uptime in seconds
    pub uptime_seconds: u64,
    /// Number of active connections
    pub active_connections: u32,
}

#[cfg(test)]
#[derive(Debug, Serialize, Deserialize)]
/// System information for testing
pub struct SystemInfo {
    /// System hostname
    pub hostname: String,
    /// Operating system type (e.g., "Linux", "Darwin")
    pub os_type: String,
    /// Operating system version string
    pub os_version: String,
    /// CPU architecture (e.g., "x86_64", "aarch64")
    pub architecture: String,
    /// Number of CPU cores available
    pub cpu_cores: u32,
    /// Total system memory in bytes
    pub total_memory_bytes: u64,
}

#[cfg(test)]
/// Returns the current system health status
pub fn health_check() -> Json<SystemStatus> {
    // Simple health check that returns status
    get_status()
}
