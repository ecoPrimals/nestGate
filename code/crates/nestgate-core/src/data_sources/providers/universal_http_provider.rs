//! Universal HTTP Provider
//!
//! A generic HTTP provider that can adapt any REST API to NestGate's
//! universal data capabilities. This demonstrates how external APIs
//! can be integrated without hardcoding specific providers.

use crate::data_sources::data_capabilities::*;
use crate::{NestGateError, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use tracing::{debug, info, warn};

/// Configuration for HTTP-based data providers
#[derive(Debug, Clone)]
pub struct HttpProviderConfig {
    /// Base URL for the API
    pub base_url: String,
    /// API key or token (if required)
    pub api_key: Option<String>,
    /// Request headers to include
    pub headers: HashMap<String, String>,
    /// Request timeout in seconds
    pub timeout_seconds: u64,
    /// What type of data capability this provider offers
    pub capability_type: String,
    /// Provider metadata
    pub metadata: HashMap<String, String>,
}

/// Universal HTTP provider that can adapt any REST API
pub struct UniversalHttpProvider {
    config: HttpProviderConfig,
    client: Client,
}

impl UniversalHttpProvider {
    /// Create a new universal HTTP provider
    pub fn new(config: HttpProviderConfig) -> Result<Self> {
        let mut client_builder = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds));

        // Add default headers
        let mut headers = reqwest::header::HeaderMap::new();
        for (key, value) in &config.headers {
            headers.insert(
                key.parse().map_err(|e| NestGateError::Internal {
                    message: format!("Invalid header name '{}': {}", key, e),
                    location: Some("UniversalHttpProvider::new".to_string()),
                    debug_info: None,
                    is_bug: false,
                })?,
                value.parse().map_err(|e| NestGateError::Internal {
                    message: format!("Invalid header value '{}': {}", value, e),
                    location: Some("UniversalHttpProvider::new".to_string()),
                    debug_info: None,
                    is_bug: false,
                })?,
            );
        }

        if !headers.is_empty() {
            client_builder = client_builder.default_headers(headers);
        }

        let client = client_builder.build().map_err(|e| NestGateError::Internal {
            message: format!("Failed to create HTTP client: {}", e),
            location: Some("UniversalHttpProvider::new".to_string()),
            debug_info: None,
            is_bug: false,
        })?;

        info!("🌐 Created universal HTTP provider for capability: {}", config.capability_type);

        Ok(Self { config, client })
    }

    /// Make a GET request to the API
    async fn get_request(&self, endpoint: &str, params: &HashMap<String, String>) -> Result<Value> {
        let mut url = format!("{}/{}", self.config.base_url.trim_end_matches('/'), endpoint.trim_start_matches('/'));
        
        // Add query parameters
        if !params.is_empty() {
            let query_string: String = params
                .iter()
                .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
                .collect::<Vec<_>>()
                .join("&");
            url = format!("{}?{}", url, query_string);
        }

        debug!("🔍 Making HTTP request to: {}", url);

        let mut request = self.client.get(&url);

        // Add API key if configured
        if let Some(api_key) = &self.config.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = request.send().await.map_err(|e| NestGateError::Internal {
            message: format!("HTTP request failed: {}", e),
            location: Some("UniversalHttpProvider::get_request".to_string()),
            debug_info: Some(format!("URL: {}", url)),
            is_bug: false,
        })?;

        if !response.status().is_success() {
            return Err(NestGateError::Internal {
                message: format!("HTTP request failed with status: {}", response.status()),
                location: Some("UniversalHttpProvider::get_request".to_string()),
                debug_info: Some(format!("URL: {}", url)),
                is_bug: false,
            });
        }

        let json: Value = response.json().await.map_err(|e| NestGateError::Internal {
            message: format!("Failed to parse JSON response: {}", e),
            location: Some("UniversalHttpProvider::get_request".to_string()),
            debug_info: Some(format!("URL: {}", url)),
            is_bug: false,
        })?;

        Ok(json)
    }
}

#[async_trait]
impl DataCapability for UniversalHttpProvider {
    fn capability_type(&self) -> &str {
        &self.config.capability_type
    }

    async fn can_handle(&self, request: &DataRequest) -> Result<bool> {
        // Check if the capability type matches
        Ok(request.capability_type == self.config.capability_type)
    }

    async fn execute_request(&self, request: &DataRequest) -> Result<DataResponse> {
        debug!("🚀 Executing HTTP data request for: {}", request.capability_type);

        // Convert parameters to string map for URL encoding
        let mut params = HashMap::new();
        for (key, value) in &request.parameters {
            params.insert(key.clone(), value.to_string().trim_matches('"').to_string());
        }

        // Determine endpoint based on request type
        let endpoint = request.parameters
            .get("endpoint")
            .and_then(|v| v.as_str())
            .unwrap_or("search"); // Default endpoint

        // Make the HTTP request
        let data = self.get_request(endpoint, &params).await?;

        // Create source info for attribution
        let source_info = SourceInfo {
            provider_type: self.config.capability_type.clone(),
            provider_name: self.config.metadata.get("name").cloned(),
            license: self.config.metadata.get("license").cloned(),
        };

        Ok(DataResponse {
            data,
            metadata: request.metadata.clone(),
            source_info: Some(source_info),
        })
    }

    fn get_metadata(&self) -> HashMap<String, String> {
        let mut metadata = self.config.metadata.clone();
        metadata.insert("provider_type".to_string(), "http".to_string());
        metadata.insert("base_url".to_string(), self.config.base_url.clone());
        metadata.insert("capability_type".to_string(), self.config.capability_type.clone());
        metadata
    }
}

/// Builder for creating HTTP provider configurations
pub struct HttpProviderConfigBuilder {
    base_url: String,
    api_key: Option<String>,
    headers: HashMap<String, String>,
    timeout_seconds: u64,
    capability_type: String,
    metadata: HashMap<String, String>,
}

impl HttpProviderConfigBuilder {
    pub fn new(base_url: String, capability_type: String) -> Self {
        Self {
            base_url,
            api_key: None,
            headers: HashMap::new(),
            timeout_seconds: 30,
            capability_type,
            metadata: HashMap::new(),
        }
    }

    pub fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    pub fn with_timeout(mut self, timeout_seconds: u64) -> Self {
        self.timeout_seconds = timeout_seconds;
        self
    }

    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    pub fn build(self) -> HttpProviderConfig {
        HttpProviderConfig {
            base_url: self.base_url,
            api_key: self.api_key,
            headers: self.headers,
            timeout_seconds: self.timeout_seconds,
            capability_type: self.capability_type,
            metadata: self.metadata,
        }
    }
} 