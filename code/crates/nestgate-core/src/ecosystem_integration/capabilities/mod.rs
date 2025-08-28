/// External Primal Capability Interfaces
///
/// This module defines the capability interfaces for integrating with external primals
/// through the Universal Adapter pattern, eliminating hardcoded dependencies.
pub mod compute; // Toadstool compute capabilities
pub mod intelligence;
pub mod orchestration; // Songbird orchestration capabilities
pub mod security; // BearDog security capabilities // Squirrel AI capabilities

// Re-export commonly used capability types
pub use compute::*;
pub use intelligence::*;
pub use orchestration::*;
pub use security::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Base capability request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRequest {
    pub capability_id: String,
    pub parameters: serde_json::Value,
    pub metadata: HashMap<String, String>,
}

impl CapabilityRequest {
    pub fn new(capability_id: impl Into<String>, parameters: serde_json::Value) -> Self {
        Self {
            capability_id: capability_id.into(),
            parameters,
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// Base capability response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityResponse {
    pub success: bool,
    pub data: serde_json::Value,
    pub metadata: HashMap<String, String>,
    pub error_message: Option<String>,
}

impl CapabilityResponse {
    pub fn success(data: serde_json::Value) -> Self {
        Self {
            success: true,
            data,
            metadata: HashMap::new(),
            error_message: None,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: serde_json::Value::Null,
            metadata: HashMap::new(),
            error_message: Some(message.into()),
        }
    }
}

/// Universal capability trait that all external primal adapters must implement
pub trait UniversalCapability: Send + Sync {
    /// Execute a capability request
    async fn execute(
        &self,
        request: CapabilityRequest,
    ) -> Result<CapabilityResponse, Box<dyn std::error::Error + Send + Sync>>;

    /// Get capability metadata and supported operations
    fn get_metadata(&self) -> HashMap<String, serde_json::Value>;

    /// Health check for the capability
    fn health_check(&self) -> impl std::future::Future<Output = bool> + Send;
}
