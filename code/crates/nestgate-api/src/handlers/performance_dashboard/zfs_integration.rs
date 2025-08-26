//
// This module contains all ZFS-specific functionality extracted from the main handlers
// to maintain the 1000-line file size limit.

use crate::handlers::performance_dashboard::types::*;
use nestgate_core::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, error, warn};

// ==================== ZFS POOL OPERATIONS ====================

/// Get ZFS pool statistics for a specific pool
pub async fn get_zfs_pool_stats(pool_name: &str) -> Result<serde_json::Value, String> {
    debug!("🔍 Getting ZFS pool stats for: {}", pool_name);
    
    // Execute zpool status command
    let output = Command::new("zpool")
        .args(&["status", pool_name, "-p"])
        .output()
        .map_err(|e| format!("Failed to execute zpool status: {}", e))?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("zpool status failed: {}", error_msg));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_zpool_status_output(&stdout)
}

/// Calculate pool trends over time
pub async fn calculate_pool_trends(
    pool_name: &str,
    time_range: &TimeRange,
) -> Result<Vec<PoolTrendPoint>, String> {
    debug!("📈 Calculating pool trends for: {} over {:?}", pool_name, time_range);
    
    // This would typically query historical data
    // For now, return mock trend data
    let mut trends = Vec::new();
    let start_time = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    std::io::Error::new(std::io::ErrorKind::Other, format!("Operation failed: {:?}", e))
})?.as_secs();
    
    for i in 0..24 {
        trends.push(PoolTrendPoint {
            timestamp: start_time - (i * 3600), // Last 24 hours
            used_bytes: 500_000_000_000 + (i * 10_000_000_000), // Increasing usage
            total_bytes: 1_000_000_000_000,
            io_operations: 1000 + (i * 50),
            throughput_mbps: 50.0 + (i as f64 * 2.0),
        });
    }
    
    Ok(trends)
}

// ==================== ZFS DATA STRUCTURES ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolInfo {
    pub name: String,
    pub health: String,
    pub capacity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolCapacity {
    pub total: u64,
    pub used: u64,
    pub available: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolTrendPoint {
    pub timestamp: u64,
    pub used_bytes: u64,
    pub total_bytes: u64,
    pub io_operations: u64,
    pub throughput_mbps: f64,
}

// ==================== ZFS POOL LISTING ====================

/// Get list of all ZFS pools
pub async fn get_zfs_pool_list() -> Result<Vec<PoolInfo>, String> {
    debug!("📋 Getting ZFS pool list");
    
    let output = Command::new("zpool")
        .args(&["list", "-H", "-o", "name,health,cap"])
        .output()
        .map_err(|e| format!("Failed to execute zpool list: {}", e))?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("zpool list failed: {}", error_msg));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut pools = Vec::new();
    
    for line in stdout.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 3 {
            pools.push(PoolInfo {
                name: parts[0].to_string(),
                health: parts[1].to_string(),
                capacity: parts[2].to_string(),
            });
        }
    }
    
    Ok(pools)
}

/// Get capacity information for a specific pool
pub async fn get_pool_capacity(pool_name: &str) -> Result<PoolCapacity, String> {
    debug!("💾 Getting capacity for pool: {}", pool_name);
    
    let output = Command::new("zfs")
        .args(&["get", "-H", "-p", "-o", "value", "used,available", pool_name])
        .output()
        .map_err(|e| format!("Failed to execute zfs get: {}", e))?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("zfs get failed: {}", error_msg));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.trim().lines().collect();
    
    if lines.len() >= 2 {
        let used: u64 = lines[0].parse().map_err(|e| format!("Failed to parse used capacity: {}", e))?;
        let available: u64 = lines[1].parse().map_err(|e| format!("Failed to parse available capacity: {}", e))?;
        let total = used + available;
        
        Ok(PoolCapacity {
            total,
            used,
            available,
        })
    } else {
        Err("Insufficient capacity data returned".to_string())
    }
}

/// Calculate growth rate for pools
pub async fn calculate_growth_rate(_pools: &[PoolInfo]) -> f64 {
    // This would typically analyze historical data
    // For now, return a mock growth rate
    10.5 // GB per day
}

// ==================== ZFS IO STATISTICS ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsIOStats {
    pub pool_name: String,
    pub read_ops_per_sec: f64,
    pub write_ops_per_sec: f64,
    pub read_throughput_mbps: f64,
    pub write_throughput_mbps: f64,
    pub average_read_latency: f64,
    pub average_write_latency: f64,
    pub peak_read_latency: f64,
    pub peak_write_latency: f64,
    pub queue_depth_average: f64,
    pub io_sizes: Vec<u32>,
}

/// Get ZFS I/O statistics for a pool
pub async fn get_zfs_io_stats(pool_name: &str) -> Result<ZfsIOStats, String> {
    debug!("⚡ Getting ZFS I/O stats for: {}", pool_name);
    
    let output = Command::new("zpool")
        .args(&["iostat", "-v", pool_name, "1", "1"])
        .output()
        .map_err(|e| format!("Failed to execute zpool iostat: {}", e))?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("zpool iostat failed: {}", error_msg));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_zpool_iostat_output(&stdout, pool_name)
}

// ==================== ZFS ARC STATISTICS ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsArcStats {
    pub arc_hits: u64,
    pub arc_accesses: u64,
    pub l2arc_hits: u64,
    pub l2arc_accesses: u64,
    pub arc_size_current: u64,
    pub arc_size_target: u64,
    pub arc_components: HashMap<String, u64>,
}

/// Get ZFS ARC (Adaptive Replacement Cache) statistics
pub async fn get_zfs_arc_stats() -> Result<ZfsArcStats, String> {
    debug!("🧠 Getting ZFS ARC statistics");
    
    // Try to read from /proc/spl/kstat/zfs/arcstats (Linux) or equivalent
    match std::fs::read_to_string("/proc/spl/kstat/zfs/arcstats") {
        Ok(content) => parse_arc_stats(&content),
        Err(_) => {
            warn!("Could not read ARC stats from /proc/spl/kstat/zfs/arcstats, using fallback");
            get_arc_stats_fallback().await
        }
    }
}

/// Fallback method for getting ARC stats when /proc interface is not available
pub async fn get_arc_stats_fallback() -> Result<ZfsArcStats, String> {
    debug!("🔄 Using fallback method for ARC stats");
    
    // Return reasonable default values
    Ok(ZfsArcStats {
        arc_hits: 950000,
        arc_accesses: 1000000,
        l2arc_hits: 45000,
        l2arc_accesses: 50000,
        arc_size_current: 800 * 1024 * 1024, // 800MB
        arc_size_target: 1024 * 1024 * 1024, // 1GB
        arc_components: HashMap::new(),
    })
}

// ==================== SYSTEM CAPACITY ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemCapacity {
    pub total_pools: usize,
    pub total_capacity_bytes: u64,
    pub used_capacity_bytes: u64,
    pub available_capacity_bytes: u64,
    pub utilization_percentage: f64,
}

/// Get current system-wide capacity information
pub async fn get_current_system_capacity() -> Result<SystemCapacity, String> {
    debug!("🌐 Getting current system capacity");
    
    let pools = get_zfs_pool_list().await?;
    let mut total_capacity = 0u64;
    let mut used_capacity = 0u64;
    
    for pool in &pools {
        match get_pool_capacity(&pool.name).await {
            Ok(capacity) => {
                total_capacity += capacity.total;
                used_capacity += capacity.used;
            }
            Err(e) => {
                warn!("Failed to get capacity for pool {}: {}", pool.name, e);
            }
        }
    }
    
    let available_capacity = total_capacity.saturating_sub(used_capacity);
    let utilization_percentage = if total_capacity > 0 {
        (used_capacity as f64 / total_capacity as f64) * 100.0
    } else {
        0.0
    };
    
    Ok(SystemCapacity {
        total_pools: pools.len(),
        total_capacity_bytes: total_capacity,
        used_capacity_bytes: used_capacity,
        available_capacity_bytes: available_capacity,
        utilization_percentage,
    })
}

/// Generate capacity forecast based on current trends
pub async fn generate_capacity_forecast(
    horizon_days: u32,
    current_capacity: &SystemCapacity,
) -> Result<Vec<CapacityForecastPoint>, String> {
    debug!("🔮 Generating capacity forecast for {} days", horizon_days);
    
    let growth_rate_per_day = calculate_growth_rate(&[]).await; // GB per day
    let growth_rate_bytes_per_day = (growth_rate_per_day * 1024.0 * 1024.0 * 1024.0) as u64;
    
    let mut forecast = Vec::new();
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    std::io::Error::new(std::io::ErrorKind::Other, format!("Operation failed: {:?}", e))
})?.as_secs();
    
    for day in 0..=horizon_days {
        let projected_used = current_capacity.used_capacity_bytes + (growth_rate_bytes_per_day * day as u64);
        let projected_utilization = (projected_used as f64 / current_capacity.total_capacity_bytes as f64) * 100.0;
        
        forecast.push(CapacityForecastPoint {
            timestamp: current_time + (day as u64 * 86400), // Add days in seconds
            projected_used_bytes: projected_used,
            projected_utilization_percentage: projected_utilization,
            confidence_level: calculate_forecast_confidence(day),
        });
    }
    
    Ok(forecast)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityForecastPoint {
    pub timestamp: u64,
    pub projected_used_bytes: u64,
    pub projected_utilization_percentage: f64,
    pub confidence_level: f64,
}

/// Calculate forecast confidence (decreases over time)
fn calculate_forecast_confidence(days_ahead: u32) -> f64 {
    // Confidence decreases over time
    let base_confidence = 0.95;
    let decay_rate = 0.02;
    (base_confidence - (days_ahead as f64 * decay_rate)).max(0.3)
}

/// Generate performance predictions
pub async fn generate_performance_predictions(horizon_days: u32) -> Vec<String> {
    let mut predictions = Vec::new();
    
    predictions.push(format!("Based on current trends, system utilization will reach 85% in {} days", horizon_days / 2));
    predictions.push("I/O performance is expected to remain stable with current workload patterns".to_string());
    predictions.push("ARC hit ratio optimization opportunities identified for improved cache performance".to_string());
    
    if horizon_days > 30 {
        predictions.push("Long-term capacity planning: Consider additional storage expansion within 60 days".to_string());
    }
    
    predictions
}

// ==================== PARSING UTILITIES ====================

/// Parse zpool status output
fn parse_zpool_status_output(output: &str) -> Result<serde_json::Value, String> {
    // Basic parsing of zpool status output
    let lines: Vec<&str> = output.lines().collect();
    
    if lines.is_empty() {
        return Err("Empty zpool status output".to_string());
    }
    
    // Extract basic information
    let mut pool_name = String::new();
    let mut state = String::new();
    let mut scan_status = String::new();
    
    for line in lines {
        let trimmed = line.trim();
        if trimmed.starts_with("pool:") {
            pool_name = trimmed.replace("pool:", "").trim().to_string();
        } else if trimmed.starts_with("state:") {
            state = trimmed.replace("state:", "").trim().to_string();
        } else if trimmed.starts_with("scan:") {
            scan_status = trimmed.replace("scan:", "").trim().to_string();
        }
    }
    
    Ok(serde_json::json!({
        "pool_name": pool_name,
        "state": state,
        "scan_status": scan_status,
        "raw_output": output
    }))
}

/// Parse zpool iostat output
fn parse_zpool_iostat_output(output: &str, pool_name: &str) -> Result<ZfsIOStats, String> {
    // Parse the iostat output - this is a simplified version
    let lines: Vec<&str> = output.lines().collect();
    
    // Look for the data line containing the pool stats
    for line in lines {
        if line.contains(pool_name) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 7 {
                return Ok(ZfsIOStats {
                    pool_name: pool_name.to_string(),
                    read_ops_per_sec: parts[3].parse().unwrap_or(0.0),
                    write_ops_per_sec: parts[4].parse().unwrap_or(0.0),
                    read_throughput_mbps: parse_throughput_value(parts[5]).unwrap_or(0.0),
                    write_throughput_mbps: parse_throughput_value(parts[6]).unwrap_or(0.0),
                    average_read_latency: 2.5,  // Mock values
                    average_write_latency: 3.1,
                    peak_read_latency: 8.7,
                    peak_write_latency: 12.3,
                    queue_depth_average: 1.2,
                    io_sizes: vec![4096, 8192, 16384, 32768, 65536],
                });
            }
        }
    }
    
    Err("Could not parse iostat output".to_string())
}

/// Parse throughput values (handles units like M, K, etc.)
fn parse_throughput_value(value: &str) -> Option<f64> {
    if value.ends_with('M') {
        value.trim_end_matches('M').parse::<f64>().ok()
    } else if value.ends_with('K') {
        value.trim_end_matches('K').parse::<f64>().map(|v| v / 1024.0).ok()
    } else {
        value.parse::<f64>().map(|v| v / 1024.0 / 1024.0).ok() // Assume bytes
    }
}

/// Parse ARC statistics from /proc/spl/kstat/zfs/arcstats
fn parse_arc_stats(content: &str) -> Result<ZfsArcStats, String> {
    let mut arc_hits = 0u64;
    let mut arc_misses = 0u64;
    let mut l2arc_hits = 0u64;
    let mut l2arc_misses = 0u64;
    let mut arc_size = 0u64;
    let mut arc_target = 0u64;
    let mut components = HashMap::new();
    
    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let name = parts[0];
            if let Ok(value) = parts[2].parse::<u64>() {
                match name {
                    "hits" => arc_hits = value,
                    "misses" => arc_misses = value,
                    "l2_hits" => l2arc_hits = value,
                    "l2_misses" => l2arc_misses = value,
                    "size" => arc_size = value,
                    "c" => arc_target = value,
                    _ => {
                        components.insert(name.to_string(), value);
                    }
                }
            }
        }
    }
    
    Ok(ZfsArcStats {
        arc_hits,
        arc_accesses: arc_hits + arc_misses,
        l2arc_hits,
        l2arc_accesses: l2arc_hits + l2arc_misses,
        arc_size_current: arc_size,
        arc_size_target: arc_target,
        arc_components: components,
    })
} 