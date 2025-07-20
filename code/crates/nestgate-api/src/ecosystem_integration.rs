//! Universal Ecosystem Integration Implementation
//!
//! Implements truly universal API patterns for ecosystem integration with
//! capability-based service discovery and dynamic service registration.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Universal Service Registration - ALL PARTICIPANTS MUST IMPLEMENT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalServiceRegistration {
    /// Unique service identifier (generated)
    pub service_id: Uuid,

    /// Service metadata
    pub metadata: ServiceMetadata,

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

/// Service metadata with open categorization
#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub enum ServiceCategory {
    /// Computational services
    Compute { specialties: Vec<String> },

    /// Storage and data services
    Storage { types: Vec<String> },

    /// Security and identity services
    Security { domains: Vec<String> },

    /// Network and communication services
    Network { layers: Vec<String> },

    /// Orchestration and coordination services
    Orchestration { scopes: Vec<String> },

    /// Artificial intelligence services
    Intelligence { modalities: Vec<String> },

    /// Community-defined custom categories
    Custom {
        category: String,
        subcategories: Vec<String>,
    },
}

/// Universal capability system (extensible)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceCapability {
    /// Computational capabilities
    Computation {
        types: Vec<String>,
        resources: ComputeResources,
        constraints: Vec<String>,
    },

    /// Data management capabilities
    DataManagement {
        operations: Vec<String>,
        consistency: ConsistencyLevel,
        durability: DurabilityLevel,
    },

    /// Security capabilities
    Security {
        functions: Vec<String>,
        compliance: Vec<String>,
        trust_levels: Vec<String>,
    },

    /// Network capabilities
    Networking {
        protocols: Vec<String>,
        topologies: Vec<String>,
        qos_levels: Vec<String>,
    },

    /// Coordination capabilities
    Coordination {
        patterns: Vec<String>,
        consistency: String,
        fault_tolerance: String,
    },

    /// AI/ML capabilities
    ArtificialIntelligence {
        models: Vec<String>,
        tasks: Vec<String>,
        interfaces: Vec<String>,
    },

    /// Custom capabilities (community extensible)
    Custom {
        domain: String,
        capability: String,
        parameters: HashMap<String, serde_json::Value>,
    },
}

/// Universal service endpoints - ALL PARTICIPANTS MUST IMPLEMENT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    /// Endpoint identifier
    pub endpoint_id: String,
    /// Endpoint URL
    pub url: String,
    /// Endpoint type (http, websocket, grpc, custom)
    pub endpoint_type: EndpointType,
    /// Security requirements
    pub security: SecurityRequirements,
    /// Health check configuration
    pub health_check: Option<HealthCheckConfig>,
}

/// Endpoint types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EndpointType {
    Http,
    WebSocket,
    Grpc,
    Custom(String),
}

/// Security requirements for endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRequirements {
    pub tls_required: bool,
    pub authentication_required: bool,
    pub authorization_required: bool,
    pub audit_logging: bool,
}

/// Resource specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSpec {
    /// CPU cores required (can be fractional)
    pub cpu_cores: Option<f64>,
    /// Memory requirement in MB
    pub memory_mb: Option<u64>,
    /// Storage requirement in GB
    pub storage_gb: Option<u64>,
    /// Network bandwidth in Mbps
    pub network_mbps: Option<u64>,
    /// GPU requirements
    pub gpu_requirements: Option<GpuRequirements>,
}

/// GPU requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuRequirements {
    pub gpu_count: u32,
    pub gpu_memory_gb: u32,
    pub gpu_type: Option<String>,
}

/// Integration preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationPreferences {
    pub discovery_methods: Vec<DiscoveryMethod>,
    pub communication_protocols: Vec<CommunicationProtocol>,
    pub security_preferences: SecurityPreferences,
}

/// Discovery methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryMethod {
    Registry,
    Dns,
    Multicast,
    Custom(String),
}

/// Communication protocols
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommunicationProtocol {
    Http,
    WebSocket,
    Grpc,
    Custom(String),
}

/// Security preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPreferences {
    pub preferred_auth_methods: Vec<String>,
    pub encryption_requirements: EncryptionLevel,
    pub certificate_validation: bool,
}

/// Standard request format for all ecosystem communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalRequest {
    /// Unique request identifier
    pub request_id: Uuid,

    /// Source service identifier
    pub source_service: Uuid,

    /// Target service identifier  
    pub target_service: Uuid,

    /// Request operation
    pub operation: String,

    /// Request payload
    pub payload: serde_json::Value,

    /// Security context
    pub security_context: SecurityContext,

    /// Request metadata
    pub metadata: HashMap<String, String>,

    /// Request timestamp
    pub timestamp: DateTime<Utc>,
}

/// Standard response format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalResponse {
    /// Request ID this response is for
    pub request_id: Uuid,

    /// Response status
    pub status: ResponseStatus,

    /// Response payload
    pub payload: serde_json::Value,

    /// Response metadata
    pub metadata: HashMap<String, String>,

    /// Response timestamp
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseStatus {
    Success,
    Error { code: String, message: String },
    Timeout,
    ServiceUnavailable,
}

/// Security context for all requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityContext {
    /// Authentication token
    pub auth_token: Option<String>,

    /// User/service identity
    pub identity: String,

    /// Permissions/capabilities
    pub permissions: Vec<String>,

    /// Security level required
    pub security_level: SecurityLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityLevel {
    Public,
    Internal,
    Restricted,
    Confidential,
}

// Supporting types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInfo {
    pub name: String,
    pub email: Option<String>,
    pub organization: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeResources {
    pub cpu_cores: f64,
    pub memory_gb: u32,
    pub storage_gb: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsistencyLevel {
    Eventual,
    Strong,
    Causal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DurabilityLevel {
    Memory,
    Persistent,
    Replicated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionLevel {
    None,
    Basic,
    Strong,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    pub endpoint: String,
    pub interval_seconds: u32,
    pub timeout_seconds: u32,
    pub failure_threshold: u32,
}

/// Performance capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceCapabilities {
    /// Maximum throughput in operations per second
    pub max_throughput: Option<u64>,
    /// Average latency in milliseconds
    pub avg_latency_ms: Option<u64>,
    /// Service level agreement uptime percentage
    pub sla_uptime_percent: Option<f64>,
    /// Scalability characteristics
    pub scalability: ScalabilityInfo,
}

/// Scalability information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalabilityInfo {
    /// Horizontal scaling support
    pub horizontal: bool,
    /// Vertical scaling support
    pub vertical: bool,
    /// Auto-scaling capabilities
    pub auto_scaling: bool,
}

/// Subscription handle for event subscriptions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionHandle {
    /// Subscription ID
    pub subscription_id: Uuid,
    /// Event types
    pub event_types: Vec<String>,
    /// Subscription timestamp
    pub created_at: DateTime<Utc>,
    /// Active status
    pub active: bool,
}

/// Discovered service information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredService {
    /// Service ID
    pub service_id: Uuid,
    /// Service name
    pub name: String,
    /// Service type
    pub primal_type: PrimalType,
    /// Service endpoints
    pub endpoints: Vec<String>,
    /// Service capabilities
    pub capabilities: Vec<String>,
    /// Service metadata
    pub metadata: HashMap<String, String>,
    /// Discovery timestamp
    pub discovered_at: DateTime<Utc>,
    /// Health status
    pub health_status: String,
}

/// Ecosystem error types
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum EcosystemError {
    #[error("Network error: {message}")]
    NetworkError { message: String },

    #[error("Service not found: {service_id}")]
    ServiceNotFound { service_id: String },

    #[error("Registration failed: {reason}")]
    RegistrationFailed { reason: String },

    #[error("Authentication failed")]
    AuthenticationFailed,

    #[error("Invalid configuration: {field}")]
    InvalidConfiguration { field: String },

    #[error("Timeout occurred: {operation}")]
    Timeout { operation: String },

    #[error("Internal error: {message}")]
    Internal { message: String },
}

/// Service registration response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistrationResponse {
    /// Registration success status
    pub success: bool,
    /// Assigned service ID
    pub service_id: Uuid,
    /// Registration message
    pub message: String,
    /// Registration timestamp
    pub timestamp: DateTime<Utc>,
}

/// Service registration updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistrationUpdates {
    /// Service ID to update
    pub service_id: Uuid,
    /// Updated metadata
    pub metadata: Option<ServiceMetadata>,
    /// Updated capabilities
    pub capabilities: Option<Vec<ServiceCapability>>,
    /// Updated endpoints
    pub endpoints: Option<Vec<ServiceEndpoint>>,
    /// Update timestamp
    pub timestamp: DateTime<Utc>,
}

/// Service health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealthStatus {
    /// Service ID
    pub service_id: Uuid,
    /// Health status
    pub status: String,
    /// Status message
    pub message: String,
    /// Last check timestamp
    pub last_check: DateTime<Utc>,
    /// Response time in milliseconds
    pub response_time_ms: u64,
}

/// Service event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    ServiceRegistered,
    ServiceDeregistered,
    ServiceHealthChanged,
    ServiceUpdated,
    Custom(String),
}

/// Service event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEvent {
    /// Event ID
    pub event_id: Uuid,
    /// Event type
    pub event_type: EventType,
    /// Source service ID
    pub service_id: Uuid,
    /// Event data
    pub data: serde_json::Value,
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
}

/// Primal types for ecosystem integration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrimalType {
    Storage,
    Security,
    AI,
    Compute,
    Network,
    Custom(String),
}

/// Type aliases for backward compatibility
pub type EcosystemServiceRegistration = UniversalServiceRegistration;
pub type NestGateServiceRegistration = UniversalServiceRegistration;

impl UniversalServiceRegistration {
    /// Create a new service registration
    pub fn create_registration(
        instance_id: String,
        host: String,
        port: u16,
        biome_id: Option<String>,
    ) -> Self {
        Self {
            service_id: Uuid::new_v4(),
            metadata: ServiceMetadata {
                name: "NestGate".into(),
                category: ServiceCategory::Storage {
                    types: vec!["ZFS".into(), "NAS".into()],
                },
                version: "0.1.0".into(),
                description: "Universal storage management system".into(),
                maintainer: ContactInfo {
                    name: "NestGate Team".into(),
                    email: Some("admin@nestgate.local".into()),
                    organization: Some("NestGate Project".into()),
                },
                protocols: vec!["HTTP".into(), "WebSocket".into()],
            },
            capabilities: vec![ServiceCapability::DataManagement {
                operations: vec![
                    "create".into(),
                    "read".into(),
                    "update".into(),
                    "delete".into(),
                ],
                consistency: ConsistencyLevel::Strong,
                durability: DurabilityLevel::Persistent,
            }],
            resources: ResourceSpec {
                cpu_cores: Some(4.0),
                memory_mb: Some(8192),
                storage_gb: Some(1000),
                network_mbps: Some(1000),
                gpu_requirements: None,
            },
            endpoints: vec![
                ServiceEndpoint {
                    endpoint_id: "primary".into(),
                    endpoint_type: EndpointType::Http,
                    url: format!("http://{host}:{port}"),
                    security: SecurityRequirements {
                        tls_required: false,
                        authentication_required: false,
                        authorization_required: false,
                        audit_logging: false,
                    },
                    health_check: None,
                },
                ServiceEndpoint {
                    endpoint_id: "health".into(),
                    endpoint_type: EndpointType::Http,
                    url: format!("http://{host}:{port}/health"),
                    security: SecurityRequirements {
                        tls_required: false,
                        authentication_required: false,
                        authorization_required: false,
                        audit_logging: false,
                    },
                    health_check: Some(HealthCheckConfig {
                        endpoint: format!("http://{host}:{port}/health"),
                        interval_seconds: 30,
                        timeout_seconds: 5,
                        failure_threshold: 3,
                    }),
                },
            ],
            integration: IntegrationPreferences {
                discovery_methods: vec![DiscoveryMethod::Registry],
                communication_protocols: vec![CommunicationProtocol::Http],
                security_preferences: SecurityPreferences {
                    preferred_auth_methods: vec!["none".into()],
                    encryption_requirements: EncryptionLevel::None,
                    certificate_validation: false,
                },
            },
            extensions: {
                let mut ext = HashMap::new();
                if let Some(biome) = biome_id {
                    ext.insert("biome_id".to_string(), serde_json::Value::String(biome));
                }
                ext
            },
            registration_timestamp: Utc::now(),
            service_version: "0.1.0".into(),
            instance_id,
            priority: 5,
        }
    }
}

/// Service request for ecosystem communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRequest {
    /// Request ID
    pub request_id: Uuid,
    /// Target service
    pub service_id: String,
    /// Operation to perform
    pub operation: String,
    /// Request payload
    pub payload: serde_json::Value,
    /// Request timestamp
    pub timestamp: DateTime<Utc>,
}

/// Service response for ecosystem communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceResponse {
    /// Request ID this responds to
    pub request_id: Uuid,
    /// Response success status
    pub success: bool,
    /// Response payload
    pub payload: serde_json::Value,
    /// Error message (if any)
    pub error: Option<String>,
    /// Response timestamp
    pub timestamp: DateTime<Utc>,
}

/// Service metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMetrics {
    /// Service ID
    pub service_id: Uuid,
    /// CPU usage percentage
    pub cpu_usage: f64,
    /// Memory usage in bytes
    pub memory_usage: u64,
    /// Network I/O statistics
    pub network_io: HashMap<String, u64>,
    /// Request count
    pub request_count: u64,
    /// Error count
    pub error_count: u64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Metrics timestamp
    pub timestamp: DateTime<Utc>,
}

/// Songbird integration trait
#[async_trait::async_trait]
pub trait SongbirdIntegration {
    /// Register service with Songbird
    async fn register_service(
        &self,
        registration: &UniversalServiceRegistration,
    ) -> Result<ServiceRegistrationResponse, EcosystemError>;

    /// Update service registration
    async fn update_registration(
        &self,
        service_id: &str,
        updates: ServiceRegistrationUpdates,
    ) -> Result<(), EcosystemError>;

    /// Deregister service
    async fn deregister_service(&self, service_id: &str) -> Result<(), EcosystemError>;

    /// Discover services by type
    async fn discover_services(
        &self,
        primal_type: PrimalType,
    ) -> Result<Vec<DiscoveredService>, EcosystemError>;

    /// Get service by capability
    async fn discover_services_by_capability(
        &self,
        capability: &str,
    ) -> Result<Vec<DiscoveredService>, EcosystemError>;

    /// Send request to service
    async fn send_request(
        &self,
        request: UniversalRequest,
    ) -> Result<UniversalResponse, EcosystemError>;

    /// Send service request
    async fn send_service_request(
        &self,
        target_service: &str,
        request: ServiceRequest,
    ) -> Result<ServiceResponse, EcosystemError>;

    /// Subscribe to service events
    async fn subscribe_to_events(
        &self,
        event_types: Vec<EventType>,
        callback: Box<dyn Fn(ServiceEvent) + Send + Sync>,
    ) -> Result<SubscriptionHandle, EcosystemError>;

    /// Get service health status
    async fn get_service_health(
        &self,
        service_id: &str,
    ) -> Result<ServiceHealthStatus, EcosystemError>;

    /// Get service metrics
    async fn get_service_metrics(&self, service_id: &str)
        -> Result<ServiceMetrics, EcosystemError>;
}
