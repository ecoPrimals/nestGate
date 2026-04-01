// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Crypto module tests — type serialization and delegate integration.
//!
//! Local crypto implementations have been removed; all cryptographic operations
//! are delegated to the crypto capability provider (bearDog) via `CryptoDelegate`.

#[cfg(test)]
mod crypto_tests {
    use super::super::*;

    #[test]
    fn encryption_params_default() {
        let params = EncryptionParams::default();
        assert_eq!(params.algorithm, EncryptionAlgorithm::Aes256Gcm);
        assert!(params.associated_data.is_empty());
    }

    #[test]
    fn encryption_algorithm_serialization() {
        let algo = EncryptionAlgorithm::Aes256Gcm;
        let json = serde_json::to_string(&algo).unwrap();
        let deserialized: EncryptionAlgorithm = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, algo);
    }

    #[test]
    fn encrypted_data_serialization() {
        let data = EncryptedData {
            ciphertext: vec![1, 2, 3, 4, 5],
            nonce: vec![6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17],
            algorithm: EncryptionAlgorithm::Aes256Gcm,
            timestamp: 1_234_567_890,
        };

        let json = serde_json::to_string(&data).unwrap();
        let restored: EncryptedData = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.ciphertext, data.ciphertext);
        assert_eq!(restored.nonce, data.nonce);
        assert_eq!(restored.algorithm, data.algorithm);
        assert_eq!(restored.timestamp, data.timestamp);
    }

    #[test]
    fn encryption_params_chacha_roundtrip() {
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
    fn encrypted_data_fields_roundtrip() {
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
}
