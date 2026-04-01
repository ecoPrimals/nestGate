// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Storage routing, failover, and load balancing configuration.
//!
//! Controls how requests are directed to backends, how failures trigger
//! failover, and how load is distributed across available backends.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Configuration for routing storage requests to backends
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageRoutingConfig {
    /// Routing rules (evaluated in priority order)
    pub rules: Vec<RoutingRule>,
    /// Default backend for unmatched requests
    pub default_backend: String,
    /// Enable content-based routing
    pub content_based_routing: bool,
}

impl Default for StorageRoutingConfig {
    fn default() -> Self {
        Self {
            rules: Vec::new(),
            default_backend: "filesystem".to_string(),
            content_based_routing: false,
        }
    }
}

/// A single routing rule mapping conditions to backends
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingRule {
    /// Rule name
    pub name: String,
    /// Rule condition
    pub condition: RoutingCondition,
    /// Target backend
    pub backend: String,
    /// Rule priority (higher = evaluated first)
    pub priority: u32,
}

/// Conditions that determine which backend handles a request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoutingCondition {
    /// Match based on path prefix
    PathPrefix(String),
    /// Match based on file extension
    FileExtension(String),
    /// Match based on file size
    FileSize(FileSizeCondition),
    /// Match based on content type
    ContentType(String),
    /// Custom routing condition
    Custom(String),
}

/// File size comparison for routing decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSizeCondition {
    /// Comparison operator
    pub operator: ComparisonOperator,
    /// Threshold size in bytes
    pub size: u64,
}

/// Comparison operators for routing conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonOperator {
    /// Strictly greater than
    GreaterThan,
    /// Strictly less than
    LessThan,
    /// Equal to
    Equal,
    /// Greater than or equal to
    GreaterThanOrEqual,
    /// Less than or equal to
    LessThanOrEqual,
}

/// Configuration for automatic backend failover
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageFailoverConfig {
    /// Enable automatic failover
    pub enabled: bool,
    /// Failover strategy
    pub strategy: FailoverStrategy,
    /// Failover timeout
    pub timeout: Duration,
    /// Health check configuration for failover decisions
    pub health_check: FailoverHealthCheckConfig,
}

impl Default for StorageFailoverConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            strategy: FailoverStrategy::RoundRobin,
            timeout: Duration::from_secs(30),
            health_check: FailoverHealthCheckConfig::default(),
        }
    }
}

/// Strategy for selecting failover targets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FailoverStrategy {
    /// Cycle through available backends
    RoundRobin,
    /// Use pre-configured priority ordering
    Priority,
    /// Weight-based selection
    Weighted,
    /// Geographic proximity-based selection
    Geolocation,
}

/// Health check parameters that drive failover decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailoverHealthCheckConfig {
    /// Health check interval
    pub interval: Duration,
    /// Consecutive failures before failover
    pub failure_threshold: u32,
    /// Consecutive successes before recovery
    pub recovery_threshold: u32,
}

impl Default for FailoverHealthCheckConfig {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(30),
            failure_threshold: 3,
            recovery_threshold: 2,
        }
    }
}

/// Load balancing configuration across multiple backends
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageLoadBalancingConfig {
    /// Load balancing algorithm
    pub algorithm: LoadBalancingAlgorithm,
    /// Sticky sessions
    pub sticky_sessions: bool,
    /// Session affinity timeout
    pub session_timeout: Duration,
    /// Per-backend weights
    pub weights: HashMap<String, u32>,
}

impl Default for StorageLoadBalancingConfig {
    fn default() -> Self {
        Self {
            algorithm: LoadBalancingAlgorithm::RoundRobin,
            sticky_sessions: false,
            session_timeout: Duration::from_secs(300),
            weights: HashMap::new(),
        }
    }
}

/// Algorithms for distributing load across backends
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancingAlgorithm {
    /// Simple round-robin
    RoundRobin,
    /// Weighted round-robin
    WeightedRoundRobin,
    /// Fewest active connections
    LeastConnections,
    /// Fastest response time
    LeastResponseTime,
    /// Random selection
    Random,
    /// Consistent hashing
    Consistent,
}
