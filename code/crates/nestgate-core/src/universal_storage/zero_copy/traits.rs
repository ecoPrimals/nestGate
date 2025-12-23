//! Zero-copy storage traits and interfaces.
//!
//! **MODERNIZED**: Converted from `Pin<Box<dyn Future>>` to native async (RPITIT)
//! **Performance**: Eliminates heap allocation and vtable dispatch overhead
//! **Date**: November 8, 2025

use crate::error::Result;
use crate::universal_storage::canonical_storage::CanonicalStorageBackend;
use super::buffer::{ZeroCopyBuffer, AccessPattern};
use bytes::Bytes;
use std::future::Future;

/// Enhanced zero-copy storage with memory pool and streaming
///
/// **NATIVE ASYNC**: Uses `impl Future` for zero-cost abstraction
pub trait EnhancedZeroCopyStorage: CanonicalStorageBackend {
    /// Write data using zero-copy buffer with access pattern hint
    ///
    /// **MODERNIZED**: Native async eliminates Future boxing overhead
    fn write_zero_copy<'a>(
        &self,
        key: &str,
        data: ZeroCopyBuffer<'a>,
        pattern: AccessPattern,
    ) -> impl Future<Output = Result<()>> + Send + 'a
    where
        Self: 'a;

    /// Read data into zero-copy buffer
    ///
    /// **MODERNIZED**: Direct async method, no heap allocation
    fn read_zero_copy(
        &self,
        key: &str,
    ) -> impl Future<Output = Result<ZeroCopyBuffer<'static>>> + Send + '_;
}

/// Zero-copy storage operations trait
///
/// **MODERNIZED**: Native async patterns for optimal performance
/// This trait extends the canonical storage backend with zero-copy operations
pub trait ZeroCopyStorage: CanonicalStorageBackend {
    /// Write data using zero-copy semantics
    ///
    /// **NATIVE ASYNC**: No vtable overhead, compile-time optimization
    fn write_zero_copy_data(
        &self,
        key: &str,
        data: &[u8],
    ) -> impl Future<Output = Result<()>> + Send + '_;

    /// Read data with zero-copy semantics
    ///
    /// **NATIVE ASYNC**: Direct method dispatch, zero overhead
    fn read_zero_copy_data(
        &self,
        key: &str,
    ) -> impl Future<Output = Result<Bytes>> + Send + '_;

    /// Stream data with zero-copy semantics
    ///
    /// **NATIVE ASYNC**: Eliminates Future boxing for streaming operations
    fn stream_zero_copy_data(
        &self,
        key: &str,
        chunk_size: usize,
    ) -> impl Future<Output = Result<Vec<Bytes>>> + Send + '_;
} 