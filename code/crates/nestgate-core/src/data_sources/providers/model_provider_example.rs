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
pub struct ExternalModelDataProvider {
    pub provider_name: String,
    pub endpoint: String,
    pub capabilities: Vec<String>,
}
impl ExternalModelDataProvider {
    pub const fn new(provider_name: String, endpoint: String) -> Self {
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
    fn handle_request(&self, request: &DataRequest) -> impl std::future::Future<Output = Result<DataResponse>> + Send;
    fn supported_capabilities(&self) -> Vec<String>;
    fn provider_info(&self) -> ProviderInfo;
}
#[derive(Debug, Clone)]
pub struct DataRequest {
    pub capability_type: String,
    pub parameters: Value,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct DataResponse {
    pub data: Value,
    pub metadata: HashMap<String, String>,
    pub provider: String,
}

#[derive(Debug, Clone)]
pub struct ProviderInfo {
    pub name: String,
    pub version: String,
    pub capabilities: Vec<String>,
}

impl DataCapability for ExternalModelDataProvider {
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
    
    fn supported_capabilities(&self) -> Vec<String> {
        self.capabilities.clone()
    }
    
    fn provider_info(&self) -> ProviderInfo {
        ProviderInfo {
            name: self.provider_name.clone(),
            version: "1.0.0".to_string(),
            capabilities: self.capabilities.clone(),
        }
    }
