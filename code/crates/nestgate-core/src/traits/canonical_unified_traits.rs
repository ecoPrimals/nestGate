//! **CANONICAL UNIFIED TRAIT SYSTEM**
//!
//! This is THE single source of truth for ALL traits across NestGate,
//! replacing and consolidating 50+ scattered trait definitions.

use std::collections::HashMap;
use std::future::Future;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::{NestGateError, Result};
use crate::config::canonical_master::NestGateCanonicalConfig;
use crate::unified_enums::service_types::UnifiedServiceType;

// ==================== THE CANONICAL SERVICE TRAIT ====================

/// **THE** canonical service trait that replaces ALL service traits
/// This is the single source of truth for all NestGate services
pub trait CanonicalService: Send + Sync + 'static {
    /// Service configuration type
    type Config: Clone + Send + Sync + 'static;
    
    /// Service health status type
    type Health: Clone + Send + Sync + 'static;
    
    /// Service metrics type
    type Metrics: Clone + Send + Sync + 'static;
    
    /// Service error type
    type Error: Send + Sync + std::error::Error + 'static;

    // ==================== CORE SERVICE OPERATIONS ====================

    /// Start the service - native async
    fn start(&self) -> impl Future<Output = Result<(), Self::Error>> + Send;

    /// Stop the service - native async
    fn stop(&self) -> impl Future<Output = Result<(), Self::Error>> + Send;

    /// Check service health - native async
    fn is_healthy(&self) -> impl Future<Output = Result<Self::Health, Self::Error>> + Send;

    /// Get service metrics - native async
    fn get_metrics(&self) -> impl Future<Output = Result<Self::Metrics, Self::Error>> + Send;

    /// Get service capabilities - native async
    fn capabilities(&self) -> impl Future<Output = Result<ServiceCapabilities, Self::Error>> + Send;

    /// Validate configuration - native async
    fn validate_config(&self, config: &Self::Config) -> impl Future<Output = Result<Vec<String>, Self::Error>> + Send;

    // ==================== ADDITIONAL SERVICE METHODS ====================

    /// Get service identifier - PEDANTIC ADDITION
    fn service_id(&self) -> &str {
        "unknown"
    }

    /// Get service type - PEDANTIC ADDITION
    fn service_type(&self) -> UnifiedServiceType {
        UnifiedServiceType::Generic
    }

    /// Initialize service with config - PEDANTIC ADDITION
    fn initialize(&self, config: Self::Config) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async move {
            let _ = config; // Use config parameter
            Ok(())
        }
    }

    /// Health check method - PEDANTIC ADDITION
    fn health_check(&self) -> impl Future<Output = Result<Self::Health, Self::Error>> + Send {
        async move {
            // PEDANTIC: Use is_healthy method instead of default()
            self.is_healthy().await
        }
    }

    /// Shutdown method - PEDANTIC ADDITION  
    fn shutdown(&self) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async move {
            // Default graceful shutdown
            Ok(())
        }
    }

    /// Restart method - PEDANTIC ADDITION
    fn restart(&self) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async move {
            // Default restart implementation
            Ok(())
        }
    }

    /// Update configuration method - PEDANTIC ADDITION
    fn update_config(&self, _config: Self::Config) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async move {
            // Default config update implementation
            Ok(())
        }
    }
}

// ==================== THE CANONICAL PROVIDER TRAIT ====================

/// **THE** canonical provider trait that replaces ALL provider traits
/// This is the single source of truth for all NestGate providers
pub trait CanonicalProvider<T>: Send + Sync + 'static {
    /// Provider configuration type
    type Config: Clone + Send + Sync + 'static;
    
    /// Provider error type
    type Error: Send + Sync + std::error::Error + 'static;
    
    /// Provider metadata type
    type Metadata: Clone + Send + Sync + 'static;

    // ==================== CORE PROVIDER OPERATIONS ====================

    /// Provide service instance - native async
    fn provide(&self, config: Self::Config) -> impl Future<Output = Result<T, Self::Error>> + Send;

    /// Configure provider - native async
    fn configure(&mut self, config: Self::Config) -> impl Future<Output = Result<(), Self::Error>> + Send;

    /// Get provider metadata - native async
    fn metadata(&self) -> impl Future<Output = Result<Self::Metadata, Self::Error>> + Send;

    /// Health check - native async
    fn health_check(&self) -> impl Future<Output = Result<ProviderHealth, Self::Error>> + Send;

    /// Get provider capabilities - native async
    fn capabilities(&self) -> impl Future<Output = Result<ProviderCapabilities, Self::Error>> + Send;
}

// ==================== CANONICAL STORAGE TRAIT ====================

/// **THE** canonical storage trait that replaces ALL storage traits
pub trait CanonicalStorage: CanonicalService {
    /// Storage item type
    type Item: Clone + Send + Sync + 'static;
    
    /// Storage key type
    type Key: Clone + Send + Sync + 'static;

    // ==================== STORAGE OPERATIONS ====================

    /// Read data from storage - native async
    fn read(&self, key: Self::Key) -> impl Future<Output = Result<Option<Self::Item>, Self::Error>> + Send;

    /// Write data to storage - native async
    fn write(&self, key: Self::Key, item: Self::Item) -> impl Future<Output = Result<(), Self::Error>> + Send;

    /// Delete data from storage - native async
    fn delete(&self, key: Self::Key) -> impl Future<Output = Result<bool, Self::Error>> + Send;

    /// List storage keys - native async
    fn list(&self, prefix: Option<Self::Key>) -> impl Future<Output = Result<Vec<Self::Key>, Self::Error>> + Send;

    /// Check if key exists - native async
    fn exists(&self, key: Self::Key) -> impl Future<Output = Result<bool, Self::Error>> + Send;

    /// Get storage usage statistics - native async
    fn usage_stats(&self) -> impl Future<Output = Result<StorageUsageStats, Self::Error>> + Send;
}

// ==================== CANONICAL NETWORK TRAIT ====================

/// **THE** canonical network trait that replaces ALL network service traits
pub trait CanonicalNetwork: CanonicalService {
    /// Request type
    type Request: Clone + Send + Sync + 'static;
    
    /// Response type
    type Response: Clone + Send + Sync + 'static;

    // ==================== NETWORK OPERATIONS ====================

    /// Handle network request - native async
    fn handle_request(&self, request: Self::Request) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send;

    /// Establish connection - native async
    fn connect(&self, endpoint: &str) -> impl Future<Output = Result<ConnectionHandle, Self::Error>> + Send;

    /// Close connection - native async
    fn disconnect(&self, handle: ConnectionHandle) -> impl Future<Output = Result<(), Self::Error>> + Send;

    /// Get connection status - native async
    fn connection_status(&self, handle: ConnectionHandle) -> impl Future<Output = Result<ConnectionStatus, Self::Error>> + Send;

    /// List active connections - native async
    fn list_connections(&self) -> impl Future<Output = Result<Vec<ConnectionHandle>, Self::Error>> + Send;
}

// ==================== CANONICAL SECURITY TRAIT ====================

/// **THE** canonical security trait that replaces ALL security service traits
pub trait CanonicalSecurity: CanonicalService {
    /// Authentication token type
    type Token: Clone + Send + Sync + 'static;
    
    /// User identity type
    type Identity: Clone + Send + Sync + 'static;

    // ==================== SECURITY OPERATIONS ====================

    /// Authenticate user - native async
    fn authenticate(&self, credentials: SecurityCredentials) -> impl Future<Output = Result<Self::Token, Self::Error>> + Send;

    /// Authorize operation - native async
    fn authorize(&self, token: Self::Token, operation: &str) -> impl Future<Output = Result<bool, Self::Error>> + Send;

    /// Validate token - native async
    fn validate_token(&self, token: Self::Token) -> impl Future<Output = Result<Self::Identity, Self::Error>> + Send;

    /// Revoke token - native async
    fn revoke_token(&self, token: Self::Token) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

// ==================== ADDITIONAL CANONICAL TRAITS ====================

/// MCP protocol trait
pub trait CanonicalMcp: CanonicalService {}

/// Automation trait
pub trait CanonicalAutomation: CanonicalService {}

/// Zero-cost service marker
pub trait ZeroCostService: CanonicalService {}

/// Service factory
pub trait CanonicalServiceFactory<T: CanonicalService> {
    fn create_service(&self, config: T::Config) -> impl Future<Output = Result<T, NestGateError>> + Send;
}

/// Provider factory
pub trait CanonicalProviderFactory<T, P: CanonicalProvider<T>> {
    fn create_provider(&self, config: P::Config) -> impl Future<Output = Result<P, NestGateError>> + Send;
}

// ==================== SUPPORTING TYPES ====================

/// Service capabilities
#[derive(Debug, Clone, Default, Serialize, Deserialize)] // PEDANTIC: Added Default derive
pub struct ServiceCapabilities {
    pub can_scale: bool,
    pub can_migrate: bool,
    pub can_backup: bool,
    pub supported_protocols: Vec<String>,
}

/// Provider health status
#[derive(Debug, Clone, Default, Serialize, Deserialize)] // PEDANTIC: Added Default derive  
pub struct ProviderHealth {
    pub is_healthy: bool,
    pub last_check: SystemTime,
    pub health_details: HashMap<String, String>,
}

/// Provider capabilities
#[derive(Debug, Clone, Serialize, Deserialize)] // PEDANTIC: Added Serialize/Deserialize derives
pub struct ProviderCapabilities {
    pub supported_types: Vec<UnifiedServiceType>,
    pub max_instances: Option<u32>,
}

/// Storage usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageUsageStats {
    pub total_items: u64,
    pub total_size_bytes: u64,
    pub available_space_bytes: Option<u64>,
}

/// Connection handle
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ConnectionHandle(pub u64);

/// Connection status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Active,
    Idle,
    Closed,
    Error(String),
}

/// Health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

/// Security credentials
#[derive(Debug, Clone)]
pub struct SecurityCredentials {
    pub username: String,
    pub password: String,
}

/// Cron schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CronSchedule {
    pub expression: String,
}

/// Schedule ID
#[derive(Debug, Clone, Serialize, Deserialize)] // PEDANTIC: Added Serialize/Deserialize derives
pub struct ScheduleId {
    pub id: String,
}

/// Schedule info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleInfo {
    pub id: ScheduleId,
    pub schedule: CronSchedule,
    pub next_run: Option<SystemTime>,
}
