//! Compute Capabilities - Real Implementation via Capability Discovery
//!
//! **REMOVED MOCKS**: This module previously contained MockComputeCapability.
//! **MODERN SOLUTION**: Use `universal_adapter::CapabilityDiscovery` instead.
//!
//! # Migration Guide
//!
//! **Old (Mock)**:
//! ```rust,ignore
//! let mock = MockComputeCapability::new();
//! mock.optimize_hardware(request).await?;
//! ```
//!
//! **New (Real Discovery)**:
//! ```rust,ignore
//! use nestgate_core::universal_adapter::capability_discovery::*;
//!
//! let discovery = CapabilityDiscovery::new();
//! let compute_providers = discovery
//!     .discover(CapabilityType::custom("compute".to_string()))
//!     .await?;
//!
//! if let Some(provider) = compute_providers.first() {
//!     // Use discovered compute primal
//!     provider.call("optimize_hardware", request).await?;
//! }
//! ```
//!
//! This follows the "each primal knows only itself" principle - no hardcoded
//! primal names, pure capability-based discovery.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Hardware optimization request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareOptimizationRequest {
    pub optimization_level: u8, // 1-10 scale
    pub constraints: Vec<String>,
    pub timeout_seconds: Option<u64>,
}

/// Hardware optimization response data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareOptimizationResponse {
    pub optimization_applied: bool,
    pub performance_gain: f64, // Percentage improvement
    pub recommendations: Vec<String>,
    pub metrics: HashMap<String, f64>,
}

/// Resource allocation request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAllocationRequest {
    pub resource_type: String,
    pub requested_amount: u64,
    pub priority: u8,
    pub duration_seconds: Option<u64>,
}

/// Resource allocation response data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAllocationResponse {
    pub allocated: bool,
    pub allocation_id: String,
    pub actual_amount: u64,
    pub expires_at: Option<String>,
}

/// Performance tuning request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTuningRequest {
    pub target_service: String,
    pub tuning_profile: String,
    pub custom_parameters: HashMap<String, serde_json::Value>,
}

/// Performance tuning response data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTuningResponse {
    pub metrics: HashMap<String, f64>,
    pub tuning_applied: bool,
    pub profile_used: String,
    pub warnings: Vec<String>,
}

// Note: No mock implementations - use capability discovery system instead.
// See module documentation for migration guide.
