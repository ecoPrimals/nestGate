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
pub struct InferenceRequest {
    pub model_name: String,
    pub input_data: serde_json::Value,
    pub parameters: HashMap<String, serde_json::Value>,
    pub timeout_seconds: Option<u64>,
}

/// AI inference response data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceResponse {
    pub result: serde_json::Value,
    pub confidence: Option<f64>,
    pub latency_ms: u64,
    pub model_version: String,
}

/// Model training request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingRequest {
    pub model_type: String,
    pub training_data_path: String,
    pub hyperparameters: HashMap<String, serde_json::Value>,
    pub validation_split: Option<f64>,
}

/// Model training response data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingResponse {
    pub model_id: String,
    pub accuracy: f64,
    pub training_time_seconds: u64,
    pub metrics: HashMap<String, f64>,
}

// Note: No mock implementations - use capability discovery system instead.
// See module documentation for migration guide.
