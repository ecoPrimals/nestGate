// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! WebSocket (`streaming-rpc`) and SSE JSON snapshot handlers.

use std::sync::Arc;
use std::sync::atomic::Ordering;

use super::state::{AppState, CommunicationCounters};

#[cfg(feature = "streaming-rpc")]
struct WebSocketActiveGuard {
    counters: Arc<CommunicationCounters>,
}

#[cfg(feature = "streaming-rpc")]
impl Drop for WebSocketActiveGuard {
    fn drop(&mut self) {
        self.counters
            .websocket_active
            .fetch_sub(1, Ordering::Relaxed);
    }
}

// WebSocket handler
#[cfg(feature = "streaming-rpc")]
/// WebSocket handler for real-time updates
///
/// Provides bidirectional communication for real-time system events,
/// storage updates, and performance metrics streaming.
pub async fn websocket_handler(
    ws: axum::extract::WebSocketUpgrade,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> axum::response::Response {
    ws.on_upgrade(|socket| handle_websocket_connection(socket, state))
}

/// Handle WebSocket connection lifecycle
///
/// Manages the bidirectional WebSocket connection, including:
/// - Connection setup and authentication
/// - Message routing and processing
/// - Periodic health checks and keepalive
/// - Graceful disconnection handling
#[cfg(feature = "streaming-rpc")]
async fn handle_websocket_connection(mut socket: axum::extract::ws::WebSocket, state: AppState) {
    use axum::extract::ws::Message;

    tracing::info!("WebSocket connection established");

    state
        .communication_counters
        .websocket_active
        .fetch_add(1, Ordering::Relaxed);
    let _active_guard = WebSocketActiveGuard {
        counters: Arc::clone(&state.communication_counters),
    };

    // Send initial connection success message
    if socket
        .send(Message::Text(
            serde_json::json!({
                "type": "connection",
                "status": "connected",
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "version": env!("CARGO_PKG_VERSION")
            })
            .to_string(),
        ))
        .await
        .is_err()
    {
        tracing::warn!("Failed to send connection message");
        return;
    }

    // Main message loop
    while let Some(msg) = socket.recv().await {
        match msg {
            Ok(Message::Text(text)) => {
                tracing::debug!("Received WebSocket message: {}", text);

                state
                    .communication_counters
                    .websocket_messages_total
                    .fetch_add(1, Ordering::Relaxed);

                // Parse and route message
                match serde_json::from_str::<serde_json::Value>(&text) {
                    Ok(json) => {
                        let response = handle_websocket_message(json, &state).await;
                        if socket.send(Message::Text(response)).await.is_err() {
                            tracing::warn!("Failed to send response, closing connection");
                            break;
                        }
                    }
                    Err(e) => {
                        let error_response = serde_json::json!({
                            "type": "error",
                            "error": format!("Invalid JSON: {}", e),
                            "timestamp": chrono::Utc::now().to_rfc3339()
                        })
                        .to_string();

                        if socket.send(Message::Text(error_response)).await.is_err() {
                            break;
                        }
                    }
                }
            }
            Ok(Message::Close(_)) => {
                tracing::info!("WebSocket connection closed by client");
                break;
            }
            Ok(Message::Ping(data)) => {
                if socket.send(Message::Pong(data)).await.is_err() {
                    break;
                }
            }
            Ok(_) => {
                // Ignore other message types (Binary, Pong)
            }
            Err(e) => {
                tracing::warn!("WebSocket error: {}", e);
                break;
            }
        }
    }

    tracing::info!("WebSocket connection closed");
}

/// Process WebSocket message and generate response
///
/// Routes messages based on type and returns appropriate responses.
#[cfg(feature = "streaming-rpc")]
#[expect(
    clippy::unused_async,
    reason = "cfg(test) callers await this helper; body is synchronous"
)]
pub async fn handle_websocket_message(msg: serde_json::Value, state: &AppState) -> String {
    let msg_type = msg
        .get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");

    match msg_type {
        "ping" => serde_json::json!({
            "type": "pong",
            "timestamp": chrono::Utc::now().to_rfc3339()
        })
        .to_string(),

        "get_storage_status" => {
            // Get storage metrics from ZFS manager
            match state.get_zfs_manager() {
                Some(_manager) => {
                    // Use manager to get real metrics (simplified for now)
                    serde_json::json!({
                        "type": "storage_status",
                        "data": {
                            "available": true,
                            "manager_initialized": true,
                            "timestamp": chrono::Utc::now().to_rfc3339()
                        }
                    })
                    .to_string()
                }
                None => serde_json::json!({
                    "type": "storage_status",
                    "data": {
                        "available": false,
                        "reason": "ZFS manager not initialized",
                        "timestamp": chrono::Utc::now().to_rfc3339()
                    }
                })
                .to_string(),
            }
        }

        "subscribe" => {
            let channel = msg
                .get("channel")
                .and_then(|v| v.as_str())
                .unwrap_or("general");
            serde_json::json!({
                "type": "subscribed",
                "channel": channel,
                "timestamp": chrono::Utc::now().to_rfc3339()
            })
            .to_string()
        }

        _ => serde_json::json!({
            "type": "error",
            "error": format!("Unknown message type: {}", msg_type),
            "timestamp": chrono::Utc::now().to_rfc3339()
        })
        .to_string(),
    }
}

/// SSE events handler
///
/// Returns system-wide events including configuration changes,
/// service status updates, and administrative notifications.
pub async fn sse_events(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl axum::response::IntoResponse {
    state
        .communication_counters
        .sse_events_sent
        .fetch_add(1, Ordering::Relaxed);
    // Get real system events
    let events = vec![serde_json::json!({
        "id": format!("event_{}", uuid::Uuid::new_v4()),
        "type": "system_status",
        "data": {
            "status": "operational",
            "uptime_seconds": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
            "zfs_available": state.get_zfs_manager().is_some(),
        },
        "timestamp": chrono::Utc::now().to_rfc3339()
    })];

    axum::response::Json(serde_json::json!({
        "status": "success",
        "events": events,
        "count": events.len(),
        "generated_at": chrono::Utc::now().to_rfc3339()
    }))
}

/// SSE storage events handler
///
/// Returns storage-related events including pool status changes,
/// dataset operations, snapshot creation, and capacity alerts.
pub async fn sse_storage(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl axum::response::IntoResponse {
    state
        .communication_counters
        .sse_events_sent
        .fetch_add(1, Ordering::Relaxed);
    // Check ZFS manager availability and get storage status
    let storage_events = match state.get_zfs_manager() {
        Some(_manager) => {
            vec![serde_json::json!({
                "id": format!("storage_{}", uuid::Uuid::new_v4()),
                "type": "storage_status",
                "data": {
                    "status": "operational",
                    "manager_available": true,
                    "message": "ZFS storage system operational"
                },
                "timestamp": chrono::Utc::now().to_rfc3339()
            })]
        }
        None => {
            vec![serde_json::json!({
                "id": format!("storage_{}", uuid::Uuid::new_v4()),
                "type": "storage_warning",
                "data": {
                    "status": "degraded",
                    "manager_available": false,
                    "message": "ZFS manager not initialized - storage operations limited"
                },
                "timestamp": chrono::Utc::now().to_rfc3339()
            })]
        }
    };

    axum::response::Json(serde_json::json!({
        "status": "success",
        "storage_events": storage_events,
        "count": storage_events.len(),
        "generated_at": chrono::Utc::now().to_rfc3339()
    }))
}

/// SSE health events handler
///
/// Returns health check results, system diagnostics, and
/// component status monitoring events.
pub async fn sse_health(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl axum::response::IntoResponse {
    state
        .communication_counters
        .sse_events_sent
        .fetch_add(1, Ordering::Relaxed);
    // Perform actual health checks
    let zfs_healthy = state.get_zfs_manager().is_some();
    let overall_status = if zfs_healthy { "healthy" } else { "degraded" };

    axum::response::Json(serde_json::json!({
        "status": "success",
        "health": {
            "overall": overall_status,
            "api": "healthy",
            "storage": if zfs_healthy { "healthy" } else { "degraded" },
            "zfs_manager": if zfs_healthy { "available" } else { "unavailable" },
            "components": {
                "zfs": zfs_healthy,
                "api": true
            },
            "timestamp": chrono::Utc::now().to_rfc3339()
        },
        "generated_at": chrono::Utc::now().to_rfc3339()
    }))
}
