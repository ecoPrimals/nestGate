// **PRIMAL HARDCODING DEPRECATION NOTICE**
//!
//! ⚠️  DEPRECATION WARNING: This file contains references to specific primal service names
//! (security_service, orchestration_service, compute_service, ai_service) which are being migrated to capability-based discovery.
//!
//! **MIGRATION STATUS**: 🔄 IN PROGRESS
//! **TARGET**: Replace all primal names with CapabilityCategory enums
//! **DEADLINE**: Q1 2025
//!
//! **MIGRATION GUIDE**:
//! - Replace "security_service" → CapabilityCategory::Security
//! - Replace "orchestration_service" → CapabilityCategory::Orchestration  
//! - Replace "compute_service" → CapabilityCategory::Compute
//! - Replace "ai_service" → CapabilityCategory::Intelligence
//!
//! **SEE**: `universal_adapter/capability_system.rs` for new routing patterns

use super::{RpcConnectionType, RpcError, UnifiedRpcRequest, UnifiedRpcResponse};
use nestgate_core::universal_adapter::{UniversalAdapter, types::CapabilityQuery};
use nestgate_core::universal_adapter::capability_discovery::CapabilityType as CapabilityCategory;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info, warn};
use tokio::sync::RwLock;

/// Universal RPC router using capability-based discovery
pub struct UniversalRpcRouter {
    /// Universal adapter for capability discovery
    adapter: Arc<UniversalAdapter>,
    /// Cached capability-to-connection mappings
    capability_cache: Arc<RwLock<HashMap<String, CapabilityRoute>>>,
    /// Default connection preferences
    connection_preferences: ConnectionPreferences,
}

/// Route information for a capability
#[derive(Debug, Clone)]
/// Capabilityroute
pub struct CapabilityRoute {
    /// Capability category
    pub category: CapabilityCategory,
    /// Preferred connection type for this capability
    pub connection_type: RpcConnectionType,
    /// Provider service information
    pub provider_info: ProviderInfo,
    /// Performance characteristics
    pub performance_tier: String,
    /// Last successful connection timestamp
    pub last_success: std::time::SystemTime,
}

/// Provider service information
#[derive(Debug, Clone)]
/// Providerinfo
pub struct ProviderInfo {
    /// Service identifier (not primal name)
    pub service_id: String,
    /// Service endpoint
    pub endpoint: String,
    /// Service capabilities
    pub capabilities: Vec<String>,
    /// Service health status
    pub health_status: ServiceHealthStatus,
}

/// Service health status
#[derive(Debug, Clone, PartialEq)]
/// Status values for ServiceHealth
pub enum ServiceHealthStatus {
    /// Healthy
    Healthy,
    /// Degraded
    Degraded,
    /// Unhealthy
    Unhealthy,
    /// Unknown
    Unknown,
}

/// Connection type preferences for different capability categories
#[derive(Debug, Clone)]
/// Connectionpreferences
pub struct ConnectionPreferences {
    /// Preferred connection types by capability category
    pub category_preferences: HashMap<CapabilityCategory, RpcConnectionType>,
    /// Fallback connection types
    pub fallback_types: Vec<RpcConnectionType>,
    /// Performance requirements
    pub performance_requirements: HashMap<CapabilityCategory, PerformanceRequirement>,
}

/// Performance requirements for a capability category
#[derive(Debug, Clone)]
/// Performancerequirement
pub struct PerformanceRequirement {
    /// Maximum acceptable latency in milliseconds
    pub max_latency_ms: u64,
    /// Minimum required throughput (requests per second)
    pub min_throughput_rps: f64,
    /// Required reliability percentage
    pub min_reliability_percent: f64,
}

impl UniversalRpcRouter {
    /// Create new universal RPC router
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
                pub fn new(adapter: Arc<UniversalAdapter>) -> Result<Self, RpcError>  {
        let connection_preferences = Self::create_default_preferences();
        
        let router = Self {
            adapter,
            capability_cache: Arc::new(RwLock::new(HashMap::new())),
            connection_preferences,
        };

        // Initialize capability discovery
        router.initialize_capability_discovery().await?;
        
        Ok(router)
    }

    /// Route RPC request based on capability requirements
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
                pub fn route_request(&self, request: &UnifiedRpcRequest) -> Result<UnifiedRpcResponse, RpcError>  {
        info!("🔄 Routing RPC request via universal adapter: {}", request.method);

        // Determine required capability from method
        let capability_category = self.determine_capability_category(&request.method).await?;
        
        // Get or discover capability route
        let route = self.get_capability_route(&capability_category, &request.method).await?;
        
        // Execute request using appropriate connection type
        self.execute_request_via_route(request, &route).await
    }

    /// Initialize capability discovery and caching
    async fn initialize_capability_discovery(&self) -> Result<(), RpcError> {
        debug!("🔍 Initializing universal capability discovery");

        // Discover all available capabilities in the ecosystem
        let capabilities = self.adapter
            .query_capabilities(CapabilityQuery::All)
            .await
            .map_err(|e| RpcError::ServiceUnavailable(format!("Capability discovery failed: {e}")))?;

        // Build initial capability cache
        let mut cache = self.capability_cache.write().await;
        
        for capability in capabilities {
            let connection_type = self.determine_optimal_connection_type(&capability.category);
            
            let route = CapabilityRoute {
                category: capability.category.clone(),
                connection_type,
                provider_info: ProviderInfo {
                    service_id: capability.id.clone(),
                    endpoint: format!("discovered://{capability.provider}"),
                    capabilities: vec![capability.name.clone()],
                    health_status: ServiceHealthStatus::Unknown,
                },
                performance_tier: capability._metadata
                    .get("performance_tier")
                    .cloned()
                    .unwrap_or_else(|| "standard".to_string()),
                last_success: std::time::SystemTime::now(),
            };

            cache.insert(capability.name.clone(), route);
        }

        info!("✅ Initialized {} capability routes", cache.len());
        Ok(())
    }

    /// Determine capability category from RPC method
    async fn determine_capability_category(&self, method: &str) -> Result<CapabilityCategory, RpcError> {
        match method {
            // Security capabilities (replaces security_service hardcoding)
            method if method.contains("encrypt") 
                || method.contains("decrypt") 
                || method.contains("auth") 
                || method.contains("security")
                || method.contains("key") => Ok(CapabilityCategory::Security),

            // Orchestration capabilities (replaces orchestration_service hardcoding)
            method if method.contains("register") 
                || method.contains("discover") 
                || method.contains("coordinate")
                || method.contains("workflow")
                || method.contains("service")
                || method.contains("orchestr") => Ok(CapabilityCategory::Orchestration),

            // Compute capabilities (replaces compute_service hardcoding)
            method if method.contains("compute") 
                || method.contains("process") 
                || method.contains("execute")
                || method.contains("batch")
                || method.contains("container") => Ok(CapabilityCategory::Compute),

            // AI capabilities (replaces ai_service hardcoding)
            method if method.contains("ai") 
                || method.contains("ml") 
                || method.contains("model")
                || method.contains("inference")
                || method.contains("predict") => Ok(CapabilityCategory::ArtificialIntelligence),

            // Storage capabilities
            method if method.contains("storage") 
                || method.contains("file") 
                || method.contains("data")
                || method.contains("zfs")
                || method.contains("nas") => Ok(CapabilityCategory::Storage),

            // Monitoring capabilities
            method if method.contains("metric") 
                || method.contains("monitor") 
                || method.contains("health")
                || method.contains("status") => Ok(CapabilityCategory::Monitoring),

            // Network capabilities
            method if method.contains("network") 
                || method.contains("connection") 
                || method.contains("endpoint") => Ok(CapabilityCategory::Network),

            // Default to Integration for unknown methods
            _ => {
                warn!("🤔 Unknown method category for '{}', defaulting to Integration", method);
                Ok(CapabilityCategory::Integration)
            }
        }
    }

    /// Get capability route, discovering if necessary
    async fn get_capability_route(&self, category: &CapabilityCategory, method: &str) -> Result<CapabilityRoute, RpcError> {
        // Check cache first
        let cache = self.capability_cache.read().await;
        
        // Find best matching capability for this category and method
        let best_route = cache
            .values()
            .filter(|route| &route.category == category)
            .filter(|route| self.is_route_healthy(route))
            .max_by_key(|route| self.calculate_route_score(route, method));

        if let Some(route) = best_route {
            return Ok(route.clone());
        }

        drop(cache);

        // No cached route found, perform fresh discovery
        self.discover_capability_route(category, method).await
    }

    /// Discover capability route for specific category and method
    async fn discover_capability_route(&self, category: &CapabilityCategory, method: &str) -> Result<CapabilityRoute, RpcError> {
        debug!("🔍 Discovering capability route for {:?}::{}", category, method);

        // Query adapter for capabilities in this category
        let query = CapabilityQuery::ByType(format!("{category:?}").to_lowercase());
        let capabilities = self.adapter
            .query_capabilities(query)
            .await
            .map_err(|e| RpcError::ServiceUnavailable(format!("Discovery failed: {e}")))?;

        if capabilities.is_empty() {
            return Err(RpcError::ServiceUnavailable(format!(
                "No capabilities found for category: {:?}", category
            )));
        }

        // Select best capability based on performance and availability
        let best_capability = capabilities
            .into_iter()
            .max_by_key(|cap| {
                (cap.performance_metrics.availability_percent * 100.0) as u64
                    + (cap.performance_metrics.success_rate_percent * 100.0) as u64
            })
            .ok_or_else(|| RpcError::ServiceUnavailable("No suitable capability found".to_string()))?;

        // Create route for this capability
        let connection_type = self.determine_optimal_connection_type(category);
        let route = CapabilityRoute {
            category: category.clone(),
            connection_type,
            provider_info: ProviderInfo {
                service_id: best_capability.id.clone(),
                endpoint: format!("discovered://{best_capability.provider}"),
                capabilities: vec![best_capability.name.clone()],
                health_status: ServiceHealthStatus::Healthy,
            },
            performance_tier: best_capability._metadata
                .get("performance_tier")
                .cloned()
                .unwrap_or_else(|| "standard".to_string()),
            last_success: std::time::SystemTime::now(),
        };

        // Cache the discovered route
        let mut cache = self.capability_cache.write().await;
        cache.insert(best_capability.name.clone(), route.clone());

        Ok(route)
    }

    /// Execute request via discovered route
    async fn execute_request_via_route(&self, request: &UnifiedRpcRequest, route: &CapabilityRoute) -> Result<UnifiedRpcResponse, RpcError> {
        debug!(
            "📞 Executing {} request via {:?} connection to {}",
            request.method, route.connection_type, route.provider_info.service_id
        );

        // Create capability-specific request
        let capability_request = self.create_capability_request(request, route).await?;
        
        // Execute via universal adapter
        let response = self.adapter
            .request_capability(&route.provider_info.service_id, capability_request)
            .await
            .map_err(|e| RpcError::ServiceUnavailable(format!("Capability request failed: {e}")))?;

        // Convert capability response to RPC response
        self.convert_capability_response(response, request).await
    }

    /// Create capability request from RPC request
    async fn create_capability_request(&self, rpc_request: &UnifiedRpcRequest, route: &CapabilityRoute) -> Result<serde_json::Value, RpcError> {
        let mut capability_request = serde_json::json!({
            "method": rpc_request.method,
            "parameters": rpc_request.parameters,
            "capability_category": format!("{route.category:?}"),
            "performance_tier": route.performance_tier,
            "connection_type": format!("{route.connection_type:?}"),
        });

        // Add route-specific _metadata
        if let Some(_metadata) = capability_request.as_object_mut() {
            _metadata.insert("provider_id".to_string(), serde_json::Value::String(route.provider_info.service_id.clone()));
            _metadata.insert("request_id".to_string(), serde_json::Value::String(rpc_request.id.clone()));
        }

        Ok(capability_request)
    }

    /// Convert capability response to RPC response
    async fn convert_capability_response(&self, capability_response: serde_json::Value, original_request: &UnifiedRpcRequest) -> Result<UnifiedRpcResponse, RpcError> {
        Ok(UnifiedRpcResponse {
            id: original_request.id.clone(),
            result: Some(capability_response),
            error: None,
            _metadata: HashMap::new(),
        })
    }

    /// Determine optimal connection type for capability category
    fn determine_optimal_connection_type(&self, category: &CapabilityCategory) -> RpcConnectionType {
        self.connection_preferences
            .category_preferences
            .get(category)
            .cloned()
            .unwrap_or_else(|| {
                // Default connection type based on category characteristics
                match category {
                    CapabilityCategory::Security => RpcConnectionType::Tarpc, // High performance for security
                    CapabilityCategory::Orchestration => RpcConnectionType::JsonRpc, // Standard for orchestration
                    CapabilityCategory::Monitoring => RpcConnectionType::WebSocket, // Real-time for monitoring
                    CapabilityCategory::ArtificialIntelligence => RpcConnectionType::JsonRpc, // Standard for AI
                    CapabilityCategory::Compute => RpcConnectionType::Tarpc, // High performance for compute
                    _ => RpcConnectionType::JsonRpc, // Default to JSON RPC
                }
            })
    }

    /// Check if route is healthy
    fn is_route_healthy(&self, route: &CapabilityRoute) -> bool {
        matches!(route.provider_info.health_status, ServiceHealthStatus::Healthy | ServiceHealthStatus::Degraded)
    }

    /// Calculate route score for method matching
    fn calculate_route_score(&self, route: &CapabilityRoute, method: &str) -> u64 {
        let mut score = 0u64;

        // Score based on capability matching
        for capability in &route.provider_info.capabilities {
            if method.contains(capability) {
                score += 100;
            }
        }

        // Score based on health status
        score += match route.provider_info.health_status {
            ServiceHealthStatus::Healthy => 50,
            ServiceHealthStatus::Degraded => 25,
            ServiceHealthStatus::Unhealthy => 0,
            ServiceHealthStatus::Unknown => 10,
        };

        // Score based on performance tier
        score += match route.performance_tier.as_str() {
            "enterprise" => 30,
            "high" => 25,
            "standard" => 20,
            "basic" => 15,
            _ => 10,
        };

        score
    }

    /// Create default connection preferences
    fn create_default_preferences() -> ConnectionPreferences {
        let mut category_preferences = HashMap::new();
        let mut performance_requirements = HashMap::new();

        // Security: High-performance binary RPC
        category_preferences.insert(CapabilityCategory::Security, RpcConnectionType::Tarpc);
        performance_requirements.insert(
            CapabilityCategory::Security,
            PerformanceRequirement {
                max_latency_ms: 2000,
                min_throughput_rps: 100.0,
                min_reliability_percent: 99.5,
            },
        );

        // Orchestration: Standard JSON RPC
        category_preferences.insert(CapabilityCategory::Orchestration, RpcConnectionType::JsonRpc);
        performance_requirements.insert(
            CapabilityCategory::Orchestration,
            PerformanceRequirement {
                max_latency_ms: 5000,
                min_throughput_rps: 50.0,
                min_reliability_percent: 98.0,
            },
        );

        // Monitoring: Real-time WebSocket
        category_preferences.insert(CapabilityCategory::Monitoring, RpcConnectionType::WebSocket);
        performance_requirements.insert(
            CapabilityCategory::Monitoring,
            PerformanceRequirement {
                max_latency_ms: 1000,
                min_throughput_rps: 200.0,
                min_reliability_percent: 95.0,
            },
        );

        // AI: Standard JSON RPC with higher latency tolerance
        category_preferences.insert(CapabilityCategory::ArtificialIntelligence, RpcConnectionType::JsonRpc);
        performance_requirements.insert(
            CapabilityCategory::ArtificialIntelligence,
            PerformanceRequirement {
                max_latency_ms: 30000, // AI operations can take longer
                min_throughput_rps: 10.0,
                min_reliability_percent: 95.0,
            },
        );

        // Compute: High-performance binary RPC
        category_preferences.insert(CapabilityCategory::Compute, RpcConnectionType::Tarpc);
        performance_requirements.insert(
            CapabilityCategory::Compute,
            PerformanceRequirement {
                max_latency_ms: 10000,
                min_throughput_rps: 25.0,
                min_reliability_percent: 98.0,
            },
        );

        ConnectionPreferences {
            category_preferences,
            fallback_types: vec![
                RpcConnectionType::JsonRpc,
                RpcConnectionType::WebSocket,
                RpcConnectionType::Tarpc,
            ],
            performance_requirements,
        }
    }

    /// Refresh capability cache
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn refresh_capabilities(&self) -> Result<(), RpcError>  {
        info!("🔄 Refreshing capability cache");
        
        // Clear existing cache
        {
            let mut cache = self.capability_cache.write().await;
            cache.clear();
        }

        // Reinitialize discovery
        self.initialize_capability_discovery().await?;
        
        info!("✅ Capability cache refreshed successfully");
        Ok(())
    }

    /// Get capability statistics
    pub async fn get_capability_stats(&self) -> HashMap<String, serde_json::Value> {
        let cache = self.capability_cache.read().await;
        let mut stats = HashMap::new();

        stats.insert("total_capabilities".to_string(), serde_json::Value::Number(cache.len().into()));
        
        let mut category_counts = HashMap::new();
        for route in cache.values() {
            let category = format!("{route.category:?}");
            let count = category_counts.entry(category).or_insert(0u64);
            *count += 1;
        }
        
        stats.insert("categories".to_string(), serde_json::to_value(category_counts).unwrap_or_default());
        
        let healthy_count = cache.values().filter(|route| self.is_route_healthy(route)).count();
        stats.insert("healthy_capabilities".to_string(), serde_json::Value::Number(healthy_count.into()));

        stats
    }
} 