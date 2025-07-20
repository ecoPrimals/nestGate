//! Service Connections (Legacy compatibility layer)
//!
//! Management of dynamic connections to ecosystem services with intelligent load balancing
//! This module provides legacy compatibility while transitioning to universal AI provider architecture.

use std::collections::HashMap;
use std::time::{Duration, SystemTime};

use crate::universal_ai_connections::{UniversalAIConnection, UniversalAIConnectionPool};

/// Service connection pool for managing dynamic ecosystem connections
/// This is now a compatibility wrapper around UniversalAIConnectionPool
#[derive(Debug)]
pub struct ServiceConnectionPool {
    /// Universal AI connection pool (replaces squirrel_connections)
    universal_pool: UniversalAIConnectionPool,
    /// NestGate peer connections
    pub nestgate_peers: HashMap<String, String>, // peer_id -> endpoint
    /// Health check timing
    last_health_check: SystemTime,
    /// Health check interval - planned for future health monitoring feature
    #[allow(dead_code)]
    health_check_interval: Duration,
}

/// Legacy compatibility alias for SquirrelConnection
/// Now backed by UniversalAIConnection
pub type SquirrelConnection = UniversalAIConnection;

impl ServiceConnectionPool {
    /// Create new service connection pool with universal AI provider support
    pub fn new() -> Self {
        Self {
            universal_pool: UniversalAIConnectionPool::new(),
            nestgate_peers: HashMap::new(),
            last_health_check: SystemTime::now(),
            health_check_interval: Duration::from_secs(30),
        }
    }

    /// Get the best available AI service with intelligent load balancing
    /// Legacy compatibility method that now uses universal AI provider selection
    pub fn get_best_squirrel(&self) -> Option<String> {
        self.universal_pool.get_best_squirrel()
    }

    /// Add an AI provider connection
    /// Legacy compatibility method that adds to universal AI provider pool
    pub fn add_squirrel(&mut self, squirrel_id: String, endpoint: String) {
        self.universal_pool.add_squirrel(squirrel_id, endpoint);
    }

    /// Update AI provider health metrics
    /// Legacy compatibility method that delegates to universal pool
    pub fn update_squirrel_health(
        &mut self,
        squirrel_id: &str,
        response_time_ms: u64,
        success: bool,
    ) {
        self.universal_pool
            .update_squirrel_health(squirrel_id, response_time_ms, success);
    }

    /// Perform health check on all connections (should be called periodically)
    pub async fn perform_health_check(&mut self) {
        // Delegate to universal pool
        self.universal_pool.perform_health_check().await;

        // Update local timing for compatibility
        self.last_health_check = SystemTime::now();
    }

    /// Get AI provider connection statistics
    /// Legacy compatibility method that returns squirrel-style stats
    pub fn get_squirrel_stats(&self) -> HashMap<String, (f64, u64, bool)> {
        self.universal_pool.get_squirrel_stats()
    }

    /// Add a nestgate peer
    pub fn add_nestgate_peer(&mut self, peer_id: String, endpoint: String) {
        self.nestgate_peers
            .insert(peer_id.clone(), endpoint.clone());
        // Also add to universal pool for unified management
        self.universal_pool.add_nestgate_peer(peer_id, endpoint);
    }

    /// Get access to the universal AI connection pool
    pub fn universal_pool(&self) -> &UniversalAIConnectionPool {
        &self.universal_pool
    }

    /// Get mutable access to the universal AI connection pool
    pub fn universal_pool_mut(&mut self) -> &mut UniversalAIConnectionPool {
        &mut self.universal_pool
    }

    /// Discover AI providers using universal adapter
    pub async fn discover_ai_providers(
        &mut self,
        adapter: &nestgate_core::universal_adapter::UniversalPrimalAdapter,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        self.universal_pool.discover_ai_providers(adapter).await
    }

    /// Add AI provider with capabilities (new universal method)
    pub fn add_ai_provider_with_capabilities(
        &mut self,
        provider_id: String,
        endpoint: String,
        provider_type: String,
        capabilities: Vec<String>,
    ) {
        self.universal_pool.add_ai_provider_with_capabilities(
            provider_id,
            endpoint,
            provider_type,
            capabilities,
        );
    }

    /// Get the best AI provider for specific capabilities
    pub fn get_best_ai_provider_with_capabilities(
        &self,
        capabilities: &[String],
    ) -> Option<String> {
        self.universal_pool
            .get_best_ai_provider_with_capabilities(capabilities)
    }

    /// Get AI provider by type
    pub fn get_provider_by_type(&self, provider_type: &str) -> Option<String> {
        self.universal_pool.get_provider_by_type(provider_type)
    }
}

impl Default for ServiceConnectionPool {
    fn default() -> Self {
        Self::new()
    }
}
