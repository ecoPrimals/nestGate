// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! BTSP Phase 3 transport — negotiate handler and encrypted frame loops.
//!
//! These functions are transport-agnostic (generic over `AsyncRead` /
//! `AsyncWrite`) and shared by both the legacy Unix socket server and
//! the isomorphic IPC server.

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use nestgate_types::error::{NestGateError, Result};
use serde_json::Value;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::{debug, info, warn};

use super::{
    NegotiateParams, NegotiateResult, Phase3Cipher, SessionKeys, derive_handshake_key,
    generate_server_nonce, select_cipher,
};

const MAX_FRAME_SIZE: u32 = 16 * 1024 * 1024; // 16 MiB per BTSP spec

/// Read a single length-prefixed frame from an async reader.
///
/// Frame format: `[4-byte BE u32 length][payload]`.
///
/// # Errors
///
/// Returns an error on I/O failure or if the frame exceeds 16 MiB.
pub async fn read_frame<R: AsyncReadExt + Unpin>(reader: &mut R) -> Result<Vec<u8>> {
    let mut len_buf = [0u8; 4];
    reader.read_exact(&mut len_buf).await.map_err(|e| {
        NestGateError::io_error(format!("BTSP Phase 3: failed to read frame length: {e}"))
    })?;
    let len = u32::from_be_bytes(len_buf);
    if len > MAX_FRAME_SIZE {
        return Err(NestGateError::validation_error(format!(
            "BTSP Phase 3: frame too large ({len} > {MAX_FRAME_SIZE})"
        )));
    }
    let buf_len = len as usize;
    let mut buf = vec![0u8; buf_len];
    reader.read_exact(&mut buf).await.map_err(|e| {
        NestGateError::io_error(format!("BTSP Phase 3: failed to read frame payload: {e}"))
    })?;
    Ok(buf)
}

/// Write a single length-prefixed frame to an async writer.
///
/// # Errors
///
/// Returns an error on I/O failure.
pub async fn write_frame<W: AsyncWriteExt + Unpin>(writer: &mut W, payload: &[u8]) -> Result<()> {
    let len = u32::try_from(payload.len()).map_err(|_| {
        NestGateError::validation_error("BTSP Phase 3: frame payload exceeds u32::MAX")
    })?;
    writer.write_all(&len.to_be_bytes()).await.map_err(|e| {
        NestGateError::io_error(format!("BTSP Phase 3: failed to write frame length: {e}"))
    })?;
    writer.write_all(payload).await.map_err(|e| {
        NestGateError::io_error(format!("BTSP Phase 3: failed to write frame payload: {e}"))
    })?;
    writer
        .flush()
        .await
        .map_err(|e| NestGateError::io_error(format!("BTSP Phase 3: flush failed: {e}")))?;
    Ok(())
}

/// Attempt Phase 3 cipher negotiation on the first post-handshake request.
///
/// If the first message is `btsp.negotiate` and a handshake key can be derived,
/// returns `Some(keys)` after sending the negotiate response. The caller should
/// then enter the encrypted frame loop.
///
/// If the first message is not `btsp.negotiate` or if encryption cannot be
/// established, returns `None` and the caller should fall through to plaintext.
///
/// The `first_message` is passed as a pre-parsed JSON `Value` because the
/// caller already read and parsed it to detect the method.
///
/// When `use_jsonline` is true, the negotiate response is written as
/// newline-delimited JSON; otherwise as a length-prefixed frame.
///
/// # Errors
///
/// Returns an error on I/O or serialization failure.
pub async fn try_phase3_negotiate<W: AsyncWriteExt + Unpin>(
    first_message: &Value,
    writer: &mut W,
    use_jsonline: bool,
) -> Result<Option<SessionKeys>> {
    let method = first_message
        .get("method")
        .and_then(Value::as_str)
        .unwrap_or("");

    if method != "btsp.negotiate" {
        return Ok(None);
    }

    let request_id = first_message.get("id").cloned().unwrap_or(Value::Null);

    let Some(p) = first_message.get("params").cloned() else {
        let err = serde_json::json!({
            "jsonrpc": "2.0",
            "error": {"code": -32602, "message": "Invalid params: btsp.negotiate requires params"},
            "id": request_id
        });
        write_negotiate_response(writer, &err, use_jsonline).await?;
        return Ok(None);
    };
    let params: NegotiateParams = serde_json::from_value(p).map_err(|e| {
        NestGateError::validation_error(format!("Invalid btsp.negotiate params: {e}"))
    })?;

    let offered = params.offered_ciphers();
    let selected = select_cipher(&offered);

    let Ok(family_seed) = crate::rpc::btsp_server_handshake::resolve_family_seed() else {
        debug!("BTSP Phase 3: no family seed — responding with null cipher");
        let result = NegotiateResult {
            cipher: Phase3Cipher::Null.wire_name().to_owned(),
            server_nonce: String::new(),
        };
        let resp = serde_json::json!({"jsonrpc": "2.0", "result": result, "id": request_id});
        write_negotiate_response(writer, &resp, use_jsonline).await?;
        return Ok(None);
    };

    if selected == Phase3Cipher::Null {
        debug!("BTSP Phase 3: client did not offer supported cipher — null fallback");
        let result = NegotiateResult {
            cipher: Phase3Cipher::Null.wire_name().to_owned(),
            server_nonce: String::new(),
        };
        let resp = serde_json::json!({"jsonrpc": "2.0", "result": result, "id": request_id});
        write_negotiate_response(writer, &resp, use_jsonline).await?;
        return Ok(None);
    }

    let handshake_key = derive_handshake_key(family_seed.as_bytes())?;

    let client_nonce = BASE64.decode(&params.client_nonce).map_err(|e| {
        NestGateError::validation_error(format!("Invalid base64 in client_nonce: {e}"))
    })?;

    let server_nonce = generate_server_nonce()?;
    let server_nonce_b64 = BASE64.encode(server_nonce);

    let keys = SessionKeys::derive(&handshake_key, &client_nonce, &server_nonce, true)?;

    let result = NegotiateResult {
        cipher: selected.wire_name().to_owned(),
        server_nonce: server_nonce_b64,
    };
    let resp = serde_json::json!({"jsonrpc": "2.0", "result": result, "id": request_id});
    write_negotiate_response(writer, &resp, use_jsonline).await?;

    info!(
        session_id = params.session_id.as_str(),
        cipher = selected.wire_name(),
        "BTSP Phase 3: encrypted channel negotiated"
    );

    Ok(Some(keys))
}

/// Write a Phase 3 negotiate JSON-RPC response in the appropriate framing.
async fn write_negotiate_response<W: AsyncWriteExt + Unpin>(
    writer: &mut W,
    response: &Value,
    use_jsonline: bool,
) -> Result<()> {
    if use_jsonline {
        let mut line = serde_json::to_string(response).map_err(|e| {
            NestGateError::api_internal_error(format!("BTSP Phase 3: serialize response: {e}"))
        })?;
        line.push('\n');
        writer
            .write_all(line.as_bytes())
            .await
            .map_err(|e| NestGateError::io_error(format!("BTSP Phase 3: write response: {e}")))?;
    } else {
        let payload = serde_json::to_vec(response).map_err(|e| {
            NestGateError::api_internal_error(format!("BTSP Phase 3: serialize response: {e}"))
        })?;
        write_frame(writer, &payload).await?;
    }
    writer
        .flush()
        .await
        .map_err(|e| NestGateError::io_error(format!("BTSP Phase 3: flush: {e}")))?;
    Ok(())
}

/// Encrypted BTSP frame loop — generic over the dispatch function.
///
/// Reads length-prefixed encrypted frames, decrypts, dispatches via the
/// provided async callback, encrypts the response, and writes it back.
///
/// The `dispatch` closure receives a `serde_json::Value` (the parsed
/// JSON-RPC request) and returns a `serde_json::Value` (the response).
///
/// # Errors
///
/// Returns an error on fatal I/O failure. EOF is treated as clean close.
pub async fn run_encrypted_frame_loop<R, W, F, Fut>(
    reader: &mut R,
    writer: &mut W,
    session_keys: &SessionKeys,
    dispatch: F,
) -> Result<()>
where
    R: AsyncReadExt + Unpin,
    W: AsyncWriteExt + Unpin,
    F: Fn(Value) -> Fut,
    Fut: std::future::Future<Output = Value>,
{
    loop {
        let frame = match read_frame(reader).await {
            Ok(f) => f,
            Err(e) if is_eof_error(&e) => break,
            Err(e) => {
                warn!("BTSP encrypted frame read error: {e}");
                break;
            }
        };

        let plaintext = match session_keys.decrypt(&frame) {
            Ok(p) => p,
            Err(e) => {
                warn!("BTSP decrypt error: {e}");
                break;
            }
        };

        let request: Value = match serde_json::from_slice(&plaintext) {
            Ok(v) => v,
            Err(e) => {
                let err_response = serde_json::json!({
                    "jsonrpc": "2.0",
                    "error": {"code": -32700, "message": "Parse error", "data": {"error": e.to_string()}},
                    "id": null
                });
                encrypt_and_write(writer, session_keys, &err_response).await?;
                continue;
            }
        };

        let response = dispatch(request).await;
        encrypt_and_write(writer, session_keys, &response).await?;
    }

    Ok(())
}

/// Encrypt a JSON value and write as a length-prefixed frame.
async fn encrypt_and_write<W: AsyncWriteExt + Unpin>(
    writer: &mut W,
    keys: &SessionKeys,
    response: &Value,
) -> Result<()> {
    let payload = serde_json::to_vec(response)
        .map_err(|e| NestGateError::api_internal_error(format!("BTSP Phase 3: serialize: {e}")))?;
    let encrypted = keys.encrypt(&payload)?;
    write_frame(writer, &encrypted).await
}

fn is_eof_error(e: &NestGateError) -> bool {
    let msg = e.to_string();
    msg.contains("UnexpectedEof") || msg.contains("unexpected eof")
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::duplex;

    #[tokio::test]
    async fn read_write_frame_roundtrip() {
        let (mut client, mut server) = duplex(4096);
        let payload = b"hello btsp frame";

        write_frame(&mut client, payload).await.expect("write");
        drop(client);

        let received = read_frame(&mut server).await.expect("read");
        assert_eq!(received, payload);
    }

    #[tokio::test]
    async fn frame_too_large_rejected() {
        let (mut client, mut server) = duplex(4096);

        let bad_len: u32 = 17 * 1024 * 1024; // > 16 MiB
        client
            .write_all(&bad_len.to_be_bytes())
            .await
            .expect("write len");
        drop(client);

        let result = read_frame(&mut server).await;
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("too large"), "unexpected: {err_msg}");
    }

    #[tokio::test]
    async fn try_negotiate_non_negotiate_returns_none() {
        let (_client, mut writer) = duplex(4096);
        let msg = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "health.liveness",
            "id": 1
        });

        let result = try_phase3_negotiate(&msg, &mut writer, true)
            .await
            .expect("negotiate");
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn try_negotiate_missing_params_returns_none() {
        let (mut reader, mut writer) = duplex(4096);
        let msg = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "btsp.negotiate",
            "id": 1
        });

        let result = try_phase3_negotiate(&msg, &mut writer, true)
            .await
            .expect("negotiate");
        assert!(result.is_none());

        drop(writer);
        let mut response_line = String::new();
        tokio::io::AsyncBufReadExt::read_line(
            &mut tokio::io::BufReader::new(&mut reader),
            &mut response_line,
        )
        .await
        .expect("read response");
        let resp: Value = serde_json::from_str(&response_line).expect("parse response");
        assert!(resp.get("error").is_some());
    }

    #[tokio::test]
    async fn try_negotiate_unsupported_cipher_returns_null() {
        let (mut reader, mut writer) = duplex(4096);
        let nonce_b64 = BASE64.encode([0xAA; 32]);
        let msg = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "btsp.negotiate",
            "params": {
                "session_id": "test-session",
                "ciphers": ["aes-256-gcm"],
                "client_nonce": nonce_b64
            },
            "id": 2
        });

        let result = try_phase3_negotiate(&msg, &mut writer, true)
            .await
            .expect("negotiate");
        assert!(result.is_none());

        drop(writer);
        let mut response_line = String::new();
        tokio::io::AsyncBufReadExt::read_line(
            &mut tokio::io::BufReader::new(&mut reader),
            &mut response_line,
        )
        .await
        .expect("read response");
        let resp: Value = serde_json::from_str(&response_line).expect("parse response");
        let cipher = resp["result"]["cipher"].as_str().expect("cipher field");
        assert_eq!(cipher, "null");
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn try_negotiate_with_family_seed_returns_keys() {
        temp_env::async_with_vars(
            [("FAMILY_SEED", Some("test-family-seed-for-phase3-negotiate"))],
            async {
                let (mut reader, mut writer) = duplex(4096);
                let nonce_b64 = BASE64.encode([0xBB; 32]);
                let msg = serde_json::json!({
                    "jsonrpc": "2.0",
                    "method": "btsp.negotiate",
                    "params": {
                        "session_id": "test-session-456",
                        "ciphers": ["chacha20-poly1305"],
                        "client_nonce": nonce_b64
                    },
                    "id": 3
                });

                let result = try_phase3_negotiate(&msg, &mut writer, true)
                    .await
                    .expect("negotiate");
                assert!(result.is_some(), "should return session keys");

                drop(writer);
                let mut response_line = String::new();
                tokio::io::AsyncBufReadExt::read_line(
                    &mut tokio::io::BufReader::new(&mut reader),
                    &mut response_line,
                )
                .await
                .expect("read response");
                let resp: Value = serde_json::from_str(&response_line).expect("parse response");
                assert_eq!(resp["result"]["cipher"], "chacha20-poly1305");
                assert!(resp["result"]["server_nonce"].as_str().is_some());

                let server_nonce_b64 = resp["result"]["server_nonce"].as_str().expect("nonce");
                let server_nonce = BASE64.decode(server_nonce_b64).expect("decode nonce");
                assert_eq!(server_nonce.len(), 32);
            },
        )
        .await;
    }

    #[tokio::test]
    async fn encrypted_frame_roundtrip() {
        let handshake_key = [0x42u8; 32];
        let client_nonce = [3u8; 32];
        let server_nonce = [4u8; 32];

        let server_keys = SessionKeys::derive(&handshake_key, &client_nonce, &server_nonce, true)
            .expect("server keys");
        let client_keys = SessionKeys::derive(&handshake_key, &client_nonce, &server_nonce, false)
            .expect("client keys");

        let plaintext = b"test json-rpc payload";

        let (mut tx, mut rx) = duplex(8192);

        let encrypted = server_keys.encrypt(plaintext).expect("encrypt");
        write_frame(&mut tx, &encrypted).await.expect("write");
        drop(tx);

        let frame = read_frame(&mut rx).await.expect("read");
        let decrypted = client_keys.decrypt(&frame).expect("decrypt");
        assert_eq!(decrypted, plaintext);
    }

    #[tokio::test]
    async fn encrypted_loop_dispatch_roundtrip() {
        let handshake_key = [0x42u8; 32];
        let client_nonce = [3u8; 32];
        let server_nonce = [4u8; 32];

        let server_keys = SessionKeys::derive(&handshake_key, &client_nonce, &server_nonce, true)
            .expect("server keys");
        let client_keys = SessionKeys::derive(&handshake_key, &client_nonce, &server_nonce, false)
            .expect("client keys");

        let (client_tx, server_rx) = duplex(8192);
        let (server_tx, client_rx) = duplex(8192);

        let echo_dispatch = |request: Value| async move {
            serde_json::json!({
                "jsonrpc": "2.0",
                "result": request.get("params").cloned().unwrap_or(Value::Null),
                "id": request.get("id").cloned().unwrap_or(Value::Null)
            })
        };

        let server_handle = tokio::spawn(async move {
            let mut rx = server_rx;
            let mut tx = server_tx;
            run_encrypted_frame_loop(&mut rx, &mut tx, &server_keys, echo_dispatch)
                .await
                .expect("encrypted loop");
        });

        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "test.echo",
            "params": {"hello": "world"},
            "id": 1
        });
        let request_bytes = serde_json::to_vec(&request).expect("serialize");
        let encrypted_req = client_keys.encrypt(&request_bytes).expect("encrypt");

        let mut tx = client_tx;
        let mut rx = client_rx;
        write_frame(&mut tx, &encrypted_req)
            .await
            .expect("write request");
        drop(tx);

        let response_frame = read_frame(&mut rx).await.expect("read response");
        let decrypted = client_keys.decrypt(&response_frame).expect("decrypt");
        let response: Value = serde_json::from_slice(&decrypted).expect("parse response");

        assert_eq!(response["result"]["hello"], "world");
        assert_eq!(response["id"], 1);

        server_handle.await.expect("server task");
    }
}
