//! Common Test Infrastructure for Songbird Orchestrator
//! 
//! Provides shared utilities, mock services, and test helpers

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

use songbird_orchestrator::{
    Orchestrator, OrchestratorConfig, ServiceInfo, ServiceRequest, ServiceResponse,
    ServiceMetrics, UniversalService, SongbirdError, Result, utils,
};
use tempfile::TempDir;

/// Mock service for testing
pub struct MockService {
    pub id: String,
    pub config: Option<MockConfig>,
    pub started: Arc<RwLock<bool>>,
    pub request_count: Arc<RwLock<u64>>,
    pub error_rate: Arc<RwLock<f64>>,
    pub health_status: Arc<RwLock<MockHealth>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockConfig {
    pub name: String,
    pub port: u16,
    pub enable_errors: bool,
    pub response_delay_ms: u64,
}

#[derive(Debug, Serialize)]
pub struct MockHealth {
    pub status: String,
    pub requests_handled: u64,
    pub uptime_seconds: u64,
    pub memory_usage_bytes: u64,
}

impl Default for MockConfig {
    fn default() -> Self {
        Self {
            name: "mock-service".to_string(),
            port: 8080,
            enable_errors: false,
            response_delay_ms: 10,
        }
    }
}

impl Default for MockHealth {
    fn default() -> Self {
        Self {
            status: "healthy".to_string(),
            requests_handled: 0,
            uptime_seconds: 0,
            memory_usage_bytes: 1024 * 1024, // 1MB
        }
    }
}

impl MockService {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            config: None,
            started: Arc::new(RwLock::new(false)),
            request_count: Arc::new(RwLock::new(0)),
            error_rate: Arc::new(RwLock::new(0.0)),
            health_status: Arc::new(RwLock::new(MockHealth::default())),
        }
    }
    
    pub async fn set_error_rate(&self, rate: f64) {
        *self.error_rate.write().await = rate;
    }
    
    pub async fn get_request_count(&self) -> u64 {
        *self.request_count.read().await
    }
    
    pub async fn is_started(&self) -> bool {
        *self.started.read().await
    }
}

#[async_trait]
impl UniversalService for MockService {
    type Config = MockConfig;
    type Health = MockHealth;
    type Error = Box<dyn std::error::Error + Send + Sync>;
    
    async fn initialize(&mut self, config: Self::Config) -> Result<(), Self::Error> {
        self.config = Some(config);
        Ok(())
    }
    
    async fn start(&mut self) -> Result<(), Self::Error> {
        *self.started.write().await = true;
        Ok(())
    }
    
    async fn stop(&mut self) -> Result<(), Self::Error> {
        *self.started.write().await = false;
        Ok(())
    }
    
    async fn health_check(&self) -> Result<Self::Health, Self::Error> {
        let mut health = self.health_status.write().await;
        health.requests_handled = *self.request_count.read().await;
        Ok(health.clone())
    }
    
    async fn handle_request(&self, request: ServiceRequest) -> Result<ServiceResponse, Self::Error> {
        *self.request_count.write().await += 1;
        
        // Simulate response delay
        if let Some(config) = &self.config {
            if config.response_delay_ms > 0 {
                tokio::time::sleep(Duration::from_millis(config.response_delay_ms)).await;
            }
        }
        
        // Simulate errors based on error rate
        let error_rate = *self.error_rate.read().await;
        if error_rate > 0.0 && rand::random::<f64>() < error_rate {
            return Ok(ServiceResponse::error(request.id, 500, "Simulated error"));
        }
        
        Ok(ServiceResponse::success(
            request.id,
            serde_json::json!({
                "service_id": self.id,
                "method": request.method,
                "path": request.path,
                "timestamp": chrono::Utc::now(),
                "request_count": *self.request_count.read().await
            })
        ))
    }
    
    async fn update_config(&mut self, config: Self::Config) -> Result<(), Self::Error> {
        self.config = Some(config);
        Ok(())
    }
    
    async fn get_metrics(&self) -> Result<ServiceMetrics, Self::Error> {
        Ok(ServiceMetrics {
            request_count: *self.request_count.read().await,
            error_count: 0,
            avg_response_time_ms: 10.0,
            p95_response_time_ms: 15.0,
            p99_response_time_ms: 20.0,
            cpu_usage: 0.1,
            memory_usage: 1024 * 1024,
            active_connections: 5,
            queue_depth: 0,
            throughput_rps: 100.0,
            error_rate: *self.error_rate.read().await,
            uptime_seconds: 3600,
            last_updated: chrono::Utc::now(),
            custom_metrics: HashMap::new(),
        })
    }
    
    fn service_info(&self) -> ServiceInfo {
        ServiceInfo {
            id: self.id.clone(),
            name: format!("Mock Service {}", self.id),
            version: "1.0.0".to_string(),
            service_type: "mock".to_string(),
            description: "Mock service for testing".to_string(),
            endpoints: vec![],
            capabilities: vec!["http".to_string(), "testing".to_string()],
            tags: HashMap::new(),
            metadata: HashMap::new(),
        }
    }
    
    async fn can_handle_load(&self) -> Result<bool, Self::Error> {
        Ok(*self.started.read().await)
    }
    
    async fn get_load_factor(&self) -> Result<f64, Self::Error> {
        let request_count = *self.request_count.read().await;
        // Simulate increasing load factor based on request count
        Ok((request_count as f64 / 1000.0).min(1.0))
    }
}

/// Test configuration builder
pub struct TestConfigBuilder {
    config: OrchestratorConfig<MockConfig>,
}

impl TestConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: OrchestratorConfig::default(),
        }
    }
    
    pub fn with_port(mut self, port: u16) -> Self {
        self.config.orchestrator.port = port;
        self
    }
    
    pub fn with_max_services(mut self, max_services: usize) -> Self {
        self.config.orchestrator.max_services = max_services;
        self
    }
    
    pub fn with_health_check_interval(mut self, interval: Duration) -> Self {
        self.config.orchestrator.health_check_interval = interval;
        self
    }
    
    pub fn build(self) -> OrchestratorConfig<MockConfig> {
        self.config
    }
}

impl Default for TestConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Test orchestrator setup
pub async fn setup_test_orchestrator() -> Result<Orchestrator, SongbirdError> {
    let config = TestConfigBuilder::new()
        .with_port(0) // Use random available port
        .with_health_check_interval(Duration::from_millis(100))
        .build();
    
    Orchestrator::new(config).await
}

/// Create multiple mock services for testing
pub fn create_mock_services(count: usize) -> Vec<MockService> {
    (0..count)
        .map(|i| MockService::new(format!("mock-service-{}", i)))
        .collect()
}

/// Wait for condition with timeout
pub async fn wait_for_condition<F, Fut>(
    condition: F,
    timeout: Duration,
    check_interval: Duration,
) -> Result<(), &'static str>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = bool>,
{
    let start = std::time::Instant::now();
    
    while start.elapsed() < timeout {
        if condition().await {
            return Ok(());
        }
        tokio::time::sleep(check_interval).await;
    }
    
    Err("Condition timeout")
}

/// Trait for load testing
#[async_trait]
pub trait LoadTestable {
    async fn simulate_load(&self, requests_per_second: u64, duration: Duration) -> LoadTestResults;
}

#[derive(Debug)]
pub struct LoadTestResults {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_latency_ms: f64,
    pub p95_latency_ms: f64,
    pub p99_latency_ms: f64,
    pub requests_per_second: f64,
    pub error_rate: f64,
}

/// Create an error response
pub fn create_error_response(request_id: String, error: String) -> ServiceResponse {
    ServiceResponse::error(request_id, 500, error)
}

/// Test orchestrator fixture for comprehensive testing
pub struct TestOrchestrator {
    orchestrator: Orchestrator,
    _temp_dir: Option<TempDir>,
}

impl TestOrchestrator {
    /// Create a new test orchestrator with default configuration
    pub async fn new() -> Result<Self> {
        let config = OrchestratorConfig::default();
        let orchestrator = Orchestrator::new(config).await?;
        
        Ok(Self {
            orchestrator,
            _temp_dir: None,
        })
    }
    
    /// Create a test orchestrator with custom configuration
    pub async fn with_config(config: OrchestratorConfig) -> Result<Self> {
        let orchestrator = Orchestrator::new(config).await?;
        
        Ok(Self {
            orchestrator,
            _temp_dir: None,
        })
    }
    
    /// Get a reference to the orchestrator
    pub fn orchestrator(&self) -> &Orchestrator {
        &self.orchestrator
    }
    
    /// Create a test service info
    pub fn create_test_service(&self, id: &str, service_type: &str) -> ServiceInfo {
        utils::create_test_service_info(id, id, service_type)
    }
    
    /// Create a test request
    pub fn create_test_request(&self, method: &str, path: &str) -> ServiceRequest {
        utils::create_test_request(method, path)
    }
    
    /// Cleanup resources
    pub async fn cleanup(&self) -> Result<()> {
        // Perform any necessary cleanup
        Ok(())
    }
} 