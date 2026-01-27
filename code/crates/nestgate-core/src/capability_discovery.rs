// Copyright (c) 2026 NestGate
//
//! # Capability-Based Primal Discovery
//!
//! **Deep Debt Solution**: Replace hardcoded primal names with runtime capability discovery.
//!
//! ## Problem
//!
//! Previously, NestGate hardcoded primal names like "beardog", "songbird", etc. This violates:
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
//! Songbird IPC Service (registry)
//!   ↓
//! Returns: ServiceEndpoint
//!   ↓
//! NestGate connects to crypto service
//!   (Could be BearDog, or any crypto provider!)
//! ```
//!
//! ## Usage
//!
//! ```rust,ignore
//! use nestgate_core::capability_discovery::CapabilityDiscovery;
//!
//! // Bootstrap: Discover Songbird IPC service first
//! let songbird = CapabilityDiscovery::discover_songbird_ipc().await?;
//!
//! // Create discovery client
//! let discovery = CapabilityDiscovery::new(songbird);
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

use crate::error::{NestGateError, Result};
use crate::rpc::JsonRpcClient;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[cfg(test)]
mod tests;

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
    pub fn from_response(value: Value, capability: &str) -> Result<Self> {
        let name = value["name"]
            .as_str()
            .ok_or_else(|| NestGateError::api_error("Missing service name in response"))?
            .to_string();
        
        let endpoint = value["endpoint"]
            .as_str()
            .ok_or_else(|| NestGateError::api_error("Missing endpoint in response"))?
            .to_string();
        
        let version = value["version"]
            .as_str()
            .unwrap_or("unknown")
            .to_string();
        
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
/// Uses Songbird IPC service for runtime resolution.
pub struct CapabilityDiscovery {
    /// Songbird IPC client for discovery queries
    songbird: JsonRpcClient,
    
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
    /// * `songbird` - Connected Songbird IPC client
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let songbird = CapabilityDiscovery::discover_songbird_ipc().await?;
    /// let discovery = CapabilityDiscovery::new(songbird);
    /// ```
    pub fn new(songbird: JsonRpcClient) -> Self {
        Self {
            songbird,
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
    /// - Songbird IPC is unavailable
    /// - Network communication fails
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // Discover crypto service (could be BearDog or any crypto provider)
    /// let crypto = discovery.find("crypto").await?;
    ///
    /// // Discover HTTP service (should be Songbird)
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
            } else {
                // Cache expired, remove
                self.cache.remove(capability);
                tracing::debug!(
                    capability = capability,
                    age_ms = age.as_millis(),
                    "Capability discovery cache expired"
                );
            }
        }
        
        // Query Songbird IPC service
        tracing::info!(
            capability = capability,
            "Discovering service by capability via Songbird IPC"
        );
        
        let response = self.songbird
            .call(
                "ipc.find_capability",
                json!({ "capability": capability })
            )
            .await
            .map_err(|e| {
                NestGateError::service_unavailable(&format!(
                    "Failed to discover capability '{}': {}",
                    capability, e
                ))
            })?;
        
        // Parse response
        let services = response["services"]
            .as_array()
            .ok_or_else(|| {
                NestGateError::api_error("Invalid response format: expected 'services' array")
            })?;
        
        if services.is_empty() {
            return Err(NestGateError::service_unavailable(&format!(
                "No service provides capability '{}'",
                capability
            )));
        }
        
        // Take first service (TODO: support multiple providers with load balancing)
        let service_value = &services[0];
        let endpoint = ServiceEndpoint::from_response(service_value.clone(), capability)?;
        
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
    
    /// Discover Songbird IPC service itself
    ///
    /// **Special case**: Songbird is the bootstrap service that enables
    /// discovery of all other services. It must be discovered first via
    /// environment variables or standard paths.
    ///
    /// # Discovery Order
    ///
    /// 1. Environment variable `SONGBIRD_IPC_PATH`
    /// 2. Standard Unix socket path `/primal/songbird`
    /// 3. TCP via `SONGBIRD_HOST` and `SONGBIRD_PORT` env vars
    /// 4. Default TCP `localhost:8080`
    ///
    /// # Returns
    ///
    /// Connected Songbird IPC client.
    ///
    /// # Errors
    ///
    /// Returns error if Songbird IPC cannot be discovered or connected.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // Discover Songbird IPC service
    /// let songbird = CapabilityDiscovery::discover_songbird_ipc().await?;
    ///
    /// // Use for capability discovery
    /// let discovery = CapabilityDiscovery::new(songbird);
    /// ```
    pub async fn discover_songbird_ipc() -> Result<JsonRpcClient> {
        tracing::info!("Discovering Songbird IPC service (bootstrap)");
        
        // Try environment variable first
        if let Ok(path) = env::var("SONGBIRD_IPC_PATH") {
            tracing::debug!(path = path, "Trying Songbird IPC path from environment");
            if Path::new(&path).exists() {
                match JsonRpcClient::connect_unix(&path).await {
                    Ok(client) => {
                        tracing::info!(path = path, "Connected to Songbird IPC via environment path");
                        return Ok(client);
                    }
                    Err(e) => {
                        tracing::warn!(
                            path = path,
                            error = %e,
                            "Failed to connect to Songbird IPC via environment path"
                        );
                    }
                }
            }
        }
        
        // Try standard Unix socket path
        let standard_path = "/primal/songbird";
        tracing::debug!(path = standard_path, "Trying standard Songbird IPC path");
        if Path::new(standard_path).exists() {
            match JsonRpcClient::connect_unix(standard_path).await {
                Ok(client) => {
                    tracing::info!(path = standard_path, "Connected to Songbird IPC via standard path");
                    return Ok(client);
                }
                Err(e) => {
                    tracing::warn!(
                        path = standard_path,
                        error = %e,
                        "Failed to connect to Songbird IPC via standard path"
                    );
                }
            }
        }
        
        // Try TCP via environment
        let host = env::var("SONGBIRD_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = env::var("SONGBIRD_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(8080);
        
        tracing::debug!(
            host = host,
            port = port,
            "Trying Songbird IPC via TCP"
        );
        
        // TODO: Implement TCP connection for Songbird IPC
        // For now, return error directing to Unix socket setup
        Err(NestGateError::service_unavailable(
            "Songbird IPC not found. Ensure Songbird is running and accessible via:\n\
             1. Unix socket at /primal/songbird, OR\n\
             2. Environment variable SONGBIRD_IPC_PATH, OR\n\
             3. TCP at SONGBIRD_HOST:SONGBIRD_PORT"
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


