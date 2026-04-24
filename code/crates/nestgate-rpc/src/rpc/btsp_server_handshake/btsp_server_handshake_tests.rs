// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::*;
use serde_json::{Value, json};
use tokio::io::BufReader;

#[test]
fn is_btsp_required_no_family() {
    temp_env::with_vars(
        [
            ("FAMILY_ID", None::<&str>),
            ("BIOMEOS_FAMILY_ID", None),
            ("NESTGATE_FAMILY_ID", None),
            ("BIOMEOS_INSECURE", None),
        ],
        || assert!(!is_btsp_required()),
    );
}

#[test]
fn is_btsp_required_default_family() {
    temp_env::with_vars(
        [
            ("FAMILY_ID", Some("default")),
            ("BIOMEOS_FAMILY_ID", None::<&str>),
            ("NESTGATE_FAMILY_ID", None),
            ("BIOMEOS_INSECURE", None),
        ],
        || assert!(!is_btsp_required()),
    );
}

#[test]
fn is_btsp_required_production_family() {
    temp_env::with_vars(
        [
            ("FAMILY_ID", Some("fam-prod-abc")),
            ("BIOMEOS_FAMILY_ID", None::<&str>),
            ("NESTGATE_FAMILY_ID", None),
            ("BIOMEOS_INSECURE", None),
        ],
        || assert!(is_btsp_required()),
    );
}

#[test]
fn is_btsp_required_insecure_override() {
    temp_env::with_vars(
        [
            ("FAMILY_ID", Some("fam-prod-abc")),
            ("BIOMEOS_FAMILY_ID", None::<&str>),
            ("NESTGATE_FAMILY_ID", None),
            ("BIOMEOS_INSECURE", Some("1")),
        ],
        || assert!(!is_btsp_required()),
    );
}

#[test]
fn is_btsp_required_standalone_family_disables() {
    temp_env::with_vars(
        [
            ("FAMILY_ID", Some("standalone")),
            ("BIOMEOS_FAMILY_ID", None::<&str>),
            ("NESTGATE_FAMILY_ID", Some("would-otherwise-require-btsp")),
            ("BIOMEOS_INSECURE", None),
        ],
        || assert!(!is_btsp_required()),
    );
}

#[test]
fn is_btsp_required_prefers_family_id_over_nestgate() {
    temp_env::with_vars(
        [
            ("FAMILY_ID", Some("default")),
            ("BIOMEOS_FAMILY_ID", None::<&str>),
            ("NESTGATE_FAMILY_ID", Some("prod-real")),
            ("BIOMEOS_INSECURE", None),
        ],
        || assert!(!is_btsp_required()),
    );
}

#[test]
fn is_btsp_required_via_nestgate_family_id() {
    temp_env::with_vars(
        [
            ("FAMILY_ID", None::<&str>),
            ("BIOMEOS_FAMILY_ID", None),
            ("NESTGATE_FAMILY_ID", Some("edge-nucleus")),
            ("BIOMEOS_INSECURE", None),
        ],
        || assert!(is_btsp_required()),
    );
}

#[test]
fn is_btsp_required_via_biomeos_family_id() {
    temp_env::with_vars(
        [
            ("FAMILY_ID", None::<&str>),
            ("BIOMEOS_FAMILY_ID", Some("bio-family")),
            ("NESTGATE_FAMILY_ID", None),
            ("BIOMEOS_INSECURE", None),
        ],
        || assert!(is_btsp_required()),
    );
}

#[test]
fn challenge_is_32_bytes() {
    let c = generate_challenge();
    assert_eq!(c.len(), 32);
    let c2 = generate_challenge();
    assert_ne!(c, c2, "challenges must be unique");
}

#[tokio::test]
async fn frame_roundtrip() {
    let payload = b"hello BTSP";
    let mut buf = Vec::new();
    write_frame(&mut buf, payload).await.expect("write");

    assert_eq!(buf.len(), 4 + payload.len());
    assert_eq!(&buf[..4], &(payload.len() as u32).to_be_bytes());

    let mut cursor = std::io::Cursor::new(buf);
    let read_back = read_frame(&mut cursor).await.expect("read");
    assert_eq!(read_back, payload);
}

#[tokio::test]
async fn frame_rejects_oversized() {
    let fake_len = (MAX_FRAME_SIZE + 1).to_be_bytes();
    let mut cursor = std::io::Cursor::new(fake_len.to_vec());
    let err = read_frame(&mut cursor).await.expect_err("should reject");
    assert!(err.to_string().contains("frame too large"));
}

#[tokio::test]
async fn read_frame_errors_on_truncated_payload() {
    let mut buf = Vec::new();
    buf.extend_from_slice(&10u32.to_be_bytes());
    buf.extend_from_slice(&[1_u8, 2, 3]);
    let mut cursor = std::io::Cursor::new(buf);
    let err = read_frame(&mut cursor)
        .await
        .expect_err("truncated payload should fail");
    assert!(
        err.to_string().contains("payload") || err.to_string().contains("read"),
        "{err}"
    );
}

#[tokio::test]
async fn handshake_rejects_malformed_client_hello_json() {
    let payload = b"{not-json";
    let mut input = Vec::new();
    input.extend_from_slice(&(payload.len() as u32).to_be_bytes());
    input.extend_from_slice(payload);
    let mut reader = std::io::Cursor::new(input);
    let mut writer = Vec::new();
    let err = perform_handshake(&mut reader, &mut writer, "fam")
        .await
        .expect_err("invalid JSON");
    let msg = err.to_string();
    assert!(
        msg.contains("ClientHello") || msg.contains("malformed"),
        "{msg}"
    );
    assert!(
        !writer.is_empty(),
        "error frame should be written for invalid ClientHello"
    );
}

#[tokio::test]
async fn read_frame_errors_on_truncated_length_prefix() {
    let mut cursor = std::io::Cursor::new(vec![0_u8, 1]);
    let err = read_frame(&mut cursor)
        .await
        .expect_err("incomplete length prefix");
    assert!(
        err.to_string().contains("frame length") || err.to_string().contains("read"),
        "{err}"
    );
}

#[tokio::test]
async fn write_error_frame_produces_json_error_object() {
    let mut out = Vec::new();
    write_error_frame(&mut out, "unit_test_reason").await;
    assert!(!out.is_empty());
    let mut cursor = std::io::Cursor::new(out);
    let payload = read_frame(&mut cursor).await.expect("read error frame");
    let v: Value = serde_json::from_slice(&payload).expect("valid JSON");
    assert_eq!(v["error"], "handshake_failed");
    assert_eq!(v["reason"], "unit_test_reason");
}

fn framed_json_bytes(value: &Value) -> Vec<u8> {
    let bytes = serde_json::to_vec(value).expect("serialize");
    let mut buf = Vec::with_capacity(4 + bytes.len());
    buf.extend_from_slice(&(bytes.len() as u32).to_be_bytes());
    buf.extend_from_slice(&bytes);
    buf
}

async fn run_mock_security_server(sock_path: std::path::PathBuf, scenario: MockSecurityScenario) {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    use tokio::net::UnixListener;

    let listener = UnixListener::bind(&sock_path).expect("bind mock security socket");
    for _ in 0..2 {
        let (mut stream, _) = listener.accept().await.expect("accept");
        let mut line = String::new();
        BufReader::new(&mut stream)
            .read_line(&mut line)
            .await
            .expect("read jsonrpc");
        let req: Value = serde_json::from_str(line.trim()).expect("request json");
        let method = req["method"].as_str().expect("method");
        let id = req["id"].as_u64().expect("id");
        let result = match (method, &scenario) {
            ("btsp.session.create", MockSecurityScenario::CreateMissingSessionToken) => {
                json!({"server_ephemeral_pub": "c2VydmVy", "challenge": "Y2hhbA=="})
            }
            ("btsp.session.create", MockSecurityScenario::CreateMissingServerPub) => {
                json!({"session_token": "tok-1", "challenge": "Y2hhbA=="})
            }
            ("btsp.session.create", _) => {
                json!({
                    "session_token": "tok-mock",
                    "server_ephemeral_pub": "c2VydmVyLWVwaGVtZXJhbA==",
                    "challenge": "Y2hhbGxlbmdlLWZyb20tYmVhcmRvZw==",
                })
            }
            ("btsp.session.verify", MockSecurityScenario::VerifyRejected) => {
                json!({"verified": false, "error": "bad response"})
            }
            ("btsp.session.verify", _) => {
                json!({"verified": true, "session_id": "sid-mock", "cipher": "chacha20_poly1305"})
            }
            _ => panic!("unexpected method {method}"),
        };
        let body = json!({"jsonrpc":"2.0","id": id, "result": result});
        let line_out = format!("{}\n", serde_json::to_string(&body).unwrap());
        stream.write_all(line_out.as_bytes()).await.expect("write");
        stream.flush().await.expect("flush");
    }
}

#[derive(Clone, Copy)]
enum MockSecurityScenario {
    Success,
    VerifyRejected,
    CreateMissingSessionToken,
    CreateMissingServerPub,
}

#[tokio::test]
async fn handshake_happy_path_with_mock_security_provider() {
    let dir = tempfile::tempdir().expect("tempdir");
    let sock = dir.path().join("security.sock");
    let sock_clone = sock.clone();
    let server = tokio::spawn(async move {
        run_mock_security_server(sock_clone, MockSecurityScenario::Success).await;
    });

    let hello = json!({"version": 1, "client_ephemeral_pub": "Y2xpZW50LWtleQ=="});
    let mut input = framed_json_bytes(&hello);
    let cr = json!({"response": "c2ln", "preferred_cipher": "chacha20_poly1305"});
    input.extend(framed_json_bytes(&cr));

    temp_env::async_with_vars(
        [
            ("SECURITY_SOCKET", Some(sock.to_str().expect("utf8 path"))),
            ("FAMILY_SEED", Some("dGVzdC1mYW1pbHktc2VlZA==")),
        ],
        async {
            tokio::task::yield_now().await;
            let mut reader = std::io::Cursor::new(input);
            let mut writer = Vec::new();
            let session = perform_handshake(&mut reader, &mut writer, "fam")
                .await
                .expect("handshake ok");
            assert_eq!(session.session_id, "sid-mock");
            assert_eq!(session.cipher, "chacha20_poly1305");
            assert!(session.encrypted);
            assert!(!writer.is_empty(), "ServerHello + HandshakeComplete frames");
        },
    )
    .await;
    server.abort();
}

#[tokio::test]
async fn handshake_fails_verification_writes_error_frame() {
    let dir = tempfile::tempdir().expect("tempdir");
    let sock = dir.path().join("sec2.sock");
    let sock_clone = sock.clone();
    let server = tokio::spawn(async move {
        run_mock_security_server(sock_clone, MockSecurityScenario::VerifyRejected).await;
    });

    let hello = json!({"version": 1, "client_ephemeral_pub": "Y2xpZW50"});
    let mut input = framed_json_bytes(&hello);
    let cr = json!({"response": "c2ln", "preferred_cipher": null});
    input.extend(framed_json_bytes(&cr));

    temp_env::async_with_vars(
        [
            ("SECURITY_SOCKET", Some(sock.to_str().unwrap())),
            ("FAMILY_SEED", Some("dGVzdA==")),
        ],
        async {
            tokio::task::yield_now().await;
            let mut reader = std::io::Cursor::new(input);
            let mut writer = Vec::new();
            let err = perform_handshake(&mut reader, &mut writer, "fam")
                .await
                .expect_err("verify rejected");
            assert!(
                err.to_string().contains("family") || err.to_string().contains("verification"),
                "{err}"
            );
            let mut cursor = std::io::Cursor::new(&writer);
            let mut found_family_err = false;
            while cursor.position() < writer.len() as u64 {
                let frame = read_frame(&mut cursor).await.ok();
                if let Some(bytes) = frame
                    && let Ok(v) = serde_json::from_slice::<Value>(&bytes)
                    && v["reason"] == "family_verification"
                {
                    found_family_err = true;
                }
            }
            assert!(found_family_err, "expected family_verification error frame");
        },
    )
    .await;
    server.abort();
}

#[tokio::test]
async fn handshake_create_missing_session_token_returns_internal_error() {
    let dir = tempfile::tempdir().expect("tempdir");
    let sock = dir.path().join("sec3.sock");
    let sock_clone = sock.clone();
    let server = tokio::spawn(async move {
        run_mock_security_server(sock_clone, MockSecurityScenario::CreateMissingSessionToken).await;
    });

    let hello = json!({"client_ephemeral_pub": "YQ=="});
    let input = framed_json_bytes(&hello);

    temp_env::async_with_vars(
        [
            ("SECURITY_SOCKET", Some(sock.to_str().unwrap())),
            ("FAMILY_SEED", Some("dGVzdA==")),
        ],
        async {
            tokio::task::yield_now().await;
            let mut reader = std::io::Cursor::new(input);
            let mut writer = Vec::new();
            let err = perform_handshake(&mut reader, &mut writer, "fam")
                .await
                .expect_err("missing session_token");
            assert!(
                err.to_string().contains("session_token"),
                "unexpected: {err}"
            );
        },
    )
    .await;
    server.abort();
}

#[tokio::test]
async fn handshake_create_missing_server_pub_returns_internal_error() {
    let dir = tempfile::tempdir().expect("tempdir");
    let sock = dir.path().join("sec4.sock");
    let sock_clone = sock.clone();
    let server = tokio::spawn(async move {
        run_mock_security_server(sock_clone, MockSecurityScenario::CreateMissingServerPub).await;
    });

    let hello = json!({"client_ephemeral_pub": "YQ=="});
    let input = framed_json_bytes(&hello);

    temp_env::async_with_vars(
        [
            ("SECURITY_SOCKET", Some(sock.to_str().unwrap())),
            ("FAMILY_SEED", Some("dGVzdA==")),
        ],
        async {
            tokio::task::yield_now().await;
            let mut reader = std::io::Cursor::new(input);
            let mut writer = Vec::new();
            let err = perform_handshake(&mut reader, &mut writer, "fam")
                .await
                .expect_err("missing server_ephemeral_pub");
            assert!(
                err.to_string().contains("server_ephemeral_pub"),
                "unexpected: {err}"
            );
        },
    )
    .await;
    server.abort();
}

#[tokio::test]
async fn handshake_rejects_malformed_challenge_response_json() {
    let dir = tempfile::tempdir().expect("tempdir");
    let sock = dir.path().join("sec6.sock");
    let sock_clone = sock.clone();
    let server = tokio::spawn(async move {
        run_mock_security_server(sock_clone, MockSecurityScenario::Success).await;
    });

    let hello = json!({"client_ephemeral_pub": "YQ=="});
    let mut input = framed_json_bytes(&hello);
    let bad = b"{oops";
    input.extend_from_slice(&(bad.len() as u32).to_be_bytes());
    input.extend_from_slice(bad);

    temp_env::async_with_vars(
        [
            ("SECURITY_SOCKET", Some(sock.to_str().unwrap())),
            ("FAMILY_SEED", Some("dGVzdA==")),
        ],
        async {
            tokio::task::yield_now().await;
            let mut reader = std::io::Cursor::new(input);
            let mut writer = Vec::new();
            let err = perform_handshake(&mut reader, &mut writer, "fam")
                .await
                .expect_err("bad challenge response");
            assert!(
                err.to_string().contains("ChallengeResponse")
                    || err.to_string().contains("malformed"),
                "{err}"
            );
        },
    )
    .await;
    server.abort();
}

#[test]
fn handshake_fails_when_security_provider_unavailable() {
    let hello = json!({"version": 1, "client_ephemeral_pub": "AAAA"});
    let hello_bytes = serde_json::to_vec(&hello).unwrap();
    let mut input = Vec::new();
    input.extend_from_slice(&(hello_bytes.len() as u32).to_be_bytes());
    input.extend_from_slice(&hello_bytes);

    temp_env::with_vars(
        [
            ("SECURITY_SOCKET", Some("/nonexistent/btsp-test.sock")),
            ("FAMILY_SEED", Some("dGVzdA==")),
        ],
        || {
            let rt = tokio::runtime::Runtime::new().expect("runtime");
            let mut reader = std::io::Cursor::new(&input);
            let mut writer = Vec::new();
            let result = rt.block_on(perform_handshake(&mut reader, &mut writer, "test-fam"));
            assert!(result.is_err());
            let msg = result.unwrap_err().to_string();
            assert!(
                msg.contains("security provider"),
                "error should mention security provider: {msg}"
            );
        },
    );
}

#[test]
fn handshake_fails_without_family_seed() {
    let hello = json!({"version": 1, "client_ephemeral_pub": "AAAA"});
    let hello_bytes = serde_json::to_vec(&hello).unwrap();
    let mut input = Vec::new();
    input.extend_from_slice(&(hello_bytes.len() as u32).to_be_bytes());
    input.extend_from_slice(&hello_bytes);

    temp_env::with_vars(
        [
            ("FAMILY_SEED", None::<&str>),
            ("SECURITY_SOCKET", Some("/nonexistent/btsp.sock")),
        ],
        || {
            let rt = tokio::runtime::Runtime::new().expect("runtime");
            let mut reader = std::io::Cursor::new(&input);
            let mut writer = Vec::new();
            let result = rt.block_on(perform_handshake(&mut reader, &mut writer, "fam"));
            assert!(result.is_err());
            let msg = result.unwrap_err().to_string();
            assert!(
                msg.contains("FAMILY_SEED"),
                "error should mention FAMILY_SEED: {msg}"
            );
        },
    );
}

// ── JSON-line framing tests ─────────────────────────────────────────────────

#[tokio::test]
async fn json_line_roundtrip() {
    let payload = br#"{"hello":"world"}"#;
    let mut buf = Vec::new();
    write_json_line(&mut buf, payload).await.expect("write");
    assert!(buf.ends_with(b"\n"));

    let mut cursor = BufReader::new(std::io::Cursor::new(buf));
    let read_back = read_json_line(&mut cursor).await.expect("read");
    assert_eq!(read_back, payload);
}

#[tokio::test]
async fn json_line_read_eof_returns_error() {
    let mut cursor = BufReader::new(std::io::Cursor::new(Vec::<u8>::new()));
    let err = read_json_line(&mut cursor)
        .await
        .expect_err("empty stream should fail");
    assert!(
        err.to_string().contains("closed") || err.to_string().contains("line"),
        "{err}"
    );
}

#[tokio::test]
async fn handshake_json_line_framing_happy_path() {
    let dir = tempfile::tempdir().expect("tempdir");
    let sock = dir.path().join("sec-jl.sock");
    let sock_clone = sock.clone();
    let server = tokio::spawn(async move {
        run_mock_security_server(sock_clone, MockSecurityScenario::Success).await;
    });

    let hello = json!({"version": 1, "client_ephemeral_pub": "Y2xpZW50LWtleQ=="});
    let cr = json!({"response": "c2ln", "preferred_cipher": "chacha20_poly1305"});
    let mut input = Vec::new();
    input.extend_from_slice(serde_json::to_string(&hello).unwrap().as_bytes());
    input.push(b'\n');
    input.extend_from_slice(serde_json::to_string(&cr).unwrap().as_bytes());
    input.push(b'\n');

    temp_env::async_with_vars(
        [
            ("SECURITY_SOCKET", Some(sock.to_str().expect("utf8"))),
            ("FAMILY_SEED", Some("dGVzdA==")),
        ],
        async {
            tokio::task::yield_now().await;
            let mut reader = std::io::Cursor::new(input);
            let mut writer = Vec::new();
            let session = perform_handshake(&mut reader, &mut writer, "fam")
                .await
                .expect("json-line handshake ok");
            assert_eq!(session.session_id, "sid-mock");
            assert_eq!(session.cipher, "chacha20_poly1305");
            assert!(session.encrypted);
            let output = String::from_utf8_lossy(&writer);
            assert!(
                output.contains("server_ephemeral_pub"),
                "ServerHello should appear in JSON-line output"
            );
        },
    )
    .await;
    server.abort();
}

#[tokio::test]
async fn handshake_json_line_with_session_token() {
    let dir = tempfile::tempdir().expect("tempdir");
    let sock = dir.path().join("sec-st.sock");
    let sock_clone = sock.clone();
    let server = tokio::spawn(async move {
        run_mock_security_server(sock_clone, MockSecurityScenario::Success).await;
    });

    let hello = json!({"version": 1, "client_ephemeral_pub": "Y2xpZW50"});
    let cr = json!({
        "response": "c2ln",
        "preferred_cipher": "chacha20_poly1305",
        "session_token": "tok-from-beardog"
    });
    let mut input = Vec::new();
    input.extend_from_slice(serde_json::to_string(&hello).unwrap().as_bytes());
    input.push(b'\n');
    input.extend_from_slice(serde_json::to_string(&cr).unwrap().as_bytes());
    input.push(b'\n');

    temp_env::async_with_vars(
        [
            ("SECURITY_SOCKET", Some(sock.to_str().expect("utf8"))),
            ("FAMILY_SEED", Some("dGVzdA==")),
        ],
        async {
            tokio::task::yield_now().await;
            let mut reader = std::io::Cursor::new(input);
            let mut writer = Vec::new();
            let session = perform_handshake(&mut reader, &mut writer, "fam")
                .await
                .expect("session_token handshake ok");
            assert_eq!(session.session_id, "sid-mock");
        },
    )
    .await;
    server.abort();
}
