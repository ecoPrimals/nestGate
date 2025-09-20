// **LOAD BALANCER STATISTICS - CANONICAL MODERNIZATION**
//! Stats functionality and utilities.
// Statistics and metrics for load balancer performance tracking.
// Extracted for focused responsibility and maintainability.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// **CANONICAL STRUCT**: Load balancer performance statistics
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
/// **CANONICAL STRUCT**: Statistics for individual services
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
