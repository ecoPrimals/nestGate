// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # BTSP Client-Side Wire Handshake
//!
//! Implements the *initiator* side of the BTSP handshake protocol per
//! `BTSP_PROTOCOL_STANDARD.md` §Handshake Protocol. Mirrors the server
//! implementation in [`super::btsp_server_handshake`] but from the client
//! perspective.
//!
//! All cryptographic operations are delegated to the security capability
//! provider via JSON-RPC calls to `btsp.session.create` and
//! `btsp.session.verify`.
//!
//! ## Wire Framing
//!
//! Uses JSON-line (NDJSON) framing by default — the ecosystem-converged
//! format for outbound BTSP connections. The server auto-detects this from
//! the first `{` byte.
//!
//! ## Flow (client perspective)
//!
//! 1. Ask security provider `btsp.session.create({family_seed})` →
//!    `client_ephemeral_pub`, `session_token`
//! 2. Connect to target peer
//! 3. Write `ClientHello` → `{protocol, version, client_ephemeral_pub}`
//! 4. Read `ServerHello` → extract `server_ephemeral_pub`, `challenge`
//! 5. Ask security provider `btsp.session.verify({session_token, challenge,
//!    client_ephemeral_pub, server_ephemeral_pub, preferred_cipher})` →
//!    `response`
//! 6. Write `ChallengeResponse` → `{response, preferred_cipher}`
//! 7. Read `HandshakeComplete` → extract `status`, `cipher`, `session_id`

use base64::{Engine as _, engine::general_purpose::STANDARD};
use nestgate_types::TransportEndpoint;
use nestgate_types::error::{ErrorContextExt, NestGateError, Result};
use serde::Deserialize;
use serde_json::{Value, json};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tracing::{debug, error, info};

use super::btsp_client::resolve_security_socket_path;
use super::btsp_server_handshake::{BtspSession, is_btsp_required, resolve_family_seed};
use super::isomorphic_ipc::{TransportStream, connect_transport};
use super::jsonrpc_client::JsonRpcClient;

/// BTSP protocol identifier sent in `ClientHello`.
const BTSP_PROTOCOL: &str = "btsp";

/// BTSP protocol version.
const BTSP_VERSION: u32 = 1;

/// Default preferred cipher for the ecosystem.
const DEFAULT_CIPHER: &str = "chacha20-poly1305";

/// Messages read back from the server during the handshake.
#[derive(Debug, Deserialize)]
struct ServerHello {
    server_ephemeral_pub: String,
    challenge: String,
}

#[derive(Debug, Deserialize)]
struct HandshakeComplete {
    status: String,
    cipher: String,
    session_id: String,
}

/// Perform a client-side BTSP handshake against `target`.
///
/// Returns a [`BtspSession`] and the connected [`TransportStream`] ready for
/// authenticated JSON-RPC. The stream is wrapped in a [`BufReader`] for the
/// caller to consume.
///
/// # Errors
///
/// Returns an error if the security provider is unreachable, the target
/// connection fails, or the handshake protocol fails at any step.
pub async fn perform_client_handshake(
    target: &TransportEndpoint,
) -> Result<(BtspSession, BufReader<TransportStream>)> {
    info!("BTSP client: starting handshake to {target}");

    // Step 1: Ask the security provider to create a session.
    let raw_seed = resolve_family_seed()?;
    let family_seed_b64 = STANDARD.encode(raw_seed.as_bytes());

    let security_path = resolve_security_socket_path();
    debug!(
        "BTSP client: connecting to security provider at {}",
        security_path.display()
    );

    let sp_endpoint = TransportEndpoint::uds(&security_path);
    let mut sp_client = JsonRpcClient::connect_transport(&sp_endpoint)
        .await
        .map_err(|e| {
            error!(
                "BTSP client: cannot connect to security provider at {}: {e}",
                security_path.display()
            );
            NestGateError::api_internal_error(format!(
                "BTSP client: security provider unavailable at {}: {e}",
                security_path.display()
            ))
        })?;

    let create_result = sp_client
        .call(
            "btsp.session.create",
            json!({ "family_seed": family_seed_b64 }),
        )
        .await
        .map_err(|e| {
            error!("BTSP client: btsp.session.create failed: {e}");
            NestGateError::api_internal_error(format!("BTSP client: session create failed: {e}"))
        })?;
    debug!("BTSP client: btsp.session.create response received");

    let client_ephemeral_pub = create_result
        .get("client_ephemeral_pub")
        .or_else(|| create_result.get("server_ephemeral_pub"))
        .and_then(Value::as_str)
        .ok_or_else(|| {
            error!("BTSP client: create response missing ephemeral_pub: {create_result}");
            NestGateError::api_internal_error(
                "BTSP client: missing client_ephemeral_pub from security provider",
            )
        })?
        .to_string();

    let session_token = create_result
        .get("session_token")
        .or_else(|| create_result.get("session_id"))
        .and_then(Value::as_str)
        .ok_or_else(|| {
            error!("BTSP client: create response missing session_token: {create_result}");
            NestGateError::api_internal_error(
                "BTSP client: missing session_token from security provider",
            )
        })?
        .to_string();

    debug!("BTSP client: session created (token={session_token})");

    // Step 2: Connect to the target peer.
    let stream = connect_transport(target)
        .await
        .net_ctx("BTSP client: failed to connect to target")?;
    let mut buf_reader = BufReader::new(stream);

    // Step 3: Write ClientHello (JSON-line framing).
    let client_hello = json!({
        "protocol": BTSP_PROTOCOL,
        "version": BTSP_VERSION,
        "client_ephemeral_pub": client_ephemeral_pub,
    });
    write_ndjson(buf_reader.get_mut(), &client_hello).await?;
    debug!("BTSP client: sent ClientHello");

    // Step 4: Read ServerHello.
    let server_hello: ServerHello = read_ndjson(&mut buf_reader).await.map_err(|e| {
        error!("BTSP client: failed to read ServerHello: {e}");
        e
    })?;
    debug!("BTSP client: received ServerHello");

    // Step 5: Ask security provider to compute challenge response.
    let verify_result = sp_client
        .call(
            "btsp.session.verify",
            json!({
                "session_token": session_token,
                "challenge": server_hello.challenge,
                "client_ephemeral_pub": client_ephemeral_pub,
                "server_ephemeral_pub": server_hello.server_ephemeral_pub,
                "preferred_cipher": DEFAULT_CIPHER,
                "role": "client",
            }),
        )
        .await
        .map_err(|e| {
            error!("BTSP client: btsp.session.verify failed: {e}");
            NestGateError::api_internal_error(format!("BTSP client: session verify failed: {e}"))
        })?;

    let response = verify_result
        .get("response")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            error!("BTSP client: verify response missing 'response': {verify_result}");
            NestGateError::api_internal_error(
                "BTSP client: security provider verify response missing 'response' field",
            )
        })?
        .to_string();

    debug!("BTSP client: challenge response computed");

    // Step 6: Write ChallengeResponse.
    let challenge_response = json!({
        "response": response,
        "preferred_cipher": DEFAULT_CIPHER,
    });
    write_ndjson(buf_reader.get_mut(), &challenge_response).await?;
    debug!("BTSP client: sent ChallengeResponse");

    // Step 7: Read HandshakeComplete.
    let complete: HandshakeComplete = read_ndjson(&mut buf_reader).await.map_err(|e| {
        error!("BTSP client: failed to read HandshakeComplete: {e}");
        e
    })?;

    if complete.status != "ok" {
        return Err(NestGateError::api_internal_error(format!(
            "BTSP client: handshake rejected by peer (status={})",
            complete.status
        )));
    }

    let encrypted = complete.cipher != "null";
    info!(
        "BTSP client: handshake complete (session={}, cipher={}, encrypted={encrypted})",
        complete.session_id, complete.cipher
    );

    Ok((
        BtspSession {
            session_id: complete.session_id,
            cipher: complete.cipher,
            encrypted,
        },
        buf_reader,
    ))
}

/// Connect to a [`TransportEndpoint`] with BTSP when required.
///
/// If `is_btsp_required()` returns `true`, performs the full client-side
/// BTSP handshake. Otherwise, connects plainly.
///
/// Returns a `JsonRpcClient` ready for authenticated JSON-RPC calls.
///
/// # Errors
///
/// Returns an error if the connection or handshake fails.
pub async fn connect_with_btsp(endpoint: &TransportEndpoint) -> Result<JsonRpcClient> {
    if is_btsp_required() {
        let (_session, buf_reader) = perform_client_handshake(endpoint).await?;
        Ok(JsonRpcClient::from_btsp_stream(buf_reader))
    } else {
        JsonRpcClient::connect_transport(endpoint).await
    }
}

// ── NDJSON helpers ──────────────────────────────────────────────────────────

async fn write_ndjson<W: AsyncWriteExt + Unpin>(writer: &mut W, value: &Value) -> Result<()> {
    let bytes = serde_json::to_vec(value).map_err(|e| {
        NestGateError::api_internal_error(format!("BTSP client: failed to serialize: {e}"))
    })?;
    writer
        .write_all(&bytes)
        .await
        .io_ctx("BTSP client: write failed")?;
    writer
        .write_all(b"\n")
        .await
        .io_ctx("BTSP client: write newline failed")?;
    writer.flush().await.io_ctx("BTSP client: flush failed")?;
    Ok(())
}

async fn read_ndjson<T: for<'de> Deserialize<'de>, S: AsyncBufReadExt + Unpin>(
    reader: &mut S,
) -> Result<T> {
    let mut line = String::new();
    let n = reader
        .read_line(&mut line)
        .await
        .io_ctx("BTSP client: read line failed")?;
    if n == 0 {
        return Err(NestGateError::io_error(
            "BTSP client: connection closed before complete response",
        ));
    }
    serde_json::from_str(line.trim()).map_err(|e| {
        NestGateError::validation_error(format!("BTSP client: malformed response: {e}"))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader as TokioBufReader};
    use tokio::net::UnixListener;

    fn mock_btsp_server(listener: UnixListener) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let (stream, _) = listener.accept().await.expect("accept");
            let (rh, mut wh) = tokio::io::split(stream);
            let mut br = TokioBufReader::new(rh);

            let mut line = String::new();
            br.read_line(&mut line).await.expect("read ClientHello");
            let hello: Value = serde_json::from_str(line.trim()).expect("parse ClientHello");
            assert_eq!(hello["protocol"], "btsp");
            assert_eq!(hello["version"], 1);
            assert!(hello["client_ephemeral_pub"].is_string());

            let server_hello = json!({
                "version": 1,
                "server_ephemeral_pub": "server_pub_key_base64",
                "challenge": "challenge_value_base64",
            });
            wh.write_all(format!("{}\n", serde_json::to_string(&server_hello).unwrap()).as_bytes())
                .await
                .unwrap();

            let mut line2 = String::new();
            br.read_line(&mut line2)
                .await
                .expect("read ChallengeResponse");
            let cr: Value = serde_json::from_str(line2.trim()).expect("parse ChallengeResponse");
            assert!(cr["response"].is_string());
            assert!(cr["preferred_cipher"].is_string());

            let complete = json!({
                "status": "ok",
                "cipher": "chacha20-poly1305",
                "session_id": "session_42",
            });
            wh.write_all(format!("{}\n", serde_json::to_string(&complete).unwrap()).as_bytes())
                .await
                .unwrap();
        })
    }

    fn mock_security_provider(listener: UnixListener) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let (stream, _) = listener.accept().await.expect("accept");
            let (rh, mut wh) = tokio::io::split(stream);
            let mut br = TokioBufReader::new(rh);

            let mut line = String::new();
            br.read_line(&mut line).await.expect("read session.create");
            let req: Value = serde_json::from_str(line.trim()).expect("parse");
            assert_eq!(req["method"], "btsp.session.create");
            let resp = json!({
                "jsonrpc": "2.0",
                "result": {
                    "client_ephemeral_pub": "client_pub_key_base64",
                    "session_token": "token_abc",
                },
                "id": req["id"],
            });
            wh.write_all(format!("{}\n", serde_json::to_string(&resp).unwrap()).as_bytes())
                .await
                .unwrap();

            let mut line2 = String::new();
            br.read_line(&mut line2).await.expect("read session.verify");
            let req2: Value = serde_json::from_str(line2.trim()).expect("parse");
            assert_eq!(req2["method"], "btsp.session.verify");
            assert_eq!(req2["params"]["role"], "client");
            let resp2 = json!({
                "jsonrpc": "2.0",
                "result": {
                    "verified": true,
                    "response": "challenge_response_value",
                    "session_id": "session_42",
                    "cipher": "chacha20-poly1305",
                },
                "id": req2["id"],
            });
            wh.write_all(format!("{}\n", serde_json::to_string(&resp2).unwrap()).as_bytes())
                .await
                .unwrap();
        })
    }

    #[tokio::test]
    async fn client_handshake_full_roundtrip() {
        let dir = tempfile::tempdir().expect("tempdir");

        let sp_path = dir.path().join("security.sock");
        let sp_listener = UnixListener::bind(&sp_path).expect("bind sp");
        let sp_server = mock_security_provider(sp_listener);

        let peer_path = dir.path().join("peer.sock");
        let peer_listener = UnixListener::bind(&peer_path).expect("bind peer");
        let peer_server = mock_btsp_server(peer_listener);

        let sp_str = sp_path.to_str().unwrap().to_string();
        let peer_clone = peer_path.clone();

        temp_env::async_with_vars(
            [
                ("FAMILY_SEED", Some("test_seed_hex_value")),
                ("SECURITY_PROVIDER_SOCKET", Some(sp_str.as_str())),
            ],
            async move {
                let target = TransportEndpoint::uds(&peer_clone);
                let (session, _stream) = perform_client_handshake(&target)
                    .await
                    .expect("handshake should succeed");
                assert_eq!(session.session_id, "session_42");
                assert_eq!(session.cipher, "chacha20-poly1305");
                assert!(session.encrypted);
            },
        )
        .await;

        sp_server.await.expect("sp server");
        peer_server.await.expect("peer server");
    }

    #[tokio::test]
    async fn client_handshake_rejected_by_peer() {
        let dir = tempfile::tempdir().expect("tempdir");

        let sp_path = dir.path().join("security2.sock");
        let sp_listener = UnixListener::bind(&sp_path).expect("bind sp");
        let sp_server = mock_security_provider(sp_listener);

        let peer_path = dir.path().join("peer_reject.sock");
        let peer_listener = UnixListener::bind(&peer_path).expect("bind peer");
        let peer_server = tokio::spawn(async move {
            let (stream, _) = peer_listener.accept().await.expect("accept");
            let (rh, mut wh) = tokio::io::split(stream);
            let mut br = TokioBufReader::new(rh);

            let mut line = String::new();
            br.read_line(&mut line).await.expect("read ClientHello");

            let server_hello = json!({
                "version": 1,
                "server_ephemeral_pub": "pub_key",
                "challenge": "challenge_val",
            });
            wh.write_all(format!("{}\n", serde_json::to_string(&server_hello).unwrap()).as_bytes())
                .await
                .unwrap();

            let mut line2 = String::new();
            br.read_line(&mut line2)
                .await
                .expect("read ChallengeResponse");

            let reject = json!({
                "status": "failed",
                "cipher": "null",
                "session_id": "",
            });
            wh.write_all(format!("{}\n", serde_json::to_string(&reject).unwrap()).as_bytes())
                .await
                .unwrap();
        });

        let sp_str = sp_path.to_str().unwrap().to_string();
        let peer_clone = peer_path.clone();

        temp_env::async_with_vars(
            [
                ("FAMILY_SEED", Some("test_seed")),
                ("SECURITY_PROVIDER_SOCKET", Some(sp_str.as_str())),
            ],
            async move {
                let target = TransportEndpoint::uds(&peer_clone);
                let err = perform_client_handshake(&target)
                    .await
                    .expect_err("should be rejected");
                assert!(err.to_string().contains("rejected"));
            },
        )
        .await;

        sp_server.await.expect("sp server");
        peer_server.await.expect("peer server");
    }

    #[test]
    fn connect_with_btsp_skips_when_not_required() {
        temp_env::with_vars(
            [("FAMILY_ID", None::<&str>), ("BIOMEOS_INSECURE", None)],
            || {
                assert!(!is_btsp_required());
            },
        );
    }
}
