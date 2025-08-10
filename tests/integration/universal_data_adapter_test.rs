//! Integration Tests for Universal Data Adapter
//!
//! These tests demonstrate how NestGate's universal data capabilities
//! can work with any external data provider without hardcoding specific services.

use nestgate_core::data_sources::{
    UniversalDataAdapter, DataRequest, DataCapability,
    UniversalGenomeProvider, UniversalHttpProvider,
    providers::genome_provider_example::GenomeProviderFactory,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio;

/// Mock HTTP server for testing (simulates any external API)
struct MockDataServer {
    port: u16,
}

impl MockDataServer {
    fn new() -> Self {
        Self { port: 8999 } // Use a test port
    }
    
    fn base_url(&self) -> String {
        format!("http://localhost:{}", self.port)
    }
    
    // In a real test, you'd start an actual HTTP server
    // For this example, we'll simulate the responses
}

#[tokio::test]
async fn test_universal_data_adapter_with_multiple_providers() {
    // This test shows how multiple data providers can be registered
    // without NestGate knowing their specific identities
    
    let mut adapter = UniversalDataAdapter::new();
    
    // Simulate registering providers for different capabilities
    // In reality, these would be real HTTP endpoints or database connections
    
    // Example 1: Genome data provider (could be NCBI, Ensembl, or any custom database)
    let genome_provider = create_mock_genome_provider().await.expect("Failed to create genome provider");
    adapter.register_provider(genome_provider);
    
    // Example 2: Model data provider (could be HuggingFace, ModelHub, or any custom repository)
    let model_provider = create_mock_model_provider().await.expect("Failed to create model provider");
    adapter.register_provider(model_provider);
    
    // Test that capabilities are registered
    let capabilities = adapter.get_available_capabilities();
    assert!(capabilities.contains(&"genome_data".to_string()));
    assert!(capabilities.contains(&"model_data".to_string()));
    
    // Test genome data request
    let genome_request = DataRequest {
        capability_type: "genome_data".to_string(),
        parameters: {
            let mut params = HashMap::new();
            params.insert("query".to_string(), json!("human insulin"));
            params.insert("organism".to_string(), json!("homo sapiens"));
            params
        },
        metadata: HashMap::new(),
    };
    
    let genome_response = adapter.execute_request(&genome_request).await;
    assert!(genome_response.is_ok(), "Genome data request should succeed");
    
    // Test model data request
    let model_request = DataRequest {
        capability_type: "model_data".to_string(),
        parameters: {
            let mut params = HashMap::new();
            params.insert("query".to_string(), json!("bert-base"));
            params.insert("task".to_string(), json!("text-classification"));
            params
        },
        metadata: HashMap::new(),
    };
    
    let model_response = adapter.execute_request(&model_request).await;
    assert!(model_response.is_ok(), "Model data request should succeed");
}

#[tokio::test]
async fn test_fallback_providers() {
    // This test demonstrates how fallback providers work
    // when the primary provider fails
    
    let mut adapter = UniversalDataAdapter::new();
    
    // Register a primary provider that will "fail"
    let failing_provider = create_failing_provider().await;
    adapter.register_provider(failing_provider);
    
    // Register a fallback provider
    let fallback_provider = create_mock_genome_provider().await.expect("Failed to create fallback provider");
    adapter.register_fallback_provider("genome_data".to_string(), fallback_provider);
    
    // Make a request - should succeed using fallback
    let request = DataRequest {
        capability_type: "genome_data".to_string(),
        parameters: {
            let mut params = HashMap::new();
            params.insert("query".to_string(), json!("test sequence"));
            params
        },
        metadata: HashMap::new(),
    };
    
    let response = adapter.execute_request(&request).await;
    assert!(response.is_ok(), "Request should succeed with fallback provider");
    
    // Verify the response came from a provider
    let response = response.unwrap();
    assert!(response.source_info.is_some(), "Response should have source info");
}

#[tokio::test]
async fn test_provider_agnosticism() {
    // This test shows that NestGate doesn't care about provider identity
    // It only cares about capabilities
    
    let mut adapter = UniversalDataAdapter::new();
    
    // Create providers with different "identities" but same capability
    let provider_a = create_provider_with_name("Custom Genome DB A", "genome_data").await;
    let provider_b = create_provider_with_name("Custom Genome DB B", "genome_data").await;
    
    // Register primary and fallback
    adapter.register_provider(provider_a);
    adapter.register_fallback_provider("genome_data".to_string(), provider_b);
    
    // Make request - adapter doesn't know or care which specific provider responds
    let request = DataRequest {
        capability_type: "genome_data".to_string(),
        parameters: {
            let mut params = HashMap::new();
            params.insert("query".to_string(), json!("agnostic test"));
            params
        },
        metadata: HashMap::new(),
    };
    
    let response = adapter.execute_request(&request).await;
    assert!(response.is_ok(), "Agnostic request should succeed");
    
    // The adapter successfully got genome data without knowing the provider's identity
    let response = response.unwrap();
    assert_eq!(response.data.get("capability_type").unwrap().as_str().unwrap(), "genome_data");
}

// Helper functions for creating test providers

async fn create_mock_genome_provider() -> Result<Arc<dyn DataCapability>, Box<dyn std::error::Error>> {
    // In a real test, this would connect to an actual API
    // For this example, we'll create a mock provider
    
    use nestgate_core::data_sources::providers::universal_http_provider::{HttpProviderConfigBuilder, UniversalHttpProvider};
    
    let config = HttpProviderConfigBuilder::new(
        "http://mock-genome-api.test".to_string(),
        "genome_data".to_string()
    )
    .with_metadata("provider_type".to_string(), "genome_database".to_string())
    .with_metadata("test_mode".to_string(), "true".to_string())
    .build();
    
    let provider = MockGenomeProvider::new(config);
    Ok(Arc::new(provider))
}

async fn create_mock_model_provider() -> Result<Arc<dyn DataCapability>, Box<dyn std::error::Error>> {
    let config = HttpProviderConfigBuilder::new(
        "http://mock-model-api.test".to_string(),
        "model_data".to_string()
    )
    .with_metadata("provider_type".to_string(), "model_repository".to_string())
    .with_metadata("test_mode".to_string(), "true".to_string())
    .build();
    
    let provider = MockModelProvider::new(config);
    Ok(Arc::new(provider))
}

async fn create_failing_provider() -> Arc<dyn DataCapability> {
    Arc::new(FailingProvider::new())
}

async fn create_provider_with_name(name: &str, capability: &str) -> Arc<dyn DataCapability> {
    let config = HttpProviderConfigBuilder::new(
        format!("http://{}.test", name.replace(" ", "-").to_lowercase()),
        capability.to_string()
    )
    .with_metadata("provider_name".to_string(), name.to_string())
    .with_metadata("test_mode".to_string(), "true".to_string())
    .build();
    
    Arc::new(MockGenericProvider::new(config))
}

// Mock provider implementations for testing

use nestgate_core::{NestGateError, Result};
use nestgate_core::data_sources::data_capabilities::{DataResponse, SourceInfo};
use async_trait::async_trait;

struct MockGenomeProvider {
    capability_type: String,
    metadata: HashMap<String, String>,
}

impl MockGenomeProvider {
    fn new(config: nestgate_core::data_sources::providers::universal_http_provider::HttpProviderConfig) -> Self {
        Self {
            capability_type: config.capability_type,
            metadata: config.metadata,
        }
    }
}

#[async_trait]
impl DataCapability for MockGenomeProvider {
    fn capability_type(&self) -> &str {
        &self.capability_type
    }
    
    async fn can_handle(&self, request: &DataRequest) -> Result<bool> {
        Ok(request.capability_type == self.capability_type)
    }
    
    async fn execute_request(&self, request: &DataRequest) -> Result<DataResponse> {
        // Mock response for genome data
        let mock_data = json!({
            "capability_type": "genome_data",
            "results": [
                {
                    "id": "mock_sequence_1",
                    "title": "Mock Genome Sequence",
                    "organism": "Test Organism",
                    "sequence": "ATCGATCGATCG"
                }
            ]
        });
        
        Ok(DataResponse {
            data: mock_data,
            metadata: request.metadata.clone(),
            source_info: Some(SourceInfo {
                provider_type: "genome_database".to_string(),
                provider_name: self.metadata.get("provider_name").cloned(),
                license: Some("Test License".to_string()),
            }),
        })
    }
    
    fn get_metadata(&self) -> HashMap<String, String> {
        self.metadata.clone()
    }
}

struct MockModelProvider {
    capability_type: String,
    metadata: HashMap<String, String>,
}

impl MockModelProvider {
    fn new(config: nestgate_core::data_sources::providers::universal_http_provider::HttpProviderConfig) -> Self {
        Self {
            capability_type: config.capability_type,
            metadata: config.metadata,
        }
    }
}

#[async_trait]
impl DataCapability for MockModelProvider {
    fn capability_type(&self) -> &str {
        &self.capability_type
    }
    
    async fn can_handle(&self, request: &DataRequest) -> Result<bool> {
        Ok(request.capability_type == self.capability_type)
    }
    
    async fn execute_request(&self, request: &DataRequest) -> Result<DataResponse> {
        let mock_data = json!({
            "capability_type": "model_data",
            "models": [
                {
                    "id": "mock-bert-base",
                    "name": "Mock BERT Base",
                    "type": "transformer",
                    "parameters": 110000000
                }
            ]
        });
        
        Ok(DataResponse {
            data: mock_data,
            metadata: request.metadata.clone(),
            source_info: Some(SourceInfo {
                provider_type: "model_repository".to_string(),
                provider_name: self.metadata.get("provider_name").cloned(),
                license: Some("Test License".to_string()),
            }),
        })
    }
    
    fn get_metadata(&self) -> HashMap<String, String> {
        self.metadata.clone()
    }
}

struct FailingProvider {
    capability_type: String,
}

impl FailingProvider {
    fn new() -> Self {
        Self {
            capability_type: "genome_data".to_string(),
        }
    }
}

#[async_trait]
impl DataCapability for FailingProvider {
    fn capability_type(&self) -> &str {
        &self.capability_type
    }
    
    async fn can_handle(&self, _request: &DataRequest) -> Result<bool> {
        Ok(true) // Claims it can handle requests but will fail
    }
    
    async fn execute_request(&self, _request: &DataRequest) -> Result<DataResponse> {
        Err(NestGateError::Internal {
            message: "Mock provider failure".to_string(),
            location: Some("FailingProvider::execute_request".to_string()),
            debug_info: None,
            is_bug: false,
        })
    }
    
    fn get_metadata(&self) -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        metadata.insert("provider_type".to_string(), "failing_test_provider".to_string());
        metadata
    }
}

struct MockGenericProvider {
    capability_type: String,
    metadata: HashMap<String, String>,
}

impl MockGenericProvider {
    fn new(config: nestgate_core::data_sources::providers::universal_http_provider::HttpProviderConfig) -> Self {
        Self {
            capability_type: config.capability_type,
            metadata: config.metadata,
        }
    }
}

#[async_trait]
impl DataCapability for MockGenericProvider {
    fn capability_type(&self) -> &str {
        &self.capability_type
    }
    
    async fn can_handle(&self, request: &DataRequest) -> Result<bool> {
        Ok(request.capability_type == self.capability_type)
    }
    
    async fn execute_request(&self, request: &DataRequest) -> Result<DataResponse> {
        let mock_data = json!({
            "capability_type": self.capability_type,
            "provider_name": self.metadata.get("provider_name").unwrap_or(&"Unknown".to_string()),
            "data": "mock response from generic provider"
        });
        
        Ok(DataResponse {
            data: mock_data,
            metadata: request.metadata.clone(),
            source_info: Some(SourceInfo {
                provider_type: self.capability_type.clone(),
                provider_name: self.metadata.get("provider_name").cloned(),
                license: Some("Test License".to_string()),
            }),
        })
    }
    
    fn get_metadata(&self) -> HashMap<String, String> {
        self.metadata.clone()
    }
} 