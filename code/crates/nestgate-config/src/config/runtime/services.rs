// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Services configuration module  
//!
//! **CAPABILITY-BASED DISCOVERY**: Discovers services by WHAT THEY DO, not WHO THEY ARE.
//!
//! This module implements the capability-based discovery pattern, allowing `NestGate`
//! to discover and integrate with ANY primal offering required capabilities, without
//! hardcoding specific primal names or URLs.
//!
//! # Philosophy
//!
//! - ✅ **Capability-based**: "I need security" → discovers ANY security provider
//! - ❌ **NOT identity-based**: "I need `<specific-primal>`" → vendor lock-in
//!
//! # Example
//!
//! ```ignore
//! use nestgate_core::config::runtime::get_config;
//!
//! let services = &get_config().services;
//!
//! // ✅ CORRECT: Capability-based discovery
//! if let Some(security_url) = services.get_capability_url("security") {
//!     // Connect to ANY primal offering security capability
//!     let client = SecurityClient::new(&security_url)?;
//! }
//! ```

use nestgate_types::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

/// Services configuration for capability-based discovery.
///
/// Discovers services by capability type (security, networking, AI, compute)
/// rather than hardcoded primal names, ensuring zero vendor lock-in.
///
/// # Environment Variables
///
/// **Capability-based** (set one or more as needed):
/// - `NESTGATE_CAPABILITY_SECURITY` - Security capability provider URL
/// - `NESTGATE_CAPABILITY_ORCHESTRATION` - Orchestration provider URL
/// - `NESTGATE_CAPABILITY_NETWORKING` - Networking provider URL
/// - `NESTGATE_CAPABILITY_AI` - AI/intelligence provider URL
/// - `NESTGATE_CAPABILITY_COMPUTE` - Compute provider URL
/// - `NESTGATE_CAPABILITY_ECOSYSTEM` - Ecosystem/OS provider URL
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServicesConfig {
    /// Capability URLs indexed by capability name (e.g., "security", "ai", "compute")
    #[serde(default)]
    pub discovered_capabilities: HashMap<String, String>,

    /// Service discovery enabled (default: true)
    pub discovery_enabled: bool,

    /// Service discovery port (default: 8500)
    pub discovery_port: u16,
}

impl ServicesConfig {
    /// Load services configuration from environment variables.
    ///
    /// Reads `NESTGATE_CAPABILITY_*` variables into [`Self::discovered_capabilities`].
    ///
    /// # Errors
    ///
    /// Currently returns Ok always (no validation failures), but signature
    /// includes Result for future validation additions.
    pub fn from_environment() -> Result<Self> {
        let mut discovered_capabilities = HashMap::new();

        if let Ok(url) = env::var("NESTGATE_CAPABILITY_SECURITY") {
            discovered_capabilities.insert("security".to_string(), url);
        }
        if let Ok(url) = env::var("NESTGATE_CAPABILITY_ORCHESTRATION") {
            discovered_capabilities.insert("orchestration".to_string(), url);
        }
        if let Ok(url) = env::var("NESTGATE_CAPABILITY_NETWORKING") {
            discovered_capabilities.insert("networking".to_string(), url);
        }
        if let Ok(url) = env::var("NESTGATE_CAPABILITY_AI") {
            discovered_capabilities.insert("ai".to_string(), url);
        }
        if let Ok(url) = env::var("NESTGATE_CAPABILITY_COMPUTE") {
            discovered_capabilities.insert("compute".to_string(), url);
        }
        if let Ok(url) = env::var("NESTGATE_CAPABILITY_ECOSYSTEM") {
            discovered_capabilities.insert("ecosystem".to_string(), url);
        }

        Ok(Self {
            discovered_capabilities,
            discovery_enabled: env::var("NESTGATE_DISCOVERY_ENABLED")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(true),
            // ✅ SOVEREIGNTY: Environment-driven discovery port
            discovery_port: env::var("NESTGATE_DISCOVERY_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(8083), // Safe default for discovery service
        })
    }

    /// Get service URL by capability type.
    ///
    /// **CAPABILITY-BASED DISCOVERY**: Use this instead of hardcoded service names.
    /// Discovers services by WHAT THEY DO, not WHO THEY ARE.
    ///
    /// # Arguments
    ///
    /// * `capability` - The capability type (e.g., "security", "ai", "compute")
    ///
    /// # Returns
    ///
    /// The URL of a service providing this capability, or `None` if not configured.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // ✅ CORRECT: Capability-based
    /// let security_url = services.get_capability_url("security");
    /// // Discovers ANY primal offering the security capability
    /// ```
    /// Resolve a service URL by capability — **primary API** for capability-first access.
    ///
    /// Looks up the `discovered_capabilities` map (populated from `NESTGATE_CAPABILITY_*`
    /// at init time).
    #[must_use]
    pub fn resolve_by_capability(&self, capability: &str) -> Option<String> {
        self.discovered_capabilities
            .get(capability)
            .cloned()
            .or_else(|| {
                // Capability aliases for backward-compatible queries
                match capability {
                    "networking" => self.discovered_capabilities.get("orchestration").cloned(),
                    "intelligence" => self.discovered_capabilities.get("ai").cloned(),
                    "os" | "system" => self.discovered_capabilities.get("ecosystem").cloned(),
                    _ => None,
                }
            })
    }

    /// Same as [`Self::resolve_by_capability`] — kept for existing call sites.
    #[must_use]
    pub fn get_capability_url(&self, capability: &str) -> Option<String> {
        self.resolve_by_capability(capability)
    }

    /// Check if a capability is configured.
    ///
    /// Returns `true` if any service providing this capability is known.
    #[must_use]
    pub fn has_capability(&self, capability: &str) -> bool {
        self.resolve_by_capability(capability).is_some()
    }

    /// List all configured capabilities.
    ///
    /// Returns a sorted list of capability types that have providers configured.
    #[must_use]
    pub fn available_capabilities(&self) -> Vec<String> {
        let mut caps: Vec<String> = self.discovered_capabilities.keys().cloned().collect();
        caps.sort();
        caps
    }

    /// Get capability URL with local fallback.
    ///
    /// **CAPABILITY-BASED**: Returns URL for a capability type, falling back
    /// to localhost discovery if not configured.
    ///
    /// This is useful for development where services may be running locally.
    ///
    /// # Primal Sovereignty
    ///
    /// Falls back to environment-configurable localhost endpoints. No hardcoded assumptions.
    #[must_use]
    pub fn capability_url_or_local(&self, capability: &str) -> String {
        use std::env;

        use crate::constants::get_api_port;
        use crate::constants::hardcoding::runtime_fallback_ports;

        let port = match capability {
            "security" => env::var("NESTGATE_SECURITY_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(runtime_fallback_ports::HTTP),
            "networking" | "orchestration" => env::var("NESTGATE_ORCHESTRATION_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(runtime_fallback_ports::METRICS),
            _ => env::var("NESTGATE_API_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or_else(get_api_port),
        };

        // ✅ SOVEREIGNTY: Environment-driven host with compile-time constant fallback
        let host = env::var("NESTGATE_SERVICE_HOST")
            .unwrap_or_else(|_| std::net::Ipv4Addr::LOCALHOST.to_string());

        self.resolve_by_capability(capability)
            .unwrap_or_else(|| format!("http://{host}:{port}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_based_discovery() {
        let mut config = ServicesConfig::default();
        config.discovered_capabilities.insert(
            "security".to_string(),
            "http://security-provider:8080".to_string(),
        );

        assert!(config.has_capability("security"));
        assert_eq!(
            config.get_capability_url("security"),
            Some("http://security-provider:8080".to_string())
        );
    }

    #[test]
    fn test_available_capabilities() {
        let mut config = ServicesConfig::default();
        config
            .discovered_capabilities
            .insert("security".to_string(), "http://sec:8080".to_string());
        config
            .discovered_capabilities
            .insert("ai".to_string(), "http://ai:9000".to_string());

        let caps = config.available_capabilities();
        assert!(caps.contains(&"security".to_string()));
        assert!(caps.contains(&"ai".to_string()));
    }

    #[test]
    fn test_no_hardcoded_primal_names() {
        // This test verifies the capability-based pattern is working
        let config = ServicesConfig::default();

        // Should work with ANY provider, not specific primals
        assert_eq!(config.get_capability_url("security"), None);
        assert_eq!(config.get_capability_url("nonexistent"), None);
    }
}
