// **ORCHESTRATION TRAITS - CANONICAL MODERNIZED**
//! Orchestration trait definitions for universal providers
// Orchestration and workflow management traits for universal primal integration.
// Native async traits without async_trait overhead for optimal performance.

use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Universal orchestration primal provider trait
/// **CANONICAL MODERNIZATION**: Native async trait without `async_trait` overhead
pub trait OrchestrationPrimalProvider: Send + Sync {
    /// Schedule a workflow execution
    fn schedule_workflow(
        &self,
        workflow_spec: &WorkflowSpec,
    ) -> impl std::future::Future<Output = Result<String>> + Send;
    /// Execute workflow immediately
    fn execute_workflow(
        &self,
        workflow_id: &str,
    ) -> impl std::future::Future<Output = Result<WorkflowResult>> + Send;

    /// Get workflow status
    fn get_workflow_status(
        &self,
        workflow_id: &str,
    ) -> impl std::future::Future<Output = Result<WorkflowStatus>> + Send;

    /// Cancel running workflow
    fn cancel_workflow(
        &self,
        workflow_id: &str,
    ) -> impl std::future::Future<Output = Result<()>> + Send;

    /// List active workflows
    fn list_workflows(&self)
        -> impl std::future::Future<Output = Result<Vec<WorkflowInfo>>> + Send;

    /// Register service for orchestration
    fn register_service(
        &self,
        service_spec: &ServiceSpec,
    ) -> impl std::future::Future<Output = Result<String>> + Send;

    /// Discover available services
    fn discover_services(
        &self,
        capability: &str,
    ) -> impl std::future::Future<Output = Result<Vec<ServiceInfo>>> + Send;

    /// Route request to appropriate service
    fn route_request(
        &self,
        request: &ServiceRequest,
    ) -> impl std::future::Future<Output = Result<ServiceResponse>> + Send;

    /// Monitor service health
    fn monitor_service_health(
        &self,
        _service_id: &str,
    ) -> impl std::future::Future<Output = Result<ServiceHealth>> + Send;

    /// Scale service instances
    fn scale_service(
        &self,
        _service_id: &str,
        target_instances: u32,
    ) -> impl std::future::Future<Output = Result<ScalingResult>> + Send;

    /// Create service dependency graph
    fn create_dependency_graph(
        &self,
        services: &[String],
    ) -> impl std::future::Future<Output = Result<DependencyGraph>> + Send;

    /// Validate workflow definition
    fn validate_workflow(
        &self,
        workflow_spec: &WorkflowSpec,
    ) -> impl std::future::Future<Output = Result<ValidationResult>> + Send;
}

/// Workflow specification
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Workflowspec
pub struct WorkflowSpec {
    /// Name
    pub name: String,
    /// Human-readable description
    pub description: String,
    /// Steps
    pub steps: Vec<WorkflowStep>,
    /// Triggers
    pub triggers: Vec<WorkflowTrigger>,
    /// Timeout Seconds
    pub timeout_seconds: Option<u64>,
    /// Retry Policy
    pub retry_policy: Option<RetryPolicy>,
}
/// Individual workflow step
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Workflowstep
pub struct WorkflowStep {
    /// Name
    pub name: String,
    /// Service
    pub service: String,
    /// Action
    pub action: String,
    /// Parameters
    pub parameters: HashMap<String, serde_json::Value>,
    /// Depends On
    pub depends_on: Vec<String>,
    /// Timeout Seconds
    pub timeout_seconds: Option<u64>,
}
/// Workflow trigger definition
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Workflowtrigger
pub struct WorkflowTrigger {
    /// Trigger Type
    pub trigger_type: TriggerType,
    /// Condition
    pub condition: String,
    /// Parameters
    pub parameters: HashMap<String, String>,
}
/// Types of workflow triggers
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of Trigger
pub enum TriggerType {
    /// Schedule
    Schedule,
    /// Event
    Event,
    /// Webhook
    Webhook,
    /// Filechange
    FileChange,
    /// Servicestatus
    ServiceStatus,
}
/// Retry policy for failed operations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Retrypolicy
pub struct RetryPolicy {
    /// Max Attempts
    pub max_attempts: u32,
    /// Initial Delay Seconds
    pub initial_delay_seconds: u64,
    /// Max Delay Seconds
    pub max_delay_seconds: u64,
    /// Backoff Multiplier
    pub backoff_multiplier: f64,
}
/// Workflow execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Workflowresult
pub struct WorkflowResult {
    /// Workflow identifier
    pub workflow_id: String,
    /// Status
    pub status: WorkflowStatus,
    /// Started At
    pub started_at: std::time::SystemTime,
    /// Completed At
    pub completed_at: Option<std::time::SystemTime>,
    /// Steps Completed
    pub steps_completed: Vec<String>,
    /// Error Message
    pub error_message: Option<String>,
    /// Outputs
    pub outputs: HashMap<String, serde_json::Value>,
}
/// Workflow execution status
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Status values for Workflow
pub enum WorkflowStatus {
    /// Pending
    Pending,
    /// Running
    Running,
    /// Completed
    Completed,
    /// Failed
    Failed,
    /// Cancelled
    Cancelled,
    /// Retrying
    Retrying,
}
/// Workflow information summary
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Workflowinfo
pub struct WorkflowInfo {
    /// Unique identifier
    pub id: String,
    /// Name
    pub name: String,
    /// Status
    pub status: WorkflowStatus,
    /// Timestamp when this was created
    pub created_at: std::time::SystemTime,
    /// Last Updated
    pub last_updated: std::time::SystemTime,
}
/// Service specification for registration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Servicespec
pub struct ServiceSpec {
    /// Name
    pub name: String,
    /// Version
    pub version: String,
    /// Capabilities
    pub capabilities: Vec<String>,
    /// Endpoints
    pub endpoints: HashMap<String, String>,
    /// Health Check Endpoint
    pub health_check_endpoint: Option<String>,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}
/// Service information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Serviceinfo
pub struct ServiceInfo {
    /// Unique identifier
    pub id: String,
    /// Name
    pub name: String,
    /// Version
    pub version: String,
    /// Capabilities
    pub capabilities: Vec<String>,
    /// Status
    pub status: ServiceStatus,
    /// Last Seen
    pub last_seen: std::time::SystemTime,
}
/// Service status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Status values for Service
pub enum ServiceStatus {
    /// Healthy
    Healthy,
    /// Degraded
    Degraded,
    /// Unhealthy
    Unhealthy,
    /// Unknown
    Unknown,
}
/// Service request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for Service operation
pub struct ServiceRequest {
    /// Service identifier
    pub service_id: String,
    /// Action
    pub action: String,
    /// Parameters
    pub parameters: HashMap<String, serde_json::Value>,
    /// Timeout Seconds
    pub timeout_seconds: Option<u64>,
}
/// Service response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for Service operation
pub struct ServiceResponse {
    /// Success
    pub success: bool,
    /// Data
    pub data: Option<serde_json::Value>,
    /// Error Message
    pub error_message: Option<String>,
    /// Execution Time Ms
    pub execution_time_ms: u64,
}
/// Service health information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Servicehealth
pub struct ServiceHealth {
    /// Status
    pub status: ServiceStatus,
    /// Response Time Ms
    pub response_time_ms: u64,
    /// Last Check
    pub last_check: std::time::SystemTime,
    /// Details
    pub details: HashMap<String, String>,
}
/// Service scaling result
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Scalingresult
pub struct ScalingResult {
    /// Service identifier
    pub service_id: String,
    /// Previous Instances
    pub previous_instances: u32,
    /// Target Instances
    pub target_instances: u32,
    /// Actual Instances
    pub actual_instances: u32,
    /// Scaling Time Ms
    pub scaling_time_ms: u64,
}
/// Service dependency graph
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Dependencygraph
pub struct DependencyGraph {
    /// Nodes
    pub nodes: Vec<String>,
    /// Edges
    pub edges: Vec<DependencyEdge>,
    /// Cycles
    pub cycles: Vec<Vec<String>>,
}
/// Dependency relationship between services
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Dependencyedge
pub struct DependencyEdge {
    /// From
    pub from: String,
    /// To
    pub to: String,
    /// Dependency Type
    pub dependency_type: DependencyType,
}
/// Types of service dependencies
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of Dependency
pub enum DependencyType {
    /// Required
    Required,
    /// Optional
    Optional,
    /// Conditional
    Conditional,
}
/// Workflow validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Validationresult
pub struct ValidationResult {
    /// Valid
    pub valid: bool,
    /// Errors
    pub errors: Vec<String>,
    /// Warnings
    pub warnings: Vec<String>,
    /// Estimated Duration Seconds
    pub estimated_duration_seconds: Option<u64>,
}
