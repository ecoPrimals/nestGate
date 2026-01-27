//! # Crypto Delegation - BearDog Integration
//!
//! **Deep Debt Solution**: Replace DEVELOPMENT STUB with capability-based delegation.
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
//!   ↓
//! CryptoDelegate::new()
//!   ↓
//! CapabilityDiscovery::find("crypto")
//!   ↓
//! ServiceMetadataStore → Find service with "crypto" capability
//!   ↓
//! Connect to crypto provider (could be BearDog, or any crypto service!)
//!   ↓
//! Delegate crypto.* operations via JSON-RPC
//! ```
//!
//! ## Usage
//!
//! ```rust,ignore
//! use nestgate_core::crypto::CryptoDelegate;
//!
//! // Create delegate (discovers crypto provider automatically)
//! let crypto = CryptoDelegate::new().await?;
//!
//! // Encrypt data (delegated to BearDog or other crypto provider)
//! let encrypted = crypto.encrypt(b"sensitive data", &params).await?;
//!
//! // Decrypt data
//! let decrypted = crypto.decrypt(&encrypted).await?;
//!
//! // Generate key
//! let key = crypto.generate_key(32).await?;
//! ```
//!
//! ## Benefits
//!
//! - ✅ Zero crypto dependencies in NestGate (Pure Rust still maintained!)
//! - ✅ Primal autonomy (BearDog can be replaced by any crypto provider)
//! - ✅ Runtime discovery (no hardcoded endpoints)
//! - ✅ Capability-based (discover by "crypto" capability)
//! - ✅ Production-ready (eliminates DEVELOPMENT STUB)
//!
//! ## Semantic Method Mapping
//!
//! | NestGate Method | JSON-RPC Method | BearDog Implementation |
//! |-----------------|-----------------|------------------------|
//! | `encrypt()` | `crypto.encrypt` | AES-256-GCM / ChaCha20 |
//! | `decrypt()` | `crypto.decrypt` | AES-256-GCM / ChaCha20 |
//! | `generate_key()` | `crypto.generate_key` | Secure random (rand) |
//! | `generate_nonce()` | `crypto.generate_nonce` | Secure random |
//! | `hash()` | `crypto.hash` | SHA-256 / SHA-512 |
//! | `verify_hash()` | `crypto.verify_hash` | Constant-time compare |
//!
//! ## References
//!
//! - `CAPABILITY_MAPPINGS.md` - NestGate capability requirements
//! - `wateringHole/SEMANTIC_METHOD_NAMING_STANDARD.md` - Semantic naming
//! - `wateringHole/PRIMAL_SOVEREIGNTY_STANDARD.md` - Self-knowledge principle

use crate::{
    capability_discovery::{CapabilityDiscovery, ServiceEndpoint},
    crypto::{EncryptedData, EncryptionAlgorithm, EncryptionParams},
    rpc::JsonRpcClient,
    NestGateError, Result,
};
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, info, warn};

/// Crypto operations delegator to external crypto service (BearDog or compatible)
///
/// This struct discovers and delegates to a primal providing the "crypto" capability.
/// It maintains no crypto implementation itself, following the separation of concerns
/// and primal sovereignty principles.
pub struct CryptoDelegate {
    /// JSON-RPC client to crypto provider
    client: Arc<JsonRpcClient>,

    /// Endpoint information (for logging/debugging only)
    endpoint: ServiceEndpoint,
}

impl CryptoDelegate {
    /// Create new crypto delegate by discovering crypto provider
    ///
    /// # Discovery Process
    ///
    /// 1. Discover Songbird IPC service (registry)
    /// 2. Query for services providing "crypto" capability
    /// 3. Connect to first available crypto provider
    /// 4. Return delegate ready for operations
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - No crypto provider found (no service with "crypto" capability)
    /// - Connection to crypto provider fails
    /// - Songbird discovery fails
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let crypto = CryptoDelegate::new().await?;
    /// // crypto is now connected to BearDog (or any crypto provider)
    /// ```
    pub async fn new() -> Result<Self> {
        info!("🔍 Discovering crypto provider via capability-based discovery...");

        // Discover Songbird IPC service (registry)
        let songbird = CapabilityDiscovery::discover_songbird_ipc().await.map_err(|e| {
            NestGateError::discovery_error(&format!("Failed to discover Songbird: {}", e))
        })?;

        // Create capability discovery client
        let discovery = CapabilityDiscovery::new(songbird);

        // Find service providing "crypto" capability
        let endpoint = discovery.find("crypto").await.map_err(|e| {
            NestGateError::discovery_error(&format!(
                "No crypto provider found. Is BearDog running? Error: {}",
                e
            ))
        })?;

        info!(
            "✅ Found crypto provider: {} at {}",
            endpoint.name, endpoint.endpoint
        );

        // Connect to crypto provider
        let client = JsonRpcClient::connect(&endpoint.endpoint).await?;

        Ok(Self {
            client: Arc::new(client),
            endpoint,
        })
    }

    /// Create delegate with explicit endpoint (for testing)
    ///
    /// This bypasses discovery and connects directly to a specific endpoint.
    /// Useful for testing or when crypto provider location is known.
    pub async fn with_endpoint(endpoint: &str) -> Result<Self> {
        debug!("🔌 Connecting directly to crypto provider: {}", endpoint);

        let client = JsonRpcClient::connect(endpoint).await?;

        Ok(Self {
            client: Arc::new(client),
            endpoint: ServiceEndpoint {
                capability: "crypto".to_string(),
                name: "crypto-provider".to_string(),
                endpoint: endpoint.to_string(),
                version: "unknown".to_string(),
                discovered_at: std::time::Instant::now(),
            },
        })
    }

    /// Get crypto provider information
    ///
    /// Returns the discovered service endpoint information.
    /// Useful for logging, monitoring, and debugging.
    pub fn provider_info(&self) -> &ServiceEndpoint {
        &self.endpoint
    }

    /// Encrypt data using crypto provider
    ///
    /// Delegates to `crypto.encrypt` semantic method on discovered provider.
    ///
    /// # Arguments
    ///
    /// * `plaintext` - Data to encrypt
    /// * `params` - Encryption parameters (algorithm, associated data)
    ///
    /// # Security
    ///
    /// - Encryption is performed by the crypto provider (BearDog)
    /// - Uses provider's secure random nonce generation
    /// - Provides authenticated encryption (AEAD)
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let encrypted = crypto.encrypt(b"secret data", &params).await?;
    /// ```
    pub async fn encrypt(
        &self,
        plaintext: &[u8],
        params: &EncryptionParams,
    ) -> Result<EncryptedData> {
        debug!(
            "🔐 Delegating encryption to {} ({} bytes)",
            self.endpoint.name,
            plaintext.len()
        );

        // Prepare JSON-RPC request
        let request = json!({
            "plaintext": base64::encode(plaintext),
            "algorithm": match params.algorithm {
                EncryptionAlgorithm::Aes256Gcm => "aes256gcm",
                EncryptionAlgorithm::ChaCha20Poly1305 => "chacha20poly1305",
            },
            "associated_data": base64::encode(&params.associated_data),
        });

        // Call crypto.encrypt on provider
        let response = self.client.call("crypto.encrypt", request).await?;

        // Parse response
        let ciphertext = base64::decode(
            response["ciphertext"]
                .as_str()
                .ok_or_else(|| NestGateError::rpc_error("Missing ciphertext in response"))?,
        )
        .map_err(|e| NestGateError::rpc_error(&format!("Invalid base64 ciphertext: {}", e)))?;

        let nonce = base64::decode(
            response["nonce"]
                .as_str()
                .ok_or_else(|| NestGateError::rpc_error("Missing nonce in response"))?,
        )
        .map_err(|e| NestGateError::rpc_error(&format!("Invalid base64 nonce: {}", e)))?;

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        debug!("✅ Encryption complete ({} bytes)", ciphertext.len());

        Ok(EncryptedData {
            ciphertext,
            nonce,
            algorithm: params.algorithm,
            timestamp,
        })
    }

    /// Decrypt data using crypto provider
    ///
    /// Delegates to `crypto.decrypt` semantic method on discovered provider.
    ///
    /// # Security
    ///
    /// - Decryption is performed by the crypto provider
    /// - Verifies authentication tag before decryption
    /// - Detects tampering and forgery
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Authentication tag verification fails
    /// - Data has been tampered with
    /// - Decryption operation fails
    pub async fn decrypt(&self, encrypted: &EncryptedData) -> Result<Vec<u8>> {
        debug!(
            "🔓 Delegating decryption to {} ({} bytes)",
            self.endpoint.name,
            encrypted.ciphertext.len()
        );

        // Prepare JSON-RPC request
        let request = json!({
            "ciphertext": base64::encode(&encrypted.ciphertext),
            "nonce": base64::encode(&encrypted.nonce),
            "algorithm": match encrypted.algorithm {
                EncryptionAlgorithm::Aes256Gcm => "aes256gcm",
                EncryptionAlgorithm::ChaCha20Poly1305 => "chacha20poly1305",
            },
        });

        // Call crypto.decrypt on provider
        let response = self.client.call("crypto.decrypt", request).await?;

        // Parse response
        let plaintext = base64::decode(
            response["plaintext"]
                .as_str()
                .ok_or_else(|| NestGateError::rpc_error("Missing plaintext in response"))?,
        )
        .map_err(|e| NestGateError::rpc_error(&format!("Invalid base64 plaintext: {}", e)))?;

        debug!("✅ Decryption complete ({} bytes)", plaintext.len());

        Ok(plaintext)
    }

    /// Generate secure random key
    ///
    /// Delegates to `crypto.generate_key` semantic method.
    ///
    /// # Arguments
    ///
    /// * `length` - Key length in bytes (e.g., 32 for 256-bit key)
    ///
    /// # Security
    ///
    /// - Uses cryptographically secure random number generator
    /// - Keys are generated by crypto provider (BearDog)
    pub async fn generate_key(&self, length: usize) -> Result<Vec<u8>> {
        debug!("🔑 Generating key ({} bytes) via {}", length, self.endpoint.name);

        let request = json!({ "length": length });

        let response = self.client.call("crypto.generate_key", request).await?;

        let key = base64::decode(
            response["key"]
                .as_str()
                .ok_or_else(|| NestGateError::rpc_error("Missing key in response"))?,
        )
        .map_err(|e| NestGateError::rpc_error(&format!("Invalid base64 key: {}", e)))?;

        if key.len() != length {
            return Err(NestGateError::rpc_error(&format!(
                "Key length mismatch: expected {}, got {}",
                length,
                key.len()
            )));
        }

        debug!("✅ Key generated ({} bytes)", key.len());

        Ok(key)
    }

    /// Generate secure random nonce
    ///
    /// Delegates to `crypto.generate_nonce` semantic method.
    ///
    /// # Arguments
    ///
    /// * `algorithm` - Algorithm requiring the nonce
    ///
    /// # Returns
    ///
    /// Nonce of appropriate length for the algorithm:
    /// - AES-256-GCM: 12 bytes (96 bits)
    /// - ChaCha20-Poly1305: 12 bytes (96 bits)
    pub async fn generate_nonce(&self, algorithm: EncryptionAlgorithm) -> Result<Vec<u8>> {
        debug!("🎲 Generating nonce for {:?} via {}", algorithm, self.endpoint.name);

        let request = json!({
            "algorithm": match algorithm {
                EncryptionAlgorithm::Aes256Gcm => "aes256gcm",
                EncryptionAlgorithm::ChaCha20Poly1305 => "chacha20poly1305",
            }
        });

        let response = self.client.call("crypto.generate_nonce", request).await?;

        let nonce = base64::decode(
            response["nonce"]
                .as_str()
                .ok_or_else(|| NestGateError::rpc_error("Missing nonce in response"))?,
        )
        .map_err(|e| NestGateError::rpc_error(&format!("Invalid base64 nonce: {}", e)))?;

        debug!("✅ Nonce generated ({} bytes)", nonce.len());

        Ok(nonce)
    }

    /// Hash data using secure hash function
    ///
    /// Delegates to `crypto.hash` semantic method.
    ///
    /// # Arguments
    ///
    /// * `data` - Data to hash
    /// * `algorithm` - Hash algorithm ("sha256", "sha512")
    pub async fn hash(&self, data: &[u8], algorithm: &str) -> Result<Vec<u8>> {
        debug!(
            "🔨 Hashing {} bytes with {} via {}",
            data.len(),
            algorithm,
            self.endpoint.name
        );

        let request = json!({
            "data": base64::encode(data),
            "algorithm": algorithm
        });

        let response = self.client.call("crypto.hash", request).await?;

        let hash = base64::decode(
            response["hash"]
                .as_str()
                .ok_or_else(|| NestGateError::rpc_error("Missing hash in response"))?,
        )
        .map_err(|e| NestGateError::rpc_error(&format!("Invalid base64 hash: {}", e)))?;

        debug!("✅ Hash computed ({} bytes)", hash.len());

        Ok(hash)
    }

    /// Verify hash matches data
    ///
    /// Delegates to `crypto.verify_hash` semantic method.
    ///
    /// # Security
    ///
    /// - Uses constant-time comparison (timing attack resistant)
    /// - Verification performed by crypto provider
    pub async fn verify_hash(&self, data: &[u8], hash: &[u8], algorithm: &str) -> Result<bool> {
        debug!(
            "🔍 Verifying hash ({} bytes) with {} via {}",
            hash.len(),
            algorithm,
            self.endpoint.name
        );

        let request = json!({
            "data": base64::encode(data),
            "hash": base64::encode(hash),
            "algorithm": algorithm
        });

        let response = self.client.call("crypto.verify_hash", request).await?;

        let valid = response["valid"]
            .as_bool()
            .ok_or_else(|| NestGateError::rpc_error("Missing 'valid' in response"))?;

        debug!("✅ Hash verification: {}", if valid { "VALID" } else { "INVALID" });

        Ok(valid)
    }

    /// Check if crypto provider is healthy
    ///
    /// Calls `health.check` on the crypto provider.
    pub async fn health_check(&self) -> Result<Value> {
        self.client.call("health.check", json!({})).await
    }
}

/// Helper module for base64 encoding/decoding
mod base64 {
    use crate::{NestGateError, Result};

    /// Encode bytes to base64 string
    pub fn encode(data: &[u8]) -> String {
        use base64::Engine;
        base64::engine::general_purpose::STANDARD.encode(data)
    }

    /// Decode base64 string to bytes
    pub fn decode(data: &str) -> Result<Vec<u8>> {
        use base64::Engine;
        base64::engine::general_purpose::STANDARD
            .decode(data)
            .map_err(|e| NestGateError::api_error(&format!("Base64 decode error: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires running BearDog instance
    async fn test_crypto_delegate_discovery() {
        // This test requires:
        // 1. Songbird IPC service running
        // 2. BearDog registered with "crypto" capability
        // 3. BearDog responding to crypto.* methods

        let result = CryptoDelegate::new().await;
        assert!(result.is_ok(), "Should discover crypto provider");

        let delegate = result.unwrap();
        assert_eq!(delegate.endpoint.capability, "crypto");
    }

    #[tokio::test]
    #[ignore] // Requires running BearDog instance
    async fn test_crypto_delegate_encrypt_decrypt() {
        let delegate = CryptoDelegate::new().await.unwrap();

        let plaintext = b"Hello, BearDog!";
        let params = EncryptionParams {
            algorithm: EncryptionAlgorithm::Aes256Gcm,
            associated_data: Vec::new(),
        };

        // Encrypt
        let encrypted = delegate.encrypt(plaintext, &params).await.unwrap();
        assert!(!encrypted.ciphertext.is_empty());
        assert!(!encrypted.nonce.is_empty());

        // Decrypt
        let decrypted = delegate.decrypt(&encrypted).await.unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[tokio::test]
    #[ignore] // Requires running BearDog instance
    async fn test_crypto_delegate_key_generation() {
        let delegate = CryptoDelegate::new().await.unwrap();

        let key = delegate.generate_key(32).await.unwrap();
        assert_eq!(key.len(), 32);
    }

    #[test]
    fn test_encryption_params_default() {
        let params = EncryptionParams::default();
        assert_eq!(params.algorithm, EncryptionAlgorithm::Aes256Gcm);
        assert!(params.associated_data.is_empty());
    }
}
