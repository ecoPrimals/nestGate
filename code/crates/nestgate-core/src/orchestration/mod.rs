//! Orchestration Module - Capability Discovery Integration
//!
//! **ARCHITECTURE NOTE**: Orchestration is Songbird's domain, not NestGate's.
//! NestGate is a **storage primal** focused on ZFS, datasets, and data management.
//!
//! # Migration from Stubs
//!
//! **Old Approach (DELETED)**:
//! ```rust,ignore
//! // DON'T: Stub implementations that duplicate Songbird
//! let orchestrator = MockOrchestrator::new();
//! orchestrator.execute_workflow(workflow).await?;
//! ```
//!
//! **Modern Approach (USE THIS)**:
//! ```rust,ignore
//! use nestgate_core::universal_adapter::capability_discovery::*;
//!
//! // Discover orchestration primal (Songbird) dynamically  
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
//! 1. **Sovereignty**: No hardcoded "Songbird" references
//! 2. **Flexibility**: Works with any orchestration primal
//! 3. **Production Ready**: Real discovery, no stubs
//! 4. **Clean Boundaries**: NestGate does storage, Songbird does orchestration
//!
//! # Trait Definitions
//!
//! These traits define the interface for capability discovery.
//! **Do NOT implement these** - discover them via capability system.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Workflow execution request (for capability requests)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRequest {
    pub workflow_id: String,
    pub steps: Vec<WorkflowStep>,
    pub inputs: HashMap<String, serde_json::Value>,
    pub timeout_secs: Option<u64>,
}

/// Individual workflow step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub step_id: String,
    pub action: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub dependencies: Vec<String>,
}

/// Workflow execution status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkflowStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Workflow execution result (returned by capability provider)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowResult {
    pub workflow_id: String,
    pub status: WorkflowStatus,
    pub outputs: HashMap<String, serde_json::Value>,
    pub error: Option<String>,
}

// NOTE: No implementations here - use capability discovery to find Songbird
// See module documentation for examples
