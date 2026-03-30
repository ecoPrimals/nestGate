// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! CPU performance configuration module
//! Provides unified CPU performance tuning and monitoring settings.

use serde::{Deserialize, Serialize};
use std::time::Duration;

use nestgate_types::error::{NestGateError, Result};

// ==================== CPU PERFORMANCE CONFIGURATION ====================

/// CPU performance configuration for optimizing CPU usage and parallelism.
///
/// Controls CPU affinity, thread pooling, scheduling, SIMD optimizations,
/// and CPU monitoring for maximum performance.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for `CpuPerformance`
pub struct CpuPerformanceConfig {
    /// CPU affinity settings for core pinning.
    pub affinity: CpuAffinityConfig,

    /// Thread pool configuration for parallel execution.
    pub thread_pools: ThreadPoolConfig,

    /// CPU scheduling policy and priority.
    pub scheduling: CpuSchedulingConfig,

    /// SIMD optimization settings for vectorization.
    pub simd: SimdConfig,

    /// CPU monitoring for usage tracking.
    pub monitoring: CpuMonitoringConfig,
}

/// CPU affinity configuration for binding threads to specific cores.
///
/// Enables CPU core pinning to reduce context switching and improve cache locality.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `CpuAffinity`
pub struct CpuAffinityConfig {
    /// Whether CPU affinity is enabled.
    pub enabled: bool,

    /// List of preferred CPU core indices.
    pub preferred_cores: Vec<usize>,

    /// Isolation strategy for core assignment.
    pub isolation: IsolationStrategy,
}

/// CPU core isolation strategy.
///
/// Determines how threads are distributed across CPU cores.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Isolationstrategy
pub enum IsolationStrategy {
    /// No isolation - OS decides scheduling.
    None,
    /// Soft isolation - prefer specific cores but allow migration.
    Soft,
    /// Hard isolation - strictly bind threads to cores.
    Hard,
    /// Adaptive isolation - adjust based on load.
    Adaptive,
}

/// Thread pool configuration for managing worker threads.
///
/// Controls thread pool sizing, keep-alive time, queue size, and thread naming.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `ThreadPool`
pub struct ThreadPoolConfig {
    /// Core thread pool size (default: CPU count).
    pub core_size: usize,

    /// Maximum thread pool size (default: CPU count * 2).
    pub max_size: usize,

    /// Thread keep-alive duration before termination.
    pub keep_alive: Duration,

    /// Work queue size for pending tasks.
    pub queue_size: usize,

    /// Thread naming pattern (e.g., "nestgate-worker-{}").
    pub thread_name_pattern: String,
}

/// CPU scheduling configuration for process priority and scheduling policy.
///
/// Controls how the OS schedules CPU time for the application.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `CpuScheduling`
pub struct CpuSchedulingConfig {
    /// Scheduling policy to use.
    pub policy: SchedulingPolicy,

    /// Process priority (-20 to 19, lower = higher priority).
    pub priority: i32,

    /// Nice value for priority adjustment (Unix).
    pub nice: Option<i32>,
}

/// CPU scheduling policy.
///
/// Determines how the OS scheduler allocates CPU time.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Schedulingpolicy
pub enum SchedulingPolicy {
    /// Normal time-sharing scheduling.
    Normal,
    /// First-in-first-out real-time scheduling.
    Fifo,
    /// Round-robin real-time scheduling.
    RoundRobin,
    /// Batch scheduling for non-interactive processes.
    Batch,
    /// Idle scheduling - only runs when system is idle.
    Idle,
}

/// SIMD (Single Instruction Multiple Data) configuration for vectorization.
///
/// Enables CPU vector instructions for parallel data processing.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Simd
pub struct SimdConfig {
    /// Whether SIMD optimizations are enabled.
    pub enabled: bool,

    /// List of supported SIMD instruction sets to use.
    pub instruction_sets: Vec<SimdInstructionSet>,

    /// Whether to enable compiler auto-vectorization.
    pub auto_vectorization: bool,
}

/// SIMD instruction set extensions.
///
/// CPU-specific vector instruction sets for parallel operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Simdinstructionset
pub enum SimdInstructionSet {
    /// Streaming SIMD Extensions.
    Sse,
    /// SSE2 (Pentium 4+).
    Sse2,
    /// SSE3 (Prescott+).
    Sse3,
    /// SSE4.1 (Penryn+).
    Sse4_1,
    /// SSE4.2 (Nehalem+).
    Sse4_2,
    /// Advanced Vector Extensions (Sandy Bridge+).
    Avx,
    /// AVX2 (Haswell+).
    Avx2,
    /// AVX-512 (Skylake-X+).
    Avx512,
}

/// CPU monitoring configuration for tracking CPU usage.
///
/// Enables alerts and metrics for CPU consumption.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `CpuMonitoring`
pub struct CpuMonitoringConfig {
    /// Whether CPU monitoring is enabled.
    pub enabled: bool,

    /// Monitoring interval between samples.
    pub interval: Duration,

    /// CPU usage threshold for alerts (0.0-1.0, e.g., 0.8 = 80%).
    pub usage_threshold: f64,
}

// ==================== DEFAULT IMPLEMENTATIONS ====================

impl Default for CpuAffinityConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: false,
            preferred_cores: Vec::new(),
            isolation: IsolationStrategy::None,
        }
    }
}

impl Default for ThreadPoolConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            core_size: std::thread::available_parallelism().map_or(4, std::num::NonZero::get),
            max_size: std::thread::available_parallelism().map_or(4, std::num::NonZero::get) * 2,
            keep_alive: Duration::from_secs(60),
            queue_size: 1000,
            thread_name_pattern: "nestgate-worker-{}".to_string(),
        }
    }
}

impl Default for CpuSchedulingConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            policy: SchedulingPolicy::Normal,
            priority: 0,
            nice: None,
        }
    }
}

impl Default for SimdConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            instruction_sets: vec![SimdInstructionSet::Sse2, SimdInstructionSet::Avx],
            auto_vectorization: true,
        }
    }
}

impl Default for CpuMonitoringConfig {
    /// Returns the default instance
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

#[cfg(test)]
mod tests {
    use super::*;

    fn serde_roundtrip<T>(v: &T)
    where
        T: serde::Serialize + serde::de::DeserializeOwned,
    {
        let s = serde_json::to_string(v).expect("to_string");
        let _: T = serde_json::from_str(&s).expect("from_str");
    }

    #[test]
    fn cpu_performance_default_validate_serde() {
        let c = CpuPerformanceConfig::default();
        c.validate().expect("validate");
        serde_roundtrip(&c);
    }

    #[test]
    fn isolation_and_scheduling_variants() {
        for i in [
            IsolationStrategy::None,
            IsolationStrategy::Soft,
            IsolationStrategy::Hard,
            IsolationStrategy::Adaptive,
        ] {
            serde_roundtrip(&i);
        }
        for p in [
            SchedulingPolicy::Normal,
            SchedulingPolicy::Fifo,
            SchedulingPolicy::RoundRobin,
            SchedulingPolicy::Batch,
            SchedulingPolicy::Idle,
        ] {
            serde_roundtrip(&p);
        }
    }
}
