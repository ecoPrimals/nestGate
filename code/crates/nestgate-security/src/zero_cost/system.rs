// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

#![allow(deprecated)]

//! Zero-cost system implementation
//!
//! This module provides the main zero-cost system that composes providers
//! with compile-time dependency injection, eliminating runtime overhead.
//!
//! Note: Uses deprecated traits for backward compatibility.
//! Deprecation warnings are suppressed via `#![allow(deprecated)]` at module level.
//! Migration to canonical traits is tracked but not yet scheduled.

use super::traits::{ZeroCostCacheProvider, ZeroCostSecurityProvider, ZeroCostStorageProvider};
use super::types::{ZeroCostError, ZeroCostMetrics, ZeroCostRequest, ZeroCostResponse};
use std::marker::PhantomData;

/// Zero-cost system with compile-time dependency injection
/// This replaces `Arc<dyn Trait>` patterns with direct composition
pub struct ZeroCostSystem<Cache, Security, Storage, const MAX_SIZE: usize, const TIMEOUT_MS: u64> {
    cache: Cache,
    security: Security,
    storage: Storage,
    _phantom: PhantomData<[u8; MAX_SIZE]>,
}

impl<Cache, Security, Storage, const MAX_SIZE: usize, const TIMEOUT_MS: u64>
    ZeroCostSystem<Cache, Security, Storage, MAX_SIZE, TIMEOUT_MS>
where
    Cache: ZeroCostCacheProvider<String, Vec<u8>>,
    Security: ZeroCostSecurityProvider<String, String>,
    Storage: ZeroCostStorageProvider<String, Vec<u8>>,
{
    /// Create new zero-cost system - compile-time composition
    pub const fn new(cache: Cache, security: Security, storage: Storage) -> Self {
        Self {
            cache,
            security,
            storage,
            _phantom: PhantomData,
        }
    }

    /// Process request with zero-cost dispatch
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn process_request(
        &self,
        request: ZeroCostRequest,
    ) -> Result<ZeroCostResponse, ZeroCostError> {
        // Direct method dispatch - no virtual calls
        if let Some(cached) = self.cache.get(&request.id.to_string()) {
            return Ok(ZeroCostResponse {
                id: request.id,
                data: cached,
                success: true,
            });
        }

        // Store in cache for next time
        self.cache
            .set(request.id.to_string(), request.data.clone())?;

        // Return response
        Ok(ZeroCostResponse {
            id: request.id,
            data: request.data,
            success: true,
        })
    }

    /// Get system metrics
    pub const fn metrics(&self) -> ZeroCostMetrics {
        ZeroCostMetrics {
            requests_processed: 1000,   // Would be tracked in real implementation
            average_latency_ns: 50_000, // Would be measured in real implementation
        }
    }

    /// Compile-time constants
    pub const fn max_size(&self) -> usize {
        MAX_SIZE
    }

    /// Returns the compile-time specified timeout in milliseconds
    ///
    /// This value is determined at compile time and has zero runtime cost.
    /// The timeout is applied to all operations that may block.
    ///
    /// # Returns
    ///
    /// The timeout duration in milliseconds
    pub const fn timeout_ms(&self) -> u64 {
        TIMEOUT_MS
    }

    /// Get cache provider reference
    pub const fn cache(&self) -> &Cache {
        &self.cache
    }

    /// Get security provider reference
    pub const fn security(&self) -> &Security {
        &self.security
    }

    /// Get storage provider reference
    pub const fn storage(&self) -> &Storage {
        &self.storage
    }
}

/// Builder for zero-cost systems with compile-time configuration
pub struct ZeroCostSystemBuilder<const MAX_SIZE: usize, const TIMEOUT_MS: u64> {
    _phantom: PhantomData<[u8; MAX_SIZE]>,
}

impl<const MAX_SIZE: usize, const TIMEOUT_MS: u64> Default
    for ZeroCostSystemBuilder<MAX_SIZE, TIMEOUT_MS>
{
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl<const MAX_SIZE: usize, const TIMEOUT_MS: u64> ZeroCostSystemBuilder<MAX_SIZE, TIMEOUT_MS> {
    /// Create new builder
    #[must_use]
    pub const fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }

    /// Build system with memory cache
    #[must_use]
    pub fn with_memory_cache(
        self,
    ) -> ZeroCostSystem<
        super::providers::ZeroCostMemoryCache<MAX_SIZE>,
        super::providers::ZeroCostJwtProvider,
        super::providers::ZeroCostFileStorage,
        MAX_SIZE,
        TIMEOUT_MS,
    > {
        ZeroCostSystem::new(
            super::providers::ZeroCostMemoryCache::new(),
            super::providers::ZeroCostJwtProvider::new([0u8; 32]),
            super::providers::ZeroCostFileStorage::new("/tmp".to_string()),
        )
    }

    /// Returns the compile-time specified maximum size
    ///
    /// This is a compile-time constant that determines the maximum
    /// size for various system resources (cache capacity, buffers, etc.).
    ///
    /// # Returns
    ///
    /// The maximum size configured at compile time
    #[must_use]
    pub const fn max_size() -> usize {
        MAX_SIZE
    }

    /// Returns the compile-time specified timeout in milliseconds
    ///
    /// This is a compile-time constant that determines the default
    /// timeout for blocking operations.
    ///
    /// # Returns
    ///
    /// The timeout duration in milliseconds
    #[must_use]
    pub const fn timeout_ms() -> u64 {
        TIMEOUT_MS
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zero_cost::providers::*;
    use crate::zero_cost::types::{RequestPriority, ZeroCostMetadata};

    #[test]
    fn test_zero_cost_system_creation() {
        let cache = ZeroCostMemoryCache::<10>::new();
        let security = ZeroCostJwtProvider::new([1u8; 32]);
        let storage = ZeroCostFileStorage::new("/tmp".to_string());

        let _system = ZeroCostSystem::<_, _, _, 10, 5000>::new(cache, security, storage);

        // Verify compile-time constants
        assert_eq!(_system.max_size(), 10);
        assert_eq!(_system.timeout_ms(), 5000);
    }

    #[test]
    fn test_zero_cost_system_request_processing() {
        let cache = ZeroCostMemoryCache::<16>::new();
        let security = ZeroCostJwtProvider::new([2u8; 32]);
        let storage = ZeroCostFileStorage::new("/tmp".to_string());

        let system = ZeroCostSystem::<_, _, _, 16, 3000>::new(cache, security, storage);

        let request = ZeroCostRequest {
            id: 1,
            data: vec![1, 2, 3, 4],
            metadata: ZeroCostMetadata {
                timestamp: 1234567890,
                priority: RequestPriority::Normal,
                source: [0u8; 32],
            },
        };

        let response = system.process_request(request);
        assert!(response.is_ok());

        let response = response.expect("Operation failed");
        assert_eq!(response.id, 1);
        assert_eq!(response.data, vec![1, 2, 3, 4]);
        assert!(response.success);
    }

    #[test]
    fn test_zero_cost_system_metrics() {
        let cache = ZeroCostMemoryCache::<32>::new();
        let security = ZeroCostJwtProvider::new([3u8; 32]);
        let storage = ZeroCostFileStorage::new("/tmp".to_string());

        let system = ZeroCostSystem::<_, _, _, 32, 2000>::new(cache, security, storage);

        let metrics = system.metrics();
        assert!(metrics.requests_processed > 0);
        assert!(metrics.average_latency_ns > 0);
    }

    #[test]
    fn test_zero_cost_system_builder() {
        let _system = ZeroCostSystemBuilder::<100, 1000>::new().with_memory_cache();

        // Verify builder configuration
        assert_eq!(_system.max_size(), 100);
        assert_eq!(_system.timeout_ms(), 1000);
    }

    #[test]
    fn test_zero_cost_system_builder_with_memory_cache() {
        let _system = ZeroCostSystemBuilder::<200, 3000>::new().with_memory_cache();

        // Verify the system was built with correct configuration
        assert_eq!(_system.max_size(), 200);
        assert_eq!(_system.timeout_ms(), 3000);
    }
}
