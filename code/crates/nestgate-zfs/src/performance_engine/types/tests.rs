// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(deprecated)]

use std::collections::HashMap;
use std::time::{Duration, UNIX_EPOCH};

use crate::types::StorageTier;

use super::system_time_serde;
use super::*;

#[derive(serde::Serialize, serde::Deserialize)]
struct TimeWrap {
    #[serde(with = "system_time_serde")]
    t: std::time::SystemTime,
}

#[test]
fn performance_engine_config_default() {
    let c = PerformanceEngineConfig::default();
    assert!(c.enable_ai_guidance);
    assert_eq!(c.max_concurrent_optimizations, 3);
}

#[test]
fn optimization_state_variants() {
    let states = [
        OptimizationState::Idle,
        OptimizationState::Collecting,
        OptimizationState::Analyzing,
        OptimizationState::Optimizing,
        OptimizationState::Validating,
        OptimizationState::Applied,
    ];
    assert_eq!(states[0], OptimizationState::default());
    assert_ne!(states[1], states[0]);
}

#[test]
fn performance_optimization_result_merge_with() {
    let mut a = PerformanceOptimizationResult {
        applied_optimizations: vec![AppliedOptimization {
            optimization_type: OptimizationType::CacheOptimization,
            description: "a".to_string(),
            performance_impact: 1.0,
            applied_at: UNIX_EPOCH + Duration::from_secs(1),
        }],
        performance_improvement: 2.0,
        bottlenecks_resolved: vec![],
        recommendations: vec!["r".to_string()],
    };
    let b = PerformanceOptimizationResult {
        applied_optimizations: vec![AppliedOptimization {
            optimization_type: OptimizationType::ArcTuning,
            description: "b".to_string(),
            performance_impact: 3.0,
            applied_at: UNIX_EPOCH + Duration::from_secs(2),
        }],
        performance_improvement: 4.0,
        bottlenecks_resolved: vec![ZfsBottleneck {
            bottleneck_type: ZfsBottleneckType::HighLatency,
            severity: BottleneckSeverity::Low,
            pool_name: "p".to_string(),
            dataset_name: None,
            description: "d".to_string(),
            impact_score: 0.1,
        }],
        recommendations: vec!["x".to_string()],
    };
    a.merge_with(b);
    assert_eq!(a.applied_optimizations.len(), 2);
    assert!((a.performance_improvement - 6.0).abs() < f64::EPSILON);
    assert_eq!(a.bottlenecks_resolved.len(), 1);
    assert_eq!(a.recommendations.len(), 2);
}

#[test]
fn alert_response_and_zfs_tuning_result_default() {
    let a = AlertResponse::default();
    assert!(!a.mitigation_applied);
    let z = ZfsTuningResult::default();
    assert!(!z.tuning_applied);
}

#[test]
fn system_time_serde_roundtrip_and_epoch_fallback() {
    let t = UNIX_EPOCH + Duration::from_secs(12345);
    let w = TimeWrap { t };
    let v = serde_json::to_value(&w).expect("ser");
    let back: TimeWrap = serde_json::from_value(v).expect("de");
    assert_eq!(back.t, w.t);

    let before_epoch = TimeWrap {
        t: UNIX_EPOCH - Duration::from_secs(10),
    };
    let v2 = serde_json::to_value(&before_epoch).expect("ser");
    let back2: TimeWrap = serde_json::from_value(v2).expect("de");
    assert_eq!(back2.t, UNIX_EPOCH);
}

#[test]
fn system_time_serde_deserialize_invalid_type_errors() {
    let err = serde_json::from_str::<TimeWrap>(r#"{"t":null}"#);
    assert!(err.is_err());
}

#[test]
fn serde_roundtrip_core_metrics_and_enums() {
    let m = ZfsPerformanceMetrics {
        timestamp: UNIX_EPOCH,
        pool_metrics: HashMap::from([(
            "tank".to_string(),
            ZfsPoolMetrics {
                pool_name: "tank".to_string(),
                read_ops: 1.0,
                write_ops: 2.0,
                read_bandwidth: 3.0,
                write_bandwidth: 4.0,
                latency: 5.0,
                cache_hit_ratio: 0.5,
                fragmentation: 10.0,
            },
        )]),
        dataset_metrics: HashMap::from([(
            "tank/d".to_string(),
            ZfsDatasetMetrics {
                dataset_name: "tank/d".to_string(),
                access_pattern: AccessPattern::Sequential,
                dedup_ratio: 1.0,
                record_size: 4096,
            },
        )]),
        system_memory: SystemMemoryUsage {
            total: 1,
            available: 1,
            used: 0,
        },
        arc_stats: ArcStatistics {
            size: 1,
            target_size: 2,
            hit_ratio: 0.9,
            miss_ratio: 0.1,
        },
    };
    let j = serde_json::to_string(&m).expect("ser");
    let _: ZfsPerformanceMetrics = serde_json::from_str(&j).expect("de");

    let patterns = [
        AccessPattern::Sequential,
        AccessPattern::Random,
        AccessPattern::Mixed,
    ];
    for p in patterns {
        let v = serde_json::to_string(&p).unwrap();
        let _: AccessPattern = serde_json::from_str(&v).unwrap();
    }

    let bottlenecks = [
        ZfsBottleneckType::HighLatency,
        ZfsBottleneckType::LowThroughput,
        ZfsBottleneckType::CacheMiss,
        ZfsBottleneckType::Fragmentation,
        ZfsBottleneckType::MemoryPressure,
        ZfsBottleneckType::CpuUtilization,
        ZfsBottleneckType::NetworkBandwidth,
        ZfsBottleneckType::DiskIo,
    ];
    for b in bottlenecks {
        let v = serde_json::to_string(&b).unwrap();
        let _: ZfsBottleneckType = serde_json::from_str(&v).unwrap();
    }

    let sev = [
        BottleneckSeverity::Low,
        BottleneckSeverity::Medium,
        BottleneckSeverity::High,
        BottleneckSeverity::Critical,
    ];
    for s in sev {
        let v = serde_json::to_string(&s).unwrap();
        let _: BottleneckSeverity = serde_json::from_str(&v).unwrap();
    }
}

#[test]
fn serde_performance_alert_and_strategies() {
    let alert = PerformanceAlert {
        alert_type: AlertType::BottleneckDetected,
        severity: AlertSeverity::Warning,
        pool_name: "p".to_string(),
        dataset_name: Some("d".to_string()),
        description: "x".to_string(),
        timestamp: UNIX_EPOCH,
    };
    let j = serde_json::to_string(&alert).expect("ser");
    let _: PerformanceAlert = serde_json::from_str(&j).expect("de");

    let strategies = [
        EcosystemOptimizationStrategy::LatencyOptimization,
        EcosystemOptimizationStrategy::ThroughputOptimization,
        EcosystemOptimizationStrategy::BalancedOptimization,
        EcosystemOptimizationStrategy::CustomStrategy("c".to_string()),
    ];
    for s in strategies {
        let v = serde_json::to_string(&s).unwrap();
        let _: EcosystemOptimizationStrategy = serde_json::from_str(&v).unwrap();
    }

    let alert_types = [
        AlertType::PerformanceDegradation,
        AlertType::BottleneckDetected,
        AlertType::ThresholdExceeded,
        AlertType::OptimizationFailed,
    ];
    for a in alert_types {
        let v = serde_json::to_string(&a).unwrap();
        let _: AlertType = serde_json::from_str(&v).unwrap();
    }

    let sev = [
        AlertSeverity::Info,
        AlertSeverity::Warning,
        AlertSeverity::Error,
        AlertSeverity::Critical,
    ];
    for s in sev {
        let v = serde_json::to_string(&s).unwrap();
        let _: AlertSeverity = serde_json::from_str(&v).unwrap();
    }
}

#[test]
fn serde_workload_and_tuning_requests() {
    let wp = WorkloadPattern {
        access_pattern: AccessPattern::Random,
        io_size_distribution: HashMap::from([("4k".to_string(), 0.5)]),
        read_write_ratio: 1.0,
        temporal_locality: 0.25,
    };
    let j = serde_json::to_string(&wp).expect("ser");
    let _: WorkloadPattern = serde_json::from_str(&j).expect("de");

    let caps = SystemCapabilities {
        cpu_cores: 8,
        memory_gb: 16,
        storage_type: "NVMe".to_string(),
        network_bandwidth_gbps: 10.0,
    };
    let ctx = ZfsConfigurationContext {
        pool_name: "p".to_string(),
        dataset_name: None,
        current_configuration: HashMap::from([("key".to_string(), "val".to_string())]),
        workload_pattern: wp.clone(),
        system_capabilities: caps.clone(),
    };
    let zpm = ZfsPerformanceMetrics {
        timestamp: UNIX_EPOCH,
        pool_metrics: HashMap::new(),
        dataset_metrics: HashMap::new(),
        system_memory: SystemMemoryUsage {
            total: 1,
            available: 1,
            used: 0,
        },
        arc_stats: ArcStatistics {
            size: 1,
            target_size: 1,
            hit_ratio: 0.5,
            miss_ratio: 0.5,
        },
    };
    let expert = ZfsExpertiseContext {
        storage_tier: StorageTier::Hot,
        access_patterns: vec![AccessPattern::Mixed],
        current_performance: zpm.clone(),
        identified_bottlenecks: vec![],
    };
    let req = ZfsTuningRequest {
        pool_name: "p".to_string(),
        dataset_name: None,
        workload_pattern: wp,
        current_configuration: HashMap::new(),
        performance_goals: vec!["latency".to_string()],
        system_capabilities: caps,
        configuration_context: ctx.clone(),
        expertise_context: expert,
    };
    let j = serde_json::to_string(&req).expect("ser");
    let _: ZfsTuningRequest = serde_json::from_str(&j).expect("de");

    let po = PerformanceOptimizationRequest {
        pool_name: "p".to_string(),
        dataset_name: None,
        optimization_strategy: EcosystemOptimizationStrategy::BalancedOptimization,
        current_metrics: zpm,
        configuration_context: ctx,
    };
    let j = serde_json::to_string(&po).expect("ser");
    let _: PerformanceOptimizationRequest = serde_json::from_str(&j).expect("de");
}

#[test]
fn serde_ai_and_ecosystem_tuning_and_alert_analysis() {
    let ai = AiOptimizationRecommendation {
        strategy: EcosystemOptimizationStrategy::LatencyOptimization,
        confidence_score: 0.9,
        expected_improvement: 12.0,
        parameter_recommendations: HashMap::from([("a".to_string(), "b".to_string())]),
        risk_assessment: "low".to_string(),
    };
    let j = serde_json::to_string(&ai).expect("ser");
    let _: AiOptimizationRecommendation = serde_json::from_str(&j).expect("de");

    let eco = EcosystemTuningRecommendations {
        recommendations: vec![ZfsTuningRecommendation {
            parameter: "recordsize".to_string(),
            recommendedvalue: "128k".to_string(),
            confidence: 0.8,
            expected_impact: 5.0,
        }],
        overall_confidence: 0.8,
        estimated_improvement: 5.0,
    };
    let j = serde_json::to_string(&eco).expect("ser");
    let _: EcosystemTuningRecommendations = serde_json::from_str(&j).expect("de");

    let analysis = EcosystemAlertAnalysis {
        root_cause_analysis: "cpu".to_string(),
        recommended_actions: vec!["tune".to_string()],
        confidence_score: 0.7,
        urgency_level: AlertSeverity::Error,
    };
    let j = serde_json::to_string(&analysis).expect("ser");
    let _: EcosystemAlertAnalysis = serde_json::from_str(&j).expect("de");

    let par = PerformanceAlertAnalysisRequest {
        alert: PerformanceAlert {
            alert_type: AlertType::ThresholdExceeded,
            severity: AlertSeverity::Info,
            pool_name: "p".to_string(),
            dataset_name: None,
            description: "d".to_string(),
            timestamp: UNIX_EPOCH,
        },
        historical_metrics: vec![],
        current_configuration: HashMap::new(),
        system_capabilities: SystemCapabilities {
            cpu_cores: 1,
            memory_gb: 1,
            storage_type: "HDD".to_string(),
            network_bandwidth_gbps: 1.0,
        },
    };
    let j = serde_json::to_string(&par).expect("ser");
    let _: PerformanceAlertAnalysisRequest = serde_json::from_str(&j).expect("de");
}

#[test]
fn optimization_type_debug_covers_all_variants() {
    let opts = [
        OptimizationType::CacheOptimization,
        OptimizationType::LatencyOptimization,
        OptimizationType::ThroughputOptimization,
        OptimizationType::FragmentationDefrag,
        OptimizationType::ArcTuning,
        OptimizationType::RecordSizeOptimization,
        OptimizationType::CompressionOptimization,
    ];
    let joined = opts
        .iter()
        .map(|o| format!("{o:?}"))
        .collect::<Vec<_>>()
        .join(" ");
    assert!(joined.contains("Cache"));
    assert!(joined.contains("Compression"));
}
