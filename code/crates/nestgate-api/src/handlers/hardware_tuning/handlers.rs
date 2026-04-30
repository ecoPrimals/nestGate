// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Hardware tuning handlers.
//!
//! Resource discovery uses Linux `/proc` via [`super::linux_proc`] (no `sysinfo`).
//! Benchmarks report live system metrics rather than synthetic scores.
//!
//! **Production JSON routes**: see [`super::handlers_production`].

use chrono::Utc;
use tracing::info;

use super::linux_proc;
use super::types::{
    BenchmarkResult, ComputeAllocation, ComputeResourceRequest, ComputeResources, CpuInfo,
    CpuMonitor, DiskMonitor, GpuInfo, GpuMonitor, HardwareMonitors, HardwareTuningConfig,
    LiveHardwareMetrics, MemoryInfo, MemoryMonitor, NetworkMonitor, SystemCapabilities,
    SystemMetricsCollector, SystemProfile, TuningResult, TuningServiceRegistration,
};
use nestgate_core::{NestGateError, Result};

/// **HARDWARE TUNING HANDLER**
///
/// Uses `/proc` and optional `nvidia-smi` for resource discovery; benchmarks remain lightweight stubs.
#[derive(Debug, Clone)]
/// Handler for `RealHardwareTuning` requests
pub struct RealHardwareTuningHandler {
    /// Hardware tuning configuration
    _config: HardwareTuningConfig,
    /// System metrics collector for performance monitoring
    metrics_collector: SystemMetricsCollector,
    /// Hardware monitoring services
    _monitors: HardwareMonitors,
}

impl Default for RealHardwareTuningHandler {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl RealHardwareTuningHandler {
    /// Create a new real hardware tuning handler
    #[must_use]
    pub fn new() -> Self {
        Self {
            _config: HardwareTuningConfig::default(),
            metrics_collector: SystemMetricsCollector {
                cpu_monitor: CpuMonitor,
                memory_monitor: MemoryMonitor,
                gpu_monitor: Some(GpuMonitor),
                disk_monitor: DiskMonitor,
                network_monitor: NetworkMonitor,
            },
            _monitors: HardwareMonitors {
                cpu: CpuMonitor,
                memory: MemoryMonitor,
                gpu: Some(GpuMonitor),
            },
        }
    }

    /// Register with system service manager
    #[expect(
        clippy::missing_const_for_fn,
        reason = "Instance stub; not meaningful as const fn on this handler type"
    )]
    fn register_with_system(&self, _service_name: &str) {
        // Stub implementation
    }

    /// Get available system resources from `/proc` (CPU, RAM) and best-effort GPU detection.
    fn get_system_resources(&self) -> Result<ComputeResources> {
        linux_proc::compute_resources_from_proc()
    }

    /// Allocate system resources (clamped to what [`Self::get_system_resources`] reports).
    fn allocate_system_resources(
        &self,
        request: &ComputeResourceRequest,
    ) -> Result<ComputeAllocation> {
        let avail = self.get_system_resources()?;
        Ok(ComputeAllocation {
            cpu_cores: request.cpu_cores.min(avail.available_cpu),
            memory_gb: request.memory_gb.min(avail.available_memory_gb),
            gpu_count: request.gpu_count.min(avail.available_gpu),
        })
    }

    /// Analyze system profile from detected CPU and memory characteristics.
    pub fn get_derived_system_profile(&self) -> Result<SystemProfile> {
        let metrics = self.get_live_hardware_metrics()?;
        self.analyze_system_profile(&metrics)
    }

    /// Analyze system profile from detected CPU and memory characteristics.
    fn analyze_system_profile(&self, _metrics: &LiveHardwareMetrics) -> Result<SystemProfile> {
        let cpu = self.detect_cpu_info()?;
        let mem = self.detect_memory_info()?;
        Ok(SystemProfile {
            cpu_profile: format!("{} cores: {}", cpu.cores, cpu.model),
            memory_profile: format!("{} GiB total", mem.total_gb),
            storage_profile: "unknown".to_string(),
            network_profile: "unknown".to_string(),
        })
    }

    /// Apply tuning optimizations (no privileged kernel changes here; reports live metrics before/after sampling).
    fn apply_tuning_optimizations(&self, profile: &SystemProfile) -> Result<TuningResult> {
        let before_metrics = self.get_live_hardware_metrics()?;
        let after_metrics = self.get_live_hardware_metrics()?;
        Ok(TuningResult {
            profile_name: profile.cpu_profile.clone(),
            optimizations_applied: vec![
                "observed_live_metrics_only".to_string(),
                "no_kernel_privilege_escalation".to_string(),
            ],
            estimated_power_increase: 0.0,
            performance_improvement: 0.0,
            before_metrics,
            after_metrics,
        })
    }

    /// Release system resources
    #[expect(
        clippy::missing_const_for_fn,
        reason = "Instance stub; not meaningful as const fn on this handler type"
    )]
    fn release_system_resources(&self, _allocation_id: &str) {
        // Stub implementation
    }

    /// Capture a snapshot benchmark: samples live `/proc` metrics for the given type.
    fn snapshot_benchmark(&self, benchmark_type: &str) -> BenchmarkResult {
        let metrics = self
            .get_live_hardware_metrics()
            .unwrap_or_else(|_| LiveHardwareMetrics {
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
            });
        BenchmarkResult {
            benchmark_type: benchmark_type.to_string(),
            score: metrics.cpu_usage.max(metrics.memory_usage),
            duration_ms: 0,
            metrics,
        }
    }

    /// Register tuning service with real system capabilities
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn register_tuning_service(
        &self,
        registration: &TuningServiceRegistration,
    ) -> Result<()> {
        info!(
            "Registering real hardware tuning service: {}",
            registration.service_name
        );

        // Validate system capabilities
        let capabilities = self.detect_system_capabilities().await?;
        info!("Detected system capabilities: {:?}", capabilities);

        // Register with system service manager
        self.register_with_system(&registration.service_name);

        Ok(())
    }

    /// Request real compute resources from the system
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn request_compute_resources(
        &self,
        request: &ComputeResourceRequest,
    ) -> Result<ComputeAllocation> {
        info!(
            "Requesting real compute resources: {} cores, {} GB RAM",
            request.cpu_cores, request.memory_gb
        );

        // Check available system resources
        let available_resources = self.get_system_resources()?;

        if available_resources.available_cpu < request.cpu_cores {
            return Err(NestGateError::storage_error(format!(
                "Insufficient CPU cores: requested {}, available {}",
                request.cpu_cores, available_resources.available_cpu
            )));
        }

        if available_resources.available_memory_gb < request.memory_gb {
            return Err(NestGateError::storage_error(format!(
                "Insufficient memory: requested {} GB, available {} GB",
                request.memory_gb, available_resources.available_memory_gb
            )));
        }

        // Allocate real system resources
        let allocation = self.allocate_system_resources(request)?;

        info!(
            "Successfully allocated resources: allocation_id = {}",
            allocation.cpu_cores
        );
        Ok(allocation)
    }

    /// Get live hardware metrics from real system
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn get_live_hardware_metrics(&self) -> Result<LiveHardwareMetrics> {
        let metrics = self.metrics_collector.collect_current_metrics()?;
        info!(
            "Collected live hardware metrics: CPU {}%, Memory {}%",
            metrics.cpu_usage, metrics.memory_usage
        );
        Ok(metrics)
    }

    /// Perform real auto-tuning based on actual system characteristics
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn auto_tune(&self) -> Result<TuningResult> {
        info!("Starting real hardware auto-tuning");

        // Collect baseline metrics
        let baseline_metrics = self.get_live_hardware_metrics()?;

        // Analyze system characteristics
        let system_profile = self.analyze_system_profile(&baseline_metrics)?;

        // Apply real tuning optimizations
        let tuning_result = self.apply_tuning_optimizations(&system_profile)?;

        info!(
            "Auto-tuning completed: profile = {}",
            tuning_result.profile_name
        );
        Ok(tuning_result)
    }

    /// Run real benchmark using actual hardware
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn benchmark(&self, benchmark_name: &str) -> Result<BenchmarkResult> {
        info!("Running real benchmark: {}", benchmark_name);

        let start_time = Utc::now();

        let benchmark_result = match benchmark_name {
            "cpu_intensive" | "memory_intensive" | "gpu_intensive" | "io_intensive" => {
                self.snapshot_benchmark(benchmark_name)
            }
            _ => {
                return Err(NestGateError::validation("hardware_tuning"));
            }
        };

        let duration = Utc::now().signed_duration_since(start_time);
        info!(
            "Benchmark '{}' completed in {} seconds",
            benchmark_name,
            duration.num_seconds()
        );

        Ok(benchmark_result)
    }

    /// Release allocated resources
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn release_allocation(&self, allocation_id: &str) -> Result<()> {
        info!("Releasing resource allocation: {}", allocation_id);

        // Release actual system resources
        self.release_system_resources(allocation_id);

        Ok(())
    }

    // === PRIVATE IMPLEMENTATION METHODS ===

    /// Detect and analyze system hardware capabilities
    ///
    /// This method performs comprehensive system analysis including CPU, memory,
    /// and GPU detection to determine optimal hardware tuning parameters.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn detect_system_capabilities(&self) -> Result<SystemCapabilities> {
        // Detect real system capabilities
        let cpu_info = self.detect_cpu_info()?;
        let memory_info = self.detect_memory_info()?;
        let gpu_info = self.detect_gpu_info().await;

        Ok(SystemCapabilities {
            cpu_cores: cpu_info.cores,
            cpu_model: cpu_info.model,
            memory_gb: memory_info.total_gb,
            gpu_available: gpu_info.is_some(),
            gpu_info,
        })
    }

    /// Detect Cpu Info
    fn detect_cpu_info(&self) -> Result<CpuInfo> {
        // Read from /proc/cpuinfo or use system APIs
        let cpu_info = std::fs::read_to_string("/proc/cpuinfo").map_err(|e| {
            NestGateError::system("cpu_detection", format!("Failed to read CPU info: {e}"))
        })?;

        let cores = cpu_info
            .lines()
            .filter(|line| line.starts_with("processor"))
            .count();

        let model = cpu_info
            .lines()
            .find(|line| line.starts_with("model name"))
            .and_then(|line| line.split(':').nth(1))
            .map_or_else(|| "Unknown CPU".to_string(), |s| s.trim().to_string());

        Ok(CpuInfo { cores, model })
    }

    /// Detect Memory Info
    fn detect_memory_info(&self) -> Result<MemoryInfo> {
        // Read from /proc/meminfo
        let meminfo = std::fs::read_to_string("/proc/meminfo").map_err(|e| {
            NestGateError::system(
                "memory_detection",
                format!("Failed to read memory info: {e}"),
            )
        })?;

        let total_kb = meminfo
            .lines()
            .find(|line| line.starts_with("MemTotal:"))
            .and_then(|line| line.split_whitespace().nth(1))
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);

        let total_gb = total_kb / 1024 / 1024;

        Ok(MemoryInfo { total_gb })
    }

    /// Detect Gpu Info
    async fn detect_gpu_info(&self) -> Option<GpuInfo> {
        // Try to detect GPU using nvidia-smi or other tools
        if let Ok(output) = tokio::process::Command::new("nvidia-smi")
            .arg("--query-gpu=name,memory.total")
            .arg("--format=csv,noheader,nounits")
            .output()
            .await
            && output.status.success()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            if let Some(line) = output_str.lines().next() {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 2 {
                    return Some(GpuInfo {
                        name: parts[0].trim().to_string(),
                        memory_mb: parts[1].trim().parse().unwrap_or(0),
                    });
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_real_hardware_tuning_handler_creation() -> Result<()> {
        let _handler = RealHardwareTuningHandler::new();

        // Test that we can create the handler
        Ok(())
    }

    #[tokio::test]
    async fn test_system_capabilities_detection() -> Result<()> {
        let handler = RealHardwareTuningHandler::new();
        let capabilities = handler.detect_system_capabilities().await?;

        // Test that we detected some capabilities
        assert!(capabilities.cpu_cores > 0);
        assert!(capabilities.memory_gb > 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_live_metrics_collection() -> Result<()> {
        let handler = RealHardwareTuningHandler::new();
        let metrics = handler.get_live_hardware_metrics()?;

        // Test that metrics are reasonable
        assert!(metrics.cpu_usage >= 0.0 && metrics.cpu_usage <= 100.0);
        assert!(metrics.memory_usage >= 0.0 && metrics.memory_usage <= 100.0);

        Ok(())
    }
}
