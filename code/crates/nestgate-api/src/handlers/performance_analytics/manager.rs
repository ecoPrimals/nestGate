//! Performance Analytics Manager
//!
//! Main performance analytics manager struct with lifecycle management.

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
// Removed unused tracing import
use chrono::Utc;

use super::types::*;
use super::collectors::collect_system_metrics;
use super::analytics::{check_alerts, generate_recommendations};
use tracing::info;
use tracing::error;

/// Performance analytics manager
pub struct PerformanceAnalytics {
    /// Configuration
    config: Arc<RwLock<PerformanceConfig>>,
    /// Historical metrics storage
    metrics_history: Arc<RwLock<Vec<SystemMetrics>>>,
    /// Active alerts
    active_alerts: Arc<RwLock<Vec<PerformanceAlert>>>,
    /// Performance recommendations
    recommendations: Arc<RwLock<Vec<PerformanceRecommendation>>>,
    /// Collection task handle
    collection_task: Option<tokio::task::JoinHandle<()>>,
}

impl PerformanceAnalytics {
    /// Create new performance analytics manager
    pub fn new(config: PerformanceConfig) -> Self {
        Self {
            config: Arc::new(RwLock::new(config)),
            metrics_history: Arc::new(RwLock::new(Vec::new())),
            active_alerts: Arc::new(RwLock::new(Vec::new())),
            recommendations: Arc::new(RwLock::new(Vec::new())),
            collection_task: None,
        }
    }

    /// Start performance monitoring
    pub async fn start_monitoring(
        &mut self,
    ) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("🔬 Starting performance analytics monitoring");

        let config = self.config.clone();
        let metrics_history = self.metrics_history.clone();
        let active_alerts = self.active_alerts.clone();
        let recommendations = self.recommendations.clone();

        let collection_task = tokio::spawn(async move {
            let mut interval =
                tokio::time::interval(Duration::from_secs(config.read().await.collection_interval));

            loop {
                interval.tick().await;

                match collect_system_metrics().await {
                    Ok(metrics) => {
                        // Store metrics
                        {
                            let mut history = metrics_history.write().await;
                            history.push(metrics.clone());

                            // Limit history size based on retention
                            let retention_hours = config.read().await.retention_days as u64 * 24;
                            let max_entries =
                                retention_hours * 3600 / config.read().await.collection_interval;

                            let len = history.len();
                            if len > max_entries as usize {
                                history.drain(0..(len - max_entries as usize));
                            }
                        }

                        // Check for alerts
                        if let Ok(alerts) =
                            check_alerts(&metrics, &*config.read().await).await
                        {
                            if !alerts.is_empty() {
                                let mut active = active_alerts.write().await;
                                active.extend(alerts);

                                // Keep only recent alerts (last 24 hours)
                                let cutoff = Utc::now() - chrono::Duration::hours(24);
                                active.retain(|alert| alert.timestamp > cutoff);
                            }
                        }

                        // Generate recommendations
                        if config.read().await.predictive_enabled {
                            if let Ok(recs) = generate_recommendations(
                                &metrics,
                                &metrics_history.read().await,
                            )
                            .await
                            {
                                if !recs.is_empty() {
                                    let mut recommendations_guard = recommendations.write().await;
                                    for rec in recs {
                                        // Only add if not already present
                                        if !recommendations_guard
                                            .iter()
                                            .any(|r| r.title == rec.title)
                                        {
                                            recommendations_guard.push(rec);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => error!("Failed to collect system metrics: {}", e),
                }
            }
        });

        self.collection_task = Some(collection_task);
        Ok(())
    }

    /// Stop performance monitoring
    pub async fn stop_monitoring(&mut self) {
        if let Some(task) = self.collection_task.take() {
            task.abort();
            info!("⏹️ Stopped performance analytics monitoring");
        }
    }

    /// Get current system metrics
    pub async fn get_current_metrics(
        &self,
    ) -> std::result::Result<SystemMetrics, Box<dyn std::error::Error + Send + Sync>> {
        collect_system_metrics().await
    }

    /// Get historical metrics
    pub async fn get_historical_metrics(&self, hours: u32) -> Vec<SystemMetrics> {
        let history = self.metrics_history.read().await;
        let cutoff = Utc::now() - chrono::Duration::hours(hours as i64);

        history
            .iter()
            .filter(|metrics| metrics.timestamp > cutoff)
            .cloned()
            .collect()
    }

    /// Get active alerts
    pub async fn get_active_alerts(&self) -> Vec<PerformanceAlert> {
        self.active_alerts.read().await.clone()
    }

    /// Get performance recommendations
    pub async fn get_recommendations(&self) -> Vec<PerformanceRecommendation> {
        self.recommendations.read().await.clone()
    }
} 