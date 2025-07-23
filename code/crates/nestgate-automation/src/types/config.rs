//! Configuration types for automation system

use nestgate_core::constants::strings::LOCALHOST_IP;
use serde::{Deserialize, Serialize};

/// Main automation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationConfig {
    pub enable_intelligent_tier_assignment: bool,
    pub enable_automatic_optimization: bool,
    pub optimization_interval_hours: u64,
    pub prediction_cache_ttl_hours: u64,
    pub min_confidence_threshold: f64,

    // Ecosystem service URLs (only available with network-integration feature)
    #[cfg(feature = "network-integration")]
    pub songbird_url: String,
    #[cfg(feature = "network-integration")]
    pub squirrel_mcp_url: String,
    #[cfg(feature = "network-integration")]
    pub toadstool_compute_url: String,

    pub data_api_endpoint: String,
    pub prediction_endpoint: String,
}

impl Default for AutomationConfig {
    fn default() -> Self {
        Self {
            enable_intelligent_tier_assignment: true,
            enable_automatic_optimization: true,
            optimization_interval_hours: 1,
            prediction_cache_ttl_hours: 1,
            min_confidence_threshold: 0.7,

            #[cfg(feature = "network-integration")]
            songbird_url: std::env::var("NESTGATE_SONGBIRD_URL")
                .unwrap_or_else(|_| {
                    format!(
                        "http://{}:{}",
                        LOCALHOST_IP,
                        nestgate_core::constants::network::api_port()
                    )
                })
                .to_string(),
            #[cfg(feature = "network-integration")]
            squirrel_mcp_url: std::env::var("NESTGATE_SQUIRREL_MCP_URL")
                .unwrap_or_else(|_| {
                    format!(
                        "http://{}:{}",
                        LOCALHOST_IP,
                        std::env::var("NESTGATE_MCP_PORT").unwrap_or_else(|_| "8081".to_string())
                    )
                })
                .to_string(),
            #[cfg(feature = "network-integration")]
            toadstool_compute_url: std::env::var("NESTGATE_TOADSTOOL_COMPUTE_URL")
                .unwrap_or_else(|_| {
                    format!(
                        "http://{}:{}",
                        LOCALHOST_IP,
                        std::env::var("NESTGATE_PREDICTION_PORT")
                            .unwrap_or_else(|_| "8082".to_string())
                    )
                })
                .to_string(),

            data_api_endpoint: std::env::var("NESTGATE_DATA_API_ENDPOINT")
                .unwrap_or_else(|_| {
                    format!(
                        "http://{}:{}",
                        LOCALHOST_IP,
                        nestgate_core::constants::network::api_port()
                    )
                })
                .to_string(),
            prediction_endpoint: std::env::var("NESTGATE_PREDICTION_ENDPOINT")
                .unwrap_or_else(|_| {
                    format!(
                        "http://{}:{}",
                        LOCALHOST_IP,
                        std::env::var("NESTGATE_PREDICTION_PORT")
                            .unwrap_or_else(|_| "8082".to_string())
                    )
                })
                .to_string(),
        }
    }
}

/// Discovery configuration for ecosystem services
#[cfg(feature = "network-integration")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryConfig {
    pub known_songbird_endpoints: Vec<String>,
    pub discovery_timeout_ms: u64,
    pub health_check_interval_ms: u64,
    pub multicast_enabled: bool,
    pub mdns_enabled: bool,
}

#[cfg(feature = "network-integration")]
impl DiscoveryConfig {
    pub fn from_automation_config(config: &AutomationConfig) -> Self {
        Self {
            known_songbird_endpoints: vec![
                config.songbird_url.clone(),
                std::env::var("NESTGATE_SONGBIRD_BACKUP_URL_1").unwrap_or_else(|_| {
                    format!(
                        "http://{}:{}",
                        LOCALHOST_IP,
                        std::env::var("NESTGATE_SONGBIRD_BACKUP_PORT")
                            .unwrap_or_else(|_| "8080".to_string())
                    )
                }),
                std::env::var("NESTGATE_SONGBIRD_BACKUP_URL_2").unwrap_or_else(|_| {
                    format!(
                        "http://{}:{}",
                        LOCALHOST_IP,
                        std::env::var("NESTGATE_MCP_PORT").unwrap_or_else(|_| "8081".to_string())
                    )
                }),
            ],
            discovery_timeout_ms: 5000,
            health_check_interval_ms: std::env::var("NESTGATE_HEALTH_CHECK_INTERVAL_MS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30000),
            multicast_enabled: true,
            mdns_enabled: true,
        }
    }
}
