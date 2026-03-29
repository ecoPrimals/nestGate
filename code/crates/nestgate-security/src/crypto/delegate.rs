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
}

#[cfg(test)]
mod tests {
    use super::*;

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
    #[allow(clippy::too_many_lines)]
    async fn crypto_delegate_roundtrip_over_mock_unix_jsonrpc() {
        use base64::Engine;
        use serde_json::{Value, json};
        use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, split};
        use tokio::net::{UnixListener, UnixStream};

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
                let result = match method {
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
                    "health.check" => json!({ "ok": true }),
                    _ => json!({ "unknown": method }),
                };
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
        assert_eq!(delegate.provider_info().capability, "crypto");

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

        // Wake accept by connecting and closing (server may block on next read)
        if let Ok(c) = UnixStream::connect(path_str).await {
            drop(c);
        }
        server.abort();
    }
}
