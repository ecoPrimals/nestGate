//! biomeOS Integration Types and Manifest Processing
//!
//! This module provides the core types and functionality for integrating
//! NestGate with biomeOS, including manifest parsing and automated provisioning.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{Result, StorageTier};

/// biomeOS manifest structure for NestGate integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeManifest {
    /// Biome metadata
    pub metadata: BiomeMetadata,
    /// Primal configurations
    pub primals: HashMap<String, PrimalConfig>,
    /// Service definitions
    pub services: HashMap<String, ServiceConfig>,
    /// Resource requirements
    pub resources: BiomeResources,
    /// Security configuration
    pub security: BiomeSecurity,
    /// Networking configuration
    pub networking: BiomeNetworking,
    /// Storage configuration (NestGate-specific)
    pub storage: BiomeStorage,
    /// Biome specialization
    pub specialization: Option<BiomeSpecialization>,
    /// Storage templates
    pub templates: Option<BiomeTemplates>,
}

/// Biome metadata information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeMetadata {
    /// Biome name
    pub name: String,
    /// Biome version
    pub version: String,
    /// Biome description
    pub description: Option<String>,
    /// Author information
    pub author: Option<String>,
    /// Creation timestamp
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Primal configuration within biome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalConfig {
    /// Primal type (nestgate, songbird, beardog, squirrel, toadstool)
    pub primal_type: String,
    /// Version requirement
    pub version: String,
    /// Configuration parameters
    pub config: HashMap<String, serde_json::Value>,
    /// Resource requirements
    pub resources: Option<PrimalResources>,
}

/// Service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    /// Service type
    pub service_type: String,
    /// Service version
    pub version: String,
    /// Service endpoints
    pub endpoints: Vec<String>,
    /// Service capabilities
    pub capabilities: Vec<String>,
    /// Service metadata
    pub metadata: HashMap<String, String>,
}

/// Biome resource requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeResources {
    /// Storage requirements
    pub storage: Option<StorageResources>,
    /// Compute requirements
    pub compute: Option<ComputeResources>,
    /// Network requirements
    pub network: Option<NetworkResources>,
}

/// Primal-specific resource requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalResources {
    /// CPU cores
    pub cpu_cores: Option<f64>,
    /// Memory in MB
    pub memory_mb: Option<u64>,
    /// Storage in GB
    pub storage_gb: Option<u64>,
}

/// Storage resource requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageResources {
    /// Total storage required in GB
    pub total_gb: u64,
    /// Storage volumes
    pub volumes: Vec<VolumeSpec>,
}

/// Volume specification for provisioning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeSpec {
    /// Volume name
    pub name: String,
    /// Volume size (e.g., "100Gi", "1Ti")
    pub size: String,
    /// Storage tier
    pub tier: String,
    /// Provisioner (should be "nestgate" for our volumes)
    pub provisioner: String,
    /// Mount path
    pub mount_path: Option<String>,
    /// Access mode (ReadWriteOnce, ReadOnlyMany, ReadWriteMany)
    pub access_mode: Option<String>,
    /// Volume options
    pub options: Option<HashMap<String, String>>,
}

/// Compute resource requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeResources {
    /// CPU cores
    pub cpu_cores: f64,
    /// Memory in MB
    pub memory_mb: u64,
    /// GPU requirements
    pub gpu: Option<GpuResources>,
}

/// GPU resource requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuResources {
    /// Number of GPUs
    pub count: u32,
    /// GPU memory in MB
    pub memory_mb: Option<u64>,
    /// GPU type preference
    pub gpu_type: Option<String>,
}

/// Network resource requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkResources {
    /// Required bandwidth in Mbps
    pub bandwidth_mbps: Option<u64>,
    /// Port requirements
    pub ports: Vec<PortSpec>,
}

/// Port specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortSpec {
    /// Port number
    pub port: u16,
    /// Protocol (TCP, UDP)
    pub protocol: String,
    /// Expose externally
    pub expose_externally: bool,
}

/// Biome security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeSecurity {
    /// Security level
    pub security_level: SecurityLevel,
    /// Encryption policies
    pub encryption_policies: Option<EncryptionPolicies>,
    /// Access controls
    pub access_controls: Option<AccessControls>,
    /// Audit requirements
    pub audit_requirements: Option<AuditRequirements>,
}

/// Security levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityLevel {
    /// Basic security
    Basic,
    /// Standard security
    Standard,
    /// High security
    High,
    /// Enterprise security
    Enterprise,
}

/// Encryption policies for BearDog integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionPolicies {
    /// Encryption at rest required
    pub at_rest: bool,
    /// Encryption in transit required
    pub in_transit: bool,
    /// Key rotation interval in days
    pub key_rotation_days: Option<u32>,
    /// Encryption provider (beardog, software)
    pub provider: Option<String>,
}

/// Access control policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControls {
    /// Default access mode
    pub default_access: String,
    /// User permissions
    pub user_permissions: HashMap<String, Vec<String>>,
    /// Group permissions
    pub group_permissions: HashMap<String, Vec<String>>,
}

/// Audit requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditRequirements {
    /// Audit level
    pub level: String,
    /// Retention period in days
    pub retention_days: u32,
    /// Audit destinations
    pub destinations: Vec<String>,
}

/// Biome networking configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeNetworking {
    /// Service mesh configuration
    pub service_mesh: Option<ServiceMeshConfig>,
    /// Load balancing
    pub load_balancing: Option<LoadBalancingConfig>,
    /// Network policies
    pub network_policies: Option<Vec<NetworkPolicy>>,
}

/// Service mesh configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMeshConfig {
    /// Enable service mesh
    pub enabled: bool,
    /// Mesh provider
    pub provider: String,
    /// Mesh configuration
    pub config: HashMap<String, serde_json::Value>,
}

/// Load balancing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingConfig {
    /// Strategy (round_robin, least_connections, weighted)
    pub strategy: String,
    /// Health check configuration
    pub health_check: Option<HealthCheckConfig>,
}

/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    /// Check interval in seconds
    pub interval_seconds: u32,
    /// Timeout in seconds
    pub timeout_seconds: u32,
    /// Health check path
    pub path: String,
}

/// Network policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPolicy {
    /// Policy name
    pub name: String,
    /// Policy rules
    pub rules: Vec<NetworkRule>,
}

/// Network rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkRule {
    /// Source specification
    pub from: Vec<String>,
    /// Destination specification
    pub to: Vec<String>,
    /// Allowed ports
    pub ports: Vec<u16>,
}

/// Biome storage configuration (NestGate-specific)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeStorage {
    /// Default storage class
    pub default_class: Option<String>,
    /// Volume definitions
    pub volumes: Vec<VolumeSpec>,
    /// Storage policies
    pub policies: Option<StoragePolicies>,
    /// Backup configuration
    pub backup: Option<BackupConfig>,
}

/// Storage policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoragePolicies {
    /// Default tier
    pub default_tier: String,
    /// Tier migration policies
    pub tier_migration: Option<TierMigrationConfig>,
    /// Retention policies
    pub retention: Option<RetentionConfig>,
}

/// Tier migration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierMigrationConfig {
    /// Enable automatic migration
    pub auto_migration: bool,
    /// Migration rules
    pub rules: Vec<MigrationRule>,
}

/// Migration rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationRule {
    /// Source tier
    pub from_tier: String,
    /// Target tier
    pub to_tier: String,
    /// Trigger condition
    pub condition: String,
    /// Age threshold in days
    pub age_days: Option<u32>,
}

/// Retention configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionConfig {
    /// Default retention in days
    pub default_days: u32,
    /// Tier-specific retention
    pub tier_retention: HashMap<String, u32>,
}

/// Backup configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    /// Enable automatic backups
    pub enabled: bool,
    /// Backup schedule (cron format)
    pub schedule: Option<String>,
    /// Backup retention in days
    pub retention_days: u32,
    /// Backup destinations
    pub destinations: Vec<String>,
}

/// Biome specialization settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeSpecialization {
    /// Specialization type
    pub specialization_type: String,
    /// Specialization parameters
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Biome templates for common configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeTemplates {
    /// Toadstool runtime templates
    pub toadstool_runtime: Option<Vec<TemplateSpec>>,
    /// Squirrel agent templates
    pub squirrel_agents: Option<Vec<TemplateSpec>>,
    /// Custom templates
    pub custom: Option<HashMap<String, Vec<TemplateSpec>>>,
}

/// Template specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateSpec {
    /// Template name
    pub name: String,
    /// Resource allocation
    pub resources: String,
    /// Configuration
    pub config: HashMap<String, serde_json::Value>,
}

/// Biome context for provisioning operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeContext {
    /// Biome ID
    pub biome_id: String,
    /// Node ID within biome
    pub node_id: String,
    /// Environment (development, staging, production)
    pub environment: String,
    /// Security context
    pub security_context: SecurityContext,
    /// Resource constraints
    pub resource_constraints: ResourceConstraints,
    /// Integration endpoints
    pub integration_endpoints: HashMap<String, String>,
}

/// Security context for operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityContext {
    /// User ID
    pub user_id: String,
    /// Authentication token
    pub auth_token: Option<String>,
    /// Authorized operations
    pub permissions: Vec<String>,
    /// Security level
    pub security_level: SecurityLevel,
}

/// Resource constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConstraints {
    /// Maximum CPU cores
    pub max_cpu_cores: Option<f64>,
    /// Maximum memory in MB
    pub max_memory_mb: Option<u64>,
    /// Maximum storage in GB
    pub max_storage_gb: Option<u64>,
    /// Resource quotas
    pub quotas: HashMap<String, u64>,
}

/// Volume information response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeInfo {
    /// Volume ID
    pub id: String,
    /// Volume name
    pub name: String,
    /// Volume size in bytes
    pub size_bytes: u64,
    /// Used space in bytes
    pub used_bytes: u64,
    /// Available space in bytes
    pub available_bytes: u64,
    /// Storage tier
    pub tier: StorageTier,
    /// Mount point
    pub mount_point: String,
    /// Volume status
    pub status: VolumeStatus,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Volume metadata
    pub metadata: HashMap<String, String>,
}

/// Volume status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VolumeStatus {
    /// Volume is being created
    Creating,
    /// Volume is available
    Available,
    /// Volume is being mounted
    Mounting,
    /// Volume is mounted
    Mounted,
    /// Volume is being unmounted
    Unmounting,
    /// Volume is in error state
    Error,
    /// Volume is being deleted
    Deleting,
}

impl BiomeManifest {
    /// Parse biome.yaml from string
    pub fn from_yaml(yaml_content: &str) -> Result<Self> {
        serde_yaml::from_str(yaml_content).map_err(|e| {
            crate::NestGateError::Internal(format!("Failed to parse biome.yaml: {}", e))
        })
    }

    /// Parse biome.yaml from file
    pub async fn from_file(file_path: &str) -> Result<Self> {
        let content = tokio::fs::read_to_string(file_path).await.map_err(|e| {
            crate::NestGateError::Internal(format!("Failed to read biome.yaml: {}", e))
        })?;

        Self::from_yaml(&content)
    }

    /// Get storage volumes for NestGate provisioning
    pub fn get_nestgate_volumes(&self) -> Vec<&VolumeSpec> {
        self.storage
            .volumes
            .iter()
            .filter(|v| v.provisioner == "nestgate")
            .collect()
    }

    /// Get Primal-specific storage templates
    pub fn get_primal_templates(&self, primal_type: &str) -> Vec<TemplateSpec> {
        if let Some(templates) = &self.templates {
            match primal_type {
                "toadstool" => templates
                    .toadstool_runtime
                    .as_ref()
                    .unwrap_or(&vec![])
                    .clone(),
                "squirrel" => templates
                    .squirrel_agents
                    .as_ref()
                    .unwrap_or(&vec![])
                    .clone(),
                custom => templates
                    .custom
                    .as_ref()
                    .and_then(|c| c.get(custom))
                    .unwrap_or(&vec![])
                    .clone(),
            }
        } else {
            vec![]
        }
    }
}

impl VolumeSpec {
    /// Parse size string to bytes (e.g., "100Gi" -> bytes)
    pub fn size_bytes(&self) -> Result<u64> {
        parse_size(&self.size)
    }

    /// Convert to storage tier enum
    pub fn storage_tier(&self) -> Result<StorageTier> {
        match self.tier.to_lowercase().as_str() {
            "hot" => Ok(StorageTier::Hot),
            "warm" => Ok(StorageTier::Warm),
            "cold" => Ok(StorageTier::Cold),
            "cache" => Ok(StorageTier::Cache),
            _ => Err(crate::NestGateError::Internal(format!(
                "Unknown storage tier: {}",
                self.tier
            ))),
        }
    }
}

/// Parse size string to bytes
fn parse_size(size_str: &str) -> Result<u64> {
    let size_str = size_str.trim();

    if size_str.ends_with("Gi") || size_str.ends_with("gi") {
        let num_str = &size_str[..size_str.len() - 2];
        let num: f64 = num_str.parse().map_err(|_| {
            crate::NestGateError::Internal(format!("Invalid size format: {}", size_str))
        })?;
        Ok((num * 1024.0 * 1024.0 * 1024.0) as u64)
    } else if size_str.ends_with("Ti") || size_str.ends_with("ti") {
        let num_str = &size_str[..size_str.len() - 2];
        let num: f64 = num_str.parse().map_err(|_| {
            crate::NestGateError::Internal(format!("Invalid size format: {}", size_str))
        })?;
        Ok((num * 1024.0 * 1024.0 * 1024.0 * 1024.0) as u64)
    } else if size_str.ends_with("Mi") || size_str.ends_with("mi") {
        let num_str = &size_str[..size_str.len() - 2];
        let num: f64 = num_str.parse().map_err(|_| {
            crate::NestGateError::Internal(format!("Invalid size format: {}", size_str))
        })?;
        Ok((num * 1024.0 * 1024.0) as u64)
    } else {
        // Assume bytes if no suffix
        size_str.parse().map_err(|_| {
            crate::NestGateError::Internal(format!("Invalid size format: {}", size_str))
        })
    }
}
