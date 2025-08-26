//
// Pure data layer handlers for system monitoring and performance metrics.
// These handlers provide clean access to monitoring data without any
// authentication or user management overhead.

use axum::{
    extract::{Query, State},
    response::Json,
};
use serde::Deserialize;
use tracing::{debug, info};

use crate::rest::models::*;
use crate::rest::{ApiState, DataError, DataResponse};

// ============================================================================
// MONITORING DATA HANDLERS
// ============================================================================

/// Query parameters for historical metrics
#[derive(Debug, Deserialize)]
pub struct MetricsHistoryQuery {
    /// Start time (ISO 8601)
    pub start: Option<String>,
    /// End time (ISO 8601)
    pub end: Option<String>,
    /// Interval (e.g., "5m", "1h", "1d")
    pub interval: Option<String>,
    /// Specific metrics to include
    pub metrics: Option<Vec<String>>,
}

/// Get current system metrics
/// GET /api/v1/monitoring/metrics
pub async fn get_metrics(
    State(state): State<ApiState>,
) -> Result<Json<DataResponse<SystemMetrics>>, Json<DataError>> {
    debug!("Getting current system metrics");

    // Get current ZFS metrics from engines
    let engines = state.zfs_engines.read().await;
    let mut total_datasets = 0;
    let total_snapshots = 0;
    let mut total_used_bytes = 0;
    let mut total_available_bytes = 0;
    let mut compression_ratios = Vec::new();

    for (_name, engine) in engines.iter() {
        let stats = engine.stats().await;
        total_datasets += 1;
        // Use available stats instead of cow_stats
        total_used_bytes += stats.total_operations * 1024; // Estimate based on operations
        total_available_bytes += 1024 * 1024 * 1024; // 1GB per dataset (placeholder)

        // Access compression stats directly (not optional in ModernZfsStats)
        {
            let compression_stats = &stats.compression_stats;
            compression_ratios.push(compression_stats.compression_ratio());
        }
    }

    // Calculate overall compression ratio
    let overall_compression_ratio = if compression_ratios.is_empty() {
        1.0
    } else {
        compression_ratios.iter().sum::<f64>() / compression_ratios.len() as f64
    };

    // Generate realistic system metrics (in production, would read from actual system)
    let metrics = SystemMetrics {
        timestamp: chrono::Utc::now(),
        cpu_usage_percent: generate_realistic_cpu_usage(),
        memory_usage_percent: generate_realistic_memory_usage(),
        disk_io: DiskIoMetrics {
            read_mbps: generate_realistic_disk_read(),
            write_mbps: generate_realistic_disk_write(),
            read_iops: generate_realistic_read_iops(),
            write_iops: generate_realistic_write_iops(),
            avg_queue_depth: generate_realistic_queue_depth(),
        },
        network_io: NetworkIoMetrics {
            rx_bytes_per_sec: generate_realistic_network_rx(),
            tx_bytes_per_sec: generate_realistic_network_tx(),
            rx_packets_per_sec: generate_realistic_network_rx_packets(),
            tx_packets_per_sec: generate_realistic_network_tx_packets(),
        },
        zfs_metrics: ZfsMetrics {
            total_datasets,
            total_snapshots: total_snapshots.try_into().unwrap_or(0),
            total_used_bytes,
            total_available_bytes,
            overall_compression_ratio,
            cache_hit_ratio: generate_realistic_cache_hit_ratio(),
        },
    };

    info!("Retrieved current system metrics");
    Ok(Json(DataResponse::new(metrics)))
}

/// Get historical metrics data
/// GET /api/v1/monitoring/metrics/history
pub async fn get_metrics_history(
    State(state): State<ApiState>,
    Query(query): Query<MetricsHistoryQuery>,
) -> Result<Json<DataResponse<Vec<SystemMetrics>>>, Json<DataError>> {
    debug!("Getting historical metrics data: {:?}", query);

    // Parse time range (simplified for demo)
    let start_time = query
        .start
        .as_deref()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .unwrap_or_else(|| chrono::Utc::now() - chrono::Duration::hours(1));

    let end_time = query
        .end
        .as_deref()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .unwrap_or_else(chrono::Utc::now);

    // Parse interval
    let interval_minutes = match query.interval.as_deref() {
        Some("1m") => 1,
        Some("5m") => 5,
        Some("15m") => 15,
        Some("1h") => 60,
        Some("1d") => 1440,
        _ => 5, // Default to 5 minutes
    };

    // Generate historical data points
    let mut metrics_history = Vec::new();
    let mut current_time = start_time;

    while current_time <= end_time {
        // Get current ZFS state (simplified)
        let engines = state.zfs_engines.read().await;
        let total_datasets = engines.len() as u32;

        let metrics = SystemMetrics {
            timestamp: current_time,
            cpu_usage_percent: generate_historical_cpu_usage(current_time),
            memory_usage_percent: generate_historical_memory_usage(current_time),
            disk_io: DiskIoMetrics {
                read_mbps: generate_historical_disk_read(current_time),
                write_mbps: generate_historical_disk_write(current_time),
                read_iops: generate_historical_read_iops(current_time),
                write_iops: generate_historical_write_iops(current_time),
                avg_queue_depth: generate_historical_queue_depth(current_time),
            },
            network_io: NetworkIoMetrics {
                rx_bytes_per_sec: generate_historical_network_rx(current_time),
                tx_bytes_per_sec: generate_historical_network_tx(current_time),
                rx_packets_per_sec: generate_historical_network_rx_packets(current_time),
                tx_packets_per_sec: generate_historical_network_tx_packets(current_time),
            },
            zfs_metrics: ZfsMetrics {
                total_datasets,
                total_snapshots: total_datasets * 3, // Assume 3 snapshots per dataset
                total_used_bytes: (total_datasets as u64) * 2 * 1024 * 1024 * 1024, // 2GB per dataset
                total_available_bytes: (total_datasets as u64) * 10 * 1024 * 1024 * 1024, // 10GB per dataset
                overall_compression_ratio: 0.68,
                cache_hit_ratio: generate_historical_cache_hit_ratio(current_time),
            },
        };

        metrics_history.push(metrics);
        current_time += chrono::Duration::minutes(interval_minutes);
    }

    info!(
        "Retrieved {} historical metrics data points",
        metrics_history.len()
    );
    Ok(Json(DataResponse::new(metrics_history)))
}

/// Get active alerts
/// GET /api/v1/monitoring/alerts
pub async fn get_alerts(
    State(state): State<ApiState>,
) -> Result<Json<DataResponse<Vec<Alert>>>, Json<DataError>> {
    debug!("Getting active alerts");

    let mut alerts = Vec::new();

    // Check ZFS-related alerts
    let engines = state.zfs_engines.read().await;
    let total_datasets = engines.len();

    // Generate sample alerts based on current state
    if total_datasets > 10 {
        alerts.push(Alert {
            id: "alert_001".to_string(),
            name: "High Dataset Count".to_string(),
            description: format!(
                "System has {} datasets, consider consolidation",
                total_datasets
            ),
            severity: AlertSeverity::Warning,
            status: AlertStatus::Active,
            triggered_at: chrono::Utc::now() - chrono::Duration::minutes(15),
            conditions: vec![AlertCondition {
                metric: "total_datasets".to_string(),
                operator: ComparisonOperator::GreaterThan,
                threshold: 10.0,
                current_value: total_datasets as f64,
            }],
            suggested_actions: vec![
                "Review dataset organization".to_string(),
                "Consider consolidating similar datasets".to_string(),
                "Implement dataset archival policy".to_string(),
            ],
        });
    }

    // Simulate system resource alerts
    let current_cpu = generate_realistic_cpu_usage();
    if current_cpu > 80.0 {
        alerts.push(Alert {
            id: "alert_002".to_string(),
            name: "High CPU Usage".to_string(),
            description: format!("CPU usage is at {:.1}%", current_cpu),
            severity: if current_cpu > 95.0 {
                AlertSeverity::Critical
            } else {
                AlertSeverity::Warning
            },
            status: AlertStatus::Active,
            triggered_at: chrono::Utc::now() - chrono::Duration::minutes(5),
            conditions: vec![AlertCondition {
                metric: "cpu_usage_percent".to_string(),
                operator: ComparisonOperator::GreaterThan,
                threshold: 80.0,
                current_value: current_cpu,
            }],
            suggested_actions: vec![
                "Check for resource-intensive processes".to_string(),
                "Consider scaling resources".to_string(),
                "Review system performance".to_string(),
            ],
        });
    }

    let current_memory = generate_realistic_memory_usage();
    if current_memory > 85.0 {
        alerts.push(Alert {
            id: "alert_003".to_string(),
            name: "High Memory Usage".to_string(),
            description: format!("Memory usage is at {:.1}%", current_memory),
            severity: if current_memory > 95.0 {
                AlertSeverity::Critical
            } else {
                AlertSeverity::Warning
            },
            status: AlertStatus::Active,
            triggered_at: chrono::Utc::now() - chrono::Duration::minutes(8),
            conditions: vec![AlertCondition {
                metric: "memory_usage_percent".to_string(),
                operator: ComparisonOperator::GreaterThan,
                threshold: 85.0,
                current_value: current_memory,
            }],
            suggested_actions: vec![
                "Clear memory caches".to_string(),
                "Restart memory-intensive services".to_string(),
                "Add more RAM if consistently high".to_string(),
            ],
        });
    }

    // Storage space alert
    let total_used = (total_datasets as u64) * 2 * 1024 * 1024 * 1024; // 2GB per dataset
    let total_available = (total_datasets as u64) * 10 * 1024 * 1024 * 1024; // 10GB per dataset
    let usage_percent = if total_available > 0 {
        (total_used as f64 / total_available as f64) * 100.0
    } else {
        0.0
    };

    if usage_percent > 80.0 {
        alerts.push(Alert {
            id: "alert_004".to_string(),
            name: "High Storage Usage".to_string(),
            description: format!("Storage usage is at {:.1}%", usage_percent),
            severity: if usage_percent > 95.0 {
                AlertSeverity::Critical
            } else {
                AlertSeverity::Warning
            },
            status: AlertStatus::Active,
            triggered_at: chrono::Utc::now() - chrono::Duration::minutes(12),
            conditions: vec![AlertCondition {
                metric: "storage_usage_percent".to_string(),
                operator: ComparisonOperator::GreaterThan,
                threshold: 80.0,
                current_value: usage_percent,
            }],
            suggested_actions: vec![
                "Clean up old snapshots".to_string(),
                "Enable compression on datasets".to_string(),
                "Add additional storage capacity".to_string(),
                "Archive old data".to_string(),
            ],
        });
    }

    // Add a resolved alert as an example
    if !alerts.is_empty() {
        alerts.push(Alert {
            id: "alert_005".to_string(),
            name: "Network Connectivity".to_string(),
            description: "Network connectivity was temporarily degraded".to_string(),
            severity: AlertSeverity::Warning,
            status: AlertStatus::Resolved,
            triggered_at: chrono::Utc::now() - chrono::Duration::hours(2),
            conditions: vec![AlertCondition {
                metric: "network_latency_ms".to_string(),
                operator: ComparisonOperator::GreaterThan,
                threshold: 100.0,
                current_value: 15.0, // Now back to normal
            }],
            suggested_actions: vec![
                "Monitor network performance".to_string(),
                "Check network infrastructure".to_string(),
            ],
        });
    }

    info!("Retrieved {} alerts", alerts.len());
    Ok(Json(DataResponse::new(alerts)))
}

// ============================================================================
// METRIC GENERATION HELPERS (for realistic demo data)
// ============================================================================

fn generate_realistic_cpu_usage() -> f64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    chrono::Utc::now().timestamp().hash(&mut hasher);
    let seed = hasher.finish();

    // Generate CPU usage between 10-90% with some variation
    let base = 25.0;
    let variation = ((seed % 100) as f64) * 0.4; // 0-40% variation
    (base + variation).min(90.0)
}

fn generate_realistic_memory_usage() -> f64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    (chrono::Utc::now().timestamp() + 1).hash(&mut hasher);
    let seed = hasher.finish();

    let base = 45.0;
    let variation = ((seed % 100) as f64) * 0.3;
    (base + variation).min(95.0)
}

fn generate_realistic_disk_read() -> f64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    (chrono::Utc::now().timestamp() + 2).hash(&mut hasher);
    let seed = hasher.finish();

    let base = 100.0;
    let variation = ((seed % 100) as f64) * 2.0;
    base + variation
}

fn generate_realistic_disk_write() -> f64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    (chrono::Utc::now().timestamp() + 3).hash(&mut hasher);
    let seed = hasher.finish();

    let base = 80.0;
    let variation = ((seed % 100) as f64) * 1.5;
    base + variation
}

fn generate_realistic_read_iops() -> u64 {
    (generate_realistic_disk_read() * 100.0) as u64
}

fn generate_realistic_write_iops() -> u64 {
    (generate_realistic_disk_write() * 100.0) as u64
}

fn generate_realistic_queue_depth() -> f64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    (chrono::Utc::now().timestamp() + 4).hash(&mut hasher);
    let seed = hasher.finish();

    let base = 2.0;
    let variation = ((seed % 100) as f64) * 0.03;
    base + variation
}

fn generate_realistic_network_rx() -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    (chrono::Utc::now().timestamp() + 5).hash(&mut hasher);
    let seed = hasher.finish();

    let base = 1024 * 1024; // 1MB/s base
    let variation = (seed % (512 * 1024)) as u64;
    base + variation
}

fn generate_realistic_network_tx() -> u64 {
    generate_realistic_network_rx() / 2 // Usually less TX than RX
}

fn generate_realistic_network_rx_packets() -> u64 {
    generate_realistic_network_rx() / 1400 // Assume ~1400 bytes per packet
}

fn generate_realistic_network_tx_packets() -> u64 {
    generate_realistic_network_tx() / 1400
}

fn generate_realistic_cache_hit_ratio() -> f64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    (chrono::Utc::now().timestamp() + 6).hash(&mut hasher);
    let seed = hasher.finish();

    let base = 0.85; // 85% base hit ratio
    let variation = ((seed % 100) as f64) * 0.001; // Small variation
    (base + variation).min(0.99)
}

// Historical data generators (with time-based variation)
fn generate_historical_cpu_usage(time: chrono::DateTime<chrono::Utc>) -> f64 {
    let base = 25.0;
    let time_factor = (time.timestamp() as f64 * 0.001).sin() * 10.0;
    (base + time_factor).max(5.0).min(90.0)
}

fn generate_historical_memory_usage(time: chrono::DateTime<chrono::Utc>) -> f64 {
    let base = 45.0;
    let time_factor = (time.timestamp() as f64 * 0.0008).sin() * 8.0;
    (base + time_factor).max(20.0).min(85.0)
}

fn generate_historical_disk_read(time: chrono::DateTime<chrono::Utc>) -> f64 {
    let base = 100.0;
    let time_factor = (time.timestamp() as f64 * 0.002).sin() * 50.0;
    (base + time_factor).max(20.0)
}

fn generate_historical_disk_write(time: chrono::DateTime<chrono::Utc>) -> f64 {
    let base = 80.0;
    let time_factor = (time.timestamp() as f64 * 0.0015).sin() * 40.0;
    (base + time_factor).max(15.0)
}

fn generate_historical_read_iops(time: chrono::DateTime<chrono::Utc>) -> u64 {
    (generate_historical_disk_read(time) * 100.0) as u64
}

fn generate_historical_write_iops(time: chrono::DateTime<chrono::Utc>) -> u64 {
    (generate_historical_disk_write(time) * 100.0) as u64
}

fn generate_historical_queue_depth(time: chrono::DateTime<chrono::Utc>) -> f64 {
    let base = 2.0;
    let time_factor = (time.timestamp() as f64 * 0.003).sin() * 0.5;
    (base + time_factor).max(0.5).min(10.0)
}

fn generate_historical_network_rx(time: chrono::DateTime<chrono::Utc>) -> u64 {
    let base = 1024 * 1024;
    let time_factor = (time.timestamp() as f64 * 0.001).sin() * 512.0 * 1024.0;
    (base as f64 + time_factor).max(0.0) as u64
}

fn generate_historical_network_tx(time: chrono::DateTime<chrono::Utc>) -> u64 {
    generate_historical_network_rx(time) / 2
}

fn generate_historical_network_rx_packets(time: chrono::DateTime<chrono::Utc>) -> u64 {
    generate_historical_network_rx(time) / 1400
}

fn generate_historical_network_tx_packets(time: chrono::DateTime<chrono::Utc>) -> u64 {
    generate_historical_network_tx(time) / 1400
}

fn generate_historical_cache_hit_ratio(time: chrono::DateTime<chrono::Utc>) -> f64 {
    let base = 0.85;
    let time_factor = (time.timestamp() as f64 * 0.0005).sin() * 0.05;
    (base + time_factor).max(0.60).min(0.99)
}
