// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! # Crypto delegation — external provider integration
//!
//! Capability-based delegation to an external crypto provider.
//!
//! ## Philosophy (Primal Sovereignty)
//!
//! - **Self-Knowledge**: NestGate knows it needs the `"crypto"` capability, not a named peer
//! - **Runtime Discovery**: Find crypto provider dynamically
//! - **Capability-Based**: Discover by capability, not hardcoded name
//! - **Zero Hardcoding**: Any provider advertising the `"crypto"` capability works
//!
//! ## Architecture
//!
//! ```text
//! NestGate (needs crypto)
//!   → CryptoDelegate::new()
//!   → CapabilityDiscovery::find("crypto")
//!   → Connect to discovered crypto provider
//!   → Delegate crypto.* operations via JSON-RPC
//! ```
//!
//! ## Semantic Method Mapping
//!
//! | Operation | JSON-RPC Method |
//! |-----------|-----------------|
//! | encrypt | `crypto.encrypt` |
//! | decrypt | `crypto.decrypt` |
//! | generate_key | `crypto.generate_key` |
//! | generate_nonce | `crypto.generate_nonce` |
//! | hash | `crypto.hash` |
//! | verify_hash | `crypto.verify_hash` |

use crate::crypto::{EncryptedData, EncryptionAlgorithm, EncryptionParams};
use nestgate_discovery::capability_discovery::{CapabilityDiscovery, ServiceEndpoint};
use nestgate_rpc::rpc::JsonRpcClient;
use nestgate_types::{NestGateError, Result};
use serde_json::{Value, json};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
use tracing::{debug, info};

/// Base64 helpers using the `base64` crate already in our dependency tree.
mod b64 {
    use base64::Engine;

    pub fn encode(data: &[u8]) -> String {
        base64::engine::general_purpose::STANDARD.encode(data)
    }

    pub fn decode(data: &str) -> nestgate_types::Result<Vec<u8>> {
        base64::engine::general_purpose::STANDARD
            .decode(data)
            .map_err(|e| nestgate_types::NestGateError::api_error(format!("base64 decode: {e}")))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn encode_decode_roundtrip() {
            let raw = b"\x00hello\xff";
            let s = encode(raw);
            assert_eq!(decode(&s).unwrap(), raw);
        }

        #[test]
        fn decode_invalid_errors() {
            assert!(decode("@@@").is_err());
        }
    }
}

/// Crypto operations delegator to an external crypto service.
///
/// Discovers a primal that advertises the "crypto" capability and delegates
/// all cryptographic operations to it via JSON-RPC semantic methods.
pub struct CryptoDelegate {
    /// JSON-RPC client to the crypto provider (mutable for `call()`)
    client: Mutex<JsonRpcClient>,
    /// Endpoint information (for logging/debugging)
    endpoint: ServiceEndpoint,
}

impl CryptoDelegate {
    /// Create new crypto delegate by discovering a crypto provider.
    ///
    /// 1. Discover orchestration IPC gateway (registry bootstrap)
    /// 2. Query for services providing "crypto" capability
    /// 3. Connect to the first available crypto provider
    pub async fn new() -> Result<Self> {
        info!("Discovering crypto provider via capability-based discovery");

        let ipc_gateway = CapabilityDiscovery::discover_orchestration_ipc()
            .await
            .map_err(|e| {
                NestGateError::internal_error(
                    format!("Failed to discover IPC gateway: {e}"),
                    "crypto_delegate",
                )
            })?;

        let mut discovery = CapabilityDiscovery::new(ipc_gateway);

        let endpoint = discovery.find("crypto").await.map_err(|e| {
            NestGateError::internal_error(
                format!("No crypto provider found: {e}"),
                "crypto_delegate",
            )
        })?;

        info!(
            "Found crypto provider: {} at {}",
            endpoint.name, endpoint.endpoint
        );

        let client = JsonRpcClient::connect_unix(&endpoint.endpoint).await?;

        Ok(Self {
            client: Mutex::new(client),
            endpoint,
        })
    }

    /// Create delegate with an explicit Unix socket path (for testing).
    pub async fn with_endpoint(path: &str) -> Result<Self> {
        debug!("Connecting directly to crypto provider: {path}");

        let client = JsonRpcClient::connect_unix(path).await?;

        Ok(Self {
            client: Mutex::new(client),
            endpoint: ServiceEndpoint {
                capability: "crypto".to_string(),
                name: "crypto-provider".to_string(),
                endpoint: path.to_string(),
                version: "unknown".to_string(),
                discovered_at: std::time::Instant::now(),
            },
        })
    }

    /// Get crypto provider information.
    pub const fn provider_info(&self) -> &ServiceEndpoint {
        &self.endpoint
    }

    /// Encrypt data — delegates to `crypto.encrypt`.
    pub async fn encrypt(
        &self,
        plaintext: &[u8],
        params: &EncryptionParams,
    ) -> Result<EncryptedData> {
        let algo_str = match params.algorithm {
            EncryptionAlgorithm::Aes256Gcm => "aes256gcm",
            EncryptionAlgorithm::ChaCha20Poly1305 => "chacha20poly1305",
        };

        let request = json!({
            "plaintext": b64::encode(plaintext),
            "algorithm": algo_str,
            "associated_data": b64::encode(&params.associated_data),
        });

        let response = self
            .client
            .lock()
            .await
            .call("crypto.encrypt", request)
            .await?;

        let ciphertext = b64::decode(
            response["ciphertext"]
                .as_str()
                .ok_or_else(|| NestGateError::api_error("Missing ciphertext in response"))?,
        )?;

        let nonce = b64::decode(
            response["nonce"]
                .as_str()
                .ok_or_else(|| NestGateError::api_error("Missing nonce in response"))?,
        )?;

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Ok(EncryptedData {
            ciphertext,
            nonce,
            algorithm: params.algorithm,
            timestamp,
        })
    }

    /// Decrypt data — delegates to `crypto.decrypt`.
    pub async fn decrypt(&self, encrypted: &EncryptedData) -> Result<Vec<u8>> {
        let algo_str = match encrypted.algorithm {
            EncryptionAlgorithm::Aes256Gcm => "aes256gcm",
            EncryptionAlgorithm::ChaCha20Poly1305 => "chacha20poly1305",
        };

        let request = json!({
            "ciphertext": b64::encode(&encrypted.ciphertext),
            "nonce": b64::encode(&encrypted.nonce),
            "algorithm": algo_str,
        });

        let response = self
            .client
            .lock()
            .await
            .call("crypto.decrypt", request)
            .await?;

        b64::decode(
            response["plaintext"]
                .as_str()
                .ok_or_else(|| NestGateError::api_error("Missing plaintext in response"))?,
        )
    }

    /// Generate a secure random key — delegates to `crypto.generate_key`.
    pub async fn generate_key(&self, length: usize) -> Result<Vec<u8>> {
        let response = self
            .client
            .lock()
            .await
            .call("crypto.generate_key", json!({ "length": length }))
            .await?;

        let key = b64::decode(
            response["key"]
                .as_str()
                .ok_or_else(|| NestGateError::api_error("Missing key in response"))?,
        )?;

        if key.len() != length {
            return Err(NestGateError::api_error(format!(
                "Key length mismatch: expected {length}, got {}",
                key.len()
            )));
        }

        Ok(key)
    }

    /// Generate a secure random nonce — delegates to `crypto.generate_nonce`.
    pub async fn generate_nonce(&self, algorithm: EncryptionAlgorithm) -> Result<Vec<u8>> {
        let algo_str = match algorithm {
            EncryptionAlgorithm::Aes256Gcm => "aes256gcm",
            EncryptionAlgorithm::ChaCha20Poly1305 => "chacha20poly1305",
        };

        let response = self
            .client
            .lock()
            .await
            .call("crypto.generate_nonce", json!({ "algorithm": algo_str }))
            .await?;

        b64::decode(
            response["nonce"]
                .as_str()
                .ok_or_else(|| NestGateError::api_error("Missing nonce in response"))?,
        )
    }

    /// Hash data — delegates to `crypto.hash`.
    pub async fn hash(&self, data: &[u8], algorithm: &str) -> Result<Vec<u8>> {
        let response = self
            .client
            .lock()
            .await
            .call(
                "crypto.hash",
                json!({
                    "data": b64::encode(data),
                    "algorithm": algorithm,
                }),
            )
            .await?;

        b64::decode(
            response["hash"]
                .as_str()
                .ok_or_else(|| NestGateError::api_error("Missing hash in response"))?,
        )
    }

    /// Verify a hash — delegates to `crypto.verify_hash`.
    pub async fn verify_hash(&self, data: &[u8], hash: &[u8], algorithm: &str) -> Result<bool> {
        let response = self
            .client
            .lock()
            .await
            .call(
                "crypto.verify_hash",
                json!({
                    "data": b64::encode(data),
                    "hash": b64::encode(hash),
                    "algorithm": algorithm,
                }),
            )
            .await?;

        response["valid"]
            .as_bool()
            .ok_or_else(|| NestGateError::api_error("Missing 'valid' in response"))
    }

    /// Health check on the crypto provider.
    pub async fn health_check(&self) -> Result<Value> {
        self.client
            .lock()
            .await
            .call("health.check", json!({}))
            .await
    }

    /// Sign a JWT token — delegates to `crypto.sign_jwt`.
    pub async fn sign_jwt(&self, claims_json: &str, algorithm: &str) -> Result<String> {
        let response = self
            .client
            .lock()
            .await
            .call(
                "crypto.sign_jwt",
                json!({
                    "claims": claims_json,
                    "algorithm": algorithm,
                }),
            )
            .await?;

        response["token"]
            .as_str()
            .map(String::from)
            .ok_or_else(|| NestGateError::api_error("Missing 'token' in sign_jwt response"))
    }

    /// Verify and decode a JWT token — delegates to `crypto.verify_jwt`.
    pub async fn verify_jwt(&self, token: &str, algorithm: &str) -> Result<String> {
        let response = self
            .client
            .lock()
            .await
            .call(
                "crypto.verify_jwt",
                json!({
                    "token": token,
                    "algorithm": algorithm,
                }),
            )
            .await?;

        response["claims"]
            .as_str()
            .map(String::from)
            .ok_or_else(|| NestGateError::api_error("Missing 'claims' in verify_jwt response"))
    }

    /// Verify a password against an Argon2 hash — delegates to `crypto.verify_password`.
    pub async fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        let response = self
            .client
            .lock()
            .await
            .call(
                "crypto.verify_password",
                json!({
                    "password": password,
                    "hash": hash,
                }),
            )
            .await?;

        response["valid"]
            .as_bool()
            .ok_or_else(|| NestGateError::api_error("Missing 'valid' in verify_password response"))
    }

    /// Compute HMAC-SHA256 — delegates to `crypto.hmac`.
    pub async fn hmac_sign(&self, key: &[u8], message: &[u8]) -> Result<Vec<u8>> {
        let response = self
            .client
            .lock()
            .await
            .call(
                "crypto.hmac",
                json!({
                    "key": b64::encode(key),
                    "message": b64::encode(message),
                    "algorithm": "sha256",
                }),
            )
            .await?;

        b64::decode(
            response["mac"]
                .as_str()
                .ok_or_else(|| NestGateError::api_error("Missing 'mac' in hmac response"))?,
        )
    }

    /// Verify HMAC-SHA256 — delegates to `crypto.hmac_verify`.
    pub async fn hmac_verify(&self, key: &[u8], message: &[u8], mac: &[u8]) -> Result<bool> {
        let response = self
            .client
            .lock()
            .await
            .call(
                "crypto.hmac_verify",
                json!({
                    "key": b64::encode(key),
                    "message": b64::encode(message),
                    "mac": b64::encode(mac),
                    "algorithm": "sha256",
                }),
            )
            .await?;

        response["valid"]
            .as_bool()
            .ok_or_else(|| NestGateError::api_error("Missing 'valid' in hmac_verify response"))
    }

    /// Generate cryptographically secure random bytes — delegates to `crypto.random_bytes`.
    pub async fn random_bytes(&self, length: usize) -> Result<Vec<u8>> {
        let response = self
            .client
            .lock()
            .await
            .call("crypto.random_bytes", json!({ "length": length }))
            .await?;

        b64::decode(
            response["bytes"].as_str().ok_or_else(|| {
                NestGateError::api_error("Missing 'bytes' in random_bytes response")
            })?,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[cfg(unix)]
    mod unix_crypto_mock {
        use super::super::CryptoDelegate;
        use base64::Engine;
        use serde_json::{Value, json};
        use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, split};
        use tokio::net::{UnixListener, UnixStream};

        /// Happy-path JSON-RPC `result` payloads for crypto semantic methods.
        pub fn dispatch(method: &str, params: &Value) -> Value {
            match method {
                "crypto.encrypt" => {
                    let pt_b = params["plaintext"].as_str().expect("pt");
                    let pt = base64::engine::general_purpose::STANDARD
                        .decode(pt_b)
                        .expect("b64");
                    json!({
                        "ciphertext": base64::engine::general_purpose::STANDARD.encode(&pt),
                        "nonce": base64::engine::general_purpose::STANDARD.encode(b"123456789012"),
                    })
                }
                "crypto.decrypt" => {
                    let ct_b = params["ciphertext"].as_str().expect("ct");
                    let ct = base64::engine::general_purpose::STANDARD
                        .decode(ct_b)
                        .expect("ct dec");
                    json!({
                        "plaintext": base64::engine::general_purpose::STANDARD.encode(ct),
                    })
                }
                "crypto.generate_key" => {
                    let len_u64 = params["length"].as_u64().unwrap_or(16);
                    let len = usize::try_from(len_u64).unwrap_or(16);
                    json!({
                        "key": base64::engine::general_purpose::STANDARD.encode(vec![7u8; len]),
                    })
                }
                "crypto.generate_nonce" => json!({
                    "nonce": base64::engine::general_purpose::STANDARD.encode(b"123456789012"),
                }),
                "crypto.hash" => {
                    let d = params["data"].as_str().expect("data");
                    let raw = base64::engine::general_purpose::STANDARD
                        .decode(d)
                        .expect("dec");
                    json!({
                        "hash": base64::engine::general_purpose::STANDARD.encode(&raw),
                    })
                }
                "crypto.verify_hash" => json!({ "valid": true }),
                "crypto.sign_jwt" => json!({ "token": "eyJhbGciOiJIUzI1NiJ9.mock.sig" }),
                "crypto.verify_jwt" => json!({ "claims": "{\"sub\":\"test\"}" }),
                "crypto.verify_password" => json!({ "valid": true }),
                "crypto.hmac" => json!({
                    "mac": base64::engine::general_purpose::STANDARD.encode(b"hmac-bytes"),
                }),
                "crypto.hmac_verify" => json!({ "valid": true }),
                "crypto.random_bytes" => json!({
                    "bytes": base64::engine::general_purpose::STANDARD.encode(b"randomness"),
                }),
                "health.check" => json!({ "ok": true }),
                _ => json!({ "unknown": method }),
            }
        }

        /// Run `CryptoDelegate::with_endpoint` against a mock server; `dispatch` builds each `result`.
        pub async fn with_mock_delegate<F, Fut, D>(dispatch: D, f: F)
        where
            D: Fn(&str, &Value) -> Value + Send + 'static,
            F: FnOnce(CryptoDelegate) -> Fut,
            Fut: std::future::Future<Output = ()>,
        {
            let dir = tempfile::tempdir().expect("tmpdir");
            let sock_path = dir.path().join("crypto.sock");
            let _ = std::fs::remove_file(&sock_path);
            let listener = UnixListener::bind(&sock_path).expect("bind");
            let path_str = sock_path.to_string_lossy().to_string();

            let server = tokio::spawn(async move {
                let (stream, _) = listener.accept().await.expect("accept");
                let (read_half, mut write_half) = split(stream);
                let mut reader = BufReader::new(read_half);
                let mut line = String::new();
                loop {
                    line.clear();
                    let n = reader.read_line(&mut line).await.expect("read");
                    if n == 0 {
                        break;
                    }
                    let req: Value = serde_json::from_str(line.trim()).expect("req");
                    let id = req["id"].as_u64().unwrap_or(1);
                    let method = req["method"].as_str().unwrap_or("");
                    let params = &req["params"];
                    let result = dispatch(method, params);
                    let resp = json!({
                        "jsonrpc": "2.0",
                        "result": result,
                        "id": id,
                    });
                    let mut out = resp.to_string();
                    out.push('\n');
                    write_half.write_all(out.as_bytes()).await.expect("write");
                    write_half.flush().await.expect("flush");
                }
            });

            let delegate = CryptoDelegate::with_endpoint(&path_str)
                .await
                .expect("delegate");
            f(delegate).await;

            if let Ok(c) = UnixStream::connect(&path_str).await {
                drop(c);
            }
            server.abort();
        }
    }

    #[test]
    fn test_encryption_params_default() {
        let params = EncryptionParams::default();
        assert_eq!(params.algorithm, EncryptionAlgorithm::Aes256Gcm);
        assert!(params.associated_data.is_empty());
    }

    #[test]
    fn encryption_params_chacha_roundtrip_serde() {
        let p = EncryptionParams {
            algorithm: EncryptionAlgorithm::ChaCha20Poly1305,
            associated_data: b"ad".to_vec(),
        };
        let json = serde_json::to_string(&p).unwrap();
        let back: EncryptionParams = serde_json::from_str(&json).unwrap();
        assert_eq!(back.algorithm, EncryptionAlgorithm::ChaCha20Poly1305);
        assert_eq!(back.associated_data, b"ad");
    }

    #[test]
    fn encrypted_data_fields_roundtrip_serde() {
        let e = EncryptedData {
            ciphertext: vec![1, 2, 3],
            nonce: vec![9],
            algorithm: EncryptionAlgorithm::ChaCha20Poly1305,
            timestamp: 42,
        };
        let json = serde_json::to_string(&e).unwrap();
        let back: EncryptedData = serde_json::from_str(&json).unwrap();
        assert_eq!(back.timestamp, 42);
        assert_eq!(back.algorithm, EncryptionAlgorithm::ChaCha20Poly1305);
    }

    #[tokio::test]
    #[ignore = "requires running crypto capability provider"]
    async fn test_crypto_delegate_discovery() {
        let result = CryptoDelegate::new().await;
        assert!(result.is_ok(), "Should discover crypto provider");

        let delegate = result.unwrap();
        assert_eq!(delegate.endpoint.capability, "crypto");
    }

    #[tokio::test]
    #[ignore = "requires running crypto capability provider"]
    async fn test_crypto_delegate_encrypt_decrypt() {
        let delegate = CryptoDelegate::new().await.unwrap();

        let plaintext = b"Hello, crypto provider!";
        let params = EncryptionParams::default();

        let encrypted = delegate.encrypt(plaintext, &params).await.unwrap();
        assert!(!encrypted.ciphertext.is_empty());
        assert!(!encrypted.nonce.is_empty());

        let decrypted = delegate.decrypt(&encrypted).await.unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[tokio::test]
    #[ignore = "requires running crypto capability provider"]
    async fn test_crypto_delegate_key_generation() {
        let delegate = CryptoDelegate::new().await.unwrap();

        let key = delegate.generate_key(32).await.unwrap();
        assert_eq!(key.len(), 32);
    }

    #[tokio::test]
    async fn with_endpoint_invalid_socket_fails_fast() {
        assert!(
            CryptoDelegate::with_endpoint("/nonexistent/nestgate-crypto.sock")
                .await
                .is_err()
        );
    }

    #[tokio::test]
    #[cfg(unix)]
    async fn crypto_delegate_roundtrip_over_mock_unix_jsonrpc() {
        use unix_crypto_mock::{dispatch, with_mock_delegate};

        with_mock_delegate(dispatch, |delegate| async move {
            assert_eq!(delegate.provider_info().capability, "crypto");
            assert_eq!(delegate.provider_info().name, "crypto-provider");

            let params = crate::crypto::EncryptionParams::default();
            let plain = b"hello-mock";
            let enc = delegate.encrypt(plain, &params).await.expect("encrypt");
            let dec = delegate.decrypt(&enc).await.expect("decrypt");
            assert_eq!(dec, plain);

            let key = delegate.generate_key(16).await.expect("gen key");
            assert_eq!(key.len(), 16);
            assert!(key.iter().all(|b| *b == 7));

            let nonce = delegate
                .generate_nonce(crate::crypto::EncryptionAlgorithm::Aes256Gcm)
                .await
                .expect("nonce");
            assert_eq!(nonce.len(), 12);

            let h = delegate.hash(b"data", "sha256").await.expect("hash");
            assert_eq!(h, b"data");

            let ok = delegate
                .verify_hash(b"data", b"data", "sha256")
                .await
                .expect("vh");
            assert!(ok);

            let hc = delegate.health_check().await.expect("health");
            assert_eq!(hc["ok"], true);

            let tok = delegate.sign_jwt("{}", "HS256").await.expect("sign_jwt");
            assert!(tok.contains("mock"));

            let claims = delegate
                .verify_jwt(&tok, "HS256")
                .await
                .expect("verify_jwt");
            assert!(claims.contains("sub"));

            assert!(
                delegate
                    .verify_password("pw", "argon2hash")
                    .await
                    .expect("verify_password")
            );

            let mac = delegate.hmac_sign(b"key", b"msg").await.expect("hmac");
            assert_eq!(mac, b"hmac-bytes");

            assert!(
                delegate
                    .hmac_verify(b"key", b"msg", &mac)
                    .await
                    .expect("hmac_verify")
            );

            let rnd = delegate.random_bytes(10).await.expect("random_bytes");
            assert_eq!(rnd, b"randomness");
        })
        .await;
    }

    #[tokio::test]
    #[cfg(unix)]
    async fn crypto_delegate_chacha_roundtrip_over_mock() {
        use unix_crypto_mock::{dispatch, with_mock_delegate};

        with_mock_delegate(dispatch, |delegate| async move {
            let params = EncryptionParams {
                algorithm: EncryptionAlgorithm::ChaCha20Poly1305,
                associated_data: b"aad".to_vec(),
            };
            let plain = b"chacha-plain";
            let enc = delegate.encrypt(plain, &params).await.expect("encrypt");
            assert_eq!(enc.algorithm, EncryptionAlgorithm::ChaCha20Poly1305);
            let dec = delegate.decrypt(&enc).await.expect("decrypt");
            assert_eq!(dec, plain);

            let n = delegate
                .generate_nonce(EncryptionAlgorithm::ChaCha20Poly1305)
                .await
                .expect("nonce");
            assert_eq!(n.len(), 12);
        })
        .await;
    }

    #[tokio::test]
    #[cfg(unix)]
    async fn crypto_delegate_new_errors_when_ipc_gateway_unreachable() {
        let dir = tempfile::tempdir().expect("tmpdir");
        let absent = dir.path().join("definitely_no_orchestration.sock");
        let _ = std::fs::remove_file(&absent);

        temp_env::async_with_vars(
            [
                ("NESTGATE_ORCHESTRATION_IPC_PATH", None::<&str>),
                ("ORCHESTRATION_IPC_PATH", None::<&str>),
                (
                    "ORCHESTRATION_IPC_STANDARD_PATH",
                    Some(absent.to_string_lossy().as_ref()),
                ),
            ],
            async {
                let err = match CryptoDelegate::new().await {
                    Err(e) => e,
                    Ok(_) => panic!("expected discovery to fail"),
                };
                let s = err.to_string();
                assert!(
                    s.contains("Failed to discover IPC gateway")
                        || s.contains("IPC gateway not found")
                        || s.contains("orchestration"),
                    "unexpected error: {s}"
                );
            },
        )
        .await;
    }

    #[tokio::test]
    #[cfg(unix)]
    async fn crypto_delegate_new_errors_when_capability_discovery_returns_empty() {
        use tokio::net::UnixListener;

        let dir = tempfile::tempdir().expect("tmpdir");
        let orch = dir.path().join("orch.sock");
        let _ = std::fs::remove_file(&orch);
        let listener = UnixListener::bind(&orch).expect("bind");
        let orch_str = orch.to_string_lossy().to_string();

        let server = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.expect("accept");
            use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
            let mut line = String::new();
            let mut br = BufReader::new(&mut stream);
            br.read_line(&mut line).await.expect("read");
            let response = json!({
                "jsonrpc": "2.0",
                "result": { "services": [] },
                "id": 1
            });
            stream
                .write_all(format!("{response}\n").as_bytes())
                .await
                .expect("write");
        });

        temp_env::async_with_vars(
            [("NESTGATE_ORCHESTRATION_IPC_PATH", Some(orch_str.as_str()))],
            async {
                let err = match CryptoDelegate::new().await {
                    Err(e) => e,
                    Ok(_) => panic!("expected empty capability list to fail"),
                };
                let s = err.to_string();
                assert!(
                    s.contains("No crypto provider") || s.contains("crypto"),
                    "unexpected error: {s}"
                );
            },
        )
        .await;

        server.abort();
    }

    #[tokio::test]
    #[cfg(unix)]
    async fn encrypt_errors_on_missing_ciphertext() {
        use unix_crypto_mock::with_mock_delegate;

        with_mock_delegate(
            |method, _| {
                if method == "crypto.encrypt" {
                    json!({ "nonce": "YQ==" })
                } else {
                    json!({})
                }
            },
            |delegate| async move {
                let err = delegate
                    .encrypt(b"x", &EncryptionParams::default())
                    .await
                    .expect_err("missing ciphertext");
                assert!(err.to_string().contains("ciphertext"));
            },
        )
        .await;
    }

    #[tokio::test]
    #[cfg(unix)]
    async fn encrypt_errors_on_invalid_ciphertext_base64() {
        use unix_crypto_mock::with_mock_delegate;

        with_mock_delegate(
            |method, _| {
                if method == "crypto.encrypt" {
                    json!({
                        "ciphertext": "@@@",
                        "nonce": "YQ==",
                    })
                } else {
                    json!({})
                }
            },
            |delegate| async move {
                let err = delegate
                    .encrypt(b"x", &EncryptionParams::default())
                    .await
                    .expect_err("bad b64");
                assert!(
                    err.to_string().contains("base64") || err.to_string().contains("decode"),
                    "{}",
                    err
                );
            },
        )
        .await;
    }

    #[tokio::test]
    #[cfg(unix)]
    async fn encrypt_errors_on_missing_nonce_field() {
        use unix_crypto_mock::with_mock_delegate;

        with_mock_delegate(
            |method, _| {
                if method == "crypto.encrypt" {
                    json!({ "ciphertext": "YQ==" })
                } else {
                    json!({})
                }
            },
            |delegate| async move {
                let err = delegate
                    .encrypt(b"x", &EncryptionParams::default())
                    .await
                    .expect_err("missing nonce");
                assert!(err.to_string().contains("nonce"));
            },
        )
        .await;
    }

    #[tokio::test]
    #[cfg(unix)]
    async fn decrypt_errors_on_missing_plaintext() {
        use unix_crypto_mock::with_mock_delegate;

        with_mock_delegate(
            |method, _| {
                if method == "crypto.decrypt" {
                    json!({ "note": "no plaintext" })
                } else {
                    json!({})
                }
            },
            |delegate| async move {
                let enc = EncryptedData {
                    ciphertext: vec![1],
                    nonce: vec![2],
                    algorithm: EncryptionAlgorithm::Aes256Gcm,
                    timestamp: 0,
                };
                let err = delegate.decrypt(&enc).await.expect_err("no plaintext");
                assert!(err.to_string().contains("plaintext"));
            },
        )
        .await;
    }

    #[tokio::test]
    #[cfg(unix)]
    async fn generate_key_errors_missing_key() {
        use unix_crypto_mock::with_mock_delegate;

        with_mock_delegate(
            |method, _| {
                if method == "crypto.generate_key" {
                    json!({ "oops": true })
                } else {
                    json!({})
                }
            },
            |delegate| async move {
                let err = delegate.generate_key(8).await.expect_err("no key field");
                assert!(err.to_string().contains("key"));
            },
        )
        .await;
    }

    #[tokio::test]
    #[cfg(unix)]
    async fn generate_key_errors_length_mismatch() {
        use base64::Engine;
        use unix_crypto_mock::with_mock_delegate;

        with_mock_delegate(
            |method, _| {
                if method == "crypto.generate_key" {
                    json!({
                        "key": base64::engine::general_purpose::STANDARD.encode([1u8, 2u8]),
                    })
                } else {
                    json!({})
                }
            },
            |delegate| async move {
                let err = delegate.generate_key(99).await.expect_err("len");
                assert!(err.to_string().contains("length") || err.to_string().contains("Key"));
            },
        )
        .await;
    }

    #[tokio::test]
    #[cfg(unix)]
    async fn generate_nonce_errors_missing_nonce() {
        use unix_crypto_mock::with_mock_delegate;

        with_mock_delegate(
            |method, _| {
                if method == "crypto.generate_nonce" {
                    json!({})
                } else {
                    json!({})
                }
            },
            |delegate| async move {
                let err = delegate
                    .generate_nonce(EncryptionAlgorithm::Aes256Gcm)
                    .await
                    .expect_err("nonce");
                assert!(err.to_string().contains("nonce"));
            },
        )
        .await;
    }

    #[tokio::test]
    #[cfg(unix)]
    async fn hash_errors_missing_hash() {
        use unix_crypto_mock::with_mock_delegate;

        with_mock_delegate(
            |method, _| {
                if method == "crypto.hash" {
                    json!({ "x": 1 })
                } else {
                    json!({})
                }
            },
            |delegate| async move {
                let err = delegate.hash(b"a", "sha256").await.expect_err("hash");
                assert!(err.to_string().contains("hash"));
            },
        )
        .await;
    }

    #[tokio::test]
    #[cfg(unix)]
    async fn verify_hash_errors_missing_valid() {
        use unix_crypto_mock::with_mock_delegate;

        with_mock_delegate(
            |method, _| {
                if method == "crypto.verify_hash" {
                    json!({})
                } else {
                    json!({})
                }
            },
            |delegate| async move {
                let err = delegate
                    .verify_hash(b"a", b"b", "sha256")
                    .await
                    .expect_err("valid");
                assert!(err.to_string().contains("valid"));
            },
        )
        .await;
    }

    #[tokio::test]
    #[cfg(unix)]
    async fn sign_jwt_errors_missing_token() {
        use unix_crypto_mock::with_mock_delegate;

        with_mock_delegate(
            |method, _| {
                if method == "crypto.sign_jwt" {
                    json!({ "not_token": true })
                } else {
                    json!({})
                }
            },
            |delegate| async move {
                let err = delegate.sign_jwt("{}", "HS256").await.expect_err("token");
                assert!(err.to_string().contains("token"));
            },
        )
        .await;
    }

    #[tokio::test]
    #[cfg(unix)]
    async fn verify_jwt_errors_missing_claims() {
        use unix_crypto_mock::with_mock_delegate;

        with_mock_delegate(
            |method, _| {
                if method == "crypto.verify_jwt" {
                    json!({ "token": "x" })
                } else {
                    json!({})
                }
            },
            |delegate| async move {
                let err = delegate.verify_jwt("x", "HS256").await.expect_err("claims");
                assert!(err.to_string().contains("claims"));
            },
        )
        .await;
    }

    #[tokio::test]
    #[cfg(unix)]
    async fn verify_password_errors_missing_valid() {
        use unix_crypto_mock::with_mock_delegate;

        with_mock_delegate(
            |method, _| {
                if method == "crypto.verify_password" {
                    json!({})
                } else {
                    json!({})
                }
            },
            |delegate| async move {
                let err = delegate.verify_password("a", "b").await.expect_err("valid");
                assert!(err.to_string().contains("valid"));
            },
        )
        .await;
    }

    #[tokio::test]
    #[cfg(unix)]
    async fn hmac_sign_errors_missing_mac() {
        use unix_crypto_mock::with_mock_delegate;

        with_mock_delegate(
            |method, _| {
                if method == "crypto.hmac" {
                    json!({})
                } else {
                    json!({})
                }
            },
            |delegate| async move {
                let err = delegate.hmac_sign(b"k", b"m").await.expect_err("mac");
                assert!(err.to_string().contains("mac"));
            },
        )
        .await;
    }

    #[tokio::test]
    #[cfg(unix)]
    async fn hmac_verify_errors_missing_valid() {
        use unix_crypto_mock::with_mock_delegate;

        with_mock_delegate(
            |method, _| {
                if method == "crypto.hmac_verify" {
                    json!({})
                } else {
                    json!({})
                }
            },
            |delegate| async move {
                let err = delegate
                    .hmac_verify(b"k", b"m", b"x")
                    .await
                    .expect_err("valid");
                assert!(err.to_string().contains("valid"));
            },
        )
        .await;
    }

    #[tokio::test]
    #[cfg(unix)]
    async fn random_bytes_errors_missing_bytes() {
        use unix_crypto_mock::with_mock_delegate;

        with_mock_delegate(
            |method, _| {
                if method == "crypto.random_bytes" {
                    json!({ "length": 4 })
                } else {
                    json!({})
                }
            },
            |delegate| async move {
                let err = delegate.random_bytes(4).await.expect_err("bytes");
                assert!(err.to_string().contains("bytes"));
            },
        )
        .await;
    }

    #[tokio::test]
    #[cfg(unix)]
    async fn health_check_propagates_jsonrpc_error() {
        use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, split};
        use tokio::net::{UnixListener, UnixStream};

        let dir = tempfile::tempdir().expect("tmpdir");
        let sock_path = dir.path().join("health_err.sock");
        let _ = std::fs::remove_file(&sock_path);
        let listener = UnixListener::bind(&sock_path).expect("bind");
        let path_str = sock_path.to_string_lossy().to_string();

        let server = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.expect("accept");
            let (read_half, mut write_half) = split(stream);
            let mut reader = BufReader::new(read_half);
            let mut line = String::new();
            reader.read_line(&mut line).await.expect("read");
            let resp = json!({
                "jsonrpc": "2.0",
                "error": { "code": -32603, "message": "health failed" },
                "id": 1
            });
            let mut out = resp.to_string();
            out.push('\n');
            write_half.write_all(out.as_bytes()).await.expect("write");
            write_half.flush().await.expect("flush");
        });

        let delegate = CryptoDelegate::with_endpoint(&path_str)
            .await
            .expect("delegate");
        let err = delegate.health_check().await.expect_err("rpc error");
        assert!(err.to_string().contains("JSON-RPC") || err.to_string().contains("-32603"));

        if let Ok(c) = UnixStream::connect(path_str).await {
            drop(c);
        }
        server.abort();
    }
}
