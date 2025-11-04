//! # Load Balancer Traits and Core Types
//! Trait definitions and implementations.
// Core traits and data structures for load balancing functionality

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::universal_traits::{ServiceInfo, ServiceRequest, ServiceResponse};
use crate::Result;

/// Load balancer trait
pub trait LoadBalancer: Send + Sync {
    /// Select a service instance for a request
    fn select_service(
        &self,
        services: &[ServiceInfo],
        request: &ServiceRequest,
    ) -> impl std::future::Future<Output = Result<ServiceInfo>> + Send;
    /// Record the response for learning
    fn record_response(
        &self,
        service: &ServiceInfo,
        response: &ServiceResponse,
    ) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Update service weights
    fn update_weights(
        &self,
        weights: HashMap<String, f64>,
    ) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Get load balancer statistics
    fn get_stats(&self) -> impl std::future::Future<Output = Result<LoadBalancerStats>> + Send;

    /// Get algorithm name
    fn algorithm(&self) -> &'static str;
}

/// Load balancing algorithms
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
