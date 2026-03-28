// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **HARDWARE TUNING STUB HELPERS**
//!
//! Helper functions for creating stub/mock data for hardware tuning operations.
//! These are used in development mode when real system integration is not available.

use chrono::Utc;

use crate::handlers::hardware_tuning::types::{
    BenchmarkResult, ComputeAllocation, ComputeResources, CpuInfo, LiveHardwareMetrics,
    SystemProfile, TuningResult,
};

/// Create zero-initialized hardware metrics (used in stub implementations)
#[must_use]
pub fn create_zero_hardware_metrics() -> LiveHardwareMetrics {
    LiveHardwareMetrics {
        timestamp: Utc::now(),
        cpu_usage: 0.0,
        memory_usage: 0.0,
        gpu_usage: 0.0,
        disk_io: 0.0,
        disk_usage: 0.0,
        network_io: 0.0,
        network_usage: 0.0,
        temperature: 0.0,
        power_consumption: 0.0,
    }
}

/// Create stub compute resources (hardcoded system resources)
#[must_use]
pub const fn create_stub_compute_resources() -> ComputeResources {
    ComputeResources {
        // ecoBin v3.0: dev stubs only; prefer `/proc` (see `hardware_tuning::linux_proc`), not sysinfo.
        available_cpu: 16,
        available_memory_gb: 64,
        available_gpu: 2, // HARDCODED - Future: Implement GPU detection
    }
}

/// Create stub compute allocation
#[must_use]
pub const fn create_stub_compute_allocation() -> ComputeAllocation {
    ComputeAllocation {
        cpu_cores: 8,
        memory_gb: 16,
        gpu_count: 1,
    }
}

/// Create stub system profile
#[must_use]
pub fn create_stub_system_profile() -> SystemProfile {
    SystemProfile {
        cpu_profile: "high_performance".to_string(),
        memory_profile: "balanced".to_string(),
        storage_profile: "fast_ssd".to_string(),
        network_profile: "gigabit".to_string(),
    }
}

/// Create stub tuning result
#[must_use]
pub fn create_stub_tuning_result() -> TuningResult {
    TuningResult {
        profile_name: "test_profile".to_string(), // HARDCODED
        optimizations_applied: vec!["cpu_governor_performance".to_string()], // HARDCODED
        estimated_power_increase: 5.0,            // HARDCODED
        performance_improvement: 15.0,            // HARDCODED
        before_metrics: create_zero_hardware_metrics(),
        after_metrics: create_zero_hardware_metrics(),
    }
}

/// Create stub benchmark result
#[must_use]
pub fn create_stub_benchmark_result(
    benchmark_type: &str,
    score: f64,
    duration_ms: u64,
) -> BenchmarkResult {
    BenchmarkResult {
        benchmark_type: benchmark_type.to_string(),
        score,
        duration_ms,
        metrics: create_zero_hardware_metrics(),
    }
}

/// Create stub CPU info
#[must_use]
pub fn create_stub_cpu_info(cores: usize, model: &str) -> CpuInfo {
    CpuInfo {
        cores,
        model: model.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_zero_hardware_metrics() {
        let metrics = create_zero_hardware_metrics();
        assert_eq!(metrics.cpu_usage, 0.0);
        assert_eq!(metrics.memory_usage, 0.0);
        assert_eq!(metrics.gpu_usage, 0.0);
    }

    #[test]
    fn test_create_stub_compute_resources() {
        let resources = create_stub_compute_resources();
        assert_eq!(resources.available_cpu, 16);
        assert_eq!(resources.available_memory_gb, 64);
        assert_eq!(resources.available_gpu, 2);
    }

    #[test]
    fn test_create_stub_compute_allocation() {
        let allocation = create_stub_compute_allocation();
        assert_eq!(allocation.cpu_cores, 8);
        assert_eq!(allocation.memory_gb, 16);
        assert_eq!(allocation.gpu_count, 1);
    }

    #[test]
    fn test_create_stub_system_profile() {
        let profile = create_stub_system_profile();
        assert_eq!(profile.cpu_profile, "high_performance");
        assert_eq!(profile.memory_profile, "balanced");
    }

    #[test]
    fn test_create_stub_tuning_result() {
        let result = create_stub_tuning_result();
        assert_eq!(result.profile_name, "test_profile");
        assert_eq!(result.performance_improvement, 15.0);
    }

    #[test]
    fn test_create_stub_benchmark_result() {
        let result = create_stub_benchmark_result("cpu_test", 85.0, 5000);
        assert_eq!(result.benchmark_type, "cpu_test");
        assert_eq!(result.score, 85.0);
        assert_eq!(result.duration_ms, 5000);
    }

    #[test]
    fn test_create_stub_cpu_info() {
        let cpu = create_stub_cpu_info(16, "Intel Core i9-13900K");
        assert_eq!(cpu.cores, 16);
        assert_eq!(cpu.model, "Intel Core i9-13900K");
    }

    // === Additional Comprehensive Tests ===

    #[test]
    fn test_zero_metrics_timestamp_is_recent() {
        let metrics = create_zero_hardware_metrics();
        let now = Utc::now();
        assert!(metrics.timestamp <= now);
        assert!(metrics.timestamp > now - chrono::Duration::seconds(1));
    }

    #[test]
    fn test_zero_metrics_all_fields_zero() {
        let metrics = create_zero_hardware_metrics();
        assert_eq!(metrics.cpu_usage, 0.0);
        assert_eq!(metrics.memory_usage, 0.0);
        assert_eq!(metrics.gpu_usage, 0.0);
        assert_eq!(metrics.disk_io, 0.0);
        assert_eq!(metrics.disk_usage, 0.0);
        assert_eq!(metrics.network_io, 0.0);
        assert_eq!(metrics.network_usage, 0.0);
        assert_eq!(metrics.temperature, 0.0);
        assert_eq!(metrics.power_consumption, 0.0);
    }

    #[test]
    fn test_stub_resources_memory_calculation() {
        let resources = create_stub_compute_resources();
        assert_eq!(resources.available_memory_gb, 64);
        assert_eq!(resources.available_cpu, 16);
    }

    #[test]
    fn test_stub_resources_has_gpu_count() {
        let resources = create_stub_compute_resources();
        assert_eq!(resources.available_gpu, 2);
        assert_eq!(resources.available_cpu, 16);
    }

    #[test]
    fn test_stub_resources_compute_availability() {
        let resources = create_stub_compute_resources();
        assert_eq!(resources.available_cpu, 16);
        assert_eq!(resources.available_memory_gb, 64);
    }

    #[test]
    fn test_stub_allocation_values() {
        let allocation = create_stub_compute_allocation();
        assert_eq!(allocation.cpu_cores, 8);
        assert_eq!(allocation.memory_gb, 16);
        assert_eq!(allocation.gpu_count, 1);
    }

    #[test]
    fn test_stub_profile_all_fields() {
        let profile = create_stub_system_profile();
        assert_eq!(profile.cpu_profile, "high_performance");
        assert_eq!(profile.memory_profile, "balanced");
        assert_eq!(profile.storage_profile, "fast_ssd");
        assert_eq!(profile.network_profile, "gigabit");
    }

    #[test]
    fn test_stub_tuning_result_optimizations() {
        let result = create_stub_tuning_result();
        assert_eq!(result.optimizations_applied.len(), 1);
        assert_eq!(result.optimizations_applied[0], "cpu_governor_performance");
    }

    #[test]
    fn test_stub_tuning_result_metrics() {
        let result = create_stub_tuning_result();
        assert_eq!(result.estimated_power_increase, 5.0);
        assert_eq!(result.performance_improvement, 15.0);
    }

    #[test]
    fn test_stub_tuning_result_has_before_after() {
        let result = create_stub_tuning_result();
        assert_eq!(result.before_metrics.cpu_usage, 0.0);
        assert_eq!(result.after_metrics.cpu_usage, 0.0);
    }

    #[test]
    fn test_stub_benchmark_custom_values() {
        let result = create_stub_benchmark_result("memory_test", 92.5, 3000);
        assert_eq!(result.benchmark_type, "memory_test");
        assert_eq!(result.score, 92.5);
        assert_eq!(result.duration_ms, 3000);
    }

    #[test]
    fn test_stub_benchmark_zero_scores() {
        let result = create_stub_benchmark_result("test", 0.0, 0);
        assert_eq!(result.score, 0.0);
        assert_eq!(result.duration_ms, 0);
    }

    #[test]
    fn test_stub_benchmark_max_scores() {
        let result = create_stub_benchmark_result("max_test", 100.0, 1000);
        assert_eq!(result.score, 100.0);
        assert_eq!(result.duration_ms, 1000);
    }

    #[test]
    fn test_stub_benchmark_has_metrics() {
        let result = create_stub_benchmark_result("test", 50.0, 5000);
        assert_eq!(result.metrics.cpu_usage, 0.0);
    }

    #[test]
    fn test_stub_benchmark_duration() {
        let result = create_stub_benchmark_result("test", 75.0, 2000);
        assert_eq!(result.duration_ms, 2000);
        assert_eq!(result.score, 75.0);
    }

    #[test]
    fn test_stub_cpu_info_zero_cores() {
        let cpu = create_stub_cpu_info(0, "Virtual CPU");
        assert_eq!(cpu.cores, 0);
    }

    #[test]
    fn test_stub_cpu_info_many_cores() {
        let cpu = create_stub_cpu_info(128, "AMD EPYC 9654");
        assert_eq!(cpu.cores, 128);
        assert_eq!(cpu.model, "AMD EPYC 9654");
    }

    #[test]
    fn test_stub_cpu_info_empty_model() {
        let cpu = create_stub_cpu_info(8, "");
        assert_eq!(cpu.model, "");
    }

    #[test]
    fn test_multiple_zero_metrics_are_independent() {
        let metrics1 = create_zero_hardware_metrics();
        let metrics2 = create_zero_hardware_metrics();
        assert!(metrics1.timestamp <= metrics2.timestamp);
    }

    #[test]
    fn test_multiple_resources_are_identical() {
        let res1 = create_stub_compute_resources();
        let res2 = create_stub_compute_resources();
        assert_eq!(res1.available_cpu, res2.available_cpu);
        assert_eq!(res1.available_memory_gb, res2.available_memory_gb);
    }

    #[test]
    fn test_benchmark_different_types() {
        let cpu_bench = create_stub_benchmark_result("cpu", 85.0, 5000);
        let mem_bench = create_stub_benchmark_result("memory", 90.0, 3000);
        let gpu_bench = create_stub_benchmark_result("gpu", 95.0, 4000);

        assert_eq!(cpu_bench.benchmark_type, "cpu");
        assert_eq!(mem_bench.benchmark_type, "memory");
        assert_eq!(gpu_bench.benchmark_type, "gpu");
    }

    #[test]
    fn test_allocation_resource_ratios() {
        let allocation = create_stub_compute_allocation();
        // 8 cores to 16 GB = 1:2 ratio
        assert_eq!(allocation.cpu_cores * 2, allocation.memory_gb);
    }

    #[test]
    fn test_resources_gpu_count() {
        let resources = create_stub_compute_resources();
        assert_eq!(resources.available_gpu, 2);
    }

    #[test]
    fn test_tuning_result_power_is_positive() {
        let result = create_stub_tuning_result();
        assert!(result.estimated_power_increase > 0.0);
    }

    #[test]
    fn test_tuning_result_performance_is_positive() {
        let result = create_stub_tuning_result();
        assert!(result.performance_improvement > 0.0);
    }

    #[test]
    fn test_profile_names_are_lowercase() {
        let profile = create_stub_system_profile();
        assert_eq!(profile.cpu_profile.to_lowercase(), profile.cpu_profile);
        assert_eq!(
            profile.memory_profile.to_lowercase(),
            profile.memory_profile
        );
    }

    #[test]
    fn test_stub_functions_dont_panic() {
        // Ensure all stub functions can be called without panicking
        let _metrics = create_zero_hardware_metrics();
        let _resources = create_stub_compute_resources();
        let _allocation = create_stub_compute_allocation();
        let _profile = create_stub_system_profile();
        let _tuning = create_stub_tuning_result();
        let _benchmark = create_stub_benchmark_result("test", 0.0, 0);
        let _cpu = create_stub_cpu_info(0, "");
    }

    #[test]
    fn test_benchmark_type_preserves_case() {
        let result = create_stub_benchmark_result("CpuTest", 85.0, 5000);
        assert_eq!(result.benchmark_type, "CpuTest");
    }

    #[test]
    fn test_cpu_info_model_with_special_chars() {
        let cpu = create_stub_cpu_info(8, "AMD Ryzen™ 7 5800X3D");
        assert_eq!(cpu.model, "AMD Ryzen™ 7 5800X3D");
    }
}
