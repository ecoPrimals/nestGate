//! # Crypto Delegation — BearDog Integration
//!
//! Capability-based delegation to an external crypto provider.
//!
//! ## Philosophy (Primal Sovereignty)
//!
//! - **Self-Knowledge**: NestGate knows it needs "crypto", not "BearDog"
//! - **Runtime Discovery**: Find crypto provider dynamically
//! - **Capability-Based**: Discover by capability, not hardcoded name
//! - **Zero Hardcoding**: Any primal providing "crypto" capability works
//!
//! ## Architecture
//!
//! ```text
//! NestGate (needs crypto)
//!   → CryptoDelegate::new()
//!   → CapabilityDiscovery::find("crypto")
//!   → Connect to discovered crypto provider (BearDog or compatible)
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

use crate::{
    capability_discovery::{CapabilityDiscovery, ServiceEndpoint},
    crypto::{EncryptedData, EncryptionAlgorithm, EncryptionParams},
    rpc::JsonRpcClient,
    NestGateError, Result,
};
use serde_json::{json, Value};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
use tracing::{debug, info};

/// Base64 helpers using the `base64` crate already in our dependency tree.
mod b64 {
    use base64::Engine;

    pub fn encode(data: &[u8]) -> String {
        base64::engine::general_purpose::STANDARD.encode(data)
    }

    pub fn decode(data: &str) -> crate::Result<Vec<u8>> {
        base64::engine::general_purpose::STANDARD
            .decode(data)
            .map_err(|e| crate::NestGateError::api_error(&format!("base64 decode: {e}")))
    }
}

/// Crypto operations delegator to an external crypto service.
///
/// Discovers a primal that advertises the "crypto" capability and delegates
/// all cryptographic operations to it via JSON-RPC semantic methods.
pub struct CryptoDelegate {
    /// JSON-RPC client to the crypto provider (mutable for call())
    client: Mutex<JsonRpcClient>,
    /// Endpoint information (for logging/debugging)
    endpoint: ServiceEndpoint,
}

impl CryptoDelegate {
    /// Create new crypto delegate by discovering a crypto provider.
    ///
    /// 1. Discover Songbird IPC service
    /// 2. Query for services providing "crypto" capability
    /// 3. Connect to the first available crypto provider
    pub async fn new() -> Result<Self> {
        info!("Discovering crypto provider via capability-based discovery");

        let ipc_gateway = CapabilityDiscovery::discover_songbird_ipc()
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
    pub fn provider_info(&self) -> &ServiceEndpoint {
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
            return Err(NestGateError::api_error(&format!(
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

    #[tokio::test]
    #[ignore = "requires running BearDog instance"]
    async fn test_crypto_delegate_discovery() {
        let result = CryptoDelegate::new().await;
        assert!(result.is_ok(), "Should discover crypto provider");

        let delegate = result.unwrap();
        assert_eq!(delegate.endpoint.capability, "crypto");
    }

    #[tokio::test]
    #[ignore = "requires running BearDog instance"]
    async fn test_crypto_delegate_encrypt_decrypt() {
        let delegate = CryptoDelegate::new().await.unwrap();

        let plaintext = b"Hello, BearDog!";
        let params = EncryptionParams::default();

        let encrypted = delegate.encrypt(plaintext, &params).await.unwrap();
        assert!(!encrypted.ciphertext.is_empty());
        assert!(!encrypted.nonce.is_empty());

        let decrypted = delegate.decrypt(&encrypted).await.unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[tokio::test]
    #[ignore = "requires running BearDog instance"]
    async fn test_crypto_delegate_key_generation() {
        let delegate = CryptoDelegate::new().await.unwrap();

        let key = delegate.generate_key(32).await.unwrap();
        assert_eq!(key.len(), 32);
    }
}
