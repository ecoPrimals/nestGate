///
/// This module contains all the type definitions for the zero-cost orchestration client,
/// separated from the main implementation for better modularity and maintainability.
use nestgate_core::zero_cost::ZeroCostServiceHealth;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

// ==================== ZERO-COST NETWORK TYPES ====================

/// Zero-cost orchestration client configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCostOrchestrationConfig {
    /// Base URL for orchestration endpoint
    pub base_url: String,
    /// Connection timeout in seconds
    pub connection_timeout_s: u64,
    /// Request timeout in seconds
    pub request_timeout_s: u64,
    /// Maximum concurrent connections
    pub max_connections: usize,
    /// Retry attempts for failed requests
    pub retry_attempts: u32,
    /// Enable connection pooling
    pub enable_pooling: bool,
    /// Health check interval in seconds
    pub health_check_interval_s: u64,
}

impl Default for ZeroCostOrchestrationConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:8080".to_string(),
            connection_timeout_s: 30,
            request_timeout_s: 60,
            max_connections: 100,
            retry_attempts: 3,
            enable_pooling: true,
            health_check_interval_s: 30,
        }
    }
}

/// Zero-cost orchestration health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCostOrchestrationHealth {
    /// Overall service health
    pub service_health: ZeroCostServiceHealth,
    /// Number of active connections
    pub active_connections: usize,
    /// Total requests processed
    pub total_requests: u64,
    /// Failed requests count
    pub failed_requests: u64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Connection pool utilization (0.0 to 1.0)
    pub pool_utilization: f64,
    /// Last successful health check
    pub last_health_check: SystemTime,
}

impl Into<ZeroCostServiceHealth> for ZeroCostOrchestrationHealth {
    fn into(self) -> ZeroCostServiceHealth {
        self.service_health
    }
}

/// Zero-cost service registration data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCostServiceRegistration {
    /// Service identifier
    pub service_id: String,
    /// Service name
    pub service_name: String,
    /// Service host
    pub host: String,
    /// Service port
    pub port: u16,
    /// Service metadata
    pub metadata: HashMap<String, String>,
    /// Service tags
    pub tags: Vec<String>,
    /// Health check endpoint
    pub health_endpoint: Option<String>,
}

/// Zero-cost service instance data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCostServiceInstance {
    /// Instance identifier
    pub instance_id: String,
    /// Service name
    pub service_name: String,
    /// Instance host
    pub host: String,
    /// Instance port
    pub port: u16,
    /// Instance status
    pub status: ZeroCostServiceStatus,
    /// Last seen timestamp
    pub last_seen: SystemTime,
    /// Instance metadata
    pub metadata: HashMap<String, String>,
}

/// Zero-cost service status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ZeroCostServiceStatus {
    Starting,
    Running,
    Degraded,
    Stopping,
    Stopped,
    Failed,
}

/// Zero-cost orchestration statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCostOrchestrationStats {
    /// Client identifier
    pub client_id: String,
    /// Client name
    pub client_name: String,
    /// Uptime in seconds
    pub uptime_seconds: u64,
    /// Total service registrations
    pub total_registrations: u64,
    /// Total service discoveries
    pub total_discoveries: u64,
    /// Total port allocations
    pub total_port_allocations: u64,
    /// Total health checks performed
    pub total_health_checks: u64,
    /// Failed operations count
    pub failed_operations: u64,
    /// Total response time in milliseconds
    pub total_response_time_ms: f64,
    /// Last health check timestamp
    pub last_health_check: Option<SystemTime>,
}
