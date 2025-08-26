
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;


/// **AUTOMATION CONFIG TYPES**
/// Configuration type definitions for the automation system
/// Main automation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationConfig {
    /// Analysis configuration
    pub analysis: AnalysisConfig,
    /// Prediction configuration  
    pub prediction: PredictionConfig,
    /// Lifecycle management configuration
    pub lifecycle: LifecycleConfig,
    /// Optimization interval in hours
    pub optimization_interval_hours: u32,
    /// Prediction cache TTL in hours
    pub prediction_cache_ttl_hours: u32,
    /// Enable intelligent tier assignment
    pub enable_intelligent_tier_assignment: bool,
    /// Enable automatic optimization
    pub enable_automatic_optimization: bool,
    /// Minimum confidence threshold for predictions
    pub min_confidence_threshold: f64,
    /// Orchestration endpoint
    pub orchestration_endpoint: Option<String>,
}

impl Default for AutomationConfig {
    fn default() -> Self {
        Self {
            analysis: AnalysisConfig::default(),
            prediction: PredictionConfig::default(),
            lifecycle: LifecycleConfig::default(),
            optimization_interval_hours: 24,
            prediction_cache_ttl_hours: 12,
            enable_intelligent_tier_assignment: true,
            enable_automatic_optimization: true,
            min_confidence_threshold: 0.7,
            orchestration_endpoint: None,
        }
    }
}

impl AutomationConfig {
    /// Create production configuration
    pub fn production() -> Self {
        Self {
            analysis: AnalysisConfig::default(),
            prediction: PredictionConfig::default(),
            lifecycle: LifecycleConfig::default(),
            optimization_interval_hours: 6, // More frequent optimization for production
            prediction_cache_ttl_hours: 24, // Longer cache for production
            enable_intelligent_tier_assignment: true,
            enable_automatic_optimization: true,
            min_confidence_threshold: 0.8, // Higher confidence for production
            orchestration_endpoint: Some("http://localhost:8080".to_string()),
        }
    }

    /// Create development configuration
    pub fn development() -> Self {
        Self {
            analysis: AnalysisConfig::default(),
            prediction: PredictionConfig::default(),
            lifecycle: LifecycleConfig::default(),
            optimization_interval_hours: 1, // More frequent optimization for development
            prediction_cache_ttl_hours: 2,  // Shorter cache for development
            enable_intelligent_tier_assignment: true,
            enable_automatic_optimization: false, // Disable auto-optimization in dev
            min_confidence_threshold: 0.5,        // Lower confidence for development
            orchestration_endpoint: Some("http://localhost:8080".to_string()),
        }
    }
}

/// Analysis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    /// Scan interval
    pub scan_interval: Duration,
    /// Maximum file size to analyze
    pub max_file_size: u64,
    /// File extensions to include
    pub include_extensions: Vec<String>,
    /// File extensions to exclude
    pub exclude_extensions: Vec<String>,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            scan_interval: Duration::from_secs(3600), // 1 hour
            max_file_size: 1024 * 1024 * 1024,        // 1GB
            include_extensions: vec!["*".to_string()],
            exclude_extensions: vec![".tmp".to_string(), ".log".to_string()],
        }
    }
}

/// Prediction configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionConfig {
    /// Prediction window in days
    pub prediction_window_days: u32,
    /// Minimum confidence threshold
    pub min_confidence: f64,
    /// Model parameters
    pub model_params: HashMap<String, f64>,
}

impl Default for PredictionConfig {
    fn default() -> Self {
        Self {
            prediction_window_days: 30,
            min_confidence: 0.7,
            model_params: HashMap::new(),
        }
    }
}

/// Lifecycle configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleConfig {
    /// Hot tier retention in days
    pub hot_retention_days: u32,
    /// Warm tier retention in days
    pub warm_retention_days: u32,
    /// Cold tier retention in days
    pub cold_retention_days: u32,
    /// Enable automatic migration
    pub auto_migration: bool,
}

impl Default for LifecycleConfig {
    fn default() -> Self {
        Self {
            hot_retention_days: 30,
            warm_retention_days: 90,
            cold_retention_days: 365,
            auto_migration: true,
        }
    }
}

/// Discovery configuration for ecosystem services
#[cfg(feature = "network-integration")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryConfig {
    pub known_orchestration_endpoints: Vec<String>,
    pub discovery_timeout_ms: u64,
    pub health_check_interval_ms: u64,
    pub multicast_enabled: bool,
    pub mdns_enabled: bool,
}

#[cfg(feature = "network-integration")]
impl DiscoveryConfig {
    pub fn from_automation_config(config: &AutomationConfig) -> Self {
        Self {
            known_orchestration_endpoints: vec![
                config
                    .orchestration_endpoint
                    .clone()
                    .unwrap_or_else(|| "http://localhost:8080".to_string()),
                std::env::var("NESTGATE_ORCHESTRATION_BACKUP_ENDPOINT_1").unwrap_or_else(|_| {
                    format!(
                        "http://{}:{}",
                        std::env::var("NESTGATE_ORCHESTRATION_IP")
                            .unwrap_or_else(|_| "127.0.0.1".to_string()),
                        std::env::var("NESTGATE_ORCHESTRATION_PORT")
                            .unwrap_or_else(|_| "8080".to_string())
                    )
                }),
                std::env::var("NESTGATE_ORCHESTRATION_BACKUP_ENDPOINT_2").unwrap_or_else(|_| {
                    format!(
                        "http://{}:{}",
                        std::env::var("NESTGATE_MCP_IP")
                            .unwrap_or_else(|_| "127.0.0.1".to_string()),
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
