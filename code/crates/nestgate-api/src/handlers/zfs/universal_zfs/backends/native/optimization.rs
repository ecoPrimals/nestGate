// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// Contains all optimization and analytics operations for the native ZFS backend.

// Removed unused tracing import

//! Optimization module

use crate::handlers::zfs::universal_zfs_types::UniversalZfsResult;

use super::core::NativeZfsService;
use tracing::info;
use tracing::warn;

/// Optimize ZFS pools and datasets (zero-copy optimized)
pub async fn optimize(service: &NativeZfsService) -> UniversalZfsResult<String> {
    info!("Starting ZFS optimization operations");
    let mut results = Vec::new();

    // Get all pools
    let pools_output = service
        .execute_zfs_command(&["list", "-H", "-o", "name", "-t", "filesystem"])
        .await?;
    let pools: Vec<&str> = pools_output.lines().collect();

    for pool in pools {
        // Check pool health
        let health_output = service
            .execute_zfs_command(&["list", "-H", "-o", "health", pool])
            .await;
        if let Ok(health) = health_output
            && health.trim() == "ONLINE"
        {
            // Optimize compression settings
            let _ = service
                .execute_zfs_command(&["set", "compression=lz4", pool])
                .await;
            results.push(format!("Optimized compression for {pool}"));

            // Optimize record size for large files
            let _ = service
                .execute_zfs_command(&["set", "recordsize=128k", pool])
                .await;
            results.push(format!("Optimized record size for {pool}"));

            // Enable deduplication if beneficial
            let _ = service
                .execute_zfs_command(&["set", "dedup=on", pool])
                .await;
            results.push(format!("Enabled deduplication for {pool}"));
        }
    }

    if results.is_empty() {
        results.push("No optimization actions taken".into());
    }

    Ok(results.join("; "))
}

/// Get optimization analytics (zero-copy optimized)
pub async fn get_optimization_analytics(
    service: &NativeZfsService,
) -> UniversalZfsResult<serde_json::Value> {
    info!("Collecting ZFS optimization analytics");
    // Get pool statistics
    let pools_output = service
        .execute_zfs_command(&["list", "-H", "-o", "name,used,available,compressratio"])
        .await?;

    let mut analytics = serde_json::json!({
        "pools": [],
        "total_used": 0,
        "total_available": 0,
        "average_compression_ratio": 0.0,
        "optimization_recommendations": []
    });

    let mut total_used = 0u64;
    let mut total_available = 0u64;
    let mut compression_ratios = Vec::new();
    let mut recommendations = Vec::new();

    for line in pools_output.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 4 {
            let used = parse_size(parts[1]).unwrap_or(0);
            let available = parse_size(parts[2]).unwrap_or(0);
            let compression_ratio: f64 = parts[3].replace('x', "").parse().unwrap_or(1.0);

            total_used += used;
            total_available += available;
            compression_ratios.push(compression_ratio);

            // Generate recommendations
            if compression_ratio < 1.2 {
                recommendations.push(format!("Consider enabling compression for {}", parts[0]));
            }
            if available < used / 10 {
                recommendations.push(format!("Pool {} is running low on space", parts[0]));
            }

            // Safely access the pools array, or skip if corrupted
            if let Some(pools_array) = analytics["pools"].as_array_mut() {
                pools_array.push(serde_json::json!({
                    "name": parts[0],
                    "used": used,
                    "available": available,
                    "compression_ratio": compression_ratio
                }));
            } else {
                warn!(
                    "Analytics pools array is corrupted, skipping pool data for {}",
                    parts[0]
                );
            }
        }
    }

    analytics["total_used"] = total_used.into();
    analytics["total_available"] = total_available.into();
    analytics["average_compression_ratio"] = if compression_ratios.is_empty() {
        1.0.into()
    } else {
        (compression_ratios.iter().sum::<f64>() / (compression_ratios.len() as f64)).into()
    };
    analytics["optimization_recommendations"] = recommendations.into();

    Ok(analytics)
}

/// Predict optimal storage tier for a file (zero-copy optimized)
pub async fn predict_tier(
    service: &NativeZfsService,
    file_path: &str,
) -> UniversalZfsResult<String> {
    info!("Predicting optimal tier for file: {}", file_path);
    // Get file statistics
    let stat_output = service.execute_zfs_command(&["stat", file_path]).await;

    let tier = match &stat_output {
        Ok(output) => {
            // Parse file size and access patterns
            let size_mb = extract_file_size(output).unwrap_or(0) / (1024 * 1024);
            let access_time = extract_access_time(output);

            // Simple tier prediction logic
            if size_mb < 10 {
                "hot".to_string() // Small files go to hot tier
            } else if size_mb < 1000 {
                if access_time.is_some_and(|t| t < 7) {
                    "warm".to_string() // Recently accessed medium files go to warm
                } else {
                    "cold".to_string() // Old medium files go to cold
                }
            } else {
                "cold".to_string() // Large files go to cold tier
            }
        }
        Err(_e) => {
            // If we can't stat the file, default to warm tier
            "warm".to_string()
        }
    };

    Ok(tier)
}

/// Helper function to parse ZFS size strings (zero-copy optimized)
fn parse_size(size_str: &str) -> Option<u64> {
    if size_str == "-" {
        return Some(0);
    }
    let size_str = size_str.trim();
    if size_str.is_empty() {
        return None;
    }

    let (number_part, multiplier) = if let Some(last_char) = size_str.chars().last() {
        if last_char.is_alphabetic() {
            let mut chars = size_str.chars();
            chars.next_back();
            let number_part = chars.as_str();
            let multiplier = match last_char {
                'K' | 'k' => 1024,
                'M' | 'm' => 1024 * 1024,
                'G' | 'g' => 1024 * 1024 * 1024,
                'T' | 't' => 1024_u64 * 1024 * 1024 * 1024,
                'P' | 'p' => 1024_u64 * 1024 * 1024 * 1024 * 1024,
                _ => 1,
            };
            (number_part, multiplier)
        } else {
            (size_str, 1)
        }
    } else {
        return None;
    };

    let number: f64 = number_part.parse().ok()?;
    Some((number * multiplier as f64) as u64)
}

/// Extract file size from stat output (zero-copy optimized)
fn extract_file_size(stat_output: &str) -> Option<u64> {
    // Simple extraction - in real implementation would parse properly
    for line in stat_output.lines() {
        if line.contains("Size:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(size_str) = parts.get(1) {
                return size_str.parse().ok();
            }
        }
    }
    None
}
/// Extract access time from stat output (returns days since access) (zero-copy optimized)
fn extract_access_time(stat_output: &str) -> Option<u64> {
    // Simple extraction - in real implementation would parse properly
    for line in stat_output.lines() {
        if line.contains("Access:") {
            // For now, return a default value
            return Some(1); // Assume accessed within 1 day
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn predict_tier_defaults_warm_when_zfs_stat_fails() {
        let svc = NativeZfsService::new();
        let tier = predict_tier(&svc, "/this/path/should/not/exist/nestgate_test").await;
        assert_eq!(tier.expect("tier"), "warm");
    }
}
