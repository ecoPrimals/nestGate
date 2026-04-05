// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]

use crate::types::StorageTier;
use nestgate_core::Result as CoreResult;
/// Alert management, notifications, and public metrics access API
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{RwLock, mpsc};
use tokio::time::interval;
use tracing::{debug, error};

use super::super::types::{
    ActiveAlert, Alert, CurrentPerformanceMetrics, PerformanceSnapshot, TierPerformanceData,
    ZfsPerformanceMonitor,
};
use super::super::types::{ActiveAlertsVec, AlertConditionsVec};

impl ZfsPerformanceMonitor {
    /// Start alert task
    pub(super) fn start_alert_task(&mut self) -> CoreResult<()> {
        let current_metrics = Arc::clone(&self.current_metrics);
        let alert_conditions = Arc::clone(&self.alert_conditions);
        let active_alerts = Arc::clone(&self.active_alerts);
        let alert_sender = self.alert_sender.clone();
        // Use default alert interval since config was removed
        let alert_interval = 60; // 60 seconds default

        let task = tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(alert_interval));

            loop {
                interval.tick().await;

                if let Some(sender) = &alert_sender
                    && let Err(e) = Self::check_alert_conditions(
                        &current_metrics,
                        &alert_conditions,
                        &active_alerts,
                        sender,
                    )
                {
                    error!("Alert checking failed: {}", e);
                }
            }
        });

        self.alert_task = Some(task);
        Ok(())
    }

    /// Check alert conditions
    pub(super) fn check_alert_conditions(
        _current_metrics: &Arc<RwLock<CurrentPerformanceMetrics>>,
        _alert_conditions: &AlertConditionsVec,
        _active_alerts: &ActiveAlertsVec,
        _alert_sender: &mpsc::Sender<Alert>,
    ) -> CoreResult<()> {
        debug!("Checking alert conditions");

        // Implementation would check current metrics against alert conditions
        // and send alerts when thresholds are exceeded
        Ok(())
    }

    /// Get current performance metrics
    pub async fn get_current_metrics(&self) -> CurrentPerformanceMetrics {
        self.current_metrics.read().await.clone()
    }

    /// Get metrics history
    pub async fn get_metrics_history(&self) -> Vec<PerformanceSnapshot> {
        self.metrics_history.read().await.iter().cloned().collect()
    }

    /// Get tier performance data
    pub async fn get_tier_performance(&self, tier: StorageTier) -> Option<TierPerformanceData> {
        self.tier_metrics.read().await.get(&tier).cloned()
    }

    /// Get active alerts
    pub async fn get_active_alerts(&self) -> Vec<ActiveAlert> {
        self.active_alerts.read().await.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::performance::ZfsPerformanceMonitor;
    use crate::types::StorageTier;

    #[tokio::test]
    async fn reporting_getters_return_cloneable_defaults() {
        let m = ZfsPerformanceMonitor::new_for_testing();
        let cur = m.get_current_metrics().await;
        assert_eq!(cur.trends.prediction_confidence, 0.5);
        assert!(m.get_metrics_history().await.is_empty());
        assert!(m.get_active_alerts().await.is_empty());
        assert!(m.get_tier_performance(StorageTier::Hot).await.is_none());
    }
}
