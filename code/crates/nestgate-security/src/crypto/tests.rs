// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **CRYPTO MODULE INTEGRATION TESTS**
//!
//! ✅ EVOLVED: Tests updated for real Pure Rust crypto implementation
//!
//! # Test Coverage
//!
//! - Encryption/decryption roundtrip
//! - Tamper detection
//! - Key management
//! - Nonce uniqueness
//! - Performance benchmarks

#[cfg(test)]
mod crypto_tests {
    use super::super::*;

    #[tokio::test]
    async fn test_secure_crypto_initialization() {
        let crypto = SecureCrypto::new();
        assert!(
            crypto.is_ok(),
            "SecureCrypto should initialize successfully"
        );

        let crypto = crypto.unwrap();
        assert_eq!(crypto.algorithm, EncryptionAlgorithm::Aes256Gcm);
        assert!(crypto.cipher.is_some(), "Cipher should be initialized");
    }

    #[tokio::test]
    async fn test_encryption_params_default() {
        let params = EncryptionParams::default();
        assert_eq!(params.algorithm, EncryptionAlgorithm::Aes256Gcm);
        assert!(params.associated_data.is_empty());
    }

    #[tokio::test]
    async fn test_encrypt_decrypt_roundtrip() {
        let crypto = SecureCrypto::new().unwrap();
        let plaintext = b"sensitive data that must be protected";
        let params = EncryptionParams::default();

        let encrypted = crypto.encrypt(plaintext, &params).unwrap();
        assert!(
            !encrypted.ciphertext.is_empty(),
            "Ciphertext should not be empty"
        );
        assert_eq!(
            encrypted.nonce.len(),
            12,
            "Nonce should be 12 bytes (96 bits)"
        );
        assert!(encrypted.timestamp > 0, "Timestamp should be set");

        let decrypted = crypto.decrypt(&encrypted).unwrap();
        assert_eq!(plaintext, &decrypted[..]);
    }

    #[tokio::test]
    async fn test_tamper_detection() {
        let crypto = SecureCrypto::new().unwrap();
        let plaintext = b"important data";
        let params = EncryptionParams::default();

        let mut encrypted = crypto.encrypt(plaintext, &params).unwrap();

        // Tamper with ciphertext
        if !encrypted.ciphertext.is_empty() {
            encrypted.ciphertext[0] ^= 0xFF;
        }

        // Should fail authentication
        let result = crypto.decrypt(&encrypted);
        assert!(result.is_err(), "Decryption should fail for tampered data");
    }

    #[tokio::test]
    async fn test_nonce_uniqueness() {
        let crypto = SecureCrypto::new().unwrap();
        let params = EncryptionParams::default();

        let encrypted1 = crypto.encrypt(b"data", &params).unwrap();
        let encrypted2 = crypto.encrypt(b"data", &params).unwrap();

        // Nonces should be different even for same plaintext
        assert_ne!(encrypted1.nonce, encrypted2.nonce);
        // Ciphertext should be different too (different nonce = different output)
        assert_ne!(encrypted1.ciphertext, encrypted2.ciphertext);
    }

    #[test]
    fn test_encryption_algorithm_serialization() {
        let algo = EncryptionAlgorithm::Aes256Gcm;
        let json = serde_json::to_string(&algo);
        assert!(json.is_ok());

        let deserialized = serde_json::from_str::<EncryptionAlgorithm>(&json.unwrap());
        assert!(deserialized.is_ok());
        assert_eq!(deserialized.unwrap(), algo);
    }

    #[test]
    fn test_encrypted_data_serialization() {
        let data = EncryptedData {
            ciphertext: vec![1, 2, 3, 4, 5],
            nonce: vec![6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17],
            algorithm: EncryptionAlgorithm::Aes256Gcm,
            timestamp: 1_234_567_890,
        };

        let json = serde_json::to_string(&data);
        assert!(json.is_ok());

        let deserialized = serde_json::from_str::<EncryptedData>(&json.unwrap());
        assert!(deserialized.is_ok());

        let restored = deserialized.unwrap();
        assert_eq!(restored.ciphertext, data.ciphertext);
        assert_eq!(restored.nonce, data.nonce);
        assert_eq!(restored.algorithm, data.algorithm);
    }

    #[test]
    fn test_with_different_algorithms() {
        let crypto_gcm = SecureCrypto::with_algorithm(EncryptionAlgorithm::Aes256Gcm);
        assert!(crypto_gcm.is_ok());

        let crypto_chacha = SecureCrypto::with_algorithm(EncryptionAlgorithm::ChaCha20Poly1305);
        assert!(crypto_chacha.is_ok());
    }

    #[test]
    fn test_key_generation() {
        let key = SecureCrypto::generate_key(32).unwrap();
        assert_eq!(key.len(), 32);

        // Keys should be unique
        let key2 = SecureCrypto::generate_key(32).unwrap();
        assert_ne!(key, key2, "Generated keys should be unique");
    }

    #[test]
    fn test_from_key() {
        let key = SecureCrypto::generate_key(32).unwrap();
        let crypto = SecureCrypto::from_key(&key);
        assert!(
            crypto.is_ok(),
            "Should create crypto from valid 32-byte key"
        );

        // Invalid key length should fail
        let short_key = vec![0u8; 16];
        let result = SecureCrypto::from_key(&short_key);
        assert!(result.is_err(), "Should reject non-32-byte key");
    }

    #[tokio::test]
    async fn test_from_key_roundtrip() {
        // Create crypto, get the key implicitly via from_key
        let key = SecureCrypto::generate_key(32).unwrap();
        let crypto = SecureCrypto::from_key(&key).unwrap();

        let plaintext = b"data encrypted with specific key";
        let params = EncryptionParams::default();
        let encrypted = crypto.encrypt(plaintext, &params).unwrap();

        // Same key should decrypt
        let crypto2 = SecureCrypto::from_key(&key).unwrap();
        let decrypted = crypto2.decrypt(&encrypted).unwrap();
        assert_eq!(plaintext, &decrypted[..]);
    }

    #[tokio::test]
    async fn test_wrong_key_fails() {
        let key1 = SecureCrypto::generate_key(32).unwrap();
        let key2 = SecureCrypto::generate_key(32).unwrap();

        let crypto1 = SecureCrypto::from_key(&key1).unwrap();
        let params = EncryptionParams::default();
        let encrypted = crypto1.encrypt(b"secret", &params).unwrap();

        // Different key should fail to decrypt
        let crypto2 = SecureCrypto::from_key(&key2).unwrap();
        let result = crypto2.decrypt(&encrypted);
        assert!(result.is_err(), "Wrong key should fail decryption");
    }

    #[tokio::test]
    async fn test_encryption_performance() {
        use std::time::Instant;

        let crypto = SecureCrypto::new().unwrap();
        let params = EncryptionParams::default();
        let data = vec![0u8; 1024]; // 1KB

        let start = Instant::now();
        for _ in 0..100 {
            let _ = crypto.encrypt(&data, &params).unwrap();
        }
        let duration = start.elapsed();

        // Should be fast: <10ms per encryption for 1KB
        let avg_ms = duration.as_millis() / 100;
        assert!(
            avg_ms < 10,
            "Encryption should be < 10ms per 1KB, got {avg_ms}ms"
        );
    }

    #[tokio::test]
    async fn test_large_data_encryption() {
        let crypto = SecureCrypto::new().unwrap();
        let params = EncryptionParams::default();
        let large_data = vec![0u8; 1024 * 1024]; // 1MB

        let encrypted = crypto.encrypt(&large_data, &params).unwrap();
        let decrypted = crypto.decrypt(&encrypted).unwrap();

        assert_eq!(large_data, decrypted);
    }

    #[tokio::test]
    async fn test_empty_data_encryption() {
        let crypto = SecureCrypto::new().unwrap();
        let params = EncryptionParams::default();

        let encrypted = crypto.encrypt(b"", &params).unwrap();
        let decrypted = crypto.decrypt(&encrypted).unwrap();
        assert!(decrypted.is_empty());
    }

    #[test]
    fn test_nonce_generation() {
        let crypto = SecureCrypto::new().unwrap();
        let nonce = crypto.generate_nonce().unwrap();
        assert_eq!(nonce.len(), 12, "AES-GCM nonce should be 12 bytes");

        let nonce2 = crypto.generate_nonce().unwrap();
        assert_ne!(nonce, nonce2, "Nonces should be unique");
    }
}
