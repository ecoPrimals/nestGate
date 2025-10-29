// **CANONICAL PERFORMANCE CONFIGURATION - MODULARIZED**
//! Module definitions and exports.
// This module consolidates ALL performance configuration variants across the NestGate ecosystem
//! into a focused, modular structure for maintainability.
//! Module definitions and exports.
// **CONSOLIDATES**:
//! - `UnifiedPerformanceConfig`
//! - `UnifiedPerformanceTestConfig`  
//! - Multiple performance/monitoring configs across all crates
//! - 18+ other performance-related configuration structures

// ==================== PERFORMANCE CONFIGURATION MODULES ====================

// Core performance configuration structures
pub mod core;
// CPU optimization settings
pub mod cpu;
// Memory optimization settings
pub mod memory;
// I/O optimization settings
pub mod io;
// Network performance settings
pub mod network;
// Caching performance settings
pub mod caching;
// Concurrency and threading configuration
pub mod concurrency;
// Performance monitoring and metrics
pub mod monitoring;
// Optimization profiles
pub mod profiles;
// Environment-specific settings
pub mod environment;
// ==================== RE-EXPORTS ====================

pub use caching::{CacheOptimizationConfig, CachePerformanceConfig, CacheWarmingConfig};
pub use concurrency::{ConcurrencyConfig, ConcurrencyModel, LoadBalancingStrategy};
pub use core::CanonicalPerformanceConfig;
pub use cpu::{
    CpuAffinityConfig, CpuPerformanceConfig, CpuSchedulingConfig, SimdConfig, ThreadPoolConfig,
};
pub use environment::{PerformanceDebugConfig, PerformanceEnvironmentConfig};
pub use io::{IoBufferingConfig, IoOptimizationConfig, IoPerformanceConfig};
pub use memory::{GarbageCollectionConfig, MemoryPerformanceConfig, MemoryPoolConfig};
pub use monitoring::{AlertingConfig, MetricsConfig, PerformanceMonitoringConfig, ProfilingConfig};
pub use network::{NetworkBufferingConfig, NetworkOptimizationConfig, NetworkPerformanceConfig};
pub use profiles::{OptimizationProfile, OptimizationProfiles};

// ==================== BACKWARD COMPATIBILITY ALIASES ====================

// Backward compatibility alias for existing PerformanceConfig usage
pub type PerformanceConfig = CanonicalPerformanceConfig;
// Backward compatibility alias for UnifiedPerformanceConfig
pub type UnifiedPerformanceConfig = CanonicalPerformanceConfig;
// Backward compatibility alias for UnifiedPerformanceTestConfig
pub type UnifiedPerformanceTestConfig = CanonicalPerformanceConfig;
