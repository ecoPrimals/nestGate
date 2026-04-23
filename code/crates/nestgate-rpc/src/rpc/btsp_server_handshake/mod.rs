// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # BTSP Server-Side Handshake
//!
//! Implements the listener side of the BTSP handshake protocol per
//! `BTSP_PROTOCOL_STANDARD.md` §Handshake Protocol. `NestGate` delegates all
//! cryptographic operations to the security capability provider via JSON-RPC
//! calls to `btsp.session.create`, `btsp.session.verify`, and
//! `btsp.negotiate`.
//!
//! ## Wire Framing
//!
//! Two encodings are supported (auto-detected from the first byte):
//! - **Length-prefixed**: `[4-byte BE u32 length][JSON payload]` (max 16 MiB)
//! - **JSON-line**: newline-delimited JSON (`{...}\n`)
//!
//! The server detects which mode the client uses from the first byte (`{` →
//! JSON-line, otherwise → length-prefixed) and responds in the same mode.
//!
//! ## Flow (server perspective)
//!
//! 1. Read `ClientHello` frame → extract `client_ephemeral_pub`
//! 2. Generate 32-byte random challenge
//! 3. Delegate to security provider: `btsp.session.create` → get `session_id`, `server_ephemeral_pub`
//! 4. Write `ServerHello` frame → `{version, server_ephemeral_pub, challenge}`
//! 5. Read `ChallengeResponse` frame → extract `response`, `preferred_cipher`
//! 6. Delegate to security provider: `btsp.session.verify` → get `verified`
//! 7. On success: `btsp.negotiate` → get negotiated `cipher`
//! 8. Write `HandshakeComplete` frame → `{cipher, session_id}`
//!
//! On failure at any step, write an error frame and close the connection.

use base64::{Engine as _, engine::general_purpose::STANDARD as B64};
use nestgate_types::error::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tracing::{debug, error, info, warn};

use super::btsp_client::resolve_security_socket_path;
use super::jsonrpc_client::JsonRpcClient;

const MAX_FRAME_SIZE: u32 = 16 * 1024 * 1024; // 16 MiB per BTSP spec
const BTSP_VERSION: u32 = 1;

/// Outcome of a successful BTSP handshake on the server side.
#[derive(Debug, Clone)]
pub struct BtspSession {
    /// Session identifier from the security capability provider.
    pub session_id: String,
    /// Negotiated cipher suite name (e.g. `chacha20_poly1305`, `hmac_plain`, `null`).
    pub cipher: String,
    /// Whether per-frame encryption is active (cipher != `null`).
    pub encrypted: bool,
}

// ── Wire frame I/O ──────────────────────────────────────────────────────────

async fn read_frame<R: AsyncReadExt + Unpin>(reader: &mut R) -> Result<Vec<u8>> {
    let mut len_buf = [0u8; 4];
    reader.read_exact(&mut len_buf).await.map_err(|e| {
        NestGateError::io_error(format!("BTSP handshake: failed to read frame length: {e}"))
    })?;
    let len = u32::from_be_bytes(len_buf);
    if len > MAX_FRAME_SIZE {
        return Err(NestGateError::validation_error(format!(
            "BTSP handshake: frame too large ({len} > {MAX_FRAME_SIZE})"
        )));
    }
    let buf_len = usize::try_from(len).map_err(|_| {
        NestGateError::validation_error("BTSP handshake: frame length does not fit usize")
    })?;
    let mut buf = vec![0u8; buf_len];
    reader.read_exact(&mut buf).await.map_err(|e| {
        NestGateError::io_error(format!("BTSP handshake: failed to read frame payload: {e}"))
    })?;
    Ok(buf)
}

async fn write_frame<W: AsyncWriteExt + Unpin>(writer: &mut W, payload: &[u8]) -> Result<()> {
    let len = u32::try_from(payload.len()).map_err(|_| {
        NestGateError::validation_error("BTSP handshake: frame payload exceeds u32::MAX")
    })?;
    writer.write_all(&len.to_be_bytes()).await.map_err(|e| {
        NestGateError::io_error(format!("BTSP handshake: failed to write frame length: {e}"))
    })?;
    writer.write_all(payload).await.map_err(|e| {
        NestGateError::io_error(format!(
            "BTSP handshake: failed to write frame payload: {e}"
        ))
    })?;
    writer
        .flush()
        .await
        .map_err(|e| NestGateError::io_error(format!("BTSP handshake: flush failed: {e}")))?;
    Ok(())
}

async fn write_error_frame<W: AsyncWriteExt + Unpin>(writer: &mut W, reason: &str) {
    let payload = serde_json::to_vec(&json!({"error": "handshake_failed", "reason": reason}))
        .unwrap_or_default();
    let _ = write_frame(writer, &payload).await;
}

// ── JSON-line framing (newline-delimited) ───────────────────────────────────

/// Read a single newline-delimited JSON message from a buffered reader.
async fn read_json_line<R: AsyncBufReadExt + Unpin>(reader: &mut R) -> Result<Vec<u8>> {
    let mut line = String::new();
    let n = reader.read_line(&mut line).await.map_err(|e| {
        NestGateError::io_error(format!("BTSP JSON-line: failed to read line: {e}"))
    })?;
    if n == 0 {
        return Err(NestGateError::io_error(
            "BTSP JSON-line: connection closed before complete line",
        ));
    }
    Ok(line.trim_end().as_bytes().to_vec())
}

/// Write a JSON value as a newline-delimited line.
async fn write_json_line<W: AsyncWriteExt + Unpin>(writer: &mut W, payload: &[u8]) -> Result<()> {
    writer
        .write_all(payload)
        .await
        .map_err(|e| NestGateError::io_error(format!("BTSP JSON-line: failed to write: {e}")))?;
    writer.write_all(b"\n").await.map_err(|e| {
        NestGateError::io_error(format!("BTSP JSON-line: failed to write newline: {e}"))
    })?;
    writer
        .flush()
        .await
        .map_err(|e| NestGateError::io_error(format!("BTSP JSON-line: flush failed: {e}")))?;
    Ok(())
}

/// Framing mode detected from the first byte of the client's message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FrameMode {
    /// 4-byte BE length prefix (standard BTSP)
    LengthPrefixed,
    /// Newline-delimited JSON lines
    JsonLine,
}

// ── Handshake messages ──────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct ClientHello {
    #[expect(
        dead_code,
        reason = "deserialized for protocol compat, validated by presence not value"
    )]
    version: Option<u32>,
    client_ephemeral_pub: String,
}

#[derive(Debug, Serialize)]
struct ServerHello {
    version: u32,
    server_ephemeral_pub: String,
    challenge: String,
}

#[derive(Debug, Deserialize)]
struct ChallengeResponse {
    response: String,
    preferred_cipher: Option<String>,
    /// BearDog-style field; used as session token when present.
    session_token: Option<String>,
}

#[derive(Debug, Serialize)]
struct HandshakeComplete {
    cipher: String,
    session_id: String,
}

// ── Challenge generation ────────────────────────────────────────────────────

fn generate_challenge() -> [u8; 32] {
    let mut buf = [0u8; 32];
    buf[..16].copy_from_slice(&uuid::Uuid::new_v4().into_bytes());
    buf[16..].copy_from_slice(&uuid::Uuid::new_v4().into_bytes());
    buf
}

// ── Public API ──────────────────────────────────────────────────────────────

/// Returns `true` when BTSP handshake is required on the server socket.
///
/// Production mode = `FAMILY_ID` (or variant) is set to a non-default value
/// AND `BIOMEOS_INSECURE` is not `"1"`.
#[must_use]
pub fn is_btsp_required() -> bool {
    let fid = std::env::var("FAMILY_ID")
        .or_else(|_| std::env::var("BIOMEOS_FAMILY_ID"))
        .or_else(|_| std::env::var("NESTGATE_FAMILY_ID"))
        .unwrap_or_default();

    if fid.is_empty() || fid == "default" || fid == "standalone" {
        return false;
    }

    !matches!(std::env::var("BIOMEOS_INSECURE").as_deref(), Ok("1"))
}

/// Perform the BTSP server-side handshake on an accepted connection.
///
/// Automatically detects framing from the first byte: `{` (0x7B) means
/// JSON-line mode (newline-delimited); anything else is standard
/// length-prefixed framing (4-byte BE header).
///
/// Delegates all cryptographic operations to the security capability provider
/// via JSON-RPC. On success, the stream is ready for (optionally encrypted)
/// JSON-RPC frames. On failure, an error frame is written and the error is
/// returned (caller should close the connection).
///
/// # Errors
///
/// Returns an error if any handshake step fails (frame read/write, JSON
/// parsing, or security-provider IPC delegation for session/verify/negotiate).
#[expect(
    clippy::too_many_lines,
    reason = "BTSP handshake is a single linear protocol sequence"
)]
pub async fn perform_handshake<R, W>(
    reader: &mut R,
    writer: &mut W,
    _family_id: &str,
) -> Result<BtspSession>
where
    R: AsyncReadExt + Unpin,
    W: AsyncWriteExt + Unpin,
{
    info!("BTSP: starting server-side handshake");

    // Wrap in BufReader so we can peek without consuming.
    let mut buf_reader = BufReader::new(reader);

    // Detect framing: peek the first byte.
    let first = buf_reader
        .fill_buf()
        .await
        .map_err(|e| NestGateError::io_error(format!("BTSP: failed to peek first byte: {e}")))?;
    let mode = if first.first() == Some(&b'{') {
        debug!("BTSP: detected JSON-line framing");
        FrameMode::JsonLine
    } else {
        debug!("BTSP: detected length-prefixed framing");
        FrameMode::LengthPrefixed
    };

    // 1. Read ClientHello
    let hello_bytes = match mode {
        FrameMode::LengthPrefixed => read_frame(&mut buf_reader).await.map_err(|e| {
            error!("BTSP: failed to read ClientHello: {e}");
            e
        })?,
        FrameMode::JsonLine => read_json_line(&mut buf_reader).await.map_err(|e| {
            error!("BTSP: failed to read ClientHello JSON-line: {e}");
            e
        })?,
    };
    let client_hello: ClientHello = match serde_json::from_slice(&hello_bytes) {
        Ok(h) => h,
        Err(e) => {
            write_error_frame(writer, "invalid_client_hello").await;
            return Err(NestGateError::validation_error(format!(
                "BTSP: malformed ClientHello: {e}"
            )));
        }
    };
    debug!("BTSP: received ClientHello");

    // 2. Generate challenge
    let challenge_bytes = generate_challenge();
    let challenge_b64 = B64.encode(challenge_bytes);

    // 3. Delegate to security provider: btsp.session.create
    let security_path = resolve_security_socket_path();
    let security_path_str = security_path.to_str().ok_or_else(|| {
        NestGateError::validation_error("BTSP: security socket path is not valid UTF-8")
    })?;

    let mut bd_client = JsonRpcClient::connect_unix(security_path_str)
        .await
        .map_err(|e| {
            error!(
                "BTSP: cannot connect to security provider at {}: {e}",
                security_path.display()
            );
            NestGateError::api_internal_error(format!(
                "BTSP: security provider unavailable at {}: {e}",
                security_path.display()
            ))
        })?;

    let create_result = bd_client
        .call(
            "btsp.session.create",
            json!({
                "family_seed_ref": "env:FAMILY_SEED",
                "client_ephemeral_pub": client_hello.client_ephemeral_pub,
                "challenge": challenge_b64,
            }),
        )
        .await
        .map_err(|e| {
            error!("BTSP: btsp.session.create failed: {e}");
            NestGateError::api_internal_error(format!("BTSP: session create failed: {e}"))
        })?;

    let session_id = create_result
        .get("session_id")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            NestGateError::api_internal_error("BTSP: missing session_id from security provider")
        })?
        .to_string();

    let server_ephemeral_pub = create_result
        .get("server_ephemeral_pub")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            NestGateError::api_internal_error(
                "BTSP: missing server_ephemeral_pub from security provider",
            )
        })?
        .to_string();

    // BearDog may return a challenge in its create response.
    let bd_challenge = create_result
        .get("challenge")
        .and_then(Value::as_str)
        .unwrap_or(&challenge_b64);

    debug!("BTSP: session created (id={session_id})");

    // 4. Write ServerHello — match framing mode
    let server_hello = ServerHello {
        version: BTSP_VERSION,
        server_ephemeral_pub: server_ephemeral_pub.clone(),
        challenge: bd_challenge.to_string(),
    };
    let server_hello_bytes = serde_json::to_vec(&server_hello).map_err(|e| {
        NestGateError::api_internal_error(format!("BTSP: failed to serialize ServerHello: {e}"))
    })?;
    match mode {
        FrameMode::LengthPrefixed => write_frame(writer, &server_hello_bytes).await?,
        FrameMode::JsonLine => write_json_line(writer, &server_hello_bytes).await?,
    }
    debug!("BTSP: sent ServerHello");

    // 5. Read ChallengeResponse
    let cr_bytes = match mode {
        FrameMode::LengthPrefixed => read_frame(&mut buf_reader).await.map_err(|e| {
            error!("BTSP: failed to read ChallengeResponse: {e}");
            e
        })?,
        FrameMode::JsonLine => read_json_line(&mut buf_reader).await.map_err(|e| {
            error!("BTSP: failed to read ChallengeResponse JSON-line: {e}");
            e
        })?,
    };
    let challenge_response: ChallengeResponse = serde_json::from_slice(&cr_bytes).map_err(|e| {
        NestGateError::validation_error(format!("BTSP: malformed ChallengeResponse: {e}"))
    })?;
    debug!("BTSP: received ChallengeResponse");

    // 6. Delegate to security provider: btsp.session.verify
    let mut bd_verify = JsonRpcClient::connect_unix(security_path_str)
        .await
        .map_err(|e| {
            NestGateError::api_internal_error(format!(
                "BTSP: security provider reconnect failed: {e}"
            ))
        })?;

    let verify_result = bd_verify
        .call(
            "btsp.session.verify",
            json!({
                "session_id": session_id,
                "client_response": challenge_response.response,
                "client_ephemeral_pub": client_hello.client_ephemeral_pub,
                "server_ephemeral_pub": server_ephemeral_pub,
                "challenge": bd_challenge,
            }),
        )
        .await
        .map_err(|e| {
            error!("BTSP: btsp.session.verify failed: {e}");
            NestGateError::api_internal_error(format!("BTSP: session verify failed: {e}"))
        })?;

    let verified = verify_result
        .get("verified")
        .and_then(Value::as_bool)
        .unwrap_or(false);

    if !verified {
        warn!("BTSP: handshake FAILED — family verification rejected");
        write_error_frame(writer, "family_verification").await;
        return Err(NestGateError::api_internal_error(
            "BTSP: handshake failed — client could not prove family membership",
        ));
    }

    debug!("BTSP: challenge-response verified");

    // 7. Negotiate cipher via btsp.negotiate
    let preferred = challenge_response
        .preferred_cipher
        .as_deref()
        .unwrap_or("chacha20_poly1305");

    let session_token = challenge_response
        .session_token
        .as_deref()
        .unwrap_or(&session_id);

    let mut bd_negotiate = JsonRpcClient::connect_unix(security_path_str)
        .await
        .map_err(|e| {
            NestGateError::api_internal_error(format!(
                "BTSP: security provider reconnect failed: {e}"
            ))
        })?;

    let negotiate_result = bd_negotiate
        .call(
            "btsp.negotiate",
            json!({
                "session_id": session_id,
                "session_token": session_token,
                "preferred_cipher": preferred,
                "bond_type": "Covalent",
            }),
        )
        .await
        .map_err(|e| {
            warn!("BTSP: cipher negotiation failed, defaulting to null: {e}");
            e
        })
        .unwrap_or_else(|_| json!({"cipher": "null"}));

    let cipher = negotiate_result
        .get("cipher")
        .and_then(Value::as_str)
        .unwrap_or("null")
        .to_string();

    // 8. Write HandshakeComplete — match framing mode
    let complete = HandshakeComplete {
        cipher: cipher.clone(),
        session_id: session_id.clone(),
    };
    let complete_bytes = serde_json::to_vec(&complete).map_err(|e| {
        NestGateError::api_internal_error(format!(
            "BTSP: failed to serialize HandshakeComplete: {e}"
        ))
    })?;
    match mode {
        FrameMode::LengthPrefixed => write_frame(writer, &complete_bytes).await?,
        FrameMode::JsonLine => write_json_line(writer, &complete_bytes).await?,
    }

    let encrypted = cipher != "null";
    info!(
        "BTSP: handshake complete (session={session_id}, cipher={cipher}, encrypted={encrypted}, framing={mode:?})"
    );

    Ok(BtspSession {
        session_id,
        cipher,
        encrypted,
    })
}

#[cfg(test)]
mod btsp_server_handshake_tests;
