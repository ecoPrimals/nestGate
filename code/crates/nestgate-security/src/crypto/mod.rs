// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Cryptographic types and bearDog IPC delegation.
//!
//! `NestGate` does not perform cryptographic operations locally — it delegates
//! all crypto to the `bearDog` primal (or any provider advertising the `"crypto"`
//! capability) via JSON-RPC IPC.
//!
//! # Architecture
//!
//! ```text
//! NestGate (needs crypto)
//!   → CryptoDelegate::new()
//!   → CapabilityDiscovery::find("crypto")
//!   → JSON-RPC calls: crypto.encrypt, crypto.sign_jwt, crypto.hash, ...
//! ```
//!
//! # Types
//!
//! This module exports the data types used across crypto operations.
//! The actual implementations live in [`delegate::CryptoDelegate`].

use serde::{Deserialize, Serialize};

/// Encryption algorithm selection.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EncryptionAlgorithm {
    /// AES-256-GCM (recommended for most use cases)
    Aes256Gcm,
    /// ChaCha20-Poly1305 (recommended for mobile/embedded)
    ChaCha20Poly1305,
}

/// Encryption parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionParams {
    /// Algorithm to use
    pub algorithm: EncryptionAlgorithm,
    /// Associated data (authenticated but not encrypted)
    pub associated_data: Vec<u8>,
}

/// Encrypted data container.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedData {
    /// The ciphertext
    pub ciphertext: Vec<u8>,
    /// The nonce used for encryption
    pub nonce: Vec<u8>,
    /// Algorithm used
    pub algorithm: EncryptionAlgorithm,
    /// Timestamp of encryption
    pub timestamp: u64,
}

impl Default for EncryptionParams {
    fn default() -> Self {
        Self {
            algorithm: EncryptionAlgorithm::Aes256Gcm,
            associated_data: Vec::new(),
        }
    }
}

/// Capability-based crypto delegation to bearDog (or any compatible provider).
///
/// Discovers a primal providing `"crypto"` capability at runtime and delegates
/// all cryptographic operations via JSON-RPC semantic methods.
pub mod delegate;

/// JWT claim types for token-based authentication.
///
/// The `JwtClaims` type is a pure data structure — signing and verification
/// are delegated to the crypto provider via [`delegate::CryptoDelegate`].
pub mod jwt_claims;

#[cfg(test)]
mod tests;
