/// Universal Service Metadata Module
/// **CONSOLIDATION COMPLETE**: Unifies all fragmented service information structures
///
/// This module replaces and consolidates:
/// - UnifiedServiceInfo (interface/service_types.rs)
/// - PrimalMetadata (ecoprimal_sdk/types.rs)
/// - ServiceMetadata (service_discovery/types.rs)
/// - ServiceInfo (zfs/manager/types.rs)
/// - ServiceInfo (mcp/protocol.rs)
///
/// **PROBLEM SOLVED**: Single source of truth for all service metadata
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::unified_enums::UnifiedServiceType;

/// **THE** Universal Service Metadata Structure
/// Comprehensive service information that encompasses all use cases across the ecosystem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalServiceMetadata {
    // === CORE IDENTIFICATION ===
    /// Unique service identifier (generated UUID)
    pub service_id: Uuid,
    /// Human-readable service name
    pub name: String,
    /// Semantic version string
    pub version: String,
    /// Service description
    pub description: String,

    // === SERVICE CLASSIFICATION ===
    /// Service type classification (unified enum)
    pub service_type: UnifiedServiceType,
    /// Service capabilities provided
    pub capabilities: Vec<ServiceCapability>,
    /// Additional specialized capabilities (domain-specific)
    pub specialized_capabilities: HashMap<String, Vec<String>>,

    // === NETWORK & ENDPOINTS ===
    /// Service endpoints for communication
    pub endpoints: Vec<ServiceEndpoint>,
    /// Primary endpoint URL
    pub primary_endpoint: Option<String>,
    /// Health check endpoint
    pub health_endpoint: Option<String>,
    /// Metrics endpoint
    pub metrics_endpoint: Option<String>,

    // === METADATA & CLASSIFICATION ===
    /// Tags for categorization and discovery
    pub tags: Vec<String>,
    /// Extended metadata (domain-specific data)
    pub metadata: HashMap<String, serde_json::Value>,
    /// String-based metadata for simple key-value pairs
    pub simple_metadata: HashMap<String, String>,

    // === ECOSYSTEM INTEGRATION ===
    /// Maintainer/author information
    pub maintainer: Option<ContactInfo>,
    /// Repository URL
    pub repository: Option<String>,
    /// Documentation URL
    pub documentation: Option<String>,
    /// License information
    pub license: Option<String>,
    /// Supported platforms
    pub supported_platforms: Vec<String>,
    /// Minimum ecosystem version required
    pub min_ecosystem_version: Option<String>,

    // === TEMPORAL TRACKING ===
    /// Service creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
    /// Last health check timestamp
    pub last_health_check: Option<DateTime<Utc>>,

    // === OPERATIONAL STATUS ===
    /// Current service status
    pub status: ServiceStatus,
    /// Current health state
    pub health_state: HealthState,
    /// Resource requirements and limits
    pub resources: Option<ResourceRequirements>,

    // === EXTENSION POINTS ===
    /// Domain-specific extensions
    pub extensions: HashMap<String, serde_json::Value>,
}

/// Service capability enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ServiceCapability {
    // === CORE CAPABILITIES ===
    HttpServer,
    WebSocket,
    TcpServer,
    UdpServer,

    // === DATA & STORAGE ===
    Database,
    FileSystem,
    ObjectStorage,
    Cache,
    MessageQueue,

    // === NETWORK & COMMUNICATION ===
    Network,
    LoadBalancing,
    ServiceDiscovery,
    ApiGateway,
    TlsTermination,

    // === SECURITY ===
    Authentication,
    Authorization,
    Encryption,
    CertificateManagement,

    // === OBSERVABILITY ===
    Monitoring,
    Logging,
    Metrics,
    Tracing,
    AlertManager,

    // === ORCHESTRATION ===
    Orchestration,
    ContainerManagement,
    WorkflowEngine,
    Automation,

    // === AI & INTELLIGENCE ===
    MachineLearning,
    ModelServing,
    DataAnalytics,
    AiPipeline,

    // === EXTENSIBLE ===
    Custom(String),
}

/// Service endpoint information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    /// Endpoint URL
    pub url: String,
    /// Protocol used
    pub protocol: CommunicationProtocol,
    /// Endpoint type/purpose
    pub endpoint_type: EndpointType,
    /// Health check URL for this endpoint
    pub health_check: Option<String>,
    /// Whether this endpoint is primary
    pub is_primary: bool,
}

/// Communication protocol enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CommunicationProtocol {
    Http,
    Https,
    WebSocket,
    WebSocketSecure,
    Tcp,
    Udp,
    Grpc,
    Custom(String),
}

/// Endpoint type classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EndpointType {
    Primary,
    Health,
    Metrics,
    Admin,
    Api,
    WebSocket,
    Custom(String),
}

/// Contact information for maintainers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInfo {
    /// Name or organization
    pub name: String,
    /// Email address
    pub email: Option<String>,
    /// Website or profile URL
    pub website: Option<String>,
}

/// Service operational status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ServiceStatus {
    /// Service is starting up
    Starting,
    /// Service is running normally
    Running,
    /// Service is stopping
    Stopping,
    /// Service is stopped
    Stopped,
    /// Service has failed
    Failed,
    /// Service is in maintenance mode
    Maintenance,
    /// Status is unknown
    Unknown,
}

/// Health state enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthState {
    /// Service is healthy
    Healthy,
    /// Service has warnings but is functional
    Warning,
    /// Service is unhealthy but running
    Unhealthy,
    /// Service is completely unavailable
    Critical,
    /// Health state is unknown
    Unknown,
}

/// Resource requirements for service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    /// CPU cores required
    pub cpu_cores: Option<f64>,
    /// Memory in MB required
    pub memory_mb: Option<u64>,
    /// Disk space in GB required
    pub disk_gb: Option<u64>,
    /// Network bandwidth in Mbps
    pub network_mbps: Option<u64>,
}

// === DEFAULT IMPLEMENTATIONS ===

impl Default for UniversalServiceMetadata {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            service_id: Uuid::new_v4(),
            name: "Unknown Service".to_string(),
            version: "0.1.0".to_string(),
            description: "No description provided".to_string(),
            service_type: UnifiedServiceType::Generic,
            capabilities: Vec::new(),
            specialized_capabilities: HashMap::new(),
            endpoints: Vec::new(),
            primary_endpoint: None,
            health_endpoint: None,
            metrics_endpoint: None,
            tags: Vec::new(),
            metadata: HashMap::new(),
            simple_metadata: HashMap::new(),
            maintainer: None,
            repository: None,
            documentation: None,
            license: None,
            supported_platforms: Vec::new(),
            min_ecosystem_version: None,
            created_at: now,
            updated_at: now,
            last_health_check: None,
            status: ServiceStatus::Unknown,
            health_state: HealthState::Unknown,
            resources: None,
            extensions: HashMap::new(),
        }
    }
}

// === CONSTRUCTORS & BUILDERS ===

impl UniversalServiceMetadata {
    /// Create new service metadata with minimal required information
    pub fn new(name: &str, service_type: UnifiedServiceType) -> Self {
        Self {
            name: name.to_string(),
            service_type,
            ..Default::default()
        }
    }

    /// Builder pattern for service metadata
    pub fn builder(
        name: &str,
        service_type: UnifiedServiceType,
    ) -> UniversalServiceMetadataBuilder {
        UniversalServiceMetadataBuilder::new(name, service_type)
    }

    /// Add a capability
    pub fn add_capability(&mut self, capability: ServiceCapability) {
        if !self.capabilities.contains(&capability) {
            self.capabilities.push(capability);
        }
        self.updated_at = Utc::now();
    }

    /// Add an endpoint
    pub fn add_endpoint(&mut self, endpoint: ServiceEndpoint) {
        self.endpoints.push(endpoint);
        self.updated_at = Utc::now();
    }

    /// Update health state
    pub fn update_health(&mut self, health_state: HealthState) {
        self.health_state = health_state;
        self.last_health_check = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Check if service has a specific capability
    pub fn has_capability(&self, capability: &ServiceCapability) -> bool {
        self.capabilities.contains(capability)
    }

    /// Get primary endpoint URL
    pub fn get_primary_endpoint(&self) -> Option<&str> {
        // First check explicit primary endpoint
        if let Some(ref primary) = self.primary_endpoint {
            return Some(primary);
        }

        // Find primary endpoint in endpoints list
        self.endpoints
            .iter()
            .find(|ep| ep.is_primary)
            .map(|ep| ep.url.as_str())
            .or_else(|| {
                // Fallback to first endpoint
                self.endpoints.first().map(|ep| ep.url.as_str())
            })
    }
}

/// Builder for UniversalServiceMetadata
pub struct UniversalServiceMetadataBuilder {
    metadata: UniversalServiceMetadata,
}

impl UniversalServiceMetadataBuilder {
    pub fn new(name: &str, service_type: UnifiedServiceType) -> Self {
        Self {
            metadata: UniversalServiceMetadata::new(name, service_type),
        }
    }

    pub fn version(mut self, version: &str) -> Self {
        self.metadata.version = version.to_string();
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.metadata.description = description.to_string();
        self
    }

    pub fn capability(mut self, capability: ServiceCapability) -> Self {
        self.metadata.add_capability(capability);
        self
    }

    pub fn endpoint(mut self, endpoint: ServiceEndpoint) -> Self {
        self.metadata.add_endpoint(endpoint);
        self
    }

    pub fn primary_endpoint(mut self, url: &str) -> Self {
        self.metadata.primary_endpoint = Some(url.to_string());
        self
    }

    pub fn tag(mut self, tag: &str) -> Self {
        self.metadata.tags.push(tag.to_string());
        self
    }

    pub fn maintainer(mut self, maintainer: ContactInfo) -> Self {
        self.metadata.maintainer = Some(maintainer);
        self
    }

    pub fn build(mut self) -> UniversalServiceMetadata {
        self.metadata.updated_at = Utc::now();
        self.metadata
    }
}

// === MIGRATION UTILITIES ===

impl UniversalServiceMetadata {
    /// Migrate from legacy UnifiedServiceInfo
    pub fn from_unified_service_config(
        config: &crate::unified_types::UnifiedServiceConfig,
    ) -> Self {
        let mut metadata = Self::new(&config.name, UnifiedServiceType::Generic);

        metadata.version = config.version.clone();
        metadata.description = config.description.clone();
        metadata.tags = Vec::new(); // UnifiedServiceConfig doesn't have tags
        metadata.metadata = std::collections::HashMap::new(); // Initialize empty

        // UnifiedServiceConfig doesn't have capabilities like the old structure
        // Add generic capability
        metadata.add_capability(ServiceCapability::Custom("generic".to_string()));

        // Add a default endpoint based on service configuration
        let default_endpoint = ServiceEndpoint {
            url: format!("http://{}:{}", config.service_name, 8080), // Default endpoint
            protocol: CommunicationProtocol::Http,
            endpoint_type: EndpointType::Primary,
            health_check: None,
            is_primary: true,
        };
        metadata.add_endpoint(default_endpoint);

        // Set primary endpoint
        metadata.primary_endpoint = Some(format!("http://{}:{}", config.service_name, 8080));

        metadata
    }
}

// === CAPABILITY PARSING ===

impl std::str::FromStr for ServiceCapability {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "httpserver" | "http_server" | "http" => Ok(ServiceCapability::HttpServer),
            "websocket" | "ws" => Ok(ServiceCapability::WebSocket),
            "database" | "db" => Ok(ServiceCapability::Database),
            "filesystem" | "file_system" | "fs" => Ok(ServiceCapability::FileSystem),
            "network" => Ok(ServiceCapability::Network),
            "authentication" | "auth" => Ok(ServiceCapability::Authentication),
            "authorization" | "authz" => Ok(ServiceCapability::Authorization),
            "monitoring" => Ok(ServiceCapability::Monitoring),
            "logging" => Ok(ServiceCapability::Logging),
            "cache" | "caching" => Ok(ServiceCapability::Cache),
            "messagequeue" | "message_queue" | "mq" => Ok(ServiceCapability::MessageQueue),
            "orchestration" => Ok(ServiceCapability::Orchestration),
            "servicediscovery" | "service_discovery" => Ok(ServiceCapability::ServiceDiscovery),
            "loadbalancing" | "load_balancing" | "lb" => Ok(ServiceCapability::LoadBalancing),
            "tlstermination" | "tls_termination" | "tls" => Ok(ServiceCapability::TlsTermination),
            "apigateway" | "api_gateway" => Ok(ServiceCapability::ApiGateway),
            _ => Ok(ServiceCapability::Custom(s.to_string())),
        }
    }
}
