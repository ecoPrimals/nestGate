//! # BYOB (Bring Your Own Biome) Storage Provider for NestGate
//!
//! Provides storage management for team biome deployments using ZFS datasets.
//! Integrates with Songbird coordination and Toadstool compute execution.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use crate::{
    dataset::{DatasetInfo, ZfsDatasetManager},
    manager::ZfsManager,
    pool::ZfsPoolManager,
    snapshot::ZfsSnapshotManager,
};
use nestgate_core::{NestGateError, Result, StorageTier};

/// BYOB storage request from Songbird
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ByobStorageRequest {
    /// Unique deployment ID
    pub deployment_id: Uuid,
    /// Team identifier
    pub team_id: String,
    /// Deployment name
    pub deployment_name: String,
    /// Storage requirements for each service
    pub storage_requirements: HashMap<String, ServiceStorageRequirements>,
    /// Team storage quotas
    pub team_quotas: TeamStorageQuotas,
    /// Network configuration for mounts
    pub network_config: StorageNetworkConfig,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
}

/// Storage requirements for a single service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStorageRequirements {
    /// Service name
    pub service_name: String,
    /// Required storage in bytes
    pub storage_bytes: u64,
    /// Storage tier preference
    pub tier: StorageTier,
    /// Volume mounts
    pub volumes: Vec<VolumeRequirement>,
    /// Persistence requirements
    pub persistence: PersistenceRequirement,
    /// Access mode
    pub access_mode: StorageAccessMode,
}

/// Volume requirement specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeRequirement {
    /// Volume name
    pub name: String,
    /// Mount path in container
    pub mount_path: String,
    /// Volume size in bytes
    pub size_bytes: u64,
    /// Storage tier
    pub tier: StorageTier,
    /// Read-only flag
    pub read_only: bool,
    /// Backup policy
    pub backup_policy: Option<String>,
}

/// Persistence requirement levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PersistenceRequirement {
    /// Ephemeral storage (deleted after deployment)
    Ephemeral,
    /// Persistent storage (survives deployment restarts)
    Persistent,
    /// Shared storage (accessible by multiple services)
    Shared,
    /// Backup storage (automatically backed up)
    Backup,
}

/// Storage access modes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageAccessMode {
    /// Read-write access by single service
    ReadWriteOnce,
    /// Read-only access by multiple services
    ReadOnlyMany,
    /// Read-write access by multiple services
    ReadWriteMany,
}

/// Team storage quotas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamStorageQuotas {
    /// Maximum total storage in bytes
    pub max_total_storage: u64,
    /// Maximum storage per tier
    pub max_per_tier: HashMap<StorageTier, u64>,
    /// Maximum number of datasets
    pub max_datasets: u32,
    /// Maximum number of snapshots
    pub max_snapshots: u32,
    /// Maximum backup retention days
    pub max_backup_retention_days: u32,
}

/// Storage network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageNetworkConfig {
    /// Network name for storage access
    pub network_name: String,
    /// NFS export configuration
    pub nfs_config: Option<NfsExportConfig>,
    /// SMB share configuration
    pub smb_config: Option<SmbShareConfig>,
}

/// NFS export configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NfsExportConfig {
    /// Export path
    pub export_path: String,
    /// Allowed hosts
    pub allowed_hosts: Vec<String>,
    /// Export options
    pub options: HashMap<String, String>,
}

/// SMB share configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmbShareConfig {
    /// Share name
    pub share_name: String,
    /// Share path
    pub share_path: String,
    /// Access permissions
    pub permissions: HashMap<String, String>,
}

/// BYOB storage response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ByobStorageResponse {
    /// Deployment ID
    pub deployment_id: Uuid,
    /// Storage status
    pub status: StorageStatus,
    /// Created datasets
    pub datasets: HashMap<String, DatasetInfo>,
    /// Storage mounts
    pub mounts: HashMap<String, StorageMount>,
    /// Storage usage
    pub usage: StorageUsage,
    /// Network endpoints
    pub endpoints: HashMap<String, StorageEndpoint>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Updated timestamp
    pub updated_at: DateTime<Utc>,
}

/// Storage deployment status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageStatus {
    /// Storage is being provisioned
    Provisioning,
    /// Storage is ready
    Ready,
    /// Storage is being removed
    Removing,
    /// Storage is removed
    Removed,
    /// Storage provisioning failed
    Failed { error: String },
}

/// Storage mount information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageMount {
    /// Mount ID
    pub mount_id: String,
    /// Dataset path
    pub dataset_path: String,
    /// Mount point
    pub mount_point: String,
    /// Mount type (NFS, SMB, local)
    pub mount_type: String,
    /// Mount options
    pub options: HashMap<String, String>,
    /// Mount status
    pub status: MountStatus,
}

/// Mount status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MountStatus {
    /// Mount is active
    Active,
    /// Mount is inactive
    Inactive,
    /// Mount failed
    Failed { error: String },
}

/// Storage usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageUsage {
    /// Total allocated storage in bytes
    pub total_allocated: u64,
    /// Total used storage in bytes
    pub total_used: u64,
    /// Usage per tier
    pub usage_per_tier: HashMap<StorageTier, TierUsage>,
    /// Usage per dataset
    pub usage_per_dataset: HashMap<String, u64>,
}

/// Tier usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierUsage {
    /// Allocated storage in bytes
    pub allocated: u64,
    /// Used storage in bytes
    pub used: u64,
    /// Number of datasets
    pub dataset_count: u32,
    /// Compression ratio
    pub compression_ratio: f64,
}

/// Storage endpoint information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageEndpoint {
    /// Endpoint type (NFS, SMB)
    pub endpoint_type: String,
    /// Endpoint URL
    pub url: String,
    /// Access credentials
    pub credentials: Option<HashMap<String, String>>,
    /// Mount instructions
    pub mount_instructions: String,
}

/// Active storage deployment tracking
#[derive(Debug)]
struct ActiveStorageDeployment {
    /// Storage request
    request: ByobStorageRequest,
    /// Storage status
    status: StorageStatus,
    /// Created datasets
    datasets: HashMap<String, DatasetInfo>,
    /// Storage mounts
    mounts: HashMap<String, StorageMount>,
    /// Storage usage
    usage: StorageUsage,
    /// Created timestamp
    created_at: Instant,
    /// Updated timestamp
    updated_at: Instant,
}

/// BYOB storage provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ByobStorageConfig {
    /// Default storage pool
    pub default_pool: String,
    /// Maximum concurrent storage operations
    pub max_concurrent_operations: u32,
    /// Default dataset quota in bytes
    pub default_dataset_quota: u64,
    /// Default backup retention days
    pub default_backup_retention_days: u32,
    /// Storage monitoring interval
    pub monitoring_interval: Duration,
    /// Cleanup timeout
    pub cleanup_timeout: Duration,
}

impl Default for ByobStorageConfig {
    fn default() -> Self {
        Self {
            default_pool: "nestpool".to_string(),
            max_concurrent_operations: 20,
            default_dataset_quota: 100 * 1024 * 1024 * 1024, // 100GB
            default_backup_retention_days: 30,
            monitoring_interval: Duration::from_secs(30),
            cleanup_timeout: Duration::from_secs(300), // 5 minutes
        }
    }
}

/// BYOB storage provider trait
#[async_trait]
pub trait ByobStorageProvider: Send + Sync {
    /// Provision storage for a team deployment
    async fn provision_storage(
        &self,
        request: ByobStorageRequest,
    ) -> Result<ByobStorageResponse>;

    /// Get storage status
    async fn get_storage_status(
        &self,
        deployment_id: Uuid,
    ) -> Result<ByobStorageResponse>;

    /// Remove storage for a deployment
    async fn remove_storage(&self, deployment_id: Uuid) -> Result<()>;

    /// List storage deployments for a team
    async fn list_team_storage(&self, team_id: &str) -> Result<Vec<ByobStorageResponse>>;

    /// Get storage usage for a deployment
    async fn get_storage_usage(&self, deployment_id: Uuid) -> Result<StorageUsage>;

    /// Create storage snapshot
    async fn create_snapshot(
        &self,
        deployment_id: Uuid,
        snapshot_name: String,
    ) -> Result<String>;

    /// Restore from snapshot
    async fn restore_snapshot(
        &self,
        deployment_id: Uuid,
        snapshot_name: String,
    ) -> Result<()>;
}

/// ZFS-based BYOB storage provider
pub struct ZfsStorageProvider {
    /// ZFS manager for operations
    zfs_manager: Arc<ZfsManager>,
    /// Configuration
    config: ByobStorageConfig,
    /// Active deployments
    active_deployments: Arc<RwLock<HashMap<Uuid, ActiveStorageDeployment>>>,
    /// Team workspace tracking
    team_workspaces: Arc<RwLock<HashMap<String, TeamWorkspace>>>,
}

/// Team workspace information
#[derive(Debug, Clone)]
struct TeamWorkspace {
    /// Team ID
    team_id: String,
    /// Root dataset path
    root_dataset: String,
    /// Storage quotas
    quotas: TeamStorageQuotas,
    /// Active deployments
    active_deployments: Vec<Uuid>,
    /// Created timestamp
    created_at: Instant,
}

impl ZfsStorageProvider {
    /// Create a new ZFS storage provider
    pub fn new(zfs_manager: Arc<ZfsManager>, config: ByobStorageConfig) -> Self {
        Self {
            zfs_manager,
            config,
            active_deployments: Arc::new(RwLock::new(HashMap::new())),
            team_workspaces: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Validate storage request
    fn validate_storage_request(&self, request: &ByobStorageRequest) -> Result<()> {
        // Check total storage requirements against team quotas
        let total_storage: u64 = request.storage_requirements.values()
            .map(|req| req.storage_bytes)
            .sum();

        if total_storage > request.team_quotas.max_total_storage {
            return Err(NestGateError::Storage(format!(
                "Total storage requirement {} exceeds team quota {}",
                total_storage, request.team_quotas.max_total_storage
            )));
        }

        // Check per-tier quotas
        let mut tier_usage: HashMap<StorageTier, u64> = HashMap::new();
        for req in request.storage_requirements.values() {
            *tier_usage.entry(req.tier.clone()).or_insert(0) += req.storage_bytes;
        }

        for (tier, usage) in tier_usage {
            if let Some(quota) = request.team_quotas.max_per_tier.get(&tier) {
                if usage > *quota {
                    return Err(NestGateError::Storage(format!(
                        "Storage requirement {} for tier {:?} exceeds quota {}",
                        usage, tier, quota
                    )));
                }
            }
        }

        // Check dataset count
        let dataset_count = request.storage_requirements.len() as u32;
        if dataset_count > request.team_quotas.max_datasets {
            return Err(NestGateError::Storage(format!(
                "Dataset count {} exceeds team quota {}",
                dataset_count, request.team_quotas.max_datasets
            )));
        }

        Ok(())
    }

    /// Create team workspace if it doesn't exist
    async fn ensure_team_workspace(&self, team_id: &str, quotas: &TeamStorageQuotas) -> Result<String> {
        let mut workspaces = self.team_workspaces.write().await;

        if let Some(workspace) = workspaces.get(team_id) {
            return Ok(workspace.root_dataset.clone());
        }

        // Create root dataset for team
        let root_dataset = format!("{}/teams/{}", self.config.default_pool, team_id);
        
        info!("Creating team workspace: {}", root_dataset);
        
        self.zfs_manager.create_dataset(
            &format!("teams/{}", team_id),
            &self.config.default_pool,
            StorageTier::Warm,
        ).await.map_err(|e| NestGateError::Storage(format!(
            "Failed to create team workspace: {}", e
        )))?;

        // Set team quotas
        self.set_dataset_quota(&root_dataset, quotas.max_total_storage).await?;

        // Create workspace tracking
        let workspace = TeamWorkspace {
            team_id: team_id.to_string(),
            root_dataset: root_dataset.clone(),
            quotas: quotas.clone(),
            active_deployments: Vec::new(),
            created_at: Instant::now(),
        };

        workspaces.insert(team_id.to_string(), workspace);

        Ok(root_dataset)
    }

    /// Set dataset quota
    async fn set_dataset_quota(&self, dataset: &str, quota_bytes: u64) -> Result<()> {
        debug!("Setting quota for dataset {}: {} bytes", dataset, quota_bytes);

        let output = tokio::process::Command::new("zfs")
            .args(["set", &format!("quota={}", quota_bytes), dataset])
            .output()
            .await
            .map_err(|e| NestGateError::Storage(format!("Failed to set quota: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(NestGateError::Storage(format!(
                "Failed to set quota: {}", error_msg
            )));
        }

        Ok(())
    }

    /// Create dataset for service
    async fn create_service_dataset(
        &self,
        team_dataset: &str,
        service_name: &str,
        requirements: &ServiceStorageRequirements,
    ) -> Result<DatasetInfo> {
        let dataset_name = format!("{}/{}", team_dataset, service_name);
        
        info!("Creating service dataset: {} on tier {:?}", dataset_name, requirements.tier);

        // Create dataset
        let dataset_info = self.zfs_manager.create_dataset(
            service_name,
            team_dataset,
            requirements.tier.clone(),
        ).await.map_err(|e| NestGateError::Storage(format!(
            "Failed to create service dataset: {}", e
        )))?;

        // Set service-specific quota
        if requirements.storage_bytes > 0 {
            self.set_dataset_quota(&dataset_name, requirements.storage_bytes).await?;
        }

        // Create volume datasets for each volume requirement
        for volume in &requirements.volumes {
            let volume_dataset = format!("{}/{}", dataset_name, volume.name);
            
            self.zfs_manager.create_dataset(
                &volume.name,
                &dataset_name,
                volume.tier.clone(),
            ).await.map_err(|e| NestGateError::Storage(format!(
                "Failed to create volume dataset: {}", e
            )))?;

            // Set volume quota
            self.set_dataset_quota(&volume_dataset, volume.size_bytes).await?;
        }

        Ok(dataset_info)
    }

    /// Create storage mounts
    async fn create_storage_mounts(
        &self,
        deployment_id: Uuid,
        team_dataset: &str,
        requirements: &HashMap<String, ServiceStorageRequirements>,
        network_config: &StorageNetworkConfig,
    ) -> Result<HashMap<String, StorageMount>> {
        let mut mounts = HashMap::new();

        for (service_name, req) in requirements {
            for volume in &req.volumes {
                let mount_id = format!("{}-{}-{}", deployment_id, service_name, volume.name);
                let dataset_path = format!("{}/{}/{}", team_dataset, service_name, volume.name);
                let mount_point = volume.mount_path.clone();

                // Create NFS export if configured
                let mount_type = if network_config.nfs_config.is_some() {
                    "nfs".to_string()
                } else {
                    "local".to_string()
                };

                let storage_mount = StorageMount {
                    mount_id: mount_id.clone(),
                    dataset_path,
                    mount_point,
                    mount_type,
                    options: HashMap::new(),
                    status: MountStatus::Active,
                };

                mounts.insert(mount_id, storage_mount);
            }
        }

        Ok(mounts)
    }

    /// Calculate storage usage
    async fn calculate_storage_usage(
        &self,
        datasets: &HashMap<String, DatasetInfo>,
    ) -> Result<StorageUsage> {
        let mut total_used = 0u64;
        let mut total_allocated = 0u64;
        let mut usage_per_tier: HashMap<StorageTier, TierUsage> = HashMap::new();
        let mut usage_per_dataset = HashMap::new();

        for (name, dataset) in datasets {
            total_used += dataset.used_space;
            total_allocated += dataset.used_space + dataset.available_space;
            
            usage_per_dataset.insert(name.clone(), dataset.used_space);

            let tier_usage = usage_per_tier.entry(dataset.tier.clone()).or_insert(TierUsage {
                allocated: 0,
                used: 0,
                dataset_count: 0,
                compression_ratio: 1.0,
            });

            tier_usage.used += dataset.used_space;
            tier_usage.allocated += dataset.used_space + dataset.available_space;
            tier_usage.dataset_count += 1;
            
            // Use compression ratio from dataset if available
            if let Some(ratio) = dataset.compression_ratio {
                tier_usage.compression_ratio = (tier_usage.compression_ratio + ratio) / 2.0;
            }
        }

        Ok(StorageUsage {
            total_allocated,
            total_used,
            usage_per_tier,
            usage_per_dataset,
        })
    }

    /// Create storage endpoints
    fn create_storage_endpoints(
        &self,
        team_dataset: &str,
        network_config: &StorageNetworkConfig,
    ) -> HashMap<String, StorageEndpoint> {
        let mut endpoints = HashMap::new();

        // Create NFS endpoint if configured
        if let Some(nfs_config) = &network_config.nfs_config {
            let endpoint = StorageEndpoint {
                endpoint_type: "nfs".to_string(),
                url: format!("nfs://{}/{}", "localhost", team_dataset), // TODO: Get actual host
                credentials: None,
                mount_instructions: format!(
                    "mount -t nfs {}:{} /mnt/point",
                    "localhost", nfs_config.export_path
                ),
            };
            endpoints.insert("nfs".to_string(), endpoint);
        }

        // Create SMB endpoint if configured
        if let Some(smb_config) = &network_config.smb_config {
            let endpoint = StorageEndpoint {
                endpoint_type: "smb".to_string(),
                url: format!("smb://{}/{}", "localhost", smb_config.share_name),
                credentials: None,
                mount_instructions: format!(
                    "mount -t cifs //{}/{} /mnt/point",
                    "localhost", smb_config.share_name
                ),
            };
            endpoints.insert("smb".to_string(), endpoint);
        }

        endpoints
    }

    /// Monitor storage usage
    async fn monitor_storage_usage(&self) -> Result<()> {
        // TODO: Implement storage monitoring
        // This would track usage patterns, performance, and optimization opportunities
        Ok(())
    }

    /// Cleanup expired deployments
    async fn cleanup_expired_deployments(&self) -> Result<()> {
        // TODO: Implement cleanup of expired or failed deployments
        Ok(())
    }
}

#[async_trait]
impl ByobStorageProvider for ZfsStorageProvider {
    async fn provision_storage(
        &self,
        request: ByobStorageRequest,
    ) -> Result<ByobStorageResponse> {
        info!("Provisioning storage for deployment: {}", request.deployment_id);

        // Validate request
        self.validate_storage_request(&request)?;

        // Check concurrent operations limit
        {
            let deployments = self.active_deployments.read().await;
            let active_count = deployments.values()
                .filter(|d| matches!(d.status, StorageStatus::Provisioning))
                .count();
            
            if active_count >= self.config.max_concurrent_operations as usize {
                return Err(NestGateError::Storage(
                    "Maximum concurrent storage operations reached".to_string()
                ));
            }
        }

        // Ensure team workspace exists
        let team_dataset = self.ensure_team_workspace(&request.team_id, &request.team_quotas).await?;

        // Create datasets for each service
        let mut datasets = HashMap::new();
        for (service_name, requirements) in &request.storage_requirements {
            let dataset_info = self.create_service_dataset(
                &team_dataset,
                service_name,
                requirements,
            ).await?;

            datasets.insert(service_name.clone(), dataset_info);
        }

        // Create storage mounts
        let mounts = self.create_storage_mounts(
            request.deployment_id,
            &team_dataset,
            &request.storage_requirements,
            &request.network_config,
        ).await?;

        // Calculate storage usage
        let usage = self.calculate_storage_usage(&datasets).await?;

        // Create storage endpoints
        let endpoints = self.create_storage_endpoints(&team_dataset, &request.network_config);

        // Create active deployment tracking
        let active_deployment = ActiveStorageDeployment {
            request: request.clone(),
            status: StorageStatus::Ready,
            datasets: datasets.clone(),
            mounts: mounts.clone(),
            usage: usage.clone(),
            created_at: Instant::now(),
            updated_at: Instant::now(),
        };

        // Store active deployment
        {
            let mut deployments = self.active_deployments.write().await;
            deployments.insert(request.deployment_id, active_deployment);
        }

        // Update team workspace
        {
            let mut workspaces = self.team_workspaces.write().await;
            if let Some(workspace) = workspaces.get_mut(&request.team_id) {
                workspace.active_deployments.push(request.deployment_id);
            }
        }

        let response = ByobStorageResponse {
            deployment_id: request.deployment_id,
            status: StorageStatus::Ready,
            datasets,
            mounts,
            usage,
            endpoints,
            created_at: request.created_at,
            updated_at: Utc::now(),
        };

        info!("Storage provisioned successfully for deployment: {}", request.deployment_id);
        Ok(response)
    }

    async fn get_storage_status(
        &self,
        deployment_id: Uuid,
    ) -> Result<ByobStorageResponse> {
        let deployments = self.active_deployments.read().await;
        
        if let Some(deployment) = deployments.get(&deployment_id) {
            let response = ByobStorageResponse {
                deployment_id,
                status: deployment.status.clone(),
                datasets: deployment.datasets.clone(),
                mounts: deployment.mounts.clone(),
                usage: deployment.usage.clone(),
                endpoints: self.create_storage_endpoints(
                    &format!("{}/teams/{}", self.config.default_pool, deployment.request.team_id),
                    &deployment.request.network_config,
                ),
                created_at: deployment.request.created_at,
                updated_at: Utc::now(),
            };

            Ok(response)
        } else {
            Err(NestGateError::Storage(format!(
                "Storage deployment {} not found",
                deployment_id
            )))
        }
    }

    async fn remove_storage(&self, deployment_id: Uuid) -> Result<()> {
        info!("Removing storage for deployment: {}", deployment_id);

        let mut deployments = self.active_deployments.write().await;
        
        if let Some(mut deployment) = deployments.remove(&deployment_id) {
            deployment.status = StorageStatus::Removing;

            // Remove datasets
            for (service_name, _dataset_info) in &deployment.datasets {
                let dataset_path = format!(
                    "{}/teams/{}/{}",
                    self.config.default_pool,
                    deployment.request.team_id,
                    service_name
                );

                // Delete dataset and all children
                let output = tokio::process::Command::new("zfs")
                    .args(["destroy", "-r", &dataset_path])
                    .output()
                    .await
                    .map_err(|e| NestGateError::Storage(format!("Failed to destroy dataset: {}", e)))?;

                if !output.status.success() {
                    let error_msg = String::from_utf8_lossy(&output.stderr);
                    warn!("Failed to destroy dataset {}: {}", dataset_path, error_msg);
                }
            }

            // Update team workspace
            {
                let mut workspaces = self.team_workspaces.write().await;
                if let Some(workspace) = workspaces.get_mut(&deployment.request.team_id) {
                    workspace.active_deployments.retain(|&id| id != deployment_id);
                }
            }

            info!("Storage removed successfully for deployment: {}", deployment_id);
        } else {
            return Err(NestGateError::Storage(format!(
                "Storage deployment {} not found",
                deployment_id
            )));
        }

        Ok(())
    }

    async fn list_team_storage(&self, team_id: &str) -> Result<Vec<ByobStorageResponse>> {
        let deployments = self.active_deployments.read().await;
        
        let mut team_deployments = Vec::new();
        for (deployment_id, deployment) in deployments.iter() {
            if deployment.request.team_id == team_id {
                let response = ByobStorageResponse {
                    deployment_id: *deployment_id,
                    status: deployment.status.clone(),
                    datasets: deployment.datasets.clone(),
                    mounts: deployment.mounts.clone(),
                    usage: deployment.usage.clone(),
                    endpoints: self.create_storage_endpoints(
                        &format!("{}/teams/{}", self.config.default_pool, team_id),
                        &deployment.request.network_config,
                    ),
                    created_at: deployment.request.created_at,
                    updated_at: Utc::now(),
                };
                team_deployments.push(response);
            }
        }

        Ok(team_deployments)
    }

    async fn get_storage_usage(&self, deployment_id: Uuid) -> Result<StorageUsage> {
        let deployments = self.active_deployments.read().await;
        
        if let Some(deployment) = deployments.get(&deployment_id) {
            // Recalculate current usage
            self.calculate_storage_usage(&deployment.datasets).await
        } else {
            Err(NestGateError::Storage(format!(
                "Storage deployment {} not found",
                deployment_id
            )))
        }
    }

    async fn create_snapshot(
        &self,
        deployment_id: Uuid,
        snapshot_name: String,
    ) -> Result<String> {
        info!("Creating snapshot {} for deployment: {}", snapshot_name, deployment_id);

        let deployments = self.active_deployments.read().await;
        
        if let Some(deployment) = deployments.get(&deployment_id) {
            let team_dataset = format!(
                "{}/teams/{}",
                self.config.default_pool,
                deployment.request.team_id
            );

            // Create recursive snapshot of entire team deployment
            let snapshot_id = self.zfs_manager.snapshot_manager
                .create_snapshot(&team_dataset, &snapshot_name, true)
                .await
                .map_err(|e| NestGateError::Storage(format!("Failed to create snapshot: {}", e)))?;

            info!("Snapshot created successfully: {}", snapshot_id);
            Ok(snapshot_id)
        } else {
            Err(NestGateError::Storage(format!(
                "Storage deployment {} not found",
                deployment_id
            )))
        }
    }

    async fn restore_snapshot(
        &self,
        deployment_id: Uuid,
        snapshot_name: String,
    ) -> Result<()> {
        info!("Restoring snapshot {} for deployment: {}", snapshot_name, deployment_id);

        let deployments = self.active_deployments.read().await;
        
        if let Some(deployment) = deployments.get(&deployment_id) {
            let team_dataset = format!(
                "{}/teams/{}",
                self.config.default_pool,
                deployment.request.team_id
            );

            // Rollback to snapshot
            let output = tokio::process::Command::new("zfs")
                .args(["rollback", &format!("{}@{}", team_dataset, snapshot_name)])
                .output()
                .await
                .map_err(|e| NestGateError::Storage(format!("Failed to rollback snapshot: {}", e)))?;

            if !output.status.success() {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                return Err(NestGateError::Storage(format!(
                    "Failed to rollback snapshot: {}", error_msg
                )));
            }

            info!("Snapshot restored successfully: {}", snapshot_name);
            Ok(())
        } else {
            Err(NestGateError::Storage(format!(
                "Storage deployment {} not found",
                deployment_id
            )))
        }
    }
}

/// Create a ZFS storage provider
pub fn create_zfs_storage_provider(
    zfs_manager: Arc<ZfsManager>,
    config: Option<ByobStorageConfig>,
) -> Arc<dyn ByobStorageProvider> {
    let config = config.unwrap_or_default();
    Arc::new(ZfsStorageProvider::new(zfs_manager, config))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_validate_storage_request() {
        // TODO: Implement tests for storage request validation
    }

    #[tokio::test]
    async fn test_team_workspace_creation() {
        // TODO: Implement tests for team workspace creation
    }

    #[tokio::test]
    async fn test_storage_provisioning() {
        // TODO: Implement tests for storage provisioning
    }
} 