// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Generate optimization recommendations from current performance metrics.

use std::collections::HashMap;

use anyhow::Result;

use super::collector;
use super::config::PerformanceThresholds;
use super::types::PerformanceRecommendation;

/// Analyze current metrics and return prioritized optimization recommendations.
///
/// # Errors
///
/// Returns an error when metrics collection fails.
pub async fn generate_performance_recommendations() -> Result<Vec<PerformanceRecommendation>> {
    let thresholds = PerformanceThresholds::from_env();
    let metrics = collector::collect_performance_metrics().await?;
    let mut recommendations = analyze_recommendations(&metrics, &thresholds);
    recommendations.sort_by_key(|rec| rec.priority);
    Ok(recommendations)
}

fn analyze_recommendations(
    metrics: &HashMap<String, f64>,
    thresholds: &PerformanceThresholds,
) -> Vec<PerformanceRecommendation> {
    let mut recommendations = Vec::new();

    if metric_at_or_above(metrics, "cpu_usage", thresholds.cpu_warning_percent) {
        recommendations.push(PerformanceRecommendation {
            id: "rec_cpu_workload".to_owned(),
            title: "Reduce CPU pressure".to_owned(),
            description: "Review CPU-intensive workloads, batch background jobs, and tune \
                          service concurrency limits to bring CPU usage below configured \
                          thresholds."
                .to_owned(),
            impact: "High — lowers scheduler contention and improves response latency.".to_owned(),
            priority: if metric_at_or_above(metrics, "cpu_usage", thresholds.cpu_critical_percent) {
                1
            } else {
                2
            },
        });
    }

    if metric_at_or_above(metrics, "memory_usage", thresholds.memory_warning_percent) {
        recommendations.push(PerformanceRecommendation {
            id: "rec_memory_capacity".to_owned(),
            title: "Reclaim or expand memory".to_owned(),
            description: "Identify memory-heavy processes, tune cache limits, or add RAM to \
                          avoid swap thrashing and out-of-memory pressure."
                .to_owned(),
            impact: "High — stabilizes throughput under sustained load.".to_owned(),
            priority: if metric_at_or_above(
                metrics,
                "memory_usage",
                thresholds.memory_critical_percent,
            ) {
                1
            } else {
                2
            },
        });
    }

    if metric_at_or_above(metrics, "disk_usage", thresholds.disk_usage_warning_percent) {
        recommendations.push(PerformanceRecommendation {
            id: "rec_disk_capacity".to_owned(),
            title: "Free or expand root storage".to_owned(),
            description: "Archive or delete stale data, rotate logs, and expand the root \
                          filesystem before free space exhaustion impacts writes."
                .to_owned(),
            impact: "Medium — prevents write failures and filesystem fragmentation.".to_owned(),
            priority: if metric_at_or_above(
                metrics,
                "disk_usage",
                thresholds.disk_usage_critical_percent,
            ) {
                1
            } else {
                3
            },
        });
    }

    if metric_at_or_above(
        metrics,
        "disk_read_latency_ms",
        thresholds.disk_read_latency_warning_ms,
    ) || metric_at_or_above(
        metrics,
        "disk_write_latency_ms",
        thresholds.disk_write_latency_warning_ms,
    ) {
        recommendations.push(PerformanceRecommendation {
            id: "rec_disk_io".to_owned(),
            title: "Investigate disk I/O bottlenecks".to_owned(),
            description: "Inspect queue depth, storage health, and workload patterns; coalesce \
                          small writes and move hot datasets to faster media if latency \
                          remains elevated."
                .to_owned(),
            impact: "High — reduces application wait time on storage operations.".to_owned(),
            priority: 1,
        });
    }

    if metric_at_or_above(metrics, "load_average_1m", thresholds.load_warning) {
        recommendations.push(PerformanceRecommendation {
            id: "rec_load_balance".to_owned(),
            title: "Balance system load".to_owned(),
            description: "Spread work across CPU cores, defer non-critical jobs, or add compute \
                          capacity when sustained load averages exceed configured limits."
                .to_owned(),
            impact: "Medium — improves fairness and tail latency under contention.".to_owned(),
            priority: if metric_at_or_above(metrics, "load_average_1m", thresholds.load_critical) {
                1
            } else {
                3
            },
        });
    }

    if metric_at_or_above(metrics, "iowait_percent", thresholds.cpu_warning_percent) {
        recommendations.push(PerformanceRecommendation {
            id: "rec_iowait".to_owned(),
            title: "Reduce I/O wait".to_owned(),
            description: "High I/O wait suggests storage-bound workloads; optimize access \
                          patterns, enable read caching, or upgrade storage performance."
                .to_owned(),
            impact: "Medium — frees CPU cycles blocked on storage completion.".to_owned(),
            priority: 2,
        });
    }

    recommendations
}

fn metric_at_or_above(metrics: &HashMap<String, f64>, key: &str, threshold: f64) -> bool {
    metrics
        .get(key)
        .copied()
        .is_some_and(|value| value >= threshold)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn sample_thresholds() -> PerformanceThresholds {
        PerformanceThresholds {
            cpu_warning_percent: 80.0,
            cpu_critical_percent: 95.0,
            memory_warning_percent: 85.0,
            memory_critical_percent: 95.0,
            disk_usage_warning_percent: 85.0,
            disk_usage_critical_percent: 95.0,
            disk_read_latency_warning_ms: 50.0,
            disk_read_latency_critical_ms: 200.0,
            disk_write_latency_warning_ms: 50.0,
            disk_write_latency_critical_ms: 200.0,
            load_warning: 4.0,
            load_critical: 8.0,
        }
    }

    #[test]
    fn no_recommendations_when_metrics_are_healthy() {
        let mut metrics = HashMap::new();
        metrics.insert("cpu_usage".to_owned(), 10.0);
        metrics.insert("memory_usage".to_owned(), 20.0);

        let recs = analyze_recommendations(&metrics, &sample_thresholds());
        assert!(recs.is_empty());
    }

    #[test]
    fn cpu_recommendation_when_usage_high() {
        let mut metrics = HashMap::new();
        metrics.insert("cpu_usage".to_owned(), 90.0);

        let recs = analyze_recommendations(&metrics, &sample_thresholds());
        assert!(recs.iter().any(|rec| rec.id == "rec_cpu_workload"));
    }

    #[test]
    fn recommendations_sorted_by_priority() {
        let mut metrics = HashMap::new();
        metrics.insert("cpu_usage".to_owned(), 96.0);
        metrics.insert("disk_read_latency_ms".to_owned(), 120.0);

        let mut recs = analyze_recommendations(&metrics, &sample_thresholds());
        recs.sort_by_key(|rec| rec.priority);
        assert!(!recs.is_empty());
        assert!(recs[0].priority <= recs[recs.len() - 1].priority);
    }
}
