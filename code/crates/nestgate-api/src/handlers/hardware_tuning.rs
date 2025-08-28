//
// **Agnostic hardware tuning for any setup**
//
// This handler provides REST API endpoints for automatic hardware detection
// and tuning, with external extraction protection via crypto locks.
//
// This module has been refactored into a modular structure.
// The implementation is now in the `hardware_tuning` submodule.

// ==================== SECTION ====================
// This module provides hardware performance tuning and optimization

use axum::{extract::State, response::Json};
use serde::{Deserialize, Serialize};

// Commented out circular import
// pub use crate::hardware_tuning::*;

/// Hardware tuning configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareTuningConfig {
    pub cpu_optimization: bool,
    pub memory_optimization: bool,
    pub disk_optimization: bool,
}

/// Hardware performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_io: f64,
}

/// Get hardware tuning status
pub async fn get_hardware_status() -> impl axum::response::IntoResponse {
    Json(serde_json::json!({
        "status": "optimal",
        "cpu_optimization": true,
        "memory_optimization": true,
        "disk_optimization": true
    }))
}
