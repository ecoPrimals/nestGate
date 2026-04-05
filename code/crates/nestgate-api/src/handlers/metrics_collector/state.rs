// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use std::sync::Arc;

use tokio::sync::broadcast;
use tokio::time::{Duration, Instant};

use crate::handlers::dashboard_types::DashboardEvent;

use super::types::RealTimeMetrics;

/// Shared state for the metrics collector
#[derive(Debug, Clone)]
pub struct MetricsCollectorState {
    /// Real-time metrics data
    pub current_metrics: Arc<tokio::sync::RwLock<Option<RealTimeMetrics>>>,
    /// Metrics collection interval
    pub collection_interval: Duration,
    /// Event broadcaster for real-time updates
    pub event_sender: Arc<broadcast::Sender<DashboardEvent>>,
    /// Last collection timestamp
    pub last_collection: Arc<tokio::sync::RwLock<Option<Instant>>>,
}

impl Default for MetricsCollectorState {
    fn default() -> Self {
        let (sender, _) = broadcast::channel(1000);
        Self {
            current_metrics: Arc::new(tokio::sync::RwLock::new(None)),
            collection_interval: Duration::from_secs(5),
            event_sender: Arc::new(sender),
            last_collection: Arc::new(tokio::sync::RwLock::new(None)),
        }
    }
}
