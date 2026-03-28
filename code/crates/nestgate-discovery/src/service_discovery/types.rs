// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

/// Universal Service Discovery Types
/// Extracted from `universal_service_discovery.rs` to maintain file size compliance
/// Contains all type definitions for the Universal Primal Architecture Standard
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
/// Universal service registration following the Universal Primal Architecture Standard
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Universalserviceregistration
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
}
/// Service metadata information
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Servicemetadata
pub struct ServiceMetadata {
    /// Human-readable service name
    pub name: String,
    /// Service category (open enumeration)
    pub category: ServiceCategory,
    /// Version information
    pub version: String,
    /// Description of service functionality
    pub description: String,
    /// Health status endpoint
    pub health_endpoint: Option<String>,
    /// Metrics endpoint
    pub metrics_endpoint: Option<String>,
}
/// Service category enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
/// Servicecategory
pub enum ServiceCategory {
    /// Storage and persistence services
    #[default]
    /// Storage
    Storage,
    /// AI and machine learning services
    AI,
    /// Security and authentication services
    Security,
    /// Network and communication services
    Network,
    /// Orchestration and workflow services
    Orchestration,
    /// Monitoring and observability services
    Monitoring,
    /// User interface and visualization services
    UI,
    /// Data processing and analytics services
    DataProcessing,
    /// Integration and adapter services
    Integration,
    /// Development and tooling services
    Development,
    /// Custom category (specify in description)
    Custom(String),
}
/// Service capability enumeration - what a service can do
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Servicecapability
pub enum ServiceCapability {
    /// Storage capabilities
    Storage(StorageType),
    /// AI capabilities  
    AI(AIModality),
    /// Security capabilities
    Security(SecurityFunction),
    /// Orchestration capabilities
    Orchestration(OrchestrationScope),
    /// Network capabilities
    Network(CommunicationProtocol),
    /// Custom capability
    Custom {
        /// Capability namespace
        namespace: String,
        /// Capability identifier
        capability: String,
        /// Capability version
        version: String,
    },
}
/// Storage type capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Types of Storage
pub enum StorageType {
    /// Object storage (S3-compatible)
    Object,
    /// Block storage
    Block,
    /// File system storage
    FileSystem,
    /// Database storage
    Database,
    /// Cache storage
    Cache,
    /// Archive storage
    Archive,
}
/// Security domain capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Securitydomain
pub enum SecurityDomain {
    /// Authentication services
    Authentication,
    /// Authorization services
    Authorization,
    /// Encryption services
    Encryption,
    /// Certificate management
    CertificateManagement,
    /// Audit and compliance
    Audit,
    /// Threat detection
    ThreatDetection,
}
/// Integration type capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Types of Integration
pub enum IntegrationType {
    /// API integration
    Api,
    /// Event-driven integration
    EventDriven,
    /// Batch integration
    Batch,
    /// Real-time streaming
    Streaming,
    /// File-based integration
    File,
}
/// Integration pattern capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Integrationpattern
pub enum IntegrationPattern {
    /// Request-response pattern
    RequestResponse,
    /// Publisher-subscriber pattern
    PubSub,
    /// Event sourcing pattern
    EventSourcing,
    /// Command query responsibility segregation
    Cqrs,
    /// Microservices pattern
    Microservices,
}
/// Communication protocol capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Communicationprotocol
pub enum CommunicationProtocol {
    /// HTTP/HTTPS protocol
    Http,
    /// gRPC protocol
    Grpc,
    /// WebSocket protocol
    WebSocket,
    /// TCP protocol
    Tcp,
    /// UDP protocol
    Udp,
    /// Message queue protocols
    MessageQueue,
}
/// Resource specification
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Resourcespec
pub struct ResourceSpec {
    /// CPU requirements (cores)
    pub cpu_cores: Option<f64>,
    /// Memory requirements (MB)
    pub memory_mb: Option<u64>,
    /// Disk space requirements (GB)
    pub disk_gb: Option<u64>,
    /// Network bandwidth requirements (Mbps)
    pub network_mbps: Option<u64>,
    /// Resource constraints
    pub constraints: ResourceConstraints,
}
/// Integration preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Integrationpreferences
pub struct IntegrationPreferences {
    /// Preferred integration types
    pub preferred_types: Vec<IntegrationType>,
    /// Preferred patterns
    pub preferred_patterns: Vec<IntegrationPattern>,
    /// Preferred protocols
    pub preferred_protocols: Vec<CommunicationProtocol>,
    /// Cost sensitivity
    pub cost_sensitivity: CostSensitivity,
}
/// Resource constraints
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Resourceconstraints
pub struct ResourceConstraints {
    /// Maximum CPU cores
    pub max_cpu_cores: Option<f64>,
    /// Maximum memory (MB)
    pub max_memory_mb: Option<u64>,
    /// Maximum disk space (GB)
    pub max_disk_gb: Option<u64>,
    /// Performance requirements
    pub performance: PerformanceRequirements,
}
/// Performance requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performancerequirements
pub struct PerformanceRequirements {
    /// Maximum latency (milliseconds)
    pub max_latency_ms: Option<u64>,
    /// Minimum throughput (requests per second)
    pub min_throughput_rps: Option<u64>,
    /// Availability requirement (percentage)
    pub availability_percent: Option<f64>,
}
/// Cost sensitivity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Costsensitivity
pub enum CostSensitivity {
    /// Cost is not a concern
    None,
    /// Low cost sensitivity
    Low,
    /// Medium cost sensitivity
    Medium,
    /// High cost sensitivity (optimize for cost)
    High,
}
/// Data operation capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Dataoperation
pub enum DataOperation {
    /// Read operations
    Read,
    /// Write operations
    Write,
    /// Update operations
    Update,
    /// Delete operations
    Delete,
    /// Query operations
    Query,
    /// Analytics operations
    Analytics,
}
/// Consistency level requirements
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Consistencylevel
pub enum ConsistencyLevel {
    /// Strong consistency
    Strong,
    /// Eventual consistency
    Eventual,
    /// Session consistency
    Session,
}
/// Durability level requirements
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Durabilitylevel
pub enum DurabilityLevel {
    /// No durability guarantees
    None,
    /// Memory-based durability
    Memory,
    /// Disk-based durability
    Disk,
    /// Replicated durability
    Replicated,
}
/// Security function capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Securityfunction
pub enum SecurityFunction {
    /// User authentication
    Authentication,
    /// Access authorization
    Authorization,
    /// Data encryption
    Encryption,
    /// Certificate management
    CertificateManagement,
    /// Security auditing
    Auditing,
    /// Intrusion detection
    IntrusionDetection,
}
/// Compliance framework requirements
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Complianceframework
pub enum ComplianceFramework {
    /// GDPR compliance
    Gdpr,
    /// HIPAA compliance
    Hipaa,
    /// SOC2 compliance
    SOC2,
    /// ISO27001 compliance
    ISO27001,
    /// PCI DSS compliance
    Pcidss,
}
/// Trust level requirements
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Trustlevel
pub enum TrustLevel {
    /// Public (no trust required)
    Public,
    /// Internal (organization trust)
    Internal,
    /// Confidential (high trust)
    Confidential,
    /// Restricted (maximum trust)
    Restricted,
}
/// Orchestration scope capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Orchestrationscope
pub enum OrchestrationScope {
    /// Single service orchestration
    Service,
    /// Multi-service orchestration
    Workflow,
    /// Cross-system orchestration
    System,
    /// Infrastructure orchestration
    Infrastructure,
}
/// Coordination pattern capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Coordinationpattern
pub enum CoordinationPattern {
    /// Centralized coordination
    Centralized,
    /// Decentralized coordination
    Decentralized,
    /// Hierarchical coordination
    Hierarchical,
    /// Peer-to-peer coordination
    PeerToPeer,
}
/// Consistency model requirements
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Consistencymodel
pub enum ConsistencyModel {
    /// Linearizable consistency
    Linearizable,
    /// Sequential consistency
    Sequential,
    /// Causal consistency
    Causal,
    /// Eventual consistency
    Eventual,
}
/// Fault tolerance level requirements
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Faulttolerancelevel
pub enum FaultToleranceLevel {
    /// No fault tolerance
    None,
    /// Basic fault tolerance
    Basic,
    /// High fault tolerance
    High,
    /// Maximum fault tolerance
    Maximum,
}
/// AI modality capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Aimodality
pub enum AIModality {
    /// Natural language processing
    Nlp,
    /// Computer vision
    Vision,
    /// Speech processing
    Speech,
    /// Machine learning
    MachineLearning,
    /// Deep learning
    DeepLearning,
    /// Reinforcement learning
    ReinforcementLearning,
}
/// AI model type capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Types of AIModel
pub enum AIModelType {
    /// Large language model
    Llm,
    /// Convolutional neural network
    Cnn,
    /// Recurrent neural network
    Rnn,
    /// Transformer model
    Transformer,
    /// Decision tree
    DecisionTree,
    /// Support vector machine
    Svm,
}
/// AI task capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Aitask
pub enum AITask {
    /// Text generation
    TextGeneration,
    /// Text classification
    TextClassification,
    /// Image recognition
    ImageRecognition,
    /// Speech recognition
    SpeechRecognition,
    /// Prediction
    Prediction,
    /// Recommendation
    Recommendation,
}
/// AI interface capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Aiinterface
pub enum AIInterface {
    /// REST API interface
    Rest,
    /// gRPC interface
    Grpc,
    /// WebSocket interface
    WebSocket,
    /// Command line interface
    Cli,
    /// Library interface
    Library,
}
/// Development context capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Developmentcontext
pub enum DevelopmentContext {
    /// Local development
    Local,
    /// Testing environment
    Testing,
    /// Staging environment
    Staging,
    /// Production environment
    Production,
    /// CI/CD pipeline
    Cicd,
}
/// Service role definition
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Servicerole
pub struct ServiceRole {
    /// Role name
    pub name: String,
    /// Required capabilities for this role
    pub required_capabilities: Vec<ServiceCapability>,
    /// Optional capabilities
    pub optional_capabilities: Vec<ServiceCapability>,
    /// Resource requirements
    pub resource_requirements: ResourceSpec,
    /// Performance requirements
    pub performance_requirements: PerformanceRequirements,
}
/// Capability requirement
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Capabilityrequirement
pub struct CapabilityRequirement {
    /// Required capability
    pub capability: ServiceCapability,
    /// Whether this requirement is optional
    pub optional: bool,
    /// Version constraints
    pub version_constraint: Option<String>,
    /// Additional parameters
    pub parameters: HashMap<String, serde_json::Value>,
}
/// Service handle for external references
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Servicehandle
pub struct ServiceHandle {
    /// Service identifier
    pub service_id: Uuid,
    /// Name
    pub name: String,
    /// Endpoints
    pub endpoints: Vec<ServiceEndpoint>,
}
/// Service information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Serviceinfo
pub struct ServiceInfo {
    /// Service identifier
    pub service_id: Uuid,
    /// Additional metadata key-value pairs
    pub metadata: ServiceMetadata,
    /// Capabilities
    pub capabilities: Vec<ServiceCapability>,
    /// Endpoints
    pub endpoints: Vec<ServiceEndpoint>,
    /// Last Seen
    pub last_seen: std::time::SystemTime,
}
/// Service endpoint definition
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Serviceendpoint
pub struct ServiceEndpoint {
    /// Url
    pub url: String,
    /// Protocol
    pub protocol: CommunicationProtocol,
    /// Health Check
    pub health_check: Option<String>,
}
/// Service requirements for discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Servicerequirements
pub struct ServiceRequirements {
    /// Capabilities
    pub capabilities: Vec<ServiceCapability>,
    /// Resource Constraints
    pub resource_constraints: Option<ResourceConstraints>,
    /// Performance Requirements
    pub performance_requirements: Option<PerformanceRequirements>,
}
/// Selection preferences for service discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Selectionpreferences
pub struct SelectionPreferences {
    /// Prefer Local
    pub prefer_local: bool,
    /// Cost Sensitivity
    pub cost_sensitivity: CostSensitivity,
    /// Performance Priority
    pub performance_priority: bool,
}
// ==================== SECTION ====================

impl Default for PerformanceRequirements {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            max_latency_ms: None,
            min_throughput_rps: None,
            availability_percent: Some(99.9),
        }
    }
}

impl Default for IntegrationPreferences {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            preferred_types: vec![IntegrationType::Api],
            preferred_patterns: vec![IntegrationPattern::RequestResponse],
            preferred_protocols: vec![CommunicationProtocol::Http],
            cost_sensitivity: CostSensitivity::Medium,
        }
    }
}

impl Default for SelectionPreferences {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            prefer_local: true,
            cost_sensitivity: CostSensitivity::Medium,
            performance_priority: false,
        }
    }
}

/// Discovered service information for network layer
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Service implementation for Discovered
pub struct DiscoveredService {
    /// Unique identifier
    pub id: String,
    /// Name
    pub name: String,
    /// Endpoint
    pub endpoint: String,
    /// Port
    pub port: u16,
    /// Capabilities
    pub capabilities: Vec<String>,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
    /// Discovered At
    pub discovered_at: std::time::SystemTime,
}
impl Default for DiscoveredService {
    /// Returns the default instance
    fn default() -> Self {
        use nestgate_config::constants::hardcoding::{addresses, ports};
        Self {
            id: Uuid::new_v4().to_string(),
            name: "unknown".to_string(),
            endpoint: addresses::LOCALHOST_NAME.to_string(),
            port: ports::HTTP_DEFAULT,
            capabilities: vec![],
            metadata: HashMap::new(),
            discovered_at: std::time::SystemTime::now(),
        }
    }
}
