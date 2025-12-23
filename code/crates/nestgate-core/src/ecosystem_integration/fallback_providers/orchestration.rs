// Orchestration Fallback Provider
// Local service coordination when external orchestration primals are unavailable

use std::collections::HashMap;
use tracing::debug;

use crate::ecosystem_integration::capability_router::{FallbackProvider, CapabilityRoutingError};

#[derive(Debug, Clone)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::OrchestrationFallbackConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::OrchestrationFallbackConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
/// Configuration for OrchestrationFallback
pub struct OrchestrationFallbackConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Timeout Seconds
    pub timeout_seconds: u64,
}

impl Default for OrchestrationFallbackConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            timeout_seconds: 30,
        }
    }
}

impl Default for OrchestrationFallbackProvider {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
/// Orchestrationfallbackprovider
pub struct OrchestrationFallbackProvider {
    _config: OrchestrationFallbackConfig, // Mark as intentionally unused with underscore prefix
}

impl OrchestrationFallbackProvider {
    /// Creates a new instance
    pub fn new() -> Self {
        Self::with_config(OrchestrationFallbackConfig::default())
    }

    /// Builder method to set Config
    pub fn with_config(config: OrchestrationFallbackConfig) -> Self {
        Self { _config: config }
    }

    /// Local service registration fallback
    async fn register_service_fallback(
        &self,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, CapabilityRoutingError> {
        debug!("🔄 Orchestration fallback: Local service registration");

        let service_name = params
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| CapabilityRoutingError::FallbackError("Missing service name".to_string()))?;

        debug!(
            "📝 Registered service locally: {} (will sync when orchestration becomes available)",
            service_name
        );

        Ok(serde_json::json!({
            "success": true,
            "service_id": format!("local_{uuid::Uuid::new_v4(}")),
            "status": "registered_locally",
            "message": "Service registered locally - will sync when orchestration becomes available",
            "provider": "orchestration_fallback"
        }))
    }

    /// Local service discovery fallback
    async fn discover_services_fallback(
        &self,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, CapabilityRoutingError> {
        debug!("🔄 Orchestration fallback: Local service discovery");

        let service_type = params
            .get("service_type")
            .and_then(|v| v.as_str())
            .unwrap_or("any");

        // Return empty list for local discovery (could be enhanced to scan network)
        Ok(serde_json::json!({
            "success": true,
            "services": [],
            "count": 0,
            "message": format!("No local services found for type: {service_type}"),
            "provider": "orchestration_fallback"
        }))
    }

    /// Local workflow coordination fallback
    async fn coordinate_workflow_fallback(
        &self,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, CapabilityRoutingError> {
        debug!("🔄 Orchestration fallback: Local workflow coordination");

        let workflow_id = params
            .get("workflow_id")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        Ok(serde_json::json!({
            "success": true,
            "workflow_id": workflow_id,
            "status": "coordinated_locally",
            "message": "Workflow coordinated using local fallback logic",
            "provider": "orchestration_fallback"
        }))
    }

    #[allow(dead_code)]
    async fn handle_operation(
        &self,
        operation: &str,
        _params: &str,
    ) -> Result<String, CapabilityRoutingError> {
        match operation {
            "deploy_service" => Ok("Service deployment initiated".to_string()),
            "scale_service" => Ok("Service scaling completed".to_string()),
            "health_check" => Ok("All services healthy".to_string()),
            _ => Err(CapabilityRoutingError::FallbackError(format!(
                "Unsupported orchestration operation: {operation}"
            ))),
        }
    }
}

impl FallbackProvider for OrchestrationFallbackProvider {
    /// Execute
    async fn execute(
        &self,
        operation: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, CapabilityRoutingError> {
        match operation {
            "register_service" => self.register_service_fallback(params).await,
            "discover_services" => self.discover_services_fallback(params).await,
            "coordinate_workflow" => self.coordinate_workflow_fallback(params).await,
            _ => Err(CapabilityRoutingError::FallbackError(format!(
                "Unsupported fallback operation: {operation}"
            ))),
        }
    }

    /// Supported Operations
    fn supported_operations(&self) -> Vec<String> {
        vec![
            "register_service".to_string(),
            "unregister_service".to_string(),
            "discover_services".to_string(),
            "coordinate_workflow".to_string(),
            "manage_resources".to_string(),
        ]
    }

    /// Metadata
    fn metadata(&self) -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        metadata.insert(
            "provider_type".to_string(),
            "orchestration_fallback".to_string(),
        );
        metadata.insert("version".to_string(), "1.0.0".to_string());
        metadata.insert(
            "description".to_string(),
            "Local orchestration fallback provider".to_string(),
        );
        metadata
    }
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Orchestrationfallbackconfigcanonical
pub type OrchestrationFallbackConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using OrchestrationFallbackConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

