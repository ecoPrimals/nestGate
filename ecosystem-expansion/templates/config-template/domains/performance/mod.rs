//! **CANONICAL PERFORMANCE CONFIGURATION - MODULARIZED**
//!
//! This module consolidates ALL performance configuration variants across the NestGate ecosystem
//! into a focused, modular structure for maintainability.
//!
//! **CONSOLIDATES**:
//! - UnifiedPerformanceConfig
//! - UnifiedPerformanceTestConfig  
//! - Multiple performance/monitoring configs across all crates
//! - 18+ other performance-related configuration structures

// ==================== PERFORMANCE CONFIGURATION MODULES ====================

/// Core performance configuration structures
pub mod core;

/// CPU optimization settings
pub mod cpu;

/// Memory optimization settings
pub mod memory;

/// I/O optimization settings
pub mod io;

/// Network performance settings
pub mod network;

/// Caching performance settings
pub mod caching;

/// Concurrency and threading configuration
pub mod concurrency;

/// Performance monitoring and metrics
pub mod monitoring;

/// Optimization profiles
pub mod profiles;

/// Environment-specific settings
pub mod environment;

// ==================== RE-EXPORTS ====================

pub use core::CanonicalPerformanceConfig;
pub use cpu::{CpuPerformanceConfig, CpuAffinityConfig, ThreadPoolConfig, CpuSchedulingConfig, SimdConfig};
pub use memory::{MemoryPerformanceConfig, MemoryPoolConfig, GarbageCollectionConfig};
pub use io::{IoPerformanceConfig, IoOptimizationConfig, IoBufferingConfig};
pub use network::{NetworkPerformanceConfig, NetworkOptimizationConfig, NetworkBufferingConfig};
pub use caching::{CachePerformanceConfig, CacheOptimizationConfig, CacheWarmingConfig};
pub use concurrency::{ConcurrencyConfig, ConcurrencyModel, LoadBalancingStrategy};
pub use monitoring::{PerformanceMonitoringConfig, MetricsConfig, ProfilingConfig, AlertingConfig};
pub use profiles::{OptimizationProfiles, OptimizationProfile};
pub use environment::{PerformanceEnvironmentConfig, PerformanceDebugConfig};

// ==================== BACKWARD COMPATIBILITY ALIASES ====================

/// Backward compatibility alias for existing PerformanceConfig usage
pub type PerformanceConfig = CanonicalPerformanceConfig;

/// Backward compatibility alias for UnifiedPerformanceConfig
pub type UnifiedPerformanceConfig = CanonicalPerformanceConfig;

/// Backward compatibility alias for UnifiedPerformanceTestConfig
pub type UnifiedPerformanceTestConfig = CanonicalPerformanceConfig; 