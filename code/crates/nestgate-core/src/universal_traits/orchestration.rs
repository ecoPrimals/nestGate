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
pub struct WorkflowSpec {
    pub name: String,
    pub description: String,
    pub steps: Vec<WorkflowStep>,
    pub triggers: Vec<WorkflowTrigger>,
    pub timeout_seconds: Option<u64>,
    pub retry_policy: Option<RetryPolicy>,
}
/// Individual workflow step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub name: String,
    pub service: String,
    pub action: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub depends_on: Vec<String>,
    pub timeout_seconds: Option<u64>,
}
/// Workflow trigger definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTrigger {
    pub trigger_type: TriggerType,
    pub condition: String,
    pub parameters: HashMap<String, String>,
}
/// Types of workflow triggers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TriggerType {
    Schedule,
    Event,
    Webhook,
    FileChange,
    ServiceStatus,
}
/// Retry policy for failed operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub initial_delay_seconds: u64,
    pub max_delay_seconds: u64,
    pub backoff_multiplier: f64,
}
/// Workflow execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowResult {
    pub workflow_id: String,
    pub status: WorkflowStatus,
    pub started_at: std::time::SystemTime,
    pub completed_at: Option<std::time::SystemTime>,
    pub steps_completed: Vec<String>,
    pub error_message: Option<String>,
    pub outputs: HashMap<String, serde_json::Value>,
}
/// Workflow execution status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
    Retrying,
}
/// Workflow information summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowInfo {
    pub id: String,
    pub name: String,
    pub status: WorkflowStatus,
    pub created_at: std::time::SystemTime,
    pub last_updated: std::time::SystemTime,
}
/// Service specification for registration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceSpec {
    pub name: String,
    pub version: String,
    pub capabilities: Vec<String>,
    pub endpoints: HashMap<String, String>,
    pub health_check_endpoint: Option<String>,
    pub metadata: HashMap<String, String>,
}
/// Service information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub capabilities: Vec<String>,
    pub status: ServiceStatus,
    pub last_seen: std::time::SystemTime,
}
/// Service status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}
/// Service request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRequest {
    pub service_id: String,
    pub action: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub timeout_seconds: Option<u64>,
}
/// Service response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceResponse {
    pub success: bool,
    pub data: Option<serde_json::Value>,
    pub error_message: Option<String>,
    pub execution_time_ms: u64,
}
/// Service health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealth {
    pub status: ServiceStatus,
    pub response_time_ms: u64,
    pub last_check: std::time::SystemTime,
    pub details: HashMap<String, String>,
}
/// Service scaling result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingResult {
    pub service_id: String,
    pub previous_instances: u32,
    pub target_instances: u32,
    pub actual_instances: u32,
    pub scaling_time_ms: u64,
}
/// Service dependency graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyGraph {
    pub nodes: Vec<String>,
    pub edges: Vec<DependencyEdge>,
    pub cycles: Vec<Vec<String>>,
}
/// Dependency relationship between services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyEdge {
    pub from: String,
    pub to: String,
    pub dependency_type: DependencyType,
}
/// Types of service dependencies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencyType {
    Required,
    Optional,
    Conditional,
}
/// Workflow validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub estimated_duration_seconds: Option<u64>,
}
