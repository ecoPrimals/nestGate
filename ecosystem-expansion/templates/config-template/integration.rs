//! Integration configuration structures - UNIVERSAL ADAPTER PATTERN

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    pub enabled: bool,
    /// Universal adapter configuration
    pub universal_adapter: UniversalAdapterConfig,
    /// Capability-based service discovery
    pub capability_discovery: CapabilityDiscoveryConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalAdapterConfig {
    pub enabled: bool,
    /// Service discovery endpoint (dynamic)
    pub discovery_endpoint: Option<String>,
    /// Maximum concurrent connections
    pub max_connections: u32,
    /// Request timeout in seconds
    pub request_timeout_secs: u64,
    /// Retry configuration
    pub retry_config: RetryConfig,
}

/// Capability-based service configuration (replaces hardcoded primal endpoints)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityDiscoveryConfig {
    /// Enable automatic capability discovery
    pub auto_discovery_enabled: bool,
    /// Required capabilities for this service
    pub required_capabilities: Vec<String>,
    /// Optional capabilities that enhance functionality
    pub optional_capabilities: Vec<String>,
    /// Capability preferences and priorities
    pub capability_preferences: HashMap<String, CapabilityPreference>,
    /// Fallback strategies when capabilities are unavailable
    pub fallback_strategies: HashMap<String, FallbackStrategy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityPreference {
    /// Preferred provider characteristics
    pub preferred_performance_tier: String,
    /// Maximum acceptable latency in milliseconds
    pub max_latency_ms: u64,
    /// Minimum required availability percentage
    pub min_availability_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FallbackStrategy {
    /// Fallback to local implementation
    pub local_fallback: bool,
    /// Alternative capability to use
    pub alternative_capability: Option<String>,
    /// Graceful degradation mode
    pub degraded_mode_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_attempts: u32,
    /// Base delay between retries in milliseconds
    pub base_delay_ms: u64,
    /// Exponential backoff multiplier
    pub backoff_multiplier: f64,
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            universal_adapter: UniversalAdapterConfig::default(),
            capability_discovery: CapabilityDiscoveryConfig::default(),
        }
    }
}

impl Default for UniversalAdapterConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            discovery_endpoint: None, // Auto-discover
            max_connections: 100,
            request_timeout_secs: 30,
            retry_config: RetryConfig::default(),
        }
    }
}

impl Default for CapabilityDiscoveryConfig {
    fn default() -> Self {
        let mut capability_preferences = HashMap::new();
        let mut fallback_strategies = HashMap::new();

        // AI/Intelligence capabilities (replaces squirrel hardcoding)
        capability_preferences.insert(
            "artificial_intelligence".to_string(),
            CapabilityPreference {
                preferred_performance_tier: "high".to_string(),
                max_latency_ms: 5000,
                min_availability_percent: 95.0,
            },
        );
        fallback_strategies.insert(
            "artificial_intelligence".to_string(),
            FallbackStrategy {
                local_fallback: true,
                alternative_capability: Some("basic_text_processing".to_string()),
                degraded_mode_enabled: true,
            },
        );

        // Security capabilities (replaces beardog hardcoding)
        capability_preferences.insert(
            "security".to_string(),
            CapabilityPreference {
                preferred_performance_tier: "enterprise".to_string(),
                max_latency_ms: 2000,
                min_availability_percent: 99.5,
            },
        );
        fallback_strategies.insert(
            "security".to_string(),
            FallbackStrategy {
                local_fallback: true,
                alternative_capability: Some("basic_authentication".to_string()),
                degraded_mode_enabled: false, // Security cannot be degraded
            },
        );

        // Orchestration capabilities (replaces songbird hardcoding)
        capability_preferences.insert(
            "orchestration".to_string(),
            CapabilityPreference {
                preferred_performance_tier: "standard".to_string(),
                max_latency_ms: 3000,
                min_availability_percent: 98.0,
            },
        );
        fallback_strategies.insert(
            "orchestration".to_string(),
            FallbackStrategy {
                local_fallback: true,
                alternative_capability: Some("simple_coordination".to_string()),
                degraded_mode_enabled: true,
            },
        );

        // Compute capabilities (replaces toadstool hardcoding)
        capability_preferences.insert(
            "compute".to_string(),
            CapabilityPreference {
                preferred_performance_tier: "high_performance".to_string(),
                max_latency_ms: 10000,
                min_availability_percent: 95.0,
            },
        );
        fallback_strategies.insert(
            "compute".to_string(),
            FallbackStrategy {
                local_fallback: true,
                alternative_capability: Some("local_processing".to_string()),
                degraded_mode_enabled: true,
            },
        );

        Self {
            auto_discovery_enabled: true,
            required_capabilities: vec![
                "storage".to_string(),
                "security".to_string(),
            ],
            optional_capabilities: vec![
                "artificial_intelligence".to_string(),
                "orchestration".to_string(),
                "compute".to_string(),
                "monitoring".to_string(),
                "analytics".to_string(),
            ],
            capability_preferences,
            fallback_strategies,
        }
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay_ms: 1000,
            backoff_multiplier: 2.0,
        }
    }
}
