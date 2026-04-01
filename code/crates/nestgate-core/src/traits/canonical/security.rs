// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **Canonical Security Trait**
//!
//! Comprehensive security trait for authentication, authorization, and encryption.
//!
//! **Extracted**: November 19, 2025 - From canonical_unified_traits.rs
//! **Lines**: ~300 (from original 1,100-line file)

use super::service::CanonicalService;
use super::types::SecurityCredentials;
use std::future::Future;

// ==================== CANONICAL SECURITY TRAIT ====================

/// **THE** canonical security trait that replaces ALL security service traits
///
/// This trait consolidates and replaces:
/// - Former `SecurityProvider` / `SecurityPrimalProvider` traits (removed)
/// - Former `ZeroCostSecurityProvider` / `NativeAsyncSecurityProvider` traits (deprecated)
///
/// **ENHANCED**: Now includes comprehensive security operations (14 methods)
/// **PERFORMANCE**: Native async throughout - zero overhead
/// **COMPLETENESS**: All security operations in single canonical location
///
/// # Consolidated Methods (November 9, 2025)
/// - Authentication (4 methods)
/// - Token management (4 methods)
/// - Encryption (4 methods)
/// - Signing/verification (2 methods)
/// - Key management (3 methods)
/// - Utilities (2 methods)
pub trait CanonicalSecurity: CanonicalService {
    /// Authentication token type
    type Token: Clone + Send + Sync + 'static;

    /// User identity type
    type Identity: Clone + Send + Sync + 'static;

    // ==================== AUTHENTICATION OPERATIONS ====================

    /// Authenticate user with credentials
    ///
    /// # Arguments
    /// * `credentials` - User credentials for authentication
    ///
    /// # Returns
    /// * `Ok(Token)` - Authentication token on success
    /// * `Err(Self::Error)` - Authentication failure
    ///
    /// # Example
    /// ```ignore
    /// let token = security.authenticate(credentials).await?;
    /// ```
    fn authenticate(
        &self,
        credentials: SecurityCredentials,
    ) -> impl Future<Output = std::result::Result<Self::Token, Self::Error>> + Send;

    /// Authorize access to resource
    ///
    /// Validates token and checks permissions for requested resource.
    ///
    /// # Arguments
    /// * `token` - Authentication token to validate
    /// * `resource` - Resource identifier being accessed
    ///
    /// # Returns
    /// * `Ok(Identity)` - Authorized user identity
    /// * `Err(Self::Error)` - Authorization failure
    fn authorize(
        &self,
        token: &Self::Token,
        resource: &str,
    ) -> impl Future<Output = std::result::Result<Self::Identity, Self::Error>> + Send;

    // ==================== TOKEN MANAGEMENT ====================

    /// Validate authentication token
    ///
    /// **Consolidated from**: ZeroCostSecurityProvider
    ///
    /// # Arguments
    /// * `token` - Token to validate
    ///
    /// # Returns
    /// * `Ok(Identity)` - Valid token, returns associated identity
    /// * `Err(Self::Error)` - Token invalid, expired, or validation error
    fn validate_token(
        &self,
        token: Self::Token,
    ) -> impl Future<Output = std::result::Result<Self::Identity, Self::Error>> + Send;

    /// Refresh authentication token
    ///
    /// **Consolidated from**: ZeroCostSecurityProvider, SecurityProvider
    ///
    /// # Arguments
    /// * `token` - Token to refresh
    ///
    /// # Returns
    /// * `Ok(Token)` - New token with extended expiry
    /// * `Err(Self::Error)` - Refresh failure (token expired or invalid)
    fn refresh_token(
        &self,
        token: &Self::Token,
    ) -> impl Future<Output = std::result::Result<Self::Token, Self::Error>> + Send;

    /// Revoke authentication token
    ///
    /// **Consolidated from**: All security providers
    ///
    /// # Arguments
    /// * `token` - Token to revoke
    ///
    /// # Returns
    /// * `Ok(())` - Token successfully revoked
    /// * `Err(Self::Error)` - Revocation failure
    fn revoke_token(
        &self,
        token: Self::Token,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    /// List all active tokens for user
    ///
    /// **Consolidated from**: SecurityProvider
    ///
    /// # Arguments
    /// * `identity` - User identity to query
    ///
    /// # Returns
    /// * `Ok(Vec<Token>)` - List of active tokens
    /// * `Err(Self::Error)` - Query failure
    fn list_active_tokens(
        &self,
        identity: &Self::Identity,
    ) -> impl Future<Output = std::result::Result<Vec<Self::Token>, Self::Error>> + Send;

    // ==================== ENCRYPTION OPERATIONS ====================

    /// Encrypt data with specified algorithm
    ///
    /// **Consolidated from**: ZeroCostSecurityProvider, EncryptionProvider
    ///
    /// # Arguments
    /// * `data` - Data to encrypt
    /// * `algorithm` - Encryption algorithm (e.g., "AES-256-GCM", "ChaCha20-Poly1305")
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` - Encrypted data
    /// * `Err(Self::Error)` - Encryption failure
    fn encrypt(
        &self,
        data: &[u8],
        algorithm: &str,
    ) -> impl Future<Output = std::result::Result<Vec<u8>, Self::Error>> + Send;

    /// Decrypt data
    ///
    /// **Consolidated from**: ZeroCostSecurityProvider, EncryptionProvider
    ///
    /// # Arguments
    /// * `encrypted_data` - Encrypted data to decrypt
    /// * `algorithm` - Decryption algorithm
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` - Decrypted data
    /// * `Err(Self::Error)` - Decryption failure
    fn decrypt(
        &self,
        encrypted_data: &[u8],
        algorithm: &str,
    ) -> impl Future<Output = std::result::Result<Vec<u8>, Self::Error>> + Send;

    /// Get encryption key for algorithm
    ///
    /// **Consolidated from**: SecurityProvider
    ///
    /// # Arguments
    /// * `algorithm` - Algorithm name
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` - Encryption key
    /// * `Err(Self::Error)` - Key retrieval failure
    fn get_encryption_key(
        &self,
        algorithm: &str,
    ) -> impl Future<Output = std::result::Result<Vec<u8>, Self::Error>> + Send;

    /// Rotate encryption key
    ///
    /// **Consolidated from**: SecurityProvider
    ///
    /// # Arguments
    /// * `algorithm` - Algorithm to rotate key for
    ///
    /// # Returns
    /// * `Ok(())` - Key rotated successfully
    /// * `Err(Self::Error)` - Rotation failure
    fn rotate_encryption_key(
        &self,
        algorithm: &str,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    // ==================== SIGNING OPERATIONS ====================

    /// Sign data cryptographically
    ///
    /// **Consolidated from**: ZeroCostSecurityProvider, SigningProvider
    ///
    /// # Arguments
    /// * `data` - Data to sign
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` - Digital signature
    /// * `Err(Self::Error)` - Signing failure
    fn sign(
        &self,
        data: &[u8],
    ) -> impl Future<Output = std::result::Result<Vec<u8>, Self::Error>> + Send;

    /// Verify cryptographic signature
    ///
    /// **Consolidated from**: ZeroCostSecurityProvider, SigningProvider
    ///
    /// # Arguments
    /// * `data` - Original data
    /// * `signature` - Signature to verify
    ///
    /// # Returns
    /// * `Ok(true)` - Signature valid
    /// * `Ok(false)` - Signature invalid
    /// * `Err(Self::Error)` - Verification error
    fn verify(
        &self,
        data: &[u8],
        signature: &[u8],
    ) -> impl Future<Output = std::result::Result<bool, Self::Error>> + Send;

    // ==================== KEY MANAGEMENT ====================

    /// Get signing key identifier
    ///
    /// **Consolidated from**: ZeroCostSecurityProvider
    ///
    /// # Returns
    /// * `Ok(String)` - Key ID used for signing
    /// * `Err(Self::Error)` - Key retrieval failure
    fn get_key_id(&self) -> impl Future<Output = std::result::Result<String, Self::Error>> + Send;

    /// Get supported algorithms
    ///
    /// **Consolidated from**: ZeroCostSecurityProvider, EncryptionProvider
    ///
    /// # Returns
    /// * `Ok(Vec<String>)` - List of supported algorithm names
    /// * `Err(Self::Error)` - Query failure
    fn supported_algorithms(
        &self,
    ) -> impl Future<Output = std::result::Result<Vec<String>, Self::Error>> + Send;

    // ==================== UTILITY OPERATIONS ====================

    /// Generate cryptographically secure random data
    ///
    /// **Consolidated from**: ZeroCostSecurityProvider
    ///
    /// # Arguments
    /// * `length` - Number of random bytes to generate
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` - Random bytes
    /// * `Err(Self::Error)` - Generation failure
    fn generate_random(
        &self,
        length: usize,
    ) -> impl Future<Output = std::result::Result<Vec<u8>, Self::Error>> + Send;

    /// Hash data with specified algorithm
    ///
    /// **Consolidated from**: ZeroCostSecurityProvider (optional)
    ///
    /// # Arguments
    /// * `data` - Data to hash
    /// * `algorithm` - Hash algorithm (e.g., "SHA-256", "SHA-512", "BLAKE3")
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` - Hash digest
    /// * `Err(Self::Error)` - Hashing failure
    ///
    /// # Default Implementation
    /// Implementations may provide default hash algorithm if None specified
    fn hash_data(
        &self,
        data: &[u8],
        algorithm: Option<&str>,
    ) -> impl Future<Output = std::result::Result<Vec<u8>, Self::Error>> + Send
    where
        Self::Error: From<crate::NestGateError>,
    {
        // Default implementation can be overridden
        async move {
            let _ = (data, algorithm);
            Err(Self::Error::from(crate::NestGateError::not_implemented(
                "hash_data not implemented for this security provider",
            )))
        }
    }
}
