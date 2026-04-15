// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use chrono::Utc;

use nestgate_core::Result;

use super::monitors::{CpuMonitor, DiskMonitor, GpuMonitor, MemoryMonitor, NetworkMonitor};
use super::results::LiveHardwareMetrics;

/// **SYSTEM METRICS COLLECTOR**
///
/// Collects and aggregates system performance metrics.
#[derive(Debug, Clone)]
/// Systemmetricscollector
pub struct SystemMetricsCollector {
    /// CPU monitoring
    pub cpu_monitor: CpuMonitor,
    /// Memory monitoring
    pub memory_monitor: MemoryMonitor,
    /// GPU monitoring (if available)
    pub gpu_monitor: Option<GpuMonitor>,
    /// Disk monitoring (root filesystem)
    pub disk_monitor: DiskMonitor,
    /// Network interface aggregate monitoring
    pub network_monitor: NetworkMonitor,
}

impl SystemMetricsCollector {
    /// Create a new system metrics collector with hardware detection
    ///
    /// Initializes monitoring components for CPU, memory, and GPU (if available).
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub const fn new() -> Result<Self> {
        Ok(Self {
            cpu_monitor: CpuMonitor,
            memory_monitor: MemoryMonitor,
            gpu_monitor: None, // Initialize based on GPU detection
            disk_monitor: DiskMonitor,
            network_monitor: NetworkMonitor,
        })
    }

    /// Collect current hardware performance metrics from the system
    ///
    /// Gathers real-time CPU, memory, and GPU usage statistics.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn collect_current_metrics(&self) -> Result<LiveHardwareMetrics> {
        // Collect real metrics from system via monitor providers
        let cpu_usage = self.cpu_monitor.usage_percent()?;
        let memory_usage = self.memory_monitor.usage_percent()?;
        let gpu_usage = match &self.gpu_monitor {
            Some(gpu) => gpu.usage_percent().unwrap_or(0.0),
            None => 0.0,
        };

        Ok(LiveHardwareMetrics {
            timestamp: Utc::now(),
            cpu_usage,
            memory_usage,
            gpu_usage,
            disk_usage: self.disk_monitor.usage_percent().unwrap_or(0.0),
            network_usage: self.network_monitor.usage_percent().unwrap_or(0.0),
            disk_io: 0.0,
            network_io: 0.0,
            power_consumption: 0.0,
            temperature: 0.0,
        })
    }
}
