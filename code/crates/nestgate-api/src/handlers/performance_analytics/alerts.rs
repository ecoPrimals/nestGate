// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Generate performance alerts from collected metrics and configured thresholds.

use std::collections::HashMap;
use std::time::SystemTime;

use anyhow::Result;

use super::collector;
use super::config::PerformanceThresholds;
use super::types::PerformanceAlert;

/// Evaluate current metrics against thresholds and return active alerts.
///
/// # Errors
///
/// Returns an error when metrics collection fails.
pub async fn generate_performance_alerts() -> Result<Vec<PerformanceAlert>> {
    let thresholds = PerformanceThresholds::from_env();
    let metrics = collector::collect_performance_metrics().await?;
    Ok(evaluate_alerts(&metrics, &thresholds, SystemTime::now()))
}

fn evaluate_alerts(
    metrics: &HashMap<String, f64>,
    thresholds: &PerformanceThresholds,
    timestamp: SystemTime,
) -> Vec<PerformanceAlert> {
    let mut alerts = Vec::new();

    if let Some(value) = metrics.get("cpu_usage") {
        push_threshold_alert(
            &mut alerts,
            "cpu",
            "CPU usage",
            *value,
            thresholds.cpu_warning_percent,
            thresholds.cpu_critical_percent,
            timestamp,
        );
    }

    if let Some(value) = metrics.get("memory_usage") {
        push_threshold_alert(
            &mut alerts,
            "memory",
            "Memory usage",
            *value,
            thresholds.memory_warning_percent,
            thresholds.memory_critical_percent,
            timestamp,
        );
    }

    if let Some(value) = metrics.get("disk_usage") {
        push_threshold_alert(
            &mut alerts,
            "disk_usage",
            "Root filesystem usage",
            *value,
            thresholds.disk_usage_warning_percent,
            thresholds.disk_usage_critical_percent,
            timestamp,
        );
    }

    if let Some(value) = metrics.get("disk_read_latency_ms") {
        push_threshold_alert(
            &mut alerts,
            "disk_read_latency",
            "Disk read latency",
            *value,
            thresholds.disk_read_latency_warning_ms,
            thresholds.disk_read_latency_critical_ms,
            timestamp,
        );
    }

    if let Some(value) = metrics.get("disk_write_latency_ms") {
        push_threshold_alert(
            &mut alerts,
            "disk_write_latency",
            "Disk write latency",
            *value,
            thresholds.disk_write_latency_warning_ms,
            thresholds.disk_write_latency_critical_ms,
            timestamp,
        );
    }

    if let Some(value) = metrics.get("load_average_1m") {
        push_threshold_alert(
            &mut alerts,
            "load_average",
            "1-minute load average",
            *value,
            thresholds.load_warning,
            thresholds.load_critical,
            timestamp,
        );
    }

    alerts
}

fn push_threshold_alert(
    alerts: &mut Vec<PerformanceAlert>,
    id_prefix: &str,
    label: &str,
    value: f64,
    warning_threshold: f64,
    critical_threshold: f64,
    timestamp: SystemTime,
) {
    if value >= critical_threshold {
        alerts.push(PerformanceAlert {
            id: format!("{id_prefix}_critical"),
            message: format!("{label} is {value:.1} (critical threshold: {critical_threshold:.1})"),
            severity: "critical".to_owned(),
            timestamp,
        });
    } else if value >= warning_threshold {
        alerts.push(PerformanceAlert {
            id: format!("{id_prefix}_warning"),
            message: format!("{label} is {value:.1} (warning threshold: {warning_threshold:.1})"),
            severity: "warning".to_owned(),
            timestamp,
        });
    }
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
    fn no_alerts_when_metrics_within_thresholds() {
        let mut metrics = HashMap::new();
        metrics.insert("cpu_usage".to_owned(), 10.0);
        metrics.insert("memory_usage".to_owned(), 20.0);

        let alerts = evaluate_alerts(&metrics, &sample_thresholds(), SystemTime::now());
        assert!(alerts.is_empty());
    }

    #[test]
    fn warning_alert_when_threshold_exceeded() {
        let mut metrics = HashMap::new();
        metrics.insert("cpu_usage".to_owned(), 82.0);

        let alerts = evaluate_alerts(&metrics, &sample_thresholds(), SystemTime::now());
        assert_eq!(alerts.len(), 1);
        assert_eq!(alerts[0].id, "cpu_warning");
        assert_eq!(alerts[0].severity, "warning");
    }

    #[test]
    fn critical_alert_takes_precedence_over_warning_band() {
        let mut metrics = HashMap::new();
        metrics.insert("memory_usage".to_owned(), 96.0);

        let alerts = evaluate_alerts(&metrics, &sample_thresholds(), SystemTime::now());
        assert_eq!(alerts.len(), 1);
        assert_eq!(alerts[0].id, "memory_critical");
        assert_eq!(alerts[0].severity, "critical");
    }
}
