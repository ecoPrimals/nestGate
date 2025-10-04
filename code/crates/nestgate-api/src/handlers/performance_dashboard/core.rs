//
// This module contains the main PerformanceDashboard struct and HTTP handler functions.

use super::{PerformanceAnalyzer, RealTimeMetricsCollector, OptimizationEngineInterface};
use super::zfs_integration::*;
use crate::handlers::performance_dashboard::types::*;
use crate::universal_primal::UniversalRequest;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Json, Response, Sse},
    response::sse::{Event, KeepAlive},
};
use std::time::Duration;
use tracing::{info, error, debug};
use nestgate_core::{NestGateError, Result, ApiResponse};
use serde::{Deserialize};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::{broadcast, RwLock};

/// Performance dashboard handler with real-time capabilities
#[derive(Debug)]
pub struct PerformanceDashboard {
    /// Real-time metrics collector
    metrics_collector: Arc<RealTimeMetricsCollector>,
    /// Performance analyzer
    performance_analyzer: Arc<PerformanceAnalyzer>,
    /// Optimization _engine interface
    optimization_engine: Arc<OptimizationEngineInterface>,
    /// Dashboard configuration
    config: DashboardConfig,
    /// Real-time event broadcaster
    event_broadcaster: Arc<broadcast::Sender<DashboardEvent>>,
    /// Dashboard state
    dashboard_state: Arc<RwLock<DashboardState>>,
}
impl PerformanceDashboard {
    /// Create a new performance dashboard
    pub fn new(config: DashboardConfig) -> Self { let (tx, _rx) = broadcast::channel(1000);

        Self {
            metrics_collector: Arc::new(RealTimeMetricsCollector::new()),
            performance_analyzer: Arc::new(PerformanceAnalyzer::new()),
            optimization_engine: Arc::new(OptimizationEngineInterface::new()),
            config,
            event_broadcaster: Arc::new(tx),
            dashboard_state: Arc::new(RwLock::new(DashboardState::new()),
         }

    /// Start the performance dashboard monitoring
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn start(&self) -> Result<()>  {
        info!("🚀 Starting Real-Time Performance Dashboard");

        // Start metrics collection
        let collector_clone = Arc::clone(&self.metrics_collector);
        let broadcaster_clone = Arc::clone(&self.event_broadcaster);
        tokio::spawn(async move {
            collector_clone.start_collection(broadcaster_clone).await;
        });

        // Start performance analysis
        let analyzer_clone = Arc::clone(&self.performance_analyzer);
        let broadcaster_clone = Arc::clone(&self.event_broadcaster);
        tokio::spawn(async move {
            analyzer_clone.start_analysis(broadcaster_clone).await;
        });

        info!("✅ Performance Dashboard started successfully");
    }

    /// Get comprehensive dashboard overview
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn get_dashboard_overview(
        &self,
        time_range: TimeRange,
    ) -> Result<DashboardOverview>  {
        debug!("📊 Generating dashboard overview for time range: {:?}", time_range);

        // Collect current metrics
        let current_metrics = self.metrics_collector.get_current_metrics().await?;

        // Get performance analysis
        let performance_analysis = self.performance_analyzer.analyze_performance(&time_range).await?;

        // Get optimization recommendations
        let optimization_recommendations = self.optimization_engine.get_recommendations().await?;

        // Generate real performance insights based on current metrics and analysis
        let mut insights = Vec::new();
        
        // CPU usage insights
        if current_metrics.system_metrics._cpu_usage > 90.0 {
            insights.push(PerformanceInsight {
                title: "High CPU Usage".to_string(),
                description: format!("CPU usage is at {"actual_error_details"}% - consider scaling resources"),
                severity: InsightSeverity::Critical,
                category: "resource".to_string(),
                recommendation: "Add more CPU cores or optimize high-CPU processes".to_string(),
                impact_score: 9.0,
                timestamp: SystemTime::now(),
            });
        } else if current_metrics.system_metrics._cpu_usage > 75.0 {
            insights.push(PerformanceInsight {
                title: "Elevated CPU Usage".to_string(),
                description: format!("CPU usage is at {"actual_error_details"}%"),
                severity: InsightSeverity::Warning,
                category: "resource".to_string(),
                recommendation: "Monitor CPU usage trends and prepare for scaling".to_string(),
                impact_score: 6.0,
                timestamp: SystemTime::now(),
            });
        }

        // Memory usage insights
        if current_metrics.system_metrics.memory_usage > 85.0 {
            insights.push(PerformanceInsight {
                title: "High Memory Usage".to_string(),
                description: format!("Memory usage is at {"actual_error_details"}%"),
                severity: InsightSeverity::Warning,
                category: "resource".to_string(),
                recommendation: "Consider increasing memory or optimizing memory-intensive processes".to_string(),
                impact_score: 7.0,
                timestamp: SystemTime::now(),
            });
        }

        // Pool utilization insights
        if current_metrics.pool_metrics.utilization_percent > 90.0 {
            insights.push(PerformanceInsight {
                title: "Critical Storage Capacity".to_string(),
                description: format!("Storage utilization is at {"actual_error_details"}%"),
                severity: InsightSeverity::Critical,
                category: "storage".to_string(),
                recommendation: "Immediately add storage capacity or migrate data".to_string(),
                impact_score: 10.0,
                timestamp: SystemTime::now(),
            });
        } else if current_metrics.pool_metrics.utilization_percent > 80.0 {
            insights.push(PerformanceInsight {
                title: "High Storage Usage".to_string(),
                description: format!("Storage utilization is at {"actual_error_details"}%"),
                severity: InsightSeverity::Warning,
                category: "storage".to_string(),
                recommendation: "Plan for storage expansion within 30 days".to_string(),
                impact_score: 8.0,
                timestamp: SystemTime::now(),
            });
        }

        // I/O latency insights
        if current_metrics.pool_metrics.avg_latency_ms > 50.0 {
            insights.push(PerformanceInsight {
                title: "High I/O Latency".to_string(),
                description: format!("Average I/O latency is {"actual_error_details"}ms"),
                severity: InsightSeverity::Warning,
                category: "performance".to_string(),
                recommendation: "Check disk health and consider SSD upgrade or pool optimization".to_string(),
                impact_score: 7.5,
                timestamp: SystemTime::now(),
            });
        }

        // Calculate real health score based on system metrics
        let mut score_components = std::collections::HashMap::new();
        let mut total_weight = 0.0f64;
        let mut weighted_score = 0.0f64;

        // CPU health component (weight: 25%)
        let cpu_score = if current_metrics.system_metrics._cpu_usage < 50.0 {
            100.0
        } else if current_metrics.system_metrics._cpu_usage < 70.0 {
            90.0 - (current_metrics.system_metrics._cpu_usage - 50.0) * 2.0
        } else if current_metrics.system_metrics._cpu_usage < 90.0 {
            50.0 - (current_metrics.system_metrics._cpu_usage - 70.0) * 1.5
        } else {
            10.0
        };
        score_components.insert("cpu".to_string(), cpu_score);
        weighted_score += cpu_score * 0.25;
        total_weight += 0.25;

        // Memory health component (weight: 20%)
        let memory_score = if current_metrics.system_metrics.memory_usage < 60.0 {
            100.0
        } else if current_metrics.system_metrics.memory_usage < 80.0 {
            90.0 - (current_metrics.system_metrics.memory_usage - 60.0) * 2.0
        } else if current_metrics.system_metrics.memory_usage < 95.0 {
            50.0 - (current_metrics.system_metrics.memory_usage - 80.0) * 2.0
        } else {
            20.0
        };
        score_components.insert("memory".to_string(), memory_score);
        weighted_score += memory_score * 0.20;
        total_weight += 0.20;

        // Storage capacity health component (weight: 30%)
        let storage_score = if current_metrics.pool_metrics.utilization_percent < 60.0 {
            100.0
        } else if current_metrics.pool_metrics.utilization_percent < 80.0 {
            95.0 - (current_metrics.pool_metrics.utilization_percent - 60.0) * 1.5
        } else if current_metrics.pool_metrics.utilization_percent < 90.0 {
            65.0 - (current_metrics.pool_metrics.utilization_percent - 80.0) * 3.0
        } else {
            25.0
        };
        score_components.insert("storage_capacity".to_string(), storage_score);
        weighted_score += storage_score * 0.30;
        total_weight += 0.30;

        // I/O performance health component (weight: 15%)
        let io_score = if current_metrics.pool_metrics.avg_latency_ms < 10.0 {
            100.0
        } else if current_metrics.pool_metrics.avg_latency_ms < 30.0 {
            90.0 - (current_metrics.pool_metrics.avg_latency_ms - 10.0) * 1.5
        } else if current_metrics.pool_metrics.avg_latency_ms < 100.0 {
            60.0 - (current_metrics.pool_metrics.avg_latency_ms - 30.0) * 0.5
        } else {
            25.0
        };
        score_components.insert("io_performance".to_string(), io_score);
        weighted_score += io_score * 0.15;
        total_weight += 0.15;

        // Network health component (weight: 10%) - simplified
        let network_score = if current_metrics.network_metrics.total_rx_bytes > 0 {
            95.0 // Assume network is healthy if we're getting data
        } else {
            70.0 // Reduced score if no network activity detected
        };
        score_components.insert("network".to_string(), network_score);
        weighted_score += network_score * 0.10;
        total_weight += 0.10;

        // Calculate overall score
        let overall_score = if total_weight > 0.0 {
            weighted_score / total_weight
        } else {
            0.0
        };

        // Determine health status based on overall score
        let health_status = if overall_score >= 90.0 {
            HealthStatus::Excellent
        } else if overall_score >= 80.0 {
            HealthStatus::Good
        } else if overall_score >= 60.0 {
            HealthStatus::Fair
        } else if overall_score >= 40.0 {
            HealthStatus::Poor
        } else {
            HealthStatus::Critical
        };

        let health_score = HealthScore {
            overall_score,
            health_status,
            score_components,
            last_updated: SystemTime::now(),
        };

        // Generate real alert summary based on current system state
        let mut critical_alerts = 0;
        let mut warning_alerts = 0;
        let mut info_alerts = 0;
        let mut recent_alerts = Vec::new();

        // Check for critical conditions
        if current_metrics.system_metrics._cpu_usage > 95.0 {
            critical_alerts += 1;
            recent_alerts.push(AlertInfo {
                id: uuid::Uuid::new_v4().to_string(),
                title: "Critical CPU Usage".to_string(),
                description: format!("CPU usage has reached {"actual_error_details"}%"),
                severity: AlertSeverity::Critical,
                category: "resource".to_string(),
                timestamp: SystemTime::now(),
                resolved: false,
                source: "system_monitor".to_string(),
            });
        }

        if current_metrics.pool_metrics.utilization_percent > 95.0 {
            critical_alerts += 1;
            recent_alerts.push(AlertInfo {
                id: uuid::Uuid::new_v4().to_string(),
                title: "Critical Storage Capacity".to_string(),
                description: format!("Storage utilization has reached {"actual_error_details"}%"),
                severity: AlertSeverity::Critical,
                category: "storage".to_string(),
                timestamp: SystemTime::now(),
                resolved: false,
                source: "pool_monitor".to_string(),
            });
        }

        if current_metrics.system_metrics.memory_usage > 95.0 {
            critical_alerts += 1;
            recent_alerts.push(AlertInfo {
                id: uuid::Uuid::new_v4().to_string(),
                title: "Critical Memory Usage".to_string(),
                description: format!("Memory usage has reached {"actual_error_details"}%"),
                severity: AlertSeverity::Critical,
                category: "resource".to_string(),
                timestamp: SystemTime::now(),
                resolved: false,
                source: "system_monitor".to_string(),
            });
        }

        // Check for warning conditions
        if current_metrics.system_metrics._cpu_usage > 80.0 && current_metrics.system_metrics._cpu_usage <= 95.0 {
            warning_alerts += 1;
            recent_alerts.push(AlertInfo {
                id: uuid::Uuid::new_v4().to_string(),
                title: "High CPU Usage".to_string(),
                description: format!("CPU usage is at {"actual_error_details"}%"),
                severity: AlertSeverity::Warning,
                category: "resource".to_string(),
                timestamp: SystemTime::now(),
                resolved: false,
                source: "system_monitor".to_string(),
            });
        }

        if current_metrics.pool_metrics.utilization_percent > 85.0 && current_metrics.pool_metrics.utilization_percent <= 95.0 {
            warning_alerts += 1;
            recent_alerts.push(AlertInfo {
                id: uuid::Uuid::new_v4().to_string(),
                title: "High Storage Usage".to_string(),
                description: format!("Storage utilization is at {"actual_error_details"}%"),
                severity: AlertSeverity::Warning,
                category: "storage".to_string(),
                timestamp: SystemTime::now(),
                resolved: false,
                source: "pool_monitor".to_string(),
            });
        }

        if current_metrics.pool_metrics.avg_latency_ms > 50.0 {
            warning_alerts += 1;
            recent_alerts.push(AlertInfo {
                id: uuid::Uuid::new_v4().to_string(),
                title: "High I/O Latency".to_string(),
                description: format!("Average I/O latency is {"actual_error_details"}ms"),
                severity: AlertSeverity::Warning,
                category: "performance".to_string(),
                timestamp: SystemTime::now(),
                resolved: false,
                source: "io_monitor".to_string(),
            });
        }

        if current_metrics.system_metrics.memory_usage > 85.0 && current_metrics.system_metrics.memory_usage <= 95.0 {
            warning_alerts += 1;
            recent_alerts.push(AlertInfo {
                id: uuid::Uuid::new_v4().to_string(),
                title: "High Memory Usage".to_string(),
                description: format!("Memory usage is at {"actual_error_details"}%"),
                severity: AlertSeverity::Warning,
                category: "resource".to_string(),
                timestamp: SystemTime::now(),
                resolved: false,
                source: "system_monitor".to_string(),
            });
        }

        // Check for info conditions
        if current_metrics.pool_metrics.fragmentation_percent > 30.0 {
            info_alerts += 1;
            recent_alerts.push(AlertInfo {
                id: uuid::Uuid::new_v4().to_string(),
                title: "Pool Fragmentation".to_string(),
                description: format!("Pool fragmentation is at {"actual_error_details"}%"),
                severity: AlertSeverity::Info,
                category: "maintenance".to_string(),
                timestamp: SystemTime::now(),
                resolved: false,
                source: "pool_monitor".to_string(),
            });
        }

        if current_metrics.pool_metrics.cache_hit_ratio < 85.0 {
            info_alerts += 1;
            recent_alerts.push(AlertInfo {
                id: uuid::Uuid::new_v4().to_string(),
                title: "Low Cache Hit Ratio".to_string(),
                description: format!("Cache hit ratio is {"actual_error_details"}%"),
                severity: AlertSeverity::Info,
                category: "performance".to_string(),
                timestamp: SystemTime::now(),
                resolved: false,
                source: "cache_monitor".to_string(),
            });
        }

        // Sort recent alerts by severity (critical first)
        recent_alerts.sort_by(|a, b| {
            match (&a.severity, &b.severity) {
                (AlertSeverity::Critical, AlertSeverity::Critical) => std::cmp::Ordering::Equal,
                (AlertSeverity::Critical, _) => std::cmp::Ordering::Less,
                (_, AlertSeverity::Critical) => std::cmp::Ordering::Greater,
                (AlertSeverity::Warning, AlertSeverity::Warning) => std::cmp::Ordering::Equal,
                (AlertSeverity::Warning, AlertSeverity::Info) => std::cmp::Ordering::Less,
                (AlertSeverity::Info, AlertSeverity::Warning) => std::cmp::Ordering::Greater,
                (AlertSeverity::Info, AlertSeverity::Info) => std::cmp::Ordering::Equal,
            }
        });

        // Convert AlertInfo to DashboardAlert
        let dashboard_alerts: Vec<DashboardAlert> = recent_alerts.into_iter().map(|alert| {
            let alert_type = match alert.category.as_str() {
                "resource" => AlertType::System,
                "storage" => AlertType::Capacity,
                "performance" => AlertType::Performance,
                "maintenance" => AlertType::Health,
                _ => AlertType::System,
            };
            
            let severity = match alert.severity {
                AlertSeverity::Critical => InsightSeverity::Critical,
                AlertSeverity::Warning => InsightSeverity::Warning,
                AlertSeverity::Info => InsightSeverity::Low,
            };
            
            DashboardAlert {
                id: alert.id,
                alert_type,
                severity,
                title: alert.title,
                description: alert.description,
                timestamp: alert.timestamp,
                acknowledged: alert.resolved,
            }
        }).collect();

        let alert_summary = AlertSummary {
            critical_alerts,
            warning_alerts,
            info_alerts,
            recent_alerts: dashboard_alerts,
        };

        Ok(DashboardOverview {
            timestamp: SystemTime::now(),
            time_range,
            current_metrics,
            performance_analysis,
            optimization_recommendations,
            insights,
            health_score,
            capacity_forecast: CapacityForecast {
                current_usage_percentage: 0.0,
                projected_usage_in_30_days: 0.0,
                projected_usage_in_90_days: 0.0,
                growth_points: vec![],
                recommendations: vec![],
            }
            alert_summary,
        })
    }
}

// HTTP Handler Functions

