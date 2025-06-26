---
title: Testing Infrastructure Integration Specification
description: Detailed specification for integrating GitClone testing infrastructure into v2 orchestrator-centric architecture
version: 2.0.0
date: 2025-01-26
status: Integration Specification
---

# Testing Infrastructure Integration Specification

## Overview

This specification details the integration of the comprehensive testing infrastructure from the `nestgate-gitclone` repository into our v2 orchestrator-centric architecture. The integration will provide sophisticated mock services, performance testing, and scenario-based testing capabilities.

## Integration Objectives

### Primary Goals
1. **Comprehensive Testing**: Provide complete testing coverage for v2 orchestrator architecture
2. **Mock Services**: Create realistic mock implementations of all v2 components
3. **Performance Testing**: Establish performance benchmarks and regression testing
4. **Scenario Testing**: Enable complex failure and recovery scenario testing

### Success Criteria
- ✅ All v2 components can be mocked for isolated testing
- ✅ Performance benchmarks established for orchestrator operations
- ✅ Test scenarios cover major failure and recovery modes
- ✅ CI/CD pipeline validates all tests automatically
- ✅ New developers can run full test suite in <15 minutes

## Component Analysis

### GitClone Testing Components to Integrate

#### 1. Mock NAS Tool (`tools/mock-nas`)
```rust
// Current GitClone mock implementation
pub struct MockNas {
    storage_pools: HashMap<String, StoragePool>,
    network_interfaces: HashMap<String, NetworkInterface>,
    system_state: SystemState,
}

impl MockNas {
    pub async fn simulate_operation(&self, operation: NasOperation) -> Result<NasResponse, MockError> {
        // Simulate NAS operations with realistic delays and responses
    }
    
    pub async fn inject_failure(&self, failure_type: FailureType) -> Result<(), MockError> {
        // Inject specific failure scenarios for testing
    }
}
```

#### 2. Performance Testing Framework (`tests/performance`)
```rust
// GitClone performance testing structure
pub struct PerformanceTest {
    test_name: String,
    concurrent_operations: usize,
    duration: Duration,
    metrics_collector: MetricsCollector,
}

impl PerformanceTest {
    pub async fn run(&self) -> Result<PerformanceReport, TestError> {
        // Execute performance test and collect metrics
    }
}
```

#### 3. Integration Test Framework (`tests/integration`)
```rust
// GitClone integration testing patterns
pub struct IntegrationTestSuite {
    test_environment: TestEnvironment,
    mock_services: HashMap<String, Box<dyn MockService>>,
}

impl IntegrationTestSuite {
    pub async fn run_test_scenario(&self, scenario: TestScenario) -> Result<TestResult, TestError> {
        // Execute integration test scenarios
    }
}
```

## v2 Testing Architecture

### Mock Orchestrator System
```rust
// tests/mock/orchestrator.rs
pub struct MockOrchestrator {
    service_registry: MockServiceRegistry,
    connection_proxy: MockConnectionProxy,
    health_monitor: MockHealthMonitor,
    mcp_federation: MockMcpFederation,
    configuration: MockConfiguration,
}

impl MockOrchestrator {
    pub async fn new() -> Self {
        Self {
            service_registry: MockServiceRegistry::new(),
            connection_proxy: MockConnectionProxy::new(),
            health_monitor: MockHealthMonitor::new(),
            mcp_federation: MockMcpFederation::new(),
            configuration: MockConfiguration::default(),
        }
    }
    
    pub async fn start(&self) -> Result<MockOrchestratorHandle, MockError> {
        // Start mock orchestrator with all services
        let handle = MockOrchestratorHandle::new(self);
        
        // Initialize mock services
        self.service_registry.start().await?;
        self.connection_proxy.start().await?;
        self.health_monitor.start().await?;
        
        Ok(handle)
    }
    
    pub async fn simulate_service_failure(&self, service_name: &str) -> Result<(), MockError> {
        // Simulate specific service failure
        self.service_registry.mark_service_unhealthy(service_name).await?;
        self.health_monitor.trigger_health_check().await?;
        
        // Wait for orchestrator to detect and respond
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        Ok(())
    }
    
    pub async fn simulate_federation_loss(&self) -> Result<(), MockError> {
        // Simulate MCP federation connection loss
        self.mcp_federation.disconnect().await?;
        
        // Verify graceful degradation to standalone mode
        let status = self.get_federation_status().await?;
        assert_eq!(status, FederationStatus::Standalone);
        
        Ok(())
    }
    
    pub async fn simulate_high_load(&self, concurrent_requests: usize) -> Result<LoadTestMetrics, MockError> {
        // Simulate high concurrent load
        let mut handles = Vec::new();
        let start_time = Instant::now();
        
        for i in 0..concurrent_requests {
            let proxy = self.connection_proxy.clone();
            let handle = tokio::spawn(async move {
                proxy.handle_request(MockRequest::new(format!("request-{}", i))).await
            });
            handles.push(handle);
        }
        
        // Wait for all requests to complete
        let results = futures::future::join_all(handles).await;
        let duration = start_time.elapsed();
        
        // Calculate metrics
        let successful_requests = results.iter().filter(|r| r.is_ok()).count();
        let failed_requests = results.len() - successful_requests;
        
        Ok(LoadTestMetrics {
            total_requests: concurrent_requests,
            successful_requests,
            failed_requests,
            duration,
            requests_per_second: successful_requests as f64 / duration.as_secs_f64(),
        })
    }
}
```

### Mock Service Components
```rust
// tests/mock/services.rs
pub struct MockServiceRegistry {
    services: Arc<RwLock<HashMap<String, MockServiceInfo>>>,
    health_status: Arc<RwLock<HashMap<String, HealthStatus>>>,
}

impl MockServiceRegistry {
    pub async fn register_service(&self, service: MockServiceInfo) -> Result<(), MockError> {
        let mut services = self.services.write().await;
        services.insert(service.name.clone(), service);
        
        let mut health = self.health_status.write().await;
        health.insert(service.name.clone(), HealthStatus::Healthy);
        
        Ok(())
    }
    
    pub async fn mark_service_unhealthy(&self, service_name: &str) -> Result<(), MockError> {
        let mut health = self.health_status.write().await;
        health.insert(service_name.to_string(), HealthStatus::Unhealthy);
        Ok(())
    }
    
    pub async fn discover_service(&self, service_name: &str) -> Option<MockServiceInfo> {
        let services = self.services.read().await;
        services.get(service_name).cloned()
    }
}

pub struct MockConnectionProxy {
    routing_rules: HashMap<String, String>,
    request_latency: Duration,
    failure_rate: f64,
}

impl MockConnectionProxy {
    pub async fn handle_request(&self, request: MockRequest) -> Result<MockResponse, MockError> {
        // Simulate request latency
        tokio::time::sleep(self.request_latency).await;
        
        // Simulate random failures
        if rand::random::<f64>() < self.failure_rate {
            return Err(MockError::RequestFailed);
        }
        
        // Route request to appropriate mock service
        let service_name = self.determine_target_service(&request.path)?;
        let response = self.forward_to_mock_service(&service_name, request).await?;
        
        Ok(response)
    }
    
    pub fn set_failure_rate(&mut self, rate: f64) {
        self.failure_rate = rate;
    }
    
    pub fn set_latency(&mut self, latency: Duration) {
        self.request_latency = latency;
    }
}

pub struct MockHealthMonitor {
    check_interval: Duration,
    failure_threshold: usize,
    service_failures: HashMap<String, usize>,
}

impl MockHealthMonitor {
    pub async fn trigger_health_check(&self) -> Result<(), MockError> {
        // Manually trigger health check for testing
        self.check_all_services().await
    }
    
    async fn check_all_services(&self) -> Result<(), MockError> {
        // Simulate health checking logic
        Ok(())
    }
}
```

### Test Scenario Framework
```rust
// tests/scenarios/mod.rs
#[derive(Debug, Clone)]
pub enum TestScenario {
    ServiceRegistration {
        service_name: String,
        expected_outcome: RegistrationOutcome,
    },
    ServiceFailure {
        service_name: String,
        failure_type: FailureType,
        recovery_expected: bool,
    },
    FederationConnection {
        mcp_endpoint: String,
        connection_timeout: Duration,
    },
    FederationLoss {
        disconnect_after: Duration,
        recovery_timeout: Duration,
    },
    HighLoad {
        concurrent_requests: usize,
        duration: Duration,
        expected_success_rate: f64,
    },
    SecurityBreach {
        attack_type: AttackType,
        expected_mitigation: MitigationType,
    },
}

pub struct ScenarioRunner {
    mock_orchestrator: MockOrchestrator,
    metrics_collector: MetricsCollector,
    test_environment: TestEnvironment,
}

impl ScenarioRunner {
    pub async fn new() -> Result<Self, TestError> {
        let mock_orchestrator = MockOrchestrator::new().await;
        let metrics_collector = MetricsCollector::new();
        let test_environment = TestEnvironment::setup().await?;
        
        Ok(Self {
            mock_orchestrator,
            metrics_collector,
            test_environment,
        })
    }
    
    pub async fn run_scenario(&self, scenario: TestScenario) -> Result<TestReport, TestError> {
        let start_time = Instant::now();
        
        let result = match scenario {
            TestScenario::ServiceRegistration { service_name, expected_outcome } => {
                self.run_service_registration_scenario(&service_name, expected_outcome).await
            }
            TestScenario::ServiceFailure { service_name, failure_type, recovery_expected } => {
                self.run_service_failure_scenario(&service_name, failure_type, recovery_expected).await
            }
            TestScenario::FederationConnection { mcp_endpoint, connection_timeout } => {
                self.run_federation_connection_scenario(&mcp_endpoint, connection_timeout).await
            }
            TestScenario::FederationLoss { disconnect_after, recovery_timeout } => {
                self.run_federation_loss_scenario(disconnect_after, recovery_timeout).await
            }
            TestScenario::HighLoad { concurrent_requests, duration, expected_success_rate } => {
                self.run_high_load_scenario(concurrent_requests, duration, expected_success_rate).await
            }
            TestScenario::SecurityBreach { attack_type, expected_mitigation } => {
                self.run_security_breach_scenario(attack_type, expected_mitigation).await
            }
        };
        
        let duration = start_time.elapsed();
        let metrics = self.metrics_collector.collect_metrics().await;
        
        Ok(TestReport {
            scenario: scenario.clone(),
            result,
            duration,
            metrics,
        })
    }
    
    async fn run_service_registration_scenario(
        &self,
        service_name: &str,
        expected_outcome: RegistrationOutcome,
    ) -> Result<ScenarioResult, TestError> {
        // Test service registration
        let service_info = MockServiceInfo {
            name: service_name.to_string(),
            endpoint: format!("http://localhost:808{}", rand::random::<u8>() % 10),
            service_type: ServiceType::Storage,
        };
        
        let registration_result = self.mock_orchestrator
            .service_registry
            .register_service(service_info)
            .await;
        
        match (registration_result, expected_outcome) {
            (Ok(_), RegistrationOutcome::Success) => Ok(ScenarioResult::Passed),
            (Err(_), RegistrationOutcome::Failure) => Ok(ScenarioResult::Passed),
            _ => Ok(ScenarioResult::Failed("Unexpected registration outcome".to_string())),
        }
    }
    
    async fn run_service_failure_scenario(
        &self,
        service_name: &str,
        failure_type: FailureType,
        recovery_expected: bool,
    ) -> Result<ScenarioResult, TestError> {
        // First register the service
        let service_info = MockServiceInfo {
            name: service_name.to_string(),
            endpoint: "http://localhost:8081".to_string(),
            service_type: ServiceType::Storage,
        };
        
        self.mock_orchestrator.service_registry.register_service(service_info).await?;
        
        // Inject failure
        match failure_type {
            FailureType::ServiceCrash => {
                self.mock_orchestrator.simulate_service_failure(service_name).await?;
            }
            FailureType::NetworkPartition => {
                // Simulate network partition
                self.mock_orchestrator.connection_proxy.set_failure_rate(1.0);
            }
            FailureType::HighLatency => {
                // Simulate high latency
                self.mock_orchestrator.connection_proxy.set_latency(Duration::from_secs(5));
            }
        }
        
        // Wait for orchestrator to detect and respond
        tokio::time::sleep(Duration::from_secs(2)).await;
        
        // Check if recovery occurred as expected
        let service_status = self.mock_orchestrator
            .service_registry
            .get_service_health(service_name)
            .await;
        
        match (service_status, recovery_expected) {
            (Some(HealthStatus::Healthy), true) => Ok(ScenarioResult::Passed),
            (Some(HealthStatus::Unhealthy), false) => Ok(ScenarioResult::Passed),
            _ => Ok(ScenarioResult::Failed("Unexpected recovery outcome".to_string())),
        }
    }
}
```

### Performance Testing Framework
```rust
// tests/performance/orchestrator_bench.rs
pub struct OrchestratorPerformanceTest {
    mock_orchestrator: MockOrchestrator,
    test_config: PerformanceTestConfig,
}

impl OrchestratorPerformanceTest {
    pub async fn new(config: PerformanceTestConfig) -> Result<Self, TestError> {
        let mock_orchestrator = MockOrchestrator::new().await;
        
        Ok(Self {
            mock_orchestrator,
            test_config: config,
        })
    }
    
    pub async fn run_service_registration_benchmark(&self) -> Result<BenchmarkResult, TestError> {
        let start_time = Instant::now();
        let mut successful_registrations = 0;
        let mut failed_registrations = 0;
        
        for i in 0..self.test_config.iterations {
            let service_info = MockServiceInfo {
                name: format!("service-{}", i),
                endpoint: format!("http://localhost:808{}", i % 10),
                service_type: ServiceType::Storage,
            };
            
            match self.mock_orchestrator.service_registry.register_service(service_info).await {
                Ok(_) => successful_registrations += 1,
                Err(_) => failed_registrations += 1,
            }
        }
        
        let duration = start_time.elapsed();
        
        Ok(BenchmarkResult {
            test_name: "service_registration".to_string(),
            iterations: self.test_config.iterations,
            duration,
            successful_operations: successful_registrations,
            failed_operations: failed_registrations,
            operations_per_second: successful_registrations as f64 / duration.as_secs_f64(),
        })
    }
    
    pub async fn run_request_routing_benchmark(&self) -> Result<BenchmarkResult, TestError> {
        let start_time = Instant::now();
        let mut successful_requests = 0;
        let mut failed_requests = 0;
        
        // Setup mock services
        for i in 0..5 {
            let service_info = MockServiceInfo {
                name: format!("service-{}", i),
                endpoint: format!("http://localhost:808{}", i),
                service_type: ServiceType::Storage,
            };
            self.mock_orchestrator.service_registry.register_service(service_info).await?;
        }
        
        // Run concurrent requests
        let mut handles = Vec::new();
        
        for i in 0..self.test_config.concurrent_requests {
            let proxy = self.mock_orchestrator.connection_proxy.clone();
            let handle = tokio::spawn(async move {
                let request = MockRequest {
                    path: format!("/api/storage/service-{}/health", i % 5),
                    method: "GET".to_string(),
                    body: None,
                };
                proxy.handle_request(request).await
            });
            handles.push(handle);
        }
        
        let results = futures::future::join_all(handles).await;
        
        for result in results {
            match result {
                Ok(Ok(_)) => successful_requests += 1,
                _ => failed_requests += 1,
            }
        }
        
        let duration = start_time.elapsed();
        
        Ok(BenchmarkResult {
            test_name: "request_routing".to_string(),
            iterations: self.test_config.concurrent_requests,
            duration,
            successful_operations: successful_requests,
            failed_operations: failed_requests,
            operations_per_second: successful_requests as f64 / duration.as_secs_f64(),
        })
    }
}
```

## Integration Implementation Plan

### Phase 1: Mock Infrastructure (Week 1)
```yaml
tasks:
  day_1_2:
    - Copy mock-nas tool to tests/mock/
    - Adapt mock infrastructure for v2 orchestrator
    - Create MockOrchestrator implementation
    - Basic mock service components
  
  day_3_4:
    - Implement MockServiceRegistry
    - Create MockConnectionProxy
    - Add MockHealthMonitor
    - Unit tests for mock components
  
  day_5:
    - Integration testing with mock orchestrator
    - Performance baseline for mock operations
    - Documentation for mock framework
```

### Phase 2: Scenario Testing (Week 2)
```yaml
tasks:
  day_1_2:
    - Create test scenario framework
    - Implement basic failure scenarios
    - Add federation testing scenarios
    - Service registration/failure tests
  
  day_3_4:
    - Implement high-load testing scenarios
    - Add security breach scenarios
    - Create scenario runner infrastructure
    - Integration with CI/CD pipeline
  
  day_5:
    - End-to-end scenario testing
    - Performance optimization
    - Test result reporting
```

### Phase 3: Performance Testing (Week 3)
```yaml
tasks:
  day_1_2:
    - Adapt performance testing framework
    - Create orchestrator-specific benchmarks
    - Implement concurrent load testing
    - Baseline performance measurements
  
  day_3_4:
    - Advanced performance scenarios
    - Memory and resource usage testing
    - Latency and throughput analysis
    - Performance regression detection
  
  day_5:
    - Performance test automation
    - CI/CD integration
    - Performance monitoring setup
```

## Test Configuration

### Test Environment Configuration
```yaml
# tests/config/test_environment.yaml
test_environment:
  orchestrator:
    port: 18080  # Use different port for testing
    log_level: "debug"
    health_check_interval: 1s  # Faster for testing
    
  mock_services:
    nestgate_core:
      port: 18081
      latency: 10ms
      failure_rate: 0.01
    nestgate_network:
      port: 18082
      latency: 5ms
      failure_rate: 0.005
    nestgate_zfs:
      port: 18083
      latency: 20ms
      failure_rate: 0.02
      
  performance_testing:
    default_iterations: 1000
    concurrent_requests: 100
    test_duration: 30s
    
  scenario_testing:
    failure_injection_delay: 1s
    recovery_timeout: 10s
    health_check_frequency: 500ms
```

### CI/CD Integration
```yaml
# .github/workflows/test_integration.yml
name: Integration Testing

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  mock_testing:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
    - name: Run mock infrastructure tests
      run: cargo test --package tests-mock
      
  scenario_testing:
    runs-on: ubuntu-latest
    needs: mock_testing
    steps:
    - uses: actions/checkout@v3
    - name: Setup test environment
      run: ./tests/setup-test-env.sh
    - name: Run scenario tests
      run: cargo test --package tests-scenarios
      
  performance_testing:
    runs-on: ubuntu-latest
    needs: mock_testing
    steps:
    - uses: actions/checkout@v3
    - name: Run performance benchmarks
      run: cargo bench --package tests-performance
    - name: Upload performance results
      uses: actions/upload-artifact@v3
      with:
        name: performance-results
        path: target/criterion/
```

## Testing Strategy

### Unit Testing
```rust
// tests/unit/mock_orchestrator_test.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_mock_orchestrator_startup() {
        let mock_orchestrator = MockOrchestrator::new().await;
        let handle = mock_orchestrator.start().await.unwrap();
        
        // Verify all components started
        assert!(handle.is_running());
        assert!(mock_orchestrator.service_registry.is_ready().await);
        assert!(mock_orchestrator.connection_proxy.is_ready().await);
        assert!(mock_orchestrator.health_monitor.is_ready().await);
        
        handle.shutdown().await.unwrap();
    }
    
    #[tokio::test]
    async fn test_service_registration_mock() {
        let mock_orchestrator = MockOrchestrator::new().await;
        
        let service_info = MockServiceInfo {
            name: "test-service".to_string(),
            endpoint: "http://localhost:8081".to_string(),
            service_type: ServiceType::Storage,
        };
        
        let result = mock_orchestrator.service_registry.register_service(service_info).await;
        assert!(result.is_ok());
        
        let discovered = mock_orchestrator.service_registry.discover_service("test-service").await;
        assert!(discovered.is_some());
    }
    
    #[tokio::test]
    async fn test_failure_injection() {
        let mock_orchestrator = MockOrchestrator::new().await;
        
        // Register service
        let service_info = MockServiceInfo {
            name: "test-service".to_string(),
            endpoint: "http://localhost:8081".to_string(),
            service_type: ServiceType::Storage,
        };
        mock_orchestrator.service_registry.register_service(service_info).await.unwrap();
        
        // Inject failure
        mock_orchestrator.simulate_service_failure("test-service").await.unwrap();
        
        // Verify service marked as unhealthy
        let health = mock_orchestrator.service_registry.get_service_health("test-service").await;
        assert_eq!(health, Some(HealthStatus::Unhealthy));
    }
}
```

### Integration Testing
```rust
// tests/integration/orchestrator_integration_test.rs
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_full_orchestrator_workflow() {
        let scenario_runner = ScenarioRunner::new().await.unwrap();
        
        // Test service registration
        let registration_scenario = TestScenario::ServiceRegistration {
            service_name: "nestgate-core".to_string(),
            expected_outcome: RegistrationOutcome::Success,
        };
        
        let result = scenario_runner.run_scenario(registration_scenario).await.unwrap();
        assert_eq!(result.result.unwrap(), ScenarioResult::Passed);
        
        // Test service failure and recovery
        let failure_scenario = TestScenario::ServiceFailure {
            service_name: "nestgate-core".to_string(),
            failure_type: FailureType::ServiceCrash,
            recovery_expected: true,
        };
        
        let result = scenario_runner.run_scenario(failure_scenario).await.unwrap();
        assert_eq!(result.result.unwrap(), ScenarioResult::Passed);
    }
    
    #[tokio::test]
    async fn test_federation_scenarios() {
        let scenario_runner = ScenarioRunner::new().await.unwrap();
        
        // Test MCP federation connection
        let federation_scenario = TestScenario::FederationConnection {
            mcp_endpoint: "http://localhost:8443".to_string(),
            connection_timeout: Duration::from_secs(10),
        };
        
        let result = scenario_runner.run_scenario(federation_scenario).await.unwrap();
        // Should pass even if MCP not available (graceful degradation)
        assert!(result.result.is_ok());
    }
}
```

## Performance Benchmarks

### Baseline Performance Targets
```yaml
performance_targets:
  service_registration:
    operations_per_second: 1000
    max_latency: 10ms
    success_rate: 99.9%
    
  request_routing:
    operations_per_second: 5000
    max_latency: 5ms
    success_rate: 99.95%
    
  health_monitoring:
    check_frequency: 1000/second
    detection_latency: 100ms
    false_positive_rate: 0.1%
    
  federation_operations:
    connection_time: 5s
    message_latency: 50ms
    throughput: 100_messages/second
```

### Performance Regression Detection
```rust
// tests/performance/regression_detection.rs
pub struct PerformanceRegressionDetector {
    baseline_metrics: HashMap<String, BenchmarkResult>,
    tolerance: f64,
}

impl PerformanceRegressionDetector {
    pub fn check_regression(&self, current: &BenchmarkResult) -> Result<RegressionStatus, TestError> {
        if let Some(baseline) = self.baseline_metrics.get(&current.test_name) {
            let performance_change = (current.operations_per_second - baseline.operations_per_second) 
                / baseline.operations_per_second;
            
            if performance_change < -self.tolerance {
                return Ok(RegressionStatus::Regression {
                    performance_drop: performance_change,
                    baseline_ops: baseline.operations_per_second,
                    current_ops: current.operations_per_second,
                });
            }
        }
        
        Ok(RegressionStatus::NoRegression)
    }
}
```

## Summary

The testing infrastructure integration will provide NestGate v2 with comprehensive testing capabilities:

### Key Benefits
1. **Complete Coverage**: Mock implementations for all v2 components
2. **Realistic Testing**: Sophisticated failure injection and scenario testing
3. **Performance Monitoring**: Continuous performance benchmarking and regression detection
4. **CI/CD Integration**: Automated testing in development pipeline

### Testing Capabilities
- ✅ **Mock Orchestrator**: Complete mock implementation for isolated testing
- ✅ **Scenario Framework**: Complex failure and recovery scenario testing
- ✅ **Performance Testing**: Benchmarking and regression detection
- ✅ **Integration Testing**: End-to-end workflow validation

The integration ensures that v2 development can proceed with confidence, backed by comprehensive testing infrastructure that validates both functionality and performance. 