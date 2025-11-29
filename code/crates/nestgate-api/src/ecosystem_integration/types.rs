//! **ECOSYSTEM INTEGRATION TYPES**
//!
//! Core types and data structures for universal ecosystem integration.
//! Extracted from the monolithic ecosystem_integration.rs file.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Universal Service Registration - ALL PARTICIPANTS MUST IMPLEMENT
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Universalserviceregistration
pub struct UniversalServiceRegistration {
    /// Unique service identifier (generated)
    pub service_id: Uuid,
    /// Service _metadata
    pub _metadata: ServiceMetadata,

    /// Capabilities this service provides
    pub capabilities: Vec<ServiceCapability>,

    /// Resource requirements and limits
    pub resources: ResourceSpec,

    /// API endpoints (dynamically discovered)
    pub endpoints: Vec<ServiceEndpoint>,

    /// Integration preferences
    pub integration: IntegrationPreferences,

    /// Extension points for custom data
    pub extensions: HashMap<String, serde_json::Value>,

    /// Registration timestamp
    pub registration_timestamp: DateTime<Utc>,

    /// Service version
    pub service_version: String,

    /// Instance identifier for multi-instance support
    pub instance_id: String,

    /// Priority level for load balancing
    pub priority: u8,
}

/// Service _metadata with open categorization
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Servicemetadata
pub struct ServiceMetadata {
    /// Human-readable service name
    pub name: String,
    /// Service category (open enumeration)
    pub category: ServiceCategory,

    /// Version information
    pub version: String,

    /// Description and documentation
    pub description: String,

    /// Maintainer information
    pub maintainer: ContactInfo,

    /// Supported protocols
    pub protocols: Vec<String>,
}

/// Open, extensible service categories
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Servicecategory
pub enum ServiceCategory {
    /// Computational services
    Compute {
        /// Specialized compute capabilities
        specialties: Vec<String>,
    },
    /// Storage and data services
    Storage {
        /// Storage service types
        types: Vec<String>,
    },
    /// Security and identity services
    Security {
        /// Security domains and specializations
        domains: Vec<String>,
    },
    /// Network and communication services
    Network {
        /// Network protocol layers
        layers: Vec<String>,
    },
    /// Orchestration and coordination services
    Orchestration {
        /// Orchestration scopes and areas
        scopes: Vec<String>,
    },
    /// Artificial intelligence services
    Intelligence {
        /// AI modalities and capabilities
        modalities: Vec<String>,
    },
    /// Community-defined custom categories
    Custom {
        /// Custom category name
        category: String,
        /// Custom subcategories
        subcategories: Vec<String>,
    },
}
/// Universal capability system (extensible)
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Servicecapability
pub enum ServiceCapability {
    /// Data processing capabilities
    DataProcessing {
        formats: Vec<String>,
        operations: Vec<String>,
    },
    /// Storage capabilities
    Storage {
        types: Vec<String>,
        consistency: String,
        durability: String,
    },
    /// Compute capabilities
    Compute {
        architectures: Vec<String>,
        specializations: Vec<String>,
    },
    /// Security capabilities
    Security {
        protocols: Vec<String>,
        compliance: Vec<String>,
    },
    /// Custom capabilities
    Custom {
        name: String,
        properties: HashMap<String, serde_json::Value>,
    },
}
/// Contact information for service maintainers
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Contactinfo
pub struct ContactInfo {
    /// Contact name or organization
    pub name: String,
    /// Contact email
    pub email: Option<String>,
    /// Website or documentation URL
    pub website: Option<String>,
    /// Support contact information
    pub support: Option<String>,
}
/// Resource specification for services
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Resourcespec
pub struct ResourceSpec {
    /// CPU requirements
    pub cpu: Option<CpuSpec>,
    /// Memory requirements
    pub memory: Option<MemorySpec>,
    /// Storage requirements
    pub storage: Option<StorageSpec>,
    /// Network requirements
    pub network: Option<NetworkSpec>,
    /// Custom resource requirements
    pub custom: HashMap<String, serde_json::Value>,
}
/// CPU specification
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Cpuspec
pub struct CpuSpec {
    /// Minimum CPU cores required
    pub min_cores: Option<f64>,
    /// Maximum CPU cores that can be utilized
    pub max_cores: Option<f64>,
    /// Architecture requirements
    pub architecture: Option<String>,
    /// Special CPU features required
    pub features: Vec<String>,
}
/// Memory specification
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Memoryspec
pub struct MemorySpec {
    /// Minimum memory in bytes
    pub min_bytes: Option<u64>,
    /// Maximum memory in bytes
    pub max_bytes: Option<u64>,
    /// Memory type requirements
    pub memory_type: Option<String>,
}
/// Storage specification
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Storagespec
pub struct StorageSpec {
    /// Minimum storage in bytes
    pub min_bytes: Option<u64>,
    /// Storage type requirements
    pub storage_type: Option<String>,
    /// IOPS requirements
    pub iops: Option<u64>,
    /// Durability requirements
    pub durability: Option<String>,
}
/// Network specification
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Networkspec
pub struct NetworkSpec {
    /// Bandwidth requirements in bytes per second
    pub bandwidth: Option<u64>,
    /// Latency requirements in milliseconds
    pub latency: Option<u64>,
    /// Protocol requirements
    pub protocols: Vec<String>,
}
/// Service endpoint specification
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Serviceendpoint
pub struct ServiceEndpoint {
    /// Endpoint name/identifier
    pub name: String,
    /// Endpoint URL
    pub url: String,
    /// Protocol used
    pub protocol: String,
    /// Health check URL
    pub health_check: Option<String>,
    /// Endpoint _metadata
    pub _metadata: HashMap<String, String>,
}
/// Integration preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Integrationpreferences
pub struct IntegrationPreferences {
    /// Preferred communication protocols
    pub protocols: Vec<String>,
    /// Data format preferences
    pub data_formats: Vec<String>,
    /// Authentication methods supported
    pub auth_methods: Vec<String>,
    /// Rate limiting preferences
    pub rate_limiting: Option<RateLimitSpec>,
    /// Circuit breaker configuration
    pub circuit_breaker: Option<CircuitBreakerSpec>,
}
/// Rate limiting specification
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Ratelimitspec
pub struct RateLimitSpec {
    /// Requests per second
    pub rps: Option<u64>,
    /// Burst capacity
    pub burst: Option<u64>,
    /// Window duration in seconds
    pub window_seconds: Option<u64>,
}
/// Circuit breaker specification
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Circuitbreakerspec
pub struct CircuitBreakerSpec {
    /// Failure threshold
    pub failure_threshold: Option<u32>,
    /// Timeout in seconds
    pub timeout_seconds: Option<u64>,
    /// Recovery time in seconds
    pub recovery_seconds: Option<u64>,
} 