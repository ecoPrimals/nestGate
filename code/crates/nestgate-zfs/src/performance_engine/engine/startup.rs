// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Engine startup: background monitoring, optimization, and bottleneck-detection loops.

use std::sync::Arc;
use std::time::Duration;

use nestgate_types::{EnvSource, env_parsed};
use tokio::time::interval;
use tracing::{debug, error, info};

use crate::error::Result;

use super::PerformanceOptimizationEngine;

impl PerformanceOptimizationEngine {
    /// Start the real-time performance optimization engine
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn start(&mut self) -> Result<()> {
        self.start_from_env_source(&nestgate_types::ProcessEnv)
            .await
    }

    /// Like [`Self::start`], but reads interval env vars from an injectable [`EnvSource`].
    pub async fn start_from_env_source(&mut self, env: &(impl EnvSource + ?Sized)) -> Result<()> {
        info!("Starting Real-time Performance Optimization Engine");

        // Start performance monitoring
        self.start_performance_monitoring(env)?;

        // Start optimization loop
        self.start_optimization_loop(env)?;

        // Start bottleneck detection
        self.start_bottleneck_detection(env)?;

        info!("Performance optimization engine started successfully");
        Ok(())
    }

    /// Start Performance Monitoring
    fn start_performance_monitoring(&self, env: &(impl EnvSource + ?Sized)) -> Result<()> {
        let monitor = self.performance_monitor.clone();
        let pool_manager = self.pool_manager.clone();
        let dataset_manager = self.dataset_manager.clone();
        let secs = env_parsed(
            env,
            "NESTGATE_ZFS_PERFORMANCE_MONITORING_INTERVAL_SECS",
            10_u64,
        );

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(secs));

            loop {
                interval.tick().await;

                if let Err(e) = monitor
                    .collect_metrics(&pool_manager, &dataset_manager)
                    .await
                {
                    error!("Performance monitoring error: {}", e);
                }
            }
        });
        Ok(())
    }

    /// Start Optimization Loop
    fn start_optimization_loop(&self, env: &(impl EnvSource + ?Sized)) -> Result<()> {
        let engine = Arc::new(self.clone());
        let secs = env_parsed(env, "NESTGATE_ZFS_OPTIMIZATION_INTERVAL_SECS", 60_u64);

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(secs));

            loop {
                interval.tick().await;

                if let Err(e) = engine.optimize_performance().await {
                    error!("Optimization loop error: {}", e);
                }
            }
        });
        Ok(())
    }

    /// Start Bottleneck Detection
    fn start_bottleneck_detection(&self, env: &(impl EnvSource + ?Sized)) -> Result<()> {
        let performance_monitor = self.performance_monitor.clone();
        let pool_manager = self.pool_manager.clone();
        let dataset_manager = self.dataset_manager.clone();
        let secs = env_parsed(
            env,
            "NESTGATE_ZFS_BOTTLENECK_DETECTION_INTERVAL_SECS",
            30_u64,
        );

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(secs));

            loop {
                interval.tick().await;

                // Comprehensive bottleneck detection
                if let Err(e) = Self::detect_and_analyze_bottlenecks(
                    &performance_monitor,
                    &pool_manager,
                    &dataset_manager,
                )
                .await
                {
                    error!("Bottleneck detection failed: {}", e);
                } else {
                    debug!("Bottleneck detection completed successfully");
                }
            }
        });
        Ok(())
    }
}
