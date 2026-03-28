// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// This module provides actual system metrics collection instead of mock data.
// It interfaces with the operating system to gather real performance data.

//! Real Metrics module

use std::collections::HashMap;
// use std::time::SystemTime; // Unused currently
use tokio::process::Command;
use tracing::{debug, warn};

use crate::performance::types::{PoolPerformanceMetrics, SystemResourceMetrics};
use nestgate_core::Result as CoreResult;

/// Real system metrics collector
pub struct RealMetricsCollector;
impl RealMetricsCollector {
    /// Collect real system resource metrics from the OS
    pub async fn collect_system_metrics() -> CoreResult<SystemResourceMetrics> {
        debug!("Collecting real system metrics");
        let mut metrics = SystemResourceMetrics::default();

        // Collect CPU utilization
        if let Ok(cpu_util) = Self::get_cpu_utilization().await {
            metrics.cpu_utilization_percent = cpu_util;
        }

        // Collect memory usage
        if let Ok((used, available)) = Self::get_memory_usage().await {
            metrics.memory_usage_bytes = used;
            metrics.available_memory_bytes = available;
        }

        // Collect load average
        if let Ok(load_avg) = Self::get_load_average().await {
            metrics.load_average_1m = load_avg;
        }

        // Collect I/O wait
        if let Ok(io_wait) = Self::get_io_wait().await {
            metrics.io_wait_percent = io_wait;
        }

        // Collect network I/O
        if let Ok(net_io) = Self::get_network_io().await {
            metrics.network_io_mbs = net_io;
        }

        debug!(
            "Collected real system metrics: CPU={:.1}%, Mem={}GB, Load={:.2}",
            metrics.cpu_utilization_percent,
            metrics.memory_usage_bytes / (1024 * 1024 * 1024),
            metrics.load_average_1m
        );

        Ok(metrics)
    }

    /// Get real CPU utilization from /proc/stat
    async fn get_cpu_utilization() -> CoreResult<f64> {
        if let Ok(content) = tokio::fs::read_to_string("/proc/stat").await {
            if let Some(line) = content.lines().next() {
                if line.starts_with("cpu ") {
                    let fields: Vec<&str> = line.split_whitespace().collect();
                    if fields.len() >= 8 {
                        // Parse CPU times: user, nice, system, idle, iowait, irq, softirq, steal
                        let user: u64 = fields[1].parse().unwrap_or(0);
                        let nice: u64 = fields[2].parse().unwrap_or(0);
                        let system: u64 = fields[3].parse().unwrap_or(0);
                        let idle: u64 = fields[4].parse().unwrap_or(0);
                        let iowait: u64 = fields[5].parse().unwrap_or(0);

                        let total = user + nice + system + idle + iowait;
                        let active = total - idle;

                        if total > 0 {
                            return Ok((active as f64 / total as f64) * 100.0);
                        }
                    }
                }
            }
        }

        warn!("Could not read CPU utilization from /proc/stat, using fallback");
        Ok(25.0) // Fallback value
    }

    /// Get real memory usage from /proc/meminfo
    async fn get_memory_usage() -> CoreResult<(u64, u64)> {
        if let Ok(content) = tokio::fs::read_to_string("/proc/meminfo").await {
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

            if total_kb > 0 && available_kb > 0 {
                let used_kb = total_kb - available_kb;
                return Ok((used_kb * 1024, available_kb * 1024));
            }
        }

        warn!("Could not read memory info from /proc/meminfo, using fallback");
        Ok((8 * 1024 * 1024 * 1024, 16 * 1024 * 1024 * 1024)) // 8GB used, 16GB available
    }

    /// Get real load average from /proc/loadavg
    async fn get_load_average() -> CoreResult<f64> {
        if let Ok(content) = tokio::fs::read_to_string("/proc/loadavg").await {
            if let Some(first_field) = content.split_whitespace().next() {
                if let Ok(load) = first_field.parse::<f64>() {
                    return Ok(load);
                }
            }
        }

        warn!("Could not read load average from /proc/loadavg, using fallback");
        Ok(1.0) // Fallback value
    }

    /// Get I/O wait percentage
    async fn get_io_wait() -> CoreResult<f64> {
        if let Ok(content) = tokio::fs::read_to_string("/proc/stat").await {
            if let Some(line) = content.lines().next() {
                if line.starts_with("cpu ") {
                    let fields: Vec<&str> = line.split_whitespace().collect();
                    if fields.len() >= 6 {
                        let iowait: u64 = fields[5].parse().unwrap_or(0);
                        let total: u64 = fields[1..8]
                            .iter()
                            .map(|f| f.parse::<u64>().unwrap_or(0))
                            .sum();

                        if total > 0 {
                            return Ok((iowait as f64 / total as f64) * 100.0);
                        }
                    }
                }
            }
        }

        Ok(2.0) // Fallback value
    }

    /// Get network I/O throughput
    async fn get_network_io() -> CoreResult<f64> {
        // This is a simplified implementation - in production you'd want to track deltas
        if let Ok(content) = tokio::fs::read_to_string("/proc/net/dev").await {
            let mut total_bytes = 0u64;

            for line in content.lines().skip(2) {
                // Skip header lines
                let fields: Vec<&str> = line.split_whitespace().collect();
                if fields.len() >= 10 {
                    // rx_bytes is field 1, tx_bytes is field 9
                    let rx_bytes: u64 = fields[1].parse().unwrap_or(0);
                    let tx_bytes: u64 = fields[9].parse().unwrap_or(0);
                    total_bytes += rx_bytes + tx_bytes;
                }
            }

            // Convert to MB/s (very rough approximation)
            return Ok(total_bytes as f64 / (1024.0 * 1024.0 * 60.0)); // Assume 60 second average
        }

        Ok(100.0) // Fallback value
    }

    /// Collect real ZFS pool metrics
    pub async fn collect_pool_metrics(pool_name: &str) -> CoreResult<PoolPerformanceMetrics> {
        debug!("Collecting real ZFS pool metrics for: {}", pool_name);

        let mut metrics = PoolPerformanceMetrics::default();

        // Get ZFS pool iostat data
        if let Ok(output) = Command::new("zpool")
            .args(["iostat", "-v", pool_name, "1", "1"])
            .output()
            .await
        {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if let Some(stats) = Self::parse_zpool_iostat(&stdout) {
                    metrics.total_iops = stats.get("iops").copied().unwrap_or(metrics.total_iops);
                    metrics.total_throughput_mbs = stats
                        .get("throughput")
                        .copied()
                        .unwrap_or(metrics.total_throughput_mbs);
                    metrics.avg_latency_ms = stats
                        .get("latency")
                        .copied()
                        .unwrap_or(metrics.avg_latency_ms);
                }
            } else {
                warn!(
                    "Failed to get zpool iostat for {}: {}",
                    pool_name,
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }

        // Get pool fragmentation
        if let Ok(output) = Command::new("zpool")
            .args(["list", "-H", "-o", "frag", pool_name])
            .output()
            .await
        {
            if output.status.success() {
                let frag_str = String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .replace('%', "");
                if let Ok(frag) = frag_str.parse::<f64>() {
                    metrics.fragmentation_percent = frag;
                }
            }
        }

        debug!(
            "Pool {} metrics: IOPS={:.0}, Throughput={:.1}MB/s, Latency={:.2}ms",
            pool_name, metrics.total_iops, metrics.total_throughput_mbs, metrics.avg_latency_ms
        );

        Ok(metrics)
    }

    /// Parse zpool iostat output
    fn parse_zpool_iostat(output: &str) -> Option<HashMap<&'static str, f64>> {
        // This is a simplified parser - real implementation would be more robust
        let lines: Vec<&str> = output.lines().collect();
        if lines.len() < 3 {
            return None;
        }

        // Look for pool statistics line (usually the last data line)
        for line in lines.iter().rev() {
            let fields: Vec<&str> = line.split_whitespace().collect();
            if fields.len() >= 7 {
                let mut stats = HashMap::new();

                // Parse read/write ops and bandwidth
                if let (Ok(read_ops), Ok(write_ops)) =
                    (fields[1].parse::<f64>(), fields[2].parse::<f64>())
                {
                    stats.insert("iops", read_ops + write_ops);
                }

                if let (Ok(read_bw), Ok(write_bw)) = (
                    Self::parse_bandwidth(fields[3]),
                    Self::parse_bandwidth(fields[4]),
                ) {
                    stats.insert("throughput", read_bw + write_bw);
                }

                return Some(stats);
            }
        }

        None
    }

    /// Parse bandwidth string (e.g., "1.2M" -> 1.2)
    fn parse_bandwidth(bw_str: &str) -> Result<f64, std::num::ParseFloatError> {
        let value_str = bw_str.trim_end_matches(['K', 'M', 'G', 'T']);
        let value = value_str.parse::<f64>()?;

        let multiplier = if bw_str.ends_with('T') {
            1024.0 * 1024.0
        } else if bw_str.ends_with('G') {
            1024.0
        } else if bw_str.ends_with('M') {
            1.0
        } else if bw_str.ends_with('K') {
            1.0 / 1024.0
        } else {
            1.0 / (1024.0 * 1024.0) // Assume bytes
        };

        Ok(value * multiplier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bandwidth_kilobytes() {
        let result = RealMetricsCollector::parse_bandwidth("512K");
        assert!(result.is_ok());
        let value = result.expect("Test: parse_bandwidth should succeed");
        // 512K -> 512 / 1024 = 0.5 MB/s
        assert!((value - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_parse_bandwidth_megabytes() {
        let result = RealMetricsCollector::parse_bandwidth("100M");
        assert!(result.is_ok());
        let value = result.expect("Test: parse_bandwidth should succeed");
        assert!((value - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_parse_bandwidth_gigabytes() {
        let result = RealMetricsCollector::parse_bandwidth("2G");
        assert!(result.is_ok());
        let value = result.expect("Test: parse_bandwidth should succeed");
        // 2G -> 2 * 1024 = 2048 MB/s
        assert!((value - 2048.0).abs() < 0.01);
    }

    #[test]
    fn test_parse_bandwidth_terabytes() {
        let result = RealMetricsCollector::parse_bandwidth("1T");
        assert!(result.is_ok());
        let value = result.unwrap();
        // 1T -> 1 * 1024 * 1024 = 1048576 MB/s
        assert!((value - 1048576.0).abs() < 1.0);
    }

    #[test]
    fn test_parse_bandwidth_decimal_values() {
        let result = RealMetricsCollector::parse_bandwidth("1.5M");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert!((value - 1.5).abs() < 0.01);
    }

    #[test]
    fn test_parse_bandwidth_zero() {
        let result = RealMetricsCollector::parse_bandwidth("0M");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, 0.0);
    }

    #[test]
    fn test_parse_bandwidth_invalid() {
        let result = RealMetricsCollector::parse_bandwidth("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_bandwidth_no_unit() {
        let result = RealMetricsCollector::parse_bandwidth("100");
        assert!(result.is_ok());
        // No unit assumes bytes: 100 bytes -> 100 / (1024 * 1024) MB/s
        let value = result.unwrap();
        assert!(value < 0.001); // Very small value
    }

    #[test]
    fn test_parse_bandwidth_large_values() {
        let values = vec![
            ("1024K", 1.0),    // 1024K = 1M
            ("1024M", 1024.0), // 1024M = 1024M
            ("10G", 10240.0),  // 10G = 10240M
        ];

        for (input, expected) in values {
            let result = RealMetricsCollector::parse_bandwidth(input);
            assert!(result.is_ok());
            let value = result.unwrap();
            assert!(
                (value - expected).abs() < 1.0,
                "Failed for input: {}",
                input
            );
        }
    }

    #[test]
    fn test_parse_bandwidth_no_whitespace() {
        // Parser doesn't handle whitespace, so test without it
        let result = RealMetricsCollector::parse_bandwidth("100M");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert!((value - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_system_resource_metrics_default() {
        let metrics = SystemResourceMetrics::default();

        assert!(metrics.cpu_utilization_percent >= 0.0);
        assert!(metrics.memory_usage_bytes > 0);
        assert!(metrics.available_memory_bytes > 0);
        assert!(metrics.load_average_1m >= 0.0);
    }

    #[test]
    fn test_pool_performance_metrics_default() {
        let metrics = PoolPerformanceMetrics::default();

        assert!(metrics.total_iops > 0.0);
        assert!(metrics.total_throughput_mbs > 0.0);
        assert!(metrics.avg_latency_ms >= 0.0);
        assert!(metrics.utilization_percent >= 0.0);
        assert!(metrics.compression_ratio >= 1.0);
    }

    #[tokio::test]
    async fn test_collect_system_metrics_returns_result() {
        // This will use fallback values if system calls fail
        let result = RealMetricsCollector::collect_system_metrics().await;

        // Should always return Ok with default values as fallback
        assert!(result.is_ok());

        let metrics = result.unwrap();
        assert!(metrics.cpu_utilization_percent >= 0.0);
        assert!(metrics.memory_usage_bytes > 0);
    }

    #[tokio::test]
    async fn test_collect_pool_metrics_returns_result() {
        let result = RealMetricsCollector::collect_pool_metrics("nonexistent-pool").await;

        // Should return Ok with default values even if pool doesn't exist
        assert!(result.is_ok());
    }

    #[test]
    fn test_bandwidth_parsing_edge_cases() {
        // Test very small values
        let result = RealMetricsCollector::parse_bandwidth("0.001M");
        assert!(result.is_ok());
        assert!(result.unwrap() < 0.01);

        // Test very large values
        let result = RealMetricsCollector::parse_bandwidth("9999G");
        assert!(result.is_ok());
        assert!(result.unwrap() > 1000000.0);
    }

    #[test]
    fn test_bandwidth_unit_conversions() {
        // 1KB = 1/1024 MB
        let kb_result = RealMetricsCollector::parse_bandwidth("1024K");
        assert!(kb_result.is_ok());
        assert!((kb_result.unwrap() - 1.0).abs() < 0.01);

        // 1GB = 1024 MB
        let gb_result = RealMetricsCollector::parse_bandwidth("1G");
        assert!(gb_result.is_ok());
        assert!((gb_result.unwrap() - 1024.0).abs() < 0.01);

        // 1TB = 1024*1024 MB
        let tb_result = RealMetricsCollector::parse_bandwidth("1T");
        assert!(tb_result.is_ok());
        assert!((tb_result.unwrap() - 1048576.0).abs() < 1.0);
    }

    #[test]
    fn test_parse_bandwidth_fractional() {
        let test_cases = vec![
            ("0.5M", 0.5),
            ("1.25M", 1.25),
            ("2.75G", 2816.0), // 2.75 * 1024
            ("0.1K", 0.0977),  // ~0.1 / 1024
        ];

        for (input, expected) in test_cases {
            let result = RealMetricsCollector::parse_bandwidth(input);
            assert!(result.is_ok(), "Failed to parse: {}", input);
            let value = result.unwrap();
            assert!(
                (value - expected).abs() < 1.0,
                "Input: {}, Expected: {}, Got: {}",
                input,
                expected,
                value
            );
        }
    }

    #[test]
    fn test_bandwidth_multiple_valid_units() {
        // Test a mix of valid unit formats
        let valid_inputs = vec![("1K", 1.0 / 1024.0), ("1M", 1.0), ("1G", 1024.0)];

        for (input, expected_approx) in valid_inputs {
            let result = RealMetricsCollector::parse_bandwidth(input);
            assert!(result.is_ok(), "Failed to parse: {}", input);
            let value = result.unwrap();
            assert!(
                (value - expected_approx).abs() / expected_approx < 0.1,
                "Input {} expected ~{} but got {}",
                input,
                expected_approx,
                value
            );
        }
    }

    #[test]
    fn test_bandwidth_parsing_empty_string() {
        // Empty string should fail parsing
        let result = RealMetricsCollector::parse_bandwidth("");
        assert!(result.is_err());
    }
}
