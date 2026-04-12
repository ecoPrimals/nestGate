// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(deprecated, reason = "migration to RuntimePortResolver in progress")]
// **CANONICAL SERVICE CONFIGURATION**
//! Service configuration functionality and utilities.
//! This module provides the unified service configuration for the `NestGate` system,
//! consolidating all service-related configuration patterns.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// ==================== SERVICE CONFIGURATION ====================

/// Canonical service configuration - THE single source of truth
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Service
pub struct ServiceConfig {
    // Core identification
    /// Service name identifier
    pub name: String,
    /// Service version
    pub version: String,
    /// Service name for registration
    pub service_name: String,
    /// Whether the service is enabled
    pub enabled: bool,

    // Configuration sub-components
    /// Type of service (API, Storage, Network, etc.)
    pub service_type: ServiceType,
    /// Endpoint configuration
    pub endpoint_config: ServiceEndpointConfig,
    /// Health check configuration
    pub health_config: ServiceHealthConfig,
    /// Metrics collection configuration
    pub metrics_config: ServiceMetricsConfig,
    /// Scaling configuration
    pub scaling_config: ServiceScalingConfig,
    /// Security configuration
    pub security_config: ServiceSecurityConfig,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}

impl Default for ServiceConfig {
    /// Returns the default instance
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
/// Types of Service
pub enum ServiceType {
    /// API service
    Api,
    /// Storage service
    Storage,
    /// Network service
    Network,
    /// Security service
    Security,
    /// Monitoring service
    Monitoring,
    /// Automation service
    Automation,
    /// Testing service
    Testing,
}

// ==================== ENDPOINT CONFIGURATION ====================

/// Service endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `ServiceEndpoint`
pub struct ServiceEndpointConfig {
    /// Bind endpoint address
    pub bind_endpoint: String,
    /// Service port number
    pub port: u16,
    /// Whether TLS is enabled
    pub tls_enabled: bool,
    /// Maximum concurrent connections
    pub max_connections: usize,
    /// Keep-alive timeout duration
    pub keep_alive_timeout: Duration,
}

impl Default for ServiceEndpointConfig {
    /// Returns the default instance
    fn default() -> Self {
        use crate::constants::hardcoding::{addresses, runtime_fallback_ports};
        Self {
            bind_endpoint: addresses::LOCALHOST_IPV4.to_string(),
            port: runtime_fallback_ports::HTTP,
            tls_enabled: false,
            max_connections: 10000,
            keep_alive_timeout: Duration::from_secs(30),
        }
    }
}

// ==================== HEALTH CONFIGURATION ====================

/// Service health configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `ServiceHealth`
pub struct ServiceHealthConfig {
    /// Whether health checks are enabled
    pub enabled: bool,
    /// Interval between health checks
    pub check_interval: Duration,
    /// Health check timeout
    pub timeout: Duration,
    /// Number of failures before marking unhealthy
    pub failure_threshold: u32,
}

impl Default for ServiceHealthConfig {
    /// Returns the default instance
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
/// Configuration for `ServiceMetrics`
pub struct ServiceMetricsConfig {
    /// Whether metrics collection is enabled
    pub enabled: bool,
    /// Interval between metric collections
    pub collection_interval: Duration,
    /// How long to retain metrics
    pub retention_period: Duration,
    /// Whether metric export is enabled
    pub export_enabled: bool,
}

impl Default for ServiceMetricsConfig {
    /// Returns the default instance
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
/// Configuration for `ServiceScaling`
pub struct ServiceScalingConfig {
    /// Whether auto-scaling is enabled
    pub auto_scaling: bool,
    /// Minimum number of instances
    pub min_instances: usize,
    /// Maximum number of instances
    pub max_instances: usize,
    /// Target CPU utilization percentage
    pub target_cpu_percent: f64,
    /// CPU threshold to trigger scale-up
    pub scale_up_threshold: f64,
    /// CPU threshold to trigger scale-down
    pub scale_down_threshold: f64,
}

impl Default for ServiceScalingConfig {
    /// Returns the default instance
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
/// Configuration for `ServiceSecurity`
pub struct ServiceSecurityConfig {
    /// Whether authentication is required
    pub authentication_required: bool,
    /// Whether authorization checks are enabled
    pub authorization_enabled: bool,
    /// Whether rate limiting is enabled
    pub rate_limiting: bool,
    /// List of allowed CORS origins
    pub allowed_origins: Vec<String>,
}

impl Default for ServiceSecurityConfig {
    /// Returns the default instance
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

/// Backward compatibility alias for `UnifiedServiceConfig`
pub type UnifiedServiceConfig = ServiceConfig;
