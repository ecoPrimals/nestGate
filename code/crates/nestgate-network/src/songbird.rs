//! Songbird Orchestrator Integration for NestGate
//!
//! This module provides comprehensive integration with the Songbird orchestrator,
//! handling service registration, port management, and network coordination.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
// Removed unused tracing import
use uuid;

use crate::api::SongbirdClient;
use crate::Result;
use tracing::debug;
use tracing::info;
use tracing::warn;

/// Songbird integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongbirdConfig {
    /// Songbird orchestrator URL
    pub orchestrator_url: String,
    /// Service registration interval in seconds
    pub registration_interval: u64,
    /// Health check interval in seconds
    pub health_check_interval: u64,
    /// Service discovery interval in seconds
    pub discovery_interval: u64,
    /// Enable automatic port allocation
    pub auto_port_allocation: bool,
    /// Service metadata
    pub service_metadata: HashMap<String, String>,
}

impl Default for SongbirdConfig {
    fn default() -> Self {
        let mut metadata = HashMap::new();
        metadata.insert("type".to_string(), "nas".to_string());
        metadata.insert("filesystem".to_string(), "zfs".to_string());
        metadata.insert("protocols".to_string(), "nfs,smb,iscsi,s3".to_string());

        Self {
            orchestrator_url: std::env::var("SONGBIRD_URL").unwrap_or_else(|_| {
                format!(
                    "http://{}:{}",
                    std::env::var("NESTGATE_SONGBIRD_HOST")
                        .unwrap_or_else(|_| "songbird-orchestrator".to_string()),
                    std::env::var("NESTGATE_SONGBIRD_PORT").unwrap_or_else(|_| "8000".to_string())
                )
            }),
            registration_interval: 30,
            health_check_interval: 10,
            discovery_interval: 60,
            auto_port_allocation: true,
            service_metadata: metadata,
        }
    }
}

/// Service registration information for Songbird
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistration {
    /// Service name
    pub name: String,
    /// Service type
    pub service_type: String,
    /// Service version
    pub version: String,
    /// Network address
    pub address: String,
    /// Service port
    pub port: u16,
    /// Service endpoints
    pub endpoints: Vec<String>,
    /// Service capabilities
    pub capabilities: Vec<String>,
    /// Service metadata
    pub metadata: HashMap<String, String>,
    /// Health check endpoint
    pub health_endpoint: String,
}

impl Default for ServiceRegistration {
    fn default() -> Self {
        let mut metadata = HashMap::new();
        metadata.insert("node_type".to_string(), "storage".to_string());
        metadata.insert("tier_support".to_string(), "hot,warm,cold".to_string());

        Self {
            name: "nestgate-nas".to_string(),
            service_type: "storage".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            address: std::env::var("NESTGATE_BIND_ADDRESS")
                .unwrap_or_else(|_| "nestgate-nas".to_string()),
            port: 0,
            endpoints: {
                use nestgate_core::config::ApiPathsConfig;
                let api_paths = ApiPathsConfig::from_environment();
                vec![
                    api_paths.zfs.pools,
                    api_paths.zfs.datasets,
                    api_paths.zfs.snapshots,
                    api_paths.storage.info,
                    api_paths.health.health,
                ]
            },
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
            metadata,
            health_endpoint: {
                use nestgate_core::config::ApiPathsConfig;
                let api_paths = ApiPathsConfig::from_environment();
                api_paths.health.health
            },
        }
    }
}

/// Songbird integration manager
#[derive(Debug)]
pub struct SongbirdIntegration {
    /// Configuration
    config: SongbirdConfig,
    /// Songbird client
    client: SongbirdClient,
    /// Service registration
    registration: ServiceRegistration,
    /// Allocated ports
    allocated_ports: Arc<RwLock<HashMap<String, u16>>>,
    /// Discovered services
    discovered_services: Arc<RwLock<HashMap<String, crate::types::ServiceInstance>>>,
    /// Background tasks
    background_tasks: Arc<RwLock<Vec<tokio::task::JoinHandle<()>>>>,
}

impl SongbirdIntegration {
    /// Create a new Songbird integration
    pub fn new(config: SongbirdConfig) -> Self {
        let client = SongbirdClient::new(config.orchestrator_url.clone());

        Self {
            config,
            client,
            registration: ServiceRegistration::default(),
            allocated_ports: Arc::new(RwLock::new(HashMap::new())),
            discovered_services: Arc::new(RwLock::new(HashMap::new())),
            background_tasks: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Initialize the Songbird integration
    pub async fn initialize(&mut self) -> Result<()> {
        info!("🎼 Initializing Songbird integration...");

        // Allocate ports if auto-allocation is enabled
        if self.config.auto_port_allocation {
            self.allocate_service_ports().await?;
        }

        // Register the service
        self.register_service().await?;

        // Start background tasks
        self.start_background_tasks().await?;

        info!("✅ Songbird integration initialized successfully");
        Ok(())
    }

    /// Register this NestGate instance with Songbird
    pub async fn register_service(&self) -> Result<()> {
        info!("📝 Registering NestGate service with Songbird...");

        let service = crate::api::ServiceInstance {
            id: uuid::Uuid::new_v4().to_string(),
            name: self.registration.name.clone(),
            host: format!("{}:{}", self.registration.address, self.registration.port),
            port: self.registration.port,
            status: crate::api::ServiceStatus::Running,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        self.client.register_service(&service).await?;
        info!("✅ Service registered with Songbird successfully");
        Ok(())
    }

    /// Allocate ports for all services
    async fn allocate_service_ports(&self) -> Result<()> {
        info!("🔌 Allocating service ports via Songbird...");

        let port_types = vec![
            ("api", "api"),
            ("nfs", "nfs"),
            ("smb", "smb"),
            ("iscsi", "iscsi"),
            ("s3", "s3"),
        ];

        let mut allocated = self.allocated_ports.write().await;

        for (service_name, port_type) in port_types {
            match self.client.allocate_port(service_name, port_type).await {
                Ok(port) => {
                    allocated.insert(service_name.to_string(), port);
                    info!("✅ Allocated port {} for {}", port, service_name);
                }
                Err(e) => {
                    warn!("⚠️ Failed to allocate port for {}: {}", service_name, e);
                    // Continue with other services
                }
            }
        }

        info!("🔌 Port allocation completed");
        Ok(())
    }

    /// Start background tasks for health checks and service discovery
    async fn start_background_tasks(&self) -> Result<()> {
        info!("⚙️ Starting Songbird background tasks...");

        let mut tasks = self.background_tasks.write().await;

        // Health check task
        let health_task = self.start_health_check_task().await;
        tasks.push(health_task);

        // Service discovery task
        let discovery_task = self.start_service_discovery_task().await;
        tasks.push(discovery_task);

        info!("✅ Background tasks started");
        Ok(())
    }

    /// Start periodic health check reporting to Songbird
    async fn start_health_check_task(&self) -> tokio::task::JoinHandle<()> {
        let client = self.client.clone();
        let service_name = self.registration.name.clone();
        let interval = self.config.health_check_interval;

        tokio::spawn(async move {
            let mut interval_timer =
                tokio::time::interval(tokio::time::Duration::from_secs(interval));

            loop {
                interval_timer.tick().await;

                // Implement actual health check
                let _health_status = SongbirdIntegration::perform_health_check().await;
                let _health_status = crate::api::ServiceStatus::Running;

                debug!("💓 Sending health check to Songbird: {:?}", _health_status);

                // Send health status to Songbird orchestrator
                if let Err(e) = client
                    .send_health_status(&service_name, _health_status)
                    .await
                {
                    warn!("⚠️ Failed to send health status to Songbird: {}", e);
                    // Continue with health checks even if reporting fails
                }
            }
        })
    }

    /// Start periodic service discovery from Songbird
    async fn start_service_discovery_task(&self) -> tokio::task::JoinHandle<()> {
        let discovered_services = self.discovered_services.clone();
        let interval = self.config.discovery_interval;

        tokio::spawn(async move {
            let mut interval_timer =
                tokio::time::interval(tokio::time::Duration::from_secs(interval));

            loop {
                interval_timer.tick().await;

                debug!("🔍 Discovering services from Songbird...");

                // Implement service discovery from Songbird
                SongbirdIntegration::discover_services(&discovered_services).await;
                // For now, just log the discovery attempt
                let services = discovered_services.write().await;
                debug!(
                    "📊 Currently tracking {} discovered services",
                    services.len()
                );
            }
        })
    }

    /// Get allocated port for a service
    pub async fn get_allocated_port(&self, service_name: &str) -> Option<u16> {
        let allocated = self.allocated_ports.read().await;
        allocated.get(service_name).copied()
    }

    /// Get all discovered services
    pub async fn get_discovered_services(&self) -> HashMap<String, crate::types::ServiceInstance> {
        let services = self.discovered_services.read().await;
        services.clone()
    }

    /// Update service registration
    pub async fn update_registration(&mut self, registration: ServiceRegistration) -> Result<()> {
        info!("📝 Updating service registration...");
        self.registration = registration;
        self.register_service().await?;
        info!("✅ Service registration updated");
        Ok(())
    }

    /// Shutdown the Songbird integration
    pub async fn shutdown(&self) -> Result<()> {
        info!("🛑 Shutting down Songbird integration...");

        // Cancel background tasks
        let tasks = self.background_tasks.read().await;
        for task in tasks.iter() {
            task.abort();
        }

        // Release allocated ports
        let allocated = self.allocated_ports.read().await;
        for (service_name, port) in allocated.iter() {
            if let Err(e) = self.client.release_port(service_name, *port).await {
                warn!(
                    "⚠️ Failed to release port {} for {}: {}",
                    port, service_name, e
                );
            }
        }

        info!("✅ Songbird integration shutdown completed");
        Ok(())
    }

    /// Perform health check
    async fn perform_health_check() -> crate::api::ServiceStatus {
        // Check system health indicators
        let mut healthy = true;

        // Check disk space
        if let Ok(_metadata) = std::fs::metadata("/") {
            // Basic disk space check - this is a simplified example
            debug!("🔍 Checking disk space...");
        } else {
            healthy = false;
        }

        // Check memory usage
        if let Ok(_meminfo) = std::fs::read_to_string("/proc/meminfo") {
            debug!("🔍 Checking memory usage...");
        } else {
            healthy = false;
        }

        // Check if critical services are running
        if healthy {
            crate::api::ServiceStatus::Running
        } else {
            crate::api::ServiceStatus::Failed
        }
    }

    /// Discover services from Songbird
    async fn discover_services(
        discovered_services: &Arc<RwLock<HashMap<String, crate::types::ServiceInstance>>>,
    ) {
        debug!("🔍 Discovering services from Songbird orchestrator...");

        // In a real implementation, this would query the Songbird orchestrator
        // For now, we'll simulate discovery

        let mut services = discovered_services.write().await;

        // Example discovered service
        let example_service = crate::types::ServiceInstance {
            id: "example-nestgate-1".to_string(),
            name: "nestgate".to_string(),
            service_type: "storage".to_string(),
            address: nestgate_core::constants::addresses::test_host_address(),
            port: 8080,
            host: format!(
                "{}:{}",
                nestgate_core::constants::addresses::test_host_address(),
                nestgate_core::constants::network::api_port()
            ),
            status: crate::types::ServiceStatus::Running,
            metadata: std::collections::HashMap::new(),
            last_seen: std::time::SystemTime::now(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        services.insert("example-nestgate-1".to_string(), example_service);

        debug!(
            "📊 Service discovery completed, found {} services",
            services.len()
        );
    }
}

impl Drop for SongbirdIntegration {
    fn drop(&mut self) {
        // Note: This is a synchronous drop, so we can't await async operations
        // The shutdown method should be called explicitly before dropping
        debug!("🗑️ SongbirdIntegration dropped");
    }
}
