//! Intelligence/AI Capabilities - Real Implementation via Capability Discovery
//!
//! **REMOVED MOCKS**: This module previously contained MockIntelligenceCapability.
//! **MODERN SOLUTION**: Use `universal_adapter::CapabilityDiscovery` instead.
//!
//! # Migration Guide
//!
//! **Old (Mock)**:
//! ```rust,ignore
//! let mock = MockIntelligenceCapability::new();
//! mock.infer(request).await?;
//! ```
//!
//! **New (Real Discovery)**:
//! ```rust,ignore
//! use nestgate_core::universal_adapter::capability_discovery::*;
//!
//! let discovery = CapabilityDiscovery::new();
//! let ai_providers = discovery
//!     .discover(CapabilityType::custom("ai".to_string()))
//!     .await?;
//!
//! if let Some(provider) = ai_providers.first() {
//!     // Use discovered AI primal
//!     provider.call("infer", request).await?;
//! }
//! ```
//!
//! Follows sovereignty principle - no hardcoded primal names.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// AI inference request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for Inference operation
pub struct InferenceRequest {
    /// Model name
    pub model_name: String,
    /// Input Data
    pub input_data: serde_json::Value,
    /// Parameters
    pub parameters: HashMap<String, serde_json::Value>,
    /// Timeout Seconds
    pub timeout_seconds: Option<u64>,
}

/// AI inference response data
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for Inference operation
pub struct InferenceResponse {
    /// Result
    pub result: serde_json::Value,
    /// Confidence
    pub confidence: Option<f64>,
    /// Latency Ms
    pub latency_ms: u64,
    /// Model Version
    pub model_version: String,
}

/// Model training request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for Training operation
pub struct TrainingRequest {
    /// Model Type
    pub model_type: String,
    /// Training Data Path
    pub training_data_path: String,
    /// Hyperparameters
    pub hyperparameters: HashMap<String, serde_json::Value>,
    /// Validation Split
    pub validation_split: Option<f64>,
}

/// Model training response data
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for Training operation
pub struct TrainingResponse {
    /// Model identifier
    pub model_id: String,
    /// Accuracy
    pub accuracy: f64,
    /// Training Time Seconds
    pub training_time_seconds: u64,
    /// Metrics
    pub metrics: HashMap<String, f64>,
}

// Note: No mock implementations - use capability discovery system instead.
// See module documentation for migration guide.
