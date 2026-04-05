// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **ERROR RECOVERY AND RESILIENCE**
//!
//! Comprehensive error recovery patterns and resilience mechanisms for NestGate.

pub mod circuit_breaker;
pub mod graceful_degradation;
pub mod health_monitoring;
pub mod retry_strategy;

// Re-export main types
pub use circuit_breaker::{CircuitBreaker, CircuitBreakerConfig, CircuitState};
pub use graceful_degradation::{DegradationLevel, FallbackStrategy, GracefulDegradation};
pub use health_monitoring::{ComponentHealth, HealthCheck, HealthMonitor};
pub use retry_strategy::{ExponentialBackoff, RetryConfig, RetryStrategy};

#[cfg(test)]
mod recovery_edge_cases; // Nov 23, 2025 - P1-5 edge case tests

#[cfg(test)]
mod recovery_integration_tests;
