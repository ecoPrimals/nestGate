// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! BTSP handshake client — JSON-RPC calls to the security capability provider.
//!
//! `NestGate` owns storage, not crypto. All BTSP session management is delegated
//! to the security capability provider via `btsp.session.create`,
//! `btsp.session.verify`, and `btsp.negotiate` JSON-RPC 2.0 methods.
//!
//! Pre-BTSP traffic uses newline-delimited JSON-RPC over a Unix domain socket.

use std::path::PathBuf;

use serde::Deserialize;
use serde_json::{Value, json};

use super::jsonrpc_client::JsonRpcClient;
use nestgate_types::error::{NestGateError, Result};

/// Final fallback path when [`resolve_security_socket_path`] exhausts all 5 higher-priority
/// tiers (env vars and `$XDG_RUNTIME_DIR/biomeos/` discovery).
pub const DEFAULT_SECURITY_SOCKET_PATH: &str = "/run/capability/security.sock";

/// Session lifecycle as reported by the security capability provider.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BtspSessionStatus {
    /// Session is active and usable.
    Active,
    /// Session failed; includes provider message.
    Failed(String),
    /// Session not yet established.
    Pending,
}

/// Outcome of [`BtspClient::initiate_handshake`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BtspHandshakeResult {
    /// Session identifier from the security capability provider.
    pub session_id: String,
    /// Negotiated or preferred cipher suite name (e.g. `chacha20`).
    pub cipher: String,
    /// Session state from the provider.
    pub status: BtspSessionStatus,
}

/// JSON-RPC caller for the security provider's BTSP methods over a Unix domain socket.
#[derive(Debug, Clone)]
pub struct BtspClient {
    security_socket: PathBuf,
}

/// Returns `true` when BTSP is mandatory.
///
/// Delegates to [`super::btsp_server_handshake::is_btsp_required`] so
/// client and server use identical env-var resolution and sentinel logic.
#[must_use]
pub fn is_btsp_required() -> bool {
    super::btsp_server_handshake::is_btsp_required()
}

/// Resolves the security capability provider's Unix socket path.
///
/// Precedence:
/// 1. `SECURITY_PROVIDER_SOCKET` env
/// 2. `CRYPTO_PROVIDER_SOCKET` env
/// 3. `SECURITY_SOCKET` env
/// 4. `SECURITY_ENDPOINT` if it is a local filesystem path (not a `scheme://` URL)
/// 5. Capability-scoped discovery: `$XDG_RUNTIME_DIR/biomeos/{security,crypto}.sock`
/// 6. [`DEFAULT_SECURITY_SOCKET_PATH`]
#[must_use]
pub fn resolve_security_socket_path() -> PathBuf {
    for var in [
        "SECURITY_PROVIDER_SOCKET",
        "CRYPTO_PROVIDER_SOCKET",
        "SECURITY_SOCKET",
    ] {
        if let Ok(p) = std::env::var(var)
            && !p.is_empty()
        {
            return PathBuf::from(p);
        }
    }
    if let Ok(p) = std::env::var("SECURITY_ENDPOINT")
        && !p.contains("://")
        && !p.is_empty()
    {
        return PathBuf::from(p);
    }
    if let Some(path) = discover_security_socket_xdg() {
        return path;
    }
    PathBuf::from(DEFAULT_SECURITY_SOCKET_PATH)
}

/// Capability socket names probed during XDG runtime directory discovery.
///
/// Names are capability-based (not primal-specific): `security.sock` is the
/// canonical name, `crypto.sock` is an alias accepted by some providers.
const SECURITY_SOCKET_CANDIDATES: &[&str] = &["security.sock", "crypto.sock"];

/// Scans `$XDG_RUNTIME_DIR/{socket_dir}/` for a security capability provider socket.
///
/// The socket subdirectory defaults to `biomeos` but can be overridden via
/// `ECOSYSTEM_SOCKET_DIR` for alternative deployment layouts.
fn discover_security_socket_xdg() -> Option<PathBuf> {
    let runtime_dir = std::env::var("XDG_RUNTIME_DIR").ok()?;
    let socket_dir =
        std::env::var("ECOSYSTEM_SOCKET_DIR").unwrap_or_else(|_| "biomeos".to_string());
    let base = PathBuf::from(runtime_dir).join(socket_dir);
    for name in SECURITY_SOCKET_CANDIDATES {
        let candidate = base.join(name);
        if candidate.exists() {
            return Some(candidate);
        }
    }
    None
}

impl BtspClient {
    /// Wraps a path to the security provider's JSON-RPC socket.
    #[must_use]
    pub const fn new(security_socket: PathBuf) -> Self {
        Self { security_socket }
    }

    /// Builds a client using [`resolve_security_socket_path`].
    #[must_use]
    pub fn from_env() -> Self {
        Self::new(resolve_security_socket_path())
    }

    fn socket_path_str(&self) -> Result<&str> {
        self.security_socket.to_str().ok_or_else(|| {
            NestGateError::validation_error(
                "SECURITY_SOCKET path is not valid UTF-8 (security IPC requires UTF-8 paths)",
            )
        })
    }

    /// Connects to the security provider, sends `btsp.session.create`, and returns the handshake outcome.
    ///
    /// # Errors
    ///
    /// Returns an error if the socket connection fails or the provider returns a malformed response.
    pub async fn initiate_handshake(&self, family_id: &str) -> Result<BtspHandshakeResult> {
        let mut client = JsonRpcClient::connect_unix(self.socket_path_str()?).await?;
        let result = client
            .call(
                "btsp.session.create",
                json!({
                    "family_id": family_id,
                    "preferred_cipher": "chacha20",
                }),
            )
            .await?;
        parse_handshake_result(&result)
    }

    /// Sends `btsp.session.verify` for an existing session.
    ///
    /// # Errors
    ///
    /// Returns an error if the socket connection fails or the response is unparseable.
    pub async fn verify_session(&self, session_id: &str) -> Result<bool> {
        let mut client = JsonRpcClient::connect_unix(self.socket_path_str()?).await?;
        let result = client
            .call("btsp.session.verify", json!({ "session_id": session_id }))
            .await?;
        parse_verify_result(&result)
    }

    /// Negotiates a cipher suite for the session via `btsp.negotiate`.
    ///
    /// # Errors
    ///
    /// Returns an error if the socket connection fails or the provider omits the cipher field.
    pub async fn negotiate_cipher(&self, session_id: &str, bond_type: &str) -> Result<String> {
        let mut client = JsonRpcClient::connect_unix(self.socket_path_str()?).await?;
        let result = client
            .call(
                "btsp.negotiate",
                json!({
                    "session_id": session_id,
                    "bond_type": bond_type,
                    "supported_ciphers": ["chacha20", "hmac_plain"],
                }),
            )
            .await?;
        parse_cipher_name(&result)
    }
}

fn parse_handshake_result(result: &Value) -> Result<BtspHandshakeResult> {
    let session_id = result
        .get("session_id")
        .or_else(|| result.get("id"))
        .and_then(Value::as_str)
        .map(String::from)
        .ok_or_else(|| {
            NestGateError::api_internal_error("btsp.session.create: missing session_id in result")
        })?;
    let cipher = result
        .get("cipher")
        .or_else(|| result.get("negotiated_cipher"))
        .and_then(Value::as_str)
        .unwrap_or("chacha20")
        .to_string();
    let status = parse_status_value(result.get("status"));
    Ok(BtspHandshakeResult {
        session_id,
        cipher,
        status,
    })
}

fn parse_status_value(v: Option<&Value>) -> BtspSessionStatus {
    let Some(val) = v else {
        return BtspSessionStatus::Pending;
    };
    if let Some(s) = val.as_str() {
        return match s.to_ascii_lowercase().as_str() {
            "active" => BtspSessionStatus::Active,
            "failed" => BtspSessionStatus::Failed("failed".to_string()),
            _ => BtspSessionStatus::Pending,
        };
    }
    if let Ok(obj) = serde_json::from_value::<StatusObj>(val.clone()) {
        return match obj.state.to_ascii_lowercase().as_str() {
            "active" => BtspSessionStatus::Active,
            "failed" => BtspSessionStatus::Failed(obj.message.unwrap_or_default()),
            _ => BtspSessionStatus::Pending,
        };
    }
    BtspSessionStatus::Pending
}

#[derive(Deserialize)]
struct StatusObj {
    state: String,
    message: Option<String>,
}

fn parse_verify_result(result: &Value) -> Result<bool> {
    if let Some(b) = result.get("valid").and_then(Value::as_bool) {
        return Ok(b);
    }
    if let Some(b) = result.get("ok").and_then(Value::as_bool) {
        return Ok(b);
    }
    if let Some(s) = result.get("status").and_then(Value::as_str) {
        return Ok(s.eq_ignore_ascii_case("valid") || s.eq_ignore_ascii_case("active"));
    }
    Err(NestGateError::api_internal_error(
        "btsp.session.verify: expected valid/ok/status in result",
    ))
}

fn parse_cipher_name(result: &Value) -> Result<String> {
    result
        .get("cipher")
        .or_else(|| result.get("negotiated_cipher"))
        .and_then(Value::as_str)
        .map(String::from)
        .ok_or_else(|| {
            NestGateError::api_internal_error("btsp.negotiate: missing cipher in result")
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    use tokio::net::UnixListener;

    fn clear_all_security_vars() -> Vec<(&'static str, Option<&'static str>)> {
        vec![
            ("SECURITY_PROVIDER_SOCKET", None),
            ("CRYPTO_PROVIDER_SOCKET", None),
            ("SECURITY_SOCKET", None),
            ("SECURITY_ENDPOINT", None),
            ("XDG_RUNTIME_DIR", None),
        ]
    }

    #[test]
    fn resolve_security_provider_socket_wins() {
        let mut vars = clear_all_security_vars();
        vars.push(("SECURITY_PROVIDER_SOCKET", Some("/provider/sec.sock")));
        vars.push(("SECURITY_SOCKET", Some("/old/sec.sock")));
        temp_env::with_vars(vars, || {
            assert_eq!(
                resolve_security_socket_path(),
                PathBuf::from("/provider/sec.sock")
            );
        });
    }

    #[test]
    fn resolve_crypto_provider_socket_second() {
        let mut vars = clear_all_security_vars();
        vars.push(("CRYPTO_PROVIDER_SOCKET", Some("/crypto/sec.sock")));
        vars.push(("SECURITY_SOCKET", Some("/old/sec.sock")));
        temp_env::with_vars(vars, || {
            assert_eq!(
                resolve_security_socket_path(),
                PathBuf::from("/crypto/sec.sock")
            );
        });
    }

    #[test]
    fn resolve_security_socket_env_order() {
        let mut vars = clear_all_security_vars();
        vars.push(("SECURITY_SOCKET", Some("/sock/a")));
        vars.push(("SECURITY_ENDPOINT", Some("/sock/b")));
        temp_env::with_vars(vars, || {
            assert_eq!(resolve_security_socket_path(), PathBuf::from("/sock/a"));
        });
    }

    #[test]
    fn resolve_security_endpoint_skips_url() {
        let mut vars = clear_all_security_vars();
        vars.push(("SECURITY_ENDPOINT", Some("http://127.0.0.1:9")));
        temp_env::with_vars(vars, || {
            assert_eq!(
                resolve_security_socket_path(),
                std::path::Path::new(DEFAULT_SECURITY_SOCKET_PATH)
            );
        });
    }

    #[test]
    fn resolve_xdg_discovery_finds_security_sock() {
        let dir = tempfile::tempdir().expect("tempdir");
        let biomeos = dir.path().join("biomeos");
        std::fs::create_dir_all(&biomeos).unwrap();
        std::fs::write(biomeos.join("security.sock"), "").unwrap();

        let xdg_str = dir.path().to_str().unwrap().to_string();
        let mut vars = clear_all_security_vars();
        vars.push(("XDG_RUNTIME_DIR", Some(xdg_str.as_str())));
        temp_env::with_vars(vars, || {
            assert_eq!(
                resolve_security_socket_path(),
                biomeos.join("security.sock")
            );
        });
    }

    #[test]
    fn resolve_xdg_discovery_finds_crypto_sock() {
        let dir = tempfile::tempdir().expect("tempdir");
        let biomeos = dir.path().join("biomeos");
        std::fs::create_dir_all(&biomeos).unwrap();
        std::fs::write(biomeos.join("crypto.sock"), "").unwrap();

        let xdg_str = dir.path().to_str().unwrap().to_string();
        let mut vars = clear_all_security_vars();
        vars.push(("XDG_RUNTIME_DIR", Some(xdg_str.as_str())));
        temp_env::with_vars(vars, || {
            assert_eq!(resolve_security_socket_path(), biomeos.join("crypto.sock"));
        });
    }

    #[test]
    fn resolve_empty_env_skipped() {
        let mut vars = clear_all_security_vars();
        vars.push(("SECURITY_PROVIDER_SOCKET", Some("")));
        vars.push(("SECURITY_SOCKET", Some("/real.sock")));
        temp_env::with_vars(vars, || {
            assert_eq!(resolve_security_socket_path(), PathBuf::from("/real.sock"));
        });
    }

    #[test]
    fn is_btsp_required_respects_family_and_insecure() {
        temp_env::with_vars(
            [("FAMILY_ID", None::<&str>), ("BIOMEOS_INSECURE", None)],
            || assert!(!is_btsp_required()),
        );
        temp_env::with_vars(
            [("FAMILY_ID", Some("prod")), ("BIOMEOS_INSECURE", None)],
            || assert!(is_btsp_required()),
        );
        temp_env::with_vars(
            [("FAMILY_ID", Some("prod")), ("BIOMEOS_INSECURE", Some("1"))],
            || assert!(!is_btsp_required()),
        );
        temp_env::with_vars(
            [("FAMILY_ID", Some("default")), ("BIOMEOS_INSECURE", None)],
            || assert!(!is_btsp_required()),
        );
    }

    #[tokio::test]
    async fn initiate_handshake_roundtrip() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("btsp-mock.sock");
        let listener = UnixListener::bind(&path).expect("bind");
        let server = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.expect("accept");
            let (rh, mut wh) = tokio::io::split(stream);
            let mut line = String::new();
            let mut br = BufReader::new(rh);
            br.read_line(&mut line).await.expect("read req");
            let req: Value = serde_json::from_str(line.trim()).expect("json");
            assert_eq!(req["method"], "btsp.session.create");
            assert_eq!(req["params"]["family_id"], "fam-x");
            assert_eq!(req["params"]["preferred_cipher"], "chacha20");
            let resp = json!({
                "jsonrpc": "2.0",
                "result": {
                    "session_id": "sess-42",
                    "cipher": "chacha20",
                    "status": "active"
                },
                "id": req["id"],
            });
            wh.write_all(format!("{}\n", serde_json::to_string(&resp).unwrap()).as_bytes())
                .await
                .unwrap();
        });

        let client = BtspClient::new(path);
        let r = client.initiate_handshake("fam-x").await.expect("handshake");
        assert_eq!(r.session_id, "sess-42");
        assert_eq!(r.cipher, "chacha20");
        assert_eq!(r.status, BtspSessionStatus::Active);
        server.await.expect("server");
    }

    #[tokio::test]
    async fn verify_and_negotiate_roundtrip() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("btsp-mock2.sock");
        let listener = UnixListener::bind(&path).expect("bind");
        let server = tokio::spawn(async move {
            for expected in ["btsp.session.verify", "btsp.negotiate"] {
                let (stream, _) = listener.accept().await.expect("accept");
                let (rh, mut wh) = tokio::io::split(stream);
                let mut line = String::new();
                let mut br = BufReader::new(rh);
                br.read_line(&mut line).await.expect("read");
                let req: Value = serde_json::from_str(line.trim()).unwrap();
                assert_eq!(req["method"], expected);
                let body = if expected == "btsp.session.verify" {
                    json!({"jsonrpc":"2.0","result":{"valid":true},"id":req["id"]})
                } else {
                    json!({"jsonrpc":"2.0","result":{"cipher":"hmac_plain"},"id":req["id"]})
                };
                wh.write_all(format!("{}\n", serde_json::to_string(&body).unwrap()).as_bytes())
                    .await
                    .unwrap();
            }
        });

        let c = BtspClient::new(path);
        assert!(c.verify_session("s1").await.expect("verify"));
        assert_eq!(
            c.negotiate_cipher("s1", "peer").await.expect("negotiate"),
            "hmac_plain"
        );
        server.await.expect("server");
    }

    #[tokio::test]
    async fn connect_missing_socket_errors() {
        let client = BtspClient::new(PathBuf::from(
            "/nonexistent/nestgate/btsp_client_missing.sock",
        ));
        let e = client.initiate_handshake("x").await.unwrap_err();
        assert!(e.to_string().contains("Failed to connect") || e.to_string().contains("connect"));
    }
}
