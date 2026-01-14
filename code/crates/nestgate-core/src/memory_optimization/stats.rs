//! **MEMORY STATISTICS**
//!
//! Comprehensive memory usage tracking and analysis.

use std::sync::atomic::{AtomicU64, Ordering};

/// **MEMORY USAGE STATISTICS**
///
/// Comprehensive memory usage tracking and analysis
#[derive(Debug, Default)]
pub struct MemoryStats {
    /// Total allocations made
    pub total_allocations: AtomicU64,
    /// Total deallocations made
    pub total_deallocations: AtomicU64,
    /// Current active allocations
    pub active_allocations: AtomicU64,
    /// Peak memory usage in bytes
    pub peak_memory_usage: AtomicU64,
    /// Current memory usage in bytes
    pub current_memory_usage: AtomicU64,
    /// Total bytes allocated
    pub total_bytes_allocated: AtomicU64,
    /// Total bytes deallocated
    pub total_bytes_deallocated: AtomicU64,
    /// Large allocation count (>1MB)
    pub large_allocation_count: AtomicU64,
    /// Small allocation count (<1KB)
    pub small_allocation_count: AtomicU64,
}

impl MemoryStats {
    /// Creates a new instance
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Record an allocation
    pub fn record_allocation(&self, size: usize) {
        self.total_allocations.fetch_add(1, Ordering::Relaxed);
        self.active_allocations.fetch_add(1, Ordering::Relaxed);
        self.total_bytes_allocated
            .fetch_add(size as u64, Ordering::Relaxed);
        
        let current = self
            .current_memory_usage
            .fetch_add(size as u64, Ordering::Relaxed)
            + size as u64;
        let peak = self.peak_memory_usage.load(Ordering::Relaxed);
        if current > peak {
            self.peak_memory_usage.store(current, Ordering::Relaxed);
        }
        
        if size > 1_048_576 {
            // 1MB
            self.large_allocation_count.fetch_add(1, Ordering::Relaxed);
        } else if size < 1024 {
            // 1KB
            self.small_allocation_count.fetch_add(1, Ordering::Relaxed);
        }
    }
    
    /// Record a deallocation
    pub fn record_deallocation(&self, size: usize) {
        self.total_deallocations.fetch_add(1, Ordering::Relaxed);
        self.active_allocations.fetch_sub(1, Ordering::Relaxed);
        self.total_bytes_deallocated
            .fetch_add(size as u64, Ordering::Relaxed);
        self.current_memory_usage
            .fetch_sub(size as u64, Ordering::Relaxed);
    }
    
    /// Get memory efficiency ratio (deallocations / allocations)
    #[must_use]
    pub fn efficiency_ratio(&self) -> f64 {
        let allocs = self.total_allocations.load(Ordering::Relaxed) as f64;
        let deallocs = self.total_deallocations.load(Ordering::Relaxed) as f64;
        if allocs > 0.0 {
            deallocs / allocs
        } else {
            0.0
        }
    }
    
    /// Check if there are potential memory leaks
    #[must_use]
    pub fn has_potential_leaks(&self) -> bool {
        let active = self.active_allocations.load(Ordering::Relaxed);
        let total = self.total_allocations.load(Ordering::Relaxed);
        
        // If more than 10% of allocations are still active, potential leak
        active > 0 && (active as f64 / total as f64) > 0.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_stats_creation() {
        let stats = MemoryStats::new();
        assert_eq!(stats.total_allocations.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_record_allocation() {
        let stats = MemoryStats::new();
        stats.record_allocation(1024);
        assert_eq!(stats.total_allocations.load(Ordering::Relaxed), 1);
        assert_eq!(stats.total_bytes_allocated.load(Ordering::Relaxed), 1024);
    }

    #[test]
    fn test_record_deallocation() {
        let stats = MemoryStats::new();
        stats.record_allocation(1024);
        stats.record_deallocation(1024);
        assert_eq!(stats.total_deallocations.load(Ordering::Relaxed), 1);
        assert_eq!(stats.active_allocations.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_efficiency_ratio() {
        let stats = MemoryStats::new();
        stats.record_allocation(1024);
        stats.record_deallocation(1024);
        assert!((stats.efficiency_ratio() - 1.0).abs() < 0.01);
    }
}
