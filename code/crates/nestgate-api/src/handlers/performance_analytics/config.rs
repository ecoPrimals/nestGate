// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Threshold and analysis configuration loaded from environment variables.

use super::types::AnalysisConfig;

/// Alert and recommendation thresholds for performance analytics.
#[derive(Debug, Clone, Copy)]
pub struct PerformanceThresholds {
    /// CPU usage percentage that triggers a warning alert
    pub cpu_warning_percent: f64,
    /// CPU usage percentage that triggers a critical alert
    pub cpu_critical_percent: f64,
    /// Memory usage percentage that triggers a warning alert
    pub memory_warning_percent: f64,
    /// Memory usage percentage that triggers a critical alert
    pub memory_critical_percent: f64,
    /// Root filesystem usage percentage that triggers a warning alert
    pub disk_usage_warning_percent: f64,
    /// Root filesystem usage percentage that triggers a critical alert
    pub disk_usage_critical_percent: f64,
    /// Average disk read latency (ms) that triggers a warning alert
    pub disk_read_latency_warning_ms: f64,
    /// Average disk read latency (ms) that triggers a critical alert
    pub disk_read_latency_critical_ms: f64,
    /// Average disk write latency (ms) that triggers a warning alert
    pub disk_write_latency_warning_ms: f64,
    /// Average disk write latency (ms) that triggers a critical alert
    pub disk_write_latency_critical_ms: f64,
    /// 1-minute load average that triggers a warning alert
    pub load_warning: f64,
    /// 1-minute load average that triggers a critical alert
    pub load_critical: f64,
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
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
}

impl PerformanceThresholds {
    /// Load thresholds from `NESTGATE_PERF_*` environment variables.
    #[must_use]
    pub fn from_env() -> Self {
        let defaults = Self::default();
        Self {
            cpu_warning_percent: env_f64(
                "NESTGATE_PERF_CPU_WARNING_PERCENT",
                defaults.cpu_warning_percent,
            ),
            cpu_critical_percent: env_f64(
                "NESTGATE_PERF_CPU_CRITICAL_PERCENT",
                defaults.cpu_critical_percent,
            ),
            memory_warning_percent: env_f64(
                "NESTGATE_PERF_MEMORY_WARNING_PERCENT",
                defaults.memory_warning_percent,
            ),
            memory_critical_percent: env_f64(
                "NESTGATE_PERF_MEMORY_CRITICAL_PERCENT",
                defaults.memory_critical_percent,
            ),
            disk_usage_warning_percent: env_f64(
                "NESTGATE_PERF_DISK_USAGE_WARNING_PERCENT",
                defaults.disk_usage_warning_percent,
            ),
            disk_usage_critical_percent: env_f64(
                "NESTGATE_PERF_DISK_USAGE_CRITICAL_PERCENT",
                defaults.disk_usage_critical_percent,
            ),
            disk_read_latency_warning_ms: env_f64(
                "NESTGATE_PERF_DISK_READ_LATENCY_WARNING_MS",
                defaults.disk_read_latency_warning_ms,
            ),
            disk_read_latency_critical_ms: env_f64(
                "NESTGATE_PERF_DISK_READ_LATENCY_CRITICAL_MS",
                defaults.disk_read_latency_critical_ms,
            ),
            disk_write_latency_warning_ms: env_f64(
                "NESTGATE_PERF_DISK_WRITE_LATENCY_WARNING_MS",
                defaults.disk_write_latency_warning_ms,
            ),
            disk_write_latency_critical_ms: env_f64(
                "NESTGATE_PERF_DISK_WRITE_LATENCY_CRITICAL_MS",
                defaults.disk_write_latency_critical_ms,
            ),
            load_warning: env_f64("NESTGATE_PERF_LOAD_WARNING", defaults.load_warning),
            load_critical: env_f64("NESTGATE_PERF_LOAD_CRITICAL", defaults.load_critical),
        }
    }
}

impl super::types::AnalysisConfig {
    /// Load analysis settings from `NESTGATE_PERF_*` environment variables.
    #[must_use]
    pub fn from_env() -> Self {
        analysis_config_from_env()
    }
}

fn analysis_config_from_env() -> AnalysisConfig {
    AnalysisConfig {
        interval_seconds: env_u64("NESTGATE_PERF_ANALYSIS_INTERVAL_SECONDS", 60),
        predictive_enabled: env_bool("NESTGATE_PERF_PREDICTIVE_ENABLED", false),
    }
}

fn env_f64(key: &str, default: f64) -> f64 {
    std::env::var(key)
        .ok()
        .and_then(|value| value.parse::<f64>().ok())
        .filter(|value| value.is_finite() && *value >= 0.0)
        .unwrap_or(default)
}

fn env_u64(key: &str, default: u64) -> u64 {
    std::env::var(key)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(default)
}

fn env_bool(key: &str, default: bool) -> bool {
    match std::env::var(key) {
        Ok(value) => matches!(
            value.to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        ),
        Err(_) => default,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thresholds_use_defaults_without_env() {
        let thresholds = PerformanceThresholds::from_env();
        assert_eq!(thresholds.cpu_warning_percent, 80.0);
        assert_eq!(thresholds.disk_read_latency_warning_ms, 50.0);
    }

    #[test]
    fn analysis_config_reads_default_interval() {
        let config = analysis_config_from_env();
        assert_eq!(config.interval_seconds, 60);
        assert!(!config.predictive_enabled);
    }
}
