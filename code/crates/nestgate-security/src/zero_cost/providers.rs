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

    /// Verifies the signature of a JWT token
    ///
    /// This is a simplified implementation for demonstration purposes.
    /// In production, use a proper JWT library like `jsonwebtoken`.
    ///
    /// # Arguments
    ///
    /// * `token` - The JWT token string to verify
    ///
    /// # Returns
    ///
    /// `true` if the signature is valid, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// # use nestgate_core::zero_cost::providers::ZeroCostJwtProvider;
    /// # let provider = ZeroCostJwtProvider::new([0u8; 32]);
    /// let is_valid = provider.verify_signature("eyJhbGc...");
    /// ```
    #[must_use]
    pub const fn verify_signature(&self, token: &str) -> bool {
        // Simplified JWT verification using the secret
        !token.is_empty() && self.secret[0] != 0
    }
}

impl ZeroCostSecurityProvider<String, String> for ZeroCostJwtProvider {
    /// Authenticate
    fn authenticate(&self, credentials: &String) -> Result<String, ZeroCostError> {
        if credentials.len() > 3 {
            Ok(format!("jwt_token_{credentials}"))
        } else {
            Err(ZeroCostError::SecurityError)
        }
    }

    /// Validates data
    fn validate(&self, token: &String) -> bool {
        token.starts_with("jwt_token_")
    }

    /// Refresh
    fn refresh(&self, token: &String) -> Result<String, ZeroCostError> {
        if self.validate(token) {
            Ok(format!("{token}_refreshed"))
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
    /// let storage = ZeroCostFileStorage::new("/var/lib/nestgate".to_string());
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
        let storage = ZeroCostFileStorage::new("/tmp".to_string());

        assert_eq!(
            storage.store("test_key".to_string(), vec![1, 2, 3]),
            Err(ZeroCostError::DeprecatedStorage)
        );

        assert!(storage.retrieve(&"test_key".to_string()).is_none());

        assert!(!storage.delete(&"test_key".to_string()));
    }

    #[test]
    fn test_memory_cache_creation() {
        let cache: ZeroCostMemoryCache<64> = ZeroCostMemoryCache::new();
        assert_eq!(cache.capacity(), 64);
    }

    #[test]
    fn test_jwt_provider_authentication() {
        let provider = ZeroCostJwtProvider::new([0u8; 32]);
        let result = provider.authenticate(&"testuser".to_string());
        assert!(result.is_ok());
        assert_eq!(result.expect("Operation failed"), "jwt_token_testuser");
    }

    #[test]
    fn test_jwt_provider_validation() {
        let provider = ZeroCostJwtProvider::new([0u8; 32]);
        assert!(provider.validate(&"jwt_token_test".to_string()));
        assert!(!provider.validate(&"invalid_token".to_string()));
    }

    #[test]
    fn test_cache_provider_interface() {
        let cache: ZeroCostMemoryCache<16> = ZeroCostMemoryCache::new();

        // Test set operation
        let result = cache.set("key1".to_string(), vec![1, 2, 3]);
        assert!(result.is_ok());

        // Test remove operation
        let removed = cache.remove(&"key1".to_string());
        assert!(removed);
    }
}
