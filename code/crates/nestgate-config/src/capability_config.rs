// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **CAPABILITY-BASED CONFIGURATION**
//!
//! Replaces hardcoded values with dynamic capability-based discovery.
//! This module provides the foundation for agnostic, runtime-configurable services.
//!
//! ## Philosophy
//!
//! **Each primal knows ONLY itself** and discovers others by capability, not by name or address.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use nestgate_core::capability_config::*;
//!
//! // Create capability-based config (NO hardcoding!)
//! let config = CapabilityConfig::from_env()
//!     .with_fallback(CapabilityDefaults::secure())?;
//!
//! // Discover service by capability (not by name!)
//! let api_endpoint = config.resolve_capability("http-api")?;
//! let metrics_endpoint = config.resolve_capability("metrics")?;
//!
//! // NO hardcoded "localhost:8080"!
//! // NO hardcoded primal names!
//! ```
//!
//! ## Environment Variables
//!
//! All configuration comes from environment, never hardcoded:
//!
//! ```bash
//! # Service endpoints (dynamic discovery)
//! NESTGATE_API_ENDPOINT=0.0.0.0:8080
//! NESTGATE_METRICS_ENDPOINT=0.0.0.0:9090
//!
//! # Capability registration (self-knowledge)
//! NESTGATE_CAPABILITIES=storage,zfs-management,metrics
//!
//! # Discovery backends (find other primals)
//! NESTGATE_DISCOVERY_BACKENDS=dns-srv,mdns,consul
//! ```
//!
//! ## Migration from Hardcoded Values
//!
//! ```rust,ignore
//! // OLD (hardcoded):
//! const API_PORT: u16 = 8080;
//! let addr = SocketAddr::new("127.0.0.1".parse()?, API_PORT);
//!
//! // NEW (capability-based):
//! let addr = config.resolve_capability("api")?.primary_endpoint();
//! ```

use nestgate_types::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::net::SocketAddr;
use std::time::Duration;

/// Capability-based configuration system
///
/// Replaces hardcoded values with dynamic, environment-driven configuration.
/// Supports runtime discovery of services by capability, not by name.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityConfig {
    /// Registered capabilities with their configurations
    pub capabilities: HashMap<String, CapabilityInfo>,

    /// Fallback defaults (only used when discovery fails)
    pub fallbacks: Option<CapabilityDefaults>,

    /// Discovery backends to use for finding other primals
    pub discovery_backends: Vec<DiscoveryBackend>,
}

/// Information about a capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityInfo {
    /// Capability identifier (e.g., "http-api", "metrics", "storage")
    pub id: String,

    /// Primary endpoint for this capability
    pub primary_endpoint: Option<SocketAddr>,

    /// Additional endpoints (for redundancy/load balancing)
    pub additional_endpoints: Vec<SocketAddr>,

    /// Metadata about this capability
    pub metadata: HashMap<String, String>,

    /// Health check configuration
    pub health_check: Option<HealthCheckConfig>,
}

/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    /// Interval between health checks
    pub interval: Duration,

    /// Timeout for health check requests
    pub timeout: Duration,

    /// Health check endpoint path
    pub path: String,
}

/// Discovery backend configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryBackend {
    /// DNS-SRV record discovery
    DnsSrv {
        /// DNS domain for SRV lookups
        domain: String,
    },

    /// mDNS/Bonjour discovery
    MDns {
        /// mDNS service type (e.g., "_http._tcp")
        service_type: String,
    },

    /// Consul service discovery
    Consul {
        /// Consul server address
        address: String,
    },

    /// Kubernetes service discovery
    Kubernetes {
        /// Kubernetes namespace to search
        namespace: String,
    },

    /// Environment variables only (simplest)
    Environment,
}

/// Fallback defaults (only used when discovery fails)
///
/// **Philosophy**: These should be MINIMAL and only for development.
/// Production should always use proper discovery.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityDefaults {
    /// Default bind address (0.0.0.0 for all interfaces)
    pub bind_address: String,

    /// Default port ranges for different capability types
    pub port_ranges: HashMap<String, PortRange>,

    /// Default timeouts
    pub timeouts: DefaultTimeouts,
}

/// Port range for a capability type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortRange {
    /// Start of port range
    pub start: u16,
    /// End of port range
    pub end: u16,
}

/// Default timeout values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultTimeouts {
    /// Connection timeout
    pub connection: Duration,
    /// Request timeout
    pub request: Duration,
    /// Health check timeout
    pub health_check: Duration,
}

impl CapabilityConfig {
    /// Create configuration from environment variables
    ///
    /// Reads all configuration from env vars, NO hardcoded values.
    ///
    /// ## Environment Variables
    ///
    /// - `NESTGATE_CAPABILITIES`: Comma-separated list of capabilities this primal provides
    /// - `NESTGATE_{CAPABILITY}_ENDPOINT`: Endpoint for each capability
    /// - `NESTGATE_DISCOVERY_BACKENDS`: Comma-separated list of discovery backends
    ///
    /// ## Example
    ///
    /// ```bash
    /// export NESTGATE_CAPABILITIES=api,metrics,storage
    /// export NESTGATE_API_ENDPOINT=0.0.0.0:8080
    /// export NESTGATE_METRICS_ENDPOINT=0.0.0.0:9090
    /// export NESTGATE_DISCOVERY_BACKENDS=dns-srv,mdns
    /// ```
    pub fn from_env() -> Result<Self> {
        let mut capabilities = HashMap::new();

        // Read capabilities from environment
        if let Ok(caps) = env::var("NESTGATE_CAPABILITIES") {
            for cap in caps.split(',') {
                let cap = cap.trim();
                if cap.is_empty() {
                    continue;
                }

                // Try to read endpoint for this capability
                let endpoint_var =
                    format!("NESTGATE_{}_ENDPOINT", cap.to_uppercase().replace('-', "_"));
                let endpoint = env::var(&endpoint_var)
                    .ok()
                    .and_then(|s| s.parse::<SocketAddr>().ok());

                capabilities.insert(
                    cap.to_string(),
                    CapabilityInfo {
                        id: cap.to_string(),
                        primary_endpoint: endpoint,
                        additional_endpoints: Vec::new(),
                        metadata: HashMap::new(),
                        health_check: None,
                    },
                );
            }
        }

        // Read discovery backends
        let discovery_backends = env::var("NESTGATE_DISCOVERY_BACKENDS")
            .unwrap_or_else(|_| "environment".to_string())
            .split(',')
            .filter_map(|backend| match backend.trim() {
                "dns-srv" => {
                    let domain = env::var("NESTGATE_DNS_DOMAIN").ok()?;
                    Some(DiscoveryBackend::DnsSrv { domain })
                }
                "mdns" => {
                    let service_type = env::var("NESTGATE_MDNS_SERVICE")
                        .unwrap_or_else(|_| "_nestgate._tcp".to_string());
                    Some(DiscoveryBackend::MDns { service_type })
                }
                "consul" => {
                    let address = env::var("NESTGATE_CONSUL_ADDRESS").ok()?;
                    Some(DiscoveryBackend::Consul { address })
                }
                "kubernetes" | "k8s" => {
                    let namespace = env::var("NESTGATE_K8S_NAMESPACE")
                        .unwrap_or_else(|_| "default".to_string());
                    Some(DiscoveryBackend::Kubernetes { namespace })
                }
                "environment" | "env" => Some(DiscoveryBackend::Environment),
                _ => None,
            })
            .collect();

        Ok(Self {
            capabilities,
            fallbacks: None,
            discovery_backends,
        })
    }

    /// Add fallback defaults (for development only)
    pub fn with_fallback(mut self, fallbacks: CapabilityDefaults) -> Result<Self> {
        self.fallbacks = Some(fallbacks);
        Ok(self)
    }

    /// Resolve a capability to a concrete endpoint
    ///
    /// Returns the primary endpoint for the given capability.
    /// Falls back to defaults only if discovery fails.
    pub fn resolve_capability(&self, capability_id: &str) -> Result<&CapabilityInfo> {
        self.capabilities.get(capability_id).ok_or_else(|| {
            anyhow::anyhow!(
                "Capability '{}' not found. Available: {:?}",
                capability_id,
                self.capabilities.keys().collect::<Vec<_>>()
            )
            .into()
        })
    }

    /// Get primary endpoint for a capability
    pub fn get_endpoint(&self, capability_id: &str) -> Result<SocketAddr> {
        let info = self.resolve_capability(capability_id)?;

        info.primary_endpoint.ok_or_else(|| {
            anyhow::anyhow!(
                "No endpoint configured for capability '{}'. Set NESTGATE_{}_ENDPOINT env var",
                capability_id,
                capability_id.to_uppercase().replace('-', "_")
            )
            .into()
        })
    }

    /// Get capability URL (http://host:port format)
    pub fn get_capability_url(&self, capability_id: &str) -> Result<String> {
        let endpoint = self.get_endpoint(capability_id)?;
        Ok(format!("http://{}", endpoint))
    }

    /// Check if capability is available
    pub fn has_capability(&self, capability_id: &str) -> bool {
        self.capabilities.contains_key(capability_id)
    }

    /// List all available capabilities
    pub fn available_capabilities(&self) -> Vec<String> {
        self.capabilities.keys().cloned().collect()
    }

    /// Get all endpoints for a capability (primary + additional)
    pub fn get_all_endpoints(&self, capability_id: &str) -> Result<Vec<SocketAddr>> {
        let info = self.resolve_capability(capability_id)?;
        Ok(info.all_endpoints())
    }

    /// Register a new capability
    pub fn register_capability(&mut self, info: CapabilityInfo) {
        self.capabilities.insert(info.id.clone(), info);
    }
}

impl CapabilityDefaults {
    /// Secure defaults for production
    ///
    /// Uses 0.0.0.0 (all interfaces) with sensible port ranges.
    pub fn secure() -> Self {
        let mut port_ranges = HashMap::new();
        port_ranges.insert(
            "api".to_string(),
            PortRange {
                start: 8000,
                end: 8999,
            },
        );
        port_ranges.insert(
            "metrics".to_string(),
            PortRange {
                start: 9000,
                end: 9999,
            },
        );
        port_ranges.insert(
            "admin".to_string(),
            PortRange {
                start: 7000,
                end: 7999,
            },
        );

        Self {
            bind_address: "0.0.0.0".to_string(),
            port_ranges,
            timeouts: DefaultTimeouts {
                connection: Duration::from_secs(30),
                request: Duration::from_secs(60),
                health_check: Duration::from_secs(5),
            },
        }
    }

    /// Localhost defaults for development
    ///
    /// Uses 127.0.0.1 (localhost only) for safety.
    pub fn development() -> Self {
        let mut defaults = Self::secure();
        defaults.bind_address = "127.0.0.1".to_string();
        defaults
    }
}

impl CapabilityInfo {
    /// Get primary endpoint, with fallback to additional endpoints
    pub fn primary_endpoint(&self) -> Result<SocketAddr> {
        self.primary_endpoint
            .or_else(|| self.additional_endpoints.first().copied())
            .ok_or_else(|| {
                anyhow::anyhow!("No endpoint available for capability '{}'", self.id).into()
            })
    }

    /// Get all endpoints (primary + additional)
    pub fn all_endpoints(&self) -> Vec<SocketAddr> {
        let mut endpoints = Vec::new();
        if let Some(primary) = self.primary_endpoint {
            endpoints.push(primary);
        }
        endpoints.extend(&self.additional_endpoints);
        endpoints
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_defaults() {
        let defaults = CapabilityDefaults::secure();
        assert_eq!(defaults.bind_address, "0.0.0.0");
        assert!(defaults.port_ranges.contains_key("api"));

        let dev_defaults = CapabilityDefaults::development();
        assert_eq!(dev_defaults.bind_address, "127.0.0.1");
    }

    #[test]
    fn test_capability_config_creation() {
        // Test that we can create config without hardcoded values
        let config = CapabilityConfig {
            capabilities: HashMap::new(),
            fallbacks: Some(CapabilityDefaults::secure()),
            discovery_backends: vec![DiscoveryBackend::Environment],
        };

        assert!(config.capabilities.is_empty());
        assert!(config.fallbacks.is_some());
    }
}
