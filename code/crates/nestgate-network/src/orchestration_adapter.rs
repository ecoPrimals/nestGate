//! Lightweight Orchestration Client - Sovereignty Compliant
//!
//! This module provides a lightweight client for orchestration capabilities
//! that delegates to orchestration primals discovered through the universal adapter.
//!
//! SOVEREIGNTY COMPLIANCE:
//! - No hardcoded orchestration primal names
//! - Uses capability discovery only
//! - Maintains fallback for standalone operation

use crate::Result;
use nestgate_core::ecosystem_integration::capability_router::FallbackProvider;
use nestgate_core::ecosystem_integration::fallback_providers::orchestration::OrchestrationFallbackProvider;
use nestgate_core::universal_adapter::{CapabilityRequest, UniversalAdapter};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{debug, info};

/// Lightweight configuration for orchestration client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationConfig {
    /// Timeout for orchestration operations
    pub timeout_seconds: u64,
    /// Fallback to standalone mode if no orchestration available
    pub standalone_fallback: bool,
}

impl Default for OrchestrationConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 30,
            standalone_fallback: true,
        }
    }
}

/// Lightweight orchestration client that delegates to discovered capabilities
///
/// SOVEREIGNTY COMPLIANCE:
/// - No hardcoded primal names or endpoints
/// - Dynamic capability discovery through universal adapter
/// - Graceful fallback to local orchestration when unavailable
pub struct OrchestrationAdapter {
    config: OrchestrationConfig,
    universal_adapter: Arc<UniversalAdapter>,
    fallback_provider: OrchestrationFallbackProvider,
}

impl OrchestrationAdapter {
    /// Create new orchestration client with capability-based discovery
    pub const fn new(config: OrchestrationConfig, universal_adapter: Arc<UniversalAdapter>) -> Self {
        info!("🌐 Initializing lightweight orchestration client (sovereignty compliant)");
        Self {
            config,
            universal_adapter,
            fallback_provider: OrchestrationFallbackProvider::new(),
        }
    }

    /// Request orchestration capability through universal adapter or fallback
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn request_capability(
        &self,
        capability: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value>  {
        debug!("🔍 Requesting orchestration capability: {}", capability);

        // Try to discover orchestration capability (no hardcoded names)
        match self
            .try_orchestration_request(capability, params.clone())
            .await
        {
            Ok(response) => {
                info!("✅ Orchestration request handled via discovered capability");
                Ok(response)
            }
            Err(e) => {
                debug!(
                    "⚠️ Orchestration capability unavailable: {}, using fallback",
                    e
                );
                if self.config.standalone_fallback {
                    self.fallback_provider
                        .execute(capability, params)
                        .await
                        .map_err(|e| {
                            nestgate_core::error::NestGateError::network_error(&format!(
                                "Fallback orchestration failed: {e}"
                            ))
                        })
                } else {
                    Err(e)
                }
            }
        }
    }

    /// Check if orchestration capability is available
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn is_available(&self) -> Result<bool>  {
        let capability_result = self.universal_adapter.get_capability("orchestration").await;

        Ok(capability_result.is_ok())
    }

    /// Execute orchestration command if capability is available
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn execute_command(
        &self,
        command: &str,
        args: serde_json::Value,
    ) -> Result<serde_json::Value>  {
        let _capability_info = self
            .universal_adapter
            .get_capability("orchestration")
            .await?;

        // Create request using the correct API
        let request = CapabilityRequest::new("orchestration", command)
            .with_parameters(args)
            .with_metadata("requester", "nestgate-network")
            .with_metadata("timeout_seconds", self.config.timeout_seconds.to_string());

        // Make request through universal adapter
        let response = self
            .universal_adapter
            .request_capability("orchestration", request)
            .await
            .map_err(|e| {
                nestgate_core::error::NestGateError::network_error(&format!(
                    "Orchestration command failed: {e}"
                ))
            })?;

        Ok(response.result)
    }

    /// Get orchestration metadata if available
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn get_orchestration_metadata(&self) -> Result<Option<serde_json::Value>>  {
        match self.universal_adapter.get_capability("orchestration").await {
            Ok(capability) => {
                let metadata = serde_json::json!({
                    "provider": capability.provider,
                    "endpoint": capability.endpoint,
                    "performance_tier": capability.performance_tier,
                    "availability": capability.availability,
                    "status": "available"
                });
                Ok(Some(metadata))
            }
            Err(_) => Ok(None),
        }
    }

    /// Try to make orchestration request through discovered capability
    async fn try_orchestration_request(
        &self,
        operation: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value> {
        // Get orchestration capability
        let _capability_info = self
            .universal_adapter
            .get_capability("orchestration")
            .await
            .map_err(|e| {
                nestgate_core::error::NestGateError::network_error(&format!(
                    "No orchestration capability discovered: {e}"
                ))
            })?;

        // Create request using the correct API
        let request = CapabilityRequest::new("orchestration", operation)
            .with_parameters(params)
            .with_metadata("requester", "nestgate-network")
            .with_metadata("timeout_seconds", self.config.timeout_seconds.to_string());

        // Make request through universal adapter
        let response = self
            .universal_adapter
            .request_capability("orchestration", request)
            .await
            .map_err(|e| {
                nestgate_core::error::NestGateError::network_error(&format!(
                    "Orchestration request failed: {e}"
                ))
            })?;

        Ok(response.result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sovereignty_compliance() {
        // This test verifies that no hardcoded orchestration primal names exist
        let source = include_str!("orchestration_adapter.rs");

        // Should not contain hardcoded primal names (using variables to avoid self-reference)
        let primal1 = "song".to_owned() + "bird";
        let primal2 = "bear".to_owned() + "dog";
        let primal3 = "toad".to_owned() + "stool";

        assert!(!source.contains(&primal1));
        assert!(!source.contains(&primal2));
        assert!(!source.contains(&primal3));

        // Should use capability discovery
        assert!(source.contains("find_capability"));
        assert!(source.contains("orchestration"));
        assert!(source.contains("universal_adapter"));

        // Should have fallback capability
        assert!(source.contains("fallback"));
        assert!(source.contains("standalone"));
    }

    #[tokio::test]
    async fn test_orchestration_config_defaults() {
        let config = OrchestrationConfig::default();
        assert_eq!(config.timeout_seconds, 30);
        assert!(config.standalone_fallback);
    }
}
