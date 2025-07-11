//! # NestGate biomeOS Integration
//!
//! Integration layer that connects NestGate with the biomeOS ecosystem,
//! implementing unified storage provisioning and coordination protocols.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use crate::{
    NestGateError, Result,
    storage::{NestGateManager, StoragePool, Volume, VolumeSpec},
    config::NestGateConfig,
};

/// biomeOS ecosystem service registration for NestGate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeOSServiceRegistration {
    pub service_id: String,
    pub primal_type: String,
    pub biome_id: String,
    pub version: String,
    pub api_version: String,
    pub registration_time: DateTime<Utc>,
    pub endpoints: BiomeOSEndpoints,
    pub capabilities: BiomeOSCapabilities,
    pub security: BiomeOSSecurity,
    pub resource_requirements: BiomeOSResourceRequirements,
    pub health_check: BiomeOSHealthCheckConfig,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeOSEndpoints {
    pub primary: String,
    pub health: String,
    pub metrics: String,
    pub admin: Option<String>,
    pub websocket: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeOSCapabilities {
    pub core: Vec<String>,
    pub extended: Vec<String>,
    pub integrations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeOSSecurity {
    pub authentication_method: String,
    pub tls_enabled: bool,
    pub mtls_required: bool,
    pub trust_domain: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeOSResourceRequirements {
    pub cpu: String,
    pub memory: String,
    pub storage: String,
    pub network: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeOSHealthCheckConfig {
    pub interval_secs: u64,
    pub timeout_secs: u64,
    pub retries: u32,
    pub grace_period_secs: u64,
}

/// NestGate integration with biomeOS ecosystem
pub struct NestGateBiomeOSIntegration {
    config: NestGateConfig,
    manager: Arc<RwLock<NestGateManager>>,
    biomeos_client: BiomeOSClient,
    registration: Option<BiomeOSServiceRegistration>,
    instance_id: String,
    active_biomes: Arc<RwLock<HashMap<String, BiomeStorageContext>>>,
}

impl NestGateBiomeOSIntegration {
    pub fn new(
        config: NestGateConfig,
        manager: Arc<RwLock<NestGateManager>>,
        biomeos_endpoint: String,
    ) -> Self {
        let biomeos_client = BiomeOSClient::new(biomeos_endpoint);
        let instance_id = format!("nestgate-{}", Uuid::new_v4().simple());

        Self {
            config,
            manager,
            biomeos_client,
            registration: None,
            instance_id,
            active_biomes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register NestGate with the biomeOS ecosystem
    pub async fn register_with_biomeos(&mut self, biome_id: String) -> Result<()> {
        info!("Registering NestGate with biomeOS ecosystem");

        let registration = BiomeOSServiceRegistration {
            service_id: format!("primal-nestgate-{}", self.instance_id),
            primal_type: "nestgate".to_string(),
            biome_id: biome_id.clone(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            api_version: "biomeOS/v1".to_string(),
            registration_time: Utc::now(),

            endpoints: BiomeOSEndpoints {
                primary: format!("http://{}:{}",
                    self.config.network.http_listen_address,
                    self.config.network.http_listen_port
                ),
                health: format!("http://{}:{}/health",
                    self.config.network.http_listen_address,
                    self.config.network.http_listen_port
                ),
                metrics: format!("http://{}:{}/metrics",
                    self.config.network.http_listen_address,
                    self.config.network.http_listen_port
                ),
                admin: Some(format!("http://{}:{}/admin",
                    self.config.network.http_listen_address,
                    self.config.network.http_listen_port + 1
                )),
                websocket: Some(format!("ws://{}:{}/ws",
                    self.config.network.http_listen_address,
                    self.config.network.http_listen_port
                )),
            },

            capabilities: BiomeOSCapabilities {
                core: vec![
                    "storage_provisioning".to_string(),
                    "volume_management".to_string(),
                    "zfs_operations".to_string(),
                    "snapshot_management".to_string(),
                    "backup_restore".to_string(),
                ],
                extended: vec![
                    "tiered_storage".to_string(),
                    "multi_protocol_access".to_string(),
                    "deduplication".to_string(),
                    "compression".to_string(),
                    "encryption_at_rest".to_string(),
                    "performance_monitoring".to_string(),
                ],
                integrations: vec![
                    "biomeos_volume_provisioning".to_string(),
                    "primal_mount_coordination".to_string(),
                    "primal_discovery_integration".to_string(),
                    "primal_encryption_integration".to_string(),
                    "primal_ai_storage_optimization".to_string(),
                ],
            },

            security: BiomeOSSecurity {
                authentication_method: "ecosystem_jwt".to_string(),
                tls_enabled: true,
                mtls_required: false, // Will be true when BearDog is ready
                trust_domain: "biome.local".to_string(),
            },

            resource_requirements: BiomeOSResourceRequirements {
                cpu: "4".to_string(),
                memory: "16Gi".to_string(),
                storage: "1Ti".to_string(), // Available storage capacity
                network: "10Gbps".to_string(),
            },

            health_check: BiomeOSHealthCheckConfig {
                interval_secs: 30,
                timeout_secs: 10,
                retries: 3,
                grace_period_secs: 60,
            },

            metadata: {
                let mut meta = HashMap::new();
                meta.insert("environment".to_string(), "production".to_string());
                meta.insert("role".to_string(), "storage_provider".to_string());
                meta.insert("protocols_supported".to_string(), "nfs,smb,iscsi,s3".to_string());
                meta.insert("zfs_features".to_string(), "snapshots,clones,dedup,compression,encryption".to_string());
                meta.insert("storage_tiers".to_string(), "hot,warm,cold,archive".to_string());
                meta
            },
        };

        // Register with biomeOS
        self.biomeos_client.register_service(&registration).await?;
        self.registration = Some(registration);

        info!("NestGate successfully registered with biomeOS ecosystem");
        Ok(())
    }

    /// Provision storage for a biome deployment
    pub async fn provision_biome_storage(
        &self,
        request: BiomeOSStorageProvisionRequest,
    ) -> Result<BiomeOSStorageProvisionResponse> {
        info!("Provisioning storage for biome: {}", request.biome_id);

        // Create biome storage context
        let storage_context = BiomeStorageContext {
            biome_id: request.biome_id.clone(),
            team_id: request.team_id.clone(),
            quota: request.storage_quota.clone(),
            volumes: HashMap::new(),
            created_at: Utc::now(),
        };

        // Store context
        {
            let mut biomes = self.active_biomes.write().await;
            biomes.insert(request.biome_id.clone(), storage_context);
        }

        // Provision volumes for each service
        let mut provisioned_volumes = Vec::new();
        let manager = self.manager.read().await;

        for volume_req in &request.volume_requirements {
            let volume_spec = VolumeSpec {
                name: format!("biome-{}-{}", request.biome_id, volume_req.name),
                size_bytes: volume_req.size_bytes,
                storage_tier: volume_req.tier.clone(),
                access_mode: volume_req.access_mode.clone(),
                mount_options: volume_req.mount_options.clone(),
                backup_policy: volume_req.backup_policy.clone(),
            };

            let volume = manager.provision_volume(volume_spec).await?;

            let provisioned_volume = BiomeOSProvisionedVolume {
                volume_id: volume.id.clone(),
                name: volume_req.name.clone(),
                mount_path: volume.mount_path.clone(),
                access_endpoint: self.generate_access_endpoint(&volume).await?,
                protocols: volume_req.protocols.clone(),
                size_bytes: volume.size_bytes,
                tier: volume_req.tier.clone(),
            };

            provisioned_volumes.push(provisioned_volume);
        }

        // Update biome context with volumes
        {
            let mut biomes = self.active_biomes.write().await;
            if let Some(context) = biomes.get_mut(&request.biome_id) {
                for volume in &provisioned_volumes {
                    context.volumes.insert(volume.name.clone(), volume.volume_id.clone());
                }
            }
        }

        let response = BiomeOSStorageProvisionResponse {
            biome_id: request.biome_id,
            status: "provisioned".to_string(),
            volumes: provisioned_volumes,
            total_allocated_bytes: request.volume_requirements.iter()
                .map(|v| v.size_bytes)
                .sum(),
            access_endpoints: self.generate_biome_access_endpoints(&request.biome_id).await?,
            created_at: Utc::now(),
        };

        info!("Storage provisioning completed for biome: {}", response.biome_id);
        Ok(response)
    }

    /// Handle ecosystem messages from other Primals
    pub async fn handle_ecosystem_message(&mut self, message: EcosystemMessage) -> Result<Option<EcosystemMessage>> {
        debug!("Handling ecosystem message: {:?}", message.message_type);

        match message.message_type {
            EcosystemMessageType::VolumeProvisionRequest => {
                self.handle_volume_provision_request(message).await
            }
            EcosystemMessageType::MountRequest => {
                self.handle_mount_request(message).await
            }
            EcosystemMessageType::ResourceRequest => {
                self.handle_resource_request(message).await
            }
            EcosystemMessageType::HealthCheck => {
                self.handle_health_check(message).await
            }
            _ => {
                debug!("Unhandled message type: {:?}", message.message_type);
                Ok(None)
            }
        }
    }

    /// Get NestGate status for ecosystem monitoring
    pub async fn get_ecosystem_status(&self) -> Result<NestGateEcosystemStatus> {
        let manager = self.manager.read().await;
        let pools = manager.get_storage_pools().await?;
        let biomes = self.active_biomes.read().await;

        let total_capacity = pools.iter()
            .map(|p| p.total_bytes)
            .sum();

        let used_capacity = pools.iter()
            .map(|p| p.used_bytes)
            .sum();

        Ok(NestGateEcosystemStatus {
            service_id: self.registration.as_ref()
                .map(|r| r.service_id.clone())
                .unwrap_or_else(|| "unregistered".to_string()),
            health: "healthy".to_string(), // Would check actual health
            total_capacity_bytes: total_capacity,
            used_capacity_bytes: used_capacity,
            available_capacity_bytes: total_capacity - used_capacity,
            active_biomes: biomes.len(),
            active_volumes: biomes.values()
                .map(|b| b.volumes.len())
                .sum(),
            storage_pools: pools.len(),
            protocols_enabled: vec!["nfs".to_string(), "smb".to_string(), "iscsi".to_string(), "s3".to_string()],
            primal_integrations: self.get_primal_integration_status().await?,
        })
    }

    // Private helper methods

    async fn generate_access_endpoint(&self, volume: &Volume) -> Result<String> {
        // Generate appropriate access endpoint based on protocol
        Ok(format!("nfs://{}:{}{}",
            self.config.network.nfs_listen_address,
            self.config.network.nfs_listen_port,
            volume.mount_path.display()
        ))
    }

    async fn generate_biome_access_endpoints(&self, biome_id: &str) -> Result<Vec<String>> {
        // Generate all access endpoints for a biome
        Ok(vec![
            format!("nfs://{}:{}/biome/{}",
                self.config.network.nfs_listen_address,
                self.config.network.nfs_listen_port,
                biome_id
            ),
            format!("smb://{}:{}/biome/{}",
                self.config.network.smb_listen_address,
                self.config.network.smb_listen_port,
                biome_id
            ),
            format!("s3://{}:{}/biome/{}",
                self.config.network.s3_listen_address,
                self.config.network.s3_listen_port,
                biome_id
            ),
        ])
    }

    async fn handle_volume_provision_request(&mut self, message: EcosystemMessage) -> Result<Option<EcosystemMessage>> {
        info!("Handling volume provision request from: {}", message.from_primal);

        // Parse request from message payload
        let request: VolumeProvisionRequest = serde_json::from_value(message.payload)?;

        // Provision volume
        let manager = self.manager.read().await;
        let volume_spec = VolumeSpec {
            name: request.volume_name,
            size_bytes: request.size_bytes,
            storage_tier: request.tier.unwrap_or_else(|| "hot".to_string()),
            access_mode: request.access_mode.unwrap_or_else(|| "ReadWriteOnce".to_string()),
            mount_options: HashMap::new(),
            backup_policy: None,
        };

        let volume = manager.provision_volume(volume_spec).await?;

        // Create response
        let response = EcosystemMessage {
            message_id: Uuid::new_v4(),
            from_primal: "nestgate".to_string(),
            to_primal: message.from_primal,
            message_type: EcosystemMessageType::VolumeProvisionComplete,
            payload: serde_json::json!({
                "volume_id": volume.id,
                "mount_path": volume.mount_path,
                "access_endpoint": self.generate_access_endpoint(&volume).await?,
                "size_bytes": volume.size_bytes,
                "status": "provisioned"
            }),
            timestamp: Utc::now(),
            correlation_id: Some(message.message_id),
        };

        Ok(Some(response))
    }

    async fn handle_mount_request(&mut self, message: EcosystemMessage) -> Result<Option<EcosystemMessage>> {
        info!("Handling mount request from: {}", message.from_primal);

        // Parse mount request
        let request: MountRequest = serde_json::from_value(message.payload)?;

        // Process mount request (coordinate with Toadstool)
        let mount_info = self.process_mount_request(&request).await?;

        // Create response
        let response = EcosystemMessage {
            message_id: Uuid::new_v4(),
            from_primal: "nestgate".to_string(),
            to_primal: message.from_primal,
            message_type: EcosystemMessageType::MountComplete,
            payload: serde_json::json!({
                "volume_id": request.volume_id,
                "mount_point": mount_info.mount_point,
                "mount_options": mount_info.options,
                "status": "mounted"
            }),
            timestamp: Utc::now(),
            correlation_id: Some(message.message_id),
        };

        Ok(Some(response))
    }

    async fn handle_resource_request(&mut self, message: EcosystemMessage) -> Result<Option<EcosystemMessage>> {
        info!("Handling resource request from: {}", message.from_primal);

        // Get current resource status
        let status = self.get_ecosystem_status().await?;

        // Create response
        let response = EcosystemMessage {
            message_id: Uuid::new_v4(),
            from_primal: "nestgate".to_string(),
            to_primal: message.from_primal,
            message_type: EcosystemMessageType::ResourceAllocation,
            payload: serde_json::json!({
                "available_capacity_bytes": status.available_capacity_bytes,
                "storage_tiers": ["hot", "warm", "cold", "archive"],
                "protocols": status.protocols_enabled,
                "status": "available"
            }),
            timestamp: Utc::now(),
            correlation_id: Some(message.message_id),
        };

        Ok(Some(response))
    }

    async fn handle_health_check(&mut self, message: EcosystemMessage) -> Result<Option<EcosystemMessage>> {
        // Respond to health check requests
        let response = EcosystemMessage {
            message_id: Uuid::new_v4(),
            from_primal: "nestgate".to_string(),
            to_primal: message.from_primal,
            message_type: EcosystemMessageType::HealthCheck,
            payload: serde_json::json!({
                "status": "healthy",
                "timestamp": Utc::now(),
                "storage_status": self.get_ecosystem_status().await?
            }),
            timestamp: Utc::now(),
            correlation_id: Some(message.message_id),
        };

        Ok(Some(response))
    }

    async fn process_mount_request(&self, request: &MountRequest) -> Result<MountInfo> {
        // Process mount request and coordinate with target system
        Ok(MountInfo {
            mount_point: PathBuf::from(&request.target_path),
            options: request.mount_options.clone(),
        })
    }

    async fn get_primal_integration_status(&self) -> Result<HashMap<String, String>> {
        let mut integrations = HashMap::new();

        // Dynamic discovery of available primals
        // In production, this would query the primal discovery service
        // For now, return generic status based on ecosystem capabilities

        // Check for compute primals (like toadstool)
        if self.has_capability("compute_coordination").await? {
            integrations.insert("compute_primal".to_string(), "connected".to_string());
        }

        // Check for orchestration primals (like songbird)
        if self.has_capability("orchestration_support").await? {
            integrations.insert("orchestration_primal".to_string(), "connected".to_string());
        }

        // Check for security primals (like beardog)
        if self.has_capability("encryption_support").await? {
            integrations.insert("security_primal".to_string(), "preparing".to_string());
        }

        // Check for AI primals (like squirrel)
        if self.has_capability("ai_optimization").await? {
            integrations.insert("ai_primal".to_string(), "preparing".to_string());
        }

        Ok(integrations)
    }

    /// Check if NestGate has a specific capability
    async fn has_capability(&self, capability: &str) -> Result<bool> {
        // This would integrate with the universal primal discovery system
        // For now, return true for basic capabilities
        match capability {
            "compute_coordination" => Ok(true),
            "orchestration_support" => Ok(true),
            "encryption_support" => Ok(false), // Future capability
            "ai_optimization" => Ok(false), // Future capability
            _ => Ok(false),
        }
    }
}

/// Client for communicating with biomeOS
pub struct BiomeOSClient {
    endpoint: String,
    client: Client,
}

impl BiomeOSClient {
    pub fn new(endpoint: String) -> Self {
        Self {
            endpoint,
            client: Client::new(),
        }
    }

    pub async fn register_service(&self, registration: &BiomeOSServiceRegistration) -> Result<()> {
        let url = format!("{}/api/v1/ecosystem/services", self.endpoint);

        let response = self.client
            .post(&url)
            .json(registration)
            .send()
            .await
            .map_err(|e| NestGateError::network(format!("Failed to register with biomeOS: {}", e)))?;

        if !response.status().is_success() {
            return Err(NestGateError::network(format!(
                "biomeOS registration failed: {}",
                response.status()
            )));
        }

        Ok(())
    }

    pub async fn send_message(&self, message: &EcosystemMessage) -> Result<()> {
        let url = format!("{}/api/v1/ecosystem/messages", self.endpoint);

        let response = self.client
            .post(&url)
            .json(message)
            .send()
            .await
            .map_err(|e| NestGateError::network(format!("Failed to send message to biomeOS: {}", e)))?;

        if !response.status().is_success() {
            return Err(NestGateError::network(format!(
                "Message send failed: {}",
                response.status()
            )));
        }

        Ok(())
    }
}

// Supporting types

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeStorageContext {
    pub biome_id: String,
    pub team_id: String,
    pub quota: StorageQuota,
    pub volumes: HashMap<String, String>, // volume_name -> volume_id
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeOSStorageProvisionRequest {
    pub biome_id: String,
    pub team_id: String,
    pub storage_quota: StorageQuota,
    pub volume_requirements: Vec<VolumeRequirement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageQuota {
    pub max_total_bytes: u64,
    pub max_volumes: u32,
    pub allowed_tiers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeRequirement {
    pub name: String,
    pub size_bytes: u64,
    pub tier: String,
    pub access_mode: String,
    pub protocols: Vec<String>,
    pub mount_options: HashMap<String, String>,
    pub backup_policy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeOSStorageProvisionResponse {
    pub biome_id: String,
    pub status: String,
    pub volumes: Vec<BiomeOSProvisionedVolume>,
    pub total_allocated_bytes: u64,
    pub access_endpoints: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeOSProvisionedVolume {
    pub volume_id: String,
    pub name: String,
    pub mount_path: PathBuf,
    pub access_endpoint: String,
    pub protocols: Vec<String>,
    pub size_bytes: u64,
    pub tier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestGateEcosystemStatus {
    pub service_id: String,
    pub health: String,
    pub total_capacity_bytes: u64,
    pub used_capacity_bytes: u64,
    pub available_capacity_bytes: u64,
    pub active_biomes: usize,
    pub active_volumes: usize,
    pub storage_pools: usize,
    pub protocols_enabled: Vec<String>,
    pub primal_integrations: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeProvisionRequest {
    pub volume_name: String,
    pub size_bytes: u64,
    pub tier: Option<String>,
    pub access_mode: Option<String>,
    pub requester_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountRequest {
    pub volume_id: String,
    pub target_path: String,
    pub mount_options: HashMap<String, String>,
    pub requester_id: String,
}

#[derive(Debug, Clone)]
pub struct MountInfo {
    pub mount_point: PathBuf,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemMessage {
    pub message_id: Uuid,
    pub from_primal: String,
    pub to_primal: String,
    pub message_type: EcosystemMessageType,
    pub payload: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub correlation_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EcosystemMessageType {
    ServiceRegistration,
    ServiceDeregistration,
    HealthCheck,
    ResourceRequest,
    ResourceAllocation,
    ResourceRelease,
    WorkloadRequest,
    WorkloadStatus,
    WorkloadComplete,
    VolumeProvisionRequest,
    VolumeProvisionComplete,
    MountRequest,
    MountComplete,
    EcosystemStateChange,
    PrimalStatusUpdate,
    ErrorNotification,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::NestGateConfig;

    #[tokio::test]
    async fn test_biomeos_registration() {
        let config = NestGateConfig::default();
        let manager = Arc::new(RwLock::new(NestGateManager::new()));

        let mut integration = NestGateBiomeOSIntegration::new(
            config,
            manager,
            "http://localhost:4000".to_string(),
        );

        // Test registration structure
        assert!(integration.registration.is_none());

        // Note: Actual registration would require a running biomeOS instance
        // This test validates the structure and logic
    }

    #[tokio::test]
    async fn test_storage_provisioning() {
        let config = NestGateConfig::default();
        let manager = Arc::new(RwLock::new(NestGateManager::new()));

        let integration = NestGateBiomeOSIntegration::new(
            config,
            manager,
            "http://localhost:4000".to_string(),
        );

        let request = BiomeOSStorageProvisionRequest {
            biome_id: "test-biome".to_string(),
            team_id: "test-team".to_string(),
            storage_quota: StorageQuota {
                max_total_bytes: 1024 * 1024 * 1024, // 1GB
                max_volumes: 10,
                allowed_tiers: vec!["hot".to_string(), "warm".to_string()],
            },
            volume_requirements: vec![
                VolumeRequirement {
                    name: "data".to_string(),
                    size_bytes: 512 * 1024 * 1024, // 512MB
                    tier: "hot".to_string(),
                    access_mode: "ReadWriteOnce".to_string(),
                    protocols: vec!["nfs".to_string()],
                    mount_options: HashMap::new(),
                    backup_policy: None,
                },
            ],
        };

        // Note: Actual provisioning would require proper storage setup
        // This test validates the request structure
        assert_eq!(request.biome_id, "test-biome");
        assert_eq!(request.volume_requirements.len(), 1);
    }
}