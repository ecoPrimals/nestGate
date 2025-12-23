//! Orchestration Module - Capability Discovery Integration
//!
//! **ARCHITECTURE NOTE**: Orchestration is the networking layer's domain, not NestGate's.
//! NestGate is a **storage primal** focused on ZFS, datasets, and data management.
//!
//! # Migration from Stubs
//!
//! **Old Approach (DELETED)**:
//! ```rust,ignore
//! // DON'T: Stub implementations that duplicate orchestration layer
//! let orchestrator = MockOrchestrator::new();
//! orchestrator.execute_workflow(workflow).await?;
//! ```
//!
//! **Modern Approach (USE THIS)**:
//! ```rust,ignore
//! use nestgate_core::universal_adapter::capability_discovery::*;
//!
//! // Discover orchestration capability dynamically (any provider)
//! let discovery = CapabilityDiscovery::new();
//! let orchestration_providers = discovery
//!     .discover(CapabilityType::Orchestration)
//!     .await?;
//!
//! if let Some(provider) = orchestration_providers.first() {
//!     // Use discovered primal for orchestration
//!     provider.execute_workflow(workflow).await?;
//! } else {
//!     // Fallback: direct execution (no orchestration)
//!     warn!("No orchestration primal found, executing directly");
//!     execute_directly(workflow).await?;
//! }
//! ```
//!
//! # Why This Architecture?
//!
//! 1. **Sovereignty**: No hardcoded service references
//! 2. **Flexibility**: Works with any orchestration provider
//! 3. **Production Ready**: Real discovery, no stubs
//! 4. **Clean Boundaries**: NestGate does storage, orchestration layer handles workflows
//!
//! # Trait Definitions
//!
//! These traits define the interface for capability discovery.
//! **Do NOT implement these** - discover them via capability system.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Workflow execution request (for capability requests)
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for Workflow operation
pub struct WorkflowRequest {
    /// Workflow identifier
    pub workflow_id: String,
    /// Steps
    pub steps: Vec<WorkflowStep>,
    /// Inputs
    pub inputs: HashMap<String, serde_json::Value>,
    /// Timeout Secs
    pub timeout_secs: Option<u64>,
}

/// Individual workflow step
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Workflowstep
pub struct WorkflowStep {
    /// Step identifier
    pub step_id: String,
    /// Action
    pub action: String,
    /// Parameters
    pub parameters: HashMap<String, serde_json::Value>,
    /// Dependencies
    pub dependencies: Vec<String>,
}

/// Workflow execution status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

/// Workflow execution result (returned by capability provider)
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Workflowresult
pub struct WorkflowResult {
    /// Workflow identifier
    pub workflow_id: String,
    /// Status
    pub status: WorkflowStatus,
    /// Outputs
    pub outputs: HashMap<String, serde_json::Value>,
    /// Error
    pub error: Option<String>,
}

// NOTE: No implementations here - use capability discovery to find orchestration capability
// See module documentation for examples
