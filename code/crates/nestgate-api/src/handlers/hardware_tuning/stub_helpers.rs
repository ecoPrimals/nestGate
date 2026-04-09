// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![warn(missing_docs)]

//! Best-effort hardware snapshots from Linux procfs and sysfs.
//!
//! On non-Linux hosts, or when `/proc` is unreadable, these helpers fall back to safe defaults
//! (zeros or minimal resource counts) without panicking. The `create_*` names are kept for
//! compatibility with older dev-stub call sites.

use super::linux_proc;
use super::types::{
    BenchmarkResult, ComputeAllocation, ComputeResources, CpuInfo, LiveHardwareMetrics,
    SystemProfile, TuningResult,
};

/// Live hardware metrics from `/proc` (CPU, memory, disk, network) when available; otherwise zeros.
#[must_use]
pub fn create_zero_hardware_metrics() -> LiveHardwareMetrics {
    linux_proc::live_hardware_metrics_best_effort()
}

/// Compute resources discovered via [`linux_proc::compute_resources_from_proc`], with fallbacks.
#[must_use]
pub fn create_stub_compute_resources() -> ComputeResources {
    linux_proc::compute_resources_from_proc().unwrap_or(ComputeResources {
        available_cpu: 1,
        available_memory_gb: 1,
        available_gpu: 0,
    })
}

/// Allocation matching the current host snapshot (full available resources).
#[must_use]
pub fn create_stub_compute_allocation() -> ComputeAllocation {
    let r = create_stub_compute_resources();
    ComputeAllocation {
        cpu_cores: r.available_cpu,
        memory_gb: r.available_memory_gb,
        gpu_count: r.available_gpu,
    }
}

/// Derived profile strings from CPU model, memory, and `/sys/block` / `/sys/class/net`.
#[must_use]
pub fn create_stub_system_profile() -> SystemProfile {
    let cpu = create_stub_cpu_info();
    let mem_gb = linux_proc::mem_total_gib().unwrap_or(1);
    SystemProfile {
        cpu_profile: format!("{} cores: {}", cpu.cores, cpu.model),
        memory_profile: format!("{mem_gb} GiB total"),
        storage_profile: linux_proc::storage_profile_from_sysfs(),
        network_profile: linux_proc::network_profile_from_sysfs(),
    }
}

/// Observational tuning report using live metrics before/after sampling (no privileged changes).
#[must_use]
pub fn create_stub_tuning_result() -> TuningResult {
    let before_metrics = linux_proc::live_hardware_metrics_best_effort();
    let after_metrics = linux_proc::live_hardware_metrics_best_effort();
    let profile = create_stub_system_profile();
    let profile_name = profile.cpu_profile;
    TuningResult {
        profile_name,
        optimizations_applied: vec![
            "observed_live_metrics_only".to_string(),
            "no_kernel_privilege_escalation".to_string(),
        ],
        estimated_power_increase: 0.0,
        performance_improvement: 0.0,
        before_metrics,
        after_metrics,
    }
}

/// Benchmark envelope carrying a live metrics snapshot from procfs.
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

/// CPU model and core count from `/proc/cpuinfo` (Linux) or host parallelism (other Unix).
#[must_use]
pub fn create_stub_cpu_info() -> CpuInfo {
    let cores = linux_proc::logical_cpu_count()
        .map(|n| usize::try_from(n.max(1)).unwrap_or(1))
        .unwrap_or(1);
    let model = linux_proc::cpu_model_best_effort().unwrap_or_else(|_| "unknown".to_string());
    CpuInfo { cores, model }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;

    #[test]
    fn zero_metrics_timestamp_is_recent() {
        let metrics = create_zero_hardware_metrics();
        let now = Utc::now();
        assert!(metrics.timestamp <= now);
        assert!(metrics.timestamp > now - chrono::Duration::seconds(2));
    }

    #[test]
    fn zero_metrics_percentages_in_range() {
        let m = create_zero_hardware_metrics();
        assert!(m.cpu_usage >= 0.0 && m.cpu_usage <= 100.0);
        assert!(m.memory_usage >= 0.0 && m.memory_usage <= 100.0);
        assert!(m.gpu_usage >= 0.0 && m.gpu_usage <= 100.0);
        assert!(m.disk_usage >= 0.0 && m.disk_usage <= 100.0);
    }

    #[test]
    fn stub_compute_resources_nonzero_cpu() {
        let resources = create_stub_compute_resources();
        assert!(resources.available_cpu >= 1);
        assert!(resources.available_memory_gb >= 1);
    }

    #[test]
    fn stub_allocation_matches_resources() {
        let r = create_stub_compute_resources();
        let allocation = create_stub_compute_allocation();
        assert_eq!(allocation.cpu_cores, r.available_cpu);
        assert_eq!(allocation.memory_gb, r.available_memory_gb);
        assert_eq!(allocation.gpu_count, r.available_gpu);
    }

    #[test]
    fn stub_system_profile_strings_nonempty() {
        let profile = create_stub_system_profile();
        assert!(!profile.cpu_profile.is_empty());
        assert!(!profile.memory_profile.is_empty());
        assert!(!profile.storage_profile.is_empty());
        assert!(!profile.network_profile.is_empty());
    }

    #[test]
    fn stub_tuning_result_uses_observational_profile() {
        let result = create_stub_tuning_result();
        assert!(!result.profile_name.is_empty());
        assert_eq!(result.optimizations_applied.len(), 2);
    }

    #[test]
    fn stub_benchmark_carries_type() {
        let result = create_stub_benchmark_result("cpu_test", 85.0, 5000);
        assert_eq!(result.benchmark_type, "cpu_test");
        assert_eq!(result.score, 85.0);
    }

    #[test]
    fn stub_cpu_info_matches_probe() {
        let cpu = create_stub_cpu_info();
        assert!(cpu.cores >= 1);
        assert!(!cpu.model.is_empty());
    }

    #[test]
    fn stub_functions_do_not_panic() {
        let _ = create_zero_hardware_metrics();
        let _ = create_stub_compute_resources();
        let _ = create_stub_compute_allocation();
        let _ = create_stub_system_profile();
        let _ = create_stub_tuning_result();
        let _ = create_stub_benchmark_result("test", 0.0, 0);
        let _ = create_stub_cpu_info();
    }
}
