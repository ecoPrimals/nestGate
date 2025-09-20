//
// Intelligent routing system that determines the best RPC method
// (tarpc, JSON RPC, or WebSocket) based on request characteristics.

use super::{RpcConnectionType, RpcError, UnifiedRpcRequest};
use std::collections::HashMap;
use tracing::{debug, info};

/// Unified RPC router for intelligent request routing
pub struct UnifiedRpcRouter {
    /// Routing rules based on method patterns
    method_rules: HashMap<String, RpcConnectionType>,
    /// Routing rules based on target service
    target_rules: HashMap<String, RpcConnectionType>,
    /// Default connection type
    default_connection: RpcConnectionType,
}
impl UnifiedRpcRouter {
    /// Create new unified RPC router
    #[must_use]
    pub fn new() -> Self {
        let mut router = Self {
            method_rules: HashMap::new(),
            target_rules: HashMap::new(),
            default_connection: RpcConnectionType::WebSocket,
        };

        // Initialize routing rules
        router.initialize_routing_rules();
        router
    }

    /// Initialize default routing rules
    fn initialize_routing_rules(&mut self) {
        // Security-related methods route to security via tarpc
        self.method_rules
            .insert("encrypt_data".to_string(), RpcConnectionType::Tarpc);
        self.method_rules
            .insert("decrypt_data".to_string(), RpcConnectionType::Tarpc);
        self.method_rules
            .insert("generate_key".to_string(), RpcConnectionType::Tarpc);
        self.method_rules
            .insert("authenticate_user".to_string(), RpcConnectionType::Tarpc);
        self.method_rules
            .insert("get_security_status".to_string(), RpcConnectionType::Tarpc);
        self.method_rules.insert(
            "stream_security_events".to_string(),
            RpcConnectionType::Tarpc,
        );
        self.method_rules.insert(
            "stream_threat_detection".to_string(),
            RpcConnectionType::Tarpc,
        );
        self.method_rules
            .insert("stream_audit_logs".to_string(), RpcConnectionType::Tarpc);

        // Orchestration-related methods route to orchestration via JSON RPC
        self.method_rules
            .insert("register_service".to_string(), RpcConnectionType::JsonRpc);
        self.method_rules
            .insert("discover_services".to_string(), RpcConnectionType::JsonRpc);
        self.method_rules.insert(
            "coordinate_workflow".to_string(),
            RpcConnectionType::JsonRpc,
        );
        self.method_rules
            .insert("get_service_status".to_string(), RpcConnectionType::JsonRpc);
        self.method_rules
            .insert("allocate_port".to_string(), RpcConnectionType::JsonRpc);
        self.method_rules.insert(
            "stream_service_events".to_string(),
            RpcConnectionType::JsonRpc,
        );
        self.method_rules.insert(
            "stream_workflow_status".to_string(),
            RpcConnectionType::JsonRpc,
        );
        self.method_rules.insert(
            "stream_network_topology".to_string(),
            RpcConnectionType::JsonRpc,
        );

        // Real-time data methods route to WebSocket
        self.method_rules.insert(
            "get_real_time_metrics".to_string(),
            RpcConnectionType::WebSocket,
        );
        self.method_rules.insert(
            "start_metrics_stream".to_string(),
            RpcConnectionType::WebSocket,
        );
        self.method_rules.insert(
            "stream_realtime_metrics".to_string(),
            RpcConnectionType::WebSocket,
        );
        self.method_rules.insert(
            "stream_zfs_events".to_string(),
            RpcConnectionType::WebSocket,
        );
        self.method_rules.insert(
            "stream_storage_events".to_string(),
            RpcConnectionType::WebSocket,
        );
        self.method_rules.insert(
            "stream_system_logs".to_string(),
            RpcConnectionType::WebSocket,
        );
        self.method_rules.insert(
            "stream_performance_data".to_string(),
            RpcConnectionType::WebSocket,
        );

        // Capability-based service routing (sovereignty compliant)
        self.target_rules
            .insert("security-encryption".to_string(), RpcConnectionType::Tarpc);
        self.target_rules.insert(
            "orchestration-discovery".to_string(),
            RpcConnectionType::JsonRpc,
        );
        self.target_rules.insert(
            "storage-management".to_string(),
            RpcConnectionType::WebSocket,
        );
        self.target_rules
            .insert("ai-text-generation".to_string(), RpcConnectionType::JsonRpc);
        self.target_rules
            .insert("ai-embedding".to_string(), RpcConnectionType::JsonRpc);
        self.target_rules.insert(
            "ecosystem-management".to_string(),
            RpcConnectionType::WebSocket,
        );

        info!(
            "🔀 RPC router initialized with {} method rules and {} target rules",
            self.method_rules.len(),
            self.target_rules.len()
        );
    }

    /// Route a request to the appropriate connection type
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn route_request(
        &self,
        request: &UnifiedRpcRequest,
    ) -> Result<RpcConnectionType, RpcError>  {
        debug!(
            "🔀 Routing RPC request: {} -> {}",
            request.method, request.target
        );

        // 1. Check method-specific rules first (most specific)
        if let Some(&connection_type) = self.method_rules.get(&request.method) {
            debug!(
                "✅ Routed by method rule: {} -> {:?}",
                request.method, connection_type
            );
            return Ok(connection_type);
        }

        // 2. Check target service rules
        if let Some(&connection_type) = self.target_rules.get(&request.target) {
            debug!(
                "✅ Routed by target rule: {} -> {:?}",
                request.target, connection_type
            );
            return Ok(connection_type);
        }

        // 3. Apply heuristic routing based on method patterns
        let connection_type = self.apply_heuristic_routing(request);
        debug!(
            "✅ Routed by heuristic: {} -> {:?}",
            request.method, connection_type
        );

        Ok(connection_type)
    }

    /// Apply heuristic routing based on request characteristics
    fn apply_heuristic_routing(&self, request: &UnifiedRpcRequest) -> RpcConnectionType {
        let method = &request.method;

        // Security-related patterns
        if method.contains("encrypt")
            || method.contains("decrypt")
            || method.contains("auth")
            || method.contains("security")
            || method.contains("crypto")
            || method.contains("key")
            || method.contains("threat")
            || method.contains("audit")
        {
            return RpcConnectionType::Tarpc;
        }

        // Orchestration patterns
        if method.contains("register")
            || method.contains("discover")
            || method.contains("coordinate")
            || method.contains("workflow")
            || method.contains("service")
            || method.contains("port")
            || method.contains("topology")
            || method.contains("mesh")
        {
            return RpcConnectionType::JsonRpc;
        }

        // Streaming patterns
        if method.starts_with("stream_")
            || method.contains("realtime")
            || method.contains("live")
            || method.contains("metrics")
            || method.contains("events")
            || method.contains("logs")
            || request.streaming
        {
            return RpcConnectionType::WebSocket;
        }

        // High-frequency operations
        if method.contains("get_") && (method.contains("status") || method.contains("health")) {
            return RpcConnectionType::WebSocket;
        }

        // Default to WebSocket for unknown methods
        self.default_connection
    }

    /// Add custom routing rule for a method
    pub fn add_method_rule(&mut self, method: String, connection_type: RpcConnectionType) {
        self.method_rules.insert(method.clone(), connection_type);
        debug!("➕ Added method rule: {} -> {:?}", method, connection_type);
    }

    /// Add custom routing rule for a target service
    pub fn add_target_rule(&mut self, target: String, connection_type: RpcConnectionType) {
        self.target_rules.insert(target.clone(), connection_type);
        debug!("➕ Added target rule: {} -> {:?}", target, connection_type);
    }

    /// Remove routing rule for a method
    pub fn remove_method_rule(&mut self, method: &str) {
        if self.method_rules.remove(method).is_some() {
            debug!("➖ Removed method rule: {}", method);
        }
    }

    /// Remove routing rule for a target service
    pub fn remove_target_rule(&mut self, target: &str) {
        if self.target_rules.remove(target).is_some() {
            debug!("➖ Removed target rule: {}", target);
        }
    }

    /// Get routing statistics
    pub const fn get_routing_stats(&self) -> RoutingStats {
        RoutingStats {
            method_rules_count: self.method_rules.len(),
            target_rules_count: self.target_rules.len(),
            default_connection: self.default_connection,
            method_rules: self.method_rules.clone(),
            target_rules: self.target_rules.clone(),
        }
    }

    /// Set default connection type
    pub fn set_default_connection(&mut self, connection_type: RpcConnectionType) {
        self.default_connection = connection_type;
        debug!("🔧 Set default connection type: {:?}", connection_type);
    }

    /// Get connection type recommendation for a method pattern
    pub fn recommend_connection_type(
        &self,
        method_pattern: &str,
    ) -> Vec<(RpcConnectionType, String)> {
        let mut recommendations = Vec::new();

        // Analyze pattern and provide recommendations
        if method_pattern.contains("encrypt") || method_pattern.contains("security") {
            recommendations.push((
                RpcConnectionType::Tarpc,
                "High-performance binary protocol recommended for security operations".to_string(),
            ));
        }

        if method_pattern.contains("service") || method_pattern.contains("orchestr") {
            recommendations.push((
                RpcConnectionType::JsonRpc,
                "Standard JSON RPC recommended for service orchestration".to_string(),
            ));
        }

        if method_pattern.contains("stream") || method_pattern.contains("realtime") {
            recommendations.push((
                RpcConnectionType::WebSocket,
                "WebSocket recommended for real-time streaming data".to_string(),
            ));
        }

        if recommendations.is_empty() {
            recommendations.push((
                self.default_connection,
                "Default connection type for general operations".to_string(),
            ));
        }

        recommendations
    }
}

/// Routing statistics
#[derive(Debug, Clone)]
pub struct RoutingStats {
    /// Number of method-based routing rules
    pub method_rules_count: usize,
    /// Number of target-based routing rules
    pub target_rules_count: usize,
    /// Default connection type for unmatched requests
    pub default_connection: RpcConnectionType,
    /// Method-specific routing rules
    pub method_rules: HashMap<String, RpcConnectionType>,
    /// Target-specific routing rules
    pub target_rules: HashMap<String, RpcConnectionType>,
}
impl Default for UnifiedRpcRouter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn create_test_request(method: &str, target: &str) -> UnifiedRpcRequest {
        UnifiedRpcRequest {
            id: Uuid::new_v4(),
            source: "test".to_string(),
            target: target.to_string(),
            method: method.to_string(),
            _params: serde_json::json!({}),
            priority: crate::rest::rpc::types::RequestPriority::Normal,
            streaming: false,
            timeout: Some(std::time::Duration::from_secs(30)),
            _metadata: HashMap::new(),
            timestamp: chrono::Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_security_method_routing() {
        let router = UnifiedRpcRouter::new();
        let request = create_test_request("encrypt_data", "unknown");

        let connection_type = router.route_request(&request).await.unwrap_or_else(|e| {
            tracing::error!("Unwrap failed: {:?}", e);
            panic!("Test failed: unable to continue: {:?}", e);
        });
        assert_eq!(connection_type, RpcConnectionType::Tarpc);
    }

    #[tokio::test]
    async fn test_orchestration_method_routing() {
        let router = UnifiedRpcRouter::new();
        let request = create_test_request("register_service", "unknown");

        let connection_type = router.route_request(&request).await.unwrap_or_else(|e| {
            tracing::error!("Unwrap failed: {:?}", e);
            panic!("Test failed: unable to continue: {:?}", e);
        });
        assert_eq!(connection_type, RpcConnectionType::JsonRpc);
    }

    #[tokio::test]
    async fn test_streaming_method_routing() {
        let router = UnifiedRpcRouter::new();
        let request = create_test_request("stream_realtime_metrics", "unknown");

        let connection_type = router.route_request(&request).await.unwrap_or_else(|e| {
            tracing::error!("Unwrap failed: {:?}", e);
            panic!("Test failed: unable to continue: {:?}", e);
        });
        assert_eq!(connection_type, RpcConnectionType::WebSocket);
    }

    #[tokio::test]
    async fn test_target_based_routing() {
        let router = UnifiedRpcRouter::new();
        let request = create_test_request("unknown_method", "security");

        let connection_type = router.route_request(&request).await.unwrap_or_else(|e| {
            tracing::error!("Unwrap failed: {:?}", e);
            panic!("Test failed: unable to continue: {:?}", e);
        });
        assert_eq!(connection_type, RpcConnectionType::WebSocket);
    }

    #[tokio::test]
    async fn test_heuristic_routing() {
        let router = UnifiedRpcRouter::new();
        let request = create_test_request("decrypt_sensitive_data", "unknown_service");

        let connection_type = router.route_request(&request).await.unwrap_or_else(|e| {
            tracing::error!("Unwrap failed: {:?}", e);
            panic!("Test failed: unable to continue: {:?}", e);
        });
        assert_eq!(connection_type, RpcConnectionType::Tarpc);
    }
}
