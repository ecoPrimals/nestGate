//! Scalability features for the orchestrator

use serde::{Deserialize, Serialize};

/// Scalability configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalabilityConfig {
    /// Maximum number of concurrent services
    pub max_services: usize,
    /// Service instance scaling configuration
    pub scaling: ServiceScalingConfig,
    /// Load balancing configuration
    pub load_balancing: LoadBalancingConfig,
}

impl Default for ScalabilityConfig {
    fn default() -> Self {
        Self {
            max_services: 100,
            scaling: ServiceScalingConfig::default(),
            load_balancing: LoadBalancingConfig::default(),
        }
    }
}

/// Service scaling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceScalingConfig {
    /// Enable auto-scaling
    pub auto_scaling: bool,
    /// Minimum instances per service
    pub min_instances: usize,
    /// Maximum instances per service
    pub max_instances: usize,
    /// CPU threshold for scaling up (percentage)
    pub scale_up_cpu_threshold: f64,
    /// CPU threshold for scaling down (percentage)
    pub scale_down_cpu_threshold: f64,
}

impl Default for ServiceScalingConfig {
    fn default() -> Self {
        Self {
            auto_scaling: true,
            min_instances: 1,
            max_instances: 10,
            scale_up_cpu_threshold: 80.0,
            scale_down_cpu_threshold: 20.0,
        }
    }
}

/// Load balancing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingConfig {
    /// Load balancing algorithm
    pub algorithm: LoadBalancingAlgorithm,
    /// Health check interval
    pub health_check_interval_ms: u64,
    /// Connection timeout
    pub connection_timeout_ms: u64,
}

impl Default for LoadBalancingConfig {
    fn default() -> Self {
        Self {
            algorithm: LoadBalancingAlgorithm::RoundRobin,
            health_check_interval_ms: 5000,
            connection_timeout_ms: 30000,
        }
    }
}

/// Load balancing algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancingAlgorithm {
    /// Round robin distribution
    RoundRobin,
    /// Weighted round robin
    WeightedRoundRobin,
    /// Least connections
    LeastConnections,
    /// Random selection
    Random,
} 