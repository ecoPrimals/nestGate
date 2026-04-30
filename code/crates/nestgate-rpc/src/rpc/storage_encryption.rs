// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # Native encrypt-at-rest for storage operations
//!
//! Provides ChaCha20-Poly1305 encryption/decryption using a 32-byte purpose key.
//!
//! ## Key Resolution (startup, once)
//!
//! 1. `NESTGATE_ENCRYPTION_KEY` env var (hex-encoded 32 bytes) — bootstrap override
//! 2. Security provider `secrets.retrieve("nucleus:{family}:purpose:storage")` via JSON-RPC
//! 3. `None` — standalone mode, no encryption (backward compatible)
//!
//! ## Envelope Format
//!
//! ```json
//! {"v":1,"ct":"<base64 ciphertext>","n":"<base64 nonce>","alg":"chacha20-poly1305"}
//! ```
//!
//! ## Backward Compatibility
//!
//! On retrieve, data is checked for the envelope signature. Unencrypted data is
//! returned as-is, allowing transparent migration from plaintext storage.

use base64::Engine;
use chacha20poly1305::aead::{Aead, KeyInit, OsRng};
use chacha20poly1305::{AeadCore, ChaCha20Poly1305, Nonce};
use nestgate_types::error::{NestGateError, Result};
use serde_json::Value;
use tracing::{debug, info, warn};

const ENVELOPE_VERSION: u64 = 1;
const ENVELOPE_ALG: &str = "chacha20-poly1305";
const CHACHA20_KEY_LEN: usize = 32;

fn b64_encode(data: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD.encode(data)
}

fn b64_decode(s: &str) -> Result<Vec<u8>> {
    base64::engine::general_purpose::STANDARD
        .decode(s)
        .map_err(|e| NestGateError::api_error(format!("base64 decode: {e}")))
}

/// Native ChaCha20-Poly1305 encrypt-at-rest for storage data.
///
/// Holds a 32-byte purpose key and provides envelope encrypt/decrypt.
#[derive(Clone)]
pub struct StorageEncryption {
    key: [u8; CHACHA20_KEY_LEN],
}

impl StorageEncryption {
    /// Create from an explicit 32-byte key.
    #[must_use]
    pub const fn new(key: [u8; CHACHA20_KEY_LEN]) -> Self {
        Self { key }
    }

    /// Try to load encryption key from `NESTGATE_ENCRYPTION_KEY` env var.
    ///
    /// Accepts hex-encoded (64 chars) or base64-encoded (44 chars) 32-byte keys.
    #[must_use]
    pub fn from_env() -> Option<Self> {
        let raw = std::env::var("NESTGATE_ENCRYPTION_KEY").ok()?;
        let raw = raw.trim();
        if raw.is_empty() {
            return None;
        }

        let bytes = if raw.len() == 64 && raw.chars().all(|c| c.is_ascii_hexdigit()) {
            hex_decode(raw).ok()?
        } else {
            b64_decode(raw).ok()?
        };

        if bytes.len() != CHACHA20_KEY_LEN {
            warn!(
                "NESTGATE_ENCRYPTION_KEY decoded to {} bytes (expected {CHACHA20_KEY_LEN}), ignoring",
                bytes.len()
            );
            return None;
        }

        let mut key = [0u8; CHACHA20_KEY_LEN];
        key.copy_from_slice(&bytes);
        info!("Storage encryption key loaded from NESTGATE_ENCRYPTION_KEY");
        Some(Self { key })
    }

    /// Try to retrieve the storage purpose key from the security capability
    /// provider via `secrets.retrieve`.
    ///
    /// Connects using `SECURITY_PROVIDER_SOCKET` env, falling back to the
    /// 6-tier security socket discovery chain.
    pub async fn from_provider(family_id: &str) -> Option<Self> {
        let socket_path = resolve_security_provider_socket()?;
        debug!("Attempting storage key retrieval from security provider at {socket_path}");

        let secret_name = format!("nucleus:{family_id}:purpose:storage");
        let mut client = match super::JsonRpcClient::connect_unix(&socket_path).await {
            Ok(c) => c,
            Err(e) => {
                debug!("Cannot connect to security provider for key retrieval: {e}");
                return None;
            }
        };

        let response = match client
            .call(
                "secrets.retrieve",
                serde_json::json!({ "name": secret_name }),
            )
            .await
        {
            Ok(r) => r,
            Err(e) => {
                debug!("secrets.retrieve failed: {e}");
                return None;
            }
        };

        let key_b64 = response["value"]
            .as_str()
            .or_else(|| response["key"].as_str())?;
        let bytes = b64_decode(key_b64).ok()?;
        if bytes.len() != CHACHA20_KEY_LEN {
            warn!(
                "secrets.retrieve returned {} bytes (expected {CHACHA20_KEY_LEN}), ignoring",
                bytes.len()
            );
            return None;
        }

        let mut key = [0u8; CHACHA20_KEY_LEN];
        key.copy_from_slice(&bytes);
        info!("Storage encryption key loaded from security provider");
        Some(Self { key })
    }

    /// Resolve encryption key: env override first, then provider, then `None`.
    pub async fn resolve(family_id: Option<&str>) -> Option<Self> {
        if let Some(enc) = Self::from_env() {
            return Some(enc);
        }
        if let Some(fid) = family_id
            && let Some(enc) = Self::from_provider(fid).await
        {
            return Some(enc);
        }
        None
    }

    /// Encrypt `plaintext` and return the envelope as JSON bytes.
    ///
    /// Each call generates a fresh random 12-byte nonce.
    ///
    /// # Errors
    ///
    /// Returns an error if the AEAD encryption or JSON serialization fails.
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
        let cipher = ChaCha20Poly1305::new((&self.key).into());
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let ciphertext = cipher.encrypt(&nonce, plaintext).map_err(|e| {
            NestGateError::internal_error(format!("encryption failed: {e}"), "storage_encryption")
        })?;

        let envelope = serde_json::json!({
            "v": ENVELOPE_VERSION,
            "ct": b64_encode(&ciphertext),
            "n": b64_encode(nonce.as_slice()),
            "alg": ENVELOPE_ALG,
        });
        serde_json::to_vec(&envelope)
            .map_err(|e| NestGateError::io_error(format!("envelope serialize: {e}")))
    }

    /// Decrypt an encrypted envelope, returning the original plaintext.
    ///
    /// # Errors
    ///
    /// Returns an error if the envelope is malformed, the version/algorithm is
    /// unsupported, or the AEAD decryption fails (wrong key, tampered data).
    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let envelope: Value = serde_json::from_slice(data)
            .map_err(|e| NestGateError::api_error(format!("envelope parse: {e}")))?;

        let v = envelope["v"]
            .as_u64()
            .ok_or_else(|| NestGateError::api_error("missing envelope version"))?;
        if v != ENVELOPE_VERSION {
            return Err(NestGateError::api_error(format!(
                "unsupported envelope version: {v}"
            )));
        }

        let alg = envelope["alg"]
            .as_str()
            .ok_or_else(|| NestGateError::api_error("missing envelope algorithm"))?;
        if alg != ENVELOPE_ALG {
            return Err(NestGateError::api_error(format!(
                "unsupported algorithm: {alg}"
            )));
        }

        let ct_b64 = envelope["ct"]
            .as_str()
            .ok_or_else(|| NestGateError::api_error("missing ciphertext"))?;
        let n_b64 = envelope["n"]
            .as_str()
            .ok_or_else(|| NestGateError::api_error("missing nonce"))?;

        let ciphertext = b64_decode(ct_b64)?;
        let nonce_bytes = b64_decode(n_b64)?;

        if nonce_bytes.len() != 12 {
            return Err(NestGateError::api_error(format!(
                "invalid nonce length: {} (expected 12)",
                nonce_bytes.len()
            )));
        }
        let nonce = Nonce::from_slice(&nonce_bytes);

        let cipher = ChaCha20Poly1305::new((&self.key).into());
        cipher.decrypt(nonce, ciphertext.as_ref()).map_err(|e| {
            NestGateError::internal_error(format!("decryption failed: {e}"), "storage_encryption")
        })
    }

    /// Check whether `data` looks like an encrypted envelope.
    ///
    /// Does a lightweight JSON probe for the `v`, `ct`, `n`, and `alg` fields
    /// without fully validating the ciphertext.
    #[must_use]
    pub fn is_encrypted_envelope(data: &[u8]) -> bool {
        let Ok(v) = serde_json::from_slice::<Value>(data) else {
            return false;
        };
        v.get("v").and_then(Value::as_u64) == Some(ENVELOPE_VERSION)
            && v.get("ct").and_then(Value::as_str).is_some()
            && v.get("n").and_then(Value::as_str).is_some()
            && v.get("alg").and_then(Value::as_str) == Some(ENVELOPE_ALG)
    }
}

/// Resolve the security provider socket path from environment.
///
/// Precedence: `SECURITY_PROVIDER_SOCKET` → `BEARDOG_SOCKET` (legacy) →
/// 6-tier security socket discovery.
fn resolve_security_provider_socket() -> Option<String> {
    for var in ["SECURITY_PROVIDER_SOCKET", "BEARDOG_SOCKET"] {
        if let Ok(p) = std::env::var(var)
            && !p.is_empty()
        {
            return Some(p);
        }
    }
    let path = super::btsp_client::resolve_security_socket_path();
    path.to_str().map(String::from)
}

/// Hex-decode a string to bytes.
fn hex_decode(s: &str) -> Result<Vec<u8>> {
    (0..s.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&s[i..i + 2], 16)
                .map_err(|e| NestGateError::api_error(format!("hex decode: {e}")))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_key() -> [u8; 32] {
        let mut key = [0u8; 32];
        for (i, b) in key.iter_mut().enumerate() {
            *b = i as u8;
        }
        key
    }

    #[test]
    fn encrypt_decrypt_round_trip() {
        let enc = StorageEncryption::new(test_key());
        let plaintext = b"hello sovereign storage";
        let envelope = enc.encrypt(plaintext).expect("encrypt");
        let decrypted = enc.decrypt(&envelope).expect("decrypt");
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn envelope_format_matches_spec() {
        let enc = StorageEncryption::new(test_key());
        let envelope_bytes = enc.encrypt(b"test").expect("encrypt");
        let envelope: Value = serde_json::from_slice(&envelope_bytes).expect("parse");
        assert_eq!(envelope["v"], 1);
        assert_eq!(envelope["alg"], "chacha20-poly1305");
        assert!(envelope["ct"].as_str().is_some());
        assert!(envelope["n"].as_str().is_some());
        assert_eq!(envelope.as_object().expect("obj").len(), 4);
    }

    #[test]
    fn is_encrypted_envelope_detects_valid() {
        let enc = StorageEncryption::new(test_key());
        let envelope = enc.encrypt(b"data").expect("encrypt");
        assert!(StorageEncryption::is_encrypted_envelope(&envelope));
    }

    #[test]
    fn is_encrypted_envelope_rejects_plaintext() {
        assert!(!StorageEncryption::is_encrypted_envelope(b"hello world"));
        assert!(!StorageEncryption::is_encrypted_envelope(
            b"{\"key\": \"value\"}"
        ));
        assert!(!StorageEncryption::is_encrypted_envelope(
            b"{\"v\":1,\"alg\":\"aes-256\"}"
        ));
    }

    #[test]
    fn decrypt_wrong_key_fails() {
        let enc1 = StorageEncryption::new(test_key());
        let mut other_key = test_key();
        other_key[0] = 0xFF;
        let enc2 = StorageEncryption::new(other_key);

        let envelope = enc1.encrypt(b"secret").expect("encrypt");
        assert!(enc2.decrypt(&envelope).is_err());
    }

    #[test]
    fn nonces_are_unique() {
        let enc = StorageEncryption::new(test_key());
        let e1 = enc.encrypt(b"a").expect("e1");
        let e2 = enc.encrypt(b"a").expect("e2");

        let v1: Value = serde_json::from_slice(&e1).expect("p1");
        let v2: Value = serde_json::from_slice(&e2).expect("p2");
        assert_ne!(v1["n"], v2["n"], "nonces must differ between encryptions");
    }

    #[test]
    fn empty_plaintext_works() {
        let enc = StorageEncryption::new(test_key());
        let envelope = enc.encrypt(b"").expect("encrypt empty");
        let decrypted = enc.decrypt(&envelope).expect("decrypt empty");
        assert!(decrypted.is_empty());
    }

    #[test]
    fn large_payload_round_trip() {
        let enc = StorageEncryption::new(test_key());
        let data = vec![0xAB_u8; 1_000_000];
        let envelope = enc.encrypt(&data).expect("encrypt large");
        let decrypted = enc.decrypt(&envelope).expect("decrypt large");
        assert_eq!(decrypted, data);
    }

    #[test]
    fn hex_decode_works() {
        assert_eq!(
            hex_decode("deadbeef").unwrap(),
            vec![0xde, 0xad, 0xbe, 0xef]
        );
    }

    #[test]
    fn from_env_hex_key() {
        let hex_key = "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f";
        temp_env::with_var("NESTGATE_ENCRYPTION_KEY", Some(hex_key), || {
            let enc = StorageEncryption::from_env().expect("from_env");
            assert_eq!(enc.key, test_key());
        });
    }

    #[test]
    fn from_env_base64_key() {
        let b64_key = b64_encode(&test_key());
        temp_env::with_var("NESTGATE_ENCRYPTION_KEY", Some(&b64_key), || {
            let enc = StorageEncryption::from_env().expect("from_env b64");
            assert_eq!(enc.key, test_key());
        });
    }

    #[test]
    fn from_env_empty_returns_none() {
        temp_env::with_var("NESTGATE_ENCRYPTION_KEY", Some(""), || {
            assert!(StorageEncryption::from_env().is_none());
        });
    }

    #[test]
    fn from_env_wrong_length_returns_none() {
        temp_env::with_var("NESTGATE_ENCRYPTION_KEY", Some("deadbeef"), || {
            assert!(StorageEncryption::from_env().is_none());
        });
    }

    #[test]
    fn from_env_unset_returns_none() {
        temp_env::with_var("NESTGATE_ENCRYPTION_KEY", None::<&str>, || {
            assert!(StorageEncryption::from_env().is_none());
        });
    }

    #[test]
    fn unsupported_version_rejected() {
        let bad = serde_json::json!({
            "v": 99, "ct": "AAAA", "n": "AAAA", "alg": "chacha20-poly1305"
        });
        let enc = StorageEncryption::new(test_key());
        assert!(enc.decrypt(&serde_json::to_vec(&bad).unwrap()).is_err());
    }

    #[test]
    fn unsupported_algorithm_rejected() {
        let bad = serde_json::json!({
            "v": 1, "ct": "AAAA", "n": "AAAA", "alg": "aes-256-gcm"
        });
        let enc = StorageEncryption::new(test_key());
        assert!(enc.decrypt(&serde_json::to_vec(&bad).unwrap()).is_err());
    }
}
