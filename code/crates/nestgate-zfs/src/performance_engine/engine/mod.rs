// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]
//
// This module contains the main performance optimization engine that coordinates
// all ZFS performance monitoring and optimization activities.

//! Engine module

mod bottlenecks;
mod optimization;
mod startup;

use std::sync::Arc;

use crate::{config::ZfsConfig, dataset::ZfsDatasetManager, pool::ZfsPoolManager};

use super::monitoring::RealTimePerformanceMonitor;
use super::types::{OptimizationState, PerformanceEngineConfig};

/// Real-time Performance Optimization Engine
///
/// Monitors ZFS performance in real-time and applies optimizations based on:
/// - `NestGate`'s deep ZFS storage expertise
/// - Ecosystem AI recommendations for optimization strategies
/// - Real-time performance metrics and bottleneck detection
#[derive(Debug)]
pub struct PerformanceOptimizationEngine {
    config: ZfsConfig,
    pool_manager: Arc<ZfsPoolManager>,
    dataset_manager: Arc<ZfsDatasetManager>,
    // Real-time performance monitoring
    performance_monitor: Arc<RealTimePerformanceMonitor>,
    optimization_state: Arc<tokio::sync::RwLock<OptimizationState>>,

    // Configuration
    engine_config: PerformanceEngineConfig,
}

impl PerformanceOptimizationEngine {
    /// Creates a new performance optimization engine with the given configuration and managers.
    #[must_use]
    pub fn new(
        config: ZfsConfig,
        pool_manager: Arc<ZfsPoolManager>,
        dataset_manager: Arc<ZfsDatasetManager>,
    ) -> Self {
        Self {
            config,
            pool_manager,
            dataset_manager,
            performance_monitor: Arc::new(RealTimePerformanceMonitor::new()),
            optimization_state: Arc::new(tokio::sync::RwLock::new(OptimizationState::default())),
            engine_config: PerformanceEngineConfig::default(),
        }
    }
}

impl Clone for PerformanceOptimizationEngine {
    /// Clone
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            pool_manager: Arc::clone(&self.pool_manager),
            dataset_manager: Arc::clone(&self.dataset_manager),
            performance_monitor: Arc::clone(&self.performance_monitor),
            optimization_state: Arc::clone(&self.optimization_state),
            engine_config: self.engine_config.clone(),
        }
    }
}

#[cfg(test)]
impl PerformanceOptimizationEngine {
    pub(crate) async fn test_detect_bottlenecks(
        &self,
    ) -> crate::error::Result<Vec<super::types::ZfsBottleneck>> {
        Self::detect_and_analyze_bottlenecks(
            &self.performance_monitor,
            &self.pool_manager,
            &self.dataset_manager,
        )
        .await
    }
}

#[cfg(test)]
mod tests;
