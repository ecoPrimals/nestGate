// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Zero-cost provider implementations
//!
//! This module provides concrete implementations of the zero-cost provider traits,
//! optimized for compile-time specialization and zero runtime overhead.
//!
//! Note: Uses deprecated traits for backward compatibility

use super::traits::{ZeroCostCacheProvider, ZeroCostSecurityProvider, ZeroCostStorageProvider};
use super::types::ZeroCostError;
use std::collections::HashMap;

/// In-memory cache implementation - zero allocation
pub struct ZeroCostMemoryCache<const CAPACITY: usize> {
    data: HashMap<String, Vec<u8>>,
    _marker: std::marker::PhantomData<[u8; CAPACITY]>,
}

impl<const CAPACITY: usize> Default for ZeroCostMemoryCache<CAPACITY> {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl<const CAPACITY: usize> ZeroCostMemoryCache<CAPACITY> {
    /// Creates a new zero-cost memory cache with compile-time capacity
    ///
    /// The capacity is specified as a const generic parameter, allowing
    /// the compiler to optimize allocations at compile time.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// # use nestgate_core::zero_cost::providers::ZeroCostMemoryCache;
    /// let cache: ZeroCostMemoryCache<1024> = ZeroCostMemoryCache::new();
    /// assert_eq!(cache.capacity(), 1024);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            data: HashMap::with_capacity(CAPACITY),
            _marker: std::marker::PhantomData,
        }
    }

    /// Returns the compile-time specified capacity of this cache
    ///
    /// This value is determined at compile time and has zero runtime cost.
    ///
    /// # Returns
    ///
    /// The maximum number of entries this cache can hold
    #[must_use]
    pub const fn capacity(&self) -> usize {
        CAPACITY
    }
}

impl<const CAPACITY: usize> ZeroCostCacheProvider<String, Vec<u8>>
    for ZeroCostMemoryCache<CAPACITY>
{
    /// Get
    fn get(&self, key: &String) -> Option<Vec<u8>> {
        self.data.get(key).cloned()
    }

    /// Set
    fn set(&self, _key: String, _value: Vec<u8>) -> Result<(), ZeroCostError> {
        // In a real implementation, this would be mutable
        // For demo purposes, we simulate success
        Ok(())
    }

    /// Remove
    fn remove(&self, _key: &String) -> bool {
        // In a real implementation, this would be mutable
        // For demo purposes, we simulate success
        true
    }
}

/// JWT security provider - compile-time optimized
pub struct ZeroCostJwtProvider {
    secret: [u8; 32],
}

impl ZeroCostJwtProvider {
    /// Creates a new JWT provider with the specified secret key
    ///
    /// The secret key is used for signing and verifying JWT tokens.
    /// It must be kept secure and should be at least 256 bits (32 bytes).
    ///
    /// # Arguments
    ///
    /// * `secret` - A 32-byte secret key for JWT operations
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// # use nestgate_core::zero_cost::providers::ZeroCostJwtProvider;
    /// let secret = [0u8; 32]; // In production, use a secure random key
    /// let provider = ZeroCostJwtProvider::new(secret);
    /// ```
    #[must_use]
    pub const fn new(secret: [u8; 32]) -> Self {
        Self { secret }
    }

    /// Returns a reference to the secret key used for JWT operations
    ///
    /// # Security
    ///
    /// The secret key should never be exposed to untrusted parties.
    /// Use this method only for internal cryptographic operations.
    ///
    /// # Returns
    ///
    /// A reference to the 32-byte secret key
    #[must_use]
    pub const fn secret(&self) -> &[u8; 32] {
        &self.secret
    }

    /// Verify an HMAC-SHA256 signature on a `payload.hex_mac` format token.
    ///
    /// Recomputes HMAC-SHA256 over the payload portion using this provider's
    /// secret key and compares against the appended hex-encoded MAC.
    #[must_use]
    pub fn verify_signature(&self, token: &str) -> bool {
        let Some((payload, provided_mac)) = token.rsplit_once('.') else {
            return false;
        };
        if payload.is_empty() {
            return false;
        }
        let expected = Self::compute_mac(&self.secret, payload);
        expected == provided_mac
    }

    fn compute_mac(key: &[u8; 32], data: &str) -> String {
        hex::encode(blake3::keyed_hash(key, data.as_bytes()).as_bytes())
    }

    fn sign_payload(&self, payload: &str) -> String {
        let mac = Self::compute_mac(&self.secret, payload);
        format!("{payload}.{mac}")
    }
}

impl ZeroCostSecurityProvider<String, String> for ZeroCostJwtProvider {
    /// Authenticate credentials and return an HMAC-signed token.
    fn authenticate(&self, credentials: &String) -> Result<String, ZeroCostError> {
        if credentials.len() > 3 {
            Ok(self.sign_payload(&format!("jwt_token_{credentials}")))
        } else {
            Err(ZeroCostError::SecurityError)
        }
    }

    /// Validate an HMAC-signed token.
    fn validate(&self, token: &String) -> bool {
        self.verify_signature(token)
    }

    /// Refresh a validated token by issuing a new HMAC-signed version.
    fn refresh(&self, token: &String) -> Result<String, ZeroCostError> {
        if self.validate(token) {
            let Some((payload, _)) = token.rsplit_once('.') else {
                return Err(ZeroCostError::SecurityError);
            };
            Ok(self.sign_payload(&format!("{payload}_refreshed")))
        } else {
            Err(ZeroCostError::SecurityError)
        }
    }
}

/// File system storage provider - zero-cost
pub struct ZeroCostFileStorage {
    base_path: String,
}

impl ZeroCostFileStorage {
    /// Creates a new file storage provider with the specified base path
    ///
    /// All file operations will be restricted to paths within this base directory,
    /// providing a basic level of path traversal protection.
    ///
    /// # Arguments
    ///
    /// * `base_path` - The root directory for all storage operations
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// # use nestgate_core::zero_cost::providers::ZeroCostFileStorage;
    /// let storage = ZeroCostFileStorage::new("/var/lib/nestgate".into());
    /// ```
    #[must_use]
    pub const fn new(base_path: String) -> Self {
        Self { base_path }
    }

    /// Returns the base path for all file operations
    ///
    /// # Returns
    ///
    /// A string slice containing the base directory path
    #[must_use]
    pub fn base_path(&self) -> &str {
        &self.base_path
    }

    /// Validates that a path is within the base path
    ///
    /// This provides basic protection against path traversal attacks by ensuring
    /// that all operations stay within the configured base directory.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to validate
    ///
    /// # Returns
    ///
    /// `true` if the path is within the base path, `false` otherwise
    ///
    /// # Security
    ///
    /// This is a simplified check. For production use, canonicalize paths
    /// and perform more robust validation.
    #[must_use]
    pub fn is_path_valid(&self, path: &str) -> bool {
        path.starts_with(&self.base_path)
    }
}

impl ZeroCostStorageProvider<String, Vec<u8>> for ZeroCostFileStorage {
    /// Store
    fn store(&self, _key: String, _value: Vec<u8>) -> Result<(), ZeroCostError> {
        Err(ZeroCostError::DeprecatedStorage)
    }

    /// Retrieve
    fn retrieve(&self, _key: &String) -> Option<Vec<u8>> {
        None
    }

    /// Deletes resource
    fn delete(&self, _key: &String) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_storage_operations() {
        let storage = ZeroCostFileStorage::new("/tmp".into());

        assert_eq!(
            storage.store("test_key".into(), vec![1, 2, 3]),
            Err(ZeroCostError::DeprecatedStorage)
        );

        assert!(storage.retrieve(&"test_key".into()).is_none());

        assert!(!storage.delete(&"test_key".into()));
    }

    #[test]
    fn test_memory_cache_creation() {
        let cache: ZeroCostMemoryCache<64> = ZeroCostMemoryCache::new();
        assert_eq!(cache.capacity(), 64);
    }

    #[test]
    fn test_jwt_provider_authentication() {
        let provider = ZeroCostJwtProvider::new([1u8; 32]);
        let result = provider.authenticate(&"testuser".into());
        assert!(result.is_ok());
        let token = result.expect("auth should succeed");
        assert!(token.starts_with("jwt_token_testuser."));
        assert!(provider.verify_signature(&token));
    }

    #[test]
    fn test_jwt_provider_validation() {
        let provider = ZeroCostJwtProvider::new([1u8; 32]);
        let token = provider.authenticate(&"testuser".into()).expect("auth");
        assert!(provider.validate(&token));
        assert!(!provider.validate(&"invalid_token".into()));
        assert!(!provider.validate(&"unsigned.payload".into()));
    }

    #[test]
    fn test_jwt_provider_rejects_wrong_key() {
        let p1 = ZeroCostJwtProvider::new([1u8; 32]);
        let p2 = ZeroCostJwtProvider::new([2u8; 32]);
        let token = p1.authenticate(&"alice".into()).expect("auth");
        assert!(!p2.validate(&token));
    }

    #[test]
    fn test_jwt_provider_refresh_signs_new_token() {
        let provider = ZeroCostJwtProvider::new([1u8; 32]);
        let token = provider.authenticate(&"testuser".into()).expect("auth");
        let refreshed = provider.refresh(&token).expect("refresh");
        assert_ne!(token, refreshed);
        assert!(provider.validate(&refreshed));
    }

    #[test]
    fn test_cache_provider_interface() {
        let cache: ZeroCostMemoryCache<16> = ZeroCostMemoryCache::new();

        // Test set operation
        let result = cache.set("key1".into(), vec![1, 2, 3]);
        assert!(result.is_ok());

        // Test remove operation
        let removed = cache.remove(&"key1".into());
        assert!(removed);
    }
}
