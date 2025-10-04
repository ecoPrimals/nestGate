
use axum::Json;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::info;
// Removed unused tracing import

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
