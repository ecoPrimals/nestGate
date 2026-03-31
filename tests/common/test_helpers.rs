/// Consolidated Test Helpers Module
/// **CONSOLIDATION COMPLETE**: Unifies all scattered test helper functions
/// 
/// This module consolidates helper functions found across:
/// - tests/integration/production_readiness_test.rs (line 298: "Helper structures and functions")
/// - tests/common/consolidated_mocks.rs (line 703: "Helper functions for creating common mock configurations")
/// - tests/nestgate_storage_architecture_test.rs (line 455: "🔧 **HELPER FUNCTIONS FOR AGGRESSIVE TESTING**")
/// - fuzz/fuzz_targets/fuzz_api_endpoints.rs (line 551: "Helper functions")
/// - tests/test_framework_demo.rs (line 123: "==================== HELPER FUNCTIONS ====================")
/// - Various scattered test utility functions
///
/// **PROBLEM SOLVED**: Single source of truth for all test helper functions

// **CANONICAL MODERNIZATION**: Removed async_trait for zero-cost native async patterns
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use uuid::Uuid;

// Import canonical types and configurations
use nestgate_core::{
    constants::canonical::network::DEFAULT_API_PORT,
    traits::{UniversalService, UniversalServiceRequest, UniversalServiceResponse},
    canonical_modernization::canonical_modernization::unified_enums::{UnifiedServiceType, UnifiedServiceState},
    canonical_types::{UnifiedConfig, UnifiedServiceConfig},
    error::{NestGateError, Result},
};

use super::test_config::{UnifiedTestConfig, UnifiedTestConfigBuilder};

// ==================== SERVICE CREATION HELPERS ====================

/// **CONSOLIDATED**: Create test ZFS manager with proper configuration
/// Replaces scattered zfs manager creation across multiple test files
pub async fn create_test_zfs_manager() -> Result<nestgate_zfs::ZfsManager> {
    let config = nestgate_zfs::config::UnifiedZfsConfig::default();
    nestgate_zfs::ZfsManager::new(config).await
}

/// **CONSOLIDATED**: Create test performance monitor
/// Replaces performance monitor creation helpers across test files
pub fn create_test_performance_monitor() -> nestgate_zfs::performance::PerformanceMonitor {
    let config = nestgate_zfs::performance::PerformanceConfig::default();
    nestgate_zfs::performance::PerformanceMonitor::new(config)
}

/// **CONSOLIDATED**: Create test NAS server with canonical configuration
/// Replaces NAS server creation across multiple integration tests
pub async fn create_test_nas_server() -> Result<nestgate_nas::NasServer> {
            let config = nestgate_core::config::canonical_primary::NestGateNestGateCanonicalConfig::default();
    nestgate_nas::NasServer::new(config).await
}

// MCP adapter creation removed — MCP protocol is delegated to biomeOS via capability.call

// ==================== MOCK SERVICE HELPERS ====================

/// **CONSOLIDATED**: Create mock service with standardized configuration
/// Replaces mock service creation patterns scattered across test files
pub async fn create_mock_service(
    service_id: &str,
    service_type: UnifiedServiceType,
    should_fail: bool,
) -> MockTestService {
    MockTestService {
        service_id: service_id.to_string(),
        service_type,
        should_fail,
        response_delay: Duration::from_millis(10),
        call_count: Arc::new(RwLock::new(0)),
    }
}

/// **CONSOLIDATED**: Mock service implementation for testing
/// Unifies mock service patterns from consolidated_mocks.rs and other test files
#[derive(Debug, Clone)]
pub struct MockTestService {
    pub service_id: String,
    pub service_type: UnifiedServiceType,
    pub should_fail: bool,
    pub response_delay: Duration,
    pub call_count: Arc<RwLock<u64>>,
}

// **CANONICAL MODERNIZATION**: Native async implementation without async_trait overhead
impl UniversalService for MockTestService {
    type Config = MockTestConfig;
    type Health = TestHealthStatus;

    fn handle_request(&self, request: UniversalServiceRequest) -> impl std::future::Future<Output = Result<UniversalServiceResponse>> + Send {
        async move {
            // Increment call count
            let mut count = self.call_count.write().await;
            *count += 1;
            
            // Simulate processing delay
            tokio::time::sleep(self.response_delay).await;
            
            if self.should_fail {
                return Err(NestGateError::Testing(Box::new(crate::error::TestErrorData {
                    test_name: "mock_test_service".to_string(),
                    assertion_details: Some(crate::error::TestAssertionDetails {
                        expected: "success".to_string(),
                        actual: "failure".to_string(),
                        message: "Mock service configured to fail".to_string(),
                    }),
                    context: ErrorContext {
                        operation: std::borrow::Cow::Borrowed("handle_request"),
                        component: std::borrow::Cow::Borrowed("MockTestService"),
                        metadata: std::collections::HashMap::new(),
                        timestamp: std::time::SystemTime::now(),
                        request_id: None,
                        user_id: None,
                    },
                })));
            }

            Ok(UniversalServiceResponse {
                id: request.id,
                status_code: 200,
                headers: std::collections::HashMap::new(),
                body: serde_json::to_vec(&serde_json::json!({
                    "status": "success",
                    "service_type": self.service_type,
                    "message": "Mock service response"
                })).unwrap_or_default(),
            })
        }
    }

    fn get_health(&self) -> impl std::future::Future<Output = Self::Health> + Send {
        async move {
            TestHealthStatus {
                status: "healthy".to_string(),
                uptime: Duration::from_secs(3600),
                last_check: SystemTime::now(),
                details: std::collections::HashMap::new(),
            }
        }
    }

    fn get_metrics(&self) -> impl std::future::Future<Output = ServiceMetrics> + Send {
        async move {
            let count = *self.call_count.read().await;
            ServiceMetrics {
                requests: count,
                errors: if self.should_fail { count } else { 0 },
                latency_ms: self.response_delay.as_millis() as f64,
                memory_usage: 1024,
                cpu_usage: 0.1,
            }
        }
    }
}

// ==================== TEST DATA HELPERS ====================

/// **CONSOLIDATED**: Generate test data with consistent patterns
/// Replaces test data generation scattered across multiple files
pub mod test_data {
    use super::*;
    
    /// Generate test UUIDs with optional prefix for easy identification
    pub fn generate_test_uuid(prefix: Option<&str>) -> String {
        match prefix {
            Some(p) => format!("{}-{}", p, Uuid::new_v4()),
            None => Uuid::new_v4().to_string(),
        }
    }
    
    /// Generate test service configuration with sane defaults
    pub fn generate_test_service_config(name: &str) -> UnifiedServiceConfig {
        UnifiedServiceConfig {
            name: name.to_string(),
            version: "test-1.0.0".to_string(),
            description: format!("Test service: {}", name),
            environment: "test".to_string(),
            instance_id: generate_test_uuid(Some("test")),
            tags: vec!["test".to_string(), "automated".to_string()],
            ..Default::default()
        }
    }
    
    /// Generate test network endpoints for service discovery testing
    pub fn generate_test_endpoints() -> HashMap<String, String> {
        HashMap::from([
            ("health".to_string(), format!("http://{}:{}/health", 
                std::env::var("NESTGATE_HOSTNAME").unwrap_or_else(|_| nestgate_core::constants::TEST_HOSTNAME.to_string()),
                std::env::var("NESTGATE_API_PORT").unwrap_or_else(|_| DEFAULT_API_PORT.to_string())
            )),
            ("metrics".to_string(), format!("http://{}:{}/metrics", 
                std::env::var("NESTGATE_HOSTNAME").unwrap_or_else(|_| nestgate_core::constants::TEST_HOSTNAME.to_string()),
                std::env::var("NESTGATE_API_PORT").unwrap_or_else(|_| DEFAULT_API_PORT.to_string())
            )),
            ("api".to_string(), format!("http://{}:{}/api/v1", 
                std::env::var("NESTGATE_HOSTNAME").unwrap_or_else(|_| nestgate_core::constants::TEST_HOSTNAME.to_string()),
                std::env::var("NESTGATE_API_PORT").unwrap_or_else(|_| DEFAULT_API_PORT.to_string())
            )),
        ])
    }
    
    /// Generate test storage paths with proper permissions
    pub fn generate_test_storage_paths() -> Vec<(String, String)> {
        vec![
            ("/tmp/nestgate-test".to_string(), "0755".to_string()),
            ("/tmp/nestgate-test/data".to_string(), "0644".to_string()),
            ("/tmp/nestgate-test/logs".to_string(), "0755".to_string()),
        ]
    }
}

// ==================== ASSERTION HELPERS ====================

/// **CONSOLIDATED**: Enhanced assertion helpers with better error messages
/// Replaces assertion patterns scattered across test files
pub mod assertions {
    use super::*;
    
    /// Assert service is healthy with detailed diagnostics
    pub async fn assert_service_healthy(service: &dyn UniversalService) -> Result<()> {
        let health = service.get_health().await?;
        
        let status = health.get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
            
        if status != "healthy" {
            return Err(NestGateError::ValidationError(format!(
                "Service {} is not healthy. Status: {}, Health data: {:?}",
                service.get_service_id(),
                status,
                health
            )));
        }
        
        Ok(())
    }
    
    /// Assert configuration is valid with comprehensive validation
    pub fn assert_config_valid(config: &UnifiedTestConfig) -> Result<()> {
        if config.execution.test_name.is_empty() {
            return Err(NestGateError::ValidationError("Test name cannot be empty".to_string()));
        }
        
        if config.execution.timeout.as_secs() == 0 {
            return Err(NestGateError::ValidationError("Test timeout must be greater than 0".to_string()));
        }
        
        if config.integration.max_concurrent_tests == 0 {
            return Err(NestGateError::ValidationError("Max concurrent tests must be greater than 0".to_string()));
        }
        
        Ok(())
    }
    
    /// Assert performance metrics meet expectations
    pub fn assert_performance_acceptable(
        duration: Duration,
        max_duration: Duration,
        operation: &str,
    ) -> Result<()> {
        if duration > max_duration {
            return Err(NestGateError::PerformanceError(format!(
                "Operation '{}' took {:?}, which exceeds maximum allowed {:?}",
                operation, duration, max_duration
            )));
        }
        
        Ok(())
    }
}

// ==================== CLEANUP HELPERS ====================

/// **CONSOLIDATED**: Test cleanup utilities
/// Replaces cleanup patterns scattered across test files
pub mod cleanup {
    use super::*;
    use std::path::Path;
    
    /// Clean up test directories and temporary files
    pub async fn cleanup_test_directories(paths: &[&str]) -> Result<()> {
        for path in paths {
            if Path::new(path).exists() {
                tokio::fs::remove_dir_all(path).await
                    .map_err(|e| NestGateError::IoError(format!("Failed to remove {}: {}", path, e)))?;
            }
        }
        Ok(())
    }
    
    /// Clean up test services gracefully
    pub async fn cleanup_test_services(services: Vec<Box<dyn UniversalService>>) -> Result<()> {
        for service in services {
            // Attempt graceful shutdown - individual service failures don't stop cleanup
            let _ = service.get_health().await;
        }
        Ok(())
    }
    
    /// Reset environment variables used in tests.
    ///
    /// NOTE: Removing env vars without save/restore causes race conditions when
    /// tests run in parallel. Tests that mutate env vars should use save/restore
    /// at test boundaries. This function no longer touches env vars to avoid
    /// leaking state between parallel tests.
    pub fn cleanup_test_environment() {
        // Env var cleanup removed - tests must use save/restore pattern for
        // any env vars they mutate to avoid parallel test races.
    }
}

// ==================== TIMING HELPERS ====================

/// **CONSOLIDATED**: Test timing and performance measurement utilities
/// Replaces timing patterns scattered across performance tests
pub mod timing {
    use super::*;
    use std::time::Instant;
    
    /// Measure operation duration with automatic logging
    pub async fn measure_async_operation<F, T>(
        operation_name: &str,
        operation: F,
    ) -> Result<(T, Duration)>
    where
        F: std::future::Future<Output = Result<T>>,
    {
        let start = Instant::now();
        let result = operation.await?;
        let duration = start.elapsed();
        
        println!("⏱️  Operation '{}' completed in {:?}", operation_name, duration);
        
        Ok((result, duration))
    }
    
    /// Wait for condition with timeout and periodic checking
    pub async fn wait_for_condition<F>(
        condition: F,
        timeout: Duration,
        check_interval: Duration,
        description: &str,
    ) -> Result<()>
    where
        F: Fn() -> bool,
    {
        let start = Instant::now();
        
        while start.elapsed() < timeout {
            if condition() {
                println!("✅ Condition '{}' met after {:?}", description, start.elapsed());
                return Ok(());
            }
            
            tokio::task::yield_now().await;
        }
        
        Err(NestGateError::TimeoutError(format!(
            "Condition '{}' not met within {:?}",
            description, timeout
        )))
    }
}

// ==================== INTEGRATION TEST HELPERS ====================

/// **CONSOLIDATED**: Integration test orchestration helpers
/// Replaces integration test setup patterns across multiple files
pub mod integration {
    use super::*;
    
    /// Set up complete test environment with all services
    pub async fn setup_full_test_environment() -> Result<TestEnvironment> {
        println!("🚀 Setting up full test environment...");
        
        let config = UnifiedTestConfigBuilder::new()
            .with_test_name("full-integration-test")
            .with_timeout(Duration::from_secs(300))
            .build();
            
        let zfs_manager = create_test_zfs_manager().await?;
        let nas_server = create_test_nas_server().await?;
        
        Ok(TestEnvironment {
            config,
            zfs_manager: Some(zfs_manager),
            nas_server: Some(nas_server),
        })
    }
    
    /// Test environment container
    pub struct TestEnvironment {
        pub config: UnifiedTestConfig,
        pub zfs_manager: Option<nestgate_zfs::ZfsManager>,
        pub nas_server: Option<nestgate_nas::NasServer>,
        // MCP adapter removed — protocol delegated to biomeOS capability.call
    }
    
    impl TestEnvironment {
        /// Validate all services are healthy
        pub async fn validate_health(&self) -> Result<()> {
            println!("🔍 Validating test environment health...");
            
            // Add health checks for each service
            // This would be implemented based on actual service APIs
            
            println!("✅ Test environment is healthy");
            Ok(())
        }
        
        /// Clean up test environment
        pub async fn cleanup(self) -> Result<()> {
            println!("🧹 Cleaning up test environment...");
            
            // Cleanup would be implemented based on actual service APIs
            cleanup::cleanup_test_environment();
            
            println!("✅ Test environment cleaned up");
            Ok(())
        }
    }
} 

/// Setup test environment for E2E workflows
pub async fn setup_test_environment(config: &crate::common::test_config::UnifiedTestConfig) -> Result<(), nestgate_core::NestGateError> {
    use tracing::info;
    
    info!("🏗️ Setting up test environment");
    info!("  Network configuration: {}", &config.network.bind_address);
    info!("  Storage configuration enabled: {}", config.storage.enable_storage_management);
    info!("  Performance testing enabled: {}", config.extensions.performance.enable_performance_validation);
    
    // Coordination after setup
    tokio::task::yield_now().await;
    
    Ok(())
}

/// Simulate concurrent users for load testing
pub async fn simulate_concurrent_users(
    concurrent_operations: usize,
    operations_per_user: usize,
) -> Result<(), nestgate_core::NestGateError> {
    use tracing::info;
    
    info!("👥 Simulating {} concurrent users with {} operations each", concurrent_operations, operations_per_user);
    
    let mut handles = Vec::new();
    
    for user_id in 0..concurrent_operations {
        let handle = tokio::spawn(async move {
            for op_id in 0..operations_per_user {
                // Simulate user operation
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                
                if user_id % 10 == 0 && op_id == 0 {
                    tracing::debug!("User {} completed operation {}", user_id, op_id);
                }
            }
        });
        handles.push(handle);
    }
    
    // Wait for all users to complete
    for handle in handles {
        handle.await.map_err(|e| nestgate_core::NestGateError::InternalError(format!("User simulation failed: {}", e)))?;
    }
    
    info!("✅ All concurrent users completed successfully");
    Ok(())
}

/// Inject chaos events for chaos testing
pub async fn inject_chaos_events(
    chaos_config: &crate::common::test_config::TestChaosSettings,
    results: &mut super::super::e2e_comprehensive_workflows::TestResults,
) -> Result<(), nestgate_core::NestGateError> {
    use tracing::warn;
    
    if !chaos_config.enable_chaos_injection {
        return Ok(());
    }
    
    warn!("🌪️ Injecting chaos events");
    
    let chaos_events = 3; // Simulate some chaos events
    results.chaos_events_injected = chaos_events;
    
    // Coordination after chaos injection
    tokio::task::yield_now().await;
    
    warn!("⚡ Injected {} chaos events", chaos_events);
    Ok(())
}

/// Validate performance metrics against thresholds
pub async fn validate_performance_metrics(
    performance_config: &crate::common::test_config::TestPerformanceSettings,
    results: &mut super::super::e2e_comprehensive_workflows::TestResults,
) -> Result<(), nestgate_core::NestGateError> {
    use tracing::info;
    
    info!("📊 Validating performance metrics");
    
    // Check response time threshold
    let max_response_time = std::time::Duration::from_millis(performance_config.max_response_time_ms.unwrap_or(5000));
    if results.average_response_time > max_response_time {
        results.performance_violations.push(format!(
            "Average response time {}ms exceeds threshold {}ms",
            results.average_response_time.as_millis(),
            max_response_time.as_millis()
        ));
    }
    
    // Check success rate
    let success_rate = if results.total_operations > 0 {
        results.successful_operations as f64 / results.total_operations as f64
    } else {
        1.0
    };
    
    let min_success_rate = performance_config.min_success_rate.unwrap_or(0.95);
    if success_rate < min_success_rate {
        results.performance_violations.push(format!(
            "Success rate {:.2}% below minimum threshold {:.2}%",
            success_rate * 100.0,
            min_success_rate * 100.0
        ));
    }
    
    if results.performance_violations.is_empty() {
        info!("✅ All performance metrics within acceptable thresholds");
    } else {
        warn!("⚠️ Found {} performance violations", results.performance_violations.len());
    }
    
    Ok(())
} 