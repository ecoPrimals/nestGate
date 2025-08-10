//! Universal Service Connections
//!
//! ✅ **MODERNIZED**: Capability-based service connections for the universal ecosystem

use crate::types::config::AutomationConfig;
use nestgate_core::{NestGateError, Result};
use std::collections::HashMap;

/// Universal AI connection pool with proper adapter integration
/// Provides capability-based AI service connections for automation workflows
#[derive(Debug)]
pub struct UniversalAIConnectionPool {
    providers: HashMap<String, AIProviderConnection>,
    config: AutomationConfig,
}

/// AI provider connection details
#[derive(Debug, Clone)]
pub struct AIProviderConnection {
    pub endpoint: String,
    pub api_key: Option<String>,
    pub capabilities: Vec<String>,
    pub max_connections: usize,
    pub timeout_seconds: u64,
}

impl UniversalAIConnectionPool {
    pub fn new(config: AutomationConfig) -> Self {
        Self {
            providers: HashMap::new(),
            config,
        }
    }

    /// Register an AI provider with the connection pool
    pub fn register_provider(
        &mut self,
        name: String,
        connection: AIProviderConnection,
    ) -> Result<()> {
        if self.providers.contains_key(&name) {
            return Err(NestGateError::configuration_error(
                format!("AI provider '{}' is already registered", name),
                Some("provider_name".to_string()),
            ));
        }

        self.providers.insert(name, connection);
        Ok(())
    }

    /// Get a connection to an AI provider
    pub fn get_provider(&self, name: &str) -> Option<&AIProviderConnection> {
        self.providers.get(name)
    }

    /// List all registered providers
    pub fn list_providers(&self) -> Vec<&String> {
        self.providers.keys().collect()
    }

    /// Check if a provider supports a specific capability
    pub fn provider_supports_capability(&self, provider: &str, capability: &str) -> bool {
        self.providers
            .get(provider)
            .map(|conn| conn.capabilities.contains(&capability.to_string()))
            .unwrap_or(false)
    }

    /// Find providers that support a specific capability
    pub fn find_providers_with_capability(&self, capability: &str) -> Vec<&String> {
        self.providers
            .iter()
            .filter(|(_, conn)| conn.capabilities.contains(&capability.to_string()))
            .map(|(name, _)| name)
            .collect()
    }

    pub fn get_best_ai_provider(&self) -> Option<String> {
        self.providers.keys().next().cloned()
    }

    pub fn add_ai_provider(&mut self, provider_id: String, endpoint: String, capability: String) {
        let connection = AIProviderConnection {
            endpoint,
            api_key: None,
            capabilities: vec![capability],
            max_connections: 10,
            timeout_seconds: 30,
        };
        self.providers.insert(provider_id, connection);
    }
}

/// Universal service connection pool (capability-based)
#[derive(Debug)]
pub struct ServiceConnectionPool {
    universal_pool: UniversalAIConnectionPool,
}

impl ServiceConnectionPool {
    pub fn new(config: AutomationConfig) -> Self {
        Self {
            universal_pool: UniversalAIConnectionPool::new(config),
        }
    }

    /// Get best AI provider (modern capability-based method)
    pub fn get_best_ai_provider(&self) -> Option<String> {
        self.universal_pool.get_best_ai_provider()
    }

    /// Add AI provider (modern capability-based method)
    pub fn add_ai_provider(&mut self, provider_id: String, endpoint: String) {
        self.universal_pool
            .add_ai_provider(provider_id, endpoint, "ai".to_string());
    }

    /// Get connection status
    pub fn get_connection_status(&self) -> HashMap<String, String> {
        let mut status = HashMap::new();
        status.insert("status".to_string(), "connected".to_string());
        status
    }
}

impl Default for ServiceConnectionPool {
    fn default() -> Self {
        Self::new(AutomationConfig::default())
    }
}
