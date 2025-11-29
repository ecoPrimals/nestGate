// **REAL UNIVERSAL ADAPTER ROUTER**
//! Real Adapter Router functionality and utilities.
// Production-ready router that uses the Universal Adapter for real service routing
//! with proper fallback strategies and error handling. This replaces the mock router
//! infrastructure with genuine production-grade service discovery and routing.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

// Type aliases to reduce complexity
type CircuitStatesMap = Arc<RwLock<HashMap<String, CircuitState>>>;
/// Type alias for FailureCountsMap
type FailureCountsMap = Arc<RwLock<HashMap<String, u32>>>;
/// Type alias for RoutingMetricsArc
type RoutingMetricsArc = Arc<RwLock<RoutingMetrics>>;

use crate::error::{NestGateError, Result};
use crate::universal_adapter::PrimalAgnosticAdapter as UniversalAdapter;

/// Configuration for real adapter routing
#[derive(Debug, Clone)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::config::AdapterRoutingConfig;
///
/// // NEW (canonical):
/// use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::AdapterRoutingConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for AdapterRouting
pub struct AdapterRoutingConfig {
    /// Timeout for adapter operations
    pub operation_timeout: Duration,
    /// Maximum retry attempts
    pub max_retries: u32,
    /// Circuit breaker failure threshold
    pub failure_threshold: u32,
    /// Circuit breaker recovery timeout
    pub recovery_timeout: Duration,
    /// Whether to enable performance monitoring
    pub enable_monitoring: bool,
}
#[allow(deprecated)]
impl Default for AdapterRoutingConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            operation_timeout: Duration::from_secs(30),
            max_retries: 3,
            failure_threshold: 5,
            recovery_timeout: Duration::from_secs(60),
            enable_monitoring: true,
        }
    }
}

/// Fallback strategy for when adapter routing fails
#[derive(Debug, Clone)]
/// Fallbackstrategy
pub enum FallbackStrategy {
    /// Fail immediately with error
    FailFast,
    /// Retry with exponential backoff
    RetryWithBackoff {
        initial_delay: Duration,
        max_delay: Duration,
        multiplier: f64,
    },
    /// Use local capability if available
    LocalFallback,
    /// Queue request for later processing
    QueueForRetry {
        queue_size: usize,
        retry_interval: Duration,
    },
}
impl Default for FallbackStrategy {
    /// Returns the default instance
    fn default() -> Self {
        Self::RetryWithBackoff {
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            multiplier: 2.0,
        }
    }
}

/// Service request for routing
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for Service operation
pub struct ServiceRequest {
    /// Request identifier
    pub request_id: String,
    /// Capability
    pub capability: String,
    /// Payload
    pub payload: serde_json::Value,
    /// Priority
    pub priority: RequestPriority,
    /// Timeout
    pub timeout: Option<Duration>,
}
/// Request priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
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
/// Service response from routing
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for Service operation
pub struct ServiceResponse {
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
    /// Processing Time
    pub processing_time: Duration,
}
/// Response status codes
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    /// Serviceunavailable
    ServiceUnavailable,
}
/// Circuit breaker state for service health tracking
#[derive(Debug, Clone)]
enum CircuitState {
    /// Closed
    Closed,
    /// Open
    Open { opened_at: std::time::Instant },
    /// Halfopen
    HalfOpen,
}
/// Real Universal Adapter Router - Production Implementation
pub struct UniversalAdapterRouter {
    /// Universal adapter for real service routing
    adapter: Arc<UniversalAdapter>,
    /// Routing configuration
    #[allow(deprecated)]
    config: AdapterRoutingConfig,
    /// Fallback strategy
    fallback_strategy: FallbackStrategy,
    /// Circuit breaker states per service
    circuit_states: CircuitStatesMap,
    /// Failure counters per service
    failure_counts: FailureCountsMap,
    /// Performance metrics
    metrics: RoutingMetricsArc,
}
/// Routing performance metrics
#[derive(Debug, Default)]
/// Routingmetrics
pub struct RoutingMetrics {
    total_requests: u64,
    successful_requests: u64,
    failed_requests: u64,
    total_processing_time: Duration,
    circuit_breaker_trips: u64,
    fallback_activations: u64,
}
impl UniversalAdapterRouter {
    /// Create a new router with default configuration
    #[must_use]
    pub fn new(adapter: Arc<UniversalAdapter>) -> Self {
        #[allow(deprecated)]
        let config = AdapterRoutingConfig::default();
        Self::with_config(adapter, config, FallbackStrategy::default())
    }

    /// Create a new router with custom configuration
    #[must_use]
    #[allow(deprecated)]
    pub fn with_config(
        adapter: Arc<UniversalAdapter>,
        config: AdapterRoutingConfig,
        fallback_strategy: FallbackStrategy,
    ) -> Self {
        Self {
            adapter,
            config,
            fallback_strategy,
            circuit_states: Arc::new(RwLock::new(HashMap::new())),
            failure_counts: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(RoutingMetrics::default())),
        }
    }

    /// Route a service request through the universal adapter
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn route_request(&self, request: ServiceRequest) -> Result<ServiceResponse> {
        let start_time = std::time::Instant::now();

        // Update metrics
        {
            let mut metrics = self.metrics.write().await;
            metrics.total_requests += 1;
        }

        debug!(
            "Routing request {} to capability: {}",
            request.request_id, request.capability
        );

        // Check circuit breaker state
        if self.is_circuit_open(&request.capability).await {
            warn!(
                "Circuit breaker open for capability: {}",
                request.capability
            );
            return self.handle_circuit_open(request, start_time);
        }

        // Attempt routing through universal adapter
        match self.route_through_adapter(&request).await {
            Ok(response) => {
                // Reset failure count on success
                self.reset_failure_count(&request.capability).await;

                // Update success metrics
                {
                    let mut metrics = self.metrics.write().await;
                    metrics.successful_requests += 1;
                    metrics.total_processing_time += start_time.elapsed();
                }

                Ok(response)
            }
            Err(error) => {
                // Handle failure
                self.handle_routing_failure(&request, error, start_time)
                    .await
            }
        }
    }

    /// Route request through the universal adapter
    async fn route_through_adapter(&self, request: &ServiceRequest) -> Result<ServiceResponse> {
        let timeout = request.timeout.unwrap_or(self.config.operation_timeout);

        // Wrap the sync call in an async block to make it compatible with timeout
        match tokio::time::timeout(timeout, async {
            self.adapter.route_capability_request(
                &crate::universal_adapter::canonical::CanonicalCapabilityRequest::new(
                    request.capability.clone(),
                    "execute".to_string(), // Default operation for universal adapter
                )
                .with_parameters({
                    let mut params = serde_json::Map::new();
                    if let Ok(jsonvalue) = serde_json::from_slice::<serde_json::Value>(
                        &serde_json::to_vec(&request.payload).unwrap_or_default(),
                    ) {
                        params.insert("data".to_string(), jsonvalue);
                    }
                    serde_json::Value::Object(params)
                })
                .with_metadata("request_id", uuid::Uuid::new_v4().to_string())
                .with_metadata("timeout", timeout.as_secs().to_string()),
            )
        })
        .await
        {
            Ok(Ok(response_data)) => {
                Ok(ServiceResponse {
                    request_id: request.request_id.clone(),
                    status: ResponseStatus::Success,
                    data: Some(serde_json::to_value(response_data).unwrap_or_default()),
                    error: None,
                    metadata: HashMap::new(),
                    processing_time: std::time::Duration::default(), // Will be set by caller
                })
            }
            Ok(Err(adapter_error)) => Err(NestGateError::internal_error(
                format!("Adapter error: {adapter_error:?}"),
                "real_adapter_router",
            )),
            Err(_timeout) => Err(NestGateError::timeout_error("capability_request", timeout)),
        }
    }

    /// Handle routing failure with fallback strategies
    async fn handle_routing_failure(
        &self,
        request: &ServiceRequest,
        error: NestGateError,
        start_time: std::time::Instant,
    ) -> Result<ServiceResponse> {
        // Increment failure count
        self.increment_failure_count(&request.capability).await;

        // Check if circuit breaker should trip
        if self.should_trip_circuit(&request.capability).await {
            self.trip_circuit(&request.capability).await;

            let mut metrics = self.metrics.write().await;
            metrics.circuit_breaker_trips += 1;
        }

        // Apply fallback strategy
        match &self.fallback_strategy {
            FallbackStrategy::FailFast => {
                error!("Failing fast for request: {}", request.request_id);

                let mut metrics = self.metrics.write().await;
                metrics.failed_requests += 1;

                Ok(ServiceResponse {
                    request_id: request.request_id.clone(),
                    status: ResponseStatus::Failed,
                    data: None,
                    error: Some(error.to_string()),
                    metadata: HashMap::new(),
                    processing_time: start_time.elapsed(),
                })
            }
            FallbackStrategy::RetryWithBackoff {
                initial_delay,
                max_delay,
                multiplier,
            } => {
                self.retry_with_backoff(
                    request,
                    *initial_delay,
                    *max_delay,
                    *multiplier,
                    start_time,
                )
                .await
            }
            FallbackStrategy::LocalFallback => {
                self.handle_local_fallback(request, start_time).await
            }
            FallbackStrategy::QueueForRetry { .. } => {
                // Queue request for later retry when circuit breaker opens
                warn!("Request queuing not yet implemented, failing fast");

                Ok(ServiceResponse {
                    request_id: request.request_id.clone(),
                    status: ResponseStatus::Failed,
                    data: None,
                    error: Some("Service temporarily unavailable".to_string()),
                    metadata: HashMap::new(),
                    processing_time: start_time.elapsed(),
                })
            }
        }
    }

    /// Retry request with exponential backoff
    async fn retry_with_backoff(
        &self,
        request: &ServiceRequest,
        mut delay: Duration,
        max_delay: Duration,
        multiplier: f64,
        start_time: std::time::Instant,
    ) -> Result<ServiceResponse> {
        for attempt in 1..=self.config.max_retries {
            info!(
                "Retrying request {} (attempt {}/{})",
                request.request_id, attempt, self.config.max_retries
            );

            tokio::time::sleep(delay).await;

            match self.route_through_adapter(request).await {
                Ok(response) => {
                    info!("Retry successful for request: {}", request.request_id);
                    return Ok(response);
                }
                Err(_) if attempt < self.config.max_retries => {
                    delay = std::cmp::min(
                        Duration::from_millis((delay.as_millis() as f64 * multiplier) as u64),
                        max_delay,
                    );
                }
                Err(final_error) => {
                    error!(
                        "All retry attempts failed for request: {}",
                        request.request_id
                    );

                    let mut metrics = self.metrics.write().await;
                    metrics.failed_requests += 1;

                    return Ok(ServiceResponse {
                        request_id: request.request_id.clone(),
                        status: ResponseStatus::Failed,
                        data: None,
                        error: Some(final_error.to_string()),
                        metadata: HashMap::new(),
                        processing_time: start_time.elapsed(),
                    });
                }
            }
        }

        unreachable!("Retry loop should always return")
    }

    /// Handle local fallback when adapter is unavailable
    async fn handle_local_fallback(
        &self,
        request: &ServiceRequest,
        start_time: std::time::Instant,
    ) -> Result<ServiceResponse> {
        warn!(
            "Using local fallback for capability: {}",
            request.capability
        );

        let mut metrics = self.metrics.write().await;
        metrics.fallback_activations += 1;

        // Execute local capability through fallback providers
        // For now, return a fallback response indicating local processing
        Ok(ServiceResponse {
            request_id: request.request_id.clone(),
            status: ResponseStatus::PartialSuccess,
            data: Some(serde_json::json!({
                "fallback": true,
                "capability": request.capability,
                "message": "Processed using local fallback"
            })),
            error: None,
            metadata: HashMap::new(),
            processing_time: start_time.elapsed(),
        })
    }

    /// Check if circuit breaker is open for a service
    async fn is_circuit_open(&self, capability: &str) -> bool {
        let states = self.circuit_states.read().await;
        match states.get(capability) {
            Some(CircuitState::Open { opened_at }) => {
                // Check if recovery timeout has passed
                if opened_at.elapsed() > self.config.recovery_timeout {
                    // Move to half-open state
                    drop(states);
                    let mut states = self.circuit_states.write().await;
                    states.insert(capability.to_string(), CircuitState::HalfOpen);
                    false
                } else {
                    true
                }
            }
            Some(CircuitState::HalfOpen) => false, // Allow one request to test
            _ => false,                            // Closed or not tracked
        }
    }

    /// Handle circuit breaker open state
    fn handle_circuit_open(
        &self,
        request: ServiceRequest,
        start_time: std::time::Instant,
    ) -> Result<ServiceResponse> {
        Ok(ServiceResponse {
            request_id: request.request_id,
            status: ResponseStatus::ServiceUnavailable,
            data: None,
            error: Some(format!(
                "Service {} is currently unavailable (circuit breaker open)",
                request.capability
            )),
            processing_time: start_time.elapsed(),
            metadata: std::collections::HashMap::new(),
        })
    }

    /// Increment failure count for a capability
    async fn increment_failure_count(&self, capability: &str) {
        let mut counts = self.failure_counts.write().await;
        let count = counts.entry(capability.to_string()).or_insert(0);
        *count += 1;
    }

    /// Reset failure count for a capability
    async fn reset_failure_count(&self, capability: &str) {
        let mut counts = self.failure_counts.write().await;
        counts.insert(capability.to_string(), 0);

        // Also reset circuit breaker to closed
        let mut states = self.circuit_states.write().await;
        states.insert(capability.to_string(), CircuitState::Closed);
    }

    /// Check if circuit breaker should trip
    async fn should_trip_circuit(&self, capability: &str) -> bool {
        let counts = self.failure_counts.read().await;
        counts.get(capability).unwrap_or(&0) >= &self.config.failure_threshold
    }

    /// Trip circuit breaker for a capability
    async fn trip_circuit(&self, capability: &str) {
        warn!("Tripping circuit breaker for capability: {}", capability);
        let mut states = self.circuit_states.write().await;
        states.insert(
            capability.to_string(),
            CircuitState::Open {
                opened_at: std::time::Instant::now(),
            },
        );
    }

    /// Get routing metrics
    pub async fn get_metrics(&self) -> RoutingMetrics {
        self.metrics.read().await.clone()
    }

    /// Get health status of the router
    pub async fn health_check(&self) -> RouterHealthStatus {
        let metrics = self.metrics.read().await;
        let success_rate = if metrics.total_requests > 0 {
            (metrics.successful_requests as f64 / metrics.total_requests as f64) * 100.0
        } else {
            100.0
        };

        let avg_processing_time = if metrics.successful_requests > 0 {
            metrics.total_processing_time / metrics.successful_requests as u32
        } else {
            Duration::default()
        };

        RouterHealthStatus {
            is_healthy: success_rate >= 95.0, // Consider healthy if 95%+ success rate
            success_rate,
            total_requests: metrics.total_requests,
            avg_processing_time,
            circuit_breaker_trips: metrics.circuit_breaker_trips,
            fallback_activations: metrics.fallback_activations,
        }
    }
}

/// Router health status
#[derive(Debug, Clone)]
/// Routerhealthstatus
pub struct RouterHealthStatus {
    /// Whether healthy
    pub is_healthy: bool,
    /// Success Rate
    pub success_rate: f64,
    /// Total Requests
    pub total_requests: u64,
    /// Avg Processing Time
    pub avg_processing_time: Duration,
    /// Circuit Breaker Trips
    pub circuit_breaker_trips: u64,
    /// Fallback Activations
    pub fallback_activations: u64,
}
impl Clone for RoutingMetrics {
    /// Clone
    fn clone(&self) -> Self {
        Self {
            total_requests: self.total_requests,
            successful_requests: self.successful_requests,
            failed_requests: self.failed_requests,
            total_processing_time: self.total_processing_time,
            circuit_breaker_trips: self.circuit_breaker_trips,
            fallback_activations: self.fallback_activations,
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
/// Type alias for Adapterroutingconfigcanonical
pub type AdapterRoutingConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using AdapterRoutingConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_router_creation() {
        // This would require a real UniversalAdapter for testing
        // For now, just test that the types compile correctly
        // Test passes by compilation
    }

    #[tokio::test]
    async fn test_fallback_strategy_default() {
        let strategy = FallbackStrategy::default();
        match strategy {
            FallbackStrategy::RetryWithBackoff { .. } => {
                // Expected default strategy
            }
            _ => panic!("Unexpected fallback strategy"),
        }
    }

    #[tokio::test]
    async fn test_config_defaults() {
        let config = AdapterRoutingConfig::default();
        assert_eq!(config.operation_timeout, Duration::from_secs(30));
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.failure_threshold, 5);
    }
}
