// **CANONICAL MEMORY CONFIGURATION**
//! Memory configuration functionality and utilities.
//! This module provides the unified memory configuration for the NestGate system,
//! consolidating all memory-related configuration patterns.

use serde::{Deserialize, Serialize};
use std::time::Duration;

// ==================== MEMORY CONFIGURATION ====================

/// Canonical memory configuration - THE single source of truth
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Memory
pub struct MemoryConfig {
    /// Enable memory management
    pub enabled: bool,
    /// Memory pool configuration
    pub pool_config: MemoryPoolConfig,
    /// Allocation strategy
    pub allocation_strategy: AllocationStrategy,
    /// Memory limits
    pub limits: MemoryLimits,
    /// Garbage collection settings
    pub gc_config: GarbageCollectionConfig,
    /// Memory monitoring
    pub monitoring: MemoryMonitoringConfig,
}

impl Default for MemoryConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            pool_config: MemoryPoolConfig::default(),
            allocation_strategy: AllocationStrategy::default(),
            limits: MemoryLimits::default(),
            gc_config: GarbageCollectionConfig::default(),
            monitoring: MemoryMonitoringConfig::default(),
        }
    }
}

// ==================== MEMORY POOL CONFIGURATION ====================

/// Memory pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for MemoryPool
pub struct MemoryPoolConfig {
    /// Enable memory pooling
    pub enabled: bool,
    /// Initial pool size in bytes
    pub initial_size: u64,
    /// Maximum pool size in bytes
    pub max_size: u64,
    /// Pool growth factor
    pub growth_factor: f64,
    /// Pool shrink threshold
    pub shrink_threshold: f64,
}

impl Default for MemoryPoolConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            initial_size: 64 * 1024 * 1024, // 64MB
            max_size: 512 * 1024 * 1024,    // 512MB
            growth_factor: 1.5,
            shrink_threshold: 0.25,
        }
    }
}

// ==================== ALLOCATION STRATEGY ====================

/// Memory allocation strategy
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Allocationstrategy
pub enum AllocationStrategy {
    /// First-fit allocation strategy
    #[default]
    /// Firstfit
    FirstFit,
    /// Best-fit allocation strategy
    BestFit,
    /// Worst-fit allocation strategy
    WorstFit,
    /// Buddy system allocation
    BuddySystem,
    /// Slab allocator
    SlabAllocator,
}

// ==================== MEMORY LIMITS ====================

/// Memory limits configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Memorylimits
pub struct MemoryLimits {
    /// Maximum memory usage in bytes
    pub max_memory: u64,
    /// Maximum heap size in bytes
    pub max_heap: u64,
    /// Maximum stack size in bytes
    pub max_stack: u64,
    /// Memory warning threshold (percentage)
    pub warning_threshold: f64,
    /// Memory critical threshold (percentage)
    pub critical_threshold: f64,
}

impl Default for MemoryLimits {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            max_memory: 2 * 1024 * 1024 * 1024, // 2GB
            max_heap: 1024 * 1024 * 1024,       // 1GB
            max_stack: 8 * 1024 * 1024,         // 8MB
            warning_threshold: 0.8,
            critical_threshold: 0.95,
        }
    }
}

// ==================== GARBAGE COLLECTION ====================

/// Garbage collection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for GarbageCollection
pub struct GarbageCollectionConfig {
    /// Enable automatic garbage collection
    pub enabled: bool,
    /// GC trigger threshold (percentage of max memory)
    pub trigger_threshold: f64,
    /// GC interval
    pub interval: Duration,
    /// Aggressive GC on low memory
    pub aggressive_on_low_memory: bool,
}

impl Default for GarbageCollectionConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            trigger_threshold: 0.75,
            interval: Duration::from_secs(60),
            aggressive_on_low_memory: true,
        }
    }
}

// ==================== MEMORY MONITORING ====================

/// Memory monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for MemoryMonitoring
pub struct MemoryMonitoringConfig {
    /// Enable memory monitoring
    pub enabled: bool,
    /// Monitoring interval
    pub interval: Duration,
    /// Track allocation patterns
    pub track_allocations: bool,
    /// Track memory leaks
    pub track_leaks: bool,
}

impl Default for MemoryMonitoringConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(30),
            track_allocations: false,
            track_leaks: true,
        }
    }
}

// ==================== BACKWARD COMPATIBILITY ALIASES ====================

/// Backward compatibility alias for UnifiedMemoryConfig
pub type UnifiedMemoryConfig = MemoryConfig;
