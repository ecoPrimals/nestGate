//! Configuration types for clustering
//!
//! This module contains configuration structs for cluster setup,
//! including cluster-wide settings and individual node configuration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

// ==================== CONFIGURATION TYPES ====================

/// Cluster configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterConfig {
    /// Cluster name
    pub cluster_name: String,
    /// Node identifier
    pub node_id: String,
    /// Bind Endpoint
    pub bind_endpoint: SocketAddr,
    /// Nodes
    pub nodes: Vec<ClusterNodeConfig>,
    /// Election Timeout Ms
    pub election_timeout_ms: u64,
    /// Heartbeat Interval Ms
    pub heartbeat_interval_ms: u64,
    /// Max Missed Heartbeats
    pub max_missed_heartbeats: u32,
    /// Discovery Enabled
    pub discovery_enabled: bool,
    /// Discovery Multicast Endpoint
    pub discovery_multicast_endpoint: String,
    /// Discovery Port
    pub discovery_port: u16,
    /// Encryption Enabled
    pub encryption_enabled: bool,
    /// Cluster Secret
    pub cluster_secret: Option<String>,
}

/// Individual cluster node configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterNodeConfig {
    /// Node identifier
    pub node_id: String,
    /// Endpoint
    pub endpoint: SocketAddr,
    /// Region
    pub region: Option<String>,
    /// Zone
    pub zone: Option<String>,
    /// Weight
    pub weight: u32,
    /// Tags
    pub tags: HashMap<String, String>,
}

// ==================== DEFAULT IMPLEMENTATIONS ====================

impl Default for ClusterConfig {
    fn default() -> Self {
        // Use environment variable or default
        use crate::config::environment::EnvironmentConfig;
        
        let env_config = EnvironmentConfig::from_env()
            .unwrap_or_else(|_| EnvironmentConfig::default());
        
        let bind_addr = env_config.network.host.clone();
        let port = env_config.network.port.get();
        let default_bind = format!("{}:{}", bind_addr, port);
        
        let bind_addr_str = std::env::var("NESTGATE_CLUSTER_BIND")
            .unwrap_or(default_bind);
        // Parse with fallback to default if invalid
        let bind_endpoint = bind_addr_str.parse().unwrap_or_else(|_| {
            tracing::warn!("Invalid NESTGATE_CLUSTER_BIND address '{}', using default", bind_addr_str);
            format!("{}:{}", bind_addr, port).parse().unwrap_or_else(|_| {
                // Final fallback using environment-driven defaults
                let fallback_host = std::env::var("NESTGATE_FALLBACK_HOST")
                    .unwrap_or_else(|_| bind_addr.clone());
                format!("{}:{}", fallback_host, port)
                    .parse()
                    .unwrap_or_else(|_| {
                        // Last resort: use loopback
                        format!("127.0.0.1:{}", port)
                            .parse()
                            .expect("Localhost fallback must be valid")
                    })
            })
        });
        
        let discovery_port = std::env::var("NESTGATE_DISCOVERY_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(crate::config::ports::DISCOVERY_SERVICE);
        
        Self {
            cluster_name: "nestgate-cluster".to_string(),
            node_id: uuid::Uuid::new_v4().to_string(),
            bind_endpoint,
            nodes: vec![],
            election_timeout_ms: 5000,
            heartbeat_interval_ms: 1000,
            max_missed_heartbeats: 3,
            discovery_enabled: true,
            discovery_multicast_endpoint: std::env::var("NESTGATE_MULTICAST_ADDR")
                .unwrap_or_else(|_| "224.0.0.1".to_string()),
            discovery_port,
            encryption_enabled: true,
            cluster_secret: None,
        }
    }
}
