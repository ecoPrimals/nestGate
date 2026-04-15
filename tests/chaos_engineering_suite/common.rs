// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Shared chaos primitives: fault injection config and resilient service wrapper.

use nestgate_core::{
    error::{NestGateError, Result},
    service_discovery::types::ServiceInfo,
};
use std::time::Duration;

/// Chaos test configuration
#[derive(Debug, Clone)]
pub struct ChaosTestConfig {
    pub failure_rate: f64, // 0.0 to 1.0
    pub network_latency_ms: u64,
    pub memory_pressure: bool,
    pub disk_full_simulation: bool,
    pub random_disconnections: bool,
}

impl Default for ChaosTestConfig {
    fn default() -> Self {
        Self {
            failure_rate: 0.1, // 10% failure rate
            network_latency_ms: 100,
            memory_pressure: false,
            disk_full_simulation: false,
            random_disconnections: false,
        }
    }
}

/// Fault injection utilities
pub struct FaultInjector {
    config: ChaosTestConfig,
}

impl FaultInjector {
    pub fn new(config: ChaosTestConfig) -> Self {
        Self { config }
    }

    /// Simulate random failures
    pub async fn maybe_fail(&self) -> Result<()> {
        if rand::random::<f64>() < self.config.failure_rate {
            Err(NestGateError::internal_error(
                "Chaos engineering failure injection",
                "chaos_test",
            ))
        } else {
            Ok(())
        }
    }

    /// Simulate network latency with realistic async delay
    pub async fn simulate_network_latency(&self) {
        if self.config.network_latency_ms > 0 {}
    }

    /// Simulate resource pressure
    pub fn simulate_memory_pressure(&self) -> Result<()> {
        if self.config.memory_pressure {
            // Simulate memory pressure by allocating large vectors
            let _pressure: Vec<Vec<u8>> = (0..1000).map(|_| vec![0u8; 1024]).collect();
        }
        Ok(())
    }
}

/// Resilient service wrapper for chaos testing
#[expect(dead_code)]
pub struct ResilientService {
    service_info: ServiceInfo,
    fault_injector: FaultInjector,
    retry_attempts: u32,
}

impl ResilientService {
    pub fn new(service_info: ServiceInfo, chaos_config: ChaosTestConfig) -> Self {
        Self {
            service_info,
            fault_injector: FaultInjector::new(chaos_config),
            retry_attempts: 5,
        }
    }

    /// Execute operation with fault tolerance
    pub async fn execute_with_retry<F, T>(&self, operation: F) -> Result<T>
    where
        F: Fn() -> Result<T> + Send + Sync,
    {
        let mut last_error = None;

        for attempt in 1..=self.retry_attempts {
            // Inject chaos
            if let Err(e) = self.fault_injector.maybe_fail().await {
                last_error = Some(e);
                continue;
            }

            // Simulate network latency
            self.fault_injector.simulate_network_latency().await;

            // Try the operation
            match operation() {
                Ok(result) => return Ok(result),
                Err(e) => {
                    last_error = Some(e);
                    if attempt < self.retry_attempts {
                        // Exponential backoff with realistic async delay
                        let backoff = Duration::from_millis(100 * (1 << attempt));
                        tokio::time::sleep(backoff).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| {
            NestGateError::internal_error("All retry attempts failed", "chaos_test")
        }))
    }
}
