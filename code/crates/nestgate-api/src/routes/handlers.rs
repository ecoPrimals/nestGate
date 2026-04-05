// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Lightweight HTTP handlers: health, communication stats, operational events.

use super::state::AppState;

/// Health Check
pub async fn health_check() -> axum::response::Json<serde_json::Value> {
    axum::response::Json(serde_json::json!({
        "status": "ok",
        "service": "nestgate-api",
        "version": env!("CARGO_PKG_VERSION"),
        "communication_layers": {
            "websocket": true,
            "sse": true,
            "streaming_rpc": true,
            "mcp_streaming": true,
            "event_coordination": true
        }
    }))
}

/// GET `/api/v1/communication/stats` — current communication layer counters from [`AppState::communication_counters`].
///
/// Reflects live WebSocket traffic when `streaming-rpc` is enabled and clients connect; SSE JSON
/// snapshot routes increment `sse.events_sent`. Layers that are not instrumented remain at zero.
pub async fn get_communication_stats(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> axum::response::Json<serde_json::Value> {
    axum::response::Json(state.communication_counters.to_json_snapshot())
}

/// GET `/api/v1/events` — operational events stored in [`AppState::event_log`].
///
/// Returns an empty `events` array when nothing has been recorded; entries are never synthesized.
pub async fn get_events(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> axum::response::Json<serde_json::Value> {
    let events = state.event_log.read().await.clone();
    let total_events = events.len();
    axum::response::Json(serde_json::json!({
        "events": events,
        "total_events": total_events,
    }))
}
