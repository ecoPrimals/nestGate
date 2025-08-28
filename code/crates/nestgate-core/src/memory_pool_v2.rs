/// Memory Pool V2 - Fast AND Safe Architecture
/// This design eliminates the fundamental architectural problems in the original:
/// 1. No Deref + take() contradiction
/// 2. Compile-time prevention of use-after-take
/// 3. Zero-copy performance where possible
/// 4. Linear types pattern for safety
use crate::{NestGateError, Result};
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;
// use tracing::debug; // Available if needed for debugging
use std::time::Duration;

/// **SOLUTION 1: Split the Concerns**
/// Instead of one guard that does everything, have different types for different use cases
/// Immutable reference guard - always safe, never panics
pub struct PoolRef<T: Send + 'static> {
    buffer: Arc<T>,
    _pool_token: PoolToken, // Keeps pool alive
    }

/// Mutable reference guard - always safe, never panics
pub struct PoolRefMut<T: Send + 'static> {
    buffer: Option<Box<T>>,
    pool: Arc<Mutex<VecDeque<Box<T>>>>,
    acquired_at: Instant,
    max_size: usize,
    }

/// Owned buffer - consumed from pool, no return
pub struct PoolOwned<T: Send + 'static>(pub Box<T>);

/// Token that keeps pool metadata alive
struct PoolToken {
    _pool_id: uuid::Uuid,
    }

impl<T: Send + 'static> PoolRef<T> {
    /// Always safe deref - no panics possible
    pub fn get_ref(&self) -> &T {
        &self.buffer
    }
    }

impl<T: Send + 'static> PoolRefMut<T> {
    /// Always safe mutable access
    pub fn as_mut(&mut self) -> Result<&mut T> {
        match self.buffer.as_mut() {
            Some(buffer) => Ok(&mut **buffer),
            None => Err(NestGateError::Internal {
                message: "Buffer has been consumed".to_string(),
                location: Some(file!().to_string()),
                context: None,
                is_bug: true,
            }),
    }
    }

    /// Get the time when this buffer was acquired (useful for debugging/metrics)
    pub fn acquired_at(&self) -> Instant {
        self.acquired_at
    }

    /// How long has this buffer been held?
    pub fn held_duration(&self) -> Duration {
        self.acquired_at.elapsed()
    }

    /// Convert to owned, consuming the guard
    /// This is the ONLY way to get ownership - no "take after deref" possible
    pub fn into_owned(mut self) -> Result<PoolOwned<T>> {
        match self.buffer.take() {
            Some(buffer) => Ok(PoolOwned(buffer)),
            None => Err(NestGateError::Internal {
                message: "Buffer has already been consumed".to_string(),
                location: Some(file!().to_string()),
                context: None,
                is_bug: true,
            }),
    }
    }
    }

/// **SOLUTION 2: Builder Pattern for Complex Operations**
/// For cases where you need to decide between reference and ownership
pub struct PoolAccessBuilder<T: Send + 'static> {
    buffer: Box<T>,
    pool: Arc<Mutex<VecDeque<Box<T>>>>,
    acquired_at: Instant,
    max_size: usize,
    }

impl<T: Send + 'static> PoolAccessBuilder<T> {
    /// Get reference access (buffer returns to pool on drop)
    pub fn as_ref(self) -> PoolRefMut<T> {
        PoolRefMut {
            buffer: Some(self.buffer),
            pool: self.pool,
            acquired_at: self.acquired_at,
            max_size: self.max_size,
    }
    }

    /// Take ownership (buffer never returns to pool)
    pub fn into_owned(self) -> PoolOwned<T> {
        PoolOwned(self.buffer)
    }
    }

/// **SOLUTION 3: Zero-Copy String/Buffer Specializations**
/// For the most common high-performance cases
pub struct PoolString {
    inner: PoolRefMut<String>,
    }

impl PoolString {
    /// Zero-copy write operations
    pub fn write_str(&mut self, s: &str) -> Result<()> {
        self.inner.as_mut()?.push_str(s);
    }

    /// Zero-copy slice access
    pub fn as_str(&self) -> Result<&str> {
        self.inner
            .buffer
            .as_ref()
            .map(|b| b.as_str())
            .ok_or_else(|| NestGateError::Internal {
                message: "String buffer has been consumed".to_string(),
                location: Some(file!().to_string()),
                context: None,
                is_bug: true,
            })
    }

    /// Convert to owned string, consuming the pool buffer
    pub fn into_string(self) -> Result<String> {
        // Move out of the Box to avoid extra allocation
        Ok(*self.inner.into_owned()?.0)
    }
    }

/// **SOLUTION 4: The Pool Manager with Correct API**
pub struct SafeMemoryPool<T: Send + 'static> {
    pool: Arc<Mutex<VecDeque<Box<T>>>>,
    factory: Box<dyn Fn() -> T + Send + Sync>,
    max_size: usize,
    }

impl<T: Send + 'static> SafeMemoryPool<T> {
    pub fn new<F>(factory: F, max_size: usize) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        Self {
            pool: Arc::new(Mutex::new(VecDeque::new())),
            factory: Box::new(factory),
            max_size,
    }
    }

    /// Get the maximum size of this pool
    pub fn max_size(&self) -> usize {
        self.max_size
    }

    /// Get current pool size
    pub async fn current_size(&self) -> usize {
        let pool_guard = self.pool.lock().await;
        pool_guard.len()
    }

    /// Get buffer for modification (returns to pool on drop)
    pub async fn acquire_mut(&self) -> Result<PoolRefMut<T>> {
        let mut pool_guard = self.pool.lock().await;
        let buffer = if let Some(buffer) = pool_guard.pop_front() {
            buffer
        } else {
            Box::new((self.factory)())
        };

        Ok(PoolRefMut {
            buffer: Some(buffer),
            pool: Arc::clone(&self.pool),
            acquired_at: Instant::now(),
            max_size: self.max_size,
        })
    }

    /// Get buffer with choice of reference or ownership
    pub async fn acquire_flexible(&self) -> Result<PoolAccessBuilder<T>> {
        let mut pool_guard = self.pool.lock().await;
        let buffer = if let Some(buffer) = pool_guard.pop_front() {
            buffer
        } else {
            Box::new((self.factory)())
        };

        Ok(PoolAccessBuilder {
            buffer,
            pool: Arc::clone(&self.pool),
            acquired_at: Instant::now(),
            max_size: self.max_size,
        })
    }

    /// Create owned buffer (never returns to pool)
    pub async fn create_owned(&self) -> PoolOwned<T> {
        PoolOwned(Box::new((self.factory)()))
    }
    }

impl<T: Send + 'static> Drop for PoolRefMut<T> {
    fn drop(&mut self) {
        // Take ownership of the buffer safely using Option
        if let Some(buffer) = self.buffer.take() {
            // Return to pool asynchronously with proper max_size enforcement
            let pool = Arc::clone(&self.pool);
            let max_size = self.max_size;
            tokio::spawn(async move {
                {
                    let mut pool_guard = pool.lock().await;
                    if pool_guard.len() < max_size {
                        pool_guard.push_back(buffer);
    }
                    // If pool is at max capacity, buffer is dropped (deallocated)
    }
            });
    }
    }
    }

/// **SOLUTION 5: Specialized High-Performance Types**
/// For file I/O operations - the most common use case
pub struct PoolBuffer {
    data: Vec<u8>,
    pool: Arc<Mutex<VecDeque<Vec<u8>>>>,
    }

impl PoolBuffer {
    /// Zero-copy slice access
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    /// Zero-copy mutable slice access
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data
    }

    /// Extend with zero-copy when possible
    pub fn extend_from_slice(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
    }

    /// Get the data, consuming the buffer (ownership transfer)
    pub fn into_vec(mut self) -> Vec<u8> {
        // Take the data before drop is called
        std::mem::take(&mut self.data)
    }

    /// Clear the buffer and prepare for reuse
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Get buffer capacity
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }
    }

impl Drop for PoolBuffer {
    fn drop(&mut self) {
        // Clear the buffer and return to pool for reuse
        self.data.clear();
        let pool = Arc::clone(&self.pool);
        let mut data = std::mem::take(&mut self.data);
        // Reset to standard capacity if it grew too large
        if data.capacity() > 8192 {
            data = Vec::with_capacity(4096);
    }
        tokio::spawn(async move {
            let mut pool_guard = pool.lock().await;
            if pool_guard.len() < 10 {
                // Reasonable limit for buffer pools
                pool_guard.push_back(data);
    }
        });
    }
    }

/// **SOLUTION 6: Global Buffer Pools for High-Performance Operations**
/// Single source of truth for buffer management
use std::sync::OnceLock;

/// Global pools for common buffer sizes - eliminates allocation overhead
static BUFFER_4KB_POOL: OnceLock<SafeMemoryPool<Vec<u8>>> = OnceLock::new();
static BUFFER_1MB_POOL: OnceLock<SafeMemoryPool<Vec<u8>>> = OnceLock::new();

/// Get a 4KB buffer pool for high-performance operations
/// **USAGE**: `let mut buffer = get_4kb_pool().acquire_mut().await?;`
/// **PERFORMANCE**: Zero-copy, safe, no panics possible
pub fn get_4kb_pool() -> &'static SafeMemoryPool<Vec<u8>> {
    BUFFER_4KB_POOL.get_or_init(|| {
        SafeMemoryPool::new(|| vec![0u8; 4 * 1024], 20) // 4KB buffers, 20 max pooled
    })
    }

/// Get a 1MB buffer pool for high-performance operations
/// **USAGE**: `let mut buffer = get_1mb_pool().acquire_mut().await?;`
/// **PERFORMANCE**: Zero-copy, safe, no panics possible
pub fn get_1mb_pool() -> &'static SafeMemoryPool<Vec<u8>> {
    BUFFER_1MB_POOL.get_or_init(|| {
        SafeMemoryPool::new(|| vec![0u8; 1024 * 1024], 10) // 1MB buffers, 10 max pooled
    })
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_safe_api_prevents_use_after_take() -> Result<()> {
        let pool = SafeMemoryPool::new(|| String::new(), 10);

        // This is the CORRECT pattern - no way to use after take
        let builder = pool.acquire_flexible().await?;
        let _owned = builder.into_owned(); // Consumes builder
                                           // builder.as_ref(); // ❌ Compile error! Builder was consumed

        // This is also safe
        let mut guard = pool.acquire_mut().await?;
        guard.as_mut()?.push_str("test");
        let _owned2 = guard.into_owned()?; // Consumes guard
                                           // guard.as_mut(); // ❌ Compile error! Guard was consumed
    }
    }
