//! Network types and structures

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Service instance in the network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInstance {
    pub id: String,
    pub name: String,
    pub service_type: String,
    pub address: String,
    pub port: u16,
    pub host: String, // For backward compatibility
    pub status: ServiceStatus,
    pub metadata: HashMap<String, String>,
    pub last_seen: SystemTime,
    pub created_at: chrono::DateTime<chrono::Utc>, // For compatibility
    pub updated_at: chrono::DateTime<chrono::Utc>, // For compatibility
}

/// Service status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServiceStatus {
    Healthy,
    Unhealthy,
    Unknown,
    Starting,
    Stopping,
    Running,
    Failed,
}

/// Network statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStats {
    pub connections_active: u32,
    pub connections_total: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub errors_total: u64,
    pub last_updated: SystemTime,
}
