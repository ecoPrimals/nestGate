//! Load Balancer Core Traits and Types
//!
//! **MIGRATED FROM**: `traits::load_balancing::traits` (November 7, 2025)
//! **CANONICAL**: This is the single source of truth for load balancing traits
//! **STATUS**: Production-ready, native async

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::universal_traits::{ServiceInfo, ServiceRequest, ServiceResponse};
use crate::Result;

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
pub enum LoadBalancingAlgorithm {
    RoundRobin,
    WeightedRoundRobin,
    LeastConnections,
    Random,
    WeightedRandom,
    HealthAware,
}

/// Load balancer statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerStats {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time: f64,
    pub service_stats: HashMap<String, ServiceStats>,
    pub algorithm: String,
    pub health_aware: bool,
}

/// Statistics for individual services
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ServiceStats {
    pub requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time: f64,
    pub current_load: f64,
}

impl Default for ServiceStats {
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

