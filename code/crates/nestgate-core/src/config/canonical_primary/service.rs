// **CANONICAL SERVICE CONFIGURATION**
//! Service configuration functionality and utilities.
//! This module provides the unified service configuration for the NestGate system,
//! consolidating all service-related configuration patterns.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// ==================== SERVICE CONFIGURATION ====================

/// Canonical service configuration - THE single source of truth
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    // Core identification
    pub name: String,
    pub version: String,
    pub service_name: String,
    pub enabled: bool,
    
    // Configuration sub-components
    pub service_type: ServiceType,
    pub endpoint_config: ServiceEndpointConfig,
    pub health_config: ServiceHealthConfig,
    pub metrics_config: ServiceMetricsConfig,
    pub scaling_config: ServiceScalingConfig,
    pub security_config: ServiceSecurityConfig,
    pub metadata: HashMap<String, String>,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            name: "nestgate-service".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            service_name: "nestgate-service".to_string(),
            enabled: true,
            service_type: ServiceType::Api,
            endpoint_config: ServiceEndpointConfig::default(),
            health_config: ServiceHealthConfig::default(),
            metrics_config: ServiceMetricsConfig::default(),
            scaling_config: ServiceScalingConfig::default(),
            security_config: ServiceSecurityConfig::default(),
            metadata: HashMap::new(),
        }
    }
}

// ==================== SERVICE TYPES ====================

/// Service types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceType {
    Api,
    Storage,
    Network,
    Security,
    Monitoring,
    Automation,
    Testing,
}

// ==================== ENDPOINT CONFIGURATION ====================

/// Service endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpointConfig {
    pub bind_endpoint: String,
    pub port: u16,
    pub tls_enabled: bool,
    pub max_connections: usize,
    pub keep_alive_timeout: Duration,
}

impl Default for ServiceEndpointConfig {
    fn default() -> Self {
        Self {
            bind_endpoint: "127.0.0.1".to_string(),
            port: 8080,
            tls_enabled: false,
            max_connections: 10000,
            keep_alive_timeout: Duration::from_secs(30),
        }
    }
}

// ==================== HEALTH CONFIGURATION ====================

/// Service health configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealthConfig {
    pub enabled: bool,
    pub check_interval: Duration,
    pub timeout: Duration,
    pub failure_threshold: u32,
}

impl Default for ServiceHealthConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_interval: Duration::from_secs(30),
            timeout: Duration::from_secs(5),
            failure_threshold: 3,
        }
    }
}

// ==================== METRICS CONFIGURATION ====================

/// Service metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMetricsConfig {
    pub enabled: bool,
    pub collection_interval: Duration,
    pub retention_period: Duration,
    pub export_enabled: bool,
}

impl Default for ServiceMetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collection_interval: Duration::from_secs(15),
            retention_period: Duration::from_secs(86400), // 24 hours
            export_enabled: true,
        }
    }
}

// ==================== SCALING CONFIGURATION ====================

/// Service scaling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceScalingConfig {
    pub auto_scaling: bool,
    pub min_instances: usize,
    pub max_instances: usize,
    pub target_cpu_percent: f64,
    pub scale_up_threshold: f64,
    pub scale_down_threshold: f64,
}

impl Default for ServiceScalingConfig {
    fn default() -> Self {
        Self {
            auto_scaling: false,
            min_instances: 1,
            max_instances: 10,
            target_cpu_percent: 70.0,
            scale_up_threshold: 80.0,
            scale_down_threshold: 30.0,
        }
    }
}

// ==================== SECURITY CONFIGURATION ====================

/// Service security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceSecurityConfig {
    pub authentication_required: bool,
    pub authorization_enabled: bool,
    pub rate_limiting: bool,
    pub allowed_origins: Vec<String>,
}

impl Default for ServiceSecurityConfig {
    fn default() -> Self {
        Self {
            authentication_required: true,
            authorization_enabled: true,
            rate_limiting: true,
            allowed_origins: vec!["*".to_string()],
        }
    }
}

// ==================== BACKWARD COMPATIBILITY ALIASES ====================

/// Backward compatibility alias for UnifiedServiceConfig
pub type UnifiedServiceConfig = ServiceConfig;

