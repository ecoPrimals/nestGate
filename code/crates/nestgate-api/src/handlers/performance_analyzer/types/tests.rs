// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use super::*;
use chrono::Utc;

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
