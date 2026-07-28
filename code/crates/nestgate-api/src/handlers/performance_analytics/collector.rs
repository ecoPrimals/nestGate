// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Collect performance metrics from `/proc` via `nestgate_platform::linux_proc`.

use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result, bail};
use nestgate_platform::linux_proc;
use tracing::debug;

/// Aggregate disk I/O counters used for latency and throughput metrics.
#[derive(Debug, Default, Clone, Copy)]
struct DiskIoTotals {
    read_operations: u64,
    write_operations: u64,
    read_time_ms: u64,
    write_time_ms: u64,
    read_bytes: u64,
    write_bytes: u64,
}

/// Collect current performance metrics keyed for analytics consumers.
///
/// # Errors
///
/// Returns an error when required platform metrics are unavailable or `/proc`
/// data cannot be parsed.
pub async fn collect_performance_metrics() -> Result<HashMap<String, f64>> {
    #[cfg(not(target_os = "linux"))]
    {
        let _ = ();
        bail!("performance metrics require a Linux `/proc` filesystem");
    }

    #[cfg(target_os = "linux")]
    {
        collect_linux_metrics().await
    }
}

#[cfg(target_os = "linux")]
async fn collect_linux_metrics() -> Result<HashMap<String, f64>> {
    debug!("Collecting performance analytics metrics from /proc");

    let mut metrics = HashMap::new();

    let cpu_usage = linux_proc::globalcpu_usage_percent_from_stat()
        .context("CPU usage unavailable from /proc/stat")?;
    metrics.insert("cpu_usage".to_owned(), cpu_usage);

    let memory_usage = linux_proc::memory_usage_percent()
        .context("memory usage unavailable from /proc/meminfo")?;
    metrics.insert("memory_usage".to_owned(), memory_usage);

    let disk_usage = root_disk_usage_percent()?;
    metrics.insert("disk_usage".to_owned(), disk_usage);

    if let Some((one, five, fifteen)) = linux_proc::load_averages() {
        metrics.insert("load_average_1m".to_owned(), one);
        metrics.insert("load_average_5m".to_owned(), five);
        metrics.insert("load_average_15m".to_owned(), fifteen);
    }

    if let Some(iowait_percent) = iowait_percent_from_stat()? {
        metrics.insert("iowait_percent".to_owned(), iowait_percent);
    }

    if let Some((rx, tx)) = linux_proc::network_rx_tx_bytes_sum() {
        metrics.insert("network_rx_bytes".to_owned(), rx as f64);
        metrics.insert("network_tx_bytes".to_owned(), tx as f64);
    }

    let disk_io = read_disk_io_totals().await?;
    metrics.insert(
        "disk_read_latency_ms".to_owned(),
        average_latency_ms(disk_io.read_time_ms, disk_io.read_operations),
    );
    metrics.insert(
        "disk_write_latency_ms".to_owned(),
        average_latency_ms(disk_io.write_time_ms, disk_io.write_operations),
    );
    metrics.insert("disk_read_bytes".to_owned(), disk_io.read_bytes as f64);
    metrics.insert("disk_write_bytes".to_owned(), disk_io.write_bytes as f64);
    metrics.insert(
        "disk_read_operations".to_owned(),
        disk_io.read_operations as f64,
    );
    metrics.insert(
        "disk_write_operations".to_owned(),
        disk_io.write_operations as f64,
    );

    Ok(metrics)
}

#[cfg(target_os = "linux")]
fn root_disk_usage_percent() -> Result<f64> {
    let (total, avail) = linux_proc::statvfs_space(Path::new("/"))
        .context("failed to stat root mount via statvfs")?;
    if total == 0 {
        bail!("root filesystem reports zero total capacity");
    }
    let used = total.saturating_sub(avail);
    let scaled = (u128::from(used).saturating_mul(10_000)) / u128::from(total);
    let scaled = u32::try_from(scaled).unwrap_or(10_000);
    Ok(f64::from(scaled) / 100.0)
}

#[cfg(target_os = "linux")]
fn iowait_percent_from_stat() -> Result<Option<f64>> {
    let content = std::fs::read_to_string("/proc/stat").context("read /proc/stat")?;
    let line = content
        .lines()
        .next()
        .context("missing aggregate cpu line in /proc/stat")?;
    if !line.starts_with("cpu ") {
        bail!("unexpected first line in /proc/stat");
    }
    let fields: Vec<&str> = line.split_whitespace().collect();
    if fields.len() < 6 {
        return Ok(None);
    }
    let user = parse_u64_field(fields[1], "cpu user")?;
    let nice = parse_u64_field(fields[2], "cpu nice")?;
    let system = parse_u64_field(fields[3], "cpu system")?;
    let idle = parse_u64_field(fields[4], "cpu idle")?;
    let iowait = parse_u64_field(fields[5], "cpu iowait")?;
    let total = user + nice + system + idle + iowait;
    if total == 0 {
        return Ok(None);
    }
    let scaled = (u128::from(iowait).saturating_mul(10_000)) / u128::from(total);
    let scaled = u32::try_from(scaled).unwrap_or(10_000);
    Ok(Some(f64::from(scaled) / 100.0))
}

#[cfg(target_os = "linux")]
async fn read_disk_io_totals() -> Result<DiskIoTotals> {
    let content = tokio::fs::read_to_string("/proc/diskstats")
        .await
        .context("read /proc/diskstats")?;

    let mut totals = DiskIoTotals::default();
    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 14 {
            continue;
        }
        let device_name = fields[2];
        if should_skip_disk_device(device_name) {
            continue;
        }

        totals.read_operations += parse_u64_field(fields[3], "read operations")?;
        let read_sectors = parse_u64_field(fields[5], "read sectors")?;
        totals.read_time_ms += parse_u64_field(fields[6], "read time ms")?;
        totals.write_operations += parse_u64_field(fields[7], "write operations")?;
        let write_sectors = parse_u64_field(fields[9], "write sectors")?;
        totals.write_time_ms += parse_u64_field(fields[10], "write time ms")?;
        totals.read_bytes += read_sectors.saturating_mul(512);
        totals.write_bytes += write_sectors.saturating_mul(512);
    }

    Ok(totals)
}

fn should_skip_disk_device(device_name: &str) -> bool {
    if device_name.starts_with("loop") || device_name.starts_with("ram") {
        return true;
    }
    if device_name.starts_with("nvme") {
        // Whole namespaces (`nvme0n1`) are included; partitions (`nvme0n1p2`) are skipped.
        return device_name.contains('p');
    }
    device_name
        .chars()
        .last()
        .is_some_and(|ch| ch.is_ascii_digit())
}

fn average_latency_ms(total_time_ms: u64, operations: u64) -> f64 {
    if operations == 0 {
        return 0.0;
    }
    (total_time_ms as f64) / (operations as f64)
}

fn parse_u64_field(field: &str, label: &str) -> Result<u64> {
    field
        .parse::<u64>()
        .with_context(|| format!("invalid {label} value '{field}'"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn average_latency_handles_zero_operations() {
        assert_eq!(average_latency_ms(100, 0), 0.0);
    }

    #[test]
    fn average_latency_computes_mean() {
        assert_eq!(average_latency_ms(250, 5), 50.0);
    }

    #[test]
    fn skip_partition_and_virtual_devices() {
        assert!(should_skip_disk_device("sda1"));
        assert!(should_skip_disk_device("nvme0n1p2"));
        assert!(should_skip_disk_device("loop0"));
        assert!(!should_skip_disk_device("sda"));
        assert!(!should_skip_disk_device("nvme0n1"));
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn collect_metrics_returns_expected_keys() {
        let metrics = collect_performance_metrics()
            .await
            .expect("metrics collection should succeed on Linux");
        assert!(metrics.contains_key("cpu_usage"));
        assert!(metrics.contains_key("memory_usage"));
        assert!(metrics.contains_key("disk_read_latency_ms"));
        assert!(metrics.contains_key("disk_write_latency_ms"));
    }
}
