//! CPU performance configuration module
//! Provides unified CPU performance tuning and monitoring settings.

use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::{NestGateError, Result};

// ==================== CPU PERFORMANCE CONFIGURATION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CpuPerformanceConfig {
    /// CPU affinity settings
    pub affinity: CpuAffinityConfig,

    /// Thread pool configuration
    pub thread_pools: ThreadPoolConfig,

    /// CPU scheduling
    pub scheduling: CpuSchedulingConfig,

    /// SIMD optimization
    pub simd: SimdConfig,

    /// CPU monitoring
    pub monitoring: CpuMonitoringConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuAffinityConfig {
    /// Enable CPU affinity
    pub enabled: bool,

    /// Preferred CPU cores
    pub preferred_cores: Vec<usize>,

    /// Isolation strategy
    pub isolation: IsolationStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IsolationStrategy {
    None,
    Soft,
    Hard,
    Adaptive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadPoolConfig {
    /// Core thread pool size
    pub core_size: usize,

    /// Maximum thread pool size
    pub max_size: usize,

    /// Thread keep-alive time
    pub keep_alive: Duration,

    /// Queue size
    pub queue_size: usize,

    /// Thread naming pattern
    pub thread_name_pattern: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuSchedulingConfig {
    /// Scheduling policy
    pub policy: SchedulingPolicy,

    /// Process priority
    pub priority: i32,

    /// Nice value
    pub nice: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SchedulingPolicy {
    Normal,
    Fifo,
    RoundRobin,
    Batch,
    Idle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimdConfig {
    /// Enable SIMD optimizations
    pub enabled: bool,

    /// SIMD instruction sets
    pub instruction_sets: Vec<SimdInstructionSet>,

    /// Auto-vectorization
    pub auto_vectorization: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimdInstructionSet {
    Sse,
    Sse2,
    Sse3,
    Sse4_1,
    Sse4_2,
    Avx,
    Avx2,
    Avx512,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuMonitoringConfig {
    /// Enable CPU monitoring
    pub enabled: bool,

    /// Monitoring interval
    pub interval: Duration,

    /// CPU usage threshold for alerts
    pub usage_threshold: f64,
}

// ==================== DEFAULT IMPLEMENTATIONS ====================

impl Default for CpuAffinityConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            preferred_cores: Vec::new(),
            isolation: IsolationStrategy::None,
        }
    }
}

impl Default for ThreadPoolConfig {
    fn default() -> Self {
        Self {
            core_size: num_cpus::get(),
            max_size: num_cpus::get() * 2,
            keep_alive: Duration::from_secs(60),
            queue_size: 1000,
            thread_name_pattern: "nestgate-worker-{}".to_string(),
        }
    }
}

impl Default for CpuSchedulingConfig {
    fn default() -> Self {
        Self {
            policy: SchedulingPolicy::Normal,
            priority: 0,
            nice: None,
        }
    }
}

impl Default for SimdConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            instruction_sets: vec![SimdInstructionSet::Sse2, SimdInstructionSet::Avx],
            auto_vectorization: true,
        }
    }
}

impl Default for CpuMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(30),
            usage_threshold: 0.8,
        }
    }
}

// ==================== VALIDATION METHODS ====================

impl CpuPerformanceConfig {
    /// Validate CPU performance configuration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn validate(&self) -> Result<()> {
        // Validate thread pool configuration
        if self.thread_pools.core_size == 0 {
            return Err(NestGateError::configuration_error(
                "cpu.thread_pools.core_size",
                "Core thread pool size cannot be zero",
            ));
        }

        if self.thread_pools.max_size < self.thread_pools.core_size {
            return Err(NestGateError::configuration_error(
                "cpu.thread_pools.max_size",
                "Maximum thread pool size cannot be less than core size",
            ));
        }

        // Validate CPU monitoring
        if self.monitoring.usage_threshold < 0.0 || self.monitoring.usage_threshold > 1.0 {
            return Err(NestGateError::configuration_error(
                "cpu.monitoring.usage_threshold",
                "CPU usage threshold must be between 0.0 and 1.0",
            ));
        }

        Ok(())
    }
}
