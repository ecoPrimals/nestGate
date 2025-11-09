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
pub struct WorkflowRequest {
    pub workflow_definition: serde_json::Value,
    pub inputs: HashMap<String, serde_json::Value>,
    pub execution_options: ExecutionOptions,
}

/// Execution options for workflows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionOptions {
    pub timeout_seconds: Option<u64>,
    pub retry_policy: Option<RetryPolicy>,
    pub parallelism: Option<u32>,
}

/// Retry policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub backoff_multiplier: f64,
    pub initial_delay_ms: u64,
}

/// Workflow orchestration response data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowResponse {
    pub workflow_id: String,
    pub status: WorkflowStatus,
    pub outputs: HashMap<String, serde_json::Value>,
    pub execution_time_ms: u64,
}

/// Workflow execution status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkflowStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Service coordination request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationRequest {
    pub services: Vec<String>,
    pub coordination_type: CoordinationType,
    pub constraints: HashMap<String, serde_json::Value>,
}

/// Coordination type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoordinationType {
    Sequential,
    Parallel,
    Conditional,
    EventDriven,
}

/// Service coordination response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationResponse {
    pub coordination_id: String,
    pub services_coordinated: Vec<String>,
    pub execution_plan: serde_json::Value,
}

// Note: No mock implementations - use capability discovery system instead.
// See module documentation for migration guide.
