// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Linux hardware resource discovery delegating to `nestgate_platform::linux_proc`
//! for standard system metrics, with hardware-tuning-specific extensions
//! (GPU detection, sysfs profiles).

use chrono::Utc;
use nestgate_core::{NestGateError, Result};
use nestgate_platform::linux_proc;

use super::types::{ComputeResources, GpuInfo, LiveHardwareMetrics, SystemCapabilities};

/// Read logical CPU count, delegating to `linux_proc::logical_cpu_count`.
pub fn logical_cpu_count() -> Result<u32> {
    let count = linux_proc::logical_cpu_count();
    if count == 0 {
        return Err(NestGateError::system(
            "cpu_detection",
            "logical_cpu_count returned 0".to_string(),
        ));
    }
    Ok(u32::try_from(count).unwrap_or(u32::MAX))
}

/// Total RAM in GiB from `linux_proc::total_memory_bytes`.
#[must_use]
pub fn mem_total_gib() -> u32 {
    let bytes = linux_proc::total_memory_bytes().unwrap_or(0);
    let gib = (bytes / 1024 / 1024 / 1024).max(1);
    u32::try_from(gib).unwrap_or(u32::MAX)
}

/// Best-effort GPU count: NVIDIA via `/proc/driver/nvidia/gpus` or `nvidia-smi -L`.
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

/// Build [`ComputeResources`] from `linux_proc` (and optional GPU tools).
pub fn compute_resources_from_proc() -> Result<ComputeResources> {
    Ok(ComputeResources {
        available_cpu: logical_cpu_count()?.max(1),
        available_memory_gb: mem_total_gib().max(1),
        available_gpu: gpu_count_best_effort(),
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
            .map_or_else(|| "Unknown CPU".into(), |s| s.trim().to_string());
        Ok(model)
    }
    #[cfg(not(target_os = "linux"))]
    {
        Ok("unknown".into())
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

/// CPU, memory, and optional GPU capabilities.
pub fn system_capabilities_from_proc() -> Result<SystemCapabilities> {
    let cpu_cores = usize::try_from(logical_cpu_count()?.max(1)).unwrap_or(1);
    let memory_gb = u64::from(mem_total_gib().max(1));
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

/// Aggregated live metrics from `linux_proc` + best-effort GPU/disk, with zeros on failure.
#[must_use]
pub fn live_hardware_metrics_best_effort() -> LiveHardwareMetrics {
    let cpu_usage = linux_proc::globalcpu_usage_percent_from_stat().unwrap_or(0.0);
    let memory_usage = linux_proc::memory_usage_percent().unwrap_or(0.0);
    let gpu_usage = gpu_usage_from_nvidia_smi();
    let (disk_io, disk_usage) = disk_metrics_best_effort();
    let (network_io, network_usage) = network_metrics_best_effort();
    LiveHardwareMetrics {
        timestamp: Utc::now(),
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

fn disk_metrics_best_effort() -> (f64, f64) {
    let disk_io_score = linux_proc::diskstats_entry_count().unwrap_or(0.0);
    let disk_usage = root_disk_usage_percent_df();
    (disk_io_score, disk_usage)
}

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

fn network_metrics_best_effort() -> (f64, f64) {
    let (rx, tx) = linux_proc::network_rx_tx_bytes_sum().unwrap_or((0, 0));
    let total_bytes = rx.saturating_add(tx);
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
        return "unknown".into();
    }
    #[cfg(target_os = "linux")]
    {
        let Ok(entries) = std::fs::read_dir("/sys/block") else {
            return "unknown".into();
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
            "mixed_ssd_hdd".into()
        } else if solid > 0 {
            "solid_state".into()
        } else if rotational > 0 {
            "rotational".into()
        } else {
            "unknown".into()
        }
    }
}

/// Network profile hint from `/sys/class/net` interface names (best-effort).
#[must_use]
pub fn network_profile_from_sysfs() -> String {
    #[cfg(not(target_os = "linux"))]
    {
        return "unknown".into();
    }
    #[cfg(target_os = "linux")]
    {
        let Ok(entries) = std::fs::read_dir("/sys/class/net") else {
            return "unknown".into();
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
            (true, true) => "ethernet_and_wifi".into(),
            (true, false) => "ethernet".into(),
            (false, true) => "wifi".into(),
            (false, false) => "unknown".into(),
        }
    }
}
