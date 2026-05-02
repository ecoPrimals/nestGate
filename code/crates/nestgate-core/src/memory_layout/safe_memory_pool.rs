// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **SAFE HIGH-PERFORMANCE MEMORY POOL**
//!
//! Zero-allocation memory pool using 100% safe Rust with RAII patterns.
//! This module provides the same performance intent as raw-pointer slabs while
//! using a `Mutex` wrapping `Option` with type parameter `T` per slot (no lock-free shared buffers, no manual `Send`/`Sync` impl tricks).
//!
//! ## Key Innovations
//!
//! - **RAII Handles**: Automatic deallocation through Drop
//! - **Type Safety**: Compile-time guarantees prevent double-free
//! - **Memory safety**: uses mutex-backed slots and RAII handles exclusively
//! - **Concurrency**: Bitmap + atomics for allocation; per-slot mutex for storage
//!
//! ## Example
//!
//! ```rust
//! use nestgate_core::memory_layout::safe_memory_pool::SafeMemoryPool;
//!
//! let pool = SafeMemoryPool::<String, 32>::new();
//!
//! let handle = pool.allocate("Hello".to_string()).expect("pool has capacity");
//!
//! println!("Value: {}", handle.value());
//!
//! // Automatic deallocation when handle drops
//! ```

use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

use parking_lot::{MappedMutexGuard, Mutex, MutexGuard};

/// Safe memory pool with automatic deallocation via RAII
///
/// This pool provides O(1) allocation and deallocation using only memory-safe Rust.
/// Uses a bitmap allocator with atomic operations for thread-safety.
pub struct SafeMemoryPool<T, const CAPACITY: usize = 1024> {
    /// Shared pool state
    inner: Arc<PoolInner<T, CAPACITY>>,
}

/// Inner pool state (shared between pool and handles)
struct PoolInner<T, const CAPACITY: usize> {
    /// Storage slots — mutex per slot (no hand-rolled interior mutability)
    slots: Box<[Mutex<Option<T>>; CAPACITY]>,

    /// Allocation bitmap (1 = free, 0 = allocated)
    /// Uses atomic for thread-safe operations
    free_bitmap: AtomicU64,

    /// Statistics
    stats: PoolStats,
}

/// Pool statistics
#[derive(Debug)]
pub struct PoolStats {
    allocated: AtomicUsize,
    deallocated: AtomicUsize,
    peak_usage: AtomicUsize,
}

type MutexOption<T> = Mutex<Option<T>>;

/// RAII handle to allocated memory
///
/// Automatically deallocates when dropped - no manual deallocation needed!
pub struct PoolHandle<T, const CAPACITY: usize> {
    /// Slot index
    slot: usize,
    /// Reference to pool for deallocation
    pool: Arc<PoolInner<T, CAPACITY>>,
    /// Phantom data for invariance
    _phantom: PhantomData<T>,
}

impl<T, const CAPACITY: usize> SafeMemoryPool<T, CAPACITY> {
    /// Create new safe memory pool
    ///
    /// # Panics
    ///
    /// Panics if CAPACITY > 64 (bitmap limitation for this implementation)
    #[must_use]
    pub fn new() -> Self {
        assert!(
            CAPACITY > 0 && CAPACITY < 64,
            "Safe memory pool supports 1..63 slots (single u64 bitmap)"
        );

        // Fixed-size array: exactly `CAPACITY` mutex slots, no fallible `Vec` → array conversion.
        let slots: Box<[MutexOption<T>; CAPACITY]> =
            Box::new(std::array::from_fn(|_| MutexOption::new(None)));

        // All bits set = all slots free
        let free_bitmap = (1u64 << CAPACITY) - 1;

        Self {
            inner: Arc::new(PoolInner {
                slots,
                free_bitmap: AtomicU64::new(free_bitmap),
                stats: PoolStats {
                    allocated: AtomicUsize::new(0),
                    deallocated: AtomicUsize::new(0),
                    peak_usage: AtomicUsize::new(0),
                },
            }),
        }
    }

    /// Allocate value from pool
    ///
    /// Returns RAII handle that automatically deallocates on drop.
    pub fn allocate(&self, value: T) -> Option<PoolHandle<T, CAPACITY>> {
        loop {
            let current = self.inner.free_bitmap.load(Ordering::Acquire);

            if current == 0 {
                return None; // Pool exhausted
            }

            let slot = current.trailing_zeros() as usize;
            let new_bitmap = current & !(1u64 << slot);

            if self
                .inner
                .free_bitmap
                .compare_exchange(current, new_bitmap, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                *self.inner.slots[slot].lock() = Some(value);

                self.inner.stats.allocated.fetch_add(1, Ordering::Relaxed);
                let usage = self.inner.stats.allocated.load(Ordering::Relaxed)
                    - self.inner.stats.deallocated.load(Ordering::Relaxed);
                let peak = self.inner.stats.peak_usage.load(Ordering::Relaxed);
                if usage > peak {
                    self.inner.stats.peak_usage.store(usage, Ordering::Relaxed);
                }

                return Some(PoolHandle {
                    slot,
                    pool: Arc::clone(&self.inner),
                    _phantom: PhantomData,
                });
            }
        }
    }

    /// Get current pool utilization (0.0 to 1.0)
    #[must_use]
    pub fn utilization(&self) -> f64 {
        let allocated = self.inner.stats.allocated.load(Ordering::Relaxed);
        let deallocated = self.inner.stats.deallocated.load(Ordering::Relaxed);
        let in_use = allocated.saturating_sub(deallocated);
        in_use as f64 / CAPACITY as f64
    }

    /// Get pool statistics
    #[must_use]
    pub fn stats(&self) -> &PoolStats {
        &self.inner.stats
    }

    /// Get available slots
    #[must_use]
    pub fn available(&self) -> usize {
        self.inner.free_bitmap.load(Ordering::Relaxed).count_ones() as usize
    }
}

impl<T, const CAPACITY: usize> Default for SafeMemoryPool<T, CAPACITY> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const CAPACITY: usize> Clone for SafeMemoryPool<T, CAPACITY> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T, const CAPACITY: usize> PoolHandle<T, CAPACITY> {
    /// Get immutable reference to value (locks the slot for the duration of the guard).
    ///
    /// # Panics
    ///
    /// Panics if the slot is unexpectedly empty — indicates a pool invariant violation.
    #[expect(
        clippy::expect_used,
        reason = "structural invariant: PoolHandle only exists for occupied slots"
    )]
    pub fn value(&self) -> MappedMutexGuard<'_, T> {
        MutexGuard::map(self.pool.slots[self.slot].lock(), |opt: &mut Option<T>| {
            opt.as_mut()
                .expect("PoolHandle invariant: slot must be occupied")
        })
    }

    /// Get mutable reference to value.
    ///
    /// # Panics
    ///
    /// Panics if the slot is unexpectedly empty — indicates a pool invariant violation.
    #[expect(
        clippy::expect_used,
        reason = "structural invariant: PoolHandle only exists for occupied slots"
    )]
    pub fn value_mut(&mut self) -> MappedMutexGuard<'_, T> {
        MutexGuard::map(self.pool.slots[self.slot].lock(), |opt: &mut Option<T>| {
            opt.as_mut()
                .expect("PoolHandle invariant: slot must be occupied")
        })
    }

    /// Take ownership of value, consuming the handle.
    ///
    /// # Panics
    ///
    /// Panics if the slot is unexpectedly empty — indicates a pool invariant violation.
    #[must_use]
    #[expect(
        clippy::expect_used,
        reason = "structural invariant: PoolHandle only exists for occupied slots"
    )]
    pub fn into_inner(self) -> T {
        let value = {
            let mut guard = self.pool.slots[self.slot].lock();
            guard
                .take()
                .expect("PoolHandle invariant: slot must be occupied")
        };

        let mask = 1u64 << self.slot;
        self.pool.free_bitmap.fetch_or(mask, Ordering::Release);
        self.pool.stats.deallocated.fetch_add(1, Ordering::Relaxed);

        std::mem::forget(self);

        value
    }
}

impl<T, const CAPACITY: usize> Drop for PoolHandle<T, CAPACITY> {
    fn drop(&mut self) {
        *self.pool.slots[self.slot].lock() = None;

        let mask = 1u64 << self.slot;
        self.pool.free_bitmap.fetch_or(mask, Ordering::Release);

        self.pool.stats.deallocated.fetch_add(1, Ordering::Relaxed);
    }
}

impl PoolStats {
    /// Get total allocations
    pub fn allocated(&self) -> usize {
        self.allocated.load(Ordering::Relaxed)
    }

    /// Get total deallocations
    pub fn deallocated(&self) -> usize {
        self.deallocated.load(Ordering::Relaxed)
    }

    /// Get currently in-use slots
    pub fn in_use(&self) -> usize {
        self.allocated().saturating_sub(self.deallocated())
    }

    /// Get peak usage
    pub fn peak_usage(&self) -> usize {
        self.peak_usage.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_allocation_and_automatic_deallocation() {
        let pool = SafeMemoryPool::<String, 16>::new();

        let handle = pool
            .allocate("Hello".to_string())
            .expect("Allocation failed");
        assert_eq!(handle.value().as_str(), "Hello");
        assert_eq!(pool.available(), 15);

        drop(handle);

        assert_eq!(pool.available(), 16);
    }

    #[test]
    fn test_pool_exhaustion() {
        let pool = SafeMemoryPool::<u32, 4>::new();

        let _h1 = pool.allocate(1).expect("Allocation failed");
        let _h2 = pool.allocate(2).expect("Allocation failed");
        let _h3 = pool.allocate(3).expect("Allocation failed");
        let _h4 = pool.allocate(4).expect("Allocation failed");

        assert!(pool.allocate(5).is_none());
        assert_eq!(pool.available(), 0);
    }

    #[test]
    fn test_pool_reuse() {
        let pool = SafeMemoryPool::<String, 8>::new();

        for i in 0..100 {
            let handle = pool
                .allocate(format!("Value {i}"))
                .expect("Allocation failed");
            assert_eq!(handle.value().as_str(), &format!("Value {i}"));
        }

        assert_eq!(pool.available(), 8);
    }

    #[test]
    fn test_into_inner() {
        let pool = SafeMemoryPool::<String, 8>::new();

        let handle = pool
            .allocate("Test".to_string())
            .expect("Allocation failed");
        assert_eq!(pool.available(), 7);

        let value = handle.into_inner();
        assert_eq!(value, "Test");

        assert_eq!(pool.available(), 8);
    }

    #[test]
    fn test_mutable_access() {
        let pool = SafeMemoryPool::<Vec<i32>, 8>::new();

        let mut handle = pool.allocate(vec![1, 2, 3]).expect("Allocation failed");

        handle.value_mut().push(4);

        assert_eq!(handle.value().as_slice(), &[1, 2, 3, 4]);
    }

    #[test]
    fn test_statistics() {
        let pool = SafeMemoryPool::<i32, 8>::new();

        let h1 = pool.allocate(1).unwrap();
        let h2 = pool.allocate(2).unwrap();

        assert_eq!(pool.stats().allocated(), 2);
        assert_eq!(pool.stats().in_use(), 2);

        drop(h1);
        assert_eq!(pool.stats().deallocated(), 1);
        assert_eq!(pool.stats().in_use(), 1);

        drop(h2);
        assert_eq!(pool.stats().in_use(), 0);
    }
}
