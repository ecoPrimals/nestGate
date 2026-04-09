// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Linux `/proc`-based hardware resource discovery (no `sysinfo`).
//!
//! On non-Linux hosts, values are best-effort from `std::thread::available_parallelism`
//! where applicable; memory may be unavailable without `/proc/meminfo`.

use chrono::Utc;
use nestgate_core::{NestGateError, Result};

use super::types::{ComputeResources, GpuInfo, LiveHardwareMetrics, SystemCapabilities};

/// Read logical CPU count from `/proc/cpuinfo` (Linux), else [`std::thread::available_parallelism`].
pub fn logical_cpu_count() -> Result<u32> {
    #[cfg(target_os = "linux")]
    {
        let cpu_info = std::fs::read_to_string("/proc/cpuinfo").map_err(|e| {
            NestGateError::system(
                "cpu_detection",
                format!("Failed to read /proc/cpuinfo: {e}"),
            )
        })?;
        let n = cpu_info
            .lines()
            .filter(|l| l.starts_with("processor"))
            .count();
        Ok(u32::try_from(n.max(1)).unwrap_or(1))
    }
    #[cfg(not(target_os = "linux"))]
    {
        std::thread::available_parallelism()
            .map(|n| n.get() as u32)
            .map_err(|e| {
                NestGateError::system("cpu_detection", format!("available_parallelism: {e}"))
            })
    }
}

/// Total RAM in GiB from `/proc/meminfo` `MemTotal` (Linux). On other OSes returns `Ok(0)` (unknown).
pub fn mem_total_gib() -> Result<u32> {
    #[cfg(target_os = "linux")]
    {
        let meminfo = std::fs::read_to_string("/proc/meminfo").map_err(|e| {
            NestGateError::system(
                "memory_detection",
                format!("Failed to read /proc/meminfo: {e}"),
            )
        })?;
        let total_kb = meminfo
            .lines()
            .find(|l| l.starts_with("MemTotal:"))
            .and_then(|l| l.split_whitespace().nth(1))
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);
        let gib = (total_kb / 1024 / 1024).max(1);
        Ok(u32::try_from(gib).unwrap_or(u32::MAX))
    }
    #[cfg(not(target_os = "linux"))]
    {
        Ok(0)
    }
}

/// Best-effort GPU count: NVIDIA via `nvidia-smi -L` line count, else 0.
#[must_use]
pub fn gpu_count_best_effort() -> u32 {
    #[cfg(target_os = "linux")]
    {
        if std::path::Path::new("/proc/driver/nvidia/gpus").exists()
            && let Ok(entries) = std::fs::read_dir("/proc/driver/nvidia/gpus")
        {
            return u32::try_from(entries.count()).unwrap_or(0);
        }
    }
    if let Ok(output) = std::process::Command::new("nvidia-smi")
        .args(["-L"])
        .output()
        && output.status.success()
    {
        let s = String::from_utf8_lossy(&output.stdout);
        return u32::try_from(s.lines().filter(|l| !l.trim().is_empty()).count()).unwrap_or(0);
    }
    0
}

/// Build [`ComputeResources`] from `/proc` (and optional GPU tools).
pub fn compute_resources_from_proc() -> Result<ComputeResources> {
    let available_cpu = logical_cpu_count()?.max(1);
    let available_memory_gb = {
        let g = mem_total_gib()?;
        if g == 0 { 1 } else { g }
    };
    let available_gpu = gpu_count_best_effort();
    Ok(ComputeResources {
        available_cpu,
        available_memory_gb,
        available_gpu,
    })
}

/// Human-readable CPU model from `/proc/cpuinfo` (Linux), else `"unknown"`.
pub fn cpu_model_best_effort() -> Result<String> {
    #[cfg(target_os = "linux")]
    {
        let cpu_info = std::fs::read_to_string("/proc/cpuinfo").map_err(|e| {
            NestGateError::system(
                "cpu_detection",
                format!("Failed to read /proc/cpuinfo: {e}"),
            )
        })?;
        let model = cpu_info
            .lines()
            .find(|line| {
                line.starts_with("model name")
                    || line.starts_with("Processor")
                    || line.starts_with("Hardware")
            })
            .and_then(|line| line.split(':').nth(1))
            .map_or_else(|| "Unknown CPU".to_string(), |s| s.trim().to_string());
        Ok(model)
    }
    #[cfg(not(target_os = "linux"))]
    {
        Ok("unknown".to_string())
    }
}

/// First NVIDIA GPU via synchronous `nvidia-smi` (best-effort).
#[must_use]
pub fn nvidia_gpu_info_best_effort() -> Option<GpuInfo> {
    if let Ok(output) = std::process::Command::new("nvidia-smi")
        .args([
            "--query-gpu=name,memory.total",
            "--format=csv,noheader,nounits",
        ])
        .output()
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

/// CPU, memory, and optional GPU capabilities from `/proc` and `nvidia-smi`.
pub fn system_capabilities_from_proc() -> Result<SystemCapabilities> {
    let cpu_cores = usize::try_from(logical_cpu_count()?.max(1)).unwrap_or(1);
    let memory_gb = u64::from(mem_total_gib()?.max(1));
    let cpu_model = cpu_model_best_effort()?;
    let gpu_info = nvidia_gpu_info_best_effort();
    Ok(SystemCapabilities {
        cpu_cores,
        cpu_model,
        memory_gb,
        gpu_available: gpu_info.is_some(),
        gpu_info,
    })
}

/// Aggregated live metrics from `/proc` (and best-effort `df` / `nvidia-smi`), with zeros on failure.
#[must_use]
pub fn live_hardware_metrics_best_effort() -> LiveHardwareMetrics {
    #[cfg(not(target_os = "linux"))]
    {
        return zeroed_live_metrics();
    }
    #[cfg(target_os = "linux")]
    {
        let timestamp = Utc::now();
        let cpu_usage = cpu_usage_percent_from_stat();
        let memory_usage = memory_usage_percent_from_meminfo();
        let gpu_usage = gpu_usage_from_nvidia_smi();
        let (disk_io, disk_usage) = disk_metrics_best_effort();
        let (network_io, network_usage) = network_metrics_from_proc_net_dev();
        LiveHardwareMetrics {
            timestamp,
            cpu_usage,
            memory_usage,
            gpu_usage,
            disk_io,
            disk_usage,
            network_io,
            network_usage,
            temperature: 0.0,
            power_consumption: 0.0,
        }
    }
}

#[cfg(not(target_os = "linux"))]
fn zeroed_live_metrics() -> LiveHardwareMetrics {
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

#[cfg(target_os = "linux")]
fn cpu_usage_percent_from_stat() -> f64 {
    let Ok(content) = std::fs::read_to_string("/proc/stat") else {
        return 0.0;
    };
    let Some(line) = content.lines().next() else {
        return 0.0;
    };
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 5 || parts[0] != "cpu" {
        return 0.0;
    }
    let user: u64 = parts[1].parse().unwrap_or(0);
    let nice: u64 = parts[2].parse().unwrap_or(0);
    let system: u64 = parts[3].parse().unwrap_or(0);
    let idle: u64 = parts[4].parse().unwrap_or(0);
    let total = user + nice + system + idle;
    if total == 0 {
        return 0.0;
    }
    ((total - idle) as f64 / total as f64) * 100.0
}

#[cfg(target_os = "linux")]
fn memory_usage_percent_from_meminfo() -> f64 {
    let Ok(content) = std::fs::read_to_string("/proc/meminfo") else {
        return 0.0;
    };
    let mut total_kb = 0u64;
    let mut available_kb = 0u64;
    for line in content.lines() {
        if line.starts_with("MemTotal:") {
            total_kb = line
                .split_whitespace()
                .nth(1)
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
        } else if line.starts_with("MemAvailable:") {
            available_kb = line
                .split_whitespace()
                .nth(1)
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
        }
    }
    if total_kb == 0 {
        return 0.0;
    }
    let used_kb = total_kb.saturating_sub(available_kb);
    (used_kb as f64 / total_kb as f64) * 100.0
}

#[cfg(target_os = "linux")]
fn gpu_usage_from_nvidia_smi() -> f64 {
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
            return usage;
        }
    }
    0.0
}

#[cfg(target_os = "linux")]
fn disk_metrics_best_effort() -> (f64, f64) {
    let mut disk_io_score = 0.0_f64;
    if let Ok(content) = std::fs::read_to_string("/proc/diskstats") {
        let mut sectors = 0u64;
        for line in content.lines() {
            let cols: Vec<&str> = line.split_whitespace().collect();
            if cols.len() < 10 {
                continue;
            }
            let name = cols[2];
            if name.starts_with("loop") || name.starts_with("zram") {
                continue;
            }
            let read_sectors: u64 = cols[5].parse().unwrap_or(0);
            let write_sectors: u64 = cols[9].parse().unwrap_or(0);
            sectors = sectors.saturating_add(read_sectors.saturating_add(write_sectors));
        }
        if sectors > 0 {
            disk_io_score = f64::min((sectors as f64).log10() * 12.5, 100.0);
        }
    }
    let disk_usage = root_disk_usage_percent_df();
    (disk_io_score, disk_usage)
}

#[cfg(target_os = "linux")]
fn root_disk_usage_percent_df() -> f64 {
    if let Ok(output) = std::process::Command::new("df")
        .args(["/", "--output=pcent", "-B1"])
        .output()
        && output.status.success()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Some(line) = stdout.lines().nth(1) {
            let percent_str = line.trim().trim_end_matches('%');
            if let Ok(usage) = percent_str.parse::<f64>() {
                return usage;
            }
        }
    }
    0.0
}

#[cfg(target_os = "linux")]
fn network_metrics_from_proc_net_dev() -> (f64, f64) {
    let Ok(content) = std::fs::read_to_string("/proc/net/dev") else {
        return (0.0, 0.0);
    };
    let mut total_bytes = 0u64;
    for line in content.lines().skip(2) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 10 {
            let rx_bytes: u64 = parts[1].parse().unwrap_or(0);
            let tx_bytes: u64 = parts[9].parse().unwrap_or(0);
            total_bytes = total_bytes.saturating_add(rx_bytes.saturating_add(tx_bytes));
        }
    }
    let network_io = if total_bytes > 0 {
        f64::min((total_bytes as f64).log10() * 5.0, 100.0)
    } else {
        0.0
    };
    let network_usage = if total_bytes > 0 { 10.0 } else { 0.0 };
    (network_io, network_usage)
}

/// Storage profile label from `/sys/block/*/queue/rotational` (SSD vs HDD mix).
#[must_use]
pub fn storage_profile_from_sysfs() -> String {
    #[cfg(not(target_os = "linux"))]
    {
        return "unknown".to_string();
    }
    #[cfg(target_os = "linux")]
    {
        let Ok(entries) = std::fs::read_dir("/sys/block") else {
            return "unknown".to_string();
        };
        let mut rotational = 0u32;
        let mut solid = 0u32;
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name.starts_with("loop") || name.starts_with("zram") || name.starts_with("dm-") {
                continue;
            }
            let path = format!("/sys/block/{name}/queue/rotational");
            if let Ok(s) = std::fs::read_to_string(path) {
                let v = s.trim();
                if v == "1" {
                    rotational += 1;
                } else if v == "0" {
                    solid += 1;
                }
            }
        }
        if solid > 0 && rotational > 0 {
            "mixed_ssd_hdd".to_string()
        } else if solid > 0 {
            "solid_state".to_string()
        } else if rotational > 0 {
            "rotational".to_string()
        } else {
            "unknown".to_string()
        }
    }
}

/// Network profile hint from `/sys/class/net` interface names (best-effort).
#[must_use]
pub fn network_profile_from_sysfs() -> String {
    #[cfg(not(target_os = "linux"))]
    {
        return "unknown".to_string();
    }
    #[cfg(target_os = "linux")]
    {
        let Ok(entries) = std::fs::read_dir("/sys/class/net") else {
            return "unknown".to_string();
        };
        let mut has_wifi = false;
        let mut has_eth = false;
        for entry in entries.flatten() {
            let name = entry.file_name();
            let n = name.to_string_lossy();
            if n == "lo" {
                continue;
            }
            if n.starts_with("wl") {
                has_wifi = true;
            } else if n.starts_with("en") || n.starts_with("eth") {
                has_eth = true;
            }
        }
        match (has_eth, has_wifi) {
            (true, true) => "ethernet_and_wifi".to_string(),
            (true, false) => "ethernet".to_string(),
            (false, true) => "wifi".to_string(),
            (false, false) => "unknown".to_string(),
        }
    }
}
