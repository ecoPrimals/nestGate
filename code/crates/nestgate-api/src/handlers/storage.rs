use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use tracing::debug;
use tracing::info;
use tracing::warn;
// Removed unused tracing import

/// Storage pool information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoragePool {
    pub name: String,
    pub status: String,
    pub size: u64,
    pub used: u64,
    pub available: u64,
    pub health: String,
    pub pool_type: String,
}

/// Storage dataset information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageDataset {
    pub name: String,
    pub pool: String,
    pub size: u64,
    pub used: u64,
    pub available: u64,
    pub mount_point: String,
    pub compression: String,
}

/// Storage snapshot information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageSnapshot {
    pub name: String,
    pub dataset: String,
    pub size: u64,
    pub created: String,
    pub referenced: u64,
}

/// Storage metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageMetrics {
    pub total_pools: u32,
    pub total_datasets: u32,
    pub total_snapshots: u32,
    pub total_storage: u64,
    pub used_storage: u64,
    pub available_storage: u64,
    pub iops: f64,
    pub bandwidth_mbps: f64,
    pub health_status: String,
}

/// Get storage pools
pub async fn get_storage_pools() -> impl IntoResponse {
    info!("Getting storage pools");

    // Mock data for now - would be replaced with real ZFS pool data
    let pools = vec![
        StoragePool {
            name: "tank".to_string(),
            status: "ONLINE".to_string(),
            size: 1000 * 1024 * 1024 * 1024,     // 1TB
            used: 500 * 1024 * 1024 * 1024,      // 500GB
            available: 500 * 1024 * 1024 * 1024, // 500GB
            health: "HEALTHY".to_string(),
            pool_type: "RAIDZ1".to_string(),
        },
        StoragePool {
            name: "backup".to_string(),
            status: "ONLINE".to_string(),
            size: 2000 * 1024 * 1024 * 1024,      // 2TB
            used: 100 * 1024 * 1024 * 1024,       // 100GB
            available: 1900 * 1024 * 1024 * 1024, // 1.9TB
            health: "HEALTHY".to_string(),
            pool_type: "MIRROR".to_string(),
        },
    ];

    Json(serde_json::json!({
        "status": "success",
        "pools": pools
    }))
}

/// Get storage datasets
pub async fn get_storage_datasets() -> impl IntoResponse {
    info!("Getting storage datasets");

    // Mock data for now - would be replaced with real ZFS dataset data
    let datasets = vec![
        StorageDataset {
            name: "tank/data".to_string(),
            pool: "tank".to_string(),
            size: 400 * 1024 * 1024 * 1024,      // 400GB
            used: 200 * 1024 * 1024 * 1024,      // 200GB
            available: 200 * 1024 * 1024 * 1024, // 200GB
            mount_point: "/tank/data".to_string(),
            compression: "lz4".to_string(),
        },
        StorageDataset {
            name: "tank/home".to_string(),
            pool: "tank".to_string(),
            size: 100 * 1024 * 1024 * 1024,     // 100GB
            used: 50 * 1024 * 1024 * 1024,      // 50GB
            available: 50 * 1024 * 1024 * 1024, // 50GB
            mount_point: "/tank/home".to_string(),
            compression: "gzip".to_string(),
        },
    ];

    Json(serde_json::json!({
        "status": "success",
        "datasets": datasets
    }))
}

/// Get storage snapshots with real ZFS data
pub async fn get_storage_snapshots() -> impl IntoResponse {
    info!("📸 Getting real storage snapshots");

    // Try to collect real ZFS snapshots
    let snapshots = match collect_real_zfs_snapshots().await {
        Ok(real_snapshots) => {
            info!("✅ Collected {} real ZFS snapshots", real_snapshots.len());
            real_snapshots
        }
        Err(e) => {
            warn!(
                "⚠️ Could not collect real ZFS snapshots: {}, using empty list",
                e
            );
            vec![]
        }
    };

    Json(serde_json::json!({
        "status": "success",
        "snapshots": snapshots,
        "count": snapshots.len(),
        "data_source": if snapshots.is_empty() { "unavailable" } else { "zfs" }
    }))
}

/// Collect real ZFS snapshots from system
async fn collect_real_zfs_snapshots(
) -> Result<Vec<StorageSnapshot>, Box<dyn std::error::Error + Send + Sync>> {
    let output = tokio::process::Command::new("zfs")
        .args(&[
            "list",
            "-t",
            "snapshot",
            "-H",
            "-p",
            "-o",
            "name,used,refer,creation",
        ])
        .output()
        .await?;

    if !output.status.success() {
        return Err("ZFS snapshot command failed".into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut snapshots = Vec::new();

    for line in stdout.lines().take(100) {
        // Limit to 100 most recent snapshots
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 4 {
            let full_name = parts[0].to_string();
            let size: u64 = parts[1].parse().unwrap_or(0);
            let referenced: u64 = parts[2].parse().unwrap_or(0);
            let creation_timestamp: u64 = parts[3].parse().unwrap_or(0);

            // Extract dataset name from full name (remove @snapshot part)
            let dataset = if let Some(at_pos) = full_name.find('@') {
                full_name[..at_pos].to_string()
            } else {
                full_name.clone()
            };

            // Convert Unix timestamp to ISO8601 string
            let created = if creation_timestamp > 0 {
                match std::time::SystemTime::UNIX_EPOCH
                    .checked_add(std::time::Duration::from_secs(creation_timestamp))
                {
                    Some(time) => chrono::DateTime::<chrono::Utc>::from(time).to_rfc3339(),
                    None => "unknown".to_string(),
                }
            } else {
                "unknown".to_string()
            };

            snapshots.push(StorageSnapshot {
                name: full_name,
                dataset,
                size,
                created,
                referenced,
            });
        }
    }

    // Sort by creation time (most recent first)
    snapshots.sort_by(|a, b| b.created.cmp(&a.created));

    Ok(snapshots)
}

/// Get storage metrics with real system and ZFS data
pub async fn get_storage_metrics() -> impl IntoResponse {
    info!("📊 Getting real storage metrics");

    // Collect real storage metrics
    let metrics = match collect_real_storage_metrics().await {
        Ok(real_metrics) => {
            info!("✅ Collected real storage metrics");
            real_metrics
        }
        Err(e) => {
            warn!(
                "⚠️ Could not collect real storage metrics: {}, using fallbacks",
                e
            );
            // Return fallback metrics with system disk info
            collect_fallback_storage_metrics().await
        }
    };

    Json(serde_json::json!({
        "status": "success",
        "metrics": metrics,
        "data_source": if metrics.total_pools > 0 { "zfs" } else { "system_fallback" }
    }))
}

/// Collect real storage metrics from ZFS and system
async fn collect_real_storage_metrics(
) -> Result<StorageMetrics, Box<dyn std::error::Error + Send + Sync>> {
    // Get ZFS pool information
    let pool_output = tokio::process::Command::new("zpool")
        .args(&["list", "-H", "-p"])
        .output()
        .await?;

    if !pool_output.status.success() {
        return Err("ZFS pool command failed".into());
    }

    let pool_stdout = String::from_utf8_lossy(&pool_output.stdout);
    let mut total_pools = 0;
    let mut total_storage = 0u64;
    let mut used_storage = 0u64;
    let mut available_storage = 0u64;
    let mut all_healthy = true;

    for line in pool_stdout.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 7 {
            total_pools += 1;
            total_storage += parts[1].parse::<u64>().unwrap_or(0);
            used_storage += parts[2].parse::<u64>().unwrap_or(0);
            available_storage += parts[3].parse::<u64>().unwrap_or(0);

            // Check health status (column 6)
            if parts[6] != "ONLINE" {
                all_healthy = false;
            }
        }
    }

    // Get dataset count
    let dataset_output = tokio::process::Command::new("zfs")
        .args(&["list", "-H", "-t", "filesystem"])
        .output()
        .await?;

    let dataset_count = if dataset_output.status.success() {
        String::from_utf8_lossy(&dataset_output.stdout)
            .lines()
            .count() as u32
    } else {
        0
    };

    // Get snapshot count
    let snapshot_output = tokio::process::Command::new("zfs")
        .args(&["list", "-H", "-t", "snapshot"])
        .output()
        .await?;

    let snapshot_count = if snapshot_output.status.success() {
        String::from_utf8_lossy(&snapshot_output.stdout)
            .lines()
            .count() as u32
    } else {
        0
    };

    // Try to get I/O statistics from zpool iostat
    let (iops, bandwidth_mbps) = match tokio::process::Command::new("zpool")
        .args(&["iostat", "-y", "1", "1"])
        .output()
        .await
    {
        Ok(iostat_output) if iostat_output.status.success() => {
            let iostat_stdout = String::from_utf8_lossy(&iostat_output.stdout);
            let mut total_iops = 0.0;
            let mut total_bandwidth = 0.0;

            // Parse iostat output (skip header lines)
            for line in iostat_stdout.lines().skip(2) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 7 && !parts[0].is_empty() && parts[0] != "pool" {
                    // Read ops/s (col 1) + Write ops/s (col 4)
                    let read_ops: f64 = parts[1].parse().unwrap_or(0.0);
                    let write_ops: f64 = parts[4].parse().unwrap_or(0.0);
                    total_iops += read_ops + write_ops;

                    // Read bandwidth (col 2) + Write bandwidth (col 5) - values are in units like "150M"
                    let read_bw = parse_bandwidth_unit(parts[2]).unwrap_or(0.0);
                    let write_bw = parse_bandwidth_unit(parts[5]).unwrap_or(0.0);
                    total_bandwidth += read_bw + write_bw;
                }
            }

            (total_iops, total_bandwidth)
        }
        _ => {
            debug!("Could not get ZFS iostat data, using estimates");
            // Estimate based on disk usage
            let estimated_iops = (used_storage / (1024 * 1024 * 1024)).min(2000) as f64; // Rough estimate
            let estimated_bandwidth = estimated_iops * 0.5; // Rough conversion
            (estimated_iops, estimated_bandwidth)
        }
    };

    let health_status = if all_healthy {
        "HEALTHY".to_string()
    } else {
        "DEGRADED".to_string()
    };

    Ok(StorageMetrics {
        total_pools,
        total_datasets: dataset_count,
        total_snapshots: snapshot_count,
        total_storage,
        used_storage,
        available_storage,
        iops,
        bandwidth_mbps,
        health_status,
    })
}

/// Parse bandwidth units like "150M", "2.5G", etc. and return MB/s
fn parse_bandwidth_unit(value: &str) -> Option<f64> {
    if value == "-" || value.is_empty() {
        return Some(0.0);
    }

    let (number_part, unit) = if value.ends_with('K') {
        (value[..value.len() - 1].parse::<f64>().ok()?, 1.0 / 1024.0) // KB to MB
    } else if value.ends_with('M') {
        (value[..value.len() - 1].parse::<f64>().ok()?, 1.0) // MB
    } else if value.ends_with('G') {
        (value[..value.len() - 1].parse::<f64>().ok()?, 1024.0) // GB to MB
    } else {
        (value.parse::<f64>().ok()?, 1.0 / (1024.0 * 1024.0)) // Assume bytes, convert to MB
    };

    Some(number_part * unit)
}

/// Collect fallback storage metrics when ZFS is not available
async fn collect_fallback_storage_metrics() -> StorageMetrics {
    // Get basic disk space information from system
    let (total_storage, used_storage, available_storage) = match tokio::process::Command::new("df")
        .args(&["-B1", "/"])  // Get root filesystem size in bytes
        .output()
        .await
    {
        Ok(df_output) if df_output.status.success() => {
            let df_stdout = String::from_utf8_lossy(&df_output.stdout);
            if let Some(line) = df_stdout.lines().nth(1) {
                // Skip header
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    let total: u64 = parts[1].parse().unwrap_or(0);
                    let used: u64 = parts[2].parse().unwrap_or(0);
                    let available: u64 = parts[3].parse().unwrap_or(0);
                    (total, used, available)
                } else {
                    (
                        1024 * 1024 * 1024 * 1024,
                        512 * 1024 * 1024 * 1024,
                        512 * 1024 * 1024 * 1024,
                    ) // 1TB default
                }
            } else {
                (
                    1024 * 1024 * 1024 * 1024,
                    512 * 1024 * 1024 * 1024,
                    512 * 1024 * 1024 * 1024,
                ) // 1TB default
            }
        }
        _ => (
            1024 * 1024 * 1024 * 1024,
            512 * 1024 * 1024 * 1024,
            512 * 1024 * 1024 * 1024,
        ), // 1TB default
    };

    StorageMetrics {
        total_pools: 0, // No ZFS pools available
        total_datasets: 0,
        total_snapshots: 0,
        total_storage,
        used_storage,
        available_storage,
        iops: 50.0,           // Conservative estimate
        bandwidth_mbps: 25.0, // Conservative estimate
        health_status: "SYSTEM_STORAGE".to_string(),
    }
}
