//! Zero-copy storage traits and interfaces.

use crate::error::Result;
use crate::universal_storage::canonical_storage::CanonicalStorageBackend;
use super::buffer::{ZeroCopyBuffer, AccessPattern};
use bytes::Bytes;
use std::future::Future;
use std::pin::Pin;

/// Enhanced zero-copy storage with memory pool and streaming
pub trait EnhancedZeroCopyStorage: CanonicalStorageBackend {
    /// Write data using zero-copy buffer with access pattern hint
    fn write_zero_copy<'a>(
        &self,
        key: &str,
        data: ZeroCopyBuffer<'a>,
        pattern: AccessPattern,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>>;

    /// Read data into zero-copy buffer
    fn read_zero_copy(
        &self,
        key: &str,
    ) -> Pin<Box<dyn Future<Output = Result<ZeroCopyBuffer<'static>>> + Send + '_>>;
}

/// Zero-copy storage operations trait
///
/// This trait extends the canonical storage backend with zero-copy operations
pub trait ZeroCopyStorage: CanonicalStorageBackend {
    /// Write data using zero-copy semantics
    fn write_zero_copy_data(
        &self,
        key: &str,
        data: &[u8],
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>>;

    /// Read data with zero-copy semantics
    fn read_zero_copy_data(
        &self,
        key: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Bytes>> + Send + '_>>;

    /// Stream data with zero-copy semantics
    fn stream_zero_copy_data(
        &self,
        key: &str,
        chunk_size: usize,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Bytes>>> + Send + '_>>;
} 