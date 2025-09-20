//! **PRIMAL AGNOSTIC RPC SERVICE**
//!
//! This module replaces all primal-specific RPC implementations with a universal
//! capability-based system. No more hardcoded references to security, orchestration, etc.

use crate::universal_adapter::{
    PrimalAgnosticAdapter, CapabilityCategory, CapabilityRequest, CapabilityResponse
};
use nestgate_core::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn};
use uuid::Uuid;

// ==================== UNIVERSAL RPC TYPES ====================

/// Universal RPC request that can route to any capability provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalRpcRequest {
    /// Request ID for tracking
    pub id: Uuid,
    /// Capability category needed
    pub capability: CapabilityCategory,
    /// Specific method to call
    pub method: String,
    /// Parameters for the method
    pub _params: HashMap<String, serde_json::Value>,
    /// Timeout for the request
    pub timeout_seconds: Option<u64>,
}

/// Universal RPC response from any capability provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalRpcResponse {
    /// Request ID this responds to
    pub id: Uuid,
    /// Whether the call was successful
    pub success: bool,
    /// Response data
    pub result: Option<serde_json::Value>,
    /// Error information if unsuccessful
    pub error: Option<RpcError>,
    /// Provider that handled the request
    pub provider: Option<String>,
    /// Execution time
    pub execution_time_ms: u64,
}

/// RPC error information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcError {
    /// Error code
    pub code: i32,
    /// Error message
    pub message: String,
    /// Additional error details
    pub details: Option<HashMap<String, serde_json::Value>>,
}

// ==================== PRIMAL AGNOSTIC RPC SERVICE ====================

/// Universal RPC service that eliminates all primal hardcoding
pub struct PrimalAgnosticRpcService {
    /// Universal adapter for capability-based routing
    adapter: PrimalAgnosticAdapter,
}

impl PrimalAgnosticRpcService {
    /// Create a new primal-agnostic RPC service
    pub const fn new() -> Self {
        Self {
            adapter: PrimalAgnosticAdapter::new(),
        }
    }

    /// Handle a universal RPC request (replaces primal-specific routing)
    pub fn handle_request(&self, request: UniversalRpcRequest) -> UniversalRpcResponse {
        info!("🔄 Universal RPC request: {:?}::{}", request.capability, request.method);
        
        let start_time = std::time::Instant::now();
        
        // Convert universal RPC request to capability request
        let capability_request = CapabilityRequest::new(request.capability.clone(), &request.method)
            .with_parameter("_params", serde_json::json!(request._params))
            .with_timeout(request.timeout_seconds.unwrap_or(30));

        // Route via capability system (no primal hardcoding)
        match self.adapter.request_capability(capability_request).await {
            Ok(capability_response) => {
                let execution_time = start_time.elapsed().as_millis() as u64;
                
                if capability_response.success {
                    debug!("✅ Capability request successful: {:?}::{}", request.capability, request.method);
                    UniversalRpcResponse {
                        id: request.id,
                        success: true,
                        result: Some(capability_response.data),
                        error: None,
                        provider: capability_response._metadata.get("provider").cloned(),
                        execution_time_ms: execution_time,
                    }
                } else {
                    warn!("❌ Capability request failed: {:?}::{}", request.capability, request.method);
                    UniversalRpcResponse {
                        id: request.id,
                        success: false,
                        result: None,
                        error: Some(RpcError {
                            code: -1,
                            message: capability_response.error.unwrap_or_else(|| "Unknown error".to_string()),
                            details: None,
                        }),
                        provider: capability_response._metadata.get("provider").cloned(),
                        execution_time_ms: execution_time,
                    }
                }
            }
            Err(e) => {
                let execution_time = start_time.elapsed().as_millis() as u64;
                warn!("❌ RPC routing failed: {}", e);
                
                UniversalRpcResponse {
                    id: request.id,
                    success: false,
                    result: None,
                    error: Some(RpcError {
                        code: -2,
                        message: format!("Routing error: {e}"),
                        details: None,
                    }),
                    provider: None,
                    execution_time_ms: execution_time,
                }
            }
        }
    }

    /// Register a discovered service (replaces hardcoded primal registration)
    pub fn register_service(&mut self, service: nestgate_core::universal_adapter::DiscoveredService) {
        info!("📝 Registering discovered service: {} ({})", service.name, service.service_type);
        self.adapter.register_discovered_service(service);
    }
}

impl Default for PrimalAgnosticRpcService {
    fn default() -> Self {
        Self::new()
    }
}

// ==================== MIGRATION HELPERS ====================

/// Migration helpers for replacing primal-specific RPC calls
pub mod migration_helpers {
    use super::*;

    /// Replace security-specific security RPC calls
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn security_rpc_call(
        service: &PrimalAgnosticRpcService,
        method: &str,
        _params: HashMap<String, serde_json::Value>
    ) -> Result<UniversalRpcResponse>  {
        let request = UniversalRpcRequest {
            id: Uuid::new_v4(),
            capability: CapabilityCategory::Security,
            method: method.to_string(),
            _params,
            timeout_seconds: Some(30),
        };

        Ok(service.handle_request(request).await)
    }

    /// Replace orchestration-specific orchestration RPC calls
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn orchestration_rpc_call(
        service: &PrimalAgnosticRpcService,
        method: &str,
        _params: HashMap<String, serde_json::Value>
    ) -> Result<UniversalRpcResponse>  {
        let request = UniversalRpcRequest {
            id: Uuid::new_v4(),
            capability: CapabilityCategory::Orchestration,
            method: method.to_string(),
            _params,
            timeout_seconds: Some(60), // Orchestration may take longer
        };

        Ok(service.handle_request(request).await)
    }

    /// Replace compute-specific compute RPC calls
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn compute_rpc_call(
        service: &PrimalAgnosticRpcService,
        method: &str,
        _params: HashMap<String, serde_json::Value>
    ) -> Result<UniversalRpcResponse>  {
        let request = UniversalRpcRequest {
            id: Uuid::new_v4(),
            capability: CapabilityCategory::Compute,
            method: method.to_string(),
            _params,
            timeout_seconds: Some(120), // Compute may take longer
        };

        Ok(service.handle_request(request).await)
    }

    /// Replace intelligence-specific AI RPC calls
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn intelligence_rpc_call(
        service: &PrimalAgnosticRpcService,
        method: &str,
        _params: HashMap<String, serde_json::Value>
    ) -> Result<UniversalRpcResponse>  {
        let request = UniversalRpcRequest {
            id: Uuid::new_v4(),
            capability: CapabilityCategory::Intelligence,
            method: method.to_string(),
            _params,
            timeout_seconds: Some(300), // AI processing may take longer
        };

        Ok(service.handle_request(request).await)
    }
}

// ==================== COMPATIBILITY LAYER ====================

/// Compatibility layer for existing code during migration
pub struct RpcCompatibilityLayer {
    universal_service: PrimalAgnosticRpcService,
}

impl RpcCompatibilityLayer {
    /// Create a new compatibility layer
    pub const fn new() -> Self {
        Self {
            universal_service: PrimalAgnosticRpcService::new(),
        }
    }

    /// Legacy security call compatibility (to be removed after migration)
    #[deprecated(note = "Use security_rpc_call instead")]
    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        pub async fn security_call(&self, method: &str, _params: HashMap<String, serde_json::Value>) -> Result<UniversalRpcResponse>  {
        migration_helpers::security_rpc_call(&self.universal_service, method, _params).await
    }

    /// Legacy orchestration call compatibility (to be removed after migration)
    #[deprecated(note = "Use orchestration_rpc_call instead")]
    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        pub async fn orchestration_call(&self, method: &str, _params: HashMap<String, serde_json::Value>) -> Result<UniversalRpcResponse>  {
        migration_helpers::orchestration_rpc_call(&self.universal_service, method, _params).await
    }

    /// Legacy compute call compatibility (to be removed after migration)
    #[deprecated(note = "Use compute_rpc_call instead")]
    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        pub async fn compute_call(&self, method: &str, _params: HashMap<String, serde_json::Value>) -> Result<UniversalRpcResponse>  {
        migration_helpers::compute_rpc_call(&self.universal_service, method, _params).await
    }

    /// Legacy intelligence call compatibility (to be removed after migration)
    #[deprecated(note = "Use intelligence_rpc_call instead")]
    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        pub async fn intelligence_call(&self, method: &str, _params: HashMap<String, serde_json::Value>) -> Result<UniversalRpcResponse>  {
        migration_helpers::intelligence_rpc_call(&self.universal_service, method, _params).await
    }
}

impl Default for RpcCompatibilityLayer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_universal_rpc_request() {
        let service = PrimalAgnosticRpcService::new();
        
        let request = UniversalRpcRequest {
            id: Uuid::new_v4(),
            capability: CapabilityCategory::Security,
            method: "authenticate".to_string(),
            _params: {
                let mut _params = HashMap::new();
                _params.insert("username".to_string(), serde_json::json!("test"));
                _params
            },
            timeout_seconds: Some(30),
        };

        // This will fail since no providers are registered, but should not panic
        let response = service.handle_request(request).await;
        assert!(!response.success); // Expected to fail with no providers
    }

    #[test]
    fn test_migration_helper_creation() {
        let service = PrimalAgnosticRpcService::new();
        
        // Should be able to create requests for different capabilities
        let _params = HashMap::new();
        
        // These calls should compile (actual execution would need providers)
        let _security_future = migration_helpers::security_rpc_call(&service, "test", _params.clone());
        let _orchestration_future = migration_helpers::orchestration_rpc_call(&service, "test", _params.clone());
        let _compute_future = migration_helpers::compute_rpc_call(&service, "test", _params.clone());
        let _intelligence_future = migration_helpers::intelligence_rpc_call(&service, "test", _params);
    }

    #[test]
    fn test_compatibility_layer() {
        let _compat = RpcCompatibilityLayer::new();
        
        // Compatibility layer should be created successfully
        // Legacy methods are deprecated but should still compile
    }
} 