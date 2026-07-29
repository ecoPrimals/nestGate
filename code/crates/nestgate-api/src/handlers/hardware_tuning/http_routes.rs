// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! HTTP route handlers for `/hardware/config` and `/hardware/tune`.

use std::sync::OnceLock;

use axum::{http::StatusCode, response::Json};
use chrono::{DateTime, Utc};
use nestgate_platform::linux_proc;
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::linux_proc as tuning_proc;
use super::procfs_helpers;
use super::types::{ComputeResources, HardwareTuningConfig, SystemCapabilities, SystemProfile};

/// Hardware snapshot captured once at first access (startup detection).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareStartupConfig {
    /// When the host hardware was first detected for this process.
    pub detected_at: DateTime<Utc>,
    /// Compute resources from `/proc` and best-effort GPU probes.
    pub compute: ComputeResources,
    /// CPU, memory, and optional GPU capabilities.
    pub capabilities: SystemCapabilities,
    /// Derived workload profiles (storage, network, CPU, memory).
    pub profiles: SystemProfile,
    /// Tuning configuration derived from detected hardware.
    pub tuning_config: HardwareTuningConfig,
    /// Root filesystem disk summary when available.
    pub disk: DiskSummary,
}

/// Root filesystem capacity summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskSummary {
    /// Total bytes on the root mount (when discoverable).
    pub total_bytes: u64,
    /// Available bytes on the root mount (when discoverable).
    pub available_bytes: u64,
    /// Used percentage (0–100) when total is non-zero.
    pub used_percent: f64,
}

/// A single tuning recommendation derived from live hardware metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuningRecommendation {
    /// Category (`cpu`, `memory`, `storage`, `network`, `gpu`).
    pub category: String,
    /// Actionable recommendation text.
    pub recommendation: String,
    /// Why this recommendation applies on the current host.
    pub rationale: String,
    /// Suggested value or setting when applicable.
    pub suggested_value: Option<String>,
}

static STARTUP_HARDWARE_CONFIG: OnceLock<HardwareStartupConfig> = OnceLock::new();

fn root_disk_summary() -> DiskSummary {
    let mut summary = DiskSummary {
        total_bytes: 0,
        available_bytes: 0,
        used_percent: 0.0,
    };

    #[cfg(target_os = "linux")]
    if let Ok((total, avail)) = linux_proc::statvfs_space(std::path::Path::new("/")) {
        summary.total_bytes = total;
        summary.available_bytes = avail;
        if total > 0 {
            let used = total.saturating_sub(avail);
            let scaled = (u128::from(used).saturating_mul(10_000)) / u128::from(total);
            let scaled = u32::try_from(scaled).unwrap_or(10_000);
            summary.used_percent = f64::from(scaled) / 100.0;
        }
    }

    summary
}

#[expect(
    clippy::missing_const_for_fn,
    reason = "HardwareTuningConfig carries runtime-derived CPU/memory/GPU flags"
)]
fn derive_tuning_config(
    compute: &ComputeResources,
    capabilities: &SystemCapabilities,
) -> HardwareTuningConfig {
    HardwareTuningConfig {
        cpu_cores: compute.available_cpu,
        memory_gb: compute.available_memory_gb,
        cpu_tuning_enabled: compute.available_cpu >= 2,
        memory_optimization_enabled: compute.available_memory_gb >= 4,
        gpu_tuning_enabled: capabilities.gpu_available,
        monitoring_interval: std::time::Duration::from_secs(5),
    }
}

fn detect_hardware_config() -> HardwareStartupConfig {
    let compute = tuning_proc::compute_resources_from_proc()
        .unwrap_or_else(|_| procfs_helpers::snapshot_compute_resources());
    let capabilities =
        tuning_proc::system_capabilities_from_proc().unwrap_or_else(|_| SystemCapabilities {
            cpu_cores: usize::try_from(compute.available_cpu.max(1)).unwrap_or(1),
            cpu_model: String::from("unknown"),
            memory_gb: u64::from(compute.available_memory_gb.max(1)),
            gpu_available: compute.available_gpu > 0,
            gpu_info: None,
        });
    let profiles = procfs_helpers::snapshot_system_profile();
    let tuning_config = derive_tuning_config(&compute, &capabilities);

    HardwareStartupConfig {
        detected_at: Utc::now(),
        compute,
        capabilities,
        profiles,
        tuning_config,
        disk: root_disk_summary(),
    }
}

/// Return the hardware configuration detected at process startup.
pub async fn get_hardware_config() -> Result<Json<serde_json::Value>, StatusCode> {
    let config = STARTUP_HARDWARE_CONFIG.get_or_init(detect_hardware_config);
    Ok(Json(json!({
        "status": "success",
        "detected_at": config.detected_at,
        "compute": config.compute,
        "capabilities": config.capabilities,
        "profiles": config.profiles,
        "tuning_config": config.tuning_config,
        "disk": config.disk,
    })))
}

fn build_tuning_recommendations(
    compute: &ComputeResources,
    capabilities: &SystemCapabilities,
    profiles: &SystemProfile,
    disk: &DiskSummary,
) -> Vec<TuningRecommendation> {
    let mut recommendations = Vec::new();

    recommendations.push(TuningRecommendation {
        category: "cpu".into(),
        recommendation: format!(
            "Configure worker pools to use up to {} logical CPUs",
            compute.available_cpu
        ),
        rationale: format!(
            "Host reports {} logical CPUs ({})",
            compute.available_cpu, capabilities.cpu_model
        ),
        suggested_value: Some(compute.available_cpu.to_string()),
    });

    let arc_target_gib = (u64::from(compute.available_memory_gb) / 2).max(1);
    recommendations.push(TuningRecommendation {
        category: "memory".into(),
        recommendation: format!(
            "Target ZFS ARC or page cache around {arc_target_gib} GiB on this host"
        ),
        rationale: format!(
            "Total RAM is approximately {} GiB; leave headroom for workloads",
            compute.available_memory_gb
        ),
        suggested_value: Some(format!("{arc_target_gib}G")),
    });

    let storage_rec = match profiles.storage_profile.as_str() {
        "solid_state" => (
            "Enable lz4 compression and consider smaller record sizes for SSD-backed pools",
            "Storage profile indicates solid-state media",
            Some("compression=lz4".into()),
        ),
        "rotational" => (
            "Prefer larger records and schedule scrubs during low-load windows on HDD media",
            "Storage profile indicates rotational disks",
            None,
        ),
        "mixed_ssd_hdd" => (
            "Use tiered pools: place hot datasets on SSD vdevs and bulk data on HDD vdevs",
            "Mixed SSD/HDD block devices detected under /sys/block",
            None,
        ),
        _ => (
            "Validate storage media type before applying ZFS record-size or compression defaults",
            "Storage profile could not be determined from sysfs",
            None,
        ),
    };
    recommendations.push(TuningRecommendation {
        category: "storage".into(),
        recommendation: storage_rec.0.into(),
        rationale: storage_rec.1.into(),
        suggested_value: storage_rec.2,
    });

    if disk.total_bytes > 0 && disk.used_percent > 85.0 {
        recommendations.push(TuningRecommendation {
            category: "storage".into(),
            recommendation:
                "Plan capacity expansion or dataset cleanup — root filesystem is over 85% full"
                    .into(),
            rationale: format!("Root filesystem is {:.1}% utilized", disk.used_percent),
            suggested_value: None,
        });
    }

    recommendations.push(TuningRecommendation {
        category: "network".into(),
        recommendation: match profiles.network_profile.as_str() {
            "ethernet" => "Tuned for wired throughput; increase socket buffers for bulk transfers",
            "wifi" => "Prefer smaller I/O sizes and enable retry-friendly timeouts on Wi-Fi",
            "ethernet_and_wifi" => {
                "Segment bulk storage traffic onto wired interfaces when possible"
            }
            _ => "Confirm primary network interface before tuning buffer sizes",
        }
        .into(),
        rationale: format!("Network profile: {}", profiles.network_profile),
        suggested_value: None,
    });

    if capabilities.gpu_available {
        recommendations.push(TuningRecommendation {
            category: "gpu".into(),
            recommendation: "Enable GPU-aware workloads where supported; monitor VRAM separately from system RAM"
                .into(),
            rationale: "NVIDIA or procfs GPU presence detected".into(),
            suggested_value: Some("gpu_tuning_enabled=true".into()),
        });
    }

    recommendations
}

/// Analyze current hardware and return tuning recommendations (observational; no privileged changes).
pub async fn post_hardware_tune() -> Result<Json<serde_json::Value>, StatusCode> {
    let startup = STARTUP_HARDWARE_CONFIG.get_or_init(detect_hardware_config);
    let compute = tuning_proc::compute_resources_from_proc()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let capabilities = tuning_proc::system_capabilities_from_proc()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let profiles = procfs_helpers::snapshot_system_profile();
    let disk = root_disk_summary();
    let live_metrics = tuning_proc::live_hardware_metrics_best_effort();
    let recommendations = build_tuning_recommendations(&compute, &capabilities, &profiles, &disk);

    Ok(Json(json!({
        "status": "success",
        "message": "Tuning recommendations derived from live hardware discovery (no kernel changes applied)",
        "startup_detected_at": startup.detected_at,
        "hardware_snapshot": {
            "compute": compute,
            "capabilities": capabilities,
            "profiles": profiles,
            "disk": disk,
            "live_metrics": live_metrics,
        },
        "recommendations": recommendations,
    })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hardware_config_detects_nonzero_resources() {
        let config = detect_hardware_config();
        assert!(config.compute.available_cpu >= 1);
        assert!(config.compute.available_memory_gb >= 1);
        assert!(config.capabilities.cpu_cores >= 1);
    }

    #[test]
    fn tuning_recommendations_include_core_categories() {
        let compute = ComputeResources {
            available_cpu: 8,
            available_memory_gb: 32,
            available_gpu: 0,
        };
        let capabilities = SystemCapabilities {
            cpu_cores: 8,
            cpu_model: "Test CPU".into(),
            memory_gb: 32,
            gpu_available: false,
            gpu_info: None,
        };
        let profiles = SystemProfile {
            cpu_profile: "8 cores: Test CPU".into(),
            memory_profile: "32 GiB total".into(),
            storage_profile: "solid_state".into(),
            network_profile: "ethernet".into(),
        };
        let disk = DiskSummary {
            total_bytes: 1_000_000_000_000,
            available_bytes: 500_000_000_000,
            used_percent: 50.0,
        };
        let recs = build_tuning_recommendations(&compute, &capabilities, &profiles, &disk);
        let categories: Vec<&str> = recs.iter().map(|r| r.category.as_str()).collect();
        assert!(categories.contains(&"cpu"));
        assert!(categories.contains(&"memory"));
        assert!(categories.contains(&"storage"));
        assert!(categories.contains(&"network"));
    }

    #[tokio::test]
    async fn get_hardware_config_returns_success() {
        let result = get_hardware_config().await;
        assert!(result.is_ok());
        if let Ok(Json(body)) = result {
            assert_eq!(body.get("status").and_then(|v| v.as_str()), Some("success"));
            assert!(body.get("detected_at").is_some());
            assert!(body.get("tuning_config").is_some());
        }
    }

    #[tokio::test]
    async fn post_hardware_tune_returns_recommendations() {
        let result = post_hardware_tune().await;
        assert!(result.is_ok());
        if let Ok(Json(body)) = result {
            assert_eq!(body.get("status").and_then(|v| v.as_str()), Some("success"));
            assert!(
                body.get("recommendations")
                    .and_then(|v| v.as_array())
                    .is_some_and(|a| !a.is_empty())
            );
        }
    }
}
