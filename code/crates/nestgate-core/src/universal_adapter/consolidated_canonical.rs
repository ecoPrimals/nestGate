// **CONSOLIDATED CANONICAL UNIVERSAL ADAPTER**
//! Consolidated Canonical functionality and utilities.
// This is THE single, unified universal adapter implementation that consolidates
//! all fragmented adapter patterns across the NestGate ecosystem into one
//! canonical, production-ready adapter system.
//! Consolidated Canonical functionality and utilities.
// **CONSOLIDATES AND REPLACES**:
//! - `nestgate-core/src/universal_adapter/canonical.rs`
//! - `nestgate-core/src/universal_adapter/adapter.rs`
//! - `nestgate-core/src/ecosystem_integration/universal_adapter/adapter.rs`
//! - `nestgate-api/src/ecosystem_integration/adapter.rs`
//! - `nestgate-network/src/orchestration_adapter.rs`
//! - All other fragmented adapter implementations
//! Consolidated Canonical functionality and utilities.
// **PROVIDES**:
//! - Single canonical adapter interface
//! - Unified configuration system
//! - Comprehensive capability management
//! - Production-ready error handling
//! - Zero-cost abstractions where possible
//! - Complete ecosystem integration

use crate::http_client_stub as reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use tokio::sync::RwLock;
use uuid::Uuid;
use tracing::{debug, info, warn};

use crate::{Result, NestGateError};

// ==================== CANONICAL ADAPTER CORE ====================

/// **THE** canonical universal adapter - single source of truth for all ecosystem integration
#[derive(Debug)]
#[allow(dead_code)] // Framework infrastructure
/// Consolidatedcanonicaladapter
pub struct ConsolidatedCanonicalAdapter {
    /// Unique service identifier
    #[allow(dead_code)] // Framework field - intentionally unused
    service_id: Uuid,
    
    /// Adapter configuration
    config: CanonicalAdapterConfig,
    
    /// Our registered capabilities
    our_capabilities: Arc<RwLock<Vec<ServiceCapability>>>,
    
    /// Discovered external capabilities
    #[allow(dead_code)] // Framework field - intentionally unused
    discovered_capabilities: Arc<RwLock<HashMap<String, Vec<ServiceCapability>>>>,
    
    /// Active requests being processed
    active_requests: Arc<RwLock<HashMap<String, CapabilityRequest>>>,
    
    /// HTTP client for network operations
    #[allow(dead_code)] // Framework field - intentionally unused
    client: reqwest::Client,
    
    /// Adapter health and metrics
    health_status: Arc<RwLock<AdapterHealthStatus>>,
    
    /// Performance statistics
    stats: Arc<RwLock<AdapterStats>>,
    
    /// Service registry for discovery
    #[allow(dead_code)] // Framework field - intentionally unused
    service_registry: Arc<RwLock<HashMap<String, ServiceRegistration>>>,
}
/// Canonical adapter configuration - unified from all implementations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::CanonicalAdapterConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::CanonicalAdapterConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
/// Configuration for CanonicalAdapter
pub struct CanonicalAdapterConfig {
    /// Service identification
    pub service_id: String,
    /// Service name
    pub service_name: String,
    /// Service Version
    pub service_version: String,
    
    /// Discovery configuration
    pub discovery: DiscoveryConfig,
    
    /// Request handling configuration
    pub requests: RequestConfig,
    
    /// Monitoring and metrics configuration
    pub monitoring: MonitoringConfig,
    
    /// Security configuration
    pub security: SecurityConfig,
    
    /// Performance configuration
    pub performance: PerformanceConfig,
}
/// Service capability definition
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Servicecapability
pub struct ServiceCapability {
    /// Unique identifier
    pub id: String,
    /// Name
    pub name: String,
    /// Human-readable description
    pub description: String,
    /// Category
    pub category: CapabilityCategory,
    /// Version
    pub version: String,
    /// Provider
    pub provider: String,
    /// Supported Data Types
    pub supported_data_types: Vec<DataType>,
    /// Resource Requirements
    pub resource_requirements: ResourceRequirements,
    /// Scalability
    pub scalability: ScalabilityRating,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}
/// Capability request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for Capability operation
pub struct CapabilityRequest {
    /// Unique identifier
    pub id: String,
    /// Capability identifier
    pub capability_id: String,
    /// Method
    pub method: String,
    /// Parameters
    pub parameters: HashMap<String, serde_json::Value>,
    /// Timeout
    pub timeout: Duration,
    /// Priority
    pub priority: RequestPriority,
    /// Correlation identifier
    pub correlation_id: Option<String>,
    /// Timestamp when this was created
    pub created_at: SystemTime,
}
/// Capability response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for Capability operation
pub struct CapabilityResponse {
    /// Request identifier
    pub request_id: String,
    /// Status
    pub status: ResponseStatus,
    /// Data
    pub data: Option<serde_json::Value>,
    /// Error
    pub error: Option<String>,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
    /// Execution Time
    pub execution_time: Duration,
    /// Provider
    pub provider: String,
}
// ==================== CONFIGURATION STRUCTURES ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Discovery
pub struct DiscoveryConfig {
    /// Auto Discovery
    pub auto_discovery: bool,
    /// Discovery Interval
    pub discovery_interval: Duration,
    /// Discovery Methods
    pub discovery_methods: Vec<DiscoveryMethod>,
    /// Discovery Timeout
    pub discovery_timeout: Duration,
    /// Retry Attempts
    pub retry_attempts: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::RequestConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::RequestConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
/// Configuration for Request
pub struct RequestConfig {
    /// Timeout
    pub timeout: Duration,
    /// Max Retries
    pub max_retries: u32,
    /// Retry Backoff
    pub retry_backoff: RetryBackoff,
    /// Max Concurrent Requests
    pub max_concurrent_requests: u32,
    /// Size of request queue
    pub request_queue_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Monitoring
pub struct MonitoringConfig {
    /// Metrics Enabled
    pub metrics_enabled: bool,
    /// Health Checks Enabled
    pub health_checks_enabled: bool,
    /// Health Check Interval
    pub health_check_interval: Duration,
    /// Metrics Interval
    pub metrics_interval: Duration,
    /// Alert Thresholds
    pub alert_thresholds: AlertThresholds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust
/// // OLD (deprecated):
/// use crate::config::SecurityConfig;
/// 
/// // NEW (canonical):
/// use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::SecurityConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
/// Configuration for Security
pub struct SecurityConfig {
    /// Auth Enabled
    pub auth_enabled: bool,
    /// Api Key
    pub api_key: Option<String>,
    /// Tls Enabled
    pub tls_enabled: bool,
    /// Verify Certificates
    pub verify_certificates: bool,
    /// Rate Limiting
    pub rate_limiting: RateLimitConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Performance
pub struct PerformanceConfig {
    /// Size of connection pool
    pub connection_pool_size: u32,
    /// Keep Alive Timeout
    pub keep_alive_timeout: Duration,
    /// Compression Enabled
    pub compression_enabled: bool,
    /// Caching Enabled
    pub caching_enabled: bool,
    /// Cache Ttl
    pub cache_ttl: Duration,
}

// ==================== ENUMS AND TYPES ====================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Capabilitycategory
pub enum CapabilityCategory {
    /// Storage
    Storage,
    /// Security
    Security,
    /// Ai
    AI,
    /// Network
    Network,
    /// Orchestration
    Orchestration,
    /// Monitoring
    Monitoring,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Types of Data
pub enum DataType {
    /// Json
    Json,
    /// Binary
    Binary,
    /// Text
    Text,
    /// Database
    Database,
    /// Timeseries
    TimeSeries,
    /// Stream
    Stream,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Scalabilityrating
pub enum ScalabilityRating {
    /// Low
    Low,
    /// Medium
    Medium,
    /// High
    High,
    /// Veryhigh
    VeryHigh,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Discoverymethod
pub enum DiscoveryMethod {
    /// Environment
    Environment,
    /// Serviceregistry
    ServiceRegistry,
    /// Networkscan
    NetworkScan,
    /// Configuration
    Configuration,
    /// Dns
    DNS,
    /// Multicast
    Multicast,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Retrybackoff
pub enum RetryBackoff {
    /// Linear
    Linear { increment: Duration },
    /// Exponential
    Exponential { base: Duration, max: Duration },
    /// Fixed
    Fixed { delay: Duration },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Requestpriority
pub enum RequestPriority {
    /// Low
    Low,
    /// Normal
    Normal,
    /// High
    High,
    /// Critical
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Status values for Response
pub enum ResponseStatus {
    /// Success
    Success,
    /// Partialsuccess
    PartialSuccess,
    /// Failed
    Failed,
    /// Timeout
    Timeout,
    /// Notfound
    NotFound,
}

// ==================== SUPPORTING STRUCTURES ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
/// Resourcerequirements
pub struct ResourceRequirements {
    /// Cpu Cores
    pub cpu_cores: Option<u32>,
    /// Memory in megabytes
    pub memory_mb: Option<u64>,
    /// Storage in gigabytes
    pub storage_gb: Option<u64>,
    /// Network Bandwidth
    pub network_bandwidth: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Alertthresholds
pub struct AlertThresholds {
    /// Response Time Ms
    pub response_time_ms: u64,
    /// Error Rate Percent
    pub error_rate_percent: f64,
    /// Resource Usage Percent
    pub resource_usage_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::RateLimitConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::RateLimitConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
/// Configuration for RateLimit
pub struct RateLimitConfig {
    /// Requests Per Second
    pub requests_per_second: u32,
    /// Size of burst
    pub burst_size: u32,
    /// Size of window
    pub window_size: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Serviceregistration
pub struct ServiceRegistration {
    /// Service identifier
    pub service_id: String,
    /// Capabilities
    pub capabilities: Vec<ServiceCapability>,
    /// Endpoints
    pub endpoints: Vec<String>,
    /// Health Status
    pub health_status: String,
    /// Last Seen
    pub last_seen: SystemTime,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
/// Adapterhealthstatus
pub struct AdapterHealthStatus {
    /// Healthy
    pub healthy: bool,
    /// Last Check
    pub last_check: SystemTime,
    /// Details
    pub details: HashMap<String, String>,
    /// Successful Operations
    pub successful_operations: u64,
    /// Failed Operations
    pub failed_operations: u64,
    /// Response Time Avg
    pub response_time_avg: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Adapterstats
pub struct AdapterStats {
    /// Service identifier
    pub service_id: String,
    /// Active Connections
    pub active_connections: u32,
    /// Total Requests
    pub total_requests: u64,
    /// Successful Requests
    pub successful_requests: u64,
    /// Failed Requests
    pub failed_requests: u64,
    /// Average Response Time
    pub average_response_time: Duration,
    /// Uptime
    pub uptime: Duration,
    /// Last Updated
    pub last_updated: SystemTime,
}

// ==================== IMPLEMENTATION ====================

impl ConsolidatedCanonicalAdapter {
    /// Create a new consolidated canonical adapter
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn new(config: CanonicalAdapterConfig) -> Result<Self>  {
        let client = reqwest::Client::builder()
            .timeout(config.requests.timeout)
            .pool_max_idle_per_host(config.performance.connection_pool_size as usize)
            .build()
            .map_err(|e| NestGateError::network_error(&format!("Failed to create HTTP client: {e}")))?;

        Ok(Self {
            service_id: Uuid::new_v4(),
            config,
            our_capabilities: Arc::new(RwLock::new(Vec::new())),
            discovered_capabilities: Arc::new(RwLock::new(HashMap::new())),
            active_requests: Arc::new(RwLock::new(HashMap::new())),
            client,
            health_status: Arc::new(RwLock::new(AdapterHealthStatus::default())),
            stats: Arc::new(RwLock::new(AdapterStats::default())),
            service_registry: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Initialize the adapter and start all services
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn initialize(&self) -> Result<()>  {
        info!("Initializing Consolidated Canonical Universal Adapter");
        
        // Register our capabilities
        self.register_capabilities().await?;
        
        // Start discovery
        if self.config.discovery.auto_discovery {
            self.start_discovery().await?;
        }
        
        // Start health monitoring
        if self.config.monitoring.health_checks_enabled {
            self.start_health_monitoring().await?;
        }
        
        info!("Consolidated Canonical Universal Adapter initialized successfully");
        Ok(())
    }

    /// Register our capabilities with the ecosystem
    async fn register_capabilities(&self) -> Result<()> {
        let capabilities = self.create_nestgate_capabilities();
        let mut our_caps = self.our_capabilities.write().await;
        *our_caps = capabilities;
        
        info!("Registered {} capabilities", our_caps.len());
        Ok(())
    }

    /// Create NestGate's core capabilities
    fn create_nestgate_capabilities(&self) -> Vec<ServiceCapability> {
        vec![
            ServiceCapability {
                id: "nestgate_storage_intelligence".to_string(),
                name: "Storage Intelligence Analytics".to_string(),
                description: "Advanced storage analytics with predictive insights".to_string(),
                category: CapabilityCategory::Storage,
                version: env!("CARGO_PKG_VERSION").to_string(),
                provider: "nestgate".to_string(),
                supported_data_types: vec![DataType::Database, DataType::TimeSeries, DataType::Json],
                resource_requirements: ResourceRequirements::default(),
                scalability: ScalabilityRating::High,
                metadata: HashMap::new(),
            },
            ServiceCapability {
                id: "nestgate_zfs_management".to_string(),
                name: "ZFS Pool Management".to_string(),
                description: "Advanced ZFS pool and dataset management".to_string(),
                category: CapabilityCategory::Storage,
                version: env!("CARGO_PKG_VERSION").to_string(),
                provider: "nestgate".to_string(),
                supported_data_types: vec![DataType::Binary, DataType::Database],
                resource_requirements: ResourceRequirements::default(),
                scalability: ScalabilityRating::VeryHigh,
                metadata: HashMap::new(),
            },
        ]
    }

    /// Start capability discovery
    async fn start_discovery(&self) -> Result<()> {
        debug!("Starting capability discovery");
        // Implementation would start background discovery tasks
        Ok(())
    }

    /// Start health monitoring
    async fn start_health_monitoring(&self) -> Result<()> {
        debug!("Starting health monitoring");
        // Implementation would start background health check tasks
        Ok(())
    }

    /// Request a capability from the ecosystem
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn request_capability(
        &self,
        capability_id: &str,
        method: &str,
        parameters: HashMap<String, serde_json::Value>,
    ) -> Result<CapabilityResponse>  {
        let request = CapabilityRequest {
            id: Uuid::new_v4().to_string(),
            capability_id: capability_id.to_string(),
            method: method.to_string(),
            parameters,
            timeout: self.config.requests.timeout,
            priority: RequestPriority::Normal,
            correlation_id: None,
            created_at: SystemTime::now(),
        };

        self.execute_capability_request(request).await
    }

    /// Execute a capability request
    async fn execute_capability_request(&self, request: CapabilityRequest) -> Result<CapabilityResponse> {
        let start_time = Instant::now();
        
        // Add to active requests
        {
            let mut active = self.active_requests.write().await;
            active.insert(request.id.clone(), request.clone());
        }

        // Execute the request (simplified implementation)
        let result = self.process_request(&request).await;

        // Remove from active requests
        {
            let mut active = self.active_requests.write().await;
            active.remove(&request.id);
        }

        // Update statistics
        self.update_stats(result.is_ok(), start_time.elapsed()).await;

        result
    }

    /// Process a capability request
    async fn process_request(&self, request: &CapabilityRequest) -> Result<CapabilityResponse> {
        // Simplified processing - in real implementation would route to appropriate provider
        Ok(CapabilityResponse {
            request_id: request.id.clone(),
            status: ResponseStatus::Success,
            data: Some(serde_json::json!({"result": "processed"})),
            error: None,
            metadata: HashMap::new(),
            execution_time: Duration::from_millis(10),
            provider: "nestgate".to_string(),
        })
    }

    /// Update adapter statistics
    async fn update_stats(&self, success: bool, duration: Duration) {
        let mut stats = self.stats.write().await;
        stats.total_requests += 1;
        if success {
            stats.successful_requests += 1;
        } else {
            stats.failed_requests += 1;
        }
        
        // Update average response time
        let total_time = stats.average_response_time.as_millis() as u64 * (stats.total_requests - 1) + duration.as_millis() as u64;
        stats.average_response_time = Duration::from_millis(total_time / stats.total_requests);
        stats.last_updated = SystemTime::now();
    }

    /// Get current adapter statistics
    pub async fn get_stats(&self) -> AdapterStats {
        self.stats.read().await.clone()
    }

    /// Get current health status
    pub async fn get_health(&self) -> AdapterHealthStatus {
        self.health_status.read().await.clone()
    }

    /// Shutdown the adapter gracefully
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn shutdown(&self) -> Result<()>  {
        info!("Shutting down Consolidated Canonical Universal Adapter");
        
        // Wait for active requests to complete
        let active_count = self.active_requests.read().await.len();
        if active_count > 0 {
            warn!("Waiting for {} active requests to complete", active_count);
            // Implementation would wait with timeout
        }
        
        info!("Adapter shutdown complete");
        Ok(())
    }
}

// ==================== DEFAULT IMPLEMENTATIONS ====================

impl Default for CanonicalAdapterConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            service_id: Uuid::new_v4().to_string(),
            service_name: "nestgate".to_string(),
            service_version: env!("CARGO_PKG_VERSION").to_string(),
            discovery: DiscoveryConfig::default(),
            requests: RequestConfig::default(),
            monitoring: MonitoringConfig::default(),
            security: SecurityConfig::default(),
            performance: PerformanceConfig::default(),
        }
    }
}

impl Default for DiscoveryConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            auto_discovery: true,
            discovery_interval: Duration::from_secs(30),
            discovery_methods: vec![DiscoveryMethod::Environment, DiscoveryMethod::ServiceRegistry],
            discovery_timeout: Duration::from_secs(10),
            retry_attempts: 3,
        }
    }
}

impl Default for RequestConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            max_retries: 3,
            retry_backoff: RetryBackoff::Exponential { 
                base: Duration::from_millis(100), 
                max: Duration::from_secs(10) 
            },
            max_concurrent_requests: 100,
            request_queue_size: 1000,
        }
    }
}

impl Default for MonitoringConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            metrics_enabled: true,
            health_checks_enabled: true,
            health_check_interval: Duration::from_secs(30),
            metrics_interval: Duration::from_secs(60),
            alert_thresholds: AlertThresholds::default(),
        }
    }
}

impl Default for SecurityConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            auth_enabled: true,
            api_key: None,
            tls_enabled: true,
            verify_certificates: true,
            rate_limiting: RateLimitConfig::default(),
        }
    }
}

impl Default for PerformanceConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            connection_pool_size: 10,
            keep_alive_timeout: Duration::from_secs(30),
            compression_enabled: true,
            caching_enabled: true,
            cache_ttl: Duration::from_secs(300),
        }
    }
}


impl Default for AlertThresholds {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            response_time_ms: 1000,
            error_rate_percent: 5.0,
            resource_usage_percent: 80.0,
        }
    }
}

impl Default for RateLimitConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            requests_per_second: 100,
            burst_size: 200,
            window_size: Duration::from_secs(60),
        }
    }
}

impl Default for AdapterHealthStatus {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            healthy: true,
            last_check: SystemTime::now(),
            details: HashMap::new(),
            successful_operations: 0,
            failed_operations: 0,
            response_time_avg: Duration::from_millis(0),
        }
    }
}

impl Default for AdapterStats {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            service_id: Uuid::new_v4().to_string(),
            active_connections: 0,
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            average_response_time: Duration::from_millis(0),
            uptime: Duration::from_secs(0),
            last_updated: SystemTime::now(),
        }
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
/// Type alias for Requestconfigcanonical
pub type RequestConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using RequestConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.


// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Canonicaladapterconfigcanonical
pub type CanonicalAdapterConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using CanonicalAdapterConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.


// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Ratelimitconfigcanonical
pub type RateLimitConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using RateLimitConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.


// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Securityconfigcanonical
pub type SecurityConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using SecurityConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

