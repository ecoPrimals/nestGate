//! System status endpoint handler

use axum::{
    response::IntoResponse,
    Json,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Status {
    version: &'static str,
    uptime: u64,
}

/// Get system status handler
pub async fn get_status() -> impl IntoResponse {
    let status = Status {
        version: env!("CARGO_PKG_VERSION"),
        uptime: 0, // TODO: Implement uptime tracking
    };
    Json(status)
} 