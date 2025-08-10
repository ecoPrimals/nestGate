//! Universal AI Service Connections
//!
//! Management of dynamic connections to AI primal services with capability-based discovery and intelligent load balancing

use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;

use nestgate_core::universal_traits::{ComputePrimalProvider, ServiceHealth, ServiceInstance};
use std::time::Duration;

// Manual Debug implementation for UniversalAIConnection to handle the provider field
impl std::fmt::Debug for UniversalAIConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UniversalAIConnection")
            .field("provider_id", &self.provider_id)
            .field("endpoint", &self.endpoint)
            .field("capabilities", &self.capabilities)
            .field("provider_type", &self.provider_type)
            .field("last_seen", &self.last_seen)
            .field("response_time_ms", &self.response_time_ms)
            .field("success_rate", &self.success_rate)
            .field("request_count", &self.request_count)
            .field("error_count", &self.error_count)
            .field("is_healthy", &self.is_healthy)
            .field("provider", &self.provider.is_some())
            .finish()
    }
}

/// Universal AI service connection pool for managing dynamic AI provider connections
#[derive(Debug)]
pub struct UniversalAIConnectionPool {
    /// AI provider connections by provider ID
    ai_providers: HashMap<String, UniversalAIConnection>,
    /// NestGate peer connections
    pub nestgate_peers: HashMap<String, String>, // peer_id -> endpoint
    /// Health check timing
    last_health_check: SystemTime,
    health_check_interval: Duration,
    /// Capability requirements for service selection
    required_capabilities: Vec<String>,
}

/// Connection to a universal AI provider with health metrics and capability tracking
#[derive(Clone)]
pub struct UniversalAIConnection {
    /// Provider unique identifier
    pub provider_id: String,
    /// Provider endpoint URL
    pub endpoint: String,
    /// Service capabilities (e.g., "text-generation", "image-analysis", "embedding")
    pub capabilities: Vec<String>,
    /// Provider type (e.g., "llm", "vision", "embedding")
    pub provider_type: String,
    /// Health and performance metrics
    pub last_seen: SystemTime,
    pub response_time_ms: u64,
    pub success_rate: f64,
    pub request_count: u64,
    pub error_count: u64,
    pub is_healthy: bool,
    /// Universal provider interface
    pub provider: Option<Arc<dyn ComputePrimalProvider>>,
    /// HTTP client for direct communication
    #[cfg(feature = "network-integration")]
    pub client: reqwest::Client,
}

impl UniversalAIConnection {
    /// Create new AI connection with capability detection
    pub fn new(provider_id: String, endpoint: String, provider_type: String) -> Self {
        Self {
            provider_id,
            endpoint,
            capabilities: vec![],
            provider_type,
            last_seen: SystemTime::now(),
            response_time_ms: 100, // Default optimistic response time
            success_rate: 1.0,     // Start optimistic
            request_count: 0,
            error_count: 0,
            is_healthy: true,
            provider: None,
            #[cfg(feature = "network-integration")]
            client: reqwest::Client::new(),
        }
    }

    /// Create connection with pre-known capabilities
    pub fn with_capabilities(
        provider_id: String,
        endpoint: String,
        provider_type: String,
        capabilities: Vec<String>,
    ) -> Self {
        let mut connection = Self::new(provider_id, endpoint, provider_type);
        connection.capabilities = capabilities;
        connection
    }

    /// Attach universal provider interface
    pub fn with_provider(mut self, provider: Arc<dyn ComputePrimalProvider>) -> Self {
        self.provider = Some(provider);
        self
    }

    /// Update connection health metrics after a request
    pub fn update_metrics(&mut self, response_time_ms: u64, success: bool) {
        self.last_seen = SystemTime::now();
        self.request_count += 1;

        if success {
            // Weighted average for response time (80% old, 20% new)
            self.response_time_ms = (self.response_time_ms * 4 + response_time_ms) / 5;
        } else {
            self.error_count += 1;
        }

        // Calculate success rate
        self.success_rate = if self.request_count > 0 {
            (self.request_count - self.error_count) as f64 / self.request_count as f64
        } else {
            1.0
        };

        // Update health status based on metrics
        self.is_healthy = self.success_rate > 0.8 && self.response_time_ms < 5000;
    }

    /// Calculate service score for load balancing (higher is better)
    pub fn calculate_score(&self) -> f64 {
        if !self.is_healthy {
            return 0.0;
        }

        // Score based on success rate (70%) and response time (30%)
        let success_score = self.success_rate * 0.7;
        let speed_score = if self.response_time_ms > 0 {
            (1000.0 / self.response_time_ms as f64).min(1.0) * 0.3
        } else {
            0.3
        };

        success_score + speed_score
    }

    /// Check if connection supports required capabilities
    pub fn supports_capabilities(&self, required_caps: &[String]) -> bool {
        required_caps
            .iter()
            .all(|cap| self.capabilities.contains(cap))
    }

    /// Get provider interface if available
    pub async fn get_provider(&self) -> Option<Arc<dyn ComputePrimalProvider>> {
        self.provider.clone()
    }
}

impl UniversalAIConnectionPool {
    /// Create new universal AI connection pool
    pub fn new() -> Self {
        Self {
            ai_providers: HashMap::new(),
            nestgate_peers: HashMap::new(),
            last_health_check: SystemTime::now(),
            health_check_interval: Duration::from_secs(30),
            required_capabilities: vec!["basic-compute".to_string()],
        }
    }

    /// Set required capabilities for service selection
    pub fn set_required_capabilities(&mut self, capabilities: Vec<String>) {
        self.required_capabilities = capabilities;
    }

    /// Get the best available AI provider with capability matching and intelligent load balancing
    pub fn get_best_ai_provider(&self) -> Option<String> {
        self.get_best_ai_provider_with_capabilities(&self.required_capabilities.clone())
    }

    /// Get the best available AI provider for specific capabilities
    pub fn get_best_ai_provider_with_capabilities(
        &self,
        required_caps: &[String],
    ) -> Option<String> {
        if self.ai_providers.is_empty() {
            return None;
        }

        // Find the provider with the highest score that supports required capabilities
        let best_provider = self
            .ai_providers
            .values()
            .filter(|conn| conn.is_healthy && conn.supports_capabilities(required_caps))
            .max_by(|a, b| {
                a.calculate_score()
                    .partial_cmp(&b.calculate_score())
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

        best_provider.map(|conn| conn.endpoint.clone())
    }

    /// Get AI provider by type (e.g., "llm", "vision", "embedding")
    pub fn get_provider_by_type(&self, provider_type: &str) -> Option<String> {
        let matching_providers: Vec<_> = self
            .ai_providers
            .values()
            .filter(|conn| conn.is_healthy && conn.provider_type == provider_type)
            .collect();

        if matching_providers.is_empty() {
            return None;
        }

        // Return the best scored provider of the requested type
        matching_providers
            .iter()
            .max_by(|a, b| {
                a.calculate_score()
                    .partial_cmp(&b.calculate_score())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|conn| conn.endpoint.clone())
    }

    /// Add an AI provider connection
    pub fn add_ai_provider(
        &mut self,
        provider_id: String,
        endpoint: String,
        provider_type: String,
    ) {
        let connection = UniversalAIConnection::new(provider_id.clone(), endpoint, provider_type);
        self.ai_providers.insert(provider_id, connection);
    }

    /// Add an AI provider with capabilities
    pub fn add_ai_provider_with_capabilities(
        &mut self,
        provider_id: String,
        endpoint: String,
        provider_type: String,
        capabilities: Vec<String>,
    ) {
        let connection = UniversalAIConnection::with_capabilities(
            provider_id.clone(),
            endpoint,
            provider_type,
            capabilities,
        );
        self.ai_providers.insert(provider_id, connection);
    }

    /// Attach universal provider interface to existing connection
    pub fn attach_provider_interface(
        &mut self,
        provider_id: &str,
        provider: Arc<dyn ComputePrimalProvider>,
    ) {
        if let Some(connection) = self.ai_providers.get_mut(provider_id) {
            connection.provider = Some(provider);
        }
    }

    /// Update AI provider health metrics
    pub fn update_ai_provider_health(
        &mut self,
        provider_id: &str,
        response_time_ms: u64,
        success: bool,
    ) {
        if let Some(connection) = self.ai_providers.get_mut(provider_id) {
            connection.update_metrics(response_time_ms, success);
        }
    }

    /// Perform health check on all connections (should be called periodically)
    pub async fn perform_health_check(&mut self) {
        let now = SystemTime::now();
        if now
            .duration_since(self.last_health_check)
            .unwrap_or_default()
            < self.health_check_interval
        {
            return;
        }

        self.last_health_check = now;

        // Mark connections as unhealthy if not seen recently
        let stale_threshold = Duration::from_secs(120);
        for connection in self.ai_providers.values_mut() {
            if now.duration_since(connection.last_seen).unwrap_or_default() > stale_threshold {
                connection.is_healthy = false;
            }
        }
    }

    /// Get AI provider statistics
    pub fn get_ai_provider_stats(&self) -> HashMap<String, (f64, u64, bool, Vec<String>)> {
        self.ai_providers
            .iter()
            .map(|(id, conn)| {
                (
                    id.clone(),
                    (
                        conn.success_rate,
                        conn.response_time_ms,
                        conn.is_healthy,
                        conn.capabilities.clone(),
                    ),
                )
            })
            .collect()
    }

    /// Get all healthy providers supporting specific capabilities
    pub fn get_providers_with_capabilities(&self, capabilities: &[String]) -> Vec<String> {
        self.ai_providers
            .values()
            .filter(|conn| conn.is_healthy && conn.supports_capabilities(capabilities))
            .map(|conn| conn.provider_id.clone())
            .collect()
    }

    /// Add a nestgate peer
    pub fn add_nestgate_peer(&mut self, peer_id: String, endpoint: String) {
        self.nestgate_peers.insert(peer_id, endpoint);
    }

    /// Discover AI providers from universal adapter
    pub async fn discover_ai_providers(
        &mut self,
        adapter: Arc<nestgate_core::universal_adapter::UniversalAdapter>,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        let mut discovered_count = 0;

        // Get available compute providers from universal adapter
        if let Some(_compute_provider) = adapter.get_compute_provider().await {
            let provider_instance = ServiceInstance {
                service_id: "discovered-ai-provider".to_string(),
                name: "Universal AI Provider".to_string(),
                capabilities: vec!["text-generation".to_string(), "analysis".to_string()],
                endpoints: {
                    let mut endpoints = std::collections::HashMap::new();
                    // SOVEREIGNTY FIX: Use environment-based endpoint configuration
                    if let Ok(api_endpoint) = std::env::var("API_ENDPOINT") {
                        endpoints.insert("api".to_string(), api_endpoint);
                    } else {
                        endpoints.insert("api".to_string(), "dynamic://api-capability".to_string());
                    }
                    endpoints
                },
                health_status: ServiceHealth::Healthy,
                last_seen: SystemTime::now(),
            };

            let provider_id = provider_instance.service_id.clone();
            let endpoint = provider_instance
                .endpoints
                .get("api")
                // SOVEREIGNTY FIX: Use environment-based endpoint discovery
                .unwrap_or(&std::env::var("DEFAULT_AI_ENDPOINT")
                    .unwrap_or_else(|_| "dynamic://ai-capability".to_string()))
                .clone();
            let connection = UniversalAIConnection::with_capabilities(
                provider_id.clone(),
                endpoint,
                "ai".to_string(),
                provider_instance.capabilities,
            );
            // Note: Removed .with_provider(compute_provider) due to type mismatch
            // Would need proper type conversion from ServiceCapability to ComputePrimalProvider

            self.ai_providers.insert(provider_id, connection);
            discovered_count += 1;
        }

        Ok(discovered_count)
    }
}

impl Default for UniversalAIConnectionPool {
    fn default() -> Self {
        Self::new()
    }
}
