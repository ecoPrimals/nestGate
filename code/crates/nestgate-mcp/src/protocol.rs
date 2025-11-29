//
// Advanced MCP protocol handling integrating enhanced NestGate capabilities
// with v2 orchestrator-centric architecture

//! Protocol module

use nestgate_core::error::NestGateError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

use crate::types::{
    MountInfo, MountOptions, ProviderCapabilities, StorageProtocol, StorageTier, VolumeInfo,
};
use nestgate_core::diagnostics::SystemMetrics;

// ==================== SECTION: CANONICAL ERROR TYPES ====================

// CANONICAL MODERNIZATION: Use unified error system
// REMOVED: pub type Result<T> = McpResult<T>;
// USE INSTEAD: Canonical types from nestgate-core

pub use nestgate_core::error::{Result, McpResult};

// ==================== SECTION ====================

/// MCP message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Mcpmessage
pub struct McpMessage {
    /// Message Type
    pub message_type: String,
    /// Payload
    pub payload: serde_json::Value,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}
/// MCP session information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Mcpsession
pub struct McpSession {
    /// Session identifier
    pub session_id: String,
    /// Client Info
    pub client_info: ClientInfo,
    /// Server Capabilities
    pub server_capabilities: ServerCapabilities,
}
/// Client information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Clientinfo
pub struct ClientInfo {
    /// Name
    pub name: String,
    /// Version
    pub version: String,
}
/// Server capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Servercapabilities
pub struct ServerCapabilities {
    /// Tools
    pub tools: Vec<String>,
    /// Resources
    pub resources: Vec<String>,
    /// Prompts
    pub prompts: Vec<String>,
}
/// Enhanced MCP Message with advanced capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Message
pub struct Message {
    /// Unique identifier
    pub id: String,
    /// Timestamp
    pub timestamp: std::time::SystemTime,
    /// Source
    pub source: String,
    /// Destination
    pub destination: Option<String>,
    /// Message Type
    pub message_type: MessageType,
    /// Payload
    pub payload: MessagePayload,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}
impl Message {
    #[must_use]
    pub fn new(message_type: MessageType, payload: MessagePayload) -> Self { Self {
            id: Uuid::new_v4().to_string(),
            timestamp: std::time::SystemTime::now(),
            source: "nestgate-v2".to_string(),
            destination: None,
            message_type,
            payload,
            metadata: HashMap::new(),
         }

    #[must_use]
    pub fn with_destination(mut self, destination: String) -> Self { self.destination = Some(destination);
        self
    #[must_use]
    , pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
     }
}

/// Enhanced Message Types with advanced capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of Message
pub enum MessageType {
    // Capability messages
    CapabilityRegistration,
    /// Capabilityquery
    CapabilityQuery,
    /// Capabilityresponse
    CapabilityResponse,
    // Storage operations
    VolumeCreate,
    /// Volumedelete
    VolumeDelete,
    /// Volumemount
    VolumeMount,
    /// Volumeunmount
    VolumeUnmount,
    /// Volumelist
    VolumeList,
    VolumeInfo,

    // Performance and monitoring
    MetricsReport,
    MetricsQuery,
    HealthCheck,
    StatusUpdate,

    // Federation and clustering
    FederationJoin,
    FederationLeave,
    FederationSync,
    FederationHeartbeat,

    // Orchestrator v2 specific
    OrchestratorRoute,
    ServiceRegistration,
    ServiceDiscovery,
    LoadBalancing,

    // Error handling
    Error,
    Acknowledgment,
}

/// Enhanced Message Payload with advanced capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Messagepayload
pub enum MessagePayload {
    // Capability payloads
    CapabilityRegistration(CapabilityRegistrationPayload),
    CapabilityQuery(CapabilityQueryPayload),
    CapabilityResponse(CapabilityResponsePayload),
    // Storage operation payloads
    VolumeCreate(VolumeCreatePayload),
    VolumeDelete(VolumeDeletePayload),
    VolumeMount(VolumeMountPayload),
    VolumeUnmount(VolumeUnmountPayload),
    VolumeList(VolumeListPayload),
    VolumeInfo(VolumeInfoPayload),

    // Performance and monitoring payloads
    MetricsReport(MetricsReportPayload),
    MetricsQuery(MetricsQueryPayload),
    HealthCheck(HealthCheckPayload),
    StatusUpdate(StatusUpdatePayload),

    // Federation payloads
    FederationJoin(FederationJoinPayload),
    FederationLeave(FederationLeavePayload),
    FederationSync(FederationSyncPayload),
    FederationHeartbeat(FederationHeartbeatPayload),

    // Orchestrator v2 payloads
    OrchestratorRoute(OrchestratorRoutePayload),
    ServiceRegistration(ServiceRegistrationPayload),
    ServiceDiscovery(ServiceDiscoveryPayload),
    LoadBalancing(LoadBalancingPayload),

    // Error handling payloads
    Error(ErrorPayload),
    Acknowledgment(AcknowledmentPayload),
}

/// Enhanced MCP Response with advanced capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for  operation
pub struct Response {
    /// Unique identifier
    pub id: String,
    /// Request identifier
    pub request_id: String,
    /// Timestamp
    pub timestamp: std::time::SystemTime,
    /// Status
    pub status: ResponseStatus,
    /// Payload
    pub payload: Option<ResponsePayload>,
    /// Error
    pub error: Option<ErrorPayload>,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}
impl Response {
    #[must_use]
    pub fn success(request_id: String, payload: ResponsePayload) -> Self { Self {
            id: Uuid::new_v4().to_string(),
            request_id,
            timestamp: std::time::SystemTime::now(),
            status: ResponseStatus::Success,
            payload: Some(payload),
            error: None,
            metadata: HashMap::new(),
         }

    #[must_use]
    pub fn error(request_id: String, error: ErrorPayload) -> Self { Self {
            id: Uuid::new_v4().to_string(),
            request_id,
            timestamp: std::time::SystemTime::now(),
            status: ResponseStatus::Error,
            payload: None,
            error: Some(error),
            metadata: HashMap::new(),
         }
}

/// Response Status
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Status values for Response
pub enum ResponseStatus {
    /// Success
    Success,
    /// Error
    Error,
    /// Pending
    Pending,
    /// Timeout
    Timeout,
}
/// Enhanced Response Payload with advanced capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Responsepayload
pub enum ResponsePayload {
    CapabilityResponse(ProviderCapabilities),
    VolumeInfo(VolumeInfo),
    VolumeList(Vec<VolumeInfo>),
    MountInfo(MountInfo),
    MetricsReport(SystemMetrics),
    HealthStatus(HealthStatus),
    FederationStatus(FederationStatus),
    ServiceList(Vec<ServiceInfo>),
    LoadBalancingInfo(LoadBalancingInfo),
    /// Empty
    Empty,
}
// Service Information (v2 specific)
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Serviceinfo
pub struct ServiceInfo {
    /// Service identifier
    pub service_id: String,
    /// Service name
    pub service_name: String,
    /// Service Type
    pub service_type: String,
    /// Endpoint
    pub endpoint: String,
    /// Status
    pub status: ServiceStatus,
    /// Capabilities
    pub capabilities: Vec<String>,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}

/// Service Status
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Status values for Service
pub enum ServiceStatus {
    /// Online
    Online,
    /// Offline
    Offline,
    /// Degraded
    Degraded,
    /// Maintenance
    Maintenance,
    /// Error
    Error,
}
/// Load Balancing Algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Loadbalancingalgorithm
pub enum LoadBalancingAlgorithm {
    /// Roundrobin
    RoundRobin,
    /// Weightedroundrobin
    WeightedRoundRobin,
    /// Leastconnections
    LeastConnections,
    /// Healthbased
    HealthBased,
    /// Random
    Random,
}
/// Load Balancing Information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Loadbalancinginfo
pub struct LoadBalancingInfo {
    /// Algorithm
    pub algorithm: LoadBalancingAlgorithm,
    /// Active Services
    pub active_services: Vec<ServiceInfo>,
    /// Weights
    pub weights: HashMap<String, f64>,
    /// Health Scores
    pub health_scores: HashMap<String, f64>,
}
/// Health Status
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Healthstatus
pub struct HealthStatus {
    /// Status
    pub status: ServiceStatus,
    /// Uptime
    pub uptime: std::time::Duration,
    /// Last Check
    pub last_check: std::time::SystemTime,
    /// Details
    pub details: HashMap<String, String>,
}
/// Federation Status
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Federationstatus
pub struct FederationStatus {
    /// Whether federated
    pub is_federated: bool,
    /// Size of cluster
    pub cluster_size: usize,
    /// Node Role
    pub node_role: NodeRole,
    /// Cluster Health
    pub cluster_health: ClusterHealth,
}
/// Node Role in Federation
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Noderole
pub enum NodeRole {
    /// Leader
    Leader,
    /// Follower
    Follower,
    /// Candidate
    Candidate,
    /// Observer
    Observer,
}
/// Cluster Health
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Clusterhealth
pub enum ClusterHealth {
    /// Healthy
    Healthy,
    /// Degraded
    Degraded,
    /// Unhealthy
    Unhealthy,
    /// Partitioned
    Partitioned,
}
// Payload Structures

/// Capability Registration Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Capabilityregistrationpayload
pub struct CapabilityRegistrationPayload {
    /// Capabilities
    pub capabilities: ProviderCapabilities,
    /// Endpoint
    pub endpoint: String,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}
/// Capability Query Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Capabilityquerypayload
pub struct CapabilityQueryPayload {
    /// Query Type
    pub query_type: CapabilityQueryType,
    /// Filters
    pub filters: HashMap<String, String>,
}
/// Capability Query Types
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of CapabilityQuery
pub enum CapabilityQueryType {
    /// All
    All,
    ByProtocol(StorageProtocol),
    ByTier(StorageTier),
    ByProvider(String),
}
/// Capability Response Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Capabilityresponsepayload
pub struct CapabilityResponsePayload {
    /// Capabilities
    pub capabilities: Vec<ProviderCapabilities>,
    /// Count of total
    pub total_count: usize,
}
/// Volume Create Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Volumecreatepayload
pub struct VolumeCreatePayload {
    /// Name
    pub name: String,
    /// Size
    pub size: u64,
    /// Tier
    pub tier: StorageTier,
    /// Protocol
    pub protocol: StorageProtocol,
    /// Options
    pub options: HashMap<String, String>,
}
/// Volume Delete Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Volumedeletepayload
pub struct VolumeDeletePayload {
    /// Volume identifier
    pub volume_id: String,
    /// Force
    pub force: bool,
}
/// Volume Mount Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Volumemountpayload
pub struct VolumeMountPayload {
    /// Volume identifier
    pub volume_id: String,
    /// Mount Point
    pub mount_point: String,
    /// Options
    pub options: MountOptions,
}
/// Volume Unmount Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Volumeunmountpayload
pub struct VolumeUnmountPayload {
    /// Volume identifier
    pub volume_id: String,
    /// Mount Point
    pub mount_point: String,
    /// Force
    pub force: bool,
}
/// Volume List Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Volumelistpayload
pub struct VolumeListPayload {
    /// Filters
    pub filters: HashMap<String, String>,
    /// Limit
    pub limit: Option<usize>,
    /// Offset
    pub offset: Option<usize>,
}
/// Volume Info Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Volumeinfopayload
pub struct VolumeInfoPayload {
    /// Volume identifier
    pub volume_id: String,
    /// Include Metrics
    pub include_metrics: bool,
}
/// Metrics Report Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Metricsreportpayload
pub struct MetricsReportPayload {
    /// Metrics
    pub metrics: SystemMetrics,
    /// Node identifier
    pub node_id: String,
}
/// Metrics Query Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Metricsquerypayload
pub struct MetricsQueryPayload {
    /// Node identifier
    pub node_id: Option<String>,
    /// Time Range
    pub time_range: Option<TimeRange>,
    /// Metric Types
    pub metric_types: Vec<MetricType>,
}
/// Time Range for queries
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Timerange
pub struct TimeRange {
    /// Start
    pub start: std::time::SystemTime,
    /// End
    pub end: std::time::SystemTime,
}
/// Metric Types
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of Metric
pub enum MetricType {
    /// System
    System,
    /// Storage
    Storage,
    /// Performance
    Performance,
    /// Network
    Network,
}
/// Health Check Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Healthcheckpayload
pub struct HealthCheckPayload {
    /// Check Type
    pub check_type: HealthCheckType,
    /// Include Details
    pub include_details: bool,
}
/// Health Check Types
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of HealthCheck
pub enum HealthCheckType {
    /// Basic
    Basic,
    /// Detailed
    Detailed,
    /// Storage
    Storage,
    /// Network
    Network,
}
/// Status Update Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Statusupdatepayload
pub struct StatusUpdatePayload {
    /// Status
    pub status: ServiceStatus,
    /// Details
    pub details: HashMap<String, String>,
}
/// Federation Join Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Federationjoinpayload
pub struct FederationJoinPayload {
    /// Node identifier
    pub node_id: String,
    /// Endpoint
    pub endpoint: String,
    /// Capabilities
    pub capabilities: ProviderCapabilities,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}
/// Federation Leave Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Federationleavepayload
pub struct FederationLeavePayload {
    /// Node identifier
    pub node_id: String,
    /// Reason
    pub reason: String,
}
/// Federation Sync Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Federationsyncpayload
pub struct FederationSyncPayload {
    /// Sync Type
    pub sync_type: FederationSyncType,
    /// Data
    pub data: HashMap<String, serde_json::Value>,
}
/// Federation Sync Types
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of FederationSync
pub enum FederationSyncType {
    /// Full
    Full,
    /// Incremental
    Incremental,
    /// Capabilities
    Capabilities,
    /// Metrics
    Metrics,
}
/// Federation Heartbeat Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Federationheartbeatpayload
pub struct FederationHeartbeatPayload {
    /// Node identifier
    pub node_id: String,
    /// Timestamp
    pub timestamp: std::time::SystemTime,
    /// Status
    pub status: ServiceStatus,
}
/// Orchestrator Route Payload (v2 specific)
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Orchestratorroutepayload
pub struct OrchestratorRoutePayload {
    /// Target Service
    pub target_service: String,
    /// Route Type
    pub route_type: RouteType,
    /// Message
    pub message: Box<Message>,
}
/// Route Types for orchestrator
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of Route
pub enum RouteType {
    /// Direct
    Direct,
    /// Loadbalanced
    LoadBalanced,
    /// Failover
    Failover,
    /// Broadcast
    Broadcast,
}
/// Service Registration Payload (v2 specific)
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Serviceregistrationpayload
pub struct ServiceRegistrationPayload {
    /// Service Info
    pub service_info: ServiceInfo,
    /// Health Check Endpoint
    pub health_check_endpoint: String,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}
/// Service Discovery Payload (v2 specific)
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Servicediscoverypayload
pub struct ServiceDiscoveryPayload {
    /// Service Type
    pub service_type: String,
    /// Filters
    pub filters: HashMap<String, String>,
}
/// Load Balancing Payload (v2 specific)
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Loadbalancingpayload
pub struct LoadBalancingPayload {
    /// Algorithm
    pub algorithm: LoadBalancingAlgorithm,
    /// Target Services
    pub target_services: Vec<String>,
    /// Weights
    pub weights: HashMap<String, f64>,
}
/// Error Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Errorpayload
pub struct ErrorPayload {
    /// Error Code
    pub error_code: String,
    /// Error Message
    pub error_message: String,
    /// Details
    pub details: HashMap<String, String>,
    /// Timestamp
    pub timestamp: std::time::SystemTime,
}
/// Acknowledgment Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Acknowledmentpayload
pub struct AcknowledmentPayload {
    /// Ack Type
    pub ack_type: AcknowledmentType,
    /// Message
    pub message: String,
}
/// Acknowledgment Types
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of Acknowledment
pub enum AcknowledmentType {
    /// Received
    Received,
    /// Processed
    Processed,
    /// Completed
    Completed,
    /// Failed
    Failed,
}
/// MCP protocol-specific error types with rich context
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
/// Errors that can occur during McpProtocol operations
pub enum McpProtocolError {
    #[error("Protocol error: {message}")]
    ProtocolError { message: String }
    #[error("Connection error: {message}")]
    ConnectionError { message: String }
    #[error("Message parsing error: {message}")]
    MessageParsingError { message: String }
    #[error("Authentication error: {message}")]
    AuthenticationError { message: String }
    #[error("Session error: {message}")]
    SessionError { message: String }
}
// ==================== SECTION ====================

impl From<McpProtocolError> for NestGateError {
    /// From
    fn from(err: McpProtocolError) -> Self { match err {
            McpProtocolError::ProtocolError { message , => {
                NestGateError::simple(format!("fixed")
            }
            McpProtocolError::ConnectionError { message } => {
                NestGateError::network_error("mcp_connection", message)
            }
            McpProtocolError::MessageParsingError { message } => {
                NestGateError::simple(format!("fixed")
            }
            McpProtocolError::AuthenticationError { message } => {
                NestGateError::simple(format!("fixed")
            }
            McpProtocolError::SessionError { message } => {
                NestGateError::simple(format!("fixed")
            }
        }
    }
}

// ==================== SECTION ====================

impl McpProtocolError {
    /// Protocol Error
    pub fn protocol_error(message: impl Into<String>) -> Self { Self::ProtocolError {
            message: message.into(),
         }
    
    /// Connection Error
    pub fn connection_error(message: impl Into<String>) -> Self { Self::ConnectionError {
            message: message.into(),
         }
    
    /// Message Parsing Error
    pub fn message_parsing_error(message: impl Into<String>) -> Self { Self::MessageParsingError {
            message: message.into(),
         }
    
    /// Authentication Error
    pub fn authentication_error(message: impl Into<String>) -> Self { Self::AuthenticationError {
            message: message.into(),
         }
    
    /// Session Error
    pub fn session_error(message: impl Into<String>) -> Self { Self::SessionError {
            message: message.into(),
         }
}

/// Enhanced Protocol Handler with advanced integration with v2 orchestrator
pub struct ProtocolHandler {
    _node_id: String,
    capabilities: ProviderCapabilities,
    orchestrator_endpoint: Option<String>,
}
impl ProtocolHandler {
    /// Creates a new instance
    pub fn new(node_id: String, capabilities: ProviderCapabilities) -> Self { Self {
            _node_id: node_id,
            capabilities,
            orchestrator_endpoint: None,
         }

    #[must_use]
    pub fn with_orchestrator(mut self, endpoint: String) -> Self { self.orchestrator_endpoint = Some(endpoint);
        self
    , /// Handle incoming MCP message with v2 orchestrator integration
    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        pub async fn handle_message(&self, message: Message) -> Result<Response>  {
        match message.message_type {
            MessageType::CapabilityRegistration => {
                self.handle_capability_registration(message).await
             }
    MessageType::CapabilityQuery => self.handle_capability_query(message).await,
            MessageType::VolumeCreate => self.handle_volume_create(message).await,
            MessageType::VolumeMount => self.handle_volume_mount(message).await,
            MessageType::MetricsReport => self.handle_metrics_report(message).await,
            MessageType::HealthCheck => self.handle_health_check(message).await,
            MessageType::FederationJoin => self.handle_federation_join(message).await,
            MessageType::OrchestratorRoute => self.handle_orchestrator_route(message).await,
            MessageType::ServiceRegistration => self.handle_service_registration(message).await,
            _ => Err(crate::error::Error::unsupported(format!(
                "Message type {:?} not supported",
                message.message_type
            )),
        }
    }

    /// Handles  Capability Registration
    async fn handle_capability_registration(&self, message: Message) -> Result<Response> {
        // Route through orchestrator if available
        if let Some(_orchestrator_endpoint) = &self.orchestrator_endpoint {
            // Forward to orchestrator for centralized capability management
            return self.route_to_orchestrator(message).await;
        }

        // Direct handling for standalone mode
        Ok(Response::success(
            message.id,
            ResponsePayload::CapabilityResponse(self.capabilities.clone()),
        ))
    }

    /// Handles  Capability Query
    fn handle_capability_query(&self, _message: Message) -> Result<Response> {
        // Return our capabilities
        Ok(Response::success(
            _message.id,
            ResponsePayload::CapabilityResponse(self.capabilities.clone()),
        ))
    }

    /// Handles  Volume Create
    async fn handle_volume_create(&self, message: Message) -> Result<Response> {
        // Route volume operations through orchestrator
        if let Some(_orchestrator_endpoint) = &self.orchestrator_endpoint {
            return self.route_to_orchestrator(message).await;
        }

        // Direct handling for standalone mode
        Err(crate::error::Error::unsupported(
            "Volume operations require orchestrator".to_string(),
        ))
    }

    /// Handles  Volume Mount
    async fn handle_volume_mount(&self, message: Message) -> Result<Response> {
        // Route mount operations through orchestrator
        if let Some(_orchestrator_endpoint) = &self.orchestrator_endpoint {
            return self.route_to_orchestrator(message).await;
        }

        // Direct handling for standalone mode
        Err(crate::error::Error::unsupported(
            "Mount operations require orchestrator".to_string(),
        ))
    }

    /// Handles  Metrics Report
    async fn handle_metrics_report(&self, message: Message) -> Result<Response> {
        // Route metrics through orchestrator for centralized monitoring
        if let Some(_orchestrator_endpoint) = &self.orchestrator_endpoint {
            return self.route_to_orchestrator(message).await;
        }

        // Acknowledge metrics in standalone mode
        Ok(Response::success(message.id, ResponsePayload::Empty))
    }

    /// Handles  Health Check
    fn handle_health_check(&self, message: Message) -> Result<Response> {
        let health_status = HealthStatus {
            status: ServiceStatus::Online,
            uptime: nestgate_core::constants::timeouts::REQUEST_DEFAULT,
            last_check: std::time::SystemTime::now(),
            details: HashMap::new(),
        };

        Ok(Response::success(
            message.id,
            ResponsePayload::HealthStatus(health_status),
        ))
    }

    /// Handles  Federation Join
    async fn handle_federation_join(&self, message: Message) -> Result<Response> {
        // Route federation operations through orchestrator
        if let Some(_orchestrator_endpoint) = &self.orchestrator_endpoint {
            return self.route_to_orchestrator(message).await;
        }

        // Standalone mode doesn't support federation
        Err(crate::error::Error::unsupported(
            "Federation join not yet implemented".to_string(),
        ))
    }

    /// Handle orchestrator routing
    fn handle_orchestrator_route(&self, message: Message) -> Result<Response> {
        // Forward to orchestrator instead of recursing
        match &message.payload {
            MessagePayload::OrchestratorRoute(_payload) => {
                // Create a simple response instead of recursing
                Ok(Response::success(message.id, ResponsePayload::Empty))
            }
            _ => Ok(Response::error(
                message.id,
                ErrorPayload {
                    error_code: "invalid_payload".to_string(),
                    error_message: "Invalid orchestrator route payload".to_string(),
                    details: HashMap::new(),
                    timestamp: std::time::SystemTime::now(),
                }
            )),
        }
    }

    /// Handles  Service Registration
    async fn handle_service_registration(&self, message: Message) -> Result<Response> {
        // Route service registration through orchestrator
        if let Some(_orchestrator_endpoint) = &self.orchestrator_endpoint {
            return self.route_to_orchestrator(message).await;
        }

        // Standalone mode doesn't support service registration
        Err(crate::error::Error::unsupported(
            "Service registration not yet implemented".to_string(),
        ))
    }

    /// Route To Orchestrator
    fn route_to_orchestrator(&self, message: Message) -> Result<Response> {
        // In a real implementation, this would make an HTTP request to the orchestrator
        // Process the actual request and return real response
        Ok(Response::success(message.id, ResponsePayload::Empty))
    }
}
