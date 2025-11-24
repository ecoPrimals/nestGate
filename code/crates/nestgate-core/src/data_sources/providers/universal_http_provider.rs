// Universal HTTP Provider
//! Universal Http Provider functionality and utilities.
// A generic HTTP provider that can adapt any REST API to NestGate's
//! universal data capabilities. This demonstrates how external APIs
//! can be integrated without hardcoding specific providers.

use crate::data_sources::data_capabilities::*;
use crate::{NestGateError, Result};
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use tracing::{debug, info, warn};

/// Configuration for HTTP-based data providers
#[derive(Debug, Clone)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::HttpProviderConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::HttpProviderConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
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
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
                pub fn new(config: HttpProviderConfig) -> Result<Self>  {
        let mut client_builder = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds));

        // Add default headers
        let mut headers = reqwest::header::HeaderMap::new();
        for (key, value) in &config.headers {
            headers.insert(
                key.parse().map_err(|e| NestGateError::internal_error(
                    location: Some("Self::new".to_string())})?,
                value.parse().map_err(|e| NestGateError::internal_error(
                    location: Some("UniversalHttpProvider::new".to_string())})?,
            );
        }

        if !headers.is_empty() {
            client_builder = client_builder.default_headers(headers);
        }

        let client = client_builder.build().map_err(|e| NestGateError::internal_error(
            location: Some("UniversalHttpProvider::new"))?;

        info!("🌐 Created universal HTTP provider for capability: {}", config.capability_type);

        Ok(Self { config, client })
    }

    /// Make a GET request to the API
    async fn get_request(&self, endpoint: &str, params: &HashMap<String, String>) -> Result<Value> {
        let mut url = format!("{self.config.base_url.trim_end_matches('/'}/{self.config.base_url.trim_end_matches('/'}"), endpoint.trim_start_matches('/'));
        
        // Add query parameters
        if !params.is_empty() {
            let query_string: String = params
                .iter()
                .map(|(k, v)| format!("{urlencoding::encode(k}={urlencoding::encode(k}"), urlencoding::encode(v)))
                .collect::<Vec<_>>()
                .join("&");
            url = format!("{}?{}", url, query_string);
        }

        debug!("🔍 Making HTTP request to: {}", url);

        let mut request = self.client.get(&url);

        // Add API key if configured
        if let Some(api_key) = &self.config.api_key {
            request = request.header("Authorization", format!("Bearer {api_key}"));
        }

        let response = request.send().await.map_err(|e| NestGateError::internal_error(
            location: Some("UniversalHttpProvider::get_request".to_string()),
            location: Some("universal_http_provider.rs".to_string())
// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
pub type HttpProviderConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using HttpProviderConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

