//! Configuration types for automation system

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
            songbird_url: "http://localhost:8080".to_string(),
            #[cfg(feature = "network-integration")]
            squirrel_mcp_url: "http://localhost:8081".to_string(),
            #[cfg(feature = "network-integration")]
            toadstool_compute_url: "http://localhost:8082".to_string(),
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
                "http://localhost:8080".to_string(),
                "http://localhost:8081".to_string(),
            ],
            discovery_timeout_ms: 5000,
            health_check_interval_ms: 30000,
            multicast_enabled: true,
            mdns_enabled: true,
        }
    }
} 