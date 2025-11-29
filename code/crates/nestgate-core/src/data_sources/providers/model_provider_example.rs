// Universal Data Provider Example
//! Model Provider Example functionality and utilities.
// This module demonstrates how external systems can implement data capabilities
//! for NestGate without NestGate being coupled to specific providers.
//! Model Provider Example functionality and utilities.
// **ARCHITECTURAL PRINCIPLE**: NestGate defines what it needs (data capabilities),
//! external providers implement how they provide it.

use serde_json::Value;
use std::collections::HashMap;
use crate::Result;

/// Example of how external systems can provide model data to NestGate
/// This would be implemented by AI primals like Intelligence, not by NestGate itself
#[derive(Debug, Clone)]
/// Externalmodeldataprovider
pub struct ExternalModelDataProvider {
    /// Provider name
    pub provider_name: String,
    /// Endpoint
    pub endpoint: String,
    /// Capabilities
    pub capabilities: Vec<String>,
}
impl ExternalModelDataProvider {
    /// Creates a new instance
    pub fn new(provider_name: String, endpoint: String) -> Self {
        Self {
            provider_name,
            endpoint,
            capabilities: vec![
                "model_metadata".to_string(),
                "model_discovery".to_string(),
                "capability_query".to_string(),
            ],
        }
    }
}

/// Data capability trait that external providers implement
pub trait DataCapability: Send + Sync {
    /// Handles  Request
    fn handle_request(&self, request: &DataRequest) -> impl std::future::Future<Output = Result<DataResponse>> + Send;
    /// Supported Capabilities
    fn supported_capabilities(&self) -> Vec<String>;
    /// Provider Info
    fn provider_info(&self) -> ProviderInfo;
}
#[derive(Debug, Clone)]
/// Request parameters for Data operation
pub struct DataRequest {
    /// Capability Type
    pub capability_type: String,
    /// Parameters
    pub parameters: Value,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
/// Response data for Data operation
pub struct DataResponse {
    /// Data
    pub data: Value,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
    /// Provider
    pub provider: String,
}

#[derive(Debug, Clone)]
/// Providerinfo
pub struct ProviderInfo {
    /// Name
    pub name: String,
    /// Version
    pub version: String,
    /// Capabilities
    pub capabilities: Vec<String>,
}

impl DataCapability for ExternalModelDataProvider {
    /// Handles  Request
    async fn handle_request(&self, request: &DataRequest) -> Result<DataResponse> {
        // This would be implemented by external AI systems (like Intelligence)
        // NestGate just defines the interface
        Ok(DataResponse {
            data: serde_json::json!({
                "message": "Model data requests should be handled by AI primals like Intelligence",
                "delegation_pattern": "Use universal adapter to route to AI capabilities"
            }),
            metadata: HashMap::new(),
            provider: self.provider_name.clone(),
        })
    }
    
    /// Supported Capabilities
    fn supported_capabilities(&self) -> Vec<String> {
        self.capabilities.clone()
    }
    
    /// Provider Info
    fn provider_info(&self) -> ProviderInfo {
        ProviderInfo {
            name: self.provider_name.clone(),
            version: "1.0.0".to_string(),
            capabilities: self.capabilities.clone(),
        }
    }
