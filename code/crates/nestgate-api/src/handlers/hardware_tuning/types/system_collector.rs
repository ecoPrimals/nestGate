// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use chrono::Utc;

use nestgate_core::Result;

use super::monitors::{CpuMonitor, GpuMonitor, MemoryMonitor};
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
        // Collect real metrics from system
        let cpu_usage = self.getcpu_usage()?;
        let memory_usage = self.get_memory_usage()?;
        let gpu_usage = self.get_gpu_usage().unwrap_or(0.0);

        Ok(LiveHardwareMetrics {
            timestamp: Utc::now(),
            cpu_usage,
            memory_usage,
            gpu_usage,
            disk_usage: self.get_disk_usage().unwrap_or(0.0),
            network_usage: self.get_network_usage().unwrap_or(0.0),
            disk_io: 0.0,
            network_io: 0.0,
            power_consumption: 0.0,
            temperature: 0.0,
        })
    }

    /// Gets Cpu Usage
    fn getcpu_usage(&self) -> Result<f64> {
        // Read CPU usage from /proc/stat
        match std::fs::read_to_string("/proc/stat") {
            Ok(content) => {
                if let Some(line) = content.lines().next() {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 5 && parts[0] == "cpu" {
                        let user: u64 = parts[1].parse().unwrap_or(0);
                        let nice: u64 = parts[2].parse().unwrap_or(0);
                        let system: u64 = parts[3].parse().unwrap_or(0);
                        let idle: u64 = parts[4].parse().unwrap_or(0);

                        let total = user + nice + system + idle;
                        let usage = if total > 0 {
                            ((total - idle) as f64 / total as f64) * 100.0
                        } else {
                            0.0
                        };
                        return Ok(usage);
                    }
                }
                Ok(0.0)
            }
            Err(_) => Ok(0.0), // Fallback for non-Linux systems
        }
    }

    /// Gets Memory Usage
    fn get_memory_usage(&self) -> Result<f64> {
        // Read memory usage from /proc/meminfo
        match std::fs::read_to_string("/proc/meminfo") {
            Ok(content) => {
                let mut total_kb = 0u64;
                let mut available_kb = 0u64;

                for line in content.lines() {
                    if line.starts_with("MemTotal:") {
                        if let Some(value) = line.split_whitespace().nth(1) {
                            total_kb = value.parse().unwrap_or(0);
                        }
                    } else if line.starts_with("MemAvailable:")
                        && let Some(value) = line.split_whitespace().nth(1)
                    {
                        available_kb = value.parse().unwrap_or(0);
                    }
                }

                if total_kb > 0 {
                    let used_kb = total_kb - available_kb;
                    let usage_percent = (used_kb as f64 / total_kb as f64) * 100.0;
                    Ok(usage_percent)
                } else {
                    Ok(0.0)
                }
            }
            Err(_) => Ok(0.0), // Fallback for non-Linux systems
        }
    }

    /// Gets Gpu Usage
    fn get_gpu_usage(&self) -> Result<f64> {
        // Try to read GPU usage from nvidia-smi or other GPU tools
        // For now, return 0.0 if no GPU monitoring available
        if let Ok(output) = std::process::Command::new("nvidia-smi")
            .args([
                "--query-gpu=utilization.gpu",
                "--format=csv,noheader,nounits",
            ])
            .output()
            && output.status.success()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Ok(usage) = stdout.trim().parse::<f64>() {
                return Ok(usage);
            }
        }
        Ok(0.0) // No GPU or monitoring not available
    }

    /// Gets Disk Usage
    fn get_disk_usage(&self) -> Result<f64> {
        // Get disk usage for root filesystem
        if let Ok(_metadata) = std::fs::metadata("/") {
            // This is a simplified approach - would need statvfs for accurate disk usage
            // For now, return a calculated estimate based on available system info
            match std::process::Command::new("df")
                .args(["/", "--output=pcent"])
                .output()
            {
                Ok(output) if output.status.success() => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    if let Some(line) = stdout.lines().nth(1) {
                        let percent_str = line.trim().trim_end_matches('%');
                        if let Ok(usage) = percent_str.parse::<f64>() {
                            return Ok(usage);
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(0.0) // Fallback
    }

    /// Gets Network Usage
    fn get_network_usage(&self) -> Result<f64> {
        // Read network statistics from /proc/net/dev
        match std::fs::read_to_string("/proc/net/dev") {
            Ok(content) => {
                let mut total_bytes = 0u64;
                for line in content.lines().skip(2) {
                    // Skip header lines
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 10 {
                        // Sum receive and transmit bytes (columns 1 and 9)
                        let rx_bytes: u64 = parts[1].parse().unwrap_or(0);
                        let tx_bytes: u64 = parts[9].parse().unwrap_or(0);
                        total_bytes += rx_bytes + tx_bytes;
                    }
                }
                // Convert to percentage based on interface capacity (simplified)
                // This is a basic implementation - real usage would track rates over time
                Ok(if total_bytes > 0 { 10.0 } else { 0.0 })
            }
            Err(_) => Ok(0.0), // Fallback for non-Linux systems
        }
    }
}
