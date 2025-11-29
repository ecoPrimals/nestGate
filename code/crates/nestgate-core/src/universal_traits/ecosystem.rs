// **ECOSYSTEM INTEGRATION - CANONICAL MODERNIZED**
//! Ecosystem trait definitions for universal providers
// Ecosystem integration and service discovery traits for universal primal coordination.
// Native async traits without async_trait overhead for optimal performance.

use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Type alias to reduce complexity
type ConnectivityMatrix = HashMap<String, HashMap<String, bool>>;

/// Ecosystem integration trait for cross-primal coordination
pub trait EcosystemIntegration: Send + Sync {
    /// Register this primal with the ecosystem
    fn register_primal(
        &self,
        primal_info: &PrimalInfo,
    ) -> impl std::future::Future<Output = Result<String>> + Send;
    /// Discover other primals in the ecosystem
    fn discover_primals(
        &self,
        capability_filter: Option<&str>,
    ) -> impl std::future::Future<Output = Result<Vec<PrimalInfo>>> + Send;

    /// Send inter-primal request
    fn send_inter_primal_request(
        &self,
        target_primal: &str,
        request: &InterPrimalRequest,
    ) -> impl std::future::Future<Output = Result<InterPrimalResponse>> + Send;

    /// Handle incoming inter-primal request
    fn handle_inter_primal_request(
        &self,
        request: &InterPrimalRequest,
    ) -> impl std::future::Future<Output = Result<InterPrimalResponse>> + Send;

    /// Get ecosystem health status
    fn get_ecosystem_health(
        &self,
    ) -> impl std::future::Future<Output = Result<EcosystemHealth>> + Send;

    /// Coordinate distributed operation
    fn coordinate_operation(
        &self,
    ) -> impl std::future::Future<Output = Result<OperationResult>> + Send;
}

/// Universal primal provider trait
pub trait UniversalPrimalProvider: Send + Sync {
    /// Get primal identification
    fn get_primal_id(&self) -> &str;
    /// Get primal capabilities
    fn get_capabilities(&self) -> Vec<String>;

    /// Process generic primal request
    fn process_request(
        &self,
        request: &PrimalRequest,
    ) -> impl std::future::Future<Output = Result<PrimalResponse>> + Send;

    /// Get primal health status
    fn get_health(&self) -> impl std::future::Future<Output = Result<PrimalHealth>> + Send;

    /// Get primal metrics
    fn get_metrics(&self) -> impl std::future::Future<Output = Result<PrimalMetrics>> + Send;

    /// Shutdown primal gracefully
    fn shutdown(&self) -> impl std::future::Future<Output = Result<()>> + Send;
}

/// Primal information for ecosystem registration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Primalinfo
pub struct PrimalInfo {
    /// Unique identifier
    pub id: String,
    /// Name
    pub name: String,
    /// Version
    pub version: String,
    /// Primal Type
    pub primal_type: PrimalType,
    /// Capabilities
    pub capabilities: Vec<String>,
    /// Endpoints
    pub endpoints: HashMap<String, String>,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}
/// Types of primals in the ecosystem
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of Primal
pub enum PrimalType {
    /// Storage
    Storage,
    /// Compute
    Compute,
    /// Security
    Security,
    /// Orchestration
    Orchestration,
    /// Intelligence
    Intelligence,
    /// Network
    Network,
    /// Monitoring
    Monitoring,
    Custom(String),
}
/// Inter-primal request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for InterPrimal operation
pub struct InterPrimalRequest {
    /// Request identifier
    pub request_id: String,
    /// Source Primal
    pub source_primal: String,
    /// Target Capability
    pub target_capability: String,
    /// Parameters
    pub parameters: HashMap<String, serde_json::Value>,
    /// Timeout Seconds
    pub timeout_seconds: Option<u64>,
}
/// Inter-primal response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for InterPrimal operation
pub struct InterPrimalResponse {
    /// Request identifier
    pub request_id: String,
    /// Success
    pub success: bool,
    /// Data
    pub data: Option<serde_json::Value>,
    /// Error Message
    pub error_message: Option<String>,
    /// Execution Time Ms
    pub execution_time_ms: u64,
}
/// Generic primal request
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for Primal operation
pub struct PrimalRequest {
    /// Parameters
    pub parameters: HashMap<String, serde_json::Value>,
    /// Context
    pub context: Option<PrimalContext>,
}
/// Generic primal response
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for Primal operation
pub struct PrimalResponse {
    /// Success
    pub success: bool,
    /// Data
    pub data: Option<serde_json::Value>,
    /// Error Message
    pub error_message: Option<String>,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}
/// Primal execution context
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Primalcontext
pub struct PrimalContext {
    /// Request identifier
    pub request_id: String,
    /// User identifier
    pub user_id: Option<String>,
    /// Session identifier
    pub session_id: Option<String>,
    /// Trace identifier
    pub trace_id: Option<String>,
    /// Timestamp
    pub timestamp: std::time::SystemTime,
}
/// Primal health status
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Primalhealth
pub enum PrimalHealth {
    /// Healthy
    Healthy,
    /// Degraded
    Degraded,
    /// Unhealthy
    Unhealthy,
    /// Maintenance
    Maintenance,
}
/// Primal performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Primalmetrics
pub struct PrimalMetrics {
    /// Requests Per Second
    pub requests_per_second: f64,
    /// Average Response Time Ms
    pub average_response_time_ms: f64,
    /// Error Rate Percent
    pub error_rate_percent: f64,
    /// Resource Utilization
    pub resource_utilization: HashMap<String, f64>,
    /// Timestamp
    pub timestamp: std::time::SystemTime,
}
/// Ecosystem health status
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Ecosystemhealth
pub struct EcosystemHealth {
    /// Overall Status
    pub overall_status: EcosystemStatus,
    /// Primal Statuses
    pub primal_statuses: HashMap<String, PrimalHealth>,
    /// Connectivity Matrix
    pub connectivity_matrix: ConnectivityMatrix,
    /// Last Updated
    pub last_updated: std::time::SystemTime,
}
/// Overall ecosystem status
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Status values for Ecosystem
pub enum EcosystemStatus {
    /// Healthy
    Healthy,
    /// Degraded
    Degraded,
    /// Critical
    Critical,
    /// Partitioned
    Partitioned,
}
/// Distributed operation specification
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Distributedoperation
pub struct DistributedOperation {
    /// Operation identifier
    pub operation_id: String,
    /// Operation Type
    pub operation_type: String,
    /// Participants
    pub participants: Vec<String>,
    /// Coordination Strategy
    pub coordination_strategy: CoordinationStrategy,
    /// Parameters
    pub parameters: HashMap<String, serde_json::Value>,
    /// Timeout Seconds
    pub timeout_seconds: u64,
}
/// Coordination strategies for distributed operations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Coordinationstrategy
pub enum CoordinationStrategy {
    /// Sequential
    Sequential,
    /// Parallel
    Parallel,
    /// Twophasecommit
    TwoPhaseCommit,
    /// Consensus
    Consensus,
    /// Eventualconsistency
    EventualConsistency,
}
/// Result of distributed operation
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Operationresult
pub struct OperationResult {
    /// Operation identifier
    pub operation_id: String,
    /// Success
    pub success: bool,
    /// Participant Results
    pub participant_results: HashMap<String, ParticipantResult>,
    /// Overall Result
    pub overall_result: Option<serde_json::Value>,
    /// Execution Time Ms
    pub execution_time_ms: u64,
}
/// Individual participant result in distributed operation
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Participantresult
pub struct ParticipantResult {
    /// Participant identifier
    pub participant_id: String,
    /// Success
    pub success: bool,
    /// Result
    pub result: Option<serde_json::Value>,
    /// Error Message
    pub error_message: Option<String>,
    /// Execution Time Ms
    pub execution_time_ms: u64,
}
