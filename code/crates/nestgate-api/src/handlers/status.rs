//! System status endpoint handler

use axum::{
    response::IntoResponse,
    Json,
    extract::State,
};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::info;

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemStatus {
    pub status: String,
    pub version: String,
    pub uptime: u64,
    pub timestamp: u64,
}

static START_TIME: std::sync::OnceLock<SystemTime> = std::sync::OnceLock::new();

pub fn initialize_uptime() {
    START_TIME.set(SystemTime::now()).ok();
}

/// Get system status handler
pub async fn get_status(State(_state): State<AppState>) -> Json<SystemStatus> {
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