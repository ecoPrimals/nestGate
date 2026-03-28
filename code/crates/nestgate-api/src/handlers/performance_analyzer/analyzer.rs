// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **PERFORMANCE ANALYSIS ENGINE**
//!
//! Core analysis logic for performance data processing.

use super::metrics::{MetricsError, SystemMetrics};
use serde::{Deserialize, Serialize};

/// Performance analyzer engine
#[derive(Debug)]
/// Performanceanalyzer
pub struct PerformanceAnalyzer {
    /// Analysis configuration
    pub config: AnalyzerConfig,
}

impl PerformanceAnalyzer {
    /// Create new performance analyzer
    #[must_use]
    pub const fn new(config: AnalyzerConfig) -> Self {
        Self { config }
    }

    /// Analyze system performance metrics
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn analyze_metrics(&self, metrics: &SystemMetrics) -> Result<AnalysisResult, MetricsError> {
        Ok(AnalysisResult {
            overall_score: self.calculate_overall_score(metrics),
            cpu_analysis: self.analyze_cpu(metrics.cpu_usage_percent),
            memory_analysis: self.analyze_memory(metrics.memory_usage_bytes),
            disk_analysis: self.analyze_disk(&metrics.disk_io_metrics),
            network_analysis: self.analyze_network(&metrics.network_metrics),
            recommendations: self.generate_recommendations(metrics),
            timestamp: metrics.timestamp,
        })
    }

    /// Calculate Overall Score
    fn calculate_overall_score(&self, metrics: &SystemMetrics) -> f64 {
        // Weighted performance score calculation
        let cpu_score = (100.0 - metrics.cpu_usage_percent).max(0.0);
        let memory_score = 75.0; // Placeholder calculation
        let disk_score = 80.0; // Placeholder calculation
        let network_score = 85.0; // Placeholder calculation

        cpu_score * 0.3 + memory_score * 0.3 + disk_score * 0.2 + network_score * 0.2
    }

    /// Analyze Cpu
    fn analyze_cpu(&self, cpu_usage: f64) -> ComponentAnalysis {
        let status = if cpu_usage > 90.0 {
            PerformanceStatus::Critical
        } else if cpu_usage > 70.0 {
            PerformanceStatus::Warning
        } else {
            PerformanceStatus::Good
        };

        ComponentAnalysis {
            status,
            score: (100.0 - cpu_usage).max(0.0),
            details: format!("CPU usage: {cpu_usage:.1}%"),
        }
    }

    /// Analyze Memory
    fn analyze_memory(&self, memory_usage: u64) -> ComponentAnalysis {
        // Simplified memory analysis
        ComponentAnalysis {
            status: PerformanceStatus::Good,
            score: 75.0,
            details: format!("Memory usage: {memory_usage} bytes"),
        }
    }

    /// Analyze Disk
    fn analyze_disk(&self, disk_metrics: &super::metrics::DiskIOMetrics) -> ComponentAnalysis {
        ComponentAnalysis {
            status: PerformanceStatus::Good,
            score: 80.0,
            details: format!(
                "Disk I/O: {}KB/s read, {}KB/s write",
                disk_metrics.read_bytes_per_sec / 1024,
                disk_metrics.write_bytes_per_sec / 1024
            ),
        }
    }

    /// Analyze Network
    fn analyze_network(
        &self,
        network_metrics: &super::metrics::NetworkMetrics,
    ) -> ComponentAnalysis {
        ComponentAnalysis {
            status: PerformanceStatus::Good,
            score: 85.0,
            details: format!(
                "Network: {}KB/s RX, {}KB/s TX",
                network_metrics.rx_bytes_per_sec / 1024,
                network_metrics.tx_bytes_per_sec / 1024
            ),
        }
    }

    /// Generate Recommendations
    fn generate_recommendations(&self, _metrics: &SystemMetrics) -> Vec<String> {
        vec![
            "System performance is within normal parameters".to_string(),
            "Consider enabling ZFS compression for better storage efficiency".to_string(),
        ]
    }
}

/// Analysis configuration
#[derive(Debug, Clone)]
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::AnalyzerConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::AnalyzerConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for Analyzer
pub struct AnalyzerConfig {
    /// CPU warning threshold (percentage)
    pub cpu_warning_threshold: f64,
    /// CPU critical threshold (percentage)
    pub cpu_critical_threshold: f64,
    /// Memory warning threshold (bytes)
    pub memory_warning_threshold: u64,
}

impl Default for AnalyzerConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            cpu_warning_threshold: 70.0,
            cpu_critical_threshold: 90.0,
            memory_warning_threshold: 8 * 1024 * 1024 * 1024, // 8GB
        }
    }
}

/// Complete analysis result
#[derive(Debug, Serialize, Deserialize)]
/// Analysisresult
pub struct AnalysisResult {
    /// Overall system performance score (0.0-100.0)
    pub overall_score: f64,
    /// CPU performance analysis results
    pub cpu_analysis: ComponentAnalysis,
    /// Memory performance analysis results
    pub memory_analysis: ComponentAnalysis,
    /// Disk I/O performance analysis results
    pub disk_analysis: ComponentAnalysis,
    /// Network performance analysis results
    pub network_analysis: ComponentAnalysis,
    /// Performance improvement recommendations
    pub recommendations: Vec<String>,
    /// When this analysis was performed
    pub timestamp: std::time::SystemTime,
}

/// Analysis result for individual component
#[derive(Debug, Serialize, Deserialize)]
/// Componentanalysis
pub struct ComponentAnalysis {
    /// Performance status level for this component
    pub status: PerformanceStatus,
    /// Component-specific performance score (0.0-100.0)
    pub score: f64,
    /// Detailed analysis information and metrics
    pub details: String,
}

/// Performance status levels
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Status values for Performance
pub enum PerformanceStatus {
    /// Performance is within acceptable parameters
    Good,
    /// Performance issues detected, monitoring recommended
    Warning,
    /// Critical performance issues requiring immediate attention
    Critical,
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Analyzerconfigcanonical
pub type AnalyzerConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using AnalyzerConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    #![allow(deprecated)] // Tests exercise deprecated `AnalyzerConfig` until canonical migration completes

    use super::super::metrics::{DiskIOMetrics, NetworkMetrics};
    use super::*;

    /// Creates  Test Metrics
    fn create_test_metrics() -> SystemMetrics {
        SystemMetrics {
            cpu_usage_percent: 45.0,
            memory_usage_bytes: 4 * 1024 * 1024 * 1024,
            disk_io_metrics: DiskIOMetrics {
                read_bytes_per_sec: 10 * 1024 * 1024,
                write_bytes_per_sec: 5 * 1024 * 1024,
                read_ops_per_sec: 500,
                write_ops_per_sec: 250,
            },
            network_metrics: NetworkMetrics {
                rx_bytes_per_sec: 2 * 1024 * 1024,
                tx_bytes_per_sec: 1024 * 1024,
                rx_packets_per_sec: 2000,
                tx_packets_per_sec: 1000,
            },
            timestamp: std::time::SystemTime::now(),
        }
    }

    #[test]
    fn test_analyzer_config_default() {
        let config = AnalyzerConfig::default();
        assert_eq!(config.cpu_warning_threshold, 70.0);
        assert_eq!(config.cpu_critical_threshold, 90.0);
        assert_eq!(config.memory_warning_threshold, 8 * 1024 * 1024 * 1024);
    }

    #[test]
    fn test_performance_analyzer_new() {
        let config = AnalyzerConfig::default();
        let analyzer = PerformanceAnalyzer::new(config);
        assert_eq!(analyzer.config.cpu_warning_threshold, 70.0);
    }

    #[test]
    fn test_analyze_metrics() {
        let config = AnalyzerConfig::default();
        let analyzer = PerformanceAnalyzer::new(config);
        let metrics = create_test_metrics();

        let result = analyzer.analyze_metrics(&metrics).expect("Should analyze");
        assert!(result.overall_score > 0.0);
        assert!(result.overall_score <= 100.0);
        assert!(!result.recommendations.is_empty());
    }

    #[test]
    fn test_calculate_overall_score() {
        let config = AnalyzerConfig::default();
        let analyzer = PerformanceAnalyzer::new(config);
        let metrics = create_test_metrics();

        let score = analyzer.calculate_overall_score(&metrics);
        assert!(score > 0.0);
        assert!(score <= 100.0);
    }

    #[test]
    fn test_analyze_cpu_good() {
        let config = AnalyzerConfig::default();
        let analyzer = PerformanceAnalyzer::new(config);

        let analysis = analyzer.analyze_cpu(45.0);
        assert!(matches!(analysis.status, PerformanceStatus::Good));
        assert_eq!(analysis.score, 55.0);
    }

    #[test]
    fn test_analyze_cpu_warning() {
        let config = AnalyzerConfig::default();
        let analyzer = PerformanceAnalyzer::new(config);

        let analysis = analyzer.analyze_cpu(75.0);
        assert!(matches!(analysis.status, PerformanceStatus::Warning));
    }

    #[test]
    fn test_analyze_cpu_critical() {
        let config = AnalyzerConfig::default();
        let analyzer = PerformanceAnalyzer::new(config);

        let analysis = analyzer.analyze_cpu(95.0);
        assert!(matches!(analysis.status, PerformanceStatus::Critical));
    }

    #[test]
    fn test_analyze_memory() {
        let config = AnalyzerConfig::default();
        let analyzer = PerformanceAnalyzer::new(config);

        let analysis = analyzer.analyze_memory(4 * 1024 * 1024 * 1024);
        assert!(matches!(analysis.status, PerformanceStatus::Good));
        assert_eq!(analysis.score, 75.0);
    }

    #[test]
    fn test_analyze_disk() {
        let config = AnalyzerConfig::default();
        let analyzer = PerformanceAnalyzer::new(config);

        let disk_metrics = DiskIOMetrics {
            read_bytes_per_sec: 10 * 1024 * 1024,
            write_bytes_per_sec: 5 * 1024 * 1024,
            read_ops_per_sec: 500,
            write_ops_per_sec: 250,
        };

        let analysis = analyzer.analyze_disk(&disk_metrics);
        assert!(matches!(analysis.status, PerformanceStatus::Good));
        assert_eq!(analysis.score, 80.0);
    }

    #[test]
    fn test_analyze_network() {
        let config = AnalyzerConfig::default();
        let analyzer = PerformanceAnalyzer::new(config);

        let network_metrics = NetworkMetrics {
            rx_bytes_per_sec: 2 * 1024 * 1024,
            tx_bytes_per_sec: 1024 * 1024,
            rx_packets_per_sec: 2000,
            tx_packets_per_sec: 1000,
        };

        let analysis = analyzer.analyze_network(&network_metrics);
        assert!(matches!(analysis.status, PerformanceStatus::Good));
        assert_eq!(analysis.score, 85.0);
    }

    #[test]
    fn test_generate_recommendations() {
        let config = AnalyzerConfig::default();
        let analyzer = PerformanceAnalyzer::new(config);
        let metrics = create_test_metrics();

        let recommendations = analyzer.generate_recommendations(&metrics);
        assert!(!recommendations.is_empty());
        assert!(recommendations.len() >= 2);
    }

    #[test]
    fn test_analysis_result_serialization() {
        let result = AnalysisResult {
            overall_score: 85.5,
            cpu_analysis: ComponentAnalysis {
                status: PerformanceStatus::Good,
                score: 90.0,
                details: "CPU usage: 10%".to_string(),
            },
            memory_analysis: ComponentAnalysis {
                status: PerformanceStatus::Good,
                score: 85.0,
                details: "Memory usage: 4GB".to_string(),
            },
            disk_analysis: ComponentAnalysis {
                status: PerformanceStatus::Good,
                score: 80.0,
                details: "Disk I/O: Good".to_string(),
            },
            network_analysis: ComponentAnalysis {
                status: PerformanceStatus::Good,
                score: 88.0,
                details: "Network: Good".to_string(),
            },
            recommendations: vec!["All good".to_string()],
            timestamp: std::time::SystemTime::now(),
        };

        let json = serde_json::to_string(&result).expect("Should serialize");
        let deserialized: AnalysisResult = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(result.overall_score, deserialized.overall_score);
        assert_eq!(
            result.recommendations.len(),
            deserialized.recommendations.len()
        );
    }

    #[test]
    fn test_performance_status_variants() {
        let good = PerformanceStatus::Good;
        let warning = PerformanceStatus::Warning;
        let critical = PerformanceStatus::Critical;

        let good_json = serde_json::to_string(&good).expect("Should serialize");
        let warning_json = serde_json::to_string(&warning).expect("Should serialize");
        let critical_json = serde_json::to_string(&critical).expect("Should serialize");

        assert!(good_json.contains("Good"));
        assert!(warning_json.contains("Warning"));
        assert!(critical_json.contains("Critical"));
    }

    #[test]
    fn test_component_analysis_serialization() {
        let analysis = ComponentAnalysis {
            status: PerformanceStatus::Warning,
            score: 65.5,
            details: "Test details".to_string(),
        };

        let json = serde_json::to_string(&analysis).expect("Should serialize");
        let deserialized: ComponentAnalysis =
            serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(analysis.score, deserialized.score);
        assert_eq!(analysis.details, deserialized.details);
    }
}
