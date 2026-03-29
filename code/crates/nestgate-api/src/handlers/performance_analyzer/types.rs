// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Type definitions for performance analysis

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Performance trend analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performancetrend
pub enum PerformanceTrend {
    /// Performance is improving
    Improving,
    /// Performance is stable
    Stable,
    /// Performance is degrading
    Degrading,
    /// Not enough data to determine trend
    Unknown,
}

/// Component performance analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Componentanalysis
pub struct ComponentAnalysis {
    /// Component name
    pub component_name: String,
    /// Current usage percentage
    pub current_usage: f64,
    /// Performance trend
    pub trend: PerformanceTrend,
    /// Detected anomalies
    pub anomalies: Vec<String>,
    /// Performance recommendations
    pub recommendations: Vec<String>,
}

/// Performance snapshot at a point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performancesnapshot
pub struct PerformanceSnapshot {
    /// Timestamp of the snapshot
    pub timestamp: DateTime<Utc>,
    /// CPU metrics
    pub cpu: CpuMetrics,
    /// Memory metrics
    pub memory: MemoryMetrics,
    /// Disk metrics
    pub disk: DiskMetrics,
    /// Network metrics
    pub network: NetworkMetrics,
    /// ZFS metrics
    pub zfs: ZfsMetrics,
}

/// CPU performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Cpumetrics
pub struct CpuMetrics {
    /// CPU usage percentage
    pub usage_percent: f64,
    /// Load average (1 minute)
    pub load_average_1m: f64,
    /// Load average (5 minutes)
    pub load_average_5m: f64,
    /// Load average (15 minutes)
    pub load_average_15m: f64,
    /// Number of CPU cores
    pub core_count: u32,
}

/// Memory performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Memorymetrics
pub struct MemoryMetrics {
    /// Total memory in bytes
    pub total_bytes: u64,
    /// Used memory in bytes
    pub used_bytes: u64,
    /// Available memory in bytes
    pub available_bytes: u64,
    /// Memory usage percentage
    pub usage_percent: f64,
    /// Swap usage in bytes
    pub swap_used_bytes: u64,
}

/// Disk performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Diskmetrics
pub struct DiskMetrics {
    /// Read operations per second
    pub read_ops_per_sec: f64,
    /// Write operations per second
    pub write_ops_per_sec: f64,
    /// Read throughput in bytes per second
    pub read_bytes_per_sec: f64,
    /// Write throughput in bytes per second
    pub write_bytes_per_sec: f64,
    /// Average queue depth
    pub avg_queue_depth: f64,
}

/// Network performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Networkmetrics
pub struct NetworkMetrics {
    /// Bytes received per second
    pub rx_bytes_per_sec: f64,
    /// Bytes transmitted per second
    pub tx_bytes_per_sec: f64,
    /// Packets received per second
    pub rx_packets_per_sec: f64,
    /// Packets transmitted per second
    pub tx_packets_per_sec: f64,
}

/// ZFS performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Zfsmetrics
pub struct ZfsMetrics {
    /// ARC hit ratio
    pub arc_hit_ratio: f64,
    /// ARC size in bytes
    pub arc_size_bytes: u64,
    /// L2ARC hit ratio
    pub l2arc_hit_ratio: f64,
    /// Pool capacity usage percentage
    pub pool_capacity_percent: f64,
    /// Pool health status
    pub pool_health: String,
    /// Scrub status
    pub scrub_status: String,
    /// Dataset count
    pub dataset_count: u32,
    /// Snapshot count
    pub snapshot_count: u32,
}

/// Configuration for performance analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::PerformanceAnalysisConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::PerformanceAnalysisConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for `PerformanceAnalysis`
pub struct PerformanceAnalysisConfig {
    /// Enable CPU monitoring
    pub enable_cpu_monitoring: bool,
    /// Enable memory monitoring
    pub enable_memory_monitoring: bool,
    /// Enable disk monitoring
    pub enable_disk_monitoring: bool,
    /// Enable network monitoring
    pub enable_network_monitoring: bool,
    /// Enable ZFS monitoring
    pub enable_zfs_monitoring: bool,
    /// Analysis interval in seconds
    pub analysis_interval_seconds: u64,
    /// Maximum history entries to keep
    pub max_history_entries: usize,
}

impl Default for PerformanceAnalysisConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enable_cpu_monitoring: true,
            enable_memory_monitoring: true,
            enable_disk_monitoring: true,
            enable_network_monitoring: true,
            enable_zfs_monitoring: true,
            analysis_interval_seconds: 30,
            max_history_entries: 1000,
        }
    }
}

/// Performance analysis report
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performanceanalysisreport
pub struct PerformanceAnalysisReport {
    /// Report generation timestamp
    pub generated_at: DateTime<Utc>,
    /// Overall system health score (0-100)
    pub overall_health_score: f64,
    /// Performance trends
    pub trends: PerformanceTrends,
    /// Component analyses
    pub component_analyses: Vec<ComponentAnalysis>,
    /// Performance recommendations
    pub recommendations: Vec<PerformanceRecommendation>,
    /// Critical issues detected
    pub critical_issues: Vec<String>,
    /// Warnings
    pub warnings: Vec<String>,
    /// System uptime in seconds
    pub system_uptime_seconds: u64,
    /// Analysis period start
    pub analysis_period_start: DateTime<Utc>,
    /// Analysis period end
    pub analysis_period_end: DateTime<Utc>,
}

/// Performance trends analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performancetrends
pub struct PerformanceTrends {
    /// CPU usage trend
    pub cpu_trend: PerformanceTrend,
    /// Memory usage trend
    pub memory_trend: PerformanceTrend,
    /// Disk I/O trend
    pub disk_trend: PerformanceTrend,
    /// Network I/O trend
    pub network_trend: PerformanceTrend,
    /// ZFS performance trend
    pub zfs_trend: PerformanceTrend,
}

/// CPU analysis details
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Cpuanalysis
pub struct CpuAnalysis {
    /// Current CPU usage
    pub current_usage: f64,
    /// Peak CPU usage in analysis period
    pub peak_usage: f64,
    /// Average CPU usage in analysis period
    pub average_usage: f64,
    /// CPU trend
    pub trend: PerformanceTrend,
}

/// Memory analysis details
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Memoryanalysis
pub struct MemoryAnalysis {
    /// Current memory usage percentage
    pub current_usage_percent: f64,
    /// Peak memory usage in analysis period
    pub peak_usage_percent: f64,
    /// Average memory usage in analysis period
    pub average_usage_percent: f64,
    /// Memory trend
    pub trend: PerformanceTrend,
}

/// Disk analysis details
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Diskanalysis
pub struct DiskAnalysis {
    /// Current disk I/O utilization
    pub current_io_utilization: f64,
    /// Peak IOPS in analysis period
    pub peak_iops: f64,
    /// Average IOPS in analysis period
    pub average_iops: f64,
    /// Disk I/O trend
    pub trend: PerformanceTrend,
}

/// Network analysis details
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Networkanalysis
pub struct NetworkAnalysis {
    /// Current network utilization
    pub current_utilization: f64,
    /// Peak bandwidth usage in analysis period
    pub peak_bandwidth_mbps: f64,
    /// Average bandwidth usage in analysis period
    pub average_bandwidth_mbps: f64,
    /// Network trend
    pub trend: PerformanceTrend,
}

/// ZFS analysis details
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Zfsanalysis
pub struct ZfsAnalysis {
    /// Current ARC hit ratio
    pub current_arc_hit_ratio: f64,
    /// Pool capacity usage
    pub pool_capacity_percent: f64,
    /// Pool health status
    pub pool_health: String,
    /// ZFS performance trend
    pub trend: PerformanceTrend,
}

/// Performance recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performancerecommendation
pub struct PerformanceRecommendation {
    /// Recommendation category
    pub category: String,
    /// Recommendation description
    pub description: String,
    /// Priority level (1-10, 10 being highest)
    pub priority: u8,
    /// Estimated impact
    pub estimated_impact: String,
}

/// Performance analyzer state
#[derive(Debug, Clone, Default)]
/// Performanceanalyzerstate
pub struct PerformanceAnalyzerState {
    /// Whether analysis is currently running
    pub is_running: bool,
    /// Last analysis timestamp
    pub last_analysis: Option<DateTime<Utc>>,
    /// Total analyses performed
    pub total_analyses: u64,
    /// Configuration
    pub config: PerformanceAnalysisConfig,
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Performanceanalysisconfigcanonical
pub type PerformanceAnalysisConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using PerformanceAnalysisConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== PERFORMANCE TREND TESTS ====================

    #[test]
    fn test_performance_trend_variants() {
        let trends = [
            PerformanceTrend::Improving,
            PerformanceTrend::Stable,
            PerformanceTrend::Degrading,
            PerformanceTrend::Unknown,
        ];
        assert_eq!(trends.len(), 4);
    }

    // ==================== CPU METRICS TESTS ====================

    #[test]
    fn test_cpu_metrics_creation() {
        let cpu = CpuMetrics {
            usage_percent: 45.2,
            load_average_1m: 2.5,
            load_average_5m: 2.0,
            load_average_15m: 1.8,
            core_count: 8,
        };

        assert_eq!(cpu.usage_percent, 45.2);
        assert_eq!(cpu.core_count, 8);
    }

    #[test]
    fn test_cpu_metrics_boundary_values() {
        let cpu = CpuMetrics {
            usage_percent: 0.0,
            load_average_1m: 0.0,
            load_average_5m: 0.0,
            load_average_15m: 0.0,
            core_count: 1,
        };

        assert_eq!(cpu.usage_percent, 0.0);
        assert_eq!(cpu.core_count, 1);
    }

    // ==================== MEMORY METRICS TESTS ====================

    #[test]
    fn test_memory_metrics_creation() {
        let memory = MemoryMetrics {
            total_bytes: 16_000_000_000,
            used_bytes: 8_000_000_000,
            available_bytes: 8_000_000_000,
            usage_percent: 50.0,
            swap_used_bytes: 1_000_000_000,
        };

        assert_eq!(memory.total_bytes, 16_000_000_000);
        assert_eq!(memory.usage_percent, 50.0);
    }

    #[test]
    fn test_memory_metrics_percentage_calculation() {
        let memory = MemoryMetrics {
            total_bytes: 10_000_000_000,
            used_bytes: 7_500_000_000,
            available_bytes: 2_500_000_000,
            usage_percent: 75.0,
            swap_used_bytes: 0,
        };

        assert_eq!(memory.usage_percent, 75.0);
        assert_eq!(memory.available_bytes, 2_500_000_000);
    }

    // ==================== DISK METRICS TESTS ====================

    #[test]
    fn test_disk_metrics_creation() {
        let disk = DiskMetrics {
            read_ops_per_sec: 150.0,
            write_ops_per_sec: 75.0,
            read_bytes_per_sec: 10_000_000.0,
            write_bytes_per_sec: 5_000_000.0,
            avg_queue_depth: 2.5,
        };

        assert_eq!(disk.read_ops_per_sec, 150.0);
        assert_eq!(disk.write_ops_per_sec, 75.0);
    }

    #[test]
    fn test_disk_metrics_zero_activity() {
        let disk = DiskMetrics {
            read_ops_per_sec: 0.0,
            write_ops_per_sec: 0.0,
            read_bytes_per_sec: 0.0,
            write_bytes_per_sec: 0.0,
            avg_queue_depth: 0.0,
        };

        assert_eq!(disk.avg_queue_depth, 0.0);
    }

    // ==================== NETWORK METRICS TESTS ====================

    #[test]
    fn test_network_metrics_creation() {
        let network = NetworkMetrics {
            rx_bytes_per_sec: 1_000_000.0,
            tx_bytes_per_sec: 500_000.0,
            rx_packets_per_sec: 1000.0,
            tx_packets_per_sec: 500.0,
        };

        assert_eq!(network.rx_bytes_per_sec, 1_000_000.0);
        assert_eq!(network.tx_packets_per_sec, 500.0);
    }

    // ==================== ZFS METRICS TESTS ====================

    #[test]
    fn test_zfs_metrics_creation() {
        let zfs = ZfsMetrics {
            arc_hit_ratio: 0.95,
            arc_size_bytes: 8_000_000_000,
            l2arc_hit_ratio: 0.85,
            pool_capacity_percent: 60.0,
            pool_health: "ONLINE".to_string(),
            scrub_status: "In Progress".to_string(),
            dataset_count: 10,
            snapshot_count: 25,
        };

        assert_eq!(zfs.arc_hit_ratio, 0.95);
        assert_eq!(zfs.pool_health, "ONLINE");
        assert_eq!(zfs.dataset_count, 10);
    }

    #[test]
    fn test_zfs_metrics_various_health_states() {
        let health_states = vec!["ONLINE", "DEGRADED", "FAULTED", "OFFLINE"];

        for health in health_states {
            let zfs = ZfsMetrics {
                arc_hit_ratio: 0.9,
                arc_size_bytes: 1_000_000_000,
                l2arc_hit_ratio: 0.8,
                pool_capacity_percent: 50.0,
                pool_health: health.to_string(),
                scrub_status: "None".to_string(),
                dataset_count: 5,
                snapshot_count: 10,
            };

            assert_eq!(zfs.pool_health, health);
        }
    }

    // ==================== PERFORMANCE SNAPSHOT TESTS ====================

    #[test]
    fn test_performance_snapshot_creation() {
        let snapshot = PerformanceSnapshot {
            timestamp: Utc::now(),
            cpu: CpuMetrics {
                usage_percent: 50.0,
                load_average_1m: 2.0,
                load_average_5m: 1.8,
                load_average_15m: 1.5,
                core_count: 4,
            },
            memory: MemoryMetrics {
                total_bytes: 8_000_000_000,
                used_bytes: 4_000_000_000,
                available_bytes: 4_000_000_000,
                usage_percent: 50.0,
                swap_used_bytes: 0,
            },
            disk: DiskMetrics {
                read_ops_per_sec: 100.0,
                write_ops_per_sec: 50.0,
                read_bytes_per_sec: 5_000_000.0,
                write_bytes_per_sec: 2_500_000.0,
                avg_queue_depth: 1.5,
            },
            network: NetworkMetrics {
                rx_bytes_per_sec: 500_000.0,
                tx_bytes_per_sec: 250_000.0,
                rx_packets_per_sec: 500.0,
                tx_packets_per_sec: 250.0,
            },
            zfs: ZfsMetrics {
                arc_hit_ratio: 0.9,
                arc_size_bytes: 2_000_000_000,
                l2arc_hit_ratio: 0.8,
                pool_capacity_percent: 40.0,
                pool_health: "ONLINE".to_string(),
                scrub_status: "None".to_string(),
                dataset_count: 5,
                snapshot_count: 15,
            },
        };

        assert_eq!(snapshot.cpu.core_count, 4);
        assert_eq!(snapshot.memory.usage_percent, 50.0);
        assert_eq!(snapshot.zfs.pool_health, "ONLINE");
    }

    // ==================== PERFORMANCE ANALYSIS CONFIG TESTS ====================

    #[test]
    fn test_performance_analysis_config_default() {
        let config = PerformanceAnalysisConfig::default();

        assert!(config.enable_cpu_monitoring);
        assert!(config.enable_memory_monitoring);
        assert!(config.enable_disk_monitoring);
        assert!(config.enable_network_monitoring);
        assert!(config.enable_zfs_monitoring);
        assert_eq!(config.analysis_interval_seconds, 30);
        assert_eq!(config.max_history_entries, 1000);
    }

    #[test]
    fn test_performance_analysis_config_custom() {
        let config = PerformanceAnalysisConfig {
            enable_cpu_monitoring: true,
            enable_memory_monitoring: true,
            enable_disk_monitoring: false,
            enable_network_monitoring: false,
            enable_zfs_monitoring: true,
            analysis_interval_seconds: 60,
            max_history_entries: 500,
        };

        assert!(!config.enable_disk_monitoring);
        assert_eq!(config.analysis_interval_seconds, 60);
    }

    // ==================== COMPONENT ANALYSIS TESTS ====================

    #[test]
    fn test_component_analysis_creation() {
        let analysis = ComponentAnalysis {
            component_name: "CPU".to_string(),
            current_usage: 65.5,
            trend: PerformanceTrend::Stable,
            anomalies: vec!["Spike detected at 14:30".to_string()],
            recommendations: vec!["Consider load balancing".to_string()],
        };

        assert_eq!(analysis.component_name, "CPU");
        assert_eq!(analysis.current_usage, 65.5);
        assert_eq!(analysis.anomalies.len(), 1);
        assert_eq!(analysis.recommendations.len(), 1);
    }

    // ==================== PERFORMANCE TRENDS TESTS ====================

    #[test]
    fn test_performance_trends_all_stable() {
        let trends = PerformanceTrends {
            cpu_trend: PerformanceTrend::Stable,
            memory_trend: PerformanceTrend::Stable,
            disk_trend: PerformanceTrend::Stable,
            network_trend: PerformanceTrend::Stable,
            zfs_trend: PerformanceTrend::Stable,
        };

        // All trends should be stable
        assert!(matches!(trends.cpu_trend, PerformanceTrend::Stable));
        assert!(matches!(trends.memory_trend, PerformanceTrend::Stable));
    }

    #[test]
    fn test_performance_trends_mixed() {
        let trends = PerformanceTrends {
            cpu_trend: PerformanceTrend::Improving,
            memory_trend: PerformanceTrend::Degrading,
            disk_trend: PerformanceTrend::Stable,
            network_trend: PerformanceTrend::Unknown,
            zfs_trend: PerformanceTrend::Improving,
        };

        assert!(matches!(trends.cpu_trend, PerformanceTrend::Improving));
        assert!(matches!(trends.memory_trend, PerformanceTrend::Degrading));
    }

    // ==================== PERFORMANCE RECOMMENDATION TESTS ====================

    #[test]
    fn test_performance_recommendation_creation() {
        let recommendation = PerformanceRecommendation {
            category: "CPU".to_string(),
            description: "Increase CPU allocation".to_string(),
            priority: 8,
            estimated_impact: "High".to_string(),
        };

        assert_eq!(recommendation.priority, 8);
        assert_eq!(recommendation.estimated_impact, "High");
    }

    #[test]
    fn test_performance_recommendation_priority_levels() {
        let priorities = vec![1, 5, 10];

        for priority in priorities {
            let rec = PerformanceRecommendation {
                category: "Test".to_string(),
                description: "Test recommendation".to_string(),
                priority,
                estimated_impact: "Medium".to_string(),
            };

            assert_eq!(rec.priority, priority);
        }
    }

    // ==================== PERFORMANCE ANALYZER STATE TESTS ====================

    #[test]
    fn test_performance_analyzer_state_default() {
        let state = PerformanceAnalyzerState::default();

        assert!(!state.is_running);
        assert_eq!(state.total_analyses, 0);
        assert!(state.last_analysis.is_none());
    }

    #[test]
    fn test_performance_analyzer_state_running() {
        let mut state = PerformanceAnalyzerState::default();
        state.is_running = true;
        state.total_analyses = 5;
        state.last_analysis = Some(Utc::now());

        assert!(state.is_running);
        assert_eq!(state.total_analyses, 5);
        assert!(state.last_analysis.is_some());
    }

    // ==================== CPU ANALYSIS TESTS ====================

    #[test]
    fn test_cpu_analysis_creation() {
        let analysis = CpuAnalysis {
            current_usage: 55.0,
            peak_usage: 85.0,
            average_usage: 60.0,
            trend: PerformanceTrend::Stable,
        };

        assert_eq!(analysis.current_usage, 55.0);
        assert_eq!(analysis.peak_usage, 85.0);
        assert!(matches!(analysis.trend, PerformanceTrend::Stable));
    }

    // ==================== MEMORY ANALYSIS TESTS ====================

    #[test]
    fn test_memory_analysis_creation() {
        let analysis = MemoryAnalysis {
            current_usage_percent: 70.0,
            peak_usage_percent: 90.0,
            average_usage_percent: 75.0,
            trend: PerformanceTrend::Degrading,
        };

        assert_eq!(analysis.current_usage_percent, 70.0);
        assert!(matches!(analysis.trend, PerformanceTrend::Degrading));
    }

    // ==================== DISK ANALYSIS TESTS ====================

    #[test]
    fn test_disk_analysis_creation() {
        let analysis = DiskAnalysis {
            current_io_utilization: 45.0,
            peak_iops: 5000.0,
            average_iops: 3000.0,
            trend: PerformanceTrend::Improving,
        };

        assert_eq!(analysis.peak_iops, 5000.0);
        assert!(matches!(analysis.trend, PerformanceTrend::Improving));
    }

    // ==================== NETWORK ANALYSIS TESTS ====================

    #[test]
    fn test_network_analysis_creation() {
        let analysis = NetworkAnalysis {
            current_utilization: 30.0,
            peak_bandwidth_mbps: 950.0,
            average_bandwidth_mbps: 500.0,
            trend: PerformanceTrend::Stable,
        };

        assert_eq!(analysis.peak_bandwidth_mbps, 950.0);
        assert_eq!(analysis.average_bandwidth_mbps, 500.0);
    }

    // ==================== ZFS ANALYSIS TESTS ====================

    #[test]
    fn test_zfs_analysis_creation() {
        let analysis = ZfsAnalysis {
            current_arc_hit_ratio: 0.92,
            pool_capacity_percent: 55.0,
            pool_health: "ONLINE".to_string(),
            trend: PerformanceTrend::Improving,
        };

        assert_eq!(analysis.current_arc_hit_ratio, 0.92);
        assert_eq!(analysis.pool_health, "ONLINE");
    }

    // ==================== PERFORMANCE ANALYSIS REPORT TESTS ====================

    #[test]
    fn test_performance_analysis_report_creation() {
        let report = PerformanceAnalysisReport {
            generated_at: Utc::now(),
            overall_health_score: 85.0,
            trends: PerformanceTrends {
                cpu_trend: PerformanceTrend::Stable,
                memory_trend: PerformanceTrend::Stable,
                disk_trend: PerformanceTrend::Improving,
                network_trend: PerformanceTrend::Stable,
                zfs_trend: PerformanceTrend::Improving,
            },
            component_analyses: vec![],
            recommendations: vec![],
            critical_issues: vec![],
            warnings: vec![],
            system_uptime_seconds: 86400,
            analysis_period_start: Utc::now(),
            analysis_period_end: Utc::now(),
        };

        assert_eq!(report.overall_health_score, 85.0);
        assert_eq!(report.system_uptime_seconds, 86400);
    }

    #[test]
    fn test_performance_analysis_report_with_issues() {
        let report = PerformanceAnalysisReport {
            generated_at: Utc::now(),
            overall_health_score: 60.0,
            trends: PerformanceTrends {
                cpu_trend: PerformanceTrend::Degrading,
                memory_trend: PerformanceTrend::Degrading,
                disk_trend: PerformanceTrend::Stable,
                network_trend: PerformanceTrend::Stable,
                zfs_trend: PerformanceTrend::Stable,
            },
            component_analyses: vec![],
            recommendations: vec![PerformanceRecommendation {
                category: "CPU".to_string(),
                description: "Reduce load".to_string(),
                priority: 9,
                estimated_impact: "High".to_string(),
            }],
            critical_issues: vec!["High CPU usage detected".to_string()],
            warnings: vec!["Memory usage above 80%".to_string()],
            system_uptime_seconds: 3600,
            analysis_period_start: Utc::now(),
            analysis_period_end: Utc::now(),
        };

        assert_eq!(report.overall_health_score, 60.0);
        assert_eq!(report.critical_issues.len(), 1);
        assert_eq!(report.warnings.len(), 1);
        assert_eq!(report.recommendations.len(), 1);
    }
}
