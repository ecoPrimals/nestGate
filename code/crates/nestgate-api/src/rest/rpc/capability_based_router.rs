//! **CAPABILITY-BASED RPC ROUTER**
//!
//! This module replaces primal-specific routing with capability-based routing.
//! No more hardcoded references to security, orchestration, or other primals.

use super::{RpcConnectionType, RpcError, UnifiedRpcRequest};
use crate::universal_adapter::{
    CapabilityCategory, PrimalAgnosticAdapter, CapabilityRequest
};
use std::collections::HashMap;
use tracing::{debug, info, warn};

/// Capability-based RPC router that eliminates primal hardcoding
pub struct CapabilityBasedRpcRouter {
    /// Universal adapter for capability discovery
    adapter: PrimalAgnosticAdapter,
    /// Capability to connection type mapping
    capability_connections: HashMap<CapabilityCategory, RpcConnectionType>,
    /// Method to capability category mapping
    method_capabilities: HashMap<String, CapabilityCategory>,
    /// Default connection type
    default_connection: RpcConnectionType,
}

impl CapabilityBasedRpcRouter {
    /// Create new capability-based RPC router
    #[must_use]
    pub fn new() -> Self {
        let mut router = Self {
            adapter: PrimalAgnosticAdapter::new(),
            capability_connections: HashMap::new(),
            method_capabilities: HashMap::new(),
            default_connection: RpcConnectionType::WebSocket,
        };

        // Initialize capability-based routing rules
        router.initialize_capability_routing();
        router
    }

    /// Initialize capability-based routing rules (no primal hardcoding)
    fn initialize_capability_routing(&mut self) {
        // Map capability categories to optimal connection types
        self.capability_connections.insert(
            CapabilityCategory::Security, 
            RpcConnectionType::Tarpc  // High-performance binary for security
        );
        self.capability_connections.insert(
            CapabilityCategory::Orchestration, 
            RpcConnectionType::JsonRpc  // Standard JSON for orchestration
        );
        self.capability_connections.insert(
            CapabilityCategory::Compute, 
            RpcConnectionType::JsonRpc  // Standard JSON for compute requests
        );
        self.capability_connections.insert(
            CapabilityCategory::Intelligence, 
            RpcConnectionType::JsonRpc  // Standard JSON for AI requests
        );
        self.capability_connections.insert(
            CapabilityCategory::Storage, 
            RpcConnectionType::WebSocket  // Real-time for storage events
        );
        self.capability_connections.insert(
            CapabilityCategory::Management, 
            RpcConnectionType::WebSocket  // Real-time for management
        );
        self.capability_connections.insert(
            CapabilityCategory::Network, 
            RpcConnectionType::JsonRpc  // Standard JSON for network ops
        );
        self.capability_connections.insert(
            CapabilityCategory::Data, 
            RpcConnectionType::WebSocket  // Real-time for data streams
        );

        // Map methods to capability categories (generic, not primal-specific)
        self.initialize_security_methods();
        self.initialize_orchestration_methods();
        self.initialize_compute_methods();
        self.initialize_intelligence_methods();
        self.initialize_storage_methods();
        self.initialize_management_methods();
        self.initialize_network_methods();
        self.initialize_data_methods();

        info!(
            "🔀 Capability-based RPC router initialized with {} capability mappings and {} method mappings",
            self.capability_connections.len(),
            self.method_capabilities.len()
        );
    }

    /// Initialize security capability methods (replaces security hardcoding)
    fn initialize_security_methods(&mut self) {
        let security_methods = [
            "encrypt_data",
            "decrypt_data", 
            "generate_key",
            "authenticate_user",
            "authorize_action",
            "get_security_status",
            "stream_security_events",
            "stream_threat_detection",
            "stream_audit_logs",
            "validate_certificate",
            "sign_data",
            "verify_signature",
        ];

        for method in security_methods {
            self.method_capabilities.insert(
                method.to_string(), 
                CapabilityCategory::Security
            );
        }
    }

    /// Initialize orchestration capability methods (replaces orchestration hardcoding)
    fn initialize_orchestration_methods(&mut self) {
        let orchestration_methods = [
            "register_service",
            "discover_services",
            "coordinate_workflow",
            "get_service_status",
            "allocate_port",
            "stream_service_events",
            "stream_workflow_status",
            "stream_network_topology",
            "deploy_service",
            "scale_service",
            "health_check",
            "load_balance",
        ];

        for method in orchestration_methods {
            self.method_capabilities.insert(
                method.to_string(), 
                CapabilityCategory::Orchestration
            );
        }
    }

    /// Initialize compute capability methods (replaces compute hardcoding)
    fn initialize_compute_methods(&mut self) {
        let compute_methods = [
            "run_container",
            "execute_function",
            "process_batch",
            "allocate_resources",
            "get_compute_status",
            "stream_compute_metrics",
            "optimize_performance",
            "benchmark_system",
            "tune_hardware",
            "profile_workload",
        ];

        for method in compute_methods {
            self.method_capabilities.insert(
                method.to_string(), 
                CapabilityCategory::Compute
            );
        }
    }

    /// Initialize intelligence capability methods (replaces intelligence hardcoding)
    fn initialize_intelligence_methods(&mut self) {
        let intelligence_methods = [
            "analyze_data",
            "predict_trend",
            "classify_content",
            "generate_text",
            "create_embedding",
            "train_model",
            "infer_pattern",
            "optimize_algorithm",
            "extract_features",
            "recommend_action",
        ];

        for method in intelligence_methods {
            self.method_capabilities.insert(
                method.to_string(), 
                CapabilityCategory::Intelligence
            );
        }
    }

    /// Initialize storage capability methods (NestGate's domain)
    fn initialize_storage_methods(&mut self) {
        let storage_methods = [
            "get_real_time_metrics",
            "start_metrics_stream", 
            "stream_realtime_metrics",
            "stream_zfs_events",
            "stream_storage_events",
            "stream_system_logs",
            "stream_performance_data",
            "list_pools",
            "create_dataset",
            "mount_filesystem",
            "backup_data",
        ];

        for method in storage_methods {
            self.method_capabilities.insert(
                method.to_string(), 
                CapabilityCategory::Storage
            );
        }
    }

    /// Initialize management capability methods (replaces management hardcoding)
    fn initialize_management_methods(&mut self) {
        let management_methods = [
            "deploy_application",
            "monitor_system",
            "configure_service",
            "update_software",
            "manage_resources",
            "stream_system_status",
            "collect_metrics",
            "generate_report",
        ];

        for method in management_methods {
            self.method_capabilities.insert(
                method.to_string(), 
                CapabilityCategory::Management
            );
        }
    }

    /// Initialize network capability methods
    fn initialize_network_methods(&mut self) {
        let network_methods = [
            "configure_routing",
            "manage_firewall",
            "monitor_bandwidth",
            "optimize_traffic",
            "discover_topology",
            "balance_load",
        ];

        for method in network_methods {
            self.method_capabilities.insert(
                method.to_string(), 
                CapabilityCategory::Network
            );
        }
    }

    /// Initialize data capability methods
    fn initialize_data_methods(&mut self) {
        let data_methods = [
            "stream_data",
            "process_stream",
            "cache_data",
            "query_database",
            "index_content",
            "replicate_data",
        ];

        for method in data_methods {
            self.method_capabilities.insert(
                method.to_string(), 
                CapabilityCategory::Data
            );
        }
    }

    /// Route a request using capability-based logic (no primal assumptions)
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
            "🔀 Capability-based routing: {} -> {}",
            request.method, request.target
        );

        // 1. Determine capability category for this method
        let capability = self.determine_capability(&request.method);
        
        // 2. Check if we have providers for this capability
        let providers = self.adapter.find_providers(&capability, &request.method);
        
        if providers.is_empty() {
            warn!(
                "⚠️ No providers found for capability {:?}::{}", 
                capability, request.method
            );
            // Fall back to heuristic routing
            return Ok(self.apply_heuristic_routing(request));
        }

        // 3. Get optimal connection type for this capability
        let connection_type = self.capability_connections
            .get(&capability)
            .copied()
            .unwrap_or(self.default_connection);

        debug!(
            "✅ Capability-based route: {:?}::{} -> {:?} (providers: {})",
            capability, request.method, connection_type, providers.len()
        );

        Ok(connection_type)
    }

    /// Determine capability category for a method
    fn determine_capability(&self, method: &str) -> CapabilityCategory {
        // 1. Check explicit method mapping
        if let Some(capability) = self.method_capabilities.get(method) {
            return capability.clone();
        }

        // 2. Apply heuristic capability detection
        self.detect_capability_from_method(method)
    }

    /// Detect capability from method name using heuristics
    fn detect_capability_from_method(&self, method: &str) -> CapabilityCategory {
        let method_lower = method.to_lowercase();

        // Security patterns
        if method_lower.contains("encrypt") || method_lower.contains("decrypt") 
            || method_lower.contains("auth") || method_lower.contains("security")
            || method_lower.contains("crypto") || method_lower.contains("key")
            || method_lower.contains("threat") || method_lower.contains("audit") {
            return CapabilityCategory::Security;
        }

        // Orchestration patterns  
        if method_lower.contains("register") || method_lower.contains("discover")
            || method_lower.contains("coordinate") || method_lower.contains("workflow")
            || method_lower.contains("service") || method_lower.contains("deploy")
            || method_lower.contains("scale") {
            return CapabilityCategory::Orchestration;
        }

        // Compute patterns
        if method_lower.contains("compute") || method_lower.contains("execute")
            || method_lower.contains("process") || method_lower.contains("run")
            || method_lower.contains("container") || method_lower.contains("function")
            || method_lower.contains("benchmark") || method_lower.contains("tune") {
            return CapabilityCategory::Compute;
        }

        // Intelligence patterns
        if method_lower.contains("analyze") || method_lower.contains("predict")
            || method_lower.contains("classify") || method_lower.contains("generate")
            || method_lower.contains("train") || method_lower.contains("infer")
            || method_lower.contains("recommend") || method_lower.contains("ai")
            || method_lower.contains("ml") {
            return CapabilityCategory::Intelligence;
        }

        // Storage patterns (NestGate's domain)
        if method_lower.contains("storage") || method_lower.contains("zfs")
            || method_lower.contains("pool") || method_lower.contains("dataset")
            || method_lower.contains("snapshot") || method_lower.contains("mount")
            || method_lower.contains("backup") {
            return CapabilityCategory::Storage;
        }

        // Streaming/real-time patterns
        if method_lower.contains("stream") || method_lower.contains("realtime")
            || method_lower.contains("metrics") || method_lower.contains("events") {
            return CapabilityCategory::Data;
        }

        // Default to management
        CapabilityCategory::Management
    }

    /// Apply heuristic routing when no capability providers found
    fn apply_heuristic_routing(&self, request: &UnifiedRpcRequest) -> RpcConnectionType {
        let capability = self.detect_capability_from_method(&request.method);
        
        self.capability_connections
            .get(&capability)
            .copied()
            .unwrap_or(self.default_connection)
    }

    /// Register a discovered service (capability provider)
    pub fn register_capability_provider(&mut self, service: nestgate_core::universal_adapter::DiscoveredService) {
        info!(
            "📝 Registering capability provider: {} with {} capabilities",
            service.name, service.capabilities.len()
        );
        self.adapter.register_discovered_service(service);
    }

    /// Get routing statistics
    pub fn get_routing_stats(&self) -> RoutingStats {
        let mut capability_counts = HashMap::new();
        
        for capability in self.method_capabilities.values() {
            *capability_counts.entry(capability.clone()).or_insert(0) += 1;
        }

        RoutingStats {
            total_methods: self.method_capabilities.len(),
            capability_mappings: self.capability_connections.len(),
            capability_distribution: capability_counts,
        }
    }
}

impl Default for CapabilityBasedRpcRouter {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

/// Routing statistics for monitoring
#[derive(Debug, Clone)]
/// Routingstats
pub struct RoutingStats {
    /// Total Methods
    pub total_methods: usize,
    /// Capability Mappings
    pub capability_mappings: usize,
    /// Capability Distribution
    pub capability_distribution: HashMap<CapabilityCategory, usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_detection() {
        let router = CapabilityBasedRpcRouter::new();
        
        // Test security detection
        assert_eq!(
            router.detect_capability_from_method("encrypt_data"),
            CapabilityCategory::Security
        );
        
        // Test orchestration detection
        assert_eq!(
            router.detect_capability_from_method("deploy_service"),
            CapabilityCategory::Orchestration
        );
        
        // Test storage detection (NestGate's domain)
        assert_eq!(
            router.detect_capability_from_method("list_zfs_pools"),
            CapabilityCategory::Storage
        );
        
        // Test intelligence detection
        assert_eq!(
            router.detect_capability_from_method("analyze_data"),
            CapabilityCategory::Intelligence
        );
    }

    #[test]
    fn test_routing_stats() {
        let router = CapabilityBasedRpcRouter::new();
        let stats = router.get_routing_stats();
        
        assert!(stats.total_methods > 0);
        assert!(stats.capability_mappings > 0);
        assert!(stats.capability_distribution.contains_key(&CapabilityCategory::Security));
        assert!(stats.capability_distribution.contains_key(&CapabilityCategory::Storage));
    }

    #[tokio::test]
    async fn test_capability_based_routing() {
        let router = CapabilityBasedRpcRouter::new();
        
        let request = UnifiedRpcRequest {
            method: "encrypt_data".to_string(),
            target: "security-service".to_string(),
            _params: std::collections::HashMap::new(),
        };

        // Should route to Tarpc for security operations
        let result = router.route_request(&request).await;
        assert!(result.is_ok());
        
        // Note: Will use heuristic routing since no providers registered in test
    }
} 