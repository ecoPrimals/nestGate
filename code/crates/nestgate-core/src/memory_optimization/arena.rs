//! **ARENA ALLOCATION**
//!
//! Bump allocator for efficient batch allocations.

use std::sync::Mutex;

/// **MEMORY ARENA**
///
/// Bump allocator for efficient batch allocations
pub struct MemoryArena {
    buffer: Mutex<Vec<u8>>,
    offset: Mutex<usize>,
    capacity: usize,
}

impl MemoryArena {
    /// Create new memory arena with capacity
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: Mutex::new(Vec::with_capacity(capacity)),
            offset: Mutex::new(0),
            capacity,
        }
    }
    
    /// Allocate bytes from arena
    pub fn allocate(&self, size: usize) -> Option<usize> {
        let mut offset = self.offset.lock().unwrap_or_else(|poisoned| {
            poisoned.into_inner()
        });
        
        if *offset + size <= self.capacity {
            let current = *offset;
            *offset += size;
            Some(current)
        } else {
            None // Arena full
        }
    }
    
    /// Reset arena (clear all allocations)
    pub fn reset(&self) {
        let mut offset = self.offset.lock().unwrap_or_else(|poisoned| {
            poisoned.into_inner()
        });
        *offset = 0;
    }
    
    /// Get current usage
    #[must_use]
    pub fn usage(&self) -> (usize, usize) {
        let offset = self.offset.lock().unwrap_or_else(|poisoned| {
            poisoned.into_inner()
        });
        (*offset, self.capacity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arena_allocation() {
        let arena = MemoryArena::new(1024);
        assert!(arena.allocate(512).is_some());
        assert!(arena.allocate(512).is_some());
        assert!(arena.allocate(1).is_none()); // Arena full
    }

    #[test]
    fn test_arena_reset() {
        let arena = MemoryArena::new(1024);
        arena.allocate(1024);
        arena.reset();
        assert!(arena.allocate(1024).is_some());
    }
}
