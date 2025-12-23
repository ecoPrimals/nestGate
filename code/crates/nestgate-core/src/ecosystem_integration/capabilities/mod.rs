// External Primal Capability Interfaces
//
// This module defines the capability interfaces for integrating with external primals
// through the Universal Adapter pattern, eliminating hardcoded dependencies.
//! Capabilities module

pub mod compute; // Compute compute capabilities
pub mod intelligence;
pub mod orchestration; // Orchestration orchestration capabilities
pub mod security; // Security security capabilities // Intelligence AI capabilities
// Re-export commonly used capability types
pub use compute::*;
pub use intelligence::*;
pub use orchestration::*;
pub use security::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Base capability request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for Capability operation
pub struct CapabilityRequest {
    /// Capability identifier
    pub capability_id: String,
    /// Parameters
    pub parameters: serde_json::Value,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}
impl CapabilityRequest {
    #[must_use]
    pub fn new(capability_id: impl Into<String>, parameters: serde_json::Value) -> Self {
        Self {
            capability_id: capability_id.into(),
            parameters,
            metadata: HashMap::new(),
        }
    }

    #[must_use]
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

// Base capability response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for Capability operation
pub struct CapabilityResponse {
    /// Success
    pub success: bool,
    /// Data
    pub data: serde_json::Value,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
    /// Error Message
    pub error_message: Option<String>,
}
impl CapabilityResponse {
    #[must_use]
    pub fn success(data: serde_json::Value) -> Self {
        Self {
            success: true,
            data,
            metadata: HashMap::new(),
            error_message: None,
        }
    }

    #[must_use]
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: serde_json::Value::Null,
            metadata: HashMap::new(),
            error_message: Some(message.into()),
        }
    }
}

// Universal capability trait that all external primal adapters must implement
// **MODERNIZED**: Native async patterns for zero-cost abstractions
pub trait UniversalCapability: Send + Sync {
    /// Execute a capability request - native async, no Future boxing
    fn execute(
        &self,
        request: CapabilityRequest,
    ) -> impl std::future::Future<Output = Result<CapabilityResponse, Box<dyn std::error::Error + Send + Sync>>> + Send;
    /// Get capability metadata and supported operations
    fn get_metadata(&self) -> HashMap<String, serde_json::Value>;

    /// Health check for the capability - native async
    fn health_check(&self) -> impl std::future::Future<Output = bool> + Send;
}
