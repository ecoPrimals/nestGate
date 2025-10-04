// **CANONICAL PROVIDER UNIFICATION**
//! Trait definitions and implementations.
// This module provides the unified provider pattern that replaces all
//! scattered provider interfaces with a single canonical provider trait.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;
use std::time::SystemTime;

use crate::unified_enums::service_types::UnifiedServiceType;
use crate::Result;

/// **THE** canonical universal provider trait
/// This replaces ALL scattered provider interfaces
pub trait CanonicalUniversalProvider<T>: Send + Sync + 'static {
    /// Provider configuration type
    type Config: Clone + Send + Sync + 'static;
    /// Provider error type
    type Error: Send + Sync + std::error::Error + 'static;
    /// Provider metadata type
    type Metadata: Clone + Send + Sync + 'static;
    /// Initialize provider with configuration
    fn initialize(&self, config: Self::Config) -> impl Future<Output = Result<()>> + Send;

    /// Provide service instance
    fn provide(&self) -> impl Future<Output = Result<T>> + Send;

    /// Stop provider
    fn stop(&self) -> impl Future<Output = Result<()>> + Send;

    /// Get provider metadata
    fn get_metadata(&self) -> impl Future<Output = Result<Self::Metadata>> + Send;

    /// Get provider health status
    fn health_check(&self) -> impl Future<Output = Result<ProviderHealth>> + Send;

    /// Get supported service types
    fn supported_types(&self) -> impl Future<Output = Result<Vec<UnifiedServiceType>>> + Send;

    /// Check if provider supports a service type
    fn supports_type(
        &self,
        service_type: &UnifiedServiceType,
    ) -> impl Future<Output = Result<bool>> + Send;

    /// Get provider capabilities
    fn get_capabilities(&self) -> impl Future<Output = Result<ProviderCapabilities>> + Send;

    /// Validate configuration
    fn validate_config(
        &self,
        config: &Self::Config,
    ) -> impl Future<Output = Result<Vec<String>>> + Send;
}

/// Provider health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderHealth {
    /// Overall health status
    pub status: HealthStatus,
    /// Health check timestamp
    pub checked_at: SystemTime,
    /// Detailed health information
    pub details: HashMap<String, String>,
    /// Performance metrics
    pub metrics: ProviderMetrics,
}
/// Health status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}
/// Provider capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderCapabilities {
    /// Supported operations
    pub operations: Vec<String>,
    /// Maximum concurrent requests
    pub max_concurrent: Option<u32>,
    /// Supported protocols
    pub protocols: Vec<String>,
    /// Feature flags
    pub features: HashMap<String, bool>,
}
/// Provider performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderMetrics {
    /// Request count
    pub requests_total: u64,
    /// Success count
    pub requests_successful: u64,
    /// Error count
    pub requests_failed: u64,
    /// Average response time (ms)
    pub avg_response_time_ms: f64,
    /// Current active connections
    pub active_connections: u32,
}
/// Security provider trait
pub trait SecurityProvider: CanonicalUniversalProvider<Box<dyn SecurityService>> {
    /// Authenticate request
    fn authenticate(&self, credentials: &[u8]) -> impl Future<Output = Result<AuthToken>> + Send;
    /// Authorize request
    fn authorize(
        &self,
        token: &AuthToken,
        data: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>>> + Send;

    /// Decrypt data
    fn decrypt(&self, data: &[u8]) -> impl Future<Output = Result<Option<Vec<u8>>>> + Send;

    /// Sign data
    fn sign(&self, data: &[u8]) -> impl Future<Output = Result<()>> + Send;

    /// Verify signature
    #[allow(clippy::type_complexity)]
    fn verify(
        &self,
        data: &[u8],
        signature: &[u8],
    ) -> impl Future<Output = Result<Option<(String, Vec<u8>)>>> + Send;
}

/// Storage provider trait
pub trait StorageProvider: CanonicalUniversalProvider<Box<dyn StorageService>> {
    /// Store data
    fn store(&self, key: &str, data: &[u8]) -> impl Future<Output = Result<()>> + Send;
    /// Retrieve data
    fn retrieve(&self, key: &str) -> impl Future<Output = Result<Option<Vec<u8>>>> + Send;

    /// Delete data
    fn delete(&self, key: &str) -> impl Future<Output = Result<()>> + Send;

    /// List keys
    fn list(&self, prefix: Option<&str>) -> impl Future<Output = Result<Vec<String>>> + Send;
}

/// Network provider trait
pub trait NetworkProvider: CanonicalUniversalProvider<Box<dyn NetworkService>> {
    /// Send data
    fn send(&self, destination: &str, data: &[u8]) -> impl Future<Output = Result<()>> + Send;
    /// Receive data
    fn receive(&self, timeout_ms: u64) -> impl Future<Output = Result<Option<Vec<u8>>>> + Send;

    /// Connect to endpoint
    fn connect(&self, endpoint: &str) -> impl Future<Output = Result<ConnectionHandle>> + Send;

    /// Disconnect from endpoint
    fn disconnect(&self, handle: ConnectionHandle) -> impl Future<Output = Result<()>> + Send;
}

/// Service trait definitions
/// **DEPRECATED**: Use canonical security trait instead
#[deprecated(
    since = "0.9.0",
    note = "Use crate::traits::canonical_unified_traits::CanonicalSecurity instead"
)]
pub trait SecurityService: Send + Sync {}
/// **DEPRECATED**: Duplicate trait - use canonical storage system
#[deprecated(
    since = "0.9.0",
    note = "Use crate::traits::canonical_unified_traits::CanonicalStorage instead"
)]
pub trait StorageService: Send + Sync {}
pub trait NetworkService: Send + Sync {}
pub trait CacheService: Send + Sync {}
/// Authentication token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    pub token: String,
    pub expires_at: SystemTime,
    pub permissions: Vec<String>,
}
/// Connection handle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionHandle {
    pub id: String,
    pub endpoint: String,
    pub status: ConnectionStatus,
}
/// Connection status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Connected,
    Connecting,
    Disconnected,
    Error(String),
}
// Default implementations
impl Default for ProviderHealth {
    fn default() -> Self {
        Self {
            status: HealthStatus::Unknown,
            checked_at: SystemTime::now(),
            details: HashMap::new(),
            metrics: ProviderMetrics::default(),
        }
    }
}

impl Default for ProviderCapabilities {
    fn default() -> Self {
        Self {
            operations: Vec::new(),
            max_concurrent: Some(100),
            protocols: Vec::new(),
            features: HashMap::new(),
        }
    }
}

impl Default for ProviderMetrics {
    fn default() -> Self {
        Self {
            requests_total: 0,
            requests_successful: 0,
            requests_failed: 0,
            avg_response_time_ms: 0.0,
            active_connections: 0,
        }
    }
}
