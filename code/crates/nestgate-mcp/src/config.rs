//! MCP Configuration Types
//!
//! This module contains all configuration-related types for the MCP system.

use crate::types::{AuthConfig, ProviderCapabilities, ProviderConfig, TlsConfig};
use serde::{Deserialize, Serialize};

/// Enhanced MCP Configuration for v2 with advanced capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfig {
    /// MCP cluster endpoint
    pub cluster_endpoint: String,
    /// Node identifier
    pub node_id: String,
    /// Authentication configuration
    pub auth: Option<AuthConfig>,
    /// TLS configuration
    pub tls: Option<TlsConfig>,
    /// Provider configuration
    pub provider_config: Option<ProviderConfig>,
    /// Orchestrator endpoint
    pub orchestrator_endpoint: String,
    /// Federation enabled
    pub federation_enabled: bool,
}

impl Default for McpConfig {
    fn default() -> Self {
        Self {
            cluster_endpoint: format!(
                "http://{}:{}",
                nestgate_core::constants::network::addresses::LOCALHOST,
                nestgate_core::constants::network::ports::NESTGATE_API
            ),
            node_id: "default-node".to_string(),
            auth: None,
            tls: None,
            provider_config: None,
            orchestrator_endpoint: format!(
                "http://{}:{}",
                nestgate_core::constants::network::addresses::LOCALHOST,
                nestgate_core::constants::network::ports::NESTGATE_API
            ),
            federation_enabled: false,
        }
    }
}

/// Enhanced MCP configuration with advanced capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedMcpConfig {
    pub node_id: String,
    pub cluster_name: String,
    pub federation_enabled: bool,
    pub orchestrator_endpoint: String,
    pub capabilities: ProviderCapabilities,
    pub metrics_collection_interval: u64,
    pub health_check_interval: u64,
    pub retry_config: RetryConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub base_delay_ms: u64,
    pub max_delay_ms: u64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay_ms: 100,
            max_delay_ms: 5000,
        }
    }
}

impl Default for EnhancedMcpConfig {
    fn default() -> Self {
        Self {
            node_id: "default-node".to_string(),
            cluster_name: "default-cluster".to_string(),
            federation_enabled: false,
            orchestrator_endpoint: format!(
                "http://{}:{}",
                nestgate_core::constants::network::addresses::LOCALHOST,
                nestgate_core::constants::network::ports::NESTGATE_API
            ),
            capabilities: ProviderCapabilities::default(),
            metrics_collection_interval: 30, // seconds
            health_check_interval: 10,       // seconds
            retry_config: RetryConfig::default(),
        }
    }
}
