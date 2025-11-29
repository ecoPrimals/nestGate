/// **SMART SERVICE PATTERNS MODULE**
///
/// This module provides intelligent abstractions for common service patterns,
/// eliminating the need for scattered helper functions and reducing boilerplate.
///
/// **ABSORBS COMPLEXITY FROM**:
/// - tests/common/test_helpers.rs (service creation helpers)
/// - tests/common/consolidated_mocks.rs (mock service patterns)
/// - Various integration test helper functions
/// - Service discovery and registration patterns
/// - Health check and monitoring patterns
///
/// **PROVIDES**:
/// - Generic service factories with smart defaults
/// - Service lifecycle management abstractions
/// - Health monitoring patterns
/// - Service discovery abstractions
/// - Mock service generation
use crate::Result;
use crate::traits::{UniversalServiceRequest, UniversalServiceResponse};
use crate::unified_constants;
use crate::unified_enums::service_types::UnifiedServiceType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use uuid::Uuid;
// ==================== SECTION ====================

/// **SMART SERVICE FACTORY**
/// Eliminates the need for scattered service creation helper functions
/// by providing intelligent service generation with smart defaults.
pub struct SmartServiceFactory {
    service_registry: Arc<RwLock<HashMap<String, ServiceMetadata>>>,
    default_config: ServiceFactoryConfig,
}
impl SmartServiceFactory {
    /// Create a new smart service factory
    #[must_use]
    pub fn new() -> Self {
        Self {
            service_registry: Arc::new(RwLock::new(HashMap::new())),
            default_config: ServiceFactoryConfig::default(),
        }
    }

    /// Create a service with intelligent defaults
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn create_service<T>(
        &self,
        service_type: UnifiedServiceType,
    ) -> Result<Box<dyn SmartService>>
    where
        T: SmartService + 'static,
     {
        let service_id = Uuid::new_v4().to_string();
        let metadata = ServiceMetadata {
            service_id: service_id.clone(),
            service_type: service_type.clone(),
            created_at: SystemTime::now(),
            health_status: UnifiedHealthStatus::Healthy,
            capabilities: self.get_default_capabilities(&service_type),
            endpoints: self.generate_default_endpoints(&service_type),
            configuration: self.generate_default_config(&service_type),
        };

        // Register the service
        self.service_registry
            .write()
            .await
            .insert(service_id.clone(), metadata.clone());

        // Create the service instance
        let service = SmartServiceWrapper::new(metadata, self.default_config.clone());
        Ok(Box::new(service))
    }

    /// Create a mock service for testing
    ///
    /// **TEST ONLY**: This function should only be used in test code
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    #[cfg(any(test, feature = "dev-stubs"))]
    pub fn create_mock_service(
        &self,
        service_type: UnifiedServiceType,
        behavior: MockServiceBehavior,
    ) -> Result<Box<dyn SmartService>>  {
        let service_id = format!("mock-{}", Uuid::new_v4());
        let metadata = ServiceMetadata {
            service_id: service_id.clone(),
            service_type,
            created_at: SystemTime::now(),
            health_status: UnifiedHealthStatus::Healthy,
            capabilities: vec!["mock".to_string()],
            endpoints: HashMap::new(),
            configuration: HashMap::new(),
        };

        let mock_service = MockSmartService::new(metadata, behavior);
        Ok(Box::new(mock_service))
    }

    /// Get service registry for discovery
    pub async fn get_service_registry(&self) -> HashMap<String, ServiceMetadata> {
        self.service_registry.read().await.clone()
    }

    /// Generate default capabilities for a service type
    fn get_default_capabilities(&self, service_type: &UnifiedServiceType) -> Vec<String> {
        use unified_constants::api::capabilities;

        match service_type {
            UnifiedServiceType::Storage => vec![
                capabilities::STORAGE.to_string(),
                capabilities::TIERED_STORAGE.to_string(),
                capabilities::SNAPSHOTS.to_string(),
            ],
            UnifiedServiceType::Network => vec![
                capabilities::FEDERATION.to_string(),
                capabilities::COORDINATION.to_string(),
            ],
            UnifiedServiceType::Security => vec![
                capabilities::AUTHENTICATION.to_string(),
                capabilities::ENCRYPTION.to_string(),
            ],
            UnifiedServiceType::AI => vec![
                capabilities::AI.to_string(),
                capabilities::ML.to_string(),
                capabilities::AGENTS.to_string(),
            ],
            _ => vec![capabilities::COMPUTE.to_string()],
        }
    }

    /// Generate default endpoints for a service type
    fn generate_default_endpoints(
        &self,
        service_type: &UnifiedServiceType,
    ) -> HashMap<String, String> {
        let mut endpoints = HashMap::new();
        let discovery_config = crate::config::discovery_config::ServiceDiscoveryConfig::default();

        let base_port = match service_type {
            UnifiedServiceType::Storage => unified_constants::network::ports::API_PORT,
            UnifiedServiceType::Network => unified_constants::network::ports::DISCOVERY_PORT,
            UnifiedServiceType::Security => unified_constants::network::ports::ADMIN_PORT,
            _ => unified_constants::network::ports::API_PORT,
        };

        let base_endpoint = discovery_config.build_endpoint(base_port);
        let metrics_endpoint = discovery_config.build_endpoint(base_port + 1);

        endpoints.insert(
            "health".to_string(),
            format!("{}/health", base_endpoint),
        );
        endpoints.insert(
            "metrics".to_string(),
            format!("{}/metrics", metrics_endpoint),
        );
        endpoints.insert(
            "api".to_string(),
            format!("{}/api/v1", base_endpoint),
        );

        endpoints
    }

    /// Generate default configuration for a service type
    fn generate_default_config(
        &self,
        service_type: &UnifiedServiceType,
    ) -> HashMap<String, String> {
        let mut config = HashMap::new();

        match service_type {
            UnifiedServiceType::Storage => {
                config.insert("pool_size".to_string(), "1000".to_string());
                config.insert("compression".to_string(), "lz4".to_string());
                config.insert("replication_factor".to_string(), "3".to_string());
            }
            UnifiedServiceType::Network => {
                config.insert("max_connections".to_string(), "1000".to_string());
                config.insert("timeout_secs".to_string(), "30".to_string());
            }
            UnifiedServiceType::Security => {
                config.insert("session_timeout_secs".to_string(), "3600".to_string());
                config.insert("max_login_attempts".to_string(), "5".to_string());
            }
            _ => {
                config.insert("worker_threads".to_string(), "8".to_string());
                config.insert("memory_limit_mb".to_string(), "1024".to_string());
            }
        }

        config
    }
}

impl Default for SmartServiceFactory {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

// ==================== SECTION ====================

/// **SMART SERVICE TRAIT**
/// Enhanced service trait with intelligent defaults and lifecycle management
pub trait SmartService: Send + Sync {
    /// Get service metadata
    fn metadata(&self) -> &ServiceMetadata;
    /// Start the service with intelligent initialization
    fn start(&mut self) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Stop the service gracefully
    fn stop(&mut self) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Get current health status
    fn health_check(&self) -> impl std::future::Future<Output = Result<UnifiedHealthStatus>> + Send;

    /// Handle service requests with intelligent routing - **NATIVE ASYNC**: Zero-cost request handling
    fn handle_request(
        &self,
        request: UniversalServiceRequest,
    ) -> impl std::future::Future<Output = Result<UniversalServiceResponse>> + Send;

    /// Get service metrics
    fn get_metrics(&self) -> impl std::future::Future<Output = Result<ServiceMetrics>> + Send;

    /// Update service configuration dynamically
    fn update_config(&mut self, config: HashMap<String, String>) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Get service capabilities
    fn capabilities(&self) -> &[String];
}

// ==================== SECTION ====================

/// Comprehensive service metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Servicemetadata
pub struct ServiceMetadata {
    /// Service identifier
    pub service_id: String,
    /// Service Type
    pub service_type: UnifiedServiceType,
    /// Timestamp when this was created
    pub created_at: SystemTime,
    /// Health Status
    pub health_status: UnifiedHealthStatus,
    /// Capabilities
    pub capabilities: Vec<String>,
    /// Endpoints
    pub endpoints: HashMap<String, String>,
    /// Configuration for uration
    pub configuration: HashMap<String, String>,
}
/// Service factory configuration
#[derive(Debug, Clone)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::ServiceFactoryConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::ServiceFactoryConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
/// Configuration for ServiceFactory
pub struct ServiceFactoryConfig {
    /// Default Timeout
    pub default_timeout: Duration,
    /// Health Check Interval
    pub health_check_interval: Duration,
    /// Max Retries
    pub max_retries: u32,
    /// Enable Metrics
    pub enable_metrics: bool,
    /// Enable Tracing
    pub enable_tracing: bool,
}
impl Default for ServiceFactoryConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            default_timeout: Duration::from_secs(30),
            health_check_interval: Duration::from_secs(60),
            max_retries: 3,
            enable_metrics: true,
            enable_tracing: true,
        }
    }
}

/// Service performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Servicemetrics
pub struct ServiceMetrics {
    /// Requests Total
    pub requests_total: u64,
    /// Requests Successful
    pub requests_successful: u64,
    /// Requests Failed
    pub requests_failed: u64,
    /// Average Response Time Ms
    pub average_response_time_ms: f64,
    /// Uptime Seconds
    pub uptime_seconds: u64,
    /// Memory Usage in megabytes
    pub memory_usage_mb: u64,
    /// Cpu Usage Percent
    pub cpu_usage_percent: f64,
    /// Custom Metrics
    pub custom_metrics: HashMap<String, f64>,
}
impl Default for ServiceMetrics {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            requests_total: 0,
            requests_successful: 0,
            requests_failed: 0,
            average_response_time_ms: 0.0,
            uptime_seconds: 0,
            memory_usage_mb: 0,
            cpu_usage_percent: 0.0,
            custom_metrics: HashMap::new(),
        }
    }
}

// ==================== SECTION ====================

/// Generic smart service wrapper that provides intelligent defaults
#[allow(dead_code)] // Service wrapper - field used internally
/// Smartservicewrapper
pub struct SmartServiceWrapper {
    metadata: ServiceMetadata,
    config: ServiceFactoryConfig,
    state: UnifiedServiceState,
    metrics: ServiceMetrics,
    start_time: Option<SystemTime>,
}
impl SmartServiceWrapper {
    /// Creates a new instance
    pub fn new(metadata: ServiceMetadata, config: ServiceFactoryConfig) -> Self {
        Self {
            metadata,
            config,
            state: UnifiedServiceState::Stopped,
            metrics: ServiceMetrics::default(),
            start_time: None,
        }
    }
}

impl SmartService for SmartServiceWrapper {
    /// Metadata
    fn metadata(&self) -> &ServiceMetadata {
        &self.metadata
    }

    /// Start
    async fn start(&mut self) -> Result<()> {
        self.state = UnifiedServiceState::Starting;

        // Intelligent initialization based on service type
        match self.metadata.service_type {
            UnifiedServiceType::Storage => {
                // Initialize storage subsystems
                tracing::info!("Initializing storage service: {}", self.metadata.service_id);
            }
            UnifiedServiceType::Network => {
                // Initialize network subsystems
                tracing::info!("Initializing network service: {}", self.metadata.service_id);
            }
            _ => {
                tracing::info!("Initializing generic service: {}", self.metadata.service_id);
            }
        }

        self.state = UnifiedServiceState::Running;
        self.start_time = Some(SystemTime::now());

        Ok(())
    }

    /// Stop
    async fn stop(&mut self) -> Result<()> {
        self.state = UnifiedServiceState::Stopping;

        // Graceful shutdown with intelligent cleanup
        tracing::info!("Stopping service: {}", self.metadata.service_id);

        // Wait for ongoing operations to complete
        tokio::time::sleep(Duration::from_millis(100)).await;

        self.state = UnifiedServiceState::Stopped;
        Ok(())
    }

    /// Health Check
    async fn health_check(&self) -> Result<UnifiedHealthStatus> {
        match self.state {
            UnifiedServiceState::Running => Ok(UnifiedHealthStatus::Healthy),
            UnifiedServiceState::Starting | UnifiedServiceState::Stopping => {
                Ok(UnifiedHealthStatus::Degraded)
            }
            UnifiedServiceState::Stopped => Ok(UnifiedHealthStatus::Unhealthy),
            _ => Ok(UnifiedHealthStatus::Unknown),
        }
    }

    /// Handles  Request
    fn handle_request(
        &self,
        request: UniversalServiceRequest,
    ) -> impl std::future::Future<Output = Result<UniversalServiceResponse>> + Send {
        async move {
            let start_time = std::time::Instant::now();

            // Intelligent request routing based on service type
            let response_data = match self.metadata.service_type {
                UnifiedServiceType::Storage => {
                    Some(b"Storage service response".to_vec())
                }
                UnifiedServiceType::Network => {
                    Some(b"Network service response".to_vec())
                }
                UnifiedServiceType::Security => {
                    Some(b"Security service response".to_vec())
                }
                _ => None,
            };

            // Update metrics
            let processing_time = start_time.elapsed().as_millis() as u64;
            // Note: In a real implementation, you'd update metrics atomically

            let response = UniversalServiceResponse {
                request_id: request.request_id,
                service_id: self.metadata.service_id.clone(),
                status: if response_data.is_some() {
                    crate::traits::UniversalResponseStatus::Success
                } else {
                    crate::traits::UniversalResponseStatus::NotSupported
                },
                data: response_data,
                error: None,
            };

            Ok(response)
        }
    }

    /// Gets Metrics
    async fn get_metrics(&self) -> Result<ServiceMetrics> {
        let mut metrics = self.metrics.clone();
        metrics.uptime_seconds = self.get_uptime_seconds();
        Ok(metrics)
    }

    /// Updates  Config
    async fn update_config(&mut self, config: HashMap<String, String>) -> Result<()> {
        // Intelligent configuration updates
        for (key, value) in config {
            self.metadata.configuration.insert(key, value);
        }

        tracing::info!(
            "Updated configuration for service: {}",
            self.metadata.service_id
        );
        Ok(())
    }

    /// Capabilities
    fn capabilities(&self) -> &[String] {
        &self.metadata.capabilities
    }
}

impl SmartServiceWrapper {
    /// Gets Uptime Seconds
    fn get_uptime_seconds(&self) -> u64 {
        self.start_time
            .map(|start| {
                SystemTime::now()
                    .duration_since(start)
                    .unwrap_or_default()
                    .as_secs()
            })
            .unwrap_or(0)
    }
}

// ==================== SECTION ====================

/// Mock service behavior configuration
#[cfg(any(test, feature = "dev-stubs"))]
#[derive(Debug, Clone)]
/// Mockservicebehavior
pub struct MockServiceBehavior {
    /// Response Delay
    pub response_delay: Duration,
    /// Failure Rate
    pub failure_rate: f64,
    /// Custom Responses
    pub custom_responses: HashMap<String, serde_json::Value>,
    /// Health Status
    pub health_status: UnifiedHealthStatus,
}

#[cfg(any(test, feature = "dev-stubs"))]
impl Default for MockServiceBehavior {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            response_delay: Duration::from_millis(10),
            failure_rate: 0.0,
            custom_responses: HashMap::new(),
            health_status: UnifiedHealthStatus::Healthy,
        }
    }
}

/// Mock smart service for testing
#[cfg(any(test, feature = "dev-stubs"))]
/// Service implementation for MockSmart
pub struct MockSmartService {
    metadata: ServiceMetadata,
    behavior: MockServiceBehavior,
    state: UnifiedServiceState,
    metrics: ServiceMetrics,
    start_time: Option<SystemTime>,
}

#[cfg(any(test, feature = "dev-stubs"))]
impl MockSmartService {
    /// Creates a new instance
    pub fn new(metadata: ServiceMetadata, behavior: MockServiceBehavior) -> Self {
        Self {
            metadata,
            behavior,
            state: UnifiedServiceState::Stopped,
            metrics: ServiceMetrics::default(),
            start_time: None,
        }
    }
}

#[cfg(any(test, feature = "dev-stubs"))]
impl SmartService for MockSmartService {
    /// Metadata
    fn metadata(&self) -> &ServiceMetadata {
        &self.metadata
    }

    /// Start
    async fn start(&mut self) -> Result<()> {
        self.state = UnifiedServiceState::Running;
        self.start_time = Some(SystemTime::now());
        Ok(())
    }

    /// Stop
    async fn stop(&mut self) -> Result<()> {
        self.state = UnifiedServiceState::Stopped;
        Ok(())
    }

    /// Health Check
    async fn health_check(&self) -> Result<UnifiedHealthStatus> {
        Ok(self.behavior.health_status.clone())
    }

    /// Handles  Request
    fn handle_request(
        &self,
        request: UniversalServiceRequest,
    ) -> impl std::future::Future<Output = Result<UniversalServiceResponse>> + Send {
        let behavior = self.behavior.clone();
        async move {
            // Simulate response delay
            tokio::time::sleep(behavior.response_delay).await;

            // Simulate failures
            if behavior.failure_rate > 0.0 && rand::random::<f64>() < behavior.failure_rate {
                return Ok(UniversalServiceResponse {
                    request_id: request.request_id,
                    service_id: "mock-service".to_string(),
                    status: crate::traits::UniversalResponseStatus::Error,
                    data: None,
                    error: Some("Simulated failure".to_string()),
                );
            }

            // Check for custom responses
            let response_data = behavior
                .custom_responses
                .get(&request.operation)
                .cloned()
                .or_else(|| Some(b"Mock response".to_vec()));

            Ok(UniversalServiceResponse {
                request_id: request.request_id,
                service_id: "mock-service".to_string(),
                status: crate::traits::UniversalResponseStatus::Success,
                data: response_data,
                error: None,
            })
        }
    }

    /// Gets Metrics
    async fn get_metrics(&self) -> Result<ServiceMetrics> {
        Ok(self.metrics.clone())
    }

    /// Updates  Config
    async fn update_config(&mut self, _config: HashMap<String, String>) -> Result<()> {
        Ok(())
    }

    /// Capabilities
    fn capabilities(&self) -> &[String] {
        &self.metadata.capabilities
    }
}

// ==================== SECTION ====================

/// **SMART SERVICE DISCOVERY**
/// Intelligent service discovery with automatic registration and health monitoring
pub struct SmartServiceDiscovery {
    services: Arc<RwLock<HashMap<String, ServiceRegistration>>>,
    health_monitor: Arc<RwLock<HashMap<String, HealthRecord>>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Serviceregistration
pub struct ServiceRegistration {
    /// Service identifier
    pub service_id: String,
    /// Service Type
    pub service_type: UnifiedServiceType,
    /// Endpoints
    pub endpoints: HashMap<String, String>,
    /// Capabilities
    pub capabilities: Vec<String>,
    /// Registered At
    pub registered_at: SystemTime,
    /// Last Heartbeat
    pub last_heartbeat: SystemTime,
}

#[derive(Debug, Clone)]
/// Healthrecord
pub struct HealthRecord {
    /// Service identifier
    pub service_id: String,
    /// Status
    pub status: UnifiedHealthStatus,
    /// Last Check
    pub last_check: SystemTime,
    /// Consecutive Failures
    pub consecutive_failures: u32,
}

impl SmartServiceDiscovery {
    #[must_use]
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            health_monitor: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a service with intelligent defaults
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
                pub fn register_service(&self, service: &dyn SmartService) -> Result<()>  {
        let metadata = service.metadata();
        let registration = ServiceRegistration {
            service_id: metadata.service_id.clone(),
            service_type: metadata.service_type.clone(),
            endpoints: metadata.endpoints.clone(),
            capabilities: metadata.capabilities.clone(),
            registered_at: SystemTime::now(),
            last_heartbeat: SystemTime::now(),
        };

        self.services
            .write()
            .await
            .insert(metadata.service_id.clone(), registration);

        // Initialize health record
        let health_record = HealthRecord {
            service_id: metadata.service_id.clone(),
            status: UnifiedHealthStatus::Healthy,
            last_check: SystemTime::now(),
            consecutive_failures: 0,
        };

        self.health_monitor
            .write()
            .await
            .insert(metadata.service_id.clone(), health_record);

        tracing::info!(
            "Registered service: {} ({:?})",
            metadata.service_id,
            metadata.service_type
        );
        Ok(())
    }

    /// Discover services by type with intelligent filtering
    pub async fn discover_services(
        &self,
        service_type: UnifiedServiceType,
    ) -> Vec<ServiceRegistration> {
        self.services
            .read()
            .await
            .values()
            .filter(|reg| reg.service_type == service_type)
            .cloned()
            .collect()
    }

    /// Get healthy services only
    pub async fn get_healthy_services(
        &self,
        service_type: UnifiedServiceType,
    ) -> Vec<ServiceRegistration> {
        let services = self.services.read().await;
        let health_records = self.health_monitor.read().await;

        services
            .values()
            .filter(|reg| {
                reg.service_type == service_type
                    && health_records
                        .get(&reg.service_id)
                        .map(|h| matches!(h.status, UnifiedHealthStatus::Healthy))
                        .unwrap_or(false)
            })
            .cloned()
            .collect()
    }

    /// Start health monitoring background task
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn start_health_monitoring(&self) -> Result<()>  {
        let services = self.services.clone();
        let health_monitor = self.health_monitor.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(30));

            loop {
                interval.tick().await;

                let service_list: Vec<String> = services.read().await.keys().cloned().collect();

                for service_id in service_list {
                    // In a real implementation, this would make actual health check calls
                    // For now, we'll simulate health status updates
                    let mut health_records = health_monitor.write().await;
                    if let Some(record) = health_records.get_mut(&service_id) {
                        record.last_check = SystemTime::now();
                        // Simulate occasional health issues
                        if rand::random::<f64>() < 0.05 {
                            // 5% chance of health issue
                            record.consecutive_failures += 1;
                            if record.consecutive_failures >= 3 {
                                record.status = UnifiedHealthStatus::Unhealthy;
                            } else {
                                record.status = UnifiedHealthStatus::Degraded;
                            }
                        } else {
                            record.consecutive_failures = 0;
                            record.status = UnifiedHealthStatus::Healthy;
                        }
                    }
                }
            }
        );

        Ok(())
    }
}

impl Default for SmartServiceDiscovery {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

// ==================== SECTION ====================

/// Create a smart service factory with default configuration
pub fn create_service_factory() -> SmartServiceFactory {
    SmartServiceFactory::new()
}
/// Create a mock service with default behavior
/// **TEST ONLY**: This function should only be used in test code
#[cfg(any(test, feature = "dev-stubs"))]
pub async fn create_mock_service(
    service_type: UnifiedServiceType,
) -> Result<Box<dyn SmartService>> {
    let factory = SmartServiceFactory::new();
    factory
        .create_mock_service(service_type, MockServiceBehavior::default())
        .await
}
/// Create a service discovery system
pub fn create_service_discovery() -> SmartServiceDiscovery {
    SmartServiceDiscovery::new()
}
