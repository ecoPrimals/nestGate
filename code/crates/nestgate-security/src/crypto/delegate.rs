// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

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
#[path = "delegate_tests.rs"]
mod delegate_tests;
