// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Health checks via active alerts and threshold helpers.

use axum::{extract::State, response::Json};
use tracing::{debug, info};

use crate::rest::models::{Alert, AlertCondition, AlertSeverity, AlertStatus, ComparisonOperator};
use crate::rest::{ApiState, DataError, DataResponse};

const CPU_ALERT_THRESHOLD_PERCENT: f64 = 80.0;
const MEMORY_ALERT_THRESHOLD_PERCENT: f64 = 85.0;

fn push_dataset_count_alert(alerts: &mut Vec<Alert>, total_datasets: usize) {
    alerts.push(Alert {
        id: String::from("alert_001"),
        name: String::from("High Dataset Count"),
        description: format!("System has {total_datasets} datasets, consider consolidation"),
        message: format!("High dataset count detected: {total_datasets}"),
        severity: AlertSeverity::Warning,
        status: AlertStatus::Active,
        created_at: chrono::Utc::now() - chrono::Duration::minutes(20),
        triggered_at: chrono::Utc::now() - chrono::Duration::minutes(15),
        conditions: vec![AlertCondition {
            metric_name: String::from("total_datasets"),
            operator: ComparisonOperator::GreaterThan,
            threshold: 10.0,
            duration_seconds: 300,
            currentvalue: total_datasets as f64,
        }],
        suggested_actions: vec![
            String::from("Review dataset organization"),
            String::from("Consider consolidating similar datasets"),
            String::from("Implement dataset archival policy"),
        ],
    });
}

fn pushcpu_usage_alert(alerts: &mut Vec<Alert>, current_cpu: f64) {
    alerts.push(Alert {
        id: String::from("alert_002"),
        name: String::from("High CPU Usage"),
        description: format!("CPU usage is at {current_cpu:.1}%"),
        message: format!("High CPU usage alert: {current_cpu:.1}%"),
        severity: if current_cpu > 95.0 {
            AlertSeverity::Critical
        } else {
            AlertSeverity::Warning
        },
        status: AlertStatus::Active,
        created_at: chrono::Utc::now() - chrono::Duration::minutes(10),
        triggered_at: chrono::Utc::now() - chrono::Duration::minutes(5),
        conditions: vec![AlertCondition {
            metric_name: String::from("cpu_usage_percent"),
            operator: ComparisonOperator::GreaterThan,
            threshold: CPU_ALERT_THRESHOLD_PERCENT,
            duration_seconds: 300,
            currentvalue: current_cpu,
        }],
        suggested_actions: vec![
            String::from("Check for resource-intensive processes"),
            String::from("Consider scaling resources"),
            String::from("Review system performance"),
        ],
    });
}

fn push_memory_usage_alert(alerts: &mut Vec<Alert>, current_memory: f64) {
    alerts.push(Alert {
        id: String::from("alert_003"),
        name: String::from("High Memory Usage"),
        description: format!("Memory usage is at {current_memory:.1}%"),
        message: format!("High memory usage alert: {current_memory:.1}%"),
        severity: if current_memory > 95.0 {
            AlertSeverity::Critical
        } else {
            AlertSeverity::Warning
        },
        status: AlertStatus::Active,
        created_at: chrono::Utc::now() - chrono::Duration::minutes(12),
        triggered_at: chrono::Utc::now() - chrono::Duration::minutes(8),
        conditions: vec![AlertCondition {
            metric_name: String::from("memory_usage_percent"),
            operator: ComparisonOperator::GreaterThan,
            threshold: MEMORY_ALERT_THRESHOLD_PERCENT,
            duration_seconds: 300,
            currentvalue: current_memory,
        }],
        suggested_actions: vec![
            String::from("Clear memory caches"),
            String::from("Restart memory-intensive services"),
            String::from("Add more RAM if consistently high"),
        ],
    });
}

fn push_storage_usage_alert(alerts: &mut Vec<Alert>) {
    #[cfg(not(target_os = "linux"))]
    {
        return;
    }
    #[cfg(target_os = "linux")]
    {
        let Ok((total_bytes, avail_bytes)) =
            nestgate_core::linux_proc::statvfs_space(std::path::Path::new("/"))
        else {
            return;
        };
        if total_bytes == 0 {
            return;
        }
        let used = total_bytes.saturating_sub(avail_bytes);
        let usage_percent = (used as f64 / total_bytes as f64) * 100.0;

        if usage_percent <= 80.0 {
            return;
        }

        alerts.push(Alert {
            id: String::from("alert_004"),
            name: String::from("High Storage Usage"),
            description: format!("Storage usage is at {usage_percent:.1}%"),
            message: format!("High storage usage alert: {usage_percent:.1}%"),
            severity: if usage_percent > 95.0 {
                AlertSeverity::Critical
            } else {
                AlertSeverity::Warning
            },
            status: AlertStatus::Active,
            created_at: chrono::Utc::now() - chrono::Duration::minutes(15),
            triggered_at: chrono::Utc::now() - chrono::Duration::minutes(12),
            conditions: vec![AlertCondition {
                metric_name: String::from("storage_usage_percent"),
                operator: ComparisonOperator::GreaterThan,
                threshold: 80.0,
                duration_seconds: 300,
                currentvalue: usage_percent,
            }],
            suggested_actions: vec![
                String::from("Clean up old snapshots"),
                String::from("Enable compression on datasets"),
                String::from("Add additional storage capacity"),
                String::from("Archive old data"),
            ],
        });
    }
}

fn push_resolved_network_example(alerts: &mut Vec<Alert>) {
    alerts.push(Alert {
        id: String::from("alert_005"),
        name: String::from("Network Connectivity"),
        description: String::from("Network connectivity was temporarily degraded"),
        message: String::from("Network connectivity issue resolved"),
        severity: AlertSeverity::Warning,
        status: AlertStatus::Resolved,
        created_at: chrono::Utc::now() - chrono::Duration::hours(2) - chrono::Duration::minutes(5),
        triggered_at: chrono::Utc::now() - chrono::Duration::hours(2),
        conditions: vec![AlertCondition {
            metric_name: String::from("network_latency_ms"),
            operator: ComparisonOperator::GreaterThan,
            threshold: 100.0,
            duration_seconds: 300,
            currentvalue: 15.0,
        }],
        suggested_actions: vec![
            String::from("Monitor network performance"),
            String::from("Check network infrastructure"),
        ],
    });
}

/// Get active alerts.
///
/// GET `/api/v1/monitoring/alerts`
///
/// # Errors
///
/// Returns [`Json<DataError>`] when the monitoring subsystem fails to build the response (rare for this handler).
pub async fn get_alerts(
    State(state): State<ApiState>,
) -> Result<Json<DataResponse<Vec<Alert>>>, Json<DataError>> {
    debug!("Getting active alerts");
    let mut alerts = Vec::new();

    let total_datasets = state.zfs_engines.len();

    if total_datasets > 10 {
        push_dataset_count_alert(&mut alerts, total_datasets);
    }

    // ecoBin v3.0: `/proc/stat` on Linux; `sysinfo` fallback when `/proc` parse fails.
    #[cfg(all(target_os = "linux", feature = "sysinfo"))]
    let current_cpu = nestgate_core::linux_proc::globalcpu_usage_percent_from_stat()
        .unwrap_or_else(|| {
            let mut sys = sysinfo::System::new_all();
            sys.refresh_cpu();
            f64::from(sys.global_cpu_info().cpu_usage())
        });
    #[cfg(all(target_os = "linux", not(feature = "sysinfo")))]
    let current_cpu = nestgate_core::linux_proc::globalcpu_usage_percent_from_stat().unwrap_or(0.0);
    #[cfg(all(not(target_os = "linux"), feature = "sysinfo"))]
    let current_cpu = {
        let mut sys = sysinfo::System::new_all();
        sys.refresh_cpu();
        f64::from(sys.global_cpu_info().cpu_usage())
    };
    #[cfg(all(not(target_os = "linux"), not(feature = "sysinfo")))]
    let current_cpu = 0.0;
    if current_cpu > CPU_ALERT_THRESHOLD_PERCENT {
        pushcpu_usage_alert(&mut alerts, current_cpu);
    }

    #[cfg(all(target_os = "linux", feature = "sysinfo"))]
    let current_memory = nestgate_core::linux_proc::memory_usage_percent().unwrap_or_else(|| {
        let mut sys = sysinfo::System::new_all();
        sys.refresh_memory();
        let total = sys.total_memory();
        if total == 0 {
            0.0
        } else {
            (sys.used_memory() as f64 / total as f64) * 100.0
        }
    });
    #[cfg(all(target_os = "linux", not(feature = "sysinfo")))]
    let current_memory = nestgate_core::linux_proc::memory_usage_percent().unwrap_or(0.0);
    #[cfg(all(not(target_os = "linux"), feature = "sysinfo"))]
    let current_memory = {
        let mut sys = sysinfo::System::new_all();
        sys.refresh_memory();
        let total = sys.total_memory();
        if total == 0 {
            0.0
        } else {
            (sys.used_memory() as f64 / total as f64) * 100.0
        }
    };
    #[cfg(all(not(target_os = "linux"), not(feature = "sysinfo")))]
    let current_memory = 0.0;
    if current_memory > MEMORY_ALERT_THRESHOLD_PERCENT {
        push_memory_usage_alert(&mut alerts, current_memory);
    }

    push_storage_usage_alert(&mut alerts);

    if !alerts.is_empty() {
        push_resolved_network_example(&mut alerts);
    }

    info!("Retrieved {} alerts", alerts.len());
    Ok(Json(DataResponse::new(alerts)))
}
