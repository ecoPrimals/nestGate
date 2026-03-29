// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// Copyright (c) 2026 NestGate
//
//! # Capability-Based Primal Discovery
//!
//! **Deep Debt Solution**: Replace hardcoded primal names with runtime capability discovery.
//!
//! ## Problem
//!
//! Previously, `NestGate` hardcoded primal names like "beardog", "songbird", etc. This violates:
//! - Primal autonomy (primals shouldn't know each other's names)
//! - Self-knowledge principle (only know capabilities, discover at runtime)
//! - Inter-primal interaction standards
//!
//! ## Solution
//!
//! Discover services by **capability** (what they do), not by **name** (who they are).
//!
//! ## Architecture
//!
//! ```text
//! NestGate (needs crypto)
//!   ↓
//! CapabilityDiscovery::find("crypto")
//!   ↓
//! Orchestration IPC gateway (capability registry)
//!   ↓
//! Returns: ServiceEndpoint
//!   ↓
//! NestGate connects to crypto service
//!   (Any provider that advertises the capability!)
//! ```
//!
//! ## Usage
//!
//! ```rust,ignore
//! use nestgate_core::capability_discovery::CapabilityDiscovery;
//!
//! // Bootstrap: Discover orchestration IPC gateway first
//! let ipc = CapabilityDiscovery::discover_orchestration_ipc().await?;
//!
//! // Create discovery client
//! let discovery = CapabilityDiscovery::new(ipc);
//!
//! // Discover services by capability
//! let crypto = discovery.find("crypto").await?;
//! let http = discovery.find("http").await?;
//! let storage = discovery.find("storage").await?;
//!
//! // Use discovered services
//! let response = crypto.call_rpc("crypto.generate_keypair", params).await?;
//! ```
//!
//! ## Benefits
//!
//! - ✅ Primal autonomy: No hardcoded names
//! - ✅ Self-knowledge: Only know own capabilities
//! - ✅ Runtime discovery: Flexible service providers
//! - ✅ Testing: Easy to mock capabilities
//! - ✅ Ecosystem compliant: Standards-conforming

use dashmap::DashMap;
use nestgate_types::error::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::env;
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub use nestgate_rpc::JsonRpcClient;

/// Service endpoint discovered by capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    /// Capability this service provides
    pub capability: String,

    /// Service name (for logging only, NOT for discovery!)
    pub name: String,

    /// Connection endpoint (Unix socket path or TCP address)
    pub endpoint: String,

    /// Service version
    pub version: String,

    /// When this endpoint was discovered
    #[serde(skip, default = "Instant::now")]
    pub discovered_at: Instant,
}

impl ServiceEndpoint {
    /// Create from JSON-RPC response
    pub fn from_response(value: &Value, capability: &str) -> Result<Self> {
        let name = value["name"]
            .as_str()
            .ok_or_else(|| NestGateError::api_error("Missing service name in response"))?
            .to_string();

        let endpoint = value["endpoint"]
            .as_str()
            .ok_or_else(|| NestGateError::api_error("Missing endpoint in response"))?
            .to_string();

        let version = value["version"].as_str().unwrap_or("unknown").to_string();

        Ok(Self {
            capability: capability.to_string(),
            name,
            endpoint,
            version,
            discovered_at: Instant::now(),
        })
    }
}

/// Capability-based primal discovery client
///
/// Discovers services by capability, not by hardcoded names.
/// Uses the orchestration IPC gateway for runtime resolution.
pub struct CapabilityDiscovery {
    /// IPC client for discovery queries (orchestration / registry)
    ipc_gateway: JsonRpcClient,

    /// Cache of discovered capabilities (for performance)
    cache: Arc<DashMap<String, ServiceEndpoint>>,

    /// Cache TTL (time-to-live)
    cache_ttl: Duration,
}

impl CapabilityDiscovery {
    /// Create new capability discovery client
    ///
    /// # Arguments
    ///
    /// * `ipc_gateway` - Connected orchestration IPC client
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let ipc = CapabilityDiscovery::discover_orchestration_ipc().await?;
    /// let discovery = CapabilityDiscovery::new(ipc);
    /// ```
    pub fn new(ipc_gateway: JsonRpcClient) -> Self {
        Self {
            ipc_gateway,
            cache: Arc::new(DashMap::new()),
            cache_ttl: Duration::from_secs(300), // 5 minutes
        }
    }

    /// Discover a service providing a specific capability
    ///
    /// # Arguments
    ///
    /// * `capability` - The capability to find (e.g., "crypto", "http", "storage")
    ///
    /// # Returns
    ///
    /// Service endpoint that provides the requested capability.
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - No service provides the capability
    /// - The orchestration IPC gateway is unavailable
    /// - Network communication fails
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // Discover crypto service (any provider advertising "crypto")
    /// let crypto = discovery.find("crypto").await?;
    ///
    /// // Discover HTTP proxy capability (orchestration layer)
    /// let http = discovery.find("http").await?;
    /// ```
    pub async fn find(&mut self, capability: &str) -> Result<ServiceEndpoint> {
        // Check cache first
        if let Some(entry) = self.cache.get(capability) {
            let age = entry.discovered_at.elapsed();
            if age < self.cache_ttl {
                tracing::debug!(
                    capability = capability,
                    age_ms = age.as_millis(),
                    "Capability discovery cache hit"
                );
                return Ok(entry.clone());
            }
            // Cache expired, remove
            self.cache.remove(capability);
            tracing::debug!(
                capability = capability,
                age_ms = age.as_millis(),
                "Capability discovery cache expired"
            );
        }

        // Query orchestration IPC gateway
        tracing::info!(
            capability = capability,
            "Discovering service by capability via orchestration IPC"
        );

        let response = self
            .ipc_gateway
            .call("ipc.find_capability", json!({ "capability": capability }))
            .await
            .map_err(|e| {
                NestGateError::service_unavailable(format!(
                    "Failed to discover capability '{capability}': {e}"
                ))
            })?;

        // Parse response
        let services = response["services"].as_array().ok_or_else(|| {
            NestGateError::api_error("Invalid response format: expected 'services' array")
        })?;

        if services.is_empty() {
            return Err(NestGateError::service_unavailable(format!(
                "No service provides capability '{capability}'"
            )));
        }

        // Take first service (FUTURE: load balancing across multiple providers in v0.12+)
        let endpoint = ServiceEndpoint::from_response(&services[0], capability)?;

        tracing::info!(
            capability = capability,
            service = endpoint.name,
            endpoint = endpoint.endpoint,
            version = endpoint.version,
            "Discovered service by capability"
        );

        // Cache for future use
        self.cache.insert(capability.to_string(), endpoint.clone());

        Ok(endpoint)
    }

    /// Discover the orchestration IPC gateway (bootstrap registry client)
    ///
    /// **Special case**: The orchestration layer is the bootstrap service that enables
    /// discovery of all other services. It must be reached via environment variables
    /// or standard paths.
    ///
    /// # Discovery Order
    ///
    /// 1. `NESTGATE_ORCHESTRATION_IPC_PATH` or `ORCHESTRATION_IPC_PATH`
    /// 2. Standard Unix socket from `ORCHESTRATION_IPC_STANDARD_PATH` (default: `/primal/orchestration`)
    /// 3. TCP bootstrap via `ORCHESTRATION_HOST` / `ORCHESTRATION_PORT`
    ///
    /// # Returns
    ///
    /// Connected JSON-RPC client to the IPC gateway.
    ///
    /// # Errors
    ///
    /// Returns error if the orchestration IPC endpoint cannot be discovered or connected.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let ipc = CapabilityDiscovery::discover_orchestration_ipc().await?;
    /// let discovery = CapabilityDiscovery::new(ipc);
    /// ```
    pub async fn discover_orchestration_ipc() -> Result<JsonRpcClient> {
        tracing::info!("Discovering orchestration IPC gateway (bootstrap)");

        // Unix socket path candidates (env-driven)
        let path_candidates = [
            env::var("NESTGATE_ORCHESTRATION_IPC_PATH").ok(),
            env::var("ORCHESTRATION_IPC_PATH").ok(),
        ];
        for path in path_candidates.into_iter().flatten() {
            tracing::debug!(
                path = path,
                "Trying orchestration IPC path from environment"
            );
            if Path::new(&path).exists() {
                match JsonRpcClient::connect_unix(&path).await {
                    Ok(client) => {
                        tracing::info!(
                            path = path,
                            "Connected to orchestration IPC via environment path"
                        );
                        return Ok(client);
                    }
                    Err(e) => {
                        tracing::warn!(
                            path = path,
                            error = %e,
                            "Failed to connect to orchestration IPC via environment path"
                        );
                    }
                }
            }
        }

        let standard_path = env::var("ORCHESTRATION_IPC_STANDARD_PATH")
            .unwrap_or_else(|_| "/primal/orchestration".to_string());
        tracing::debug!(path = %standard_path, "Trying standard IPC path");
        if Path::new(&standard_path).exists() {
            match JsonRpcClient::connect_unix(&standard_path).await {
                Ok(client) => {
                    tracing::info!(
                        path = %standard_path,
                        "Connected to IPC gateway via standard path"
                    );
                    return Ok(client);
                }
                Err(e) => {
                    tracing::warn!(
                        path = %standard_path,
                        error = %e,
                        "Failed to connect to IPC gateway via standard path"
                    );
                }
            }
        }

        let host = env::var("ORCHESTRATION_HOST")
            .or_else(|_| env::var("NESTGATE_DEV_HOST"))
            .or_else(|_| env::var("NESTGATE_DISCOVERY_FALLBACK_HOST"))
            .unwrap_or_else(|_| {
                tracing::warn!(
                    "Orchestration TCP bootstrap: ORCHESTRATION_HOST, NESTGATE_DEV_HOST, \
                     and NESTGATE_DISCOVERY_FALLBACK_HOST unset; using `localhost`."
                );
                "localhost".to_string()
            });
        let port = env::var("ORCHESTRATION_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(|| {
                env::var("NESTGATE_API_PORT")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or_else(nestgate_config::constants::get_api_port)
            });

        tracing::debug!(host = host, port = port, "Trying orchestration IPC via TCP");

        Err(NestGateError::service_unavailable(
            "IPC gateway not found. Ensure the orchestration service is running and accessible via:\n\
             1. NESTGATE_ORCHESTRATION_IPC_PATH or ORCHESTRATION_IPC_PATH (Unix socket path), OR\n\
             2. ORCHESTRATION_IPC_STANDARD_PATH (default: /primal/orchestration), OR\n\
             3. TCP at ORCHESTRATION_HOST:ORCHESTRATION_PORT",
        ))
    }

    /// Clear the capability discovery cache
    ///
    /// Useful for testing or when services have changed.
    pub fn clear_cache(&self) {
        self.cache.clear();
        tracing::debug!("Capability discovery cache cleared");
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> CacheStats {
        CacheStats {
            size: self.cache.len(),
            ttl_seconds: self.cache_ttl.as_secs(),
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Number of cached entries
    pub size: usize,

    /// Cache TTL in seconds
    pub ttl_seconds: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn service_endpoint_from_response_ok() {
        let v = json!({
            "name": "alpha",
            "endpoint": "/tmp/sock",
            "version": "2.0.0"
        });
        let ep = ServiceEndpoint::from_response(&v, "crypto").expect("parse");
        assert_eq!(ep.capability, "crypto");
        assert_eq!(ep.name, "alpha");
        assert_eq!(ep.endpoint, "/tmp/sock");
        assert_eq!(ep.version, "2.0.0");
    }

    #[test]
    fn service_endpoint_from_response_default_version() {
        let v = json!({
            "name": "beta",
            "endpoint": "tcp://127.0.0.1:1"
        });
        let ep = ServiceEndpoint::from_response(&v, "http").expect("parse");
        assert_eq!(ep.version, "unknown");
    }

    #[test]
    fn service_endpoint_from_response_missing_name_errors() {
        let v = json!({ "endpoint": "e" });
        assert!(ServiceEndpoint::from_response(&v, "x").is_err());
    }

    #[test]
    fn service_endpoint_from_response_missing_endpoint_errors() {
        let v = json!({ "name": "n" });
        assert!(ServiceEndpoint::from_response(&v, "x").is_err());
    }

    #[test]
    fn cache_stats_clone() {
        let s = CacheStats {
            size: 3,
            ttl_seconds: 120,
        };
        assert_eq!(s.clone().size, 3);
    }
}
