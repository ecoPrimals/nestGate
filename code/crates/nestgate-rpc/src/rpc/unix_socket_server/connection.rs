// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Connection lifecycle for Unix socket JSON-RPC sessions.
//!
//! Handles:
//! - BTSP handshake detection (binary peek, phase 2 → optional phase 3)
//! - Plaintext JSON-RPC keep-alive loop with idle-timeout reaping
//! - Encrypted frame loop delegation via [`crate::rpc::btsp_phase3::transport`]

use bytes::Bytes;
use serde_json::{Value, json};
use std::borrow::Cow;
use std::sync::Arc;
use tokio::io::BufReader;
use tokio::net::UnixStream;
use tracing::{debug, error, info, warn};

use super::dispatch::handle_request;
use super::{JsonRpcError, JsonRpcRequest, JsonRpcResponse, StorageState};
use nestgate_types::error::{NestGateError, Result};

/// Maximum idle time before a keep-alive connection is closed. The timer
/// resets on every successful request so active connections are never reaped.
const CONNECTION_IDLE_LIMIT: std::time::Duration = std::time::Duration::from_secs(300);

/// Handle a single Unix socket connection.
///
/// When BTSP is required (production), runs the 4-step handshake before
/// entering the JSON-RPC dispatch loop. Development connections proceed
/// directly.
pub(super) async fn handle_connection(stream: UnixStream, state: Arc<StorageState>) -> Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut raw_reader = BufReader::new(reader);

    if crate::rpc::btsp_server_handshake::is_btsp_required() {
        use tokio::io::AsyncBufReadExt;
        let is_json_rpc = match raw_reader.fill_buf().await {
            Ok(buf) if !buf.is_empty() => buf[0] == b'{',
            _ => false,
        };

        if is_json_rpc {
            tracing::debug!("BTSP: first byte is '{{', bypassing handshake (restricted)");
            return json_rpc_loop(&mut raw_reader, &mut writer, &state, false).await;
        }

        let family_id = state.family_id.as_deref().unwrap_or("default").to_string();

        let _session = crate::rpc::btsp_server_handshake::perform_handshake(
            &mut raw_reader,
            &mut writer,
            &family_id,
        )
        .await?;

        return post_handshake_phase3_or_plaintext(&mut raw_reader, &mut writer, &state).await;
    }

    json_rpc_loop(&mut raw_reader, &mut writer, &state, true).await
}

/// After a successful BTSP Phase 2 handshake, check whether the client
/// wants to negotiate Phase 3 (encrypted channel). If the first message is
/// `btsp.negotiate` and keys are derived, switch to the encrypted frame
/// loop; otherwise fall back to the plaintext JSON-RPC loop.
async fn post_handshake_phase3_or_plaintext<R, W>(
    reader: &mut R,
    writer: &mut W,
    state: &StorageState,
) -> Result<()>
where
    R: tokio::io::AsyncBufReadExt + Unpin,
    W: tokio::io::AsyncWriteExt + Unpin,
{
    let mut first_line = Vec::new();
    let n = reader
        .read_until(b'\n', &mut first_line)
        .await
        .map_err(|e| {
            NestGateError::io_error(format!("Failed to read first post-handshake message: {e}"))
        })?;

    if n == 0 {
        return Ok(());
    }

    let trimmed = first_line.as_slice().trim_ascii();
    if trimmed.is_empty() {
        return json_rpc_loop(reader, writer, state, true).await;
    }

    let parsed: Value = match serde_json::from_slice(trimmed) {
        Ok(v) => v,
        Err(_) => return json_rpc_loop(reader, writer, state, true).await,
    };

    let is_negotiate = parsed
        .get("method")
        .and_then(Value::as_str)
        .is_some_and(|m| m == "btsp.negotiate");

    if !is_negotiate {
        let response = dispatch_parsed_request(&parsed, state).await;
        let response_bytes = serde_json::to_vec(&response)
            .map_err(|e| NestGateError::api(format!("Failed to serialize response: {e}")))?;
        writer
            .write_all(&response_bytes)
            .await
            .map_err(|e| NestGateError::io_error(format!("Failed to write response: {e}")))?;
        writer
            .write_all(b"\n")
            .await
            .map_err(|e| NestGateError::io_error(format!("Failed to write newline: {e}")))?;
        writer
            .flush()
            .await
            .map_err(|e| NestGateError::io_error(format!("Failed to flush: {e}")))?;
        return json_rpc_loop(reader, writer, state, true).await;
    }

    let keys =
        crate::rpc::btsp_phase3::transport::try_phase3_negotiate(&parsed, writer, true).await?;

    let Some(session_keys) = keys else {
        return json_rpc_loop(reader, writer, state, true).await;
    };

    info!("BTSP Phase 3: encrypted channel established, entering encrypted frame loop");

    let state_arc = Arc::new(state.clone());
    crate::rpc::btsp_phase3::transport::run_encrypted_frame_loop(
        reader,
        writer,
        &session_keys,
        |request| {
            let st = Arc::clone(&state_arc);
            async move { dispatch_value_request(request, &st).await }
        },
    )
    .await?;

    Ok(())
}

/// Dispatch a pre-parsed JSON-RPC Value through the handler, returning a Value response.
async fn dispatch_value_request(request: Value, state: &StorageState) -> Value {
    match serde_json::from_value::<JsonRpcRequest>(request) {
        Ok(req) => {
            let resp = handle_request(req, state).await;
            serde_json::to_value(resp).unwrap_or_else(|_| {
                json!({"jsonrpc": "2.0", "error": {"code": -32603, "message": "Internal error"}, "id": null})
            })
        }
        Err(e) => json!({
            "jsonrpc": "2.0",
            "error": {"code": -32700, "message": "Parse error", "data": {"error": e.to_string()}},
            "id": null
        }),
    }
}

/// Dispatch a pre-parsed JSON-RPC Value through the handler (alias for non-negotiate first message).
async fn dispatch_parsed_request(request: &Value, state: &StorageState) -> Value {
    match serde_json::from_value::<JsonRpcRequest>(request.clone()) {
        Ok(req) => {
            let resp = handle_request(req, state).await;
            serde_json::to_value(resp).unwrap_or_else(|_| {
                json!({"jsonrpc": "2.0", "error": {"code": -32603, "message": "Internal error"}, "id": null})
            })
        }
        Err(e) => json!({
            "jsonrpc": "2.0",
            "error": {"code": -32700, "message": "Parse error", "data": {"error": e.to_string()}},
            "id": null
        }),
    }
}

/// Dispatch on an unauthenticated (BTSP-bypassed) connection.
///
/// Only BTSP-exempt methods are forwarded to the handler; everything else
/// gets a `-32604 BTSP authentication required` error.
async fn dispatch_or_reject_unauth(
    request: JsonRpcRequest,
    state: &StorageState,
) -> JsonRpcResponse {
    let method = crate::rpc::protocol::normalize_method(&request.method);
    if crate::rpc::is_btsp_exempt_method(&method) {
        return handle_request(request, state).await;
    }
    warn!(
        method = request.method.as_ref(),
        "Rejecting unauthenticated call to BTSP-gated method"
    );
    JsonRpcResponse {
        jsonrpc: Arc::from("2.0"),
        result: None,
        error: Some(JsonRpcError {
            code: -32604,
            message: Cow::Borrowed("BTSP authentication required"),
            data: Some(json!({"method": request.method})),
        }),
        id: request.id,
    }
}

/// Event-driven JSON-RPC keep-alive loop.
///
/// Uses `tokio::select!` to multiplex between I/O readiness and a resettable
/// idle timer. On idle expiry the client receives a `connection.closing`
/// notification before the socket is torn down.
///
/// When `btsp_authenticated` is `false` (BTSP required but the client sent
/// plain JSON-RPC), only BTSP-exempt methods are dispatched; all others
/// receive error -32604.
async fn json_rpc_loop<R, W>(
    reader: &mut R,
    writer: &mut W,
    state: &StorageState,
    btsp_authenticated: bool,
) -> Result<()>
where
    R: tokio::io::AsyncBufReadExt + Unpin,
    W: tokio::io::AsyncWriteExt + Unpin,
{
    let mut line = Vec::new();
    let mut requests_served: u64 = 0;

    let idle_timer = tokio::time::sleep(CONNECTION_IDLE_LIMIT);
    tokio::pin!(idle_timer);

    loop {
        line.clear();

        tokio::select! {
            result = reader.read_until(b'\n', &mut line) => {
                match result {
                    Ok(0) => break,
                    Ok(_) => {
                        idle_timer.as_mut().reset(
                            tokio::time::Instant::now() + CONNECTION_IDLE_LIMIT,
                        );

                        let trimmed = line.as_slice().trim_ascii();
                        if trimmed.is_empty() {
                            continue;
                        }

                        requests_served += 1;
                        debug!("Received request: {}", String::from_utf8_lossy(trimmed));

                        let response = match serde_json::from_slice::<JsonRpcRequest>(trimmed) {
                            Ok(request) => {
                                if btsp_authenticated {
                                    handle_request(request, state).await
                                } else {
                                    dispatch_or_reject_unauth(request, state).await
                                }
                            }
                            Err(e) => {
                                error!("Failed to parse JSON-RPC request: {}", e);
                                JsonRpcResponse {
                                    jsonrpc: Arc::from("2.0"),
                                    result: None,
                                    error: Some(JsonRpcError {
                                        code: -32700,
                                        message: Cow::Borrowed("Parse error"),
                                        data: Some(json!({"error": e.to_string()})),
                                    }),
                                    id: None,
                                }
                            }
                        };

                        let response_bytes: Bytes = serde_json::to_vec(&response)
                            .map(Bytes::from)
                            .map_err(|e| {
                                NestGateError::api(format!("Failed to serialize response: {e}"))
                            })?;

                        writer
                            .write_all(&response_bytes)
                            .await
                            .map_err(|e| {
                                NestGateError::io_error(format!("Failed to write response: {e}"))
                            })?;
                        writer.write_all(b"\n").await.map_err(|e| {
                            NestGateError::io_error(format!("Failed to write newline: {e}"))
                        })?;
                        writer.flush().await.map_err(|e| {
                            NestGateError::io_error(format!("Failed to flush response: {e}"))
                        })?;

                        debug!("Sent response ({} bytes)", response_bytes.len());
                    }
                    Err(e) => {
                        return Err(NestGateError::io_error(format!(
                            "Failed to read request: {e}"
                        )));
                    }
                }
            }
            () = &mut idle_timer => {
                debug!(
                    requests_served,
                    idle_secs = CONNECTION_IDLE_LIMIT.as_secs(),
                    "Connection idle — sending close notification"
                );
                let notification = json!({
                    "jsonrpc": "2.0",
                    "method": "connection.closing",
                    "params": {
                        "reason": "idle",
                        "idle_timeout_secs": CONNECTION_IDLE_LIMIT.as_secs(),
                        "requests_served": requests_served
                    }
                });
                let bytes: Bytes = serde_json::to_vec(&notification)
                    .map(Bytes::from)
                    .unwrap_or_default();
                let _ = writer.write_all(&bytes).await;
                let _ = writer.write_all(b"\n").await;
                let _ = writer.flush().await;
                break;
            }
        }
    }

    debug!(requests_served, "Connection closed");
    Ok(())
}
