//! **BIOMEOS TYPES AND DATA STRUCTURES**
//!
//! Core BiomeOS types, structs, and enums for manifest definitions and configuration.
//! Extracted from biomeos.rs for file size compliance.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::canonical_types::storage::StorageTier;

/// BiomeOS manifest structure for universal capability routing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeManifest {
    /// API version for biomeOS compatibility
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Resource kind
    pub kind: String,
    /// Biome metadata
    pub metadata: BiomeMetadata,
    /// Capability-based primal configurations (routed through universal adapter)
    pub capabilities: HashMap<String, CapabilityConfig>,
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
    /// Capability-based templates
    pub templates: Option<BiomeTemplates>,
    /// Agent definitions (routed via capability discovery)
    pub agents: Option<Vec<AgentSpec>>,
    /// Coordination patterns (universal cross-capability)
    pub coordination: Option<CoordinationConfig>,
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
    /// Biome labels for organization
    pub labels: Option<HashMap<String, String>>,
    /// Biome annotations for metadata
    pub annotations: Option<HashMap<String, String>>,
}

/// Capability configuration for universal adapter routing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityConfig {
    /// Capability type (ai-runtime, agent-processing, security-provider, etc.)
    pub capability_type: String,
    /// Configuration parameters
    pub config: HashMap<String, serde_json::Value>,
    /// Resource requirements
    pub resources: Option<ResourceRequirements>,
    /// Discovery preferences
    pub discovery: Option<DiscoveryPreferences>,
}

/// Resource requirements for capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    /// CPU requirements
    pub cpu: Option<String>,
    /// Memory requirements
    pub memory: Option<String>,
    /// Storage requirements
    pub storage: Option<String>,
    /// Custom resource requirements
    pub custom: Option<HashMap<String, String>>,
}

/// Discovery preferences for capability routing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryPreferences {
    /// Preferred provider types
    pub preferred_providers: Option<Vec<String>>,
    /// Fallback options
    pub fallback_enabled: bool,
    /// Timeout for discovery
    pub discovery_timeout_seconds: Option<u64>,
}

/// Service category enumeration (extensible)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceCategory {
    Storage,
    Orchestration,
    Security,
    ArtificialIntelligence,
    Compute,
    Custom(String),
}

/// Agent specification for AI/compute integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSpec {
    /// Agent name
    pub name: String,
    /// Agent type (ai-agent, compute-agent, etc.)
    pub agent_type: String,
    /// Agent configuration
    pub config: HashMap<String, serde_json::Value>,
    /// Resource requirements
    pub resources: Option<ResourceRequirements>,
}

/// Service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    /// Service name
    pub name: String,
    /// Service category
    pub category: ServiceCategory,
    /// Service image
    pub image: Option<String>,
    /// Environment variables
    pub environment: Option<HashMap<String, String>>,
    /// Port specifications
    pub ports: Option<Vec<PortSpec>>,
    /// Volume mounts
    pub volumes: Option<Vec<String>>,
    /// Resource requirements
    pub resources: Option<ResourceRequirements>,
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

/// Biome resource requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeResources {
    /// CPU requirements
    pub cpu: Option<String>,
    /// Memory requirements
    pub memory: Option<String>,
    /// Storage requirements
    pub storage: Option<String>,
    /// GPU requirements
    pub gpu: Option<String>,
    /// Custom resource requirements
    pub custom: Option<HashMap<String, String>>,
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

/// Encryption policies for security provider integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionPolicies {
    /// Encryption at rest required
    pub at_rest: bool,
    /// Encryption in transit required
    pub in_transit: bool,
    /// Key rotation interval in days
    pub key_rotation_days: Option<u32>,
    /// Encryption provider (security-service, software)
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
    /// Network mode (host, bridge, custom)
    pub mode: String,
    /// Custom network settings
    pub custom_networks: Option<Vec<NetworkSpec>>,
    /// DNS configuration
    pub dns: Option<DnsConfig>,
    /// Load balancing configuration
    pub load_balancing: Option<LoadBalancingConfig>,
}

/// Network specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSpec {
    /// Network name
    pub name: String,
    /// Network driver
    pub driver: String,
    /// Network options
    pub options: Option<HashMap<String, String>>,
}

/// DNS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsConfig {
    /// DNS servers
    pub servers: Vec<String>,
    /// Search domains
    pub search_domains: Option<Vec<String>>,
}

/// Load balancing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingConfig {
    /// Load balancing strategy
    pub strategy: String,
    /// Health check configuration
    pub health_check: Option<HealthCheckConfig>,
}

/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    /// Health check path
    pub path: String,
    /// Health check interval
    pub interval_seconds: u32,
    /// Health check timeout
    pub timeout_seconds: u32,
    /// Healthy threshold
    pub healthy_threshold: u32,
    /// Unhealthy threshold
    pub unhealthy_threshold: u32,
}

/// Biome storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeStorage {
    /// Storage driver
    pub driver: String,
    /// Volume specifications
    pub volumes: Vec<VolumeSpec>,
    /// Storage policies
    pub policies: Option<StoragePolicies>,
}

/// Volume specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeSpec {
    /// Volume name
    pub name: String,
    /// Volume size
    pub size: String,
    /// Storage tier
    pub tier: String,
    /// Mount path
    pub mount_path: String,
    /// Volume options
    pub options: Option<HashMap<String, String>>,
    /// Replication factor
    pub replication_factor: Option<u32>,
    /// Backup policy
    pub backup_policy: Option<String>,
    /// Volume status
    pub status: Option<VolumeStatus>,
    /// Replication status
    pub replication_status: String,
}

/// Storage policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoragePolicies {
    /// Backup policies
    pub backup: Option<BackupPolicy>,
    /// Retention policies
    pub retention: Option<RetentionPolicy>,
    /// Replication policies
    pub replication: Option<ReplicationPolicy>,
}

/// Backup policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupPolicy {
    /// Backup schedule
    pub schedule: String,
    /// Backup retention
    pub retention_days: u32,
    /// Backup destinations
    pub destinations: Vec<String>,
}

/// Retention policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    /// Retention period in days
    pub days: u32,
    /// Archive after retention
    pub archive_after_retention: bool,
}

/// Replication policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationPolicy {
    /// Replication factor
    pub factor: u32,
    /// Replication strategy
    pub strategy: String,
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

/// Biome specialization settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeSpecialization {
    /// Specialization type
    pub specialization_type: String,
    /// Specialization parameters
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Biome templates for capability-based configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeTemplates {
    /// AI runtime capability templates
    pub ai_runtime: Option<Vec<TemplateSpec>>,
    /// Agent processing capability templates
    pub agent_processing: Option<Vec<TemplateSpec>>,
    /// Security provider capability templates
    pub security_provider: Option<Vec<TemplateSpec>>,
    /// Orchestration capability templates
    pub orchestration: Option<Vec<TemplateSpec>>,
    /// Custom capability templates
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

/// Coordination configuration for cross-capability patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationConfig {
    /// Coordination patterns
    pub patterns: Vec<CoordinationPattern>,
    /// Coordination policies
    pub policies: Option<CoordinationPolicies>,
}

/// Coordination pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationPattern {
    /// Pattern name
    pub name: String,
    /// Pattern type
    pub pattern_type: String,
    /// Participating capabilities
    pub capabilities: Vec<String>,
    /// Configuration
    pub config: HashMap<String, serde_json::Value>,
}

/// Coordination policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationPolicies {
    /// Timeout policies
    pub timeout: Option<TimeoutPolicy>,
    /// Retry policies
    pub retry: Option<RetryPolicy>,
    /// Failure policies
    pub failure: Option<FailurePolicy>,
}

/// Timeout policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeoutPolicy {
    /// Default timeout in seconds
    pub default_seconds: u32,
    /// Per-capability timeouts
    pub per_capability: Option<HashMap<String, u32>>,
}

/// Retry policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    /// Maximum retry attempts
    pub max_attempts: u32,
    /// Retry delay in seconds
    pub delay_seconds: u32,
    /// Exponential backoff
    pub exponential_backoff: bool,
}

/// Failure policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailurePolicy {
    /// Failure handling strategy
    pub strategy: String,
    /// Fallback configurations
    pub fallbacks: Option<Vec<String>>,
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
    /// Group memberships
    pub groups: Vec<String>,
    /// Permissions
    pub permissions: Vec<String>,
    /// Security token
    pub token: Option<String>,
}

/// Resource constraints for operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConstraints {
    /// CPU limit
    pub cpu_limit: Option<String>,
    /// Memory limit
    pub memory_limit: Option<String>,
    /// Storage limit
    pub storage_limit: Option<String>,
    /// Network bandwidth limit
    pub network_limit: Option<String>,
} 