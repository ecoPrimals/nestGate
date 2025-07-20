//! Performance Analytics Analysis
//!
//! Analytics functionality for alert checking and recommendation generation.

use chrono::Utc;
use std::collections::HashMap;
use uuid::Uuid;

use super::types::*;

/// Check for performance alerts
pub async fn check_alerts(
    metrics: &SystemMetrics,
    config: &PerformanceConfig,
) -> std::result::Result<Vec<PerformanceAlert>, Box<dyn std::error::Error + Send + Sync>> {
    let mut alerts = Vec::new();

    // CPU usage alert
    if metrics.cpu.usage_percent > config.alert_thresholds.cpu_threshold {
        alerts.push(PerformanceAlert {
            id: Uuid::new_v4(),
            alert_type: AlertType::ResourceUsage,
            severity: if metrics.cpu.usage_percent > 90.0 {
                AlertSeverity::Critical
            } else {
                AlertSeverity::Warning
            },
            message: format!("High CPU usage detected: {:.1}%", metrics.cpu.usage_percent),
            metric: "cpu.usage_percent".to_string(),
            current_value: metrics.cpu.usage_percent,
            threshold_value: config.alert_thresholds.cpu_threshold,
            timestamp: Utc::now(),
            component: "CPU".to_string(),
            suggested_actions: vec![
                "Review running processes".to_string(),
                "Consider CPU scaling or optimization".to_string(),
            ],
        });
    }

    // Memory usage alert
    if metrics.memory.usage_percent > config.alert_thresholds.memory_threshold {
        alerts.push(PerformanceAlert {
            id: Uuid::new_v4(),
            alert_type: AlertType::ResourceUsage,
            severity: if metrics.memory.usage_percent > 95.0 {
                AlertSeverity::Critical
            } else {
                AlertSeverity::Warning
            },
            message: format!(
                "High memory usage detected: {:.1}%",
                metrics.memory.usage_percent
            ),
            metric: "memory.usage_percent".to_string(),
            current_value: metrics.memory.usage_percent,
            threshold_value: config.alert_thresholds.memory_threshold,
            timestamp: Utc::now(),
            component: "Memory".to_string(),
            suggested_actions: vec![
                "Check for memory leaks".to_string(),
                "Review ZFS ARC settings".to_string(),
                "Consider adding more memory".to_string(),
            ],
        });
    }

    // Check ZFS pool health
    for (pool_name, pool_metrics) in &metrics.zfs.pools {
        if pool_metrics.health != "ONLINE" {
            alerts.push(PerformanceAlert {
                id: Uuid::new_v4(),
                alert_type: AlertType::HealthIssue,
                severity: AlertSeverity::Critical,
                message: format!(
                    "ZFS pool {} health issue: {}",
                    pool_name, pool_metrics.health
                ),
                metric: format!("zfs.pools.{pool_name}.health"),
                current_value: 0.0, // Health is not numeric
                threshold_value: 1.0,
                timestamp: Utc::now(),
                component: format!("ZFS Pool {pool_name}"),
                suggested_actions: vec![
                    "Check pool status with 'zpool status'".to_string(),
                    "Review system logs for disk errors".to_string(),
                    "Consider immediate maintenance".to_string(),
                ],
            });
        }
    }

    Ok(alerts)
}

/// Generate performance recommendations
pub async fn generate_recommendations(
    current_metrics: &SystemMetrics,
    _historical_metrics: &[SystemMetrics],
) -> std::result::Result<Vec<PerformanceRecommendation>, Box<dyn std::error::Error + Send + Sync>>
{
    let mut recommendations = Vec::new();

    // ZFS ARC tuning recommendation
    if current_metrics.zfs.arc.hit_ratio < 90.0 {
        recommendations.push(PerformanceRecommendation {
            id: Uuid::new_v4(),
            recommendation_type: RecommendationType::ZfsConfiguration,
            title: "Optimize ZFS ARC Configuration".to_string(),
            description: format!(
                "ZFS ARC hit ratio is {:.1}%, which is below optimal. Consider tuning ARC parameters.",
                current_metrics.zfs.arc.hit_ratio
            ),
            expected_improvement: "5-15% improvement in disk I/O performance".to_string(),
            effort_level: EffortLevel::Medium,
            priority: Priority::Medium,
            actions: vec![
                RecommendationAction {
                    description: "Increase ARC maximum size".to_string(),
                    command: Some("echo 'zfs_arc_max=17179869184' >> /etc/modprobe.d/zfs.conf".to_string()),
                    config_changes: Some({
                        let mut changes = HashMap::new();
                        changes.insert("zfs_arc_max".to_string(), "16GB".to_string());
                        changes
                    }),
                    risk_level: RiskLevel::Low,
                },
            ],
            estimated_impact: ImpactEstimate {
                performance_improvement: 10.0,
                resource_reduction: 0.0,
                timeframe: "Immediate".to_string(),
                confidence: 85,
            },
        });
    }

    // Disk fragmentation recommendation
    for (pool_name, pool_metrics) in &current_metrics.zfs.pools {
        if pool_metrics.fragmentation > 30.0 {
            recommendations.push(PerformanceRecommendation {
                id: Uuid::new_v4(),
                recommendation_type: RecommendationType::ZfsConfiguration,
                title: format!("Defragment ZFS Pool {pool_name}"),
                description: format!(
                    "Pool {} has {:.1}% fragmentation, which may impact performance.",
                    pool_name, pool_metrics.fragmentation
                ),
                expected_improvement: "10-20% improvement in sequential I/O".to_string(),
                effort_level: EffortLevel::High,
                priority: Priority::Medium,
                actions: vec![RecommendationAction {
                    description: "Schedule ZFS pool defragmentation".to_string(),
                    command: Some(format!("zpool online -e {pool_name}")),
                    config_changes: None,
                    risk_level: RiskLevel::Medium,
                }],
                estimated_impact: ImpactEstimate {
                    performance_improvement: 15.0,
                    resource_reduction: 0.0,
                    timeframe: "1-2 hours".to_string(),
                    confidence: 75,
                },
            });
        }
    }

    Ok(recommendations)
} 