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
pub struct PrimalInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub primal_type: PrimalType,
    pub capabilities: Vec<String>,
    pub endpoints: HashMap<String, String>,
    pub metadata: HashMap<String, String>,
}
/// Types of primals in the ecosystem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrimalType {
    Storage,
    Compute,
    Security,
    Orchestration,
    Intelligence,
    Network,
    Monitoring,
    Custom(String),
}
/// Inter-primal request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterPrimalRequest {
    pub request_id: String,
    pub source_primal: String,
    pub target_capability: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub timeout_seconds: Option<u64>,
}
/// Inter-primal response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterPrimalResponse {
    pub request_id: String,
    pub success: bool,
    pub data: Option<serde_json::Value>,
    pub error_message: Option<String>,
    pub execution_time_ms: u64,
}
/// Generic primal request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalRequest {
    pub parameters: HashMap<String, serde_json::Value>,
    pub context: Option<PrimalContext>,
}
/// Generic primal response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalResponse {
    pub success: bool,
    pub data: Option<serde_json::Value>,
    pub error_message: Option<String>,
    pub metadata: HashMap<String, String>,
}
/// Primal execution context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalContext {
    pub request_id: String,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub trace_id: Option<String>,
    pub timestamp: std::time::SystemTime,
}
/// Primal health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrimalHealth {
    Healthy,
    Degraded,
    Unhealthy,
    Maintenance,
}
/// Primal performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalMetrics {
    pub requests_per_second: f64,
    pub average_response_time_ms: f64,
    pub error_rate_percent: f64,
    pub resource_utilization: HashMap<String, f64>,
    pub timestamp: std::time::SystemTime,
}
/// Ecosystem health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemHealth {
    pub overall_status: EcosystemStatus,
    pub primal_statuses: HashMap<String, PrimalHealth>,
    pub connectivity_matrix: ConnectivityMatrix,
    pub last_updated: std::time::SystemTime,
}
/// Overall ecosystem status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EcosystemStatus {
    Healthy,
    Degraded,
    Critical,
    Partitioned,
}
/// Distributed operation specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedOperation {
    pub operation_id: String,
    pub operation_type: String,
    pub participants: Vec<String>,
    pub coordination_strategy: CoordinationStrategy,
    pub parameters: HashMap<String, serde_json::Value>,
    pub timeout_seconds: u64,
}
/// Coordination strategies for distributed operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoordinationStrategy {
    Sequential,
    Parallel,
    TwoPhaseCommit,
    Consensus,
    EventualConsistency,
}
/// Result of distributed operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationResult {
    pub operation_id: String,
    pub success: bool,
    pub participant_results: HashMap<String, ParticipantResult>,
    pub overall_result: Option<serde_json::Value>,
    pub execution_time_ms: u64,
}
/// Individual participant result in distributed operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantResult {
    pub participant_id: String,
    pub success: bool,
    pub result: Option<serde_json::Value>,
    pub error_message: Option<String>,
    pub execution_time_ms: u64,
}
