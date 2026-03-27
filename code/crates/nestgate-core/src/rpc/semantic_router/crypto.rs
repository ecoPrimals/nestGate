//! Crypto domain semantic methods
//!
//! Handles crypto.* semantic method routing for cryptographic operations.
//! Delegates to BearDog or compatible crypto providers via capability discovery.

use super::SemanticRouter;
use crate::error::{NestGateError, Result};
use serde_json::{json, Value};
use tracing::debug;

/// Route crypto.encrypt → CryptoDelegate::encrypt
///
/// Delegates encryption to BearDog or compatible crypto provider.
/// Demonstrates capability-based discovery in action!
pub(super) async fn crypto_encrypt(_router: &SemanticRouter, params: Value) -> Result<Value> {
    use crate::crypto::{delegate::CryptoDelegate, EncryptionAlgorithm, EncryptionParams};
    use base64::{engine::general_purpose::STANDARD, Engine as _};

    // Discover and connect to crypto provider
    let delegate = CryptoDelegate::new().await?;

    // Parse parameters
    let plaintext_b64 = params["plaintext"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("plaintext", "base64 string required")
    })?;

    let plaintext = STANDARD.decode(plaintext_b64).map_err(|e| {
        NestGateError::invalid_input_with_field("plaintext", format!("Invalid base64: {}", e))
    })?;

    let algorithm = match params["algorithm"].as_str().unwrap_or("aes256gcm") {
        "aes256gcm" => EncryptionAlgorithm::Aes256Gcm,
        "chacha20poly1305" => EncryptionAlgorithm::ChaCha20Poly1305,
        algo => {
            return Err(NestGateError::invalid_input_with_field(
                "algorithm",
                format!("Unsupported algorithm: {}", algo),
            ))
        }
    };

    let associated_data = if let Some(ad) = params["associated_data"].as_str() {
        STANDARD.decode(ad).map_err(|e| {
            NestGateError::invalid_input_with_field(
                "associated_data",
                format!("Invalid base64: {}", e),
            )
        })?
    } else {
        Vec::new()
    };

    let encryption_params = EncryptionParams {
        algorithm,
        associated_data,
    };

    // Delegate to crypto provider
    let encrypted = delegate.encrypt(&plaintext, &encryption_params).await?;

    debug!(
        "🔐 Encryption complete via {}",
        delegate.provider_info().name
    );

    Ok(json!({
        "ciphertext": STANDARD.encode(&encrypted.ciphertext),
        "nonce": STANDARD.encode(&encrypted.nonce),
        "algorithm": match encrypted.algorithm {
            EncryptionAlgorithm::Aes256Gcm => "aes256gcm",
            EncryptionAlgorithm::ChaCha20Poly1305 => "chacha20poly1305",
        },
        "timestamp": encrypted.timestamp,
        "provider": delegate.provider_info().name
    }))
}

/// Route crypto.decrypt → CryptoDelegate::decrypt
pub(super) async fn crypto_decrypt(_router: &SemanticRouter, params: Value) -> Result<Value> {
    use crate::crypto::{delegate::CryptoDelegate, EncryptedData, EncryptionAlgorithm};
    use base64::{engine::general_purpose::STANDARD, Engine as _};

    let delegate = CryptoDelegate::new().await?;

    // Parse parameters
    let ciphertext_b64 = params["ciphertext"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("ciphertext", "base64 string required")
    })?;

    let ciphertext = STANDARD.decode(ciphertext_b64).map_err(|e| {
        NestGateError::invalid_input_with_field("ciphertext", format!("Invalid base64: {}", e))
    })?;

    let nonce_b64 = params["nonce"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("nonce", "base64 string required")
    })?;

    let nonce = STANDARD.decode(nonce_b64).map_err(|e| {
        NestGateError::invalid_input_with_field("nonce", format!("Invalid base64: {}", e))
    })?;

    let algorithm = match params["algorithm"].as_str().unwrap_or("aes256gcm") {
        "aes256gcm" => EncryptionAlgorithm::Aes256Gcm,
        "chacha20poly1305" => EncryptionAlgorithm::ChaCha20Poly1305,
        algo => {
            return Err(NestGateError::invalid_input_with_field(
                "algorithm",
                format!("Unsupported algorithm: {}", algo),
            ))
        }
    };

    let encrypted = EncryptedData {
        ciphertext,
        nonce,
        algorithm,
        timestamp: params["timestamp"].as_u64().unwrap_or(0),
    };

    // Delegate to crypto provider
    let plaintext = delegate.decrypt(&encrypted).await?;

    debug!(
        "🔓 Decryption complete via {}",
        delegate.provider_info().name
    );

    Ok(json!({
        "plaintext": STANDARD.encode(&plaintext),
        "provider": delegate.provider_info().name
    }))
}

/// Route crypto.generate_key → CryptoDelegate::generate_key
pub(super) async fn crypto_generate_key(_router: &SemanticRouter, params: Value) -> Result<Value> {
    use crate::crypto::delegate::CryptoDelegate;
    use base64::{engine::general_purpose::STANDARD, Engine as _};

    let delegate = CryptoDelegate::new().await?;

    let length = params["length"]
        .as_u64()
        .ok_or_else(|| NestGateError::invalid_input_with_field("length", "number required"))?
        as usize;

    let key = delegate.generate_key(length).await?;

    debug!(
        "🔑 Key generated ({} bytes) via {}",
        length,
        delegate.provider_info().name
    );

    Ok(json!({
        "key": STANDARD.encode(&key),
        "length": key.len(),
        "provider": delegate.provider_info().name
    }))
}

/// Route crypto.generate_nonce → CryptoDelegate::generate_nonce
pub(super) async fn crypto_generate_nonce(
    _router: &SemanticRouter,
    params: Value,
) -> Result<Value> {
    use crate::crypto::{delegate::CryptoDelegate, EncryptionAlgorithm};
    use base64::{engine::general_purpose::STANDARD, Engine as _};

    let delegate = CryptoDelegate::new().await?;

    let algorithm = match params["algorithm"].as_str().unwrap_or("aes256gcm") {
        "aes256gcm" => EncryptionAlgorithm::Aes256Gcm,
        "chacha20poly1305" => EncryptionAlgorithm::ChaCha20Poly1305,
        algo => {
            return Err(NestGateError::invalid_input_with_field(
                "algorithm",
                format!("Unsupported algorithm: {}", algo),
            ))
        }
    };

    let nonce = delegate.generate_nonce(algorithm).await?;

    debug!(
        "🎲 Nonce generated ({} bytes) via {}",
        nonce.len(),
        delegate.provider_info().name
    );

    Ok(json!({
        "nonce": STANDARD.encode(&nonce),
        "length": nonce.len(),
        "provider": delegate.provider_info().name
    }))
}

/// Route crypto.hash → CryptoDelegate::hash
pub(super) async fn crypto_hash(_router: &SemanticRouter, params: Value) -> Result<Value> {
    use crate::crypto::delegate::CryptoDelegate;
    use base64::{engine::general_purpose::STANDARD, Engine as _};

    let delegate = CryptoDelegate::new().await?;

    let data_b64 = params["data"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("data", "base64 string required"))?;

    let data = STANDARD.decode(data_b64).map_err(|e| {
        NestGateError::invalid_input_with_field("data", format!("Invalid base64: {}", e))
    })?;

    let algorithm = params["algorithm"].as_str().unwrap_or("sha256");

    let hash = delegate.hash(&data, algorithm).await?;

    debug!(
        "🔨 Hash computed ({} bytes) with {} via {}",
        hash.len(),
        algorithm,
        delegate.provider_info().name
    );

    Ok(json!({
        "hash": STANDARD.encode(&hash),
        "algorithm": algorithm,
        "provider": delegate.provider_info().name
    }))
}

/// Route crypto.verify_hash → CryptoDelegate::verify_hash
pub(super) async fn crypto_verify_hash(_router: &SemanticRouter, params: Value) -> Result<Value> {
    use crate::crypto::delegate::CryptoDelegate;
    use base64::{engine::general_purpose::STANDARD, Engine as _};

    let delegate = CryptoDelegate::new().await?;

    let data_b64 = params["data"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("data", "base64 string required"))?;

    let data = STANDARD.decode(data_b64).map_err(|e| {
        NestGateError::invalid_input_with_field("data", format!("Invalid base64: {}", e))
    })?;

    let hash_b64 = params["hash"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("hash", "base64 string required"))?;

    let hash = STANDARD.decode(hash_b64).map_err(|e| {
        NestGateError::invalid_input_with_field("hash", format!("Invalid base64: {}", e))
    })?;

    let algorithm = params["algorithm"].as_str().unwrap_or("sha256");

    let valid = delegate.verify_hash(&data, &hash, algorithm).await?;

    debug!(
        "🔍 Hash verification: {} via {}",
        if valid { "VALID" } else { "INVALID" },
        delegate.provider_info().name
    );

    Ok(json!({
        "valid": valid,
        "algorithm": algorithm,
        "provider": delegate.provider_info().name
    }))
}
