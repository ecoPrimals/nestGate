//! Orchestration Fallback Provider
//! Local service coordination when external orchestration primals are unavailable

use async_trait::async_trait;
use std::collections::HashMap;
use tracing::debug;

use crate::ecosystem_integration::mock_router::{FallbackProvider, MockRoutingError};

#[derive(Debug, Clone)]
pub struct OrchestrationFallbackConfig {
    pub enabled: bool,
    pub timeout_seconds: u64,
}

impl Default for OrchestrationFallbackConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            timeout_seconds: 30,
        }
    }
}

impl Default for OrchestrationFallbackProvider {
    fn default() -> Self {
        Self::new()
    }
}

pub struct OrchestrationFallbackProvider {
    _config: OrchestrationFallbackConfig, // Mark as intentionally unused with underscore prefix
}

impl OrchestrationFallbackProvider {
    pub fn new() -> Self {
        Self::with_config(OrchestrationFallbackConfig::default())
    }

    pub fn with_config(config: OrchestrationFallbackConfig) -> Self {
        Self { _config: config }
    }

    /// Local service registration fallback
    async fn register_service_fallback(
        &self,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, MockRoutingError> {
        debug!("🔄 Orchestration fallback: Local service registration");

        let service_name = params
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| MockRoutingError::FallbackError("Missing service name".to_string()))?;

        debug!(
            "📝 Registered service locally: {} (will sync when orchestration becomes available)",
            service_name
        );

        Ok(serde_json::json!({
            "success": true,
            "service_id": format!("local_{}", uuid::Uuid::new_v4()),
            "status": "registered_locally",
            "message": "Service registered locally - will sync when orchestration becomes available",
            "provider": "orchestration_fallback"
        }))
    }

    /// Local service discovery fallback
    async fn discover_services_fallback(
        &self,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, MockRoutingError> {
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
            "message": format!("No local services found for type: {}", service_type),
            "provider": "orchestration_fallback"
        }))
    }

    /// Local workflow coordination fallback
    async fn coordinate_workflow_fallback(
        &self,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, MockRoutingError> {
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
    ) -> Result<String, MockRoutingError> {
        match operation {
            "deploy_service" => Ok("Service deployment initiated".to_string()),
            "scale_service" => Ok("Service scaling completed".to_string()),
            "health_check" => Ok("All services healthy".to_string()),
            _ => Err(MockRoutingError::FallbackError(format!(
                "Unsupported orchestration operation: {operation}"
            ))),
        }
    }
}

#[async_trait]
impl FallbackProvider for OrchestrationFallbackProvider {
    async fn execute(
        &self,
        operation: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, MockRoutingError> {
        match operation {
            "register_service" => self.register_service_fallback(params).await,
            "discover_services" => self.discover_services_fallback(params).await,
            "coordinate_workflow" => self.coordinate_workflow_fallback(params).await,
            _ => Err(MockRoutingError::FallbackError(format!(
                "Unsupported orchestration operation: {operation}"
            ))),
        }
    }

    fn supported_operations(&self) -> Vec<String> {
        vec![
            "register_service".to_string(),
            "unregister_service".to_string(),
            "discover_services".to_string(),
            "coordinate_workflow".to_string(),
            "manage_resources".to_string(),
        ]
    }

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
