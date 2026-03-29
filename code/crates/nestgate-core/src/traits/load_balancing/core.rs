// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Load Balancer Core Traits and Types
//!
//! **MIGRATED FROM**: `traits::load_balancing::traits` (November 7, 2025)
//! **CANONICAL**: This is the single source of truth for load balancing traits
//! **STATUS**: Production-ready, native async

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::Result;
use crate::universal_traits::{ServiceInfo, ServiceRequest, ServiceResponse};

/// Load balancer trait for distributing requests across services
///
/// This trait provides the core interface for all load balancing algorithms.
/// Implementations should be thread-safe and efficient.
pub trait LoadBalancer: Send + Sync {
    /// Select a service instance for a request
    ///
    /// # Arguments
    ///
    /// * `services` - Available service instances
    /// * `request` - The incoming request to route
    ///
    /// # Returns
    ///
    /// The selected service instance or an error if none are available
    fn select_service(
        &self,
        services: &[ServiceInfo],
        request: &ServiceRequest,
    ) -> impl std::future::Future<Output = Result<ServiceInfo>> + Send;

    /// Record the response for learning and statistics
    ///
    /// This allows the load balancer to track service performance
    /// and make better routing decisions.
    fn record_response(
        &self,
        service: &ServiceInfo,
        response: &ServiceResponse,
    ) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Update service weights (for weighted algorithms)
    ///
    /// Not all algorithms use weights. Default implementation does nothing.
    fn update_weights(
        &self,
        weights: HashMap<String, f64>,
    ) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Get load balancer statistics
    fn get_stats(&self) -> impl std::future::Future<Output = Result<LoadBalancerStats>> + Send;

    /// Get algorithm name
    fn algorithm(&self) -> &'static str;
}

/// Load balancing algorithms enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Loadbalancingalgorithm
pub enum LoadBalancingAlgorithm {
    /// Roundrobin
    RoundRobin,
    /// Weightedroundrobin
    WeightedRoundRobin,
    /// Leastconnections
    LeastConnections,
    /// Random
    Random,
    /// Weightedrandom
    WeightedRandom,
    /// Healthaware
    HealthAware,
}

/// Load balancer statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Loadbalancerstats
pub struct LoadBalancerStats {
    /// Total Requests
    pub total_requests: u64,
    /// Successful Requests
    pub successful_requests: u64,
    /// Failed Requests
    pub failed_requests: u64,
    /// Average Response Time
    pub average_response_time: f64,
    /// Service Stats
    pub service_stats: HashMap<String, ServiceStats>,
    /// Algorithm
    pub algorithm: String,
    /// Health Aware
    pub health_aware: bool,
}

/// Statistics for individual services
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
/// Servicestats
pub struct ServiceStats {
    /// Requests
    pub requests: u64,
    /// Successful Requests
    pub successful_requests: u64,
    /// Failed Requests
    pub failed_requests: u64,
    /// Average Response Time
    pub average_response_time: f64,
    /// Current Load
    pub current_load: f64,
}

impl Default for ServiceStats {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            average_response_time: 0.0,
            current_load: 0.0,
        }
    }
}

impl Default for LoadBalancerStats {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            average_response_time: 0.0,
            service_stats: HashMap::new(),
            algorithm: "round_robin".to_string(),
            health_aware: false,
        }
    }
}
