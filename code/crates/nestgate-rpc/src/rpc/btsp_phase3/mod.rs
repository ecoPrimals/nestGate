// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # BTSP Phase 3 — Encrypted Channel Negotiation
//!
//! After a successful Phase 1–2 handshake, the client may send a
//! `btsp.negotiate` JSON-RPC request to upgrade the connection to
//! ChaCha20-Poly1305 AEAD framing.
//!
//! # Session Key Derivation
//!
//! ```text
//! handshake_key = HKDF-SHA256(ikm=family_seed, salt="btsp-v1", info="handshake")
//! salt          = client_nonce || server_nonce
//! c2s_key       = HKDF-SHA256(ikm=handshake_key, salt, info="btsp-session-v1-c2s")
//! s2c_key       = HKDF-SHA256(ikm=handshake_key, salt, info="btsp-session-v1-s2c")
//! ```
//!
//! # Wire Format (encrypted channel)
//!
//! Each frame is length-prefixed with an encrypted payload:
//! ```text
//! [4 bytes: length (big-endian u32)] [12 bytes: nonce] [ciphertext + 16-byte Poly1305 tag]
//! ```

pub mod transport;

use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

use nestgate_types::error::{NestGateError, Result};

const HKDF_HANDSHAKE_SALT: &[u8] = b"btsp-v1";
const HKDF_HANDSHAKE_INFO: &[u8] = b"handshake";
const HKDF_INFO_C2S: &[u8] = b"btsp-session-v1-c2s";
const HKDF_INFO_S2C: &[u8] = b"btsp-session-v1-s2c";

const NONCE_SIZE: usize = 12;
const KEY_DERIVATION_NONCE_SIZE: usize = 32;
const MIN_ENCRYPTED_FRAME: usize = NONCE_SIZE + 16;

/// Cipher suites supported in Phase 3 negotiation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase3Cipher {
    /// ChaCha20-Poly1305 AEAD (encrypted + authenticated).
    #[serde(rename = "chacha20-poly1305")]
    ChaCha20Poly1305,
    /// Plaintext (no encryption, no integrity).
    #[serde(rename = "null")]
    Null,
}

impl Phase3Cipher {
    /// Wire-format name for this cipher.
    #[must_use]
    pub const fn wire_name(self) -> &'static str {
        match self {
            Self::ChaCha20Poly1305 => "chacha20-poly1305",
            Self::Null => "null",
        }
    }
}

/// Client-to-server Phase 3 negotiate request params.
#[derive(Debug, Deserialize)]
pub struct NegotiateParams {
    /// Session ID from the Phase 1 handshake.
    pub session_id: String,
    /// Ciphers the client supports, ordered by preference.
    pub ciphers: Option<Vec<String>>,
    /// Single cipher alias (compat with legacy security capability providers).
    pub preferred_cipher: Option<String>,
    /// Client-generated random nonce for session key derivation (base64).
    pub client_nonce: String,
}

impl NegotiateParams {
    /// Flatten both `ciphers` and `preferred_cipher` into one list.
    #[must_use]
    pub fn offered_ciphers(&self) -> Vec<String> {
        if let Some(ref c) = self.ciphers {
            return c.clone();
        }
        if let Some(ref p) = self.preferred_cipher {
            return vec![p.clone()];
        }
        Vec::new()
    }
}

/// Server-to-client Phase 3 negotiate response.
#[derive(Debug, Serialize)]
pub struct NegotiateResult {
    /// The cipher selected by the server.
    pub cipher: String,
    /// Server-generated random nonce for session key derivation (base64).
    pub server_nonce: String,
}

/// Directional session keys for encrypted BTSP framing.
///
/// `encrypt_key` protects outbound frames; `decrypt_key` decrypts inbound
/// frames. Both are zeroed from memory on drop.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct SessionKeys {
    encrypt_key: [u8; 32],
    decrypt_key: [u8; 32],
}

impl std::fmt::Debug for SessionKeys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SessionKeys")
            .field("encrypt_key", &"[REDACTED]")
            .field("decrypt_key", &"[REDACTED]")
            .finish()
    }
}

impl SessionKeys {
    /// Derive directional session keys via HKDF-SHA256.
    ///
    /// `salt = client_nonce || server_nonce`, `ikm = handshake_key`.
    /// Server encrypt = s2c, server decrypt = c2s.
    ///
    /// # Errors
    ///
    /// Returns an error if HKDF expansion fails.
    pub fn derive(
        handshake_key: &[u8; 32],
        client_nonce: &[u8],
        server_nonce: &[u8],
        is_server: bool,
    ) -> Result<Self> {
        use hkdf::Hkdf;
        use sha2::Sha256;

        let mut salt = Vec::with_capacity(client_nonce.len() + server_nonce.len());
        salt.extend_from_slice(client_nonce);
        salt.extend_from_slice(server_nonce);

        let hk = Hkdf::<Sha256>::new(Some(&salt), handshake_key);

        let mut client_to_server = [0u8; 32];
        hk.expand(HKDF_INFO_C2S, &mut client_to_server)
            .map_err(|e| {
                NestGateError::api_internal_error(format!("BTSP Phase 3 HKDF c2s: {e}"))
            })?;

        let mut server_to_client = [0u8; 32];
        hk.expand(HKDF_INFO_S2C, &mut server_to_client)
            .map_err(|e| {
                NestGateError::api_internal_error(format!("BTSP Phase 3 HKDF s2c: {e}"))
            })?;

        if is_server {
            Ok(Self {
                encrypt_key: server_to_client,
                decrypt_key: client_to_server,
            })
        } else {
            Ok(Self {
                encrypt_key: client_to_server,
                decrypt_key: server_to_client,
            })
        }
    }

    /// Encrypt a plaintext payload for transmission.
    ///
    /// Returns `nonce(12) || ciphertext || tag(16)`.
    ///
    /// # Errors
    ///
    /// Returns an error on encryption failure.
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
        use chacha20poly1305::aead::{Aead, KeyInit, OsRng};
        use chacha20poly1305::{AeadCore, ChaCha20Poly1305};

        let cipher = ChaCha20Poly1305::new_from_slice(&self.encrypt_key).map_err(|e| {
            NestGateError::api_internal_error(format!("BTSP Phase 3 cipher init: {e}"))
        })?;

        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

        let ciphertext = cipher
            .encrypt(&nonce, plaintext)
            .map_err(|e| NestGateError::api_internal_error(format!("BTSP Phase 3 encrypt: {e}")))?;

        let mut frame = Vec::with_capacity(NONCE_SIZE + ciphertext.len());
        frame.extend_from_slice(&nonce);
        frame.extend_from_slice(&ciphertext);
        Ok(frame)
    }

    /// Decrypt an incoming encrypted frame.
    ///
    /// Expects `nonce(12) || ciphertext || tag(16)`.
    ///
    /// # Errors
    ///
    /// Returns an error on decryption failure or if the frame is too short.
    pub fn decrypt(&self, frame: &[u8]) -> Result<Vec<u8>> {
        use chacha20poly1305::aead::{Aead, KeyInit};
        use chacha20poly1305::{ChaCha20Poly1305, Nonce};

        if frame.len() < MIN_ENCRYPTED_FRAME {
            return Err(NestGateError::validation_error(format!(
                "BTSP Phase 3: encrypted frame too short: {} bytes (min {MIN_ENCRYPTED_FRAME})",
                frame.len()
            )));
        }

        let (nonce_bytes, ciphertext) = frame.split_at(NONCE_SIZE);
        let nonce = Nonce::from_slice(nonce_bytes);

        let cipher = ChaCha20Poly1305::new_from_slice(&self.decrypt_key).map_err(|e| {
            NestGateError::api_internal_error(format!("BTSP Phase 3 cipher init: {e}"))
        })?;

        cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| NestGateError::api_internal_error(format!("BTSP Phase 3 decrypt: {e}")))
    }
}

/// Derive the 32-byte handshake key from the raw family seed.
///
/// `HKDF-SHA256(ikm=family_seed, salt="btsp-v1", info="handshake")`
///
/// # Errors
///
/// Returns an error if HKDF expansion fails.
pub fn derive_handshake_key(family_seed: &[u8]) -> Result<[u8; 32]> {
    use hkdf::Hkdf;
    use sha2::Sha256;

    let hk = Hkdf::<Sha256>::new(Some(HKDF_HANDSHAKE_SALT), family_seed);

    let mut key = [0u8; 32];
    hk.expand(HKDF_HANDSHAKE_INFO, &mut key).map_err(|e| {
        NestGateError::api_internal_error(format!("BTSP handshake key derivation: {e}"))
    })?;
    Ok(key)
}

/// Generate a 32-byte random nonce for Phase 3 key derivation salt.
///
/// # Errors
///
/// Returns an error if the OS CSPRNG is unavailable.
pub fn generate_server_nonce() -> Result<[u8; KEY_DERIVATION_NONCE_SIZE]> {
    use rand::RngCore;

    let mut nonce = [0u8; KEY_DERIVATION_NONCE_SIZE];
    rand::rng().fill_bytes(&mut nonce);
    Ok(nonce)
}

/// Select the best cipher from the client's offered list.
///
/// Returns `ChaCha20Poly1305` if offered, otherwise `Null`.
#[must_use]
pub fn select_cipher(offered: &[String]) -> Phase3Cipher {
    if offered
        .iter()
        .any(|c| c == Phase3Cipher::ChaCha20Poly1305.wire_name())
    {
        Phase3Cipher::ChaCha20Poly1305
    } else {
        Phase3Cipher::Null
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cipher_wire_names() {
        assert_eq!(
            Phase3Cipher::ChaCha20Poly1305.wire_name(),
            "chacha20-poly1305"
        );
        assert_eq!(Phase3Cipher::Null.wire_name(), "null");
    }

    #[test]
    fn cipher_serde_roundtrip() {
        let json =
            serde_json::to_string(&Phase3Cipher::ChaCha20Poly1305).expect("serialize chacha");
        assert_eq!(json, "\"chacha20-poly1305\"");

        let parsed: Phase3Cipher = serde_json::from_str(&json).expect("deserialize chacha");
        assert_eq!(parsed, Phase3Cipher::ChaCha20Poly1305);

        let null_json = serde_json::to_string(&Phase3Cipher::Null).expect("serialize null");
        assert_eq!(null_json, "\"null\"");
    }

    #[test]
    fn session_keys_derive_deterministic() {
        let handshake_key = [0xABu8; 32];
        let client_nonce = [1u8; 32];
        let server_nonce = [2u8; 32];

        let server_keys = SessionKeys::derive(&handshake_key, &client_nonce, &server_nonce, true)
            .expect("server keys");
        let client_keys = SessionKeys::derive(&handshake_key, &client_nonce, &server_nonce, false)
            .expect("client keys");

        assert_eq!(
            server_keys.encrypt_key, client_keys.decrypt_key,
            "server encrypt must equal client decrypt"
        );
        assert_eq!(
            server_keys.decrypt_key, client_keys.encrypt_key,
            "server decrypt must equal client encrypt"
        );
        assert_ne!(
            server_keys.encrypt_key, server_keys.decrypt_key,
            "directional keys must differ"
        );
    }

    #[test]
    fn encrypt_decrypt_roundtrip() {
        let handshake_key = [0x42u8; 32];
        let client_nonce = [3u8; 32];
        let server_nonce = [4u8; 32];

        let server_keys = SessionKeys::derive(&handshake_key, &client_nonce, &server_nonce, true)
            .expect("server keys");
        let client_keys = SessionKeys::derive(&handshake_key, &client_nonce, &server_nonce, false)
            .expect("client keys");

        let plaintext = b"hello encrypted btsp from nestgate";

        let encrypted = server_keys.encrypt(plaintext).expect("encrypt");
        assert_ne!(&encrypted[..], &plaintext[..]);
        assert!(encrypted.len() > plaintext.len());

        let decrypted = client_keys.decrypt(&encrypted).expect("decrypt");
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn encrypt_decrypt_wrong_key_fails() {
        let key_a = [0x01u8; 32];
        let key_b = [0x02u8; 32];
        let nonce_c = [3u8; 32];
        let nonce_s = [4u8; 32];

        let keys_a = SessionKeys::derive(&key_a, &nonce_c, &nonce_s, true).expect("keys a");
        let keys_b = SessionKeys::derive(&key_b, &nonce_c, &nonce_s, false).expect("keys b");

        let encrypted = keys_a.encrypt(b"secret").expect("encrypt");
        assert!(keys_b.decrypt(&encrypted).is_err());
    }

    #[test]
    fn decrypt_frame_too_short() {
        let keys = SessionKeys::derive(&[0u8; 32], &[1u8; 32], &[2u8; 32], true).expect("keys");
        let result = keys.decrypt(&[0u8; 10]);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("too short"), "unexpected error: {err_msg}");
    }

    #[test]
    fn generate_server_nonce_produces_32_bytes() {
        let nonce = generate_server_nonce().expect("nonce");
        assert_eq!(nonce.len(), 32);
    }

    #[test]
    fn generate_server_nonce_is_random() {
        let a = generate_server_nonce().expect("nonce a");
        let b = generate_server_nonce().expect("nonce b");
        assert_ne!(a, b, "two nonces should differ");
    }

    #[test]
    fn select_cipher_prefers_chacha() {
        let offered = vec!["chacha20-poly1305".to_owned(), "null".to_owned()];
        assert_eq!(select_cipher(&offered), Phase3Cipher::ChaCha20Poly1305);
    }

    #[test]
    fn select_cipher_falls_back_to_null() {
        let offered = vec!["unknown-cipher".to_owned()];
        assert_eq!(select_cipher(&offered), Phase3Cipher::Null);
    }

    #[test]
    fn select_cipher_empty_list() {
        assert_eq!(select_cipher(&[]), Phase3Cipher::Null);
    }

    #[test]
    fn negotiate_params_deserialize_ciphers() {
        let json = r#"{"session_id":"abc","ciphers":["chacha20-poly1305"],"client_nonce":"AQID"}"#;
        let params: NegotiateParams = serde_json::from_str(json).expect("deserialize");
        assert_eq!(params.session_id, "abc");
        let offered = params.offered_ciphers();
        assert_eq!(offered.len(), 1);
        assert_eq!(offered[0], "chacha20-poly1305");
    }

    #[test]
    fn negotiate_params_deserialize_preferred_cipher() {
        let json =
            r#"{"session_id":"x","preferred_cipher":"chacha20-poly1305","client_nonce":"AQID"}"#;
        let params: NegotiateParams = serde_json::from_str(json).expect("deserialize");
        let offered = params.offered_ciphers();
        assert_eq!(offered, vec!["chacha20-poly1305"]);
    }

    #[test]
    fn negotiate_result_serialize() {
        let result = NegotiateResult {
            cipher: "chacha20-poly1305".to_owned(),
            server_nonce: "BAUG".to_owned(),
        };
        let json = serde_json::to_string(&result).expect("serialize");
        assert!(json.contains("chacha20-poly1305"));
        assert!(json.contains("BAUG"));
    }

    #[test]
    fn session_keys_debug_redacted() {
        let keys = SessionKeys::derive(&[0u8; 32], &[1u8; 32], &[2u8; 32], true).expect("keys");
        let debug = format!("{keys:?}");
        assert!(debug.contains("REDACTED"));
    }

    #[test]
    fn encrypt_empty_payload() {
        let keys = SessionKeys::derive(&[0x55u8; 32], &[1u8; 32], &[2u8; 32], true).expect("keys");
        let client_keys =
            SessionKeys::derive(&[0x55u8; 32], &[1u8; 32], &[2u8; 32], false).expect("keys");

        let encrypted = keys.encrypt(b"").expect("encrypt empty");
        assert_eq!(encrypted.len(), NONCE_SIZE + 16);

        let decrypted = client_keys.decrypt(&encrypted).expect("decrypt empty");
        assert!(decrypted.is_empty());
    }

    #[test]
    fn encrypt_large_payload() {
        let keys = SessionKeys::derive(&[0x77u8; 32], &[1u8; 32], &[2u8; 32], true).expect("keys");
        let client_keys =
            SessionKeys::derive(&[0x77u8; 32], &[1u8; 32], &[2u8; 32], false).expect("keys");

        let large = vec![0xFFu8; 64 * 1024];
        let encrypted = keys.encrypt(&large).expect("encrypt large");
        let decrypted = client_keys.decrypt(&encrypted).expect("decrypt large");
        assert_eq!(decrypted, large);
    }

    #[test]
    fn derive_handshake_key_deterministic() {
        let seed = b"test-family-seed-for-nestgate";
        let key1 = derive_handshake_key(seed).expect("key1");
        let key2 = derive_handshake_key(seed).expect("key2");
        assert_eq!(key1, key2);
        assert_ne!(key1, [0u8; 32]);
    }

    #[test]
    fn derive_handshake_key_differs_by_seed() {
        let key_a = derive_handshake_key(b"seed-a").expect("key a");
        let key_b = derive_handshake_key(b"seed-b").expect("key b");
        assert_ne!(key_a, key_b);
    }

    #[test]
    fn full_phase3_key_agreement() {
        let seed = b"deterministic-test-seed-for-key-verification";
        let handshake_key = derive_handshake_key(seed).expect("handshake key");

        let client_nonce = [0x01; 32];
        let server_nonce = [0x02; 32];

        let server_keys = SessionKeys::derive(&handshake_key, &client_nonce, &server_nonce, true)
            .expect("server keys");
        let client_keys = SessionKeys::derive(&handshake_key, &client_nonce, &server_nonce, false)
            .expect("client keys");

        let msg = b"bidirectional test";

        let server_encrypted = server_keys.encrypt(msg).expect("server encrypt");
        let client_decrypted = client_keys
            .decrypt(&server_encrypted)
            .expect("client decrypt");
        assert_eq!(client_decrypted, msg);

        let client_encrypted = client_keys.encrypt(msg).expect("client encrypt");
        let server_decrypted = server_keys
            .decrypt(&client_encrypted)
            .expect("server decrypt");
        assert_eq!(server_decrypted, msg);
    }
}
