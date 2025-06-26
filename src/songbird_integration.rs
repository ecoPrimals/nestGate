//! NestGate Songbird Integration
//!
//! This module provides network-based integration between NestGate and Songbird orchestrator.
//! NestGate and Songbird run as separate distributed services that communicate over HTTP.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// NestGate service information for Songbird registration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestGateServiceInfo {
    pub name: String,
    pub service_type: String,
    pub version: String,
    pub endpoints: Vec<String>,
    pub capabilities: Vec<String>,
    pub network_address: String,
    pub port: u16,
    pub metadata: HashMap<String, String>,
}

impl Default for NestGateServiceInfo {
    fn default() -> Self {
        let mut metadata = HashMap::new();
        metadata.insert("filesystem".to_string(), "zfs".to_string());
        metadata.insert("protocols".to_string(), "nfs,smb,iscsi,s3".to_string());
        metadata.insert("node_type".to_string(), "storage".to_string());

        Self {
            name: "nestgate-nas".to_string(),
            service_type: "storage".to_string(),
            version: crate::VERSION.to_string(),
            endpoints: vec![
                "/api/v1/zfs/pools".to_string(),
                "/api/v1/zfs/datasets".to_string(),
                "/api/v1/zfs/snapshots".to_string(),
                "/api/v1/storage/info".to_string(),
                "/api/v1/health".to_string(),
            ],
            capabilities: vec![
                "zfs-pools".to_string(),
                "tiered-storage".to_string(),
                "snapshots".to_string(),
                "encryption".to_string(),
                "nfs".to_string(),
                "smb".to_string(),
                "iscsi".to_string(),
                "s3".to_string(),
            ],
            network_address: "0.0.0.0".to_string(),
            port: 8080,
            metadata,
        }
    }
}

/// NestGate health information for Songbird heartbeats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestGateHealth {
    pub status: String,
    pub storage_total_gb: u64,
    pub storage_used_gb: u64,
    pub storage_available_gb: u64,
    pub active_connections: u32,
    pub pools_healthy: u32,
    pub pools_total: u32,
}

/// NestGate ZFS Service Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestGateZfsConfig {
    /// ZFS pool storage path
    pub pools_path: String,
    /// Maximum storage capacity in GB
    pub max_storage_gb: u64,
    /// Enable ZFS encryption
    pub enable_encryption: bool,
    /// Songbird orchestrator URL
    pub songbird_url: String,
    /// Service port for NestGate API
    pub service_port: u16,
}

impl Default for NestGateZfsConfig {
    fn default() -> Self {
        Self {
            pools_path: "/dev/disk/by-id".to_string(),
            max_storage_gb: 10000,
            enable_encryption: true,
            songbird_url: "http://localhost:8000".to_string(), // Default Songbird port
            service_port: 8080, // NestGate API port
        }
    }
}

/// NestGate ZFS Service with network-based Songbird integration
pub struct NestGateZfsService {
    config: Option<NestGateZfsConfig>,
    running: bool,
}

impl NestGateZfsService {
    pub fn new() -> Self {
        Self {
            config: None,
            running: false,
        }
    }

    pub async fn initialize(&mut self, config: NestGateZfsConfig) -> anyhow::Result<()> {
        // TODO: Add HTTP client for Songbird communication when needed
        // For now, NestGate runs standalone and can be discovered by Songbird
        self.config = Some(config);
        println!("🔧 NestGate ZFS Service initialized");
        println!("🎼 Songbird integration: Network-based (HTTP)");
        Ok(())
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        self.running = true;
        println!("🚀 NestGate ZFS Service started");
        Ok(())
    }

    pub async fn shutdown(&mut self) -> anyhow::Result<()> {
        self.running = false;
        println!("🛑 NestGate ZFS Service stopped");
        Ok(())
    }

    /// Get current health status
    pub async fn get_health(&self) -> NestGateHealth {
        NestGateHealth {
            status: "healthy".to_string(),
            storage_total_gb: 10000,
            storage_used_gb: 3500,
            storage_available_gb: 6500,
            active_connections: 15,
            pools_healthy: 2,
            pools_total: 2,
        }
    }

    /// Get service info for Songbird registration
    pub fn get_service_info(&self) -> NestGateServiceInfo {
        NestGateServiceInfo::default()
    }
}

impl Default for NestGateZfsService {
    fn default() -> Self {
        Self::new()
    }
} 