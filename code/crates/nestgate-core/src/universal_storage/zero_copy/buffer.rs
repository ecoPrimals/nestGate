//! Zero-copy buffer implementations and memory management.

use bytes::Bytes;
use std::ops::Deref;

/// Zero-copy data buffer for storage operations
///
/// This enum allows us to handle data without copying, using either:
/// - Borrowed data (zero-copy from caller)
/// - Owned data (when copying is necessary)
/// - Shared data (reference-counted for multiple readers)
#[derive(Debug, Clone)]
/// Zerocopybuffer
pub enum ZeroCopyBuffer<'a> {
    /// Borrowed data - no allocation, points to caller's memory
    Borrowed(&'a [u8]),
    /// Owned data - allocated when necessary
    Owned(Vec<u8>),
    /// Shared data - reference counted for multiple consumers
    Shared(Bytes),
}

impl<'a> ZeroCopyBuffer<'a> {
    /// Create a zero-copy buffer from borrowed data
    pub fn borrowed(data: &'a [u8]) -> Self {
        Self::Borrowed(data)
    }

    /// Create a zero-copy buffer from owned data
    pub fn owned(data: Vec<u8>) -> Self {
        Self::Owned(data)
    }

    /// Create a zero-copy buffer from shared data
    pub fn shared(data: Bytes) -> Self {
        Self::Shared(data)
    }

    /// Get the data as a slice
    pub fn as_slice(&self) -> &[u8] {
        match self {
            Self::Borrowed(data) => data,
            Self::Owned(data) => data.as_slice(),
            Self::Shared(data) => data.as_ref(),
        }
    }

    /// Get the length of the buffer
    pub fn len(&self) -> usize {
        match self {
            Self::Borrowed(data) => data.len(),
            Self::Owned(data) => data.len(),
            Self::Shared(data) => data.len(),
        }
    }

    /// Check if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Convert to owned bytes
    pub fn to_bytes(self) -> Bytes {
        match self {
            Self::Borrowed(data) => Bytes::copy_from_slice(data),
            Self::Owned(data) => Bytes::from(data),
            Self::Shared(data) => data,
        }
    }
}

impl<'a> Deref for ZeroCopyBuffer<'a> {
    /// Type alias for Target
    type Target = [u8];

    /// Deref
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<'a> AsRef<[u8]> for ZeroCopyBuffer<'a> {
    /// Returns as Ref
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

/// Advanced zero-copy buffer with memory pool integration
#[derive(Debug)]
/// Advancedzerocopybuffer
pub enum AdvancedZeroCopyBuffer<'a> {
    /// Standard zero-copy buffer
    Standard(ZeroCopyBuffer<'a>),
    /// Pooled buffer for efficient reuse
    Pooled(PooledBuffer),
    /// Memory-mapped buffer for large files
    Mmap(Bytes),
}

/// Pooled buffer for efficient memory reuse
#[derive(Debug)]
/// Pooledbuffer
pub struct PooledBuffer {
    data: Vec<u8>,
    capacity: usize,
    pool_id: Option<usize>,
}

impl PooledBuffer {
    /// Create a new pooled buffer
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            capacity,
            pool_id: None,
        }
    }

    /// Get the data as a slice
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    /// Get mutable access to the data
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data
    }

    /// Resize the buffer
    pub fn resize(&mut self, new_len: usize) {
        self.data.resize(new_len, 0);
    }

    /// Clear the buffer for reuse
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Get the capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get the current length
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Deref for PooledBuffer {
    /// Type alias for Target
    type Target = [u8];

    /// Deref
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

/// Access pattern hint for optimization
#[derive(Debug, Clone, Copy)]
/// Accesspattern
pub enum AccessPattern {
    /// Sequential
    Sequential,
    /// Random
    Random,
    /// Writeonce
    WriteOnce,
    /// Readmany
    ReadMany,
} 