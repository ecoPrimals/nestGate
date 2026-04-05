// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Core capacity monitoring, performance bottleneck detection, and maintenance scheduling

use crate::types::{BottleneckReport, CapacityReport, MaintenanceSchedule, SystemInfo};
use nestgate_core::error::CanonicalResult as Result;
use tracing::debug;

/// Storage capacity monitoring with basic forecasting
pub fn monitor_capacity_usage(
    dataset: &str,
    historical_data: &[SystemInfo],
) -> Result<CapacityReport> {
    debug!("Monitoring capacity usage for dataset: {}", dataset);
    // Basic capacity analysis based on historical data
    let current_usage = historical_data.last().map_or(0.0, |info| info.disk_usage);

    let growth_rate = if historical_data.len() > 1 {
        if let Some(recent) = historical_data.last() {
            let previous = &historical_data[historical_data.len() - 2];
            if let (Ok(recent_duration), Ok(previous_duration)) = (
                recent.timestamp.duration_since(std::time::UNIX_EPOCH),
                previous.timestamp.duration_since(std::time::UNIX_EPOCH),
            ) {
                let time_diff =
                    recent_duration.as_secs() as f64 - previous_duration.as_secs() as f64;
                if time_diff > 0.0 {
                    (recent.disk_usage - previous.disk_usage) / time_diff
                } else {
                    0.0
                }
            } else {
                0.0
            }
        } else {
            0.0
        }
    } else {
        0.0
    };

    Ok(CapacityReport {
        dataset: dataset.to_string(),
        current_usage: current_usage as u64,
        projected_usage: if growth_rate > 0.0 {
            ((current_usage + growth_rate * 30.0) as u64).min(100)
        } else {
            current_usage as u64
        },
        recommendations: vec![
            "Monitor capacity trends".to_string(),
            "Consider tier migration if growth continues".to_string(),
        ],
    })
}

/// Performance bottleneck detection using metrics analysis
pub fn detect_performance_bottlenecks(performance_data: &[SystemInfo]) -> Result<BottleneckReport> {
    debug!("Detecting performance bottlenecks from metrics");
    let mut bottlenecks = Vec::new();
    let mut recommendations = Vec::new();

    if let Some(latest) = performance_data.last() {
        // CPU bottleneck detection
        if latest.cpu_usage > 80.0 {
            bottlenecks.push("High CPU usage".to_string());
            recommendations.push("Consider CPU upgrade or workload optimization".to_string());
        }

        // Memory bottleneck detection
        if latest.memory_usage > 85.0 {
            bottlenecks.push("High memory usage".to_string());
            recommendations.push("Increase system memory or tune ARC settings".to_string());
        }

        // I/O bottleneck detection (using disk usage as proxy)
        if latest.disk_usage > 90.0 {
            bottlenecks.push("High disk usage".to_string());
            recommendations.push("Consider storage expansion or data cleanup".to_string());
        }
    }

    let severity = if bottlenecks.len() > 2 {
        "high"
    } else if !bottlenecks.is_empty() {
        "medium"
    } else {
        "low"
    };

    Ok(BottleneckReport {
        dataset: "system".to_string(),
        bottleneck_type: if bottlenecks.is_empty() {
            "none".to_string()
        } else {
            bottlenecks[0].clone()
        },
        severity: severity.to_string(),
        recommendations,
    })
}

/// Generate maintenance schedule based on system metrics
pub fn generate_maintenance_schedule(
    dataset: &str,
    health_data: &[SystemInfo],
) -> Result<MaintenanceSchedule> {
    debug!("Generating maintenance schedule for dataset: {}", dataset);
    let mut scheduled_tasks = Vec::new();
    let mut priority = "low";

    if let Some(latest) = health_data.last() {
        // Pool scrub scheduling (simplified check based on timestamp)
        let days_since_epoch = latest
            .timestamp
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            / 86400;
        if days_since_epoch % 30 == 0 {
            // Schedule scrub every 30 days
            scheduled_tasks.push("Schedule pool scrub".to_string());
            priority = "high";
        }

        // Snapshot cleanup scheduling (simplified check based on disk usage)
        if latest.disk_usage > 80.0 {
            scheduled_tasks.push("Clean up old snapshots".to_string());
            if priority == "low" {
                priority = "medium";
            }
        }

        // Defragmentation scheduling (simplified check based on disk usage)
        if latest.disk_usage > 85.0 {
            scheduled_tasks.push("Defragment pool".to_string());
            let _priority = if priority == "low" {
                "medium"
            } else {
                priority
            };
        }
    }

    Ok(MaintenanceSchedule {
        dataset: dataset.to_string(),
        next_maintenance: std::time::SystemTime::now() + std::time::Duration::from_secs(86400), // Next day
        tasks: scheduled_tasks,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::SystemInfo;
    use std::time::{Duration, UNIX_EPOCH};

    fn make_system_info(cpu: f64, memory: f64, disk: f64) -> SystemInfo {
        SystemInfo {
            timestamp: UNIX_EPOCH + Duration::from_secs(1000),
            cpu_usage: cpu,
            memory_usage: memory,
            disk_usage: disk,
        }
    }

    #[test]
    fn test_monitor_capacity_usage_empty() {
        let result = monitor_capacity_usage("mypool/dataset", &[]);
        assert!(result.is_ok());
        let report = result.unwrap();
        assert_eq!(report.dataset, "mypool/dataset");
        assert_eq!(report.current_usage, 0);
    }

    #[test]
    fn test_monitor_capacity_usage_with_data() {
        let data = [make_system_info(50.0, 60.0, 25.0)];
        let result = monitor_capacity_usage("pool/ds", &data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().current_usage, 25);
    }

    #[test]
    fn test_detect_performance_bottlenecks_empty() {
        let result = detect_performance_bottlenecks(&[]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().bottleneck_type, "none");
    }

    #[test]
    fn test_detect_performance_bottlenecks_high_cpu() {
        let data = [make_system_info(90.0, 50.0, 50.0)];
        let result = detect_performance_bottlenecks(&data);
        assert!(result.is_ok());
        let report = result.unwrap();
        assert!(report.recommendations.iter().any(|r| r.contains("CPU")));
    }

    #[test]
    fn test_generate_maintenance_schedule() {
        let data = [make_system_info(50.0, 50.0, 50.0)];
        let result = generate_maintenance_schedule("mypool", &data);
        assert!(result.is_ok());
        let sched = result.unwrap();
        assert_eq!(sched.dataset, "mypool");
    }
}
