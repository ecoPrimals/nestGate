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
/// Request parameters for HardwareOptimization operation
pub struct HardwareOptimizationRequest {
    /// Optimization Level
    pub optimization_level: u8, // 1-10 scale
    /// Constraints
    pub constraints: Vec<String>,
    /// Timeout Seconds
    pub timeout_seconds: Option<u64>,
}

/// Hardware optimization response data
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for HardwareOptimization operation
pub struct HardwareOptimizationResponse {
    /// Optimization Applied
    pub optimization_applied: bool,
    /// Performance Gain
    pub performance_gain: f64, // Percentage improvement
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Metrics
    pub metrics: HashMap<String, f64>,
}

/// Resource allocation request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for ResourceAllocation operation
pub struct ResourceAllocationRequest {
    /// Resource Type
    pub resource_type: String,
    /// Requested Amount
    pub requested_amount: u64,
    /// Priority
    pub priority: u8,
    /// Duration Seconds
    pub duration_seconds: Option<u64>,
}

/// Resource allocation response data
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for ResourceAllocation operation
pub struct ResourceAllocationResponse {
    /// Allocated
    pub allocated: bool,
    /// Allocation identifier
    pub allocation_id: String,
    /// Actual Amount
    pub actual_amount: u64,
    /// Expires At
    pub expires_at: Option<String>,
}

/// Performance tuning request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for PerformanceTuning operation
pub struct PerformanceTuningRequest {
    /// Target Service
    pub target_service: String,
    /// Tuning Profile
    pub tuning_profile: String,
    /// Custom Parameters
    pub custom_parameters: HashMap<String, serde_json::Value>,
}

/// Performance tuning response data
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for PerformanceTuning operation
pub struct PerformanceTuningResponse {
    /// Metrics
    pub metrics: HashMap<String, f64>,
    /// Tuning Applied
    pub tuning_applied: bool,
    /// Profile Used
    pub profile_used: String,
    /// Warnings
    pub warnings: Vec<String>,
}

// Note: No mock implementations - use capability discovery system instead.
// See module documentation for migration guide.
