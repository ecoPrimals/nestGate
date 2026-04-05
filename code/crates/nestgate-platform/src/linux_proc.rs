// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Linux system metrics via `/proc` and [`rustix::fs::statvfs`].
//!
//! Primary path on Linux (pure Rust + rustix). The optional `sysinfo` crate feature is only for
//! [`physical_cpu_count`] on non-Linux; metrics in `nestgate-observe` use `/proc` first, then
//! `sysinfo` only when the `sysinfo` feature is enabled (see that crate’s `Cargo.toml`).

use std::io;

/// Read a `MemTotal:`-style field from `/proc/meminfo` (value in kB).
#[cfg(target_os = "linux")]
fn meminfo_kb(key: &str) -> Option<u64> {
    let s = std::fs::read_to_string("/proc/meminfo").ok()?;
    for line in s.lines() {
        if line.starts_with(key) {
            return line.split_whitespace().nth(1)?.parse().ok();
        }
    }
    None
}

/// Available RAM in bytes (`MemAvailable:` with MemFree+Buffers+Cached fallback).
#[must_use]
pub fn available_memory_bytes() -> Option<u64> {
    #[cfg(target_os = "linux")]
    {
        let mut avail_kb = meminfo_kb("MemAvailable:").unwrap_or(0);
        if avail_kb == 0 {
            let free = meminfo_kb("MemFree:")?;
            let buffers = meminfo_kb("Buffers:").unwrap_or(0);
            let cached = meminfo_kb("Cached:").unwrap_or(0);
            avail_kb = free + buffers + cached;
        }
        Some(avail_kb * 1024)
    }
    #[cfg(not(target_os = "linux"))]
    {
        None
    }
}

/// Total RAM in bytes (`MemTotal:`).
#[must_use]
pub fn total_memory_bytes() -> Option<u64> {
    #[cfg(target_os = "linux")]
    {
        meminfo_kb("MemTotal:").map(|k| k * 1024)
    }
    #[cfg(not(target_os = "linux"))]
    {
        None
    }
}

/// Used RAM in bytes (total minus available).
#[must_use]
pub fn used_memory_bytes() -> Option<u64> {
    let t = total_memory_bytes()?;
    let a = available_memory_bytes()?;
    Some(t.saturating_sub(a))
}

/// Logical CPU count via [`std::thread::available_parallelism`] (ecoBin v3.0; replaces `num_cpus::get`).
#[must_use]
pub fn logical_cpu_count() -> usize {
    std::thread::available_parallelism()
        .map(std::num::NonZero::get)
        .unwrap_or(1)
}

#[cfg(target_os = "linux")]
fn physical_cpu_count_from_proc_cpuinfo() -> Option<usize> {
    use std::collections::HashSet;
    let data = std::fs::read_to_string("/proc/cpuinfo").ok()?;
    let mut pairs: HashSet<(u32, u32)> = HashSet::new();
    let mut physical_id = None::<u32>;
    let mut core_id = None::<u32>;

    for line in data.lines() {
        let line = line.trim();
        if line.starts_with("processor") {
            if let (Some(p), Some(c)) = (physical_id, core_id) {
                pairs.insert((p, c));
            }
            physical_id = None;
            core_id = None;
        } else if let Some(rest) = line.strip_prefix("physical id") {
            if rest.trim_start().starts_with(':') {
                physical_id = rest.split(':').nth(1).and_then(|s| s.trim().parse().ok());
            }
        } else if let Some(rest) = line.strip_prefix("core id")
            && rest.trim_start().starts_with(':')
        {
            core_id = rest.split(':').nth(1).and_then(|s| s.trim().parse().ok());
        }
    }
    if let (Some(p), Some(c)) = (physical_id, core_id) {
        pairs.insert((p, c));
    }
    if pairs.is_empty() {
        None
    } else {
        Some(pairs.len())
    }
}

/// Best-effort physical CPU core count: Linux `/proc/cpuinfo` first, then `sysinfo::System` (cross-platform fallback).
#[must_use]
pub fn physical_cpu_count() -> usize {
    #[cfg(target_os = "linux")]
    if let Some(n) = physical_cpu_count_from_proc_cpuinfo() {
        return n;
    }
    #[cfg(feature = "sysinfo")]
    {
        let sys = sysinfo::System::new_all();
        if let Some(n) = sys.physical_core_count() {
            return n;
        }
    }
    logical_cpu_count()
}

/// Memory usage as a percentage (0.0–100.0).
#[must_use]
pub fn memory_usage_percent() -> Option<f64> {
    let t = total_memory_bytes()?;
    let u = used_memory_bytes()?;
    if t == 0 {
        return None;
    }
    // Percentage with two decimal places via integer math (`u128` avoids overflow on `u * 10_000`).
    let scaled = (u128::from(u).saturating_mul(10_000)) / u128::from(t);
    let scaled = u32::try_from(scaled).unwrap_or(10_000);
    Some(f64::from(scaled) / 100.0)
}

/// Instantaneous global CPU busy percentage from the aggregate `cpu` line in `/proc/stat`.
#[must_use]
pub fn globalcpu_usage_percent_from_stat() -> Option<f64> {
    #[cfg(target_os = "linux")]
    {
        let content = std::fs::read_to_string("/proc/stat").ok()?;
        let line = content.lines().next()?;
        if !line.starts_with("cpu ") {
            return None;
        }
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 8 {
            return None;
        }
        let user: u64 = fields[1].parse().ok()?;
        let nice: u64 = fields[2].parse().ok()?;
        let system: u64 = fields[3].parse().ok()?;
        let idle: u64 = fields[4].parse().ok()?;
        let iowait: u64 = fields[5].parse().ok()?;
        let irq: u64 = fields[6].parse().ok()?;
        let softirq: u64 = fields[7].parse().ok()?;
        let total_active = user + nice + system + iowait + irq + softirq;
        let total = total_active + idle;
        if total == 0 {
            return None;
        }
        let scaled = (u128::from(total_active).saturating_mul(10_000)) / u128::from(total);
        let scaled = u32::try_from(scaled).unwrap_or(10_000);
        Some(f64::from(scaled) / 100.0)
    }
    #[cfg(not(target_os = "linux"))]
    {
        None
    }
}

/// Sum of received and transmitted bytes across non-loopback interfaces (`/proc/net/dev`).
#[must_use]
pub fn network_rx_tx_bytes_sum() -> Option<(u64, u64)> {
    #[cfg(target_os = "linux")]
    {
        let s = std::fs::read_to_string("/proc/net/dev").ok()?;
        let mut rx = 0u64;
        let mut tx = 0u64;
        for line in s.lines().skip(2) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 17 {
                continue;
            }
            if parts[0].starts_with("lo:") {
                continue;
            }
            rx += parts[1].parse::<u64>().unwrap_or(0);
            tx += parts[9].parse::<u64>().unwrap_or(0);
        }
        Some((rx, tx))
    }
    #[cfg(not(target_os = "linux"))]
    {
        None
    }
}

/// Rough disk activity proxy: number of non-empty lines in `/proc/diskstats` (≥ 1).
#[must_use]
pub fn diskstats_entry_count() -> Option<f64> {
    #[cfg(target_os = "linux")]
    {
        let s = std::fs::read_to_string("/proc/diskstats").ok()?;
        let n = s.lines().filter(|l| !l.trim().is_empty()).count().max(1);
        Some(f64::from(u32::try_from(n).unwrap_or(u32::MAX)))
    }
    #[cfg(not(target_os = "linux"))]
    {
        None
    }
}

/// Total and available bytes for a mount point via [`rustix::fs::statvfs`].
///
/// # Errors
///
/// Returns [`io::Error`] if `path` cannot be stated or `statvfs` fails (e.g. permission denied,
/// not a mount point, or I/O error).
#[cfg(target_os = "linux")]
pub fn statvfs_space(path: &std::path::Path) -> io::Result<(u64, u64)> {
    let v = rustix::fs::statvfs(path)?;
    let fr = v.f_frsize;
    let total = v.f_blocks.saturating_mul(fr);
    let avail = v.f_bavail.saturating_mul(fr);
    Ok((total, avail))
}

/// Total and available bytes for a mount point (non-Linux stub; see Linux build for behavior).
///
/// # Errors
///
/// Always returns [`io::ErrorKind::Unsupported`].
#[cfg(not(target_os = "linux"))]
pub fn statvfs_space(_path: &std::path::Path) -> io::Result<(u64, u64)> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "statvfs_space is only supported on Linux",
    ))
}

/// System uptime in whole seconds from `/proc/uptime` (first field).
#[must_use]
pub fn uptime_secs() -> Option<u64> {
    #[cfg(target_os = "linux")]
    {
        let s = std::fs::read_to_string("/proc/uptime").ok()?;
        let first = s.split_whitespace().next()?;
        let end = first.find('.').unwrap_or(first.len());
        first.get(..end)?.parse().ok()
    }
    #[cfg(not(target_os = "linux"))]
    {
        None
    }
}

/// Load averages (1, 5, 15 minutes) from `/proc/loadavg`.
#[must_use]
pub fn load_averages() -> Option<(f64, f64, f64)> {
    #[cfg(target_os = "linux")]
    {
        let s = std::fs::read_to_string("/proc/loadavg").ok()?;
        let mut it = s.split_whitespace();
        let one = it.next()?.parse().ok()?;
        let five = it.next()?.parse().ok()?;
        let fifteen = it.next()?.parse().ok()?;
        Some((one, five, fifteen))
    }
    #[cfg(not(target_os = "linux"))]
    {
        None
    }
}

/// First line of `/proc/version` (kernel build string), when available.
#[must_use]
pub fn kernel_version_line() -> Option<String> {
    #[cfg(target_os = "linux")]
    {
        std::fs::read_to_string("/proc/version")
            .ok()
            .and_then(|s| s.lines().next().map(str::to_owned))
    }
    #[cfg(not(target_os = "linux"))]
    {
        None
    }
}
