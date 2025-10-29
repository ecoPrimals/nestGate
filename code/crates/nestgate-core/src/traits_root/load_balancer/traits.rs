// **LOAD BALANCER TRAITS - CANONICAL MODERNIZATION**
//! Trait definitions and implementations.
// Core traits and types for load balancing functionality.
// Extracted from monolithic file for better maintainability.

use std::collections::HashMap;

use crate::universal_traits::{ServiceInfo, ServiceRequest, ServiceResponse};
use crate::Result;

use super::stats::LoadBalancerStats;

/// **CANONICAL TRAIT**: Core load balancer interface
pub trait LoadBalancer: Send + Sync {
    /// Select a service from the available services for the given request
    fn select_service(
        &self,
        services: &[ServiceInfo],
        request: &ServiceRequest,
    ) -> Result<ServiceInfo>;
    /// Record the response from a service for load balancing decisions
    fn record_response(
        &self,
        service: &ServiceInfo,
        response: &ServiceResponse,
    ) -> Result<()>;

    /// Update service weights (for weighted algorithms)
    async fn update_weights(&self, weights: HashMap<String, f64>) -> Result<()> {
        let _ = weights; // Default implementation ignores weights
        Ok(())
    }

    /// Get load balancer statistics
    fn get_stats(&self) -> Result<LoadBalancerStats>;

    /// Get the algorithm name
    fn algorithm(&self) -> &'static str;
}

/// **CANONICAL ENUM**: Load balancer implementation variants
#[derive(Debug)]
pub enum LoadBalancerImpl {
    RoundRobin(super::algorithms::RoundRobinLoadBalancer),
    WeightedRoundRobin(super::algorithms::WeightedRoundRobinLoadBalancer),
    LeastConnections(super::algorithms::LeastConnectionsLoadBalancer),
    Random(super::algorithms::RandomLoadBalancer),
    WeightedRandom(super::algorithms::WeightedRandomLoadBalancer),
    HealthAware(Box<super::implementations::HealthAwareLoadBalancerImpl>),
}
/// **CANONICAL ENUM**: Load balancing algorithm types
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LoadBalancingAlgorithm {
    RoundRobin,
    WeightedRoundRobin,
    LeastConnections,
    Random,
    WeightedRandom,
    HealthAware,
}
