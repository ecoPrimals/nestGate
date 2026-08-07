// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Event-driven JSON-RPC keep-alive loop for the isomorphic IPC server.
//!
//! Extracted from the server module (G68 file-size discipline) so that
//! protocol-level concerns live in their own compilation unit.

use bytes::Bytes;
use serde_json::Value;
use std::sync::Arc;
use tracing::{debug, error, warn};

use super::super::tcp_fallback::RpcHandler;

/// Event-driven JSON-RPC keep-alive loop.
///
/// Uses `tokio::select!` to multiplex between I/O readiness and a
/// resettable idle timer rather than wrapping reads in a brute-force
/// timeout. On idle expiry the client receives a `connection.closing`
/// JSON-RPC notification before the socket is torn down, giving it the
/// opportunity to reconnect or flush pending work.
///
/// When `btsp_authenticated` is `false` (BTSP required but the client
/// sent plain JSON-RPC), only BTSP-exempt methods (health, identity,
/// capabilities) are dispatched; all others receive error -32604.
pub async fn json_rpc_keep_alive_loop<R, W>(
    reader: &mut R,
    writer: &mut W,
    handler: &Arc<dyn RpcHandler>,
    btsp_authenticated: bool,
) -> anyhow::Result<()>
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
                        idle_timer
                            .as_mut()
                            .reset(tokio::time::Instant::now() + CONNECTION_IDLE_LIMIT);

                        let trimmed = line.as_slice().trim_ascii();
                        if trimmed.is_empty() {
                            continue;
                        }

                        requests_served += 1;

                        let response = match serde_json::from_slice::<Value>(trimmed) {
                            Ok(request) => {
                                if btsp_authenticated {
                                    handler.handle_request(request).await
                                } else {
                                    dispatch_or_reject_unauth(request, handler)
                                        .await
                                }
                            }
                            Err(e) => {
                                warn!("Invalid JSON-RPC request: {}", e);
                                {
                                    use nestgate_types::JsonRpcErrorCode;
                                    serde_json::json!({
                                        "jsonrpc": "2.0",
                                        "error": {
                                            "code": JsonRpcErrorCode::ParseError.code(),
                                            "message": JsonRpcErrorCode::ParseError.default_message(),
                                            "data": { "error": e.to_string() }
                                        },
                                        "id": null
                                    })
                                }
                            }
                        };
                        let response_bytes: Bytes =
                            serde_json::to_vec(&response).map(Bytes::from)?;
                        writer.write_all(&response_bytes).await?;
                        writer.write_all(b"\n").await?;
                        writer.flush().await?;
                    }
                    Err(e) => {
                        error!("Unix socket read error: {}", e);
                        break;
                    }
                }
            }
            () = &mut idle_timer => {
                debug!(
                    requests_served,
                    idle_secs = CONNECTION_IDLE_LIMIT.as_secs(),
                    "Connection idle — sending close notification"
                );
                let notification = serde_json::json!({
                    "jsonrpc": "2.0",
                    "method": "connection.closing",
                    "params": {
                        "reason": "idle",
                        "idle_timeout_secs": CONNECTION_IDLE_LIMIT.as_secs(),
                        "requests_served": requests_served
                    }
                });
                if let Ok(bytes) = serde_json::to_vec(&notification) {
                    let _ = writer.write_all(&bytes).await;
                    let _ = writer.write_all(b"\n").await;
                    let _ = writer.flush().await;
                }
                break;
            }
        }
    }

    debug!(requests_served, "Connection closed");
    Ok(())
}

const CONNECTION_IDLE_LIMIT: std::time::Duration = crate::rpc::protocol::CONNECTION_IDLE_LIMIT;

/// Dispatch a request on an unauthenticated (BTSP-bypassed) connection.
///
/// Only BTSP-exempt methods are forwarded to the handler; everything else
/// gets a `-32604 BTSP authentication required` error.
pub async fn dispatch_or_reject_unauth(
    request: Value,
    handler: &Arc<dyn RpcHandler>,
) -> Value {
    let method_raw = request.get("method").and_then(Value::as_str).unwrap_or("");
    let method = crate::rpc::protocol::normalize_method(method_raw);
    if crate::rpc::is_btsp_exempt_method(&method) {
        return handler.handle_request(request).await;
    }
    let id = request.get("id").cloned().unwrap_or(Value::Null);
    warn!(
        method = method_raw,
        "Rejecting unauthenticated call to BTSP-gated method"
    );
    {
        use nestgate_types::JsonRpcErrorCode;
        serde_json::json!({
            "jsonrpc": "2.0",
            "error": {
                "code": JsonRpcErrorCode::AuthRequired.code(),
                "message": JsonRpcErrorCode::AuthRequired.default_message(),
                "data": { "method": method_raw }
            },
            "id": id
        })
    }
}
