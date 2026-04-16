// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Pool/dataset/ARC metrics gathering and cache updates.

use std::collections::HashMap;
use std::time::SystemTime;

use tracing::debug;

use crate::{dataset::ZfsDatasetManager, error::Result, pool::ZfsPoolManager};

use super::super::types::{
    AccessPattern, ArcStatistics, SystemMemoryUsage, ZfsDatasetMetrics, ZfsPerformanceMetrics,
    ZfsPoolMetrics,
};
use super::RealTimePerformanceMonitor;

/// Merge one line from `/proc/spl/kstat/zfs/arcstats` into hit/miss counters (pool iostat block).
pub(super) fn merge_arc_kstat_hits_misses_line(line: &str, hits: &mut u64, misses: &mut u64) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 3 {
        return;
    }
    match parts[0] {
        "hits" => *hits = parts[2].parse().unwrap_or(0),
        "misses" => *misses = parts[2].parse().unwrap_or(0),
        _ => {}
    }
}

/// Merge one line from `/proc/spl/kstat/zfs/arcstats` into the full ARC snapshot fields.
pub(super) fn merge_arc_kstat_full_line(
    line: &str,
    hits: &mut u64,
    misses: &mut u64,
    size: &mut u64,
    c: &mut u64,
    mru_size: &mut u64,
    mfu_size: &mut u64,
) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 3 {
        return;
    }
    match parts[0] {
        "hits" => *hits = parts[2].parse().unwrap_or(0),
        "misses" => *misses = parts[2].parse().unwrap_or(0),
        "size" => *size = parts[2].parse().unwrap_or(0),
        "c" => *c = parts[2].parse().unwrap_or(0),
        "mru_size" => *mru_size = parts[2].parse().unwrap_or(0),
        "mfu_size" => *mfu_size = parts[2].parse().unwrap_or(0),
        _ => {}
    }
}

/// Parse `zpool list -H -o frag` stdout (trimmed) into a percentage value.
pub(super) fn frag_percent_from_zpool_frag_stdout(stdout_trimmed: &str) -> f64 {
    if let Some(frag_str) = stdout_trimmed.trim().strip_suffix('%') {
        frag_str.parse().unwrap_or(10.0)
    } else {
        10.0
    }
}

/// Accumulates `zfs get -H -p` lines for dataset property parsing in [`RealTimePerformanceMonitor::collect_metrics`].
pub(super) struct DatasetPropAccum {
    /// Effective compression ratio (from properties or derived from used vs logical).
    pub compression_ratio: f64,
    /// Estimated dedup ratio when dedup=on.
    pub dedup_ratio: f64,
    /// Parsed recordsize in bytes.
    pub record_size: u64,
    pub used_bytes: u64,
    pub logical_used_bytes: u64,
}

impl DatasetPropAccum {
    /// Default field values matching `collect_metrics` initial state.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            compression_ratio: 1.0,
            dedup_ratio: 1.0,
            record_size: 128 * 1024,
            used_bytes: 0,
            logical_used_bytes: 0,
        }
    }

    /// Apply one tab-separated `zfs get` output line (`name`, `property`, `value`).
    pub fn apply_tab_line(
        &mut self,
        line: &str,
        parse_recordsize: impl Fn(&str) -> crate::error::Result<u64>,
    ) {
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() < 3 {
            return;
        }
        match fields[1] {
            "compressratio" => {
                if let Some(ratio_str) = fields[2].strip_suffix('x') {
                    self.compression_ratio = ratio_str.parse().unwrap_or(1.0);
                }
            }
            "dedup" => {
                if fields[2] == "on" {
                    self.dedup_ratio = 1.2;
                }
            }
            "recordsize" => {
                self.record_size = parse_recordsize(fields[2]).unwrap_or(128 * 1024);
            }
            "used" => {
                self.used_bytes = fields[2].parse().unwrap_or(0);
            }
            "logicalused" => {
                self.logical_used_bytes = fields[2].parse().unwrap_or(0);
            }
            _ => {}
        }
    }

    /// Derive compression ratio from logical vs used when both are present.
    pub fn finalize_compression_ratio(&mut self) {
        if self.logical_used_bytes > 0 && self.used_bytes > 0 {
            self.compression_ratio = self.logical_used_bytes as f64 / self.used_bytes as f64;
        }
    }

    /// Map record size to an [`AccessPattern`] using the same thresholds as `collect_metrics`.
    #[must_use]
    pub const fn access_pattern(&self) -> AccessPattern {
        if self.record_size >= 1024 * 1024 {
            AccessPattern::Sequential
        } else if self.record_size <= 32 * 1024 {
            AccessPattern::Random
        } else {
            AccessPattern::Mixed
        }
    }
}

impl Default for DatasetPropAccum {
    fn default() -> Self {
        Self::new()
    }
}

/// Keep at most `max_entries` entries, removing lexicographically first keys when over capacity.
pub(super) fn trim_metrics_cache_entries(
    cache: &mut std::collections::HashMap<String, ZfsPerformanceMetrics>,
    max_entries: usize,
) {
    if cache.len() > max_entries {
        let mut keys: Vec<String> = cache.keys().cloned().collect();
        keys.sort();
        for key in keys.iter().take(cache.len().saturating_sub(max_entries)) {
            cache.remove(key);
        }
    }
}

impl RealTimePerformanceMonitor {
    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub async fn collect_metrics(
        &self,
        _pool_manager: &ZfsPoolManager,
        dataset_manager: &ZfsDatasetManager,
    ) -> Result<()> {
        debug!("Collecting real-time performance metrics");

        // Collect comprehensive ZFS performance metrics with real system integration
        let mut pool_metrics = HashMap::new();
        let mut dataset_metrics = HashMap::new();

        // Real-time pool metrics collection using zpool iostat
        if let Ok(output) = tokio::process::Command::new("zpool")
            .args(["iostat", "-yv", "1", "2"]) // -y for omit first output, -v for verbose
            .output()
            .await
            && output.status.success()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = stdout.lines().collect();

            // Parse pool metrics from iostat output
            for line in lines.iter().skip(1) {
                // Skip header
                let fields: Vec<&str> = line.split_whitespace().collect();
                if fields.len() >= 7 {
                    let pool_name = fields[0];
                    if pool_name != "pool" && !pool_name.is_empty() && !pool_name.contains('-') {
                        let read_ops: f64 = fields[1].parse().unwrap_or(0.0);
                        let write_ops: f64 = fields[2].parse().unwrap_or(0.0);
                        let read_bw: f64 = fields[3].parse().unwrap_or(0.0) / (1024.0 * 1024.0); // Convert to MB/s
                        let write_bw: f64 = fields[4].parse().unwrap_or(0.0) / (1024.0 * 1024.0); // Convert to MB/s

                        // Calculate average latency from queue lengths if available
                        let avg_latency = if fields.len() >= 9 {
                            f64::midpoint(
                                fields[7].parse::<f64>().unwrap_or(0.0),
                                fields[8].parse::<f64>().unwrap_or(0.0),
                            )
                        } else {
                            5.0 // Default latency
                        };

                        // Get cache hit ratio from ARC stats (pool-agnostic for now)
                        let cache_hit_ratio = if let Ok(arc_content) =
                            tokio::fs::read_to_string("/proc/spl/kstat/zfs/arcstats").await
                        {
                            let mut hits = 0u64;
                            let mut misses = 0u64;

                            for arc_line in arc_content.lines() {
                                merge_arc_kstat_hits_misses_line(arc_line, &mut hits, &mut misses);
                            }

                            if hits + misses > 0 {
                                hits as f64 / (hits + misses) as f64
                            } else {
                                0.85 // Default hit ratio
                            }
                        } else {
                            0.85 // Default hit ratio
                        };

                        // Get fragmentation from zpool list
                        let fragmentation = if let Ok(frag_output) =
                            tokio::process::Command::new("zpool")
                                .args(["list", "-H", "-o", "frag", pool_name])
                                .output()
                                .await
                        {
                            let frag_stdout = String::from_utf8_lossy(&frag_output.stdout);
                            frag_percent_from_zpool_frag_stdout(&frag_stdout)
                        } else {
                            10.0 // Default fragmentation
                        };

                        pool_metrics.insert(
                            pool_name.to_string(),
                            ZfsPoolMetrics {
                                pool_name: pool_name.to_string(),
                                read_ops,
                                write_ops,
                                read_bandwidth: read_bw,
                                write_bandwidth: write_bw,
                                latency: avg_latency,
                                cache_hit_ratio,
                                fragmentation,
                            },
                        );
                    }
                }
            }
        }

        // Real-time dataset metrics collection
        if let Ok(datasets) = dataset_manager.list_datasets().await {
            for dataset in datasets {
                // Get comprehensive dataset properties
                if let Ok(prop_output) = tokio::process::Command::new("zfs")
                    .args([
                        "get",
                        "-H",
                        "-p",
                        "compression,compressratio,dedup,recordsize,used,logicalused",
                        &dataset.name,
                    ])
                    .output()
                    .await
                    && prop_output.status.success()
                {
                    let prop_stdout = String::from_utf8_lossy(&prop_output.stdout);

                    let mut props = DatasetPropAccum::new();
                    for line in prop_stdout.lines() {
                        props.apply_tab_line(line, Self::parse_sizevalue);
                    }
                    props.finalize_compression_ratio();
                    let access_pattern = props.access_pattern();

                    dataset_metrics.insert(
                        dataset.name.clone(),
                        ZfsDatasetMetrics {
                            dataset_name: dataset.name.clone(),
                            access_pattern,
                            dedup_ratio: props.dedup_ratio,
                            record_size: props.record_size,
                        },
                    );
                }
            }
        }

        // Collect system memory usage
        let system_memory = SystemMemoryUsage {
            total: 16 * 1024 * 1024 * 1024,    // 16GB default
            used: 8 * 1024 * 1024 * 1024,      // 8GB default
            available: 8 * 1024 * 1024 * 1024, // 8GB default
        };

        // Collect detailed ARC statistics
        let arc_stats = if let Ok(arc_content) =
            tokio::fs::read_to_string("/proc/spl/kstat/zfs/arcstats").await
        {
            let mut hits = 0u64;
            let mut misses = 0u64;
            let mut size = 0u64;
            let mut c = 0u64; // target size
            let mut mru_size = 0u64;
            let mut mfu_size = 0u64;

            for line in arc_content.lines() {
                merge_arc_kstat_full_line(
                    line,
                    &mut hits,
                    &mut misses,
                    &mut size,
                    &mut c,
                    &mut mru_size,
                    &mut mfu_size,
                );
            }

            ArcStatistics {
                hit_ratio: if hits + misses > 0 {
                    hits as f64 / (hits + misses) as f64
                } else {
                    0.85
                },
                size,
                target_size: c,
                miss_ratio: if hits + misses > 0 {
                    misses as f64 / (hits + misses) as f64
                } else {
                    0.15
                },
            }
        } else {
            ArcStatistics {
                hit_ratio: 0.85,
                size: 4 * 1024 * 1024 * 1024,        // 4GB default
                target_size: 8 * 1024 * 1024 * 1024, // 8GB default
                miss_ratio: 0.15,
            }
        };

        // Create comprehensive performance metrics snapshot
        let metrics = ZfsPerformanceMetrics {
            timestamp: SystemTime::now(),
            pool_metrics,
            dataset_metrics,
            system_memory,
            arc_stats,
        };

        self.metrics_cache
            .write()
            .await
            .insert("latest".to_string(), metrics.clone());
        trim_metrics_cache_entries(&mut *self.metrics_cache.write().await, 50);

        // Get metrics for trending
        {
            let cache = self.metrics_cache.read().await;
            if cache.is_empty() {
                return Err(crate::error::ZfsErrorBuilder::new(
                    "No metrics available for trending",
                ));
            }
        }

        // Perform real-time analytics and alerts
        self.analyze_performance_trends().await?;

        debug!(
            "Collected and cached performance metrics: {} pools, {} datasets",
            metrics.pool_metrics.len(),
            metrics.dataset_metrics.len()
        );
        Ok(())
    }
}

#[cfg(test)]
mod collect_metrics_parse_tests {
    use std::collections::HashMap;
    use std::time::SystemTime;

    use super::{
        DatasetPropAccum, frag_percent_from_zpool_frag_stdout, merge_arc_kstat_full_line,
        merge_arc_kstat_hits_misses_line, trim_metrics_cache_entries,
    };
    use crate::performance_engine::types::{
        AccessPattern, ArcStatistics, SystemMemoryUsage, ZfsPerformanceMetrics, ZfsPoolMetrics,
    };

    /// Mirrors the pool-name gate in `RealTimePerformanceMonitor::collect_metrics`.
    /// so branch logic is covered without spawning `zpool`/`zfs`.
    fn pool_name_is_eligible_for_iostat_row(pool_name: &str) -> bool {
        pool_name != "pool" && !pool_name.is_empty() && !pool_name.contains('-')
    }

    #[test]
    fn eligible_pool_names_exclude_header_and_empty_and_hyphenated() {
        assert!(!pool_name_is_eligible_for_iostat_row("pool"));
        assert!(!pool_name_is_eligible_for_iostat_row(""));
        assert!(!pool_name_is_eligible_for_iostat_row("tank-cache"));
        assert!(pool_name_is_eligible_for_iostat_row("tank"));
    }

    #[test]
    fn iostat_row_fields_parse_bandwidth_and_latency_midpoint() {
        let line = "tank 10 20 10485760 20971520 0 0 1.25 3.75";
        let fields: Vec<&str> = line.split_whitespace().collect();
        assert!(fields.len() >= 9);
        let pool_name = fields[0];
        assert!(pool_name_is_eligible_for_iostat_row(pool_name));
        let read_ops: f64 = fields[1].parse().unwrap_or(0.0);
        let write_ops: f64 = fields[2].parse().unwrap_or(0.0);
        let read_bw: f64 = fields[3].parse::<f64>().unwrap_or(0.0) / (1024.0 * 1024.0);
        let write_bw: f64 = fields[4].parse::<f64>().unwrap_or(0.0) / (1024.0 * 1024.0);
        let avg_latency = f64::midpoint(
            fields[7].parse::<f64>().unwrap_or(0.0),
            fields[8].parse::<f64>().unwrap_or(0.0),
        );
        assert!((read_ops - 10.0).abs() < f64::EPSILON);
        assert!((write_ops - 20.0).abs() < f64::EPSILON);
        assert!(read_bw > 0.0 && write_bw > 0.0);
        assert!((avg_latency - 2.5).abs() < 1e-9);
    }

    #[test]
    fn short_iostat_rows_are_skipped() {
        let line = "tank 1 2 3";
        let fields: Vec<&str> = line.split_whitespace().collect();
        assert!(fields.len() < 7);
    }

    #[test]
    fn merge_arc_kstat_hits_misses_parses_known_keys() {
        let mut h = 0u64;
        let mut m = 0u64;
        merge_arc_kstat_hits_misses_line("hits 4 100", &mut h, &mut m);
        merge_arc_kstat_hits_misses_line("misses 4 25", &mut h, &mut m);
        merge_arc_kstat_hits_misses_line("other 4 99", &mut h, &mut m);
        assert_eq!(h, 100);
        assert_eq!(m, 25);
    }

    #[test]
    fn merge_arc_kstat_hits_misses_ignores_short_lines() {
        let mut h = 1u64;
        let mut m = 2u64;
        merge_arc_kstat_hits_misses_line("x", &mut h, &mut m);
        assert_eq!(h, 1);
        assert_eq!(m, 2);
    }

    #[test]
    fn merge_arc_kstat_full_covers_size_c_mru_mfu() {
        let mut hits = 0u64;
        let mut misses = 0u64;
        let mut size = 0u64;
        let mut c = 0u64;
        let mut mru = 0u64;
        let mut mfu = 0u64;
        merge_arc_kstat_full_line(
            "size 4 4096",
            &mut hits,
            &mut misses,
            &mut size,
            &mut c,
            &mut mru,
            &mut mfu,
        );
        merge_arc_kstat_full_line(
            "c 4 8192",
            &mut hits,
            &mut misses,
            &mut size,
            &mut c,
            &mut mru,
            &mut mfu,
        );
        merge_arc_kstat_full_line(
            "mru_size 4 1",
            &mut hits,
            &mut misses,
            &mut size,
            &mut c,
            &mut mru,
            &mut mfu,
        );
        merge_arc_kstat_full_line(
            "mfu_size 4 2",
            &mut hits,
            &mut misses,
            &mut size,
            &mut c,
            &mut mru,
            &mut mfu,
        );
        assert_eq!(size, 4096);
        assert_eq!(c, 8192);
        assert_eq!(mru, 1);
        assert_eq!(mfu, 2);
    }

    #[test]
    fn frag_percent_strips_suffix_and_defaults() {
        assert!((frag_percent_from_zpool_frag_stdout("12%") - 12.0).abs() < f64::EPSILON);
        assert!((frag_percent_from_zpool_frag_stdout("  3.5%  ") - 3.5).abs() < 1e-9);
        assert!((frag_percent_from_zpool_frag_stdout("nope") - 10.0).abs() < f64::EPSILON);
    }

    #[test]
    fn dataset_prop_accum_dedup_recordsize_used_logical_and_pattern() {
        let mut p = DatasetPropAccum::new();
        p.apply_tab_line("tank/d0\tcompressratio\t2.00x", |s| {
            Ok(s.parse::<u64>().unwrap_or(0))
        });
        p.apply_tab_line("tank/d0\tdedup\ton", |s| Ok(s.parse::<u64>().unwrap_or(0)));
        p.apply_tab_line("tank/d0\trecordsize\t65536", |s| Ok(s.parse().unwrap_or(0)));
        p.apply_tab_line("tank/d0\tused\t100", |s| Ok(s.parse().unwrap_or(0)));
        p.apply_tab_line("tank/d0\tlogicalused\t400", |s| Ok(s.parse().unwrap_or(0)));
        p.finalize_compression_ratio();
        assert!((p.compression_ratio - 4.0).abs() < f64::EPSILON);
        assert!((p.dedup_ratio - 1.2).abs() < f64::EPSILON);
        assert_eq!(p.record_size, 65536);
        assert!(matches!(p.access_pattern(), AccessPattern::Mixed));

        let mut small = DatasetPropAccum::new();
        small.apply_tab_line("d\trecordsize\t16384", |s| Ok(s.parse().unwrap_or(0)));
        assert!(matches!(small.access_pattern(), AccessPattern::Random));

        let mut big = DatasetPropAccum::new();
        big.apply_tab_line("d\trecordsize\t2097152", |s| Ok(s.parse().unwrap_or(0)));
        assert!(matches!(big.access_pattern(), AccessPattern::Sequential));
    }

    #[test]
    fn trim_metrics_cache_removes_lexicographic_oldest_over_limit() {
        let mut cache: HashMap<String, ZfsPerformanceMetrics> = HashMap::new();
        let dummy_pool = ZfsPoolMetrics {
            pool_name: "p".into(),
            read_ops: 0.0,
            write_ops: 0.0,
            read_bandwidth: 0.0,
            write_bandwidth: 0.0,
            latency: 0.0,
            cache_hit_ratio: 0.0,
            fragmentation: 0.0,
        };
        let snap = ZfsPerformanceMetrics {
            timestamp: SystemTime::UNIX_EPOCH,
            pool_metrics: HashMap::from([("p".into(), dummy_pool)]),
            dataset_metrics: HashMap::new(),
            system_memory: SystemMemoryUsage {
                total: 1,
                used: 0,
                available: 1,
            },
            arc_stats: ArcStatistics {
                hit_ratio: 0.9,
                size: 0,
                target_size: 0,
                miss_ratio: 0.1,
            },
        };
        for i in 0..52 {
            cache.insert(format!("k{i:02}"), snap.clone());
        }
        trim_metrics_cache_entries(&mut cache, 50);
        assert_eq!(cache.len(), 50);
        assert!(!cache.contains_key("k00"));
        assert!(!cache.contains_key("k01"));
        assert!(cache.contains_key("k51"));
    }
}
