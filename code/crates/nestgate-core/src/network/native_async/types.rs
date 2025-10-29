/// Native Async Network Types
/// Extracted from `native_async_network.rs` to maintain file size compliance
/// Contains data structures, enums, and configuration types
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
/// Service event for discovery watching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEvent {
    pub event_type: ServiceEventType,
    pub service_id: String,
    pub service_info: Option<crate::diagnostics::types::ServiceInfo>,
    pub timestamp: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}
/// Service event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceEventType {
    Registered,
    Deregistered,
    HealthChanged,
    MetadataUpdated,
    ConfigurationChanged,
}
/// Service query for filtering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceQuery {
    pub service_name: Option<String>,
    pub tags: Vec<String>,
    pub namespace: Option<String>,
    pub healthy_only: bool,
    pub metadata_filters: HashMap<String, String>,
}
/// Network connection information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConnection {
    pub connection_id: String,
    pub protocol: String,
    pub local_endpoint: String,
    pub established_at: DateTime<Utc>,
    pub status: ConnectionStatus,
    pub metadata: HashMap<String, String>,
}
/// Connection status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Connecting,
    Connected,
    Disconnected,
    Error(String),
}
/// Network request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkRequest {
    pub request_id: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
    pub timeout: Option<Duration>,
}
/// Network response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkResponse {
    pub request_id: String,
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
    pub processing_time: Duration,
}
/// Load balancer backend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerBackend {
    pub backend_id: String,
    pub endpoint: String,
    pub port: u16,
    pub weight: u32,
    pub healthy: bool,
    pub response_time_ms: f64,
    pub active_connections: u32,
    pub metadata: HashMap<String, String>,
}
