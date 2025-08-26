//
// Core capacity monitoring, performance bottleneck detection, and maintenance scheduling

use crate::error::CanonicalResult as Result;
use crate::types::{BottleneckReport, CapacityReport, MaintenanceSchedule, SystemInfo};
use tracing::debug;

/// Storage capacity monitoring with basic forecasting
pub async fn monitor_capacity_usage(
    dataset: &str,
    historical_data: &[SystemInfo],
) -> Result<CapacityReport> {
    debug!("Monitoring capacity usage for dataset: {}", dataset);

    // Basic capacity analysis based on historical data
    let current_usage = historical_data
        .last()
        .map(|info| info.used_space as f64 / info.total_space as f64 * 100.0)
        .unwrap_or(0.0);

    let growth_rate = if historical_data.len() > 1 {
        if let Some(recent) = historical_data.last() {
            let previous = &historical_data[historical_data.len() - 2];
            let time_diff = recent.timestamp - previous.timestamp;
            if time_diff > 0 {
                (recent.used_space as f64 - previous.used_space as f64) / time_diff as f64
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
        current_usage,
        growth_rate,
        projected_days_to_full: if growth_rate > 0.0 {
            Some(((100.0 - current_usage) / growth_rate) as u32)
        } else {
            None
        },
    })
}

/// Performance bottleneck detection using metrics analysis
pub async fn detect_performance_bottlenecks(
    performance_data: &[SystemInfo],
) -> Result<BottleneckReport> {
    debug!("Detecting performance bottlenecks from metrics");

    let mut bottlenecks = Vec::new();
    let mut recommendations = Vec::new();

    if let Some(latest) = performance_data.last() {
        // CPU bottleneck detection
        if latest._cpu_usage > 80.0 {
            bottlenecks.push("High CPU usage".to_string());
            recommendations.push("Consider CPU upgrade or workload optimization".to_string());
        }

        // Memory bottleneck detection
        if latest.memory_usage > 85.0 {
            bottlenecks.push("High memory usage".to_string());
            recommendations.push("Increase system memory or tune ARC settings".to_string());
        }

        // I/O bottleneck detection
        if latest.io_wait > 10.0 {
            bottlenecks.push("High I/O wait".to_string());
            recommendations.push("Consider faster storage or I/O optimization".to_string());
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
        bottlenecks,
        severity: severity.to_string(),
        recommendations,
    })
}

/// Generate maintenance schedule based on system metrics
pub async fn generate_maintenance_schedule(
    dataset: &str,
    health_data: &[SystemInfo],
) -> Result<MaintenanceSchedule> {
    debug!("Generating maintenance schedule for dataset: {}", dataset);

    let mut scheduled_tasks = Vec::new();
    let mut priority = "low";

    if let Some(latest) = health_data.last() {
        // Pool scrub scheduling
        if latest.last_scrub_days > 30 {
            scheduled_tasks.push("Schedule pool scrub".to_string());
            priority = "high";
        }

        // Snapshot cleanup scheduling
        if latest.snapshot_count > 100 {
            scheduled_tasks.push("Clean up old snapshots".to_string());
            if priority == "low" {
                priority = "medium";
            }
        }

        // Defragmentation scheduling
        if latest.fragmentation > 20.0 {
            scheduled_tasks.push("Defragment pool".to_string());
            if priority == "low" {
                priority = "medium";
            }
        }
    }

    Ok(MaintenanceSchedule {
        dataset: dataset.to_string(),
        estimated_duration: 60 * scheduled_tasks.len() as u64, // 1 hour per task
        scheduled_tasks,
        priority: priority.to_string(),
    })
}
