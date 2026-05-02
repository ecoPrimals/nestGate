// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use chrono::{DateTime, Utc};

use nestgate_core::Result;

use super::allocation::ComputeAllocation;
use super::results::LiveHardwareMetrics;

/// **CPU MONITOR**
///
/// Hardware monitor for CPU performance and utilization.
#[derive(Debug, Clone)]
/// Cpumonitor
pub struct CpuMonitor;

/// **MEMORY MONITOR**
///
/// Hardware monitor for memory usage and availability.
#[derive(Debug, Clone)]
/// Memorymonitor
pub struct MemoryMonitor;

/// **GPU MONITOR**
///
/// Hardware monitor for GPU utilization and performance.
#[derive(Debug, Clone)]
/// Gpumonitor
pub struct GpuMonitor;

/// **DISK MONITOR**
///
/// Best-effort root filesystem usage via `df` (see `DiskMonitor::usage_percent`).
#[derive(Debug, Clone)]
pub struct DiskMonitor;

/// **NETWORK MONITOR**
///
/// Best-effort aggregate activity from `/proc/net/dev` (see `NetworkMonitor::usage_percent`).
#[derive(Debug, Clone)]
pub struct NetworkMonitor;

impl CpuMonitor {
    /// Current aggregate CPU utilisation from `/proc/stat` (best-effort, single sample).
    pub(crate) fn usage_percent(&self) -> Result<f64> {
        let _: &Self = self;
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
}

impl MemoryMonitor {
    /// Resident memory pressure as a percentage of total RAM from `/proc/meminfo`.
    pub(crate) fn usage_percent(&self) -> Result<f64> {
        let _: &Self = self;
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
}

impl GpuMonitor {
    /// GPU utilisation via `nvidia-smi` when available.
    pub(crate) fn usage_percent(&self) -> Result<f64> {
        let _: &Self = self;
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
        Ok(0.0)
    }
}

impl DiskMonitor {
    /// Root mount utilisation percentage from `df` (best-effort).
    pub(crate) fn usage_percent(&self) -> Result<f64> {
        let _: &Self = self;
        if std::fs::metadata("/").is_err() {
            return Ok(0.0);
        }
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
                Ok(0.0)
            }
            _ => Ok(0.0),
        }
    }
}

impl NetworkMonitor {
    /// Heuristic activity level from `/proc/net/dev` (not a true link utilisation %).
    pub(crate) fn usage_percent(&self) -> Result<f64> {
        let _: &Self = self;
        match std::fs::read_to_string("/proc/net/dev") {
            Ok(content) => {
                let mut total_bytes = 0u64;
                for line in content.lines().skip(2) {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 10 {
                        let rx_bytes: u64 = parts[1].parse().unwrap_or(0);
                        let tx_bytes: u64 = parts[9].parse().unwrap_or(0);
                        total_bytes += rx_bytes + tx_bytes;
                    }
                }
                Ok(if total_bytes > 0 { 10.0 } else { 0.0 })
            }
            Err(_) => Ok(0.0),
        }
    }
}

/// **TUNING SESSION**
///
/// Active tuning session
#[derive(Debug, Clone)]
/// Tuningsession
pub struct TuningSession {
    /// Unique identifier for the tuning session
    pub session_id: String,
    /// Timestamp when the session was started
    pub started_at: DateTime<Utc>,
    /// Resource allocation configuration for this session
    pub resource_allocation: ComputeAllocation,
    /// Current hardware metrics being monitored
    pub current_metrics: LiveHardwareMetrics,
}

/// **HARDWARE MONITORS**
///
/// Collection of hardware monitoring services.
#[derive(Debug, Clone)]
/// Hardwaremonitors
pub struct HardwareMonitors {
    /// CPU performance monitor
    pub cpu: CpuMonitor,
    /// Memory utilization monitor
    pub memory: MemoryMonitor,
    /// GPU performance monitor (if available)
    pub gpu: Option<GpuMonitor>,
}
