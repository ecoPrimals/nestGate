// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Comprehensive tests for performance analyzer types
//!
//! These tests increase coverage by testing data structures, serialization,
//! and type behavior.

#[cfg(test)]
mod tests {
    use super::super::types::*;
    use chrono::Utc;

    // ==================== PERFORMANCE TREND TESTS ====================

    #[test]
    fn test_performance_trend_variants() {
        let improving = PerformanceTrend::Improving;
        let stable = PerformanceTrend::Stable;
        let degrading = PerformanceTrend::Degrading;
        let unknown = PerformanceTrend::Unknown;

        assert!(matches!(improving, PerformanceTrend::Improving));
        assert!(matches!(stable, PerformanceTrend::Stable));
        assert!(matches!(degrading, PerformanceTrend::Degrading));
        assert!(matches!(unknown, PerformanceTrend::Unknown));
    }

    #[test]
    fn test_performance_trend_serialization() {
        let trend = PerformanceTrend::Improving;
        let json = serde_json::to_string(&trend).expect("Test setup failed");
        assert!(!json.is_empty());

        let deserialized: PerformanceTrend =
            serde_json::from_str(&json).expect("Test setup failed");
        assert!(matches!(deserialized, PerformanceTrend::Improving));
    }

    #[test]
    fn test_performance_trend_clone() {
        let trend = PerformanceTrend::Stable;
        let cloned = trend;
        assert!(matches!(cloned, PerformanceTrend::Stable));
    }

    // ==================== COMPONENT ANALYSIS TESTS ====================

    #[test]
    fn test_component_analysis_creation() {
        let analysis = ComponentAnalysis {
            component_name: "CPU".to_string(),
            current_usage: 75.5,
            trend: PerformanceTrend::Stable,
            anomalies: vec!["High load spike".to_string()],
            recommendations: vec!["Scale up".to_string()],
        };

        assert_eq!(analysis.component_name, "CPU");
        assert_eq!(analysis.current_usage, 75.5);
        assert!(matches!(analysis.trend, PerformanceTrend::Stable));
        assert_eq!(analysis.anomalies.len(), 1);
        assert_eq!(analysis.recommendations.len(), 1);
    }

    #[test]
    fn test_component_analysis_serialization() {
        let analysis = ComponentAnalysis {
            component_name: "Memory".to_string(),
            current_usage: 82.3,
            trend: PerformanceTrend::Degrading,
            anomalies: vec![],
            recommendations: vec!["Add RAM".to_string()],
        };

        let json = serde_json::to_string(&analysis).expect("Test setup failed");
        assert!(json.contains("Memory"));
        assert!(json.contains("82.3"));

        let deserialized: ComponentAnalysis =
            serde_json::from_str(&json).expect("Test setup failed");
        assert_eq!(deserialized.component_name, "Memory");
        assert_eq!(deserialized.current_usage, 82.3);
    }

    #[test]
    fn test_component_analysis_clone() {
        let analysis = ComponentAnalysis {
            component_name: "Disk".to_string(),
            current_usage: 60.0,
            trend: PerformanceTrend::Improving,
            anomalies: vec![],
            recommendations: vec![],
        };

        let cloned = analysis.clone();
        assert_eq!(cloned.component_name, analysis.component_name);
        assert_eq!(cloned.current_usage, analysis.current_usage);
    }

    // ==================== CPU METRICS TESTS ====================

    #[test]
    fn test_cpu_metrics_creation() {
        let metrics = CpuMetrics {
            usage_percent: 45.5,
            load_average_1m: 2.5,
            load_average_5m: 2.1,
            load_average_15m: 1.9,
            core_count: 8,
        };

        assert_eq!(metrics.usage_percent, 45.5);
        assert_eq!(metrics.core_count, 8);
    }

    #[test]
    fn test_cpu_metrics_serialization() {
        let metrics = CpuMetrics {
            usage_percent: 50.0,
            load_average_1m: 1.0,
            load_average_5m: 1.2,
            load_average_15m: 1.1,
            core_count: 4,
        };

        let json = serde_json::to_string(&metrics).expect("Test setup failed");
        assert!(json.contains("50"));
        assert!(json.contains("\"core_count\":4"));
    }

    // ==================== MEMORY METRICS TESTS ====================

    #[test]
    fn test_memory_metrics_creation() {
        let metrics = MemoryMetrics {
            total_bytes: 16_000_000_000,
            used_bytes: 8_000_000_000,
            available_bytes: 8_000_000_000,
            usage_percent: 50.0,
            swap_used_bytes: 0,
        };

        assert_eq!(metrics.total_bytes, 16_000_000_000);
        assert_eq!(metrics.usage_percent, 50.0);
    }

    #[test]
    fn test_memory_metrics_serialization() {
        let metrics = MemoryMetrics {
            total_bytes: 8_000_000_000,
            used_bytes: 4_000_000_000,
            available_bytes: 4_000_000_000,
            usage_percent: 50.0,
            swap_used_bytes: 0,
        };

        let json = serde_json::to_string(&metrics).expect("Test setup failed");
        let deserialized: MemoryMetrics = serde_json::from_str(&json).expect("Test setup failed");
        assert_eq!(deserialized.total_bytes, metrics.total_bytes);
    }

    // ==================== DISK METRICS TESTS ====================

    #[test]
    fn test_disk_metrics_creation() {
        let metrics = DiskMetrics {
            read_ops_per_sec: 100.0,
            write_ops_per_sec: 50.0,
            read_bytes_per_sec: 1_000_000.0,
            write_bytes_per_sec: 500_000.0,
            avg_queue_depth: 2.5,
        };

        assert_eq!(metrics.read_ops_per_sec, 100.0);
        assert_eq!(metrics.avg_queue_depth, 2.5);
    }

    // ==================== NETWORK METRICS TESTS ====================

    #[test]
    fn test_network_metrics_creation() {
        let metrics = NetworkMetrics {
            rx_bytes_per_sec: 1_000_000.0,
            tx_bytes_per_sec: 500_000.0,
            rx_packets_per_sec: 1000.0,
            tx_packets_per_sec: 500.0,
        };

        assert_eq!(metrics.rx_bytes_per_sec, 1_000_000.0);
        assert_eq!(metrics.tx_packets_per_sec, 500.0);
    }

    // ==================== ZFS METRICS TESTS ====================

    #[test]
    fn test_zfs_metrics_creation() {
        let metrics = ZfsMetrics {
            arc_hit_ratio: 0.95,
            arc_size_bytes: 4_000_000_000,
            l2arc_hit_ratio: 0.85,
            pool_capacity_percent: 60.0,
            pool_health: "ONLINE".to_string(),
            scrub_status: "completed".to_string(),
            dataset_count: 10,
            snapshot_count: 50,
        };

        assert_eq!(metrics.arc_hit_ratio, 0.95);
        assert_eq!(metrics.pool_health, "ONLINE");
        assert_eq!(metrics.dataset_count, 10);
    }

    #[test]
    fn test_zfs_metrics_serialization() {
        let metrics = ZfsMetrics {
            arc_hit_ratio: 0.90,
            arc_size_bytes: 2_000_000_000,
            l2arc_hit_ratio: 0.80,
            pool_capacity_percent: 50.0,
            pool_health: "ONLINE".to_string(),
            scrub_status: "none requested".to_string(),
            dataset_count: 5,
            snapshot_count: 20,
        };

        let json = serde_json::to_string(&metrics).expect("Test setup failed");
        assert!(json.contains("ONLINE"));
        assert!(json.contains("0.9"));
    }

    // ==================== PERFORMANCE SNAPSHOT TESTS ====================

    #[test]
    fn test_performance_snapshot_creation() {
        let snapshot = PerformanceSnapshot {
            timestamp: Utc::now(),
            cpu: CpuMetrics {
                usage_percent: 50.0,
                load_average_1m: 1.0,
                load_average_5m: 1.0,
                load_average_15m: 1.0,
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
                read_bytes_per_sec: 1_000_000.0,
                write_bytes_per_sec: 500_000.0,
                avg_queue_depth: 2.0,
            },
            network: NetworkMetrics {
                rx_bytes_per_sec: 1_000_000.0,
                tx_bytes_per_sec: 500_000.0,
                rx_packets_per_sec: 1000.0,
                tx_packets_per_sec: 500.0,
            },
            zfs: ZfsMetrics {
                arc_hit_ratio: 0.95,
                arc_size_bytes: 2_000_000_000,
                l2arc_hit_ratio: 0.85,
                pool_capacity_percent: 50.0,
                pool_health: "ONLINE".to_string(),
                scrub_status: "completed".to_string(),
                dataset_count: 5,
                snapshot_count: 20,
            },
        };

        assert_eq!(snapshot.cpu.usage_percent, 50.0);
        assert_eq!(snapshot.memory.usage_percent, 50.0);
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

        assert!(config.enable_cpu_monitoring);
        assert!(!config.enable_disk_monitoring);
        assert_eq!(config.analysis_interval_seconds, 60);
    }

    #[test]
    fn test_performance_analysis_config_serialization() {
        let config = PerformanceAnalysisConfig::default();
        let json = serde_json::to_string(&config).expect("Test setup failed");
        let deserialized: PerformanceAnalysisConfig =
            serde_json::from_str(&json).expect("Test setup failed");

        assert_eq!(
            deserialized.analysis_interval_seconds,
            config.analysis_interval_seconds
        );
    }

    // ==================== PERFORMANCE RECOMMENDATION TESTS ====================

    #[test]
    fn test_performance_recommendation_creation() {
        let rec = PerformanceRecommendation {
            category: "CPU".to_string(),
            description: "Upgrade CPU".to_string(),
            priority: 8,
            estimated_impact: "High".to_string(),
        };

        assert_eq!(rec.category, "CPU");
        assert_eq!(rec.priority, 8);
    }

    #[test]
    fn test_performance_recommendation_serialization() {
        let rec = PerformanceRecommendation {
            category: "Memory".to_string(),
            description: "Add more RAM".to_string(),
            priority: 9,
            estimated_impact: "Very High".to_string(),
        };

        let json = serde_json::to_string(&rec).expect("Test setup failed");
        assert!(json.contains("Memory"));
        assert!(json.contains("\"priority\":9"));
    }

    // ==================== PERFORMANCE TRENDS TESTS ====================

    #[test]
    fn test_performance_trends_creation() {
        let trends = PerformanceTrends {
            cpu_trend: PerformanceTrend::Stable,
            memory_trend: PerformanceTrend::Improving,
            disk_trend: PerformanceTrend::Stable,
            network_trend: PerformanceTrend::Stable,
            zfs_trend: PerformanceTrend::Improving,
        };

        assert!(matches!(trends.cpu_trend, PerformanceTrend::Stable));
        assert!(matches!(trends.memory_trend, PerformanceTrend::Improving));
    }

    // ==================== PERFORMANCE ANALYZER STATE TESTS ====================

    #[test]
    fn test_performance_analyzer_state_default() {
        let state = PerformanceAnalyzerState::default();

        assert!(!state.is_running);
        assert!(state.last_analysis.is_none());
        assert_eq!(state.total_analyses, 0);
    }

    #[test]
    fn test_performance_analyzer_state_creation() {
        let state = PerformanceAnalyzerState {
            is_running: true,
            last_analysis: Some(Utc::now()),
            total_analyses: 42,
            config: PerformanceAnalysisConfig::default(),
        };

        assert!(state.is_running);
        assert!(state.last_analysis.is_some());
        assert_eq!(state.total_analyses, 42);
    }

    // ==================== CPU ANALYSIS TESTS ====================

    #[test]
    fn test_cpu_analysis_creation() {
        let analysis = CpuAnalysis {
            current_usage: 75.0,
            peak_usage: 95.0,
            average_usage: 65.0,
            trend: PerformanceTrend::Stable,
        };

        assert_eq!(analysis.current_usage, 75.0);
        assert_eq!(analysis.peak_usage, 95.0);
    }

    // ==================== MEMORY ANALYSIS TESTS ====================

    #[test]
    fn test_memory_analysis_creation() {
        let analysis = MemoryAnalysis {
            current_usage_percent: 70.0,
            peak_usage_percent: 85.0,
            average_usage_percent: 65.0,
            trend: PerformanceTrend::Stable,
        };

        assert_eq!(analysis.current_usage_percent, 70.0);
    }

    // ==================== DISK ANALYSIS TESTS ====================

    #[test]
    fn test_disk_analysis_creation() {
        let analysis = DiskAnalysis {
            current_io_utilization: 50.0,
            peak_iops: 1000.0,
            average_iops: 500.0,
            trend: PerformanceTrend::Stable,
        };

        assert_eq!(analysis.current_io_utilization, 50.0);
    }

    // ==================== NETWORK ANALYSIS TESTS ====================

    #[test]
    fn test_network_analysis_creation() {
        let analysis = NetworkAnalysis {
            current_utilization: 40.0,
            peak_bandwidth_mbps: 900.0,
            average_bandwidth_mbps: 500.0,
            trend: PerformanceTrend::Stable,
        };

        assert_eq!(analysis.current_utilization, 40.0);
    }

    // ==================== ZFS ANALYSIS TESTS ====================

    #[test]
    fn test_zfs_analysis_creation() {
        let analysis = ZfsAnalysis {
            current_arc_hit_ratio: 0.95,
            pool_capacity_percent: 60.0,
            pool_health: "ONLINE".to_string(),
            trend: PerformanceTrend::Stable,
        };

        assert_eq!(analysis.current_arc_hit_ratio, 0.95);
        assert_eq!(analysis.pool_health, "ONLINE");
    }

    #[test]
    fn test_zfs_analysis_serialization() {
        let analysis = ZfsAnalysis {
            current_arc_hit_ratio: 0.90,
            pool_capacity_percent: 50.0,
            pool_health: "ONLINE".to_string(),
            trend: PerformanceTrend::Improving,
        };

        let json = serde_json::to_string(&analysis).expect("Test setup failed");
        assert!(json.contains("ONLINE"));
    }
}
