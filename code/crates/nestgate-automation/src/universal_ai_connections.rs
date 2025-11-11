//
// **ZERO-COST MODERNIZATION**: Generic composition eliminates Arc<dyn> overhead
// Provides connections to AI primals (Intelligence, etc.) with compile-time dispatch
// for maximum performance in AI inference operations.

use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use std::sync::Arc;
use tokio::sync::RwLock;

// Import required traits and types
use nestgate_core::universal_traits::ComputePrimalProvider;

/// Connection state for AI providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionState {
    /// Connection is establishing
    Connecting,
    /// Connection is active and healthy
    Connected,
    /// Connection is temporarily unavailable
    Disconnected,
    /// Connection failed with error
    Failed(String),
}
/// AI request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIRequest {
    /// Request ID for tracking
    pub id: String,
    /// Request type (e.g., "text-generation", "image-analysis")
    pub request_type: String,
    /// Request payload
    pub payload: serde_json::Value,
    /// Request metadata
    pub metadata: std::collections::HashMap<String, String>,
}
/// AI response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponse {
    /// Request ID this response corresponds to
    pub request_id: String,
    /// Response success status
    pub success: bool,
    /// Response data
    pub data: Option<serde_json::Value>,
    /// Error message if failed
    pub error: Option<String>,
    /// Response metadata
    pub metadata: std::collections::HashMap<String, String>,
}
/// AI operation errors
#[derive(Debug, thiserror::Error)]
pub enum AIError {
    /// No provider configured
    #[error("No AI provider configured")]
    NoProvider,
    /// Provider communication error
    #[error("Provider error: {0}")]
    ProviderError(String),
    /// Request processing error
    #[error("Request processing error: {0}")]
    ProcessingError(String),
}
/// **ZERO-COST AI CONNECTION MANAGER**
/// **PERFORMANCE**: Generic compile-time dispatch eliminates Arc<dyn> overhead
pub struct UniversalAIConnections<P> 
where
    P: ComputePrimalProvider + Send + Sync + 'static,
{
    /// AI provider with zero-cost generic dispatch
    pub provider: Option<Arc<P>>,
    /// Connection state
    pub state: Arc<RwLock<ConnectionState>>,
    /// Generic marker for compile-time optimization
    _marker: PhantomData<P>,
}
impl<P> UniversalAIConnections<P>
where
    P: ComputePrimalProvider + Send + Sync + 'static,
{
    /// Create new AI connections manager
    pub fn new() -> Self {
        Self {
            provider: None,
            state: Arc::new(RwLock::new(ConnectionState::Disconnected)),
            _marker: PhantomData,
        }
    }

    /// Set provider with zero-cost generic dispatch
    #[must_use]
    pub fn with_provider(mut self, provider: Arc<P>) -> Self {
        self.provider = Some(provider);
        self
    }

    /// Get provider with compile-time dispatch
    #[must_use]
    pub fn get_provider(&self) -> Option<Arc<P>> {
        self.provider.clone()
    }

    /// Execute AI request with zero-cost dispatch
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn execute_request(
        &self,
        request: AIRequest,
    ) -> Result<AIResponse, AIError>  {
        if let Some(provider) = &self.provider {
            // Convert AIRequest to provider-specific request format
            let provider_request = serde_json::to_value(&request)
                .map_err(|_e| AIError::ProcessingError(e.to_string()))?;
            
            match provider.process_compute_request(provider_request).await {
                Ok(response) => {
                    Ok(AIResponse {
                        request_id: request.request_id,
                        success: true,
                        data: Some(response),
                        error: None,
                        metadata: std::collections::HashMap::new(),
                    })
                }
                Err(e) => Err(AIError::ProviderError(e.to_string()),
            }
        } else {
            Err(AIError::NoProvider)
        }
    }

    /// Update connection state
    pub async fn set_state(&self, state: ConnectionState) {
        let mut current_state = self.state.write().await;
        *current_state = state;
    }

    /// Get current connection state
    pub async fn get_state(&self) -> ConnectionState {
        self.state.read().await.clone()
    }
}

impl<P> Default for UniversalAIConnections<P>
where
    P: ComputePrimalProvider + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

/// **ZERO-COST AI CONNECTION POOL**
/// **PERFORMANCE**: Generic pool management with compile-time dispatch
pub struct UniversalAIConnectionPool<P>
where
    P: ComputePrimalProvider + Send + Sync + 'static,
{
    /// AI connections by provider ID
    connections: std::collections::HashMap<String, UniversalAIConnections<P>>,
    /// Pool configuration
    config: PoolConfig,
}
/// Pool configuration
#[derive(Debug, Clone)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust
/// // OLD (deprecated):
/// use crate::network::config::PoolConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::PoolConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
pub struct PoolConfig {
    /// Maximum number of connections
    pub max_connections: usize,
    /// Health check interval in seconds
    pub health_check_interval: u64,
    /// Connection timeout in seconds
    pub connection_timeout: u64,
}
impl Default for PoolConfig {
    fn default() -> Self { Self {
            max_connections: 10,
            health_check_interval: 30,
            connection_timeout: 30,
         }
}

impl<P> UniversalAIConnectionPool<P>
where
    P: ComputePrimalProvider + Send + Sync + 'static,
{
    /// Create new AI connection pool
    #[must_use]
    pub fn new(config: PoolConfig) -> Self {
        Self {
            connections: std::collections::HashMap::new(),
            config,
        }
    }

    /// Add provider to pool with zero-cost dispatch
    pub fn add_provider(&mut self, provider_id: String, provider: Arc<P>) {
        let connection = UniversalAIConnections::new().with_provider(provider);
        self.connections.insert(provider_id, connection);
    }

    /// Get provider connection by ID
    pub fn get_connection(&self, provider_id: &str) -> Option<&UniversalAIConnections<P>> {
        self.connections.get(provider_id)
    }

    /// Execute request on best available provider
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn execute_request(&self, request: AIRequest) -> Result<AIResponse, AIError>  {
        // Simple round-robin for now - can be enhanced with load balancing
        if let Some((_, connection)) = self.connections.iter().next() {
            connection.execute_request(request).await
        } else {
            Err(AIError::NoProvider)
        }
    }
}

impl<P> Default for UniversalAIConnectionPool<P>
where
    P: ComputePrimalProvider + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new(PoolConfig::default())
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
pub type PoolConfigCanonical = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using PoolConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

