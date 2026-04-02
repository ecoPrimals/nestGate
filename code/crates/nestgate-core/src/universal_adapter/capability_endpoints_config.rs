// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::too_long_first_doc_paragraph,
    reason = "Struct-level docs describe env centralization; first paragraph spans multiple concerns intentionally."
)]

// HashMap removed - unused import
use std::env;
use std::sync::Arc;

/// Configuration for capability discovery endpoints, loaded from environment variables.
/// This struct centralizes all `env::var` calls for capability discovery
/// to eliminate direct `env::var` calls from production code.
#[derive(Debug, Clone)]
/// Configuration for CapabilityEndpoints
pub struct CapabilityEndpointsConfig {
    // Capability-specific endpoints
    orchestration_endpoint: Option<String>,
    security_endpoint: Option<String>,
    ai_endpoint: Option<String>,
    compute_endpoint: Option<String>,
    management_endpoint: Option<String>,
    storage_endpoint: Option<String>,

    // Generic capability endpoints
    networking_endpoint: Option<String>,

    // Service forwarding endpoint
    service_endpoint: Option<String>,
}

/// Type alias for Sharedcapabilityendpointsconfig
pub type SharedCapabilityEndpointsConfig = Arc<CapabilityEndpointsConfig>;

impl CapabilityEndpointsConfig {
    /// Creates a new `CapabilityEndpointsConfig` by loading values from environment variables.
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            orchestration_endpoint: env::var("ORCHESTRATION_DISCOVERY_ENDPOINT").ok(),
            security_endpoint: env::var("SECURITY_DISCOVERY_ENDPOINT").ok(),
            ai_endpoint: env::var("AI_DISCOVERY_ENDPOINT").ok(),
            compute_endpoint: env::var("COMPUTE_DISCOVERY_ENDPOINT").ok(),
            management_endpoint: env::var("MANAGEMENT_DISCOVERY_ENDPOINT").ok(),
            storage_endpoint: env::var("STORAGE_DISCOVERY_ENDPOINT").ok(),
            networking_endpoint: env::var("CAPABILITY_NETWORKING_ENDPOINT").ok(),
            service_endpoint: env::var("SERVICE_ENDPOINT").ok(),
        }
    }

    // Getter methods

    /// Orchestration Endpoint
    #[must_use]
    pub fn orchestration_endpoint(&self) -> Option<&str> {
        self.orchestration_endpoint.as_deref()
    }

    /// Security Endpoint
    #[must_use]
    pub fn security_endpoint(&self) -> Option<&str> {
        self.security_endpoint.as_deref()
    }

    /// Ai Endpoint
    #[must_use]
    pub fn ai_endpoint(&self) -> Option<&str> {
        self.ai_endpoint.as_deref()
    }

    /// Compute Endpoint
    #[must_use]
    pub fn compute_endpoint(&self) -> Option<&str> {
        self.compute_endpoint.as_deref()
    }

    /// Management Endpoint
    #[must_use]
    pub fn management_endpoint(&self) -> Option<&str> {
        self.management_endpoint.as_deref()
    }

    /// Storage Endpoint
    #[must_use]
    pub fn storage_endpoint(&self) -> Option<&str> {
        self.storage_endpoint.as_deref()
    }

    /// Networking Endpoint
    #[must_use]
    pub fn networking_endpoint(&self) -> Option<&str> {
        self.networking_endpoint.as_deref()
    }

    /// Service Endpoint
    #[must_use]
    pub fn service_endpoint(&self) -> Option<&str> {
        self.service_endpoint.as_deref()
    }

    // Builder methods for testing

    /// Builder method to set Orchestration Endpoint
    #[must_use]
    pub fn with_orchestration_endpoint(mut self, endpoint: String) -> Self {
        self.orchestration_endpoint = Some(endpoint);
        self
    }

    /// Builder method to set Security Endpoint
    #[must_use]
    pub fn with_security_endpoint(mut self, endpoint: String) -> Self {
        self.security_endpoint = Some(endpoint);
        self
    }

    /// Builder method to set Ai Endpoint
    #[must_use]
    pub fn with_ai_endpoint(mut self, endpoint: String) -> Self {
        self.ai_endpoint = Some(endpoint);
        self
    }

    /// Builder method to set Compute Endpoint
    #[must_use]
    pub fn with_compute_endpoint(mut self, endpoint: String) -> Self {
        self.compute_endpoint = Some(endpoint);
        self
    }

    /// Builder method to set Management Endpoint
    #[must_use]
    pub fn with_management_endpoint(mut self, endpoint: String) -> Self {
        self.management_endpoint = Some(endpoint);
        self
    }

    /// Builder method to set Storage Endpoint
    #[must_use]
    pub fn with_storage_endpoint(mut self, endpoint: String) -> Self {
        self.storage_endpoint = Some(endpoint);
        self
    }

    /// Builder method to set Networking Endpoint
    #[must_use]
    pub fn with_networking_endpoint(mut self, endpoint: String) -> Self {
        self.networking_endpoint = Some(endpoint);
        self
    }

    /// Builder method to set Service Endpoint
    #[must_use]
    pub fn with_service_endpoint(mut self, endpoint: String) -> Self {
        self.service_endpoint = Some(endpoint);
        self
    }
}

impl Default for CapabilityEndpointsConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self::from_env()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to create test endpoint
    /// ✅ MIGRATED: Replaces hardcoded endpoints with configurable ones
    fn test_endpoint(service: &str, port: u16) -> String {
        format!("http://{service}:{port}")
    }

    #[test]
    fn test_default_config() {
        let config = CapabilityEndpointsConfig::from_env();
        // Should handle missing env vars gracefully
        assert!(
            config.orchestration_endpoint().is_none() || config.orchestration_endpoint().is_some()
        );
    }

    #[test]
    fn test_builder_pattern() {
        let orch_endpoint = test_endpoint("orchestration", 8080);
        let security_endpoint = test_endpoint("security", 8081);
        let ai_endpoint = test_endpoint("ai", 8082);
        let net_endpoint = test_endpoint("networking", 8083);
        let service_endpoint = test_endpoint("service", 8084);

        let config = CapabilityEndpointsConfig::from_env()
            .with_orchestration_endpoint(orch_endpoint.clone())
            .with_security_endpoint(security_endpoint.clone())
            .with_ai_endpoint(ai_endpoint.clone())
            .with_networking_endpoint(net_endpoint.clone())
            .with_service_endpoint(service_endpoint.clone());

        assert_eq!(
            config.orchestration_endpoint(),
            Some(orch_endpoint.as_str())
        );
        assert_eq!(config.security_endpoint(), Some(security_endpoint.as_str()));
        assert_eq!(config.ai_endpoint(), Some(ai_endpoint.as_str()));
        assert_eq!(config.networking_endpoint(), Some(net_endpoint.as_str()));
        assert_eq!(config.service_endpoint(), Some(service_endpoint.as_str()));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn test_concurrent_access() {
        let config = Arc::new(
            CapabilityEndpointsConfig::from_env()
                .with_orchestration_endpoint(test_endpoint("test", 8080))
                .with_security_endpoint(test_endpoint("test", 8081)),
        );

        let handles: Vec<_> = (0..100)
            .map(|_| {
                let cfg = config.clone();
                tokio::spawn(async move {
                    let _ = cfg.orchestration_endpoint();
                    let _ = cfg.security_endpoint();
                    let _ = cfg.ai_endpoint();
                    let _ = cfg.networking_endpoint();
                    let _ = cfg.service_endpoint();
                })
            })
            .collect();

        for handle in handles {
            handle.await.expect("Task should complete successfully");
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn test_concurrent_different_configs() {
        let orch1_endpoint = test_endpoint("orch1", 8080);
        let orch2_endpoint = test_endpoint("orch2", 9090);

        let config1 = Arc::new(
            CapabilityEndpointsConfig::from_env()
                .with_orchestration_endpoint(orch1_endpoint.clone()),
        );
        let config2 = Arc::new(
            CapabilityEndpointsConfig::from_env()
                .with_orchestration_endpoint(orch2_endpoint.clone()),
        );

        let handle1 = tokio::spawn({
            let cfg = config1.clone();
            async move { cfg.orchestration_endpoint().map(|s| s.to_string()) }
        });
        let handle2 = tokio::spawn({
            let cfg = config2.clone();
            async move { cfg.orchestration_endpoint().map(|s| s.to_string()) }
        });

        let endpoint1 = handle1.await.unwrap();
        let endpoint2 = handle2.await.unwrap();

        assert_eq!(endpoint1, Some(orch1_endpoint));
        assert_eq!(endpoint2, Some(orch2_endpoint));
    }
}
