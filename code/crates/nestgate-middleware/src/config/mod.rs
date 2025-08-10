/// Core Middleware Configuration Module
/// Split from unified_middleware_config.rs for maintainability and 2000-line compliance
/// Houses the main configuration structures and re-exports specialized modules
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Import the standardized config pattern
use nestgate_core::unified_config_consolidation::StandardDomainConfig;

// Re-export all module components
pub mod handlers;
pub mod performance;
pub mod security;
pub mod types;

pub use handlers::*;
pub use performance::*;
pub use security::*;
pub use types::*;

// ==================== CORE MIDDLEWARE CONFIGURATION ====================

/// Middleware-specific configuration extensions
/// Domain-specific fields for comprehensive middleware management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewareExtensions {
    /// Core middleware settings
    pub middleware: MiddlewareSettings,
    /// Request processing configuration
    pub request_processing: RequestProcessingSettings,
    /// Response handling configuration
    pub response_handling: ResponseHandlingSettings,
    /// Chain management settings
    pub chain_management: ChainManagementSettings,
    /// Performance optimization
    pub performance: MiddlewarePerformanceSettings,
    /// Security middleware settings
    pub security: MiddlewareSecuritySettings,
    /// Monitoring and observability
    pub observability: MiddlewareObservabilitySettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewareSettings {
    /// Enabled middleware types
    pub enabled_middleware: Vec<MiddlewareType>,
    /// Default middleware stack
    pub default_stack: Vec<MiddlewareType>,
    /// Per-route middleware overrides
    pub route_overrides: HashMap<String, Vec<MiddlewareType>>,
    /// Global middleware configuration
    pub global_config: HashMap<String, MiddlewareConfiguration>,
    /// Middleware priorities
    pub priorities: HashMap<MiddlewareType, u32>,
    /// Enable hot reloading
    pub hot_reload: bool,
}

/// **THE** Unified Middleware Configuration
/// Uses StandardDomainConfig<MiddlewareExtensions> pattern for full ecosystem consistency
pub type UnifiedMiddlewareConfig = StandardDomainConfig<MiddlewareExtensions>;

impl Default for MiddlewareExtensions {
    fn default() -> Self {
        Self {
            middleware: MiddlewareSettings::default(),
            request_processing: RequestProcessingSettings::default(),
            response_handling: ResponseHandlingSettings::default(),
            chain_management: ChainManagementSettings::default(),
            performance: MiddlewarePerformanceSettings::default(),
            security: MiddlewareSecuritySettings::default(),
            observability: MiddlewareObservabilitySettings::default(),
        }
    }
}

impl Default for MiddlewareSettings {
    fn default() -> Self {
        Self {
            enabled_middleware: vec![
                MiddlewareType::Logging,
                MiddlewareType::ErrorHandler,
                MiddlewareType::Security,
            ],
            default_stack: vec![
                MiddlewareType::Security,
                MiddlewareType::Auth,
                MiddlewareType::RateLimit,
                MiddlewareType::Logging,
                MiddlewareType::ErrorHandler,
            ],
            route_overrides: HashMap::new(),
            global_config: HashMap::new(),
            priorities: {
                let mut priorities = HashMap::new();
                priorities.insert(MiddlewareType::Security, 100);
                priorities.insert(MiddlewareType::Auth, 90);
                priorities.insert(MiddlewareType::RateLimit, 80);
                priorities.insert(MiddlewareType::Cors, 70);
                priorities.insert(MiddlewareType::Logging, 60);
                priorities.insert(MiddlewareType::ErrorHandler, 10);
                priorities
            },
            hot_reload: false,
        }
    }
}

// ==================== CONFIGURATION BUILDERS ====================

/// Extension trait for legacy middleware config methods
pub trait LegacyMiddlewareConfigExt {
    /// Create development configuration
    fn development_legacy() -> Self;
    /// Create production configuration
    fn production_legacy() -> Self;
}

impl LegacyMiddlewareConfigExt for UnifiedMiddlewareConfig {
    /// Create development configuration
    fn development_legacy() -> Self {
        Self {
            service: nestgate_core::unified_types::UnifiedServiceConfig {
                name: "nestgate-middleware".to_string(),
                version: "2.0.0".to_string(),
                environment: "development".to_string(),
                ..Default::default()
            },
            network: nestgate_core::unified_types::UnifiedNetworkConfig {
                bind_address: std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
                ..Default::default()
            },
            security: nestgate_core::unified_types::UnifiedSecurityConfig::default(),
            monitoring: nestgate_core::unified_types::UnifiedMonitoringConfig::default(),
            storage: nestgate_core::unified_types::UnifiedStorageConfig::default(),
            memory: nestgate_core::unified_types::UnifiedMemoryConfig::default(),
            extensions: MiddlewareExtensions {
                middleware: MiddlewareSettings {
                    hot_reload: true,
                    ..Default::default()
                },
                security: MiddlewareSecuritySettings::development(),
                performance: MiddlewarePerformanceSettings::development(),
                ..Default::default()
            },
            service_endpoints: HashMap::new(),
            feature_flags: {
                let mut flags = HashMap::new();
                flags.insert("hot_reload".to_string(), true);
                flags.insert("debug_mode".to_string(), true);
                flags
            },
        }
    }

    /// Create production configuration
    fn production_legacy() -> Self {
        Self {
            service: nestgate_core::unified_types::UnifiedServiceConfig {
                name: "nestgate-middleware".to_string(),
                version: "2.0.0".to_string(),
                environment: "production".to_string(),
                ..Default::default()
            },
            network: nestgate_core::unified_types::UnifiedNetworkConfig {
                bind_address: std::net::IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)),
                ..Default::default()
            },
            security: nestgate_core::unified_types::UnifiedSecurityConfig::default(),
            monitoring: nestgate_core::unified_types::UnifiedMonitoringConfig::default(),
            storage: nestgate_core::unified_types::UnifiedStorageConfig::default(),
            memory: nestgate_core::unified_types::UnifiedMemoryConfig::default(),
            extensions: MiddlewareExtensions {
                security: MiddlewareSecuritySettings::production(),
                performance: MiddlewarePerformanceSettings::production(),
                ..Default::default()
            },
            service_endpoints: HashMap::new(),
            feature_flags: {
                let mut flags = HashMap::new();
                flags.insert("hot_reload".to_string(), false);
                flags.insert("debug_mode".to_string(), false);
                flags.insert("production_optimizations".to_string(), true);
                flags
            },
        }
    }
}
