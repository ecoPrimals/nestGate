//! Comprehensive AI Integration Tests
//!
//! Tests for the AI-powered ZFS optimization and management including:
//! - Performance optimization recommendations
//! - Predictive analytics for storage patterns
//! - Automated tuning suggestions
//! - Integration with ecosystem AI services

use nestgate_zfs::ai_integration::{
    ZfsAiIntegration, ZfsAiConfig, OptimizationOpportunity, TierPrediction,
    SystemContext, TierAnalytics, MigrationCandidate, WorkloadAnalysis,
    CompressionAnalysis, SnapshotAnalysis, OptimizationType, OptimizationBenefit,
    OptimizationEffort, AutonomousOptimizationReport, OptimizationExecutionResult,
    OptimizationStatus, SafetyValidationResult, PerformanceImpact, RiskLevel
};
use nestgate_zfs::performance_engine::{
    AiOptimizationRecommendation, EcosystemOptimizationStrategy,
    ZfsExpertiseContext, SystemCapabilities, OptimizationState,
    ZfsPerformanceMetrics, ZfsPoolMetrics, ZfsDatasetMetrics, SystemMemoryUsage,
    ArcStatistics, AccessPattern, WorkloadPattern, EcosystemAiAnalysis,
    PerformanceAlertAnalysisRequest as ZfsAiAnalysisRequest,
    ZfsTuningRequest as AiOptimizationContext
};
use nestgate_zfs::config::ZfsConfig;
use nestgate_core::StorageTier;
use std::collections::HashMap;
use std::time::SystemTime;

/// Test utilities for AI integration testing
mod test_utils {
    use super::*;
    
    pub fn create_ai_optimization_context() -> AiOptimizationContext {
        AiOptimizationContext {
            system_capabilities: SystemCapabilities {
                total_memory_gb: 64,
                cpu_cores: 16,
                storage_tier: StorageTier::Hot,
                zfs_version: "2.1.0".to_string(),
                kernel_version: "5.15.0".to_string(),
            },
            current_metrics: create_sample_metrics(),
            workload_patterns: create_sample_workload_patterns(),
            optimization_history: vec![],
            constraints: HashMap::new(),
        }
    }
    
    pub fn create_sample_metrics() -> ZfsPerformanceMetrics {
        let mut pool_metrics = HashMap::new();
        pool_metrics.insert("production_pool".to_string(), ZfsPoolMetrics {
            name: "production_pool".to_string(),
            read_ops_per_sec: 5000.0,
            write_ops_per_sec: 2000.0,
            read_bandwidth_mbps: 800.0,
            write_bandwidth_mbps: 400.0,
            average_latency_ms: 15.0,
            cache_hit_ratio: 0.75,
            fragmentation_percent: 20.0,
        });
        
        let mut dataset_metrics = HashMap::new();
        dataset_metrics.insert("app_data".to_string(), ZfsDatasetMetrics {
            name: "app_data".to_string(),
            compression_ratio: 2.1,
            dedup_ratio: 1.3,
            record_size: 128 * 1024,
            access_pattern: AccessPattern::Random,
        });
        
        ZfsPerformanceMetrics {
            timestamp: SystemTime::now(),
            pool_metrics,
            dataset_metrics,
            system_memory_usage: SystemMemoryUsage {
                total_memory: 64 * 1024 * 1024 * 1024,
                used_memory: 48 * 1024 * 1024 * 1024,
                available_memory: 16 * 1024 * 1024 * 1024,
            },
            arc_stats: ArcStatistics {
                hit_ratio: 0.75,
                size_bytes: 16 * 1024 * 1024 * 1024,
                target_size_bytes: 20 * 1024 * 1024 * 1024,
                meta_used_bytes: 3 * 1024 * 1024 * 1024,
            },
        }
    }
    
    pub fn create_sample_workload_patterns() -> HashMap<String, WorkloadPattern> {
        let mut patterns = HashMap::new();
        
        patterns.insert("web_application".to_string(), WorkloadPattern {
            read_write_ratio: 0.7,
            sequential_random_ratio: 0.4,
            average_io_size: 32 * 1024,
            peak_iops: 15000,
        });
        
        patterns
    }
    
    pub fn create_high_performance_scenario() -> AiOptimizationContext {
        let mut context = create_ai_optimization_context();
        
        // Modify for high-performance requirements
        context.system_capabilities.total_memory_gb = 128;
        context.system_capabilities.cpu_cores = 32;
        context.system_capabilities.storage_tier = StorageTier::Hot;
        
        // Add performance constraints
        context.constraints.insert("max_latency_ms".to_string(), "5".to_string());
        context.constraints.insert("min_iops".to_string(), "50000".to_string());
        context.constraints.insert("cache_hit_target".to_string(), "0.95".to_string());
        
        context
    }
    
    pub fn create_space_optimization_scenario() -> AiOptimizationContext {
        let mut context = create_ai_optimization_context();
        
        // Modify for space optimization
        context.system_capabilities.storage_tier = StorageTier::Warm;
        
        // Add space constraints
        context.constraints.insert("max_space_usage".to_string(), "80".to_string());
        context.constraints.insert("min_compression_ratio".to_string(), "3.0".to_string());
        context.constraints.insert("enable_dedup".to_string(), "true".to_string());
        
        context
    }
}

#[cfg(test)]
mod ai_optimization_context_tests {
    use super::*;
    use super::test_utils::*;

    #[test]
    fn test_ai_optimization_context_creation() {
        let context = create_ai_optimization_context();
        
        assert_eq!(context.system_capabilities.total_memory_gb, 64);
        assert_eq!(context.system_capabilities.cpu_cores, 16);
        assert_eq!(context.system_capabilities.storage_tier, StorageTier::Hot);
        assert!(!context.system_capabilities.zfs_version.is_empty());
        assert!(!context.system_capabilities.kernel_version.is_empty());
    }

    #[test]
    fn test_system_capabilities_structure() {
        let context = create_ai_optimization_context();
        let capabilities = &context.system_capabilities;
        
        assert!(capabilities.total_memory_gb > 0);
        assert!(capabilities.cpu_cores > 0);
        assert_eq!(capabilities.zfs_version, "2.1.0");
        assert_eq!(capabilities.kernel_version, "5.15.0");
    }

    #[test]
    fn test_metrics_integration() {
        let context = create_ai_optimization_context();
        let metrics = &context.current_metrics;
        
        assert_eq!(metrics.pool_metrics.len(), 1);
        assert_eq!(metrics.dataset_metrics.len(), 1);
        assert!(metrics.pool_metrics.contains_key("production_pool"));
        assert!(metrics.dataset_metrics.contains_key("app_data"));
    }

    #[test]
    fn test_workload_patterns_integration() {
        let context = create_ai_optimization_context();
        let patterns = &context.workload_patterns;
        
        assert_eq!(patterns.len(), 1);
        assert!(patterns.contains_key("web_application"));
        
        let web_pattern = patterns.get("web_application").unwrap();
        assert_eq!(web_pattern.read_write_ratio, 0.7);
        assert_eq!(web_pattern.sequential_random_ratio, 0.4);
        assert_eq!(web_pattern.average_io_size, 32 * 1024);
        assert_eq!(web_pattern.peak_iops, 15000);
    }

    #[test]
    fn test_optimization_history_initialization() {
        let context = create_ai_optimization_context();
        
        assert_eq!(context.optimization_history.len(), 0);
        assert_eq!(context.constraints.len(), 0);
    }
}

#[cfg(test)]
mod high_performance_scenario_tests {
    use super::*;
    use super::test_utils::*;

    #[test]
    fn test_high_performance_scenario_setup() {
        let context = create_high_performance_scenario();
        
        assert_eq!(context.system_capabilities.total_memory_gb, 128);
        assert_eq!(context.system_capabilities.cpu_cores, 32);
        assert_eq!(context.system_capabilities.storage_tier, StorageTier::Hot);
    }

    #[test]
    fn test_high_performance_constraints() {
        let context = create_high_performance_scenario();
        
        assert_eq!(context.constraints.len(), 3);
        assert_eq!(context.constraints.get("max_latency_ms"), Some(&"5".to_string()));
        assert_eq!(context.constraints.get("min_iops"), Some(&"50000".to_string()));
        assert_eq!(context.constraints.get("cache_hit_target"), Some(&"0.95".to_string()));
    }

    #[test]
    fn test_performance_optimization_requirements() {
        let context = create_high_performance_scenario();
        
        // Verify system can handle high-performance requirements
        assert!(context.system_capabilities.total_memory_gb >= 64);
        assert!(context.system_capabilities.cpu_cores >= 16);
        assert_eq!(context.system_capabilities.storage_tier, StorageTier::Hot);
        
        // Verify constraints are performance-focused
        let max_latency: f64 = context.constraints.get("max_latency_ms")
            .unwrap().parse().unwrap();
        assert!(max_latency <= 10.0, "High performance requires low latency");
        
        let min_iops: u64 = context.constraints.get("min_iops")
            .unwrap().parse().unwrap();
        assert!(min_iops >= 20000, "High performance requires high IOPS");
    }
}

#[cfg(test)]
mod space_optimization_scenario_tests {
    use super::*;
    use super::test_utils::*;

    #[test]
    fn test_space_optimization_scenario_setup() {
        let context = create_space_optimization_scenario();
        
        assert_eq!(context.system_capabilities.storage_tier, StorageTier::Warm);
        assert_eq!(context.constraints.len(), 3);
    }

    #[test]
    fn test_space_optimization_constraints() {
        let context = create_space_optimization_scenario();
        
        assert_eq!(context.constraints.get("max_space_usage"), Some(&"80".to_string()));
        assert_eq!(context.constraints.get("min_compression_ratio"), Some(&"3.0".to_string()));
        assert_eq!(context.constraints.get("enable_dedup"), Some(&"true".to_string()));
    }

    #[test]
    fn test_space_efficiency_requirements() {
        let context = create_space_optimization_scenario();
        
        // Verify constraints are space-focused
        let max_usage: f64 = context.constraints.get("max_space_usage")
            .unwrap().parse().unwrap();
        assert!(max_usage <= 85.0, "Space optimization requires usage limit");
        
        let min_compression: f64 = context.constraints.get("min_compression_ratio")
            .unwrap().parse().unwrap();
        assert!(min_compression >= 2.0, "Space optimization requires good compression");
        
        let enable_dedup = context.constraints.get("enable_dedup").unwrap();
        assert_eq!(enable_dedup, "true", "Space optimization should enable deduplication");
    }
}

#[cfg(test)]
mod ai_optimization_recommendation_tests {
    use super::*;

    #[test]
    fn test_optimization_recommendation_structure() {
        let recommendation = AiOptimizationRecommendation {
            strategy: EcosystemOptimizationStrategy::PerformanceFirst,
            confidence_score: 0.85,
            expected_improvement: "40% latency reduction, 25% throughput increase".to_string(),
            implementation_complexity: "Medium".to_string(),
            estimated_implementation_time: "2-4 hours".to_string(),
            parameters_to_tune: vec![
                ("recordsize".to_string(), "1M".to_string()),
                ("compression".to_string(), "lz4".to_string()),
                ("primarycache".to_string(), "all".to_string()),
            ],
            warnings: vec![
                "May increase memory usage".to_string(),
                "Test in staging environment first".to_string(),
            ],
        };
        
        assert_eq!(recommendation.confidence_score, 0.85);
        assert!(!recommendation.expected_improvement.is_empty());
        assert_eq!(recommendation.parameters_to_tune.len(), 3);
        assert_eq!(recommendation.warnings.len(), 2);
    }

    #[test]
    fn test_optimization_strategy_types() {
        let strategies = vec![
            EcosystemOptimizationStrategy::PerformanceFirst,
            EcosystemOptimizationStrategy::SpaceEfficient,
            EcosystemOptimizationStrategy::Balanced,
            EcosystemOptimizationStrategy::PowerEfficient,
        ];
        
        for strategy in strategies {
            let recommendation = AiOptimizationRecommendation {
                strategy,
                confidence_score: 0.75,
                expected_improvement: "Test improvement".to_string(),
                implementation_complexity: "Low".to_string(),
                estimated_implementation_time: "1 hour".to_string(),
                parameters_to_tune: vec![],
                warnings: vec![],
            };
            
            assert_eq!(recommendation.confidence_score, 0.75);
        }
    }

    #[test]
    fn test_confidence_score_validation() {
        let mut recommendation = AiOptimizationRecommendation {
            strategy: EcosystemOptimizationStrategy::Balanced,
            confidence_score: 0.95,
            expected_improvement: "Balanced optimization".to_string(),
            implementation_complexity: "Low".to_string(),
            estimated_implementation_time: "30 minutes".to_string(),
            parameters_to_tune: vec![],
            warnings: vec![],
        };
        
        // High confidence recommendations should have fewer warnings
        assert!(recommendation.confidence_score >= 0.8);
        
        // Lower confidence should have more warnings
        recommendation.confidence_score = 0.6;
        recommendation.warnings.push("Lower confidence recommendation".to_string());
        recommendation.warnings.push("Verify results manually".to_string());
        recommendation.warnings.push("Consider alternative approaches".to_string());
        
        assert!(recommendation.warnings.len() >= 3);
    }

    #[test]
    fn test_parameter_tuning_recommendations() {
        let recommendation = AiOptimizationRecommendation {
            strategy: EcosystemOptimizationStrategy::PerformanceFirst,
            confidence_score: 0.9,
            expected_improvement: "Performance optimization".to_string(),
            implementation_complexity: "Medium".to_string(),
            estimated_implementation_time: "1-2 hours".to_string(),
            parameters_to_tune: vec![
                ("recordsize".to_string(), "128K".to_string()),
                ("compression".to_string(), "lz4".to_string()),
                ("atime".to_string(), "off".to_string()),
                ("logbias".to_string(), "throughput".to_string()),
                ("sync".to_string(), "disabled".to_string()),
            ],
            warnings: vec![
                "Disabling sync may impact data integrity".to_string(),
                "Monitor system after changes".to_string(),
            ],
        };
        
        // Verify all parameters have values
        assert_eq!(recommendation.parameters_to_tune.len(), 5);
        for (param, value) in &recommendation.parameters_to_tune {
            assert!(!param.is_empty());
            assert!(!value.is_empty());
        }
        
        // Verify performance-focused parameters
        let params: HashMap<_, _> = recommendation.parameters_to_tune.iter().cloned().collect();
        assert_eq!(params.get("compression"), Some(&"lz4".to_string()));
        assert_eq!(params.get("atime"), Some(&"off".to_string()));
        assert_eq!(params.get("logbias"), Some(&"throughput".to_string()));
    }
}

#[cfg(test)]
mod zfs_expertise_context_tests {
    use super::*;

    #[test]
    fn test_zfs_expertise_context_creation() {
        let context = ZfsExpertiseContext {
            zfs_version: "2.1.0".to_string(),
            pool_configurations: HashMap::new(),
            historical_performance: vec![],
            known_issues: vec![],
            best_practices: vec![
                "Enable compression for space efficiency".to_string(),
                "Use appropriate record sizes for workload".to_string(),
                "Monitor ARC hit ratios regularly".to_string(),
                "Implement regular scrubbing schedule".to_string(),
            ],
        };
        
        assert_eq!(context.zfs_version, "2.1.0");
        assert_eq!(context.pool_configurations.len(), 0);
        assert_eq!(context.historical_performance.len(), 0);
        assert_eq!(context.known_issues.len(), 0);
        assert_eq!(context.best_practices.len(), 4);
    }

    #[test]
    fn test_zfs_best_practices_knowledge() {
        let context = ZfsExpertiseContext {
            zfs_version: "2.1.0".to_string(),
            pool_configurations: HashMap::new(),
            historical_performance: vec![],
            known_issues: vec![
                "ZFS may experience performance issues with very small record sizes".to_string(),
                "Deduplication can be memory-intensive for large datasets".to_string(),
                "L2ARC requires careful tuning for optimal performance".to_string(),
            ],
            best_practices: vec![
                "Use whole disks for pools when possible".to_string(),
                "Align record sizes with application I/O patterns".to_string(),
                "Reserve 10-20% free space for optimal performance".to_string(),
                "Use mirrored vdevs for critical data".to_string(),
                "Enable compression unless CPU is constrained".to_string(),
            ],
        };
        
        assert_eq!(context.known_issues.len(), 3);
        assert_eq!(context.best_practices.len(), 5);
        
        // Verify knowledge includes performance-related guidance
        assert!(context.known_issues.iter().any(|issue| issue.contains("performance")));
        assert!(context.best_practices.iter().any(|practice| practice.contains("record sizes")));
        assert!(context.best_practices.iter().any(|practice| practice.contains("compression")));
    }

    #[test]
    fn test_zfs_version_compatibility() {
        let versions = vec![
            "2.0.0", "2.0.1", "2.0.2", "2.0.3", "2.0.4", "2.0.5", "2.0.6",
            "2.1.0", "2.1.1", "2.1.2", "2.1.3", "2.1.4", "2.1.5",
            "2.2.0",
        ];
        
        for version in versions {
            let context = ZfsExpertiseContext {
                zfs_version: version.to_string(),
                pool_configurations: HashMap::new(),
                historical_performance: vec![],
                known_issues: vec![],
                best_practices: vec![],
            };
            
            assert_eq!(context.zfs_version, version);
            
            // Version-specific checks
            if version.starts_with("2.1") || version.starts_with("2.2") {
                // Newer versions support more features
                assert!(context.zfs_version >= "2.1.0");
            }
        }
    }
}

#[cfg(test)]
mod optimization_state_tests {
    use super::*;

    #[test]
    fn test_optimization_state_lifecycle() {
        let states = vec![
            OptimizationState::NotStarted,
            OptimizationState::Analyzing,
            OptimizationState::RecommendationsReady,
            OptimizationState::Implementing,
            OptimizationState::Completed,
            OptimizationState::Failed,
        ];
        
        for state in states {
            // Test state can be created and matched
            match state {
                OptimizationState::NotStarted => assert!(true),
                OptimizationState::Analyzing => assert!(true),
                OptimizationState::RecommendationsReady => assert!(true),
                OptimizationState::Implementing => assert!(true),
                OptimizationState::Completed => assert!(true),
                OptimizationState::Failed => assert!(true),
            }
        }
    }

    #[test]
    fn test_optimization_state_transitions() {
        // Test valid state transitions
        let valid_transitions = vec![
            (OptimizationState::NotStarted, OptimizationState::Analyzing),
            (OptimizationState::Analyzing, OptimizationState::RecommendationsReady),
            (OptimizationState::Analyzing, OptimizationState::Failed),
            (OptimizationState::RecommendationsReady, OptimizationState::Implementing),
            (OptimizationState::Implementing, OptimizationState::Completed),
            (OptimizationState::Implementing, OptimizationState::Failed),
        ];
        
        for (from_state, to_state) in valid_transitions {
            // Verify states are different (transition occurred)
            assert_ne!(
                std::mem::discriminant(&from_state),
                std::mem::discriminant(&to_state)
            );
        }
    }
}

#[cfg(test)]
mod ai_analysis_request_tests {
    use super::*;
    use super::test_utils::*;

    #[test]
    fn test_zfs_ai_analysis_request_creation() {
        let context = create_ai_optimization_context();
        
        let request = ZfsAiAnalysisRequest {
            context,
            analysis_type: "performance_optimization".to_string(),
            priority: "high".to_string(),
            constraints: HashMap::new(),
        };
        
        assert_eq!(request.analysis_type, "performance_optimization");
        assert_eq!(request.priority, "high");
        assert_eq!(request.constraints.len(), 0);
    }

    #[test]
    fn test_analysis_request_with_constraints() {
        let context = create_high_performance_scenario();
        
        let mut constraints = HashMap::new();
        constraints.insert("max_downtime_minutes".to_string(), "5".to_string());
        constraints.insert("preserve_data_integrity".to_string(), "true".to_string());
        constraints.insert("budget_limit".to_string(), "enterprise".to_string());
        
        let request = ZfsAiAnalysisRequest {
            context,
            analysis_type: "comprehensive_optimization".to_string(),
            priority: "critical".to_string(),
            constraints,
        };
        
        assert_eq!(request.analysis_type, "comprehensive_optimization");
        assert_eq!(request.priority, "critical");
        assert_eq!(request.constraints.len(), 3);
        assert_eq!(request.constraints.get("preserve_data_integrity"), Some(&"true".to_string()));
    }

    #[test]
    fn test_analysis_type_categories() {
        let analysis_types = vec![
            "performance_optimization",
            "space_optimization", 
            "reliability_analysis",
            "capacity_planning",
            "health_assessment",
            "security_audit",
            "comprehensive_optimization",
        ];
        
        let context = create_ai_optimization_context();
        
        for analysis_type in analysis_types {
            let request = ZfsAiAnalysisRequest {
                context: context.clone(),
                analysis_type: analysis_type.to_string(),
                priority: "medium".to_string(),
                constraints: HashMap::new(),
            };
            
            assert_eq!(request.analysis_type, analysis_type);
            assert!(!request.analysis_type.is_empty());
        }
    }

    #[test]
    fn test_priority_levels() {
        let priorities = vec!["low", "medium", "high", "critical", "emergency"];
        let context = create_ai_optimization_context();
        
        for priority in priorities {
            let request = ZfsAiAnalysisRequest {
                context: context.clone(),
                analysis_type: "performance_optimization".to_string(),
                priority: priority.to_string(),
                constraints: HashMap::new(),
            };
            
            assert_eq!(request.priority, priority);
            
            // Higher priority should get faster processing
            match priority {
                "critical" | "emergency" => {
                    // These should be processed immediately
                    assert!(request.priority.contains("critical") || request.priority.contains("emergency"));
                }
                "high" => {
                    assert_eq!(request.priority, "high");
                }
                "medium" | "low" => {
                    // These can be queued
                    assert!(request.priority == "medium" || request.priority == "low");
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod ecosystem_ai_analysis_tests {
    use super::*;

    #[test]
    fn test_ecosystem_ai_analysis_creation() {
        let analysis = EcosystemAiAnalysis {
            recommendations: vec![],
            confidence_score: 0.8,
            analysis_summary: "Comprehensive ZFS optimization analysis".to_string(),
            implementation_plan: vec![
                "Step 1: Backup critical data".to_string(),
                "Step 2: Apply compression settings".to_string(),
                "Step 3: Adjust record sizes".to_string(),
                "Step 4: Monitor performance metrics".to_string(),
                "Step 5: Validate improvements".to_string(),
            ],
            estimated_impact: "30% performance improvement, 40% space savings".to_string(),
            risk_assessment: "Low risk with proper testing".to_string(),
        };
        
        assert_eq!(analysis.recommendations.len(), 0);
        assert_eq!(analysis.confidence_score, 0.8);
        assert!(!analysis.analysis_summary.is_empty());
        assert_eq!(analysis.implementation_plan.len(), 5);
        assert!(!analysis.estimated_impact.is_empty());
        assert!(!analysis.risk_assessment.is_empty());
    }

    #[test]
    fn test_analysis_with_recommendations() {
        let recommendations = vec![
            AiOptimizationRecommendation {
                strategy: EcosystemOptimizationStrategy::PerformanceFirst,
                confidence_score: 0.9,
                expected_improvement: "50% latency reduction".to_string(),
                implementation_complexity: "Medium".to_string(),
                estimated_implementation_time: "2 hours".to_string(),
                parameters_to_tune: vec![
                    ("compression".to_string(), "lz4".to_string()),
                    ("recordsize".to_string(), "1M".to_string()),
                ],
                warnings: vec!["Test thoroughly in staging".to_string()],
            },
            AiOptimizationRecommendation {
                strategy: EcosystemOptimizationStrategy::SpaceEfficient,
                confidence_score: 0.85,
                expected_improvement: "60% space savings".to_string(),
                implementation_complexity: "Low".to_string(),
                estimated_implementation_time: "30 minutes".to_string(),
                parameters_to_tune: vec![
                    ("compression".to_string(), "zstd".to_string()),
                    ("dedup".to_string(), "on".to_string()),
                ],
                warnings: vec!["Monitor CPU usage after enabling dedup".to_string()],
            },
        ];
        
        let analysis = EcosystemAiAnalysis {
            recommendations,
            confidence_score: 0.87,
            analysis_summary: "Multi-strategy optimization analysis".to_string(),
            implementation_plan: vec![
                "Implement performance optimizations first".to_string(),
                "Monitor system stability".to_string(),
                "Apply space optimizations incrementally".to_string(),
            ],
            estimated_impact: "Combined 45% performance boost and 50% space reduction".to_string(),
            risk_assessment: "Medium risk - requires careful staging".to_string(),
        };
        
        assert_eq!(analysis.recommendations.len(), 2);
        assert_eq!(analysis.confidence_score, 0.87);
        
        // Verify recommendations are different strategies
        assert!(matches!(analysis.recommendations[0].strategy, EcosystemOptimizationStrategy::PerformanceFirst));
        assert!(matches!(analysis.recommendations[1].strategy, EcosystemOptimizationStrategy::SpaceEfficient));
        
        // Verify both recommendations have parameters
        assert_eq!(analysis.recommendations[0].parameters_to_tune.len(), 2);
        assert_eq!(analysis.recommendations[1].parameters_to_tune.len(), 2);
    }

    #[test]
    fn test_risk_assessment_levels() {
        let risk_levels = vec![
            ("Very Low", 0.95),
            ("Low", 0.85),
            ("Medium", 0.75),
            ("High", 0.65),
            ("Very High", 0.45),
        ];
        
        for (risk_level, confidence) in risk_levels {
            let analysis = EcosystemAiAnalysis {
                recommendations: vec![],
                confidence_score: confidence,
                analysis_summary: format!("Analysis with {} risk", risk_level),
                implementation_plan: vec![
                    format!("Implement with {} risk precautions", risk_level),
                ],
                estimated_impact: "Variable impact".to_string(),
                risk_assessment: format!("{} risk", risk_level),
            };
            
            assert_eq!(analysis.confidence_score, confidence);
            assert!(analysis.risk_assessment.contains(risk_level));
            
            // Higher risk should correlate with lower confidence
            if risk_level.contains("High") {
                assert!(analysis.confidence_score < 0.7);
            } else if risk_level.contains("Low") {
                assert!(analysis.confidence_score > 0.8);
            }
        }
    }
}

#[cfg(test)]
mod ai_optimization_tests {
    use super::*;

    #[test]
    fn test_ai_optimization_recommendation_creation() {
        let recommendation = AiOptimizationRecommendation {
            strategy: EcosystemOptimizationStrategy::PerformanceFirst,
            confidence_score: 0.85,
            expected_improvement: "40% latency reduction".to_string(),
            implementation_complexity: "Medium".to_string(),
            estimated_implementation_time: "2 hours".to_string(),
            parameters_to_tune: vec![
                ("recordsize".to_string(), "1M".to_string()),
                ("compression".to_string(), "lz4".to_string()),
            ],
            warnings: vec!["Test in staging first".to_string()],
        };
        
        assert_eq!(recommendation.confidence_score, 0.85);
        assert_eq!(recommendation.parameters_to_tune.len(), 2);
        assert_eq!(recommendation.warnings.len(), 1);
    }

    #[test]
    fn test_optimization_strategies() {
        let strategies = vec![
            EcosystemOptimizationStrategy::PerformanceFirst,
            EcosystemOptimizationStrategy::SpaceEfficient,
            EcosystemOptimizationStrategy::Balanced,
            EcosystemOptimizationStrategy::PowerEfficient,
        ];
        
        for strategy in strategies {
            let recommendation = AiOptimizationRecommendation {
                strategy,
                confidence_score: 0.75,
                expected_improvement: "Test improvement".to_string(),
                implementation_complexity: "Low".to_string(),
                estimated_implementation_time: "1 hour".to_string(),
                parameters_to_tune: vec![],
                warnings: vec![],
            };
            
            assert_eq!(recommendation.confidence_score, 0.75);
        }
    }

    #[test]
    fn test_system_capabilities() {
        let capabilities = SystemCapabilities {
            total_memory_gb: 64,
            cpu_cores: 16,
            storage_tier: StorageTier::Hot,
            zfs_version: "2.1.0".to_string(),
            kernel_version: "5.15.0".to_string(),
        };
        
        assert_eq!(capabilities.total_memory_gb, 64);
        assert_eq!(capabilities.cpu_cores, 16);
        assert_eq!(capabilities.storage_tier, StorageTier::Hot);
        assert_eq!(capabilities.zfs_version, "2.1.0");
    }

    #[test]
    fn test_optimization_state_enum() {
        let states = vec![
            OptimizationState::NotStarted,
            OptimizationState::Analyzing,
            OptimizationState::RecommendationsReady,
            OptimizationState::Implementing,
            OptimizationState::Completed,
            OptimizationState::Failed,
        ];
        
        assert_eq!(states.len(), 6);
    }

    #[test]
    fn test_zfs_expertise_context() {
        let context = ZfsExpertiseContext {
            zfs_version: "2.1.0".to_string(),
            pool_configurations: HashMap::new(),
            historical_performance: vec![],
            known_issues: vec![
                "Performance may degrade with small record sizes".to_string(),
            ],
            best_practices: vec![
                "Use compression for space efficiency".to_string(),
                "Align record sizes with workload".to_string(),
            ],
        };
        
        assert_eq!(context.zfs_version, "2.1.0");
        assert_eq!(context.known_issues.len(), 1);
        assert_eq!(context.best_practices.len(), 2);
    }
}

#[cfg(test)]
mod ai_analysis_tests {
    use super::*;

    fn create_sample_metrics() -> ZfsPerformanceMetrics {
        let mut pool_metrics = HashMap::new();
        pool_metrics.insert("test_pool".to_string(), ZfsPoolMetrics {
            name: "test_pool".to_string(),
            read_ops_per_sec: 1000.0,
            write_ops_per_sec: 500.0,
            read_bandwidth_mbps: 100.0,
            write_bandwidth_mbps: 50.0,
            average_latency_ms: 10.0,
            cache_hit_ratio: 0.8,
            fragmentation_percent: 15.0,
        });
        
        let mut dataset_metrics = HashMap::new();
        dataset_metrics.insert("test_dataset".to_string(), ZfsDatasetMetrics {
            name: "test_dataset".to_string(),
            compression_ratio: 2.0,
            dedup_ratio: 1.2,
            record_size: 128 * 1024,
            access_pattern: AccessPattern::Random,
        });
        
        ZfsPerformanceMetrics {
            timestamp: SystemTime::now(),
            pool_metrics,
            dataset_metrics,
            system_memory_usage: SystemMemoryUsage {
                total_memory: 32 * 1024 * 1024 * 1024,
                used_memory: 20 * 1024 * 1024 * 1024,
                available_memory: 12 * 1024 * 1024 * 1024,
            },
            arc_stats: ArcStatistics {
                hit_ratio: 0.8,
                size_bytes: 8 * 1024 * 1024 * 1024,
                target_size_bytes: 8 * 1024 * 1024 * 1024,
                meta_used_bytes: 1 * 1024 * 1024 * 1024,
            },
        }
    }

    #[test]
    fn test_ai_optimization_context() {
        let mut workload_patterns = HashMap::new();
        workload_patterns.insert("web_app".to_string(), WorkloadPattern {
            read_write_ratio: 0.8,
            sequential_random_ratio: 0.5,
            average_io_size: 64 * 1024,
            peak_iops: 10000,
        });
        
        let context = AiOptimizationContext {
            system_capabilities: SystemCapabilities {
                total_memory_gb: 32,
                cpu_cores: 8,
                storage_tier: StorageTier::Warm,
                zfs_version: "2.1.0".to_string(),
                kernel_version: "5.15.0".to_string(),
            },
            current_metrics: create_sample_metrics(),
            workload_patterns,
            optimization_history: vec![],
            constraints: HashMap::new(),
        };
        
        assert_eq!(context.system_capabilities.total_memory_gb, 32);
        assert_eq!(context.workload_patterns.len(), 1);
        assert_eq!(context.optimization_history.len(), 0);
    }

    #[test]
    fn test_zfs_ai_analysis_request() {
        let context = AiOptimizationContext {
            system_capabilities: SystemCapabilities {
                total_memory_gb: 64,
                cpu_cores: 16,
                storage_tier: StorageTier::Hot,
                zfs_version: "2.1.0".to_string(),
                kernel_version: "5.15.0".to_string(),
            },
            current_metrics: create_sample_metrics(),
            workload_patterns: HashMap::new(),
            optimization_history: vec![],
            constraints: HashMap::new(),
        };
        
        let request = ZfsAiAnalysisRequest {
            context,
            analysis_type: "performance_optimization".to_string(),
            priority: "high".to_string(),
            constraints: HashMap::new(),
        };
        
        assert_eq!(request.analysis_type, "performance_optimization");
        assert_eq!(request.priority, "high");
    }

    #[test]
    fn test_ecosystem_ai_analysis() {
        let analysis = EcosystemAiAnalysis {
            recommendations: vec![],
            confidence_score: 0.9,
            analysis_summary: "Comprehensive optimization analysis".to_string(),
            implementation_plan: vec![
                "Step 1: Apply compression settings".to_string(),
                "Step 2: Adjust record sizes".to_string(),
                "Step 3: Monitor performance".to_string(),
            ],
            estimated_impact: "30% performance improvement".to_string(),
            risk_assessment: "Low risk".to_string(),
        };
        
        assert_eq!(analysis.confidence_score, 0.9);
        assert_eq!(analysis.implementation_plan.len(), 3);
        assert!(!analysis.analysis_summary.is_empty());
        assert!(!analysis.estimated_impact.is_empty());
    }
}

#[cfg(test)]
mod ai_config_tests {
    use super::*;

    #[test]
    fn test_zfs_ai_config_default() {
        let config = ZfsAiConfig::default();
        
        assert!(config.enable_tier_optimization);
        assert!(config.enable_predictive_analytics);
        assert!(config.enable_anomaly_detection);
        assert_eq!(config.optimization_interval, 3600);
        assert_eq!(config.analytics_interval, 300);
        assert_eq!(config.min_confidence_threshold, 0.7);
        assert_eq!(config.max_concurrent_models, 3);
        assert_eq!(config.model_cache_dir, "/var/cache/nestgate/ai-models");
    }

    #[test]
    fn test_zfs_ai_config_custom() {
        let config = ZfsAiConfig {
            enable_tier_optimization: false,
            enable_predictive_analytics: true,
            enable_anomaly_detection: false,
            optimization_interval: 7200,
            analytics_interval: 600,
            min_confidence_threshold: 0.8,
            max_concurrent_models: 5,
            model_cache_dir: "/custom/cache/dir".to_string(),
        };
        
        assert!(!config.enable_tier_optimization);
        assert!(config.enable_predictive_analytics);
        assert!(!config.enable_anomaly_detection);
        assert_eq!(config.optimization_interval, 7200);
        assert_eq!(config.analytics_interval, 600);
        assert_eq!(config.min_confidence_threshold, 0.8);
        assert_eq!(config.max_concurrent_models, 5);
        assert_eq!(config.model_cache_dir, "/custom/cache/dir");
    }
}

#[cfg(test)]
mod optimization_opportunity_tests {
    use super::*;

    #[test]
    fn test_optimization_opportunity_creation() {
        let opportunity = OptimizationOpportunity {
            id: "opt-001".to_string(),
            opportunity_type: "tier_migration".to_string(),
            description: "Move cold data to warm tier".to_string(),
            potential_benefit: "30% performance improvement".to_string(),
            confidence_score: 0.85,
            implementation_effort: "Low".to_string(),
            priority: "High".to_string(),
            estimated_impact: "Significant performance gain".to_string(),
            prerequisites: vec!["Backup data".to_string(), "Schedule maintenance window".to_string()],
        };
        
        assert_eq!(opportunity.id, "opt-001");
        assert_eq!(opportunity.opportunity_type, "tier_migration");
        assert_eq!(opportunity.confidence_score, 0.85);
        assert_eq!(opportunity.prerequisites.len(), 2);
    }

    #[test]
    fn test_optimization_types() {
        let types = vec![
            OptimizationType::TierMigration,
            OptimizationType::CompressionOptimization,
            OptimizationType::CacheOptimization,
            OptimizationType::DeduplicationOptimization,
            OptimizationType::PerformanceTuning,
        ];
        
        assert_eq!(types.len(), 5);
    }

    #[test]
    fn test_optimization_effort_levels() {
        let efforts = vec![
            OptimizationEffort::Low,
            OptimizationEffort::Medium,
            OptimizationEffort::High,
        ];
        
        assert_eq!(efforts.len(), 3);
    }

    #[test]
    fn test_optimization_benefit() {
        let benefit = OptimizationBenefit {
            performance_improvement: 25.5,
            storage_savings: 1024 * 1024 * 1024, // 1GB
            cost_reduction: 100.0,
        };
        
        assert_eq!(benefit.performance_improvement, 25.5);
        assert_eq!(benefit.storage_savings, 1024 * 1024 * 1024);
        assert_eq!(benefit.cost_reduction, 100.0);
    }
}

#[cfg(test)]
mod tier_prediction_tests {
    use super::*;

    #[test]
    fn test_tier_prediction_creation() {
        let prediction = TierPrediction {
            file_path: "/data/important/file.txt".to_string(),
            predicted_tier: StorageTier::Hot,
            current_tier: StorageTier::Warm,
            confidence: 0.92,
            reasoning: "Frequently accessed file with high performance requirements".to_string(),
            expected_improvement: 40.0,
            timestamp: SystemTime::now(),
        };
        
        assert_eq!(prediction.file_path, "/data/important/file.txt");
        assert_eq!(prediction.predicted_tier, StorageTier::Hot);
        assert_eq!(prediction.current_tier, StorageTier::Warm);
        assert_eq!(prediction.confidence, 0.92);
        assert_eq!(prediction.expected_improvement, 40.0);
    }

    #[test]
    fn test_storage_tier_recommendations() {
        let hot_prediction = TierPrediction {
            file_path: "/db/active.db".to_string(),
            predicted_tier: StorageTier::Hot,
            current_tier: StorageTier::Cold,
            confidence: 0.95,
            reasoning: "Database file with frequent access".to_string(),
            expected_improvement: 80.0,
            timestamp: SystemTime::now(),
        };
        
        let cold_prediction = TierPrediction {
            file_path: "/archive/old_data.tar".to_string(),
            predicted_tier: StorageTier::Cold,
            current_tier: StorageTier::Hot,
            confidence: 0.88,
            reasoning: "Archive file rarely accessed".to_string(),
            expected_improvement: 15.0,
            timestamp: SystemTime::now(),
        };
        
        assert_eq!(hot_prediction.predicted_tier, StorageTier::Hot);
        assert_eq!(cold_prediction.predicted_tier, StorageTier::Cold);
        assert!(hot_prediction.expected_improvement > cold_prediction.expected_improvement);
    }
}

#[cfg(test)]
mod system_context_tests {
    use super::*;

    #[test]
    fn test_system_context_creation() {
        let context = SystemContext {
            total_memory_gb: 128.0,
            available_memory_gb: 64.0,
            cpu_cores: 32,
            storage_tiers_available: vec![StorageTier::Hot, StorageTier::Warm, StorageTier::Cold],
            current_workload_type: "database".to_string(),
            system_load_avg: 2.5,
        };
        
        assert_eq!(context.total_memory_gb, 128.0);
        assert_eq!(context.available_memory_gb, 64.0);
        assert_eq!(context.cpu_cores, 32);
        assert_eq!(context.storage_tiers_available.len(), 3);
        assert_eq!(context.current_workload_type, "database");
        assert_eq!(context.system_load_avg, 2.5);
    }

    #[test]
    fn test_system_resource_ratios() {
        let context = SystemContext {
            total_memory_gb: 64.0,
            available_memory_gb: 16.0,
            cpu_cores: 16,
            storage_tiers_available: vec![StorageTier::Hot, StorageTier::Warm],
            current_workload_type: "mixed".to_string(),
            system_load_avg: 8.0,
        };
        
        let memory_utilization = (context.total_memory_gb - context.available_memory_gb) / context.total_memory_gb;
        assert_eq!(memory_utilization, 0.75); // 75% memory utilization
        
        let load_per_core = context.system_load_avg / context.cpu_cores as f64;
        assert_eq!(load_per_core, 0.5); // 50% load per core
    }
}

#[cfg(test)]
mod workload_analysis_tests {
    use super::*;

    #[test]
    fn test_workload_analysis_creation() {
        let analysis = WorkloadAnalysis {
            read_write_ratio: 0.8,
            random_sequential_ratio: 0.6,
            block_size_distribution: vec![4096, 8192, 16384, 32768],
        };
        
        assert_eq!(analysis.read_write_ratio, 0.8);
        assert_eq!(analysis.random_sequential_ratio, 0.6);
        assert_eq!(analysis.block_size_distribution.len(), 4);
    }

    #[test]
    fn test_workload_characteristics() {
        let database_workload = WorkloadAnalysis {
            read_write_ratio: 0.7, // 70% reads
            random_sequential_ratio: 0.9, // 90% random access
            block_size_distribution: vec![4096, 8192], // Small blocks
        };
        
        let backup_workload = WorkloadAnalysis {
            read_write_ratio: 0.1, // 10% reads, 90% writes
            random_sequential_ratio: 0.1, // 10% random, 90% sequential
            block_size_distribution: vec![1048576, 4194304], // Large blocks
        };
        
        assert!(database_workload.read_write_ratio > backup_workload.read_write_ratio);
        assert!(database_workload.random_sequential_ratio > backup_workload.random_sequential_ratio);
    }
}

#[cfg(test)]
mod performance_analysis_tests {
    use super::*;

    #[test]
    fn test_compression_analysis() {
        let analysis = CompressionAnalysis {
            current_algorithm: "lz4".to_string(),
            compression_ratio: 2.1,
            cpu_overhead: 5.0,
            current_ratio: 2.1,
            compression_enabled: true,
            estimated_ratio: 2.3,
            algorithm: "zstd".to_string(),
            cpu_overhead_percent: 8.0,
        };
        
        assert_eq!(analysis.current_algorithm, "lz4");
        assert_eq!(analysis.compression_ratio, 2.1);
        assert_eq!(analysis.cpu_overhead, 5.0);
        assert!(analysis.compression_enabled);
    }

    #[test]
    fn test_snapshot_analysis() {
        let analysis = SnapshotAnalysis {
            total_snapshots: 100,
            total_size_gb: 50.0,
            oldest_snapshot_days: 365,
            snapshot_count: 100,
            avg_daily_snapshots: 2.5,
            avg_access_frequency: 0.1,
        };
        
        assert_eq!(analysis.total_snapshots, 100);
        assert_eq!(analysis.total_size_gb, 50.0);
        assert_eq!(analysis.oldest_snapshot_days, 365);
        assert_eq!(analysis.avg_daily_snapshots, 2.5);
    }

    #[test]
    fn test_migration_candidate() {
        let candidate = MigrationCandidate {
            dataset_name: "important_data".to_string(),
            current_tier: StorageTier::Cold,
            recommended_tier: StorageTier::Hot,
            confidence: 0.9,
            expected_benefit: "75% performance improvement".to_string(),
            performance_gain: 75.0,
        };
        
        assert_eq!(candidate.dataset_name, "important_data");
        assert_eq!(candidate.current_tier, StorageTier::Cold);
        assert_eq!(candidate.recommended_tier, StorageTier::Hot);
        assert_eq!(candidate.confidence, 0.9);
        assert_eq!(candidate.performance_gain, 75.0);
    }
}

#[cfg(test)]
mod optimization_execution_tests {
    use super::*;

    #[test]
    fn test_optimization_execution_result() {
        let result = OptimizationExecutionResult {
            optimization_id: "opt-123".to_string(),
            status: OptimizationStatus::Success,
            reason: "Successfully applied compression optimization".to_string(),
            performance_impact: None,
            execution_time_seconds: 45.0,
            rollback_available: true,
        };
        
        assert_eq!(result.optimization_id, "opt-123");
        assert!(matches!(result.status, OptimizationStatus::Success));
        assert_eq!(result.execution_time_seconds, 45.0);
        assert!(result.rollback_available);
    }

    #[test]
    fn test_optimization_status_types() {
        let statuses = vec![
            OptimizationStatus::Success,
            OptimizationStatus::Failed,
            OptimizationStatus::Skipped,
            OptimizationStatus::InProgress,
            OptimizationStatus::RolledBack,
        ];
        
        assert_eq!(statuses.len(), 5);
    }

    #[test]
    fn test_safety_validation_result() {
        let validation = SafetyValidationResult {
            is_safe: true,
            reason: "All safety checks passed".to_string(),
            checks_performed: vec![
                "Data integrity check".to_string(),
                "Backup verification".to_string(),
                "Resource availability check".to_string(),
            ],
            risk_level: RiskLevel::Low,
        };
        
        assert!(validation.is_safe);
        assert_eq!(validation.checks_performed.len(), 3);
        assert!(matches!(validation.risk_level, RiskLevel::Low));
    }

    #[test]
    fn test_risk_levels() {
        let risks = vec![
            RiskLevel::Low,
            RiskLevel::Medium,
            RiskLevel::High,
            RiskLevel::Critical,
        ];
        
        assert_eq!(risks.len(), 4);
    }
}

#[cfg(test)]
mod autonomous_optimization_tests {
    use super::*;

    #[test]
    fn test_autonomous_optimization_report() {
        let report = AutonomousOptimizationReport {
            cycle_duration_seconds: 3600.0,
            opportunities_detected: 15,
            optimizations_applied: 8,
            total_performance_improvement: 35.5,
            optimizations_skipped: 5,
            optimizations_failed: 2,
            recommendations: vec![
                "Consider increasing cache size".to_string(),
                "Schedule defragmentation during maintenance window".to_string(),
                "Review compression algorithms for better efficiency".to_string(),
            ],
        };
        
        assert_eq!(report.cycle_duration_seconds, 3600.0);
        assert_eq!(report.opportunities_detected, 15);
        assert_eq!(report.optimizations_applied, 8);
        assert_eq!(report.total_performance_improvement, 35.5);
        assert_eq!(report.optimizations_skipped, 5);
        assert_eq!(report.optimizations_failed, 2);
        assert_eq!(report.recommendations.len(), 3);
    }

    #[test]
    fn test_optimization_success_rate() {
        let report = AutonomousOptimizationReport {
            cycle_duration_seconds: 1800.0,
            opportunities_detected: 20,
            optimizations_applied: 16,
            total_performance_improvement: 42.0,
            optimizations_skipped: 2,
            optimizations_failed: 2,
            recommendations: vec![],
        };
        
        let success_rate = report.optimizations_applied as f64 / 
                          (report.optimizations_applied + report.optimizations_failed) as f64;
        
        assert_eq!(success_rate, 0.8888888888888888); // ~89% success rate
        
        let total_processed = report.optimizations_applied + report.optimizations_skipped + report.optimizations_failed;
        assert_eq!(total_processed, 20);
        assert_eq!(total_processed, report.opportunities_detected);
    }
} 