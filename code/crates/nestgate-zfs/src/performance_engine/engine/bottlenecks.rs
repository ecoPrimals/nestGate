// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Bottleneck detection from trending performance metrics.

use tracing::{debug, info, warn};

use crate::dataset::ZfsDatasetManager;
use crate::error::Result;
use crate::pool::ZfsPoolManager;

use super::super::monitoring::RealTimePerformanceMonitor;
use super::super::types::{
    BottleneckSeverity, ZfsBottleneck, ZfsBottleneckType, ZfsPerformanceMetrics,
};

use super::PerformanceOptimizationEngine;

impl PerformanceOptimizationEngine {
    /// Detect and analyze performance bottlenecks
    pub(super) async fn detect_and_analyze_bottlenecks(
        performance_monitor: &RealTimePerformanceMonitor,
        _pool_manager: &ZfsPoolManager,
        _dataset_manager: &ZfsDatasetManager,
    ) -> Result<Vec<ZfsBottleneck>> {
        debug!("Analyzing ZFS performance bottlenecks");

        let mut bottlenecks = Vec::new();

        // Get current trending data for analysis
        let trending_data = performance_monitor.get_trending_data().await?;

        if let Some(latest_metrics) = trending_data.last() {
            analyze_pool_bottlenecks(latest_metrics, &mut bottlenecks);
            analyze_arc_bottlenecks(latest_metrics, &mut bottlenecks);
            analyze_memory_pressure(latest_metrics, &mut bottlenecks);
        }

        if bottlenecks.is_empty() {
            debug!("No performance bottlenecks detected");
        } else {
            info!("Detected {} performance bottlenecks", bottlenecks.len());
            for bottleneck in &bottlenecks {
                warn!("Bottleneck: {}", bottleneck.description);
            }
        }
        Ok(bottlenecks)
    }
}

fn analyze_pool_bottlenecks(
    latest_metrics: &ZfsPerformanceMetrics,
    bottlenecks: &mut Vec<ZfsBottleneck>,
) {
    for (pool_name, pool_metrics) in &latest_metrics.pool_metrics {
        // Check for high latency bottleneck
        if pool_metrics.latency > 50.0 {
            bottlenecks.push(ZfsBottleneck {
                bottleneck_type: ZfsBottleneckType::HighLatency,
                severity: if pool_metrics.latency > 100.0 {
                    BottleneckSeverity::Critical
                } else {
                    BottleneckSeverity::Medium
                },
                pool_name: pool_name.clone(),
                dataset_name: None,
                description: format!(
                    "Pool {} experiencing high latency: {:.2}ms",
                    pool_name, pool_metrics.latency
                ),
                impact_score: if pool_metrics.latency > 100.0 {
                    90.0
                } else {
                    60.0
                },
            });
        }

        // Check for low cache hit ratio
        if pool_metrics.cache_hit_ratio < 0.8 {
            bottlenecks.push(ZfsBottleneck {
                bottleneck_type: ZfsBottleneckType::CacheMiss,
                severity: if pool_metrics.cache_hit_ratio < 0.6 {
                    BottleneckSeverity::Critical
                } else {
                    BottleneckSeverity::Medium
                },
                pool_name: pool_name.clone(),
                dataset_name: None,
                description: format!(
                    "Pool {} has low cache hit ratio: {:.1}%",
                    pool_name,
                    pool_metrics.cache_hit_ratio * 100.0
                ),
                impact_score: if pool_metrics.cache_hit_ratio < 0.6 {
                    80.0
                } else {
                    50.0
                },
            });
        }

        // Check for high fragmentation
        if pool_metrics.fragmentation > 30.0 {
            bottlenecks.push(ZfsBottleneck {
                bottleneck_type: ZfsBottleneckType::Fragmentation,
                severity: if pool_metrics.fragmentation > 50.0 {
                    BottleneckSeverity::Critical
                } else {
                    BottleneckSeverity::Medium
                },
                pool_name: pool_name.clone(),
                dataset_name: None,
                description: format!(
                    "Pool {} has high fragmentation: {:.1}%",
                    pool_name, pool_metrics.fragmentation
                ),
                impact_score: if pool_metrics.fragmentation > 50.0 {
                    75.0
                } else {
                    45.0
                },
            });
        }
    }
}

fn analyze_arc_bottlenecks(
    latest_metrics: &ZfsPerformanceMetrics,
    bottlenecks: &mut Vec<ZfsBottleneck>,
) {
    let arc_stats = &latest_metrics.arc_stats;
    if arc_stats.hit_ratio < 0.85 {
        bottlenecks.push(ZfsBottleneck {
            bottleneck_type: ZfsBottleneckType::CacheMiss,
            severity: if arc_stats.hit_ratio < 0.7 {
                BottleneckSeverity::Critical
            } else {
                BottleneckSeverity::Medium
            },
            pool_name: "system_arc".to_string(),
            dataset_name: None,
            description: format!(
                "ARC hit ratio below optimal: {:.1}%",
                arc_stats.hit_ratio * 100.0
            ),
            impact_score: if arc_stats.hit_ratio < 0.7 {
                85.0
            } else {
                55.0
            },
        });
    }
}

fn analyze_memory_pressure(
    latest_metrics: &ZfsPerformanceMetrics,
    bottlenecks: &mut Vec<ZfsBottleneck>,
) {
    let memory_usage_percent = (latest_metrics.system_memory.used as f64
        / latest_metrics.system_memory.total as f64)
        * 100.0;
    if memory_usage_percent > 90.0 {
        bottlenecks.push(ZfsBottleneck {
            bottleneck_type: ZfsBottleneckType::MemoryPressure,
            severity: BottleneckSeverity::Critical,
            pool_name: "system_memory".to_string(),
            dataset_name: None,
            description: "actual_error_details".to_string(),
            impact_score: 95.0,
        });
    }
}
