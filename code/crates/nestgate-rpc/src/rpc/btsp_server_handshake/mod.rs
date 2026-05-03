// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # BTSP Server-Side Handshake
//!
//! Implements the listener side of the BTSP handshake protocol per
//! `BTSP_PROTOCOL_STANDARD.md` §Handshake Protocol. `NestGate` delegates all
//! cryptographic operations to the security capability provider via JSON-RPC
//! calls to `btsp.session.create` and `btsp.session.verify`.
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
//! 2. Resolve `FAMILY_SEED` from environment, base64-encode for transport
//! 3. Delegate to security provider: `btsp.session.create({family_seed})` → get
//!    `session_token`, `server_ephemeral_pub`, `challenge`
//! 4. Write `ServerHello` frame → `{version, server_ephemeral_pub, challenge}`
//! 5. Read `ChallengeResponse` frame → extract `response`, `preferred_cipher`
//! 6. Delegate to security provider: `btsp.session.verify({session_token, response,
//!    client_ephemeral_pub, preferred_cipher})` → get `verified`, `session_id`,
//!    `cipher`
//! 7. Write `HandshakeComplete` frame → `{cipher, session_id}`
//!
//! On failure at any step, write an error frame and close the connection.

use base64::{Engine as _, engine::general_purpose::STANDARD};
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

/// Mode-aware error frame: responds in the same framing the client used so a
/// JSON-line client never receives a length-prefixed error (and vice versa).
async fn write_handshake_error<W: AsyncWriteExt + Unpin>(
    writer: &mut W,
    mode: FrameMode,
    reason: &str,
) {
    let payload = serde_json::to_vec(&json!({"error": "handshake_failed", "reason": reason}))
        .unwrap_or_default();
    let _ = match mode {
        FrameMode::LengthPrefixed => write_frame(writer, &payload).await,
        FrameMode::JsonLine => write_json_line(writer, &payload).await,
    };
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
}

#[derive(Debug, Serialize)]
struct HandshakeComplete {
    status: &'static str,
    cipher: String,
    session_id: String,
}

// ── Challenge generation (test-only) ────────────────────────────────────────

#[cfg(test)]
fn generate_challenge() -> [u8; 32] {
    let mut buf = [0u8; 32];
    buf[..16].copy_from_slice(&uuid::Uuid::new_v4().into_bytes());
    buf[16..].copy_from_slice(&uuid::Uuid::new_v4().into_bytes());
    buf
}

// ── Seed resolution ─────────────────────────────────────────────────────────

/// Reads the family seed from the environment (typically a hex string).
///
/// Checks canonical `FAMILY_SEED` first, then capability-scoped
/// `SECURITY_FAMILY_SEED`, then backward-compat `BEARDOG_FAMILY_SEED` and
/// `BIOMEOS_FAMILY_SEED`. The value is trimmed to strip trailing newlines
/// that `xxd -p` or similar tools may leave. The caller is responsible for
/// base64-encoding the result before sending to the security provider.
pub(crate) fn resolve_family_seed() -> Result<String> {
    for var in [
        "FAMILY_SEED",
        "SECURITY_FAMILY_SEED",
        "BEARDOG_FAMILY_SEED",
        "BIOMEOS_FAMILY_SEED",
    ] {
        if let Ok(val) = std::env::var(var) {
            let trimmed = val.trim().to_string();
            if !trimmed.is_empty() {
                debug!("BTSP: resolved family seed from {var}");
                return Ok(trimmed);
            }
        }
    }
    Err(NestGateError::validation_error(
        "BTSP: FAMILY_SEED (or SECURITY_FAMILY_SEED) env var is required \
         for btsp.session.create",
    ))
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
/// On failure, a **mode-aware** error frame is written to the client before
/// the error is returned, so the client always sees an error message instead
/// of bare EOF.  This eliminates the "zero bytes / silent close" symptom
/// reported by `primalSpring` guidestone.
///
/// # Errors
///
/// Returns an error if any handshake step fails (frame read/write, JSON
/// parsing, or security-provider IPC delegation for session/verify).
pub async fn perform_handshake<R, W>(
    reader: &mut R,
    writer: &mut W,
    family_id: &str,
) -> Result<BtspSession>
where
    R: AsyncReadExt + Unpin,
    W: AsyncWriteExt + Unpin,
{
    info!("BTSP: starting server-side handshake");

    let mut buf_reader = BufReader::new(reader);

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

    let result = run_handshake_protocol(&mut buf_reader, writer, family_id, mode).await;
    if let Err(ref e) = result {
        write_handshake_error(writer, mode, &e.to_string()).await;
    }
    result
}

/// Inner handshake protocol: separated so that ANY error propagated via `?` is
/// caught by [`perform_handshake`], which writes a mode-aware error frame.
#[expect(
    clippy::too_many_lines,
    reason = "BTSP handshake is a single linear protocol sequence"
)]
async fn run_handshake_protocol<R, W>(
    buf_reader: &mut BufReader<&mut R>,
    writer: &mut W,
    _family_id: &str,
    mode: FrameMode,
) -> Result<BtspSession>
where
    R: AsyncReadExt + Unpin,
    W: AsyncWriteExt + Unpin,
{
    // 1. Read ClientHello
    let hello_bytes = match mode {
        FrameMode::LengthPrefixed => read_frame(buf_reader).await.map_err(|e| {
            error!("BTSP: failed to read ClientHello: {e}");
            e
        })?,
        FrameMode::JsonLine => read_json_line(buf_reader).await.map_err(|e| {
            error!("BTSP: failed to read ClientHello JSON-line: {e}");
            e
        })?,
    };
    let client_hello: ClientHello = serde_json::from_slice(&hello_bytes).map_err(|e| {
        NestGateError::validation_error(format!("BTSP: malformed ClientHello: {e}"))
    })?;
    debug!("BTSP: received ClientHello");

    // 2. Resolve family seed and base64-encode for transport.
    //    The security provider base64-decodes the param before HKDF key
    //    derivation.  All converged primals base64-encode the raw env
    //    string so the provider recovers the original hex for key agreement.
    let raw_seed = resolve_family_seed()?;
    let family_seed_b64 = STANDARD.encode(raw_seed.as_bytes());

    // 3. Delegate to security provider: btsp.session.create
    let security_path = resolve_security_socket_path();
    debug!(
        "BTSP: connecting to security provider at {}",
        security_path.display()
    );
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
    debug!("BTSP: connected to security provider");

    let create_result = bd_client
        .call(
            "btsp.session.create",
            json!({
                "family_seed": family_seed_b64,
            }),
        )
        .await
        .map_err(|e| {
            error!("BTSP: btsp.session.create failed: {e}");
            NestGateError::api_internal_error(format!("BTSP: session create failed: {e}"))
        })?;
    debug!("BTSP: btsp.session.create response received");

    // Accept both `session_token` (BTSP convergence doc) and `session_id`
    // (some security provider versions return this field name).
    let session_token = create_result
        .get("session_token")
        .or_else(|| create_result.get("session_id"))
        .and_then(Value::as_str)
        .ok_or_else(|| {
            error!("BTSP: create response missing session_token/session_id: {create_result}");
            NestGateError::api_internal_error(
                "BTSP: missing session_token/session_id from security provider",
            )
        })?
        .to_string();

    let server_ephemeral_pub = create_result
        .get("server_ephemeral_pub")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            error!("BTSP: create response missing server_ephemeral_pub: {create_result}");
            NestGateError::api_internal_error(
                "BTSP: missing server_ephemeral_pub from security provider",
            )
        })?
        .to_string();

    let bd_challenge = create_result
        .get("challenge")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            error!("BTSP: create response missing challenge: {create_result}");
            NestGateError::api_internal_error("BTSP: missing challenge from security provider")
        })?
        .to_string();

    debug!("BTSP: session created (token={session_token})");

    // 4. Write ServerHello — match framing mode
    let server_hello = ServerHello {
        version: BTSP_VERSION,
        server_ephemeral_pub: server_ephemeral_pub.clone(),
        challenge: bd_challenge.clone(),
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
        FrameMode::LengthPrefixed => read_frame(buf_reader).await.map_err(|e| {
            error!("BTSP: failed to read ChallengeResponse: {e}");
            e
        })?,
        FrameMode::JsonLine => read_json_line(buf_reader).await.map_err(|e| {
            error!("BTSP: failed to read ChallengeResponse JSON-line: {e}");
            e
        })?,
    };
    let challenge_response: ChallengeResponse = serde_json::from_slice(&cr_bytes).map_err(|e| {
        NestGateError::validation_error(format!("BTSP: malformed ChallengeResponse: {e}"))
    })?;
    debug!("BTSP: received ChallengeResponse");

    // 6. Delegate to security provider: btsp.session.verify
    //    Reuse the SAME connection from btsp.session.create — the security
    //    provider keeps sockets open and sessions may be connection-scoped.
    let preferred = challenge_response
        .preferred_cipher
        .as_deref()
        .unwrap_or("null");

    let verify_result = bd_client
        .call(
            "btsp.session.verify",
            json!({
                "session_token": session_token,
                "response": challenge_response.response,
                "client_ephemeral_pub": client_hello.client_ephemeral_pub,
                "preferred_cipher": preferred,
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
        let reason = verify_result
            .get("error")
            .and_then(Value::as_str)
            .unwrap_or("unknown");
        warn!("BTSP: handshake FAILED — family verification rejected: {reason}");
        return Err(NestGateError::api_internal_error(
            "BTSP: handshake failed — client could not prove family membership",
        ));
    }

    debug!("BTSP: challenge-response verified");

    let session_id = verify_result
        .get("session_id")
        .and_then(Value::as_str)
        .unwrap_or(&session_token)
        .to_string();

    let cipher = verify_result
        .get("cipher")
        .and_then(Value::as_str)
        .unwrap_or("null")
        .to_string();

    // 7. Write HandshakeComplete — match framing mode
    let complete = HandshakeComplete {
        status: "ok",
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
