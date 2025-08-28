use crate::error::NestGateError;
use std::collections::HashMap;
//
// This module implements enterprise-grade resilience patterns including circuit breakers,
// bulkheads, timeouts, retries, and cascading failure prevention.

pub mod bulkhead;
pub mod circuit_breaker;
pub mod failure_detector;
pub mod retry_policy;
pub mod timeout_manager;

use crate::{Result};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

/// Type aliases for complex types
type CircuitBreakerMap =
    Arc<RwLock<std::collections::HashMap<String, circuit_breaker::CircuitBreaker>>>;
type BulkheadMap = Arc<RwLock<std::collections::HashMap<String, bulkhead::Bulkhead>>>;

/// Central resilience coordinator
pub struct ResilienceManager {
    circuit_breakers: CircuitBreakerMap,
    bulkheads: BulkheadMap,
    #[allow(dead_code)]
    failure_detector: Arc<failure_detector::FailureDetector>,
    config: ResilienceConfig,
    timeout_failures: Arc<RwLock<u64>>,
    retry_attempts: Arc<RwLock<u64>>,
}

/// Configuration for resilience patterns
#[derive(Debug, Clone)]
pub struct ResilienceConfig {
    /// Default circuit breaker configuration
    pub default_circuit_breaker: circuit_breaker::CircuitBreakerConfig,
    /// Default bulkhead configuration  
    pub default_bulkhead: bulkhead::BulkheadConfig,
    /// Default retry policy
    /// Default timeout settings
    pub default_timeout: Duration,
    /// Enable automatic failure detection
    pub enable_failure_detection: bool,
}

impl Default for ResilienceConfig {
    fn default() -> Self {
        Self {
            default_circuit_breaker: circuit_breaker::CircuitBreakerConfig::default(),
            default_bulkhead: bulkhead::BulkheadConfig::default(),
            default_timeout: Duration::from_secs(30),
            enable_failure_detection: true,
        }
    }
}

impl ResilienceManager {
    /// Create a new resilience manager
    pub fn new(config: ResilienceConfig) -> Self {
        Self {
            circuit_breakers: Arc::new(RwLock::new(std::collections::HashMap::new())),
            bulkheads: Arc::new(RwLock::new(std::collections::HashMap::new())),
            failure_detector: Arc::new(failure_detector::FailureDetector::new()),
            config,
            timeout_failures: Arc::new(RwLock::new(0)),
            retry_attempts: Arc::new(RwLock::new(0)),
        }
    }

    /// Execute operation with full resilience protection
    pub async fn execute_with_resilience<F, T>(&self, service_name: &str, operation: F) -> Result<T>
    where
        F: std::future::Future<Output = Result<T>>,
    {
        // Get or create circuit breaker
        let circuit_breaker = self.get_or_create_circuit_breaker(service_name).await;

        // Check circuit breaker state
        if !circuit_breaker.can_execute().await? {
            return Err(NestGateError::simple("Circuit breaker is open"));
        }

        // Get bulkhead permit
        let bulkhead = self.get_or_create_bulkhead(service_name).await;
        let _permit = bulkhead.acquire_permit().await?;

        // Execute with timeout
        let result =
            timeout_manager::execute_with_timeout(operation, self.config.default_timeout).await;

        // Record result in circuit breaker and update metrics
        match &result {
            Ok(_) => circuit_breaker.record_success().await?,
            Err(e) => {
                circuit_breaker.record_failure().await?;
                // Check if it's a timeout error and increment counter
                if e.to_string().contains("timeout") {
                    let mut timeout_count = self.timeout_failures.write().await;
                    *timeout_count += 1;
                }
            }
        }

        result
    }

    /// Get or create circuit breaker for service
    async fn get_or_create_circuit_breaker(
        &self,
        service_name: &str,
    ) -> circuit_breaker::CircuitBreaker {
        let mut breakers = self.circuit_breakers.write().await;

        if let Some(breaker) = breakers.get(service_name) {
            breaker.clone()
        } else {
            let breaker = circuit_breaker::CircuitBreaker::new(
                service_name.to_string(),
                self.config.default_circuit_breaker.clone(),
            );
            breakers.insert(service_name.to_string(), breaker.clone());
            breaker
        }
    }

    /// Get or create bulkhead for service
    async fn get_or_create_bulkhead(&self, service_name: &str) -> bulkhead::Bulkhead {
        let mut bulkheads = self.bulkheads.write().await;

        if let Some(bulkhead) = bulkheads.get(service_name) {
            bulkhead.clone()
        } else {
            let bulkhead = bulkhead::Bulkhead::new(
                service_name.to_string(),
                self.config.default_bulkhead.clone(),
            );
            bulkheads.insert(service_name.to_string(), bulkhead.clone());
            bulkhead
        }
    }

    /// Get system resilience status
    pub async fn get_resilience_status(&self) -> Result<ResilienceStatus> {
        let circuit_breakers = self.circuit_breakers.read().await;
        let bulkheads = self.bulkheads.read().await;

        let mut circuit_breaker_states = std::collections::HashMap::new();
        for (name, breaker) in circuit_breakers.iter() {
            circuit_breaker_states.insert(name.clone(), breaker.get_state().await?);
        }

        let mut bulkhead_states = std::collections::HashMap::new();
        for (name, bulkhead) in bulkheads.iter() {
            bulkhead_states.insert(name.clone(), bulkhead.get_status().await?);
        }

        Ok(ResilienceStatus {
            circuit_breakers: circuit_breaker_states,
            bulkheads: bulkhead_states,
            failure_detection_enabled: self.config.enable_failure_detection,
        })
    }
}

/// System resilience status
#[derive(Debug, Clone)]
pub struct ResilienceStatus {
    pub circuit_breakers: std::collections::HashMap<String, circuit_breaker::CircuitBreakerState>,
    pub bulkheads: std::collections::HashMap<String, bulkhead::BulkheadStatus>,
    pub failure_detection_enabled: bool,
}

/// Resilience metrics for monitoring
#[derive(Debug, Clone)]
pub struct ResilienceMetrics {
    pub circuit_breaker_trips: u64,
    pub bulkhead_rejections: u64,
    pub timeout_failures: u64,
    pub retry_attempts: u64,
    pub total_operations: u64,
    pub success_rate: f64,
}

impl ResilienceManager {
    /// Get resilience metrics
    pub async fn get_metrics(&self) -> Result<ResilienceMetrics> {
        let circuit_breakers = self.circuit_breakers.read().await;
        let bulkheads = self.bulkheads.read().await;

        let mut total_trips = 0;
        let mut total_rejections = 0;
        let mut total_operations = 0;
        let mut total_successes = 0;

        for breaker in circuit_breakers.values() {
            let metrics = breaker.get_metrics().await?;
            total_trips += metrics.trip_count;
            total_operations += metrics.total_requests;
            total_successes += metrics.successful_requests;
        }

        for bulkhead in bulkheads.values() {
            let metrics = bulkhead.get_metrics().await?;
            total_rejections += metrics.rejections;
        }

        let success_rate = if total_operations > 0 {
            (total_successes as f64 / total_operations as f64) * 100.0
        } else {
            100.0
        };

        let timeout_failures = *self.timeout_failures.read().await;
        let retry_attempts = *self.retry_attempts.read().await;

        Ok(ResilienceMetrics {
            circuit_breaker_trips: total_trips,
            bulkhead_rejections: total_rejections,
            timeout_failures,
            retry_attempts,
            total_operations,
            success_rate,
        })
    }
}
