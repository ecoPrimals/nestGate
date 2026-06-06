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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::Ordering;

    #[tokio::test]
    async fn health_check_returns_ok_with_version() {
        let axum::response::Json(body) = health_check().await;
        assert_eq!(body["status"], "ok");
        assert_eq!(body["service"], "nestgate-api");
        assert!(body["version"].is_string());
        assert_eq!(body["communication_layers"]["websocket"], true);
    }

    #[tokio::test]
    async fn get_communication_stats_returns_zero_counters() {
        let state = AppState::new();
        let axum::response::Json(body) =
            get_communication_stats(axum::extract::State(state)).await;
        assert_eq!(body["websocket"]["active_connections"], 0);
        assert_eq!(body["total_active_connections"], 0);
        assert_eq!(body["total_messages_processed"], 0);
    }

    #[tokio::test]
    async fn get_communication_stats_reflects_incremented_counters() {
        let state = AppState::new();
        state
            .communication_counters
            .websocket_active
            .store(3, Ordering::Relaxed);
        state
            .communication_counters
            .websocket_messages_total
            .store(42, Ordering::Relaxed);
        let axum::response::Json(body) =
            get_communication_stats(axum::extract::State(state)).await;
        assert_eq!(body["websocket"]["active_connections"], 3);
        assert_eq!(body["websocket"]["total_messages"], 42);
        assert_eq!(body["total_active_connections"], 3);
        assert_eq!(body["total_messages_processed"], 42);
    }

    #[tokio::test]
    async fn get_events_returns_empty_array_initially() {
        let state = AppState::new();
        let axum::response::Json(body) = get_events(axum::extract::State(state)).await;
        assert!(body["events"].as_array().unwrap().is_empty());
        assert_eq!(body["total_events"], 0);
    }

    #[tokio::test]
    async fn get_events_returns_pushed_events() {
        let state = AppState::new();
        state
            .event_log
            .write()
            .await
            .push(serde_json::json!({"type": "test", "msg": "hello"}));
        let axum::response::Json(body) = get_events(axum::extract::State(state)).await;
        assert_eq!(body["total_events"], 1);
        assert_eq!(body["events"][0]["type"], "test");
    }
}
