// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Session and auth-mode status endpoints.

use axum::{http::StatusCode, response::Json};
use tracing::info;

/// **GET AUTH STATUS HANDLER**
///
/// Get current authentication status.
///
/// # Errors
///
/// This function currently always returns `Ok`, but returns `Result` for future error handling.
pub async fn get_auth_status() -> std::result::Result<Json<serde_json::Value>, StatusCode> {
    info!("Fetching auth status");

    Ok(Json(serde_json::json!({
        "authenticated": true,
        "mode": "production",
        "methods": ["password", "api_key", "session"],
    })))
}
