//! Orchestration Capabilities - Real Implementation via Capability Discovery
//!
//! **REMOVED MOCKS**: This module previously contained MockOrchestrationCapability.
//! **MODERN SOLUTION**: Use `universal_adapter::CapabilityDiscovery` instead.
//!
//! # Migration Guide
//!
//! **Old (Mock)**:
//! ```rust,ignore
//! let mock = MockOrchestrationCapability::new();
//! mock.orchestrate_workflow(request).await?;
//! ```
//!
//! **New (Real Discovery)**:
//! ```rust,ignore
//! use nestgate_core::universal_adapter::capability_discovery::*;
//!
//! let discovery = CapabilityDiscovery::new();
//! let orchestration_providers = discovery
//!     .discover(CapabilityType::custom("orchestration".to_string()))
//!     .await?;
//!
//! if let Some(provider) = orchestration_providers.first() {
//!     // Use discovered orchestration primal (e.g., Songbird)
//!     provider.call("orchestrate_workflow", request).await?;
//! }
//! ```
//!
//! No hardcoded "Songbird" or other primal names - pure capability discovery.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Workflow orchestration request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for Workflow operation
pub struct WorkflowRequest {
    /// Workflow Definition
    pub workflow_definition: serde_json::Value,
    /// Inputs
    pub inputs: HashMap<String, serde_json::Value>,
    /// Execution Options
    pub execution_options: ExecutionOptions,
}

/// Execution options for workflows
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Executionoptions
pub struct ExecutionOptions {
    /// Timeout Seconds
    pub timeout_seconds: Option<u64>,
    /// Retry Policy
    pub retry_policy: Option<RetryPolicy>,
    /// Parallelism
    pub parallelism: Option<u32>,
}

/// Retry policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Retrypolicy
pub struct RetryPolicy {
    /// Max Attempts
    pub max_attempts: u32,
    /// Backoff Multiplier
    pub backoff_multiplier: f64,
    /// Initial Delay Ms
    pub initial_delay_ms: u64,
}

/// Workflow orchestration response data
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for Workflow operation
pub struct WorkflowResponse {
    /// Workflow identifier
    pub workflow_id: String,
    /// Status
    pub status: WorkflowStatus,
    /// Outputs
    pub outputs: HashMap<String, serde_json::Value>,
    /// Execution Time Ms
    pub execution_time_ms: u64,
}

/// Workflow execution status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
}

/// Service coordination request
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for Coordination operation
pub struct CoordinationRequest {
    /// Services
    pub services: Vec<String>,
    /// Coordination Type
    pub coordination_type: CoordinationType,
    /// Constraints
    pub constraints: HashMap<String, serde_json::Value>,
}

/// Coordination type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of Coordination
pub enum CoordinationType {
    /// Sequential
    Sequential,
    /// Parallel
    Parallel,
    /// Conditional
    Conditional,
    /// Eventdriven
    EventDriven,
}

/// Service coordination response
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for Coordination operation
pub struct CoordinationResponse {
    /// Coordination identifier
    pub coordination_id: String,
    /// Services Coordinated
    pub services_coordinated: Vec<String>,
    /// Execution Plan
    pub execution_plan: serde_json::Value,
}

// Note: No mock implementations - use capability discovery system instead.
// See module documentation for migration guide.
