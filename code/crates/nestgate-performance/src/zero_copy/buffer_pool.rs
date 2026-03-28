// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **Zero-Copy Buffer Pool**
//!
//! Memory pool for zero-copy networking operations.
//! Pre-allocated buffers eliminate allocation overhead during I/O.
//!
//! ## Performance Benefits
//!
//! - Zero allocation during network I/O
//! - Cache-line aligned buffers (64 bytes)
//! - Lock-free concurrent access
//! - Automatic buffer recycling
//!
//! ## Safety
//!
//! **✅ 100% SAFE** - Uses safe concurrent queue (zero unsafe code)

use crate::safe_concurrent::SafeConcurrentQueue;
use std::io::{IoSlice, IoSliceMut};

// ==================== BUFFER POOL ====================

/// **ZERO-COPY BUFFER POOL**
///
/// Memory pool for zero-copy networking operations.
/// Pre-allocated buffers eliminate allocation overhead during I/O.
///
/// **✅ 100% SAFE** - Uses safe concurrent queue (zero unsafe code)
pub struct ZeroCopyBufferPool<const BUFFER_SIZE: usize = 65_536, const POOL_SIZE: usize = 1024> {
    available_buffers: SafeConcurrentQueue<ZeroCopyBuffer<BUFFER_SIZE>>,
    total_buffers: std::sync::atomic::AtomicUsize,
    buffer_hits: std::sync::atomic::AtomicU64,
    buffer_misses: std::sync::atomic::AtomicU64,
}

impl<const BUFFER_SIZE: usize, const POOL_SIZE: usize> Default
    for ZeroCopyBufferPool<BUFFER_SIZE, POOL_SIZE>
{
    fn default() -> Self {
        let pool = Self {
            available_buffers: SafeConcurrentQueue::new(),
            total_buffers: std::sync::atomic::AtomicUsize::new(0),
            buffer_hits: std::sync::atomic::AtomicU64::new(0),
            buffer_misses: std::sync::atomic::AtomicU64::new(0),
        };

        // Pre-allocate buffers
        for _ in 0..POOL_SIZE {
            let buffer = ZeroCopyBuffer::new();
            pool.available_buffers.push(buffer);
            pool.total_buffers
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }

        pool
    }
}

impl<const BUFFER_SIZE: usize, const POOL_SIZE: usize> ZeroCopyBufferPool<BUFFER_SIZE, POOL_SIZE> {
    /// Create new zero-copy buffer pool (100% SAFE)
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Get buffer from pool (zero-copy acquisition)
    pub fn acquire_buffer(&self) -> Option<ZeroCopyBuffer<BUFFER_SIZE>> {
        if let Some(buffer) = self.available_buffers.try_pop() {
            self.buffer_hits
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            Some(buffer)
        } else {
            self.buffer_misses
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            // Fallback: create new buffer (rare case)
            Some(ZeroCopyBuffer::new())
        }
    }

    /// Return buffer to pool (zero-copy release)
    pub fn release_buffer(&self, mut buffer: ZeroCopyBuffer<BUFFER_SIZE>) {
        buffer.reset();
        self.available_buffers.push(buffer);
    }

    /// Get pool statistics
    pub fn stats(&self) -> BufferPoolStats {
        BufferPoolStats {
            total_buffers: self
                .total_buffers
                .load(std::sync::atomic::Ordering::Relaxed),
            available_buffers: self.available_buffers.len(),
            buffer_hits: self.buffer_hits.load(std::sync::atomic::Ordering::Relaxed),
            buffer_misses: self
                .buffer_misses
                .load(std::sync::atomic::Ordering::Relaxed),
        }
    }
}

// ==================== ZERO-COPY BUFFER ====================

/// **ZERO-COPY BUFFER**
///
/// Pre-allocated buffer for zero-copy operations.
/// Aligned for optimal DMA and SIMD performance.
#[repr(align(64))] // Cache line aligned for optimal performance
pub struct ZeroCopyBuffer<const SIZE: usize> {
    data: [u8; SIZE],
    length: usize,
    capacity: usize,
    reference_count: std::sync::atomic::AtomicUsize,
}

impl<const SIZE: usize> Default for ZeroCopyBuffer<SIZE> {
    fn default() -> Self {
        Self {
            data: [0u8; SIZE],
            length: 0,
            capacity: SIZE,
            reference_count: std::sync::atomic::AtomicUsize::new(1),
        }
    }
}

impl<const SIZE: usize> ZeroCopyBuffer<SIZE> {
    /// Create new zero-copy buffer
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Get buffer data as slice
    pub fn as_slice(&self) -> &[u8] {
        &self.data[..self.length]
    }

    /// Get buffer data as mutable slice
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data[..self.capacity]
    }

    /// Get buffer for vectored I/O
    pub fn as_io_slice(&self) -> IoSlice<'_> {
        IoSlice::new(&self.data[..self.length])
    }

    /// Get mutable buffer for vectored I/O
    pub fn as_io_slice_mut(&mut self) -> IoSliceMut<'_> {
        IoSliceMut::new(&mut self.data[..self.capacity])
    }

    /// Set buffer length after data is written
    pub fn set_length(&mut self, length: usize) {
        self.length = length.min(self.capacity);
    }

    /// Reset buffer for reuse
    pub fn reset(&mut self) {
        self.length = 0;
        self.reference_count
            .store(1, std::sync::atomic::Ordering::Relaxed);
    }

    /// Get buffer capacity
    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get current length
    pub const fn len(&self) -> usize {
        self.length
    }

    /// Check if buffer is empty
    pub const fn is_empty(&self) -> bool {
        self.length == 0
    }
}

// ==================== STATISTICS ====================

/// Buffer pool statistics
#[derive(Debug, Clone)]
pub struct BufferPoolStats {
    /// Total buffers allocated
    pub total_buffers: usize,
    /// Available buffers in pool
    pub available_buffers: usize,
    /// Buffer hits (acquired from pool)
    pub buffer_hits: u64,
    /// Buffer misses (allocated new)
    pub buffer_misses: u64,
}

impl BufferPoolStats {
    /// Calculate hit rate percentage
    pub fn hit_rate(&self) -> f64 {
        let total = self.buffer_hits + self.buffer_misses;
        if total == 0 {
            0.0
        } else {
            (self.buffer_hits as f64 / total as f64) * 100.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_pool_creation() {
        let pool: ZeroCopyBufferPool<1024, 10> = ZeroCopyBufferPool::new();
        let stats = pool.stats();
        assert_eq!(stats.total_buffers, 10);
    }

    #[test]
    fn test_buffer_acquire_release() {
        let pool: ZeroCopyBufferPool<1024, 10> = ZeroCopyBufferPool::new();
        
        let buffer = pool.acquire_buffer().expect("Should acquire buffer");
        assert_eq!(buffer.capacity(), 1024);
        
        pool.release_buffer(buffer);
        
        let stats = pool.stats();
        assert!(stats.buffer_hits > 0);
    }

    #[test]
    fn test_buffer_operations() {
        let mut buffer = ZeroCopyBuffer::<1024>::new();
        assert_eq!(buffer.capacity(), 1024);
        assert_eq!(buffer.len(), 0);
        assert!(buffer.is_empty());
        
        buffer.set_length(100);
        assert_eq!(buffer.len(), 100);
        assert!(!buffer.is_empty());
        
        buffer.reset();
        assert_eq!(buffer.len(), 0);
    }
}
