//! Simple Memory Pool Implementation
//!
//! A lightweight, high-performance memory pool for reducing allocation overhead.
//! This implementation focuses on simplicity and safety while providing
//! measurable performance improvements.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// A simple memory pool for reusing allocations
pub struct SimpleMemoryPool {
    /// Pool of available buffers
    pool: Arc<Mutex<VecDeque<Vec<u8>>>>,
    /// Size of buffers in this pool
    buffer_size: usize,
    /// Maximum number of buffers to keep in pool
    max_pool_size: usize,
}

impl SimpleMemoryPool {
    /// Create a new memory pool
    #[must_use]
    pub fn new(buffer_size: usize, max_pool_size: usize) -> Self {
        Self {
            pool: Arc::new(Mutex::new(VecDeque::new())),
            buffer_size,
            max_pool_size,
        }
    }

    /// Get a buffer from the pool, or allocate a new one
    #[must_use]
    pub fn get_buffer(&self) -> Vec<u8> {
        if let Ok(mut pool) = self.pool.lock() {
            if let Some(mut buffer) = pool.pop_front() {
                // Reuse existing buffer
                buffer.clear();
                buffer.resize(self.buffer_size, 0);
                return buffer;
            }
        }

        // Allocate new buffer if pool is empty
        vec![0; self.buffer_size]
    }

    /// Return a buffer to the pool for reuse
    pub fn return_buffer(&self, buffer: Vec<u8>) {
        if buffer.capacity() >= self.buffer_size {
            if let Ok(mut pool) = self.pool.lock() {
                if pool.len() < self.max_pool_size {
                    pool.push_back(buffer);
                }
                // If pool is full, just drop the buffer
            }
        }
    }

    /// Get pool statistics
    #[must_use]
    pub fn stats(&self) -> PoolStats {
        let pool_size = self.pool.lock().map(|p| p.len()).unwrap_or(0);
        PoolStats {
            buffer_size: self.buffer_size,
            max_pool_size: self.max_pool_size,
            current_pool_size: pool_size,
        }
    }
}

/// Statistics about the memory pool
#[derive(Debug, Clone)]
pub struct PoolStats {
    pub buffer_size: usize,
    pub max_pool_size: usize,
    pub current_pool_size: usize,
}

/// A managed buffer that automatically returns to pool when dropped
pub struct PooledBuffer {
    buffer: Option<Vec<u8>>,
    pool: Arc<Mutex<VecDeque<Vec<u8>>>>,
    max_pool_size: usize,
}

impl PooledBuffer {
    /// Create a new pooled buffer
    pub fn new(buffer: Vec<u8>, pool: Arc<Mutex<VecDeque<Vec<u8>>>>, max_pool_size: usize) -> Self {
        Self {
            buffer: Some(buffer),
            pool,
            max_pool_size,
        }
    }

    /// Get a mutable reference to the buffer
    ///
    /// # Panics
    ///
    /// Panics if the buffer has been taken (logic error)
    pub fn buffer_mut(&mut self) -> &mut Vec<u8> {
        // Safety: Buffer should always be Some during normal usage (before drop)
        // If this fails, it indicates a serious logic error in the buffer lifecycle
        self.buffer.as_mut().unwrap_or_else(|| {
            // This should never happen in correct usage
            panic!("Logic error: Buffer has been taken before drop")
        })
    }

    /// Get a reference to the buffer
    ///
    /// # Panics
    ///
    /// Panics if the buffer has been taken (logic error)
    #[must_use]
    pub fn buffer_ref(&self) -> &Vec<u8> {
        // Safety: Buffer should always be Some during normal usage (before drop)
        // If this fails, it indicates a serious logic error in the buffer lifecycle
        self.buffer.as_ref().unwrap_or_else(|| {
            // This should never happen in correct usage
            panic!("Logic error: Buffer has been taken before drop")
        })
    }

    /// Get the length of the buffer
    #[must_use]
    pub fn len(&self) -> usize {
        self.buffer_ref().len()
    }

    /// Check if the buffer is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.buffer_ref().is_empty()
    }
}

impl Drop for PooledBuffer {
    fn drop(&mut self) {
        if let Some(buffer) = self.buffer.take() {
            // Return buffer to pool if there's space
            if let Ok(mut pool) = self.pool.lock() {
                if pool.len() < self.max_pool_size {
                    pool.push_back(buffer);
                }
            }
        }
    }
}

impl std::ops::Deref for PooledBuffer {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        self.buffer_ref()
    }
}

impl std::ops::DerefMut for PooledBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.buffer_mut()
    }
}

/// Enhanced memory pool with RAII buffer management
pub struct EnhancedMemoryPool {
    inner: SimpleMemoryPool,
}

impl EnhancedMemoryPool {
    /// Create a new enhanced memory pool
    #[must_use]
    pub fn new(buffer_size: usize, max_pool_size: usize) -> Self {
        Self {
            inner: SimpleMemoryPool::new(buffer_size, max_pool_size),
        }
    }

    /// Get a managed buffer that automatically returns to pool
    #[must_use]
    pub fn get_managed_buffer(&self) -> PooledBuffer {
        let buffer = self.inner.get_buffer();
        PooledBuffer::new(buffer, self.inner.pool.clone(), self.inner.max_pool_size)
    }

    /// Get pool statistics
    #[must_use]
    pub fn stats(&self) -> PoolStats {
        self.inner.stats()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_memory_pool() {
        let pool = SimpleMemoryPool::new(1024, 5);

        // Get a buffer
        let buffer1 = pool.get_buffer();
        assert_eq!(buffer1.len(), 1024);

        // Return it
        pool.return_buffer(buffer1);

        // Get another buffer (should reuse the first one)
        let buffer2 = pool.get_buffer();
        assert_eq!(buffer2.len(), 1024);

        let stats = pool.stats();
        assert_eq!(stats.buffer_size, 1024);
        assert_eq!(stats.max_pool_size, 5);
    }

    #[test]
    fn test_enhanced_memory_pool() {
        let pool = EnhancedMemoryPool::new(512, 3);

        {
            let mut buffer = pool.get_managed_buffer();
            assert_eq!(buffer.len(), 512);
            buffer.push(42);
            assert_eq!(buffer.len(), 513);
        } // Buffer automatically returned to pool here

        let stats = pool.stats();
        assert_eq!(stats.buffer_size, 512);
        assert_eq!(stats.max_pool_size, 3);
        assert_eq!(stats.current_pool_size, 1);
    }

    #[test]
    fn test_pool_overflow() {
        let pool = SimpleMemoryPool::new(100, 2);

        // Fill the pool
        let buffer1 = pool.get_buffer();
        let buffer2 = pool.get_buffer();
        let buffer3 = pool.get_buffer();

        pool.return_buffer(buffer1);
        pool.return_buffer(buffer2);
        pool.return_buffer(buffer3); // This should be dropped due to max_pool_size

        let stats = pool.stats();
        assert_eq!(stats.current_pool_size, 2); // Max pool size respected
    }
}
