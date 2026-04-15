// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Connection pool, health monitoring, and infrastructure types for the unified
//! RPC stack. These types preserve the public API shape while transport, real
//! pooling, and background tasks are wired incrementally.

#![expect(dead_code, reason = "Scaffold fields reserved for upcoming RPC wiring")]

use super::super::config::{
    ConnectionPoolConfig, HealthMonitoringConfig, LoadBalancingConfig, RpcSecurityConfig,
    StreamConfig,
};
use super::super::types::{RpcError, UnifiedRpcRequest};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use uuid::Uuid;

/// Connection pool for managing RPC connections
#[derive(Debug, Clone)]
pub struct ConnectionPool {
    connections: HashMap<String, Vec<ConnectionInfo>>,
    max_connections_per_service: usize,
    connection_timeout: Duration,
}

/// Connection information
#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    id: Uuid,
    service_name: String,
    endpoint: String,
    status: ConnectionStatus,
    last_used: Instant,
    created_at: Instant,
}

/// Connection status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionStatus {
    /// Connection is healthy and available
    Healthy,
    /// Connection is degraded but functional
    Degraded,
    /// Connection is unhealthy
    Unhealthy,
    /// Connection is closed
    Closed,
}

/// Monitors health of RPC connections and services.
#[derive(Debug, Clone)]
pub struct ConnectionHealthMonitor {
    health_checks: HashMap<String, HealthCheckResult>,
    check_interval: Duration,
    unhealthy_threshold: u32,
}

/// Result of a health check for a specific service.
#[derive(Debug, Clone)]
pub struct HealthCheckResult {
    service_name: String,
    is_healthy: bool,
    response_time_ms: u64,
    last_check: Instant,
    consecutive_failures: u32,
    error_message: Option<String>,
}

impl ConnectionPool {
    /// Create a connection pool from the given configuration.
    #[must_use]
    pub fn new(config: &ConnectionPoolConfig) -> Self {
        Self {
            connections: HashMap::new(),
            max_connections_per_service: config.max_connections,
            connection_timeout: config.connection_timeout,
        }
    }
}

impl ConnectionHealthMonitor {
    /// Create a health monitor from the given configuration.
    #[must_use]
    pub fn new(config: &HealthMonitoringConfig) -> Self {
        Self {
            health_checks: HashMap::new(),
            check_interval: config.check_interval,
            unhealthy_threshold: config.unhealthy_threshold,
        }
    }
}

/// Passthrough security layer — accepts all requests until the security
/// capability provider is wired via capability-based discovery.
#[derive(Debug, Clone)]
pub struct UniversalSecurityLayer;

/// Passthrough load balancer — round-robin is handled by the caller until
/// a real LB strategy is wired.
#[derive(Debug, Clone)]
pub struct LoadBalancer;

/// Passthrough stream registry — channels are created on demand until
/// persistent stream management is wired.
#[derive(Debug, Clone)]
pub struct StreamRegistry;

/// No-op metrics collector — request recording is a no-op until a real
/// metrics sink (Prometheus, OpenTelemetry) is integrated.
#[derive(Debug, Clone)]
pub struct MetricsCollector;

impl UniversalSecurityLayer {
    /// Create a passthrough security layer from configuration.
    #[must_use]
    pub const fn new(_config: &RpcSecurityConfig) -> Self {
        Self
    }

    /// Validate an incoming RPC request (passthrough until crypto IPC wired).
    ///
    /// # Errors
    ///
    /// Returns `RpcError` if validation fails (currently always succeeds).
    pub const fn validate_request(&self, _request: &UnifiedRpcRequest) -> Result<(), RpcError> {
        Ok(())
    }

    /// Check rate limits for a request source (passthrough).
    ///
    /// # Errors
    ///
    /// Returns `RpcError` if rate limit exceeded (currently always succeeds).
    pub const fn check_rate_limit(&self, _source: &str) -> Result<(), RpcError> {
        Ok(())
    }
}

impl LoadBalancer {
    /// Create a passthrough load balancer from configuration.
    #[must_use]
    pub const fn new(_config: &LoadBalancingConfig) -> Self {
        Self
    }
}

impl StreamRegistry {
    /// Create a passthrough stream registry from configuration.
    #[must_use]
    pub const fn new(_config: &StreamConfig) -> Self {
        Self
    }
}

impl MetricsCollector {
    /// Create a no-op metrics collector from configuration.
    #[must_use]
    pub const fn new(
        _config: &nestgate_core::config::canonical_primary::domains::performance::MetricsConfig,
    ) -> Self {
        Self
    }

    /// Record a request metric (no-op until real metrics sink wired).
    pub const fn record_request(&self, _service: &str, _duration: Duration) {}
}

/// ⚠️ DEPRECATED: Consolidated into `canonical_primary`.
///
/// Use `nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig`.
/// This type alias will be maintained until v0.12.0 (May 2026).
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
pub struct MetricsConfig {
    /// Whether metrics collection is enabled.
    pub enabled: bool,
    /// Interval between metrics collection cycles.
    pub interval_seconds: u64,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval_seconds: 60,
        }
    }
}

/// Canonical metrics configuration type alias.
pub type MetricsConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
