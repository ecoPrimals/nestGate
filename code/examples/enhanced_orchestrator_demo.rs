use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, warn, error};
use nestgate_orchestrator::{
    Orchestrator, OrchestratorConfig, NetworkConfig, EnvironmentConfig,
    ServiceInfo, ServiceHealth, ScalabilityConfig, HealthConfig, WebSocketConfig,
    InterTowerCommunicationConfig, TestingConfig,
    MockService, MockServiceConfig, TestScenario, TestStep, ChaosType,
    StressTestConfig, RecoveryType,
};
use nestgate_core::Result;

/// Example service that demonstrates the enhanced orchestrator capabilities
#[derive(Debug)]
struct ExampleComputeService {
    service_id: String,
    is_running: std::sync::atomic::AtomicBool,
    request_count: std::sync::atomic::AtomicU64,
    error_count: std::sync::atomic::AtomicU64,
    compute_load: std::sync::atomic::AtomicU64, // Simulated compute load
}

impl ExampleComputeService {
    fn new(service_id: String) -> Self {
        Self {
            service_id,
            is_running: std::sync::atomic::AtomicBool::new(false),
            request_count: std::sync::atomic::AtomicU64::new(0),
            error_count: std::sync::atomic::AtomicU64::new(0),
            compute_load: std::sync::atomic::AtomicU64::new(0),
        }
    }
    
    /// Simulate a compute-intensive task (like AlphaFold processing)
    async fn process_compute_task(&self, task_size: u64) -> Result<String> {
        self.request_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        
        // Simulate processing time based on task size
        let processing_time = Duration::from_millis(task_size * 10);
        sleep(processing_time).await;
        
        // Update compute load
        self.compute_load.store(task_size, std::sync::atomic::Ordering::Relaxed);
        
        // Simulate occasional failures
        if rand::random::<f64>() < 0.05 {
            self.error_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            return Err(nestgate_core::NestGateError::Internal("Compute task failed".to_string()));
        }
        
        Ok(format!("Compute task completed for service: {}", self.service_id))
    }
}

#[async_trait::async_trait]
impl ManagedService for ExampleComputeService {
    async fn start(&self) -> Result<()> {
        self.is_running.store(true, std::sync::atomic::Ordering::Relaxed);
        info!("ExampleComputeService '{}' started", self.service_id);
        Ok(())
    }
    
    async fn stop(&self) -> Result<()> {
        self.is_running.store(false, std::sync::atomic::Ordering::Relaxed);
        info!("ExampleComputeService '{}' stopped", self.service_id);
        Ok(())
    }
    
    async fn health_check(&self) -> Result<ServiceHealth> {
        if !self.is_running.load(std::sync::atomic::Ordering::Relaxed) {
            return Ok(ServiceHealth::Unknown);
        }
        
        let load = self.compute_load.load(std::sync::atomic::Ordering::Relaxed);
        let error_rate = self.get_error_rate().await;
        
        // Determine health based on load and error rate
        if error_rate > 0.2 {
            Ok(ServiceHealth::Unhealthy)
        } else if load > 80 || error_rate > 0.1 {
            Ok(ServiceHealth::Degraded)
        } else {
            Ok(ServiceHealth::Healthy)
        }
    }
    
    async fn get_load(&self) -> Result<f64> {
        let load = self.compute_load.load(std::sync::atomic::Ordering::Relaxed);
        Ok(load as f64 / 100.0) // Normalize to 0.0-1.0
    }
    
    async fn get_metrics(&self) -> Result<ServiceMetrics> {
        let error_rate = self.get_error_rate().await;
        
        Ok(ServiceMetrics {
            request_count: self.request_count.load(std::sync::atomic::Ordering::Relaxed),
            error_count: self.error_count.load(std::sync::atomic::Ordering::Relaxed),
            error_rate,
            response_time_ms: 50.0, // Simulated
            throughput_rps: 10.0,   // Simulated
            cpu_usage: self.compute_load.load(std::sync::atomic::Ordering::Relaxed) as f64 / 100.0,
            memory_usage: 0.3,      // Simulated
        })
    }
    
    async fn restart(&self) -> Result<()> {
        info!("Restarting service: {}", self.service_id);
        self.stop().await?;
        sleep(Duration::from_secs(1)).await;
        self.start().await?;
        Ok(())
    }
}

impl ExampleComputeService {
    async fn get_error_rate(&self) -> f64 {
        let total_requests = self.request_count.load(std::sync::atomic::Ordering::Relaxed);
        let error_count = self.error_count.load(std::sync::atomic::Ordering::Relaxed);
        
        if total_requests == 0 {
            0.0
        } else {
            error_count as f64 / total_requests as f64
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("debug")
        .init();
    
    info!("Starting Enhanced Orchestrator Demo");
    
    // Create enhanced orchestrator configuration
    let config = OrchestratorConfig {
        bind_address: "0.0.0.0:8090".to_string(),
        mcp_config: None, // Simplified for demo
        communication_config: Some(InterTowerCommunicationConfig {
            tower_id: "demo-tower-001".to_string(),
            bind_port: 8091,
            encryption_enabled: true,
            max_connections_per_tower: 20,
        }),
        scalability_config: ScalabilityConfig {
            registry_shard_count: 8,
            port_range_start: 9000,
            port_range_end: 9999,
            port_range_size: 100,
            websocket_config: WebSocketConfig {
                max_connections: 1000,
                max_message_queue_size: 100,
                message_rate_limit: 50,
                connection_timeout: Duration::from_secs(300),
                backpressure_threshold: 0.8,
            },
        },
        health_config: HealthConfig {
            check_interval: Duration::from_secs(5),
            failure_threshold: 3,
            recovery_threshold: 2,
            timeout: Duration::from_secs(10),
            degraded_threshold: 0.7,
            critical_threshold: 0.5,
        },
        websocket_config: WebSocketConfig::default(),
        testing_config: Some(TestingConfig {
            enable_mock_services: true,
            enable_chaos_engineering: true,
            stress_test_config: Some(StressTestConfig {
                target: "http://localhost:8090".to_string(),
                concurrent_connections: 50,
                duration: Duration::from_secs(30),
            }),
        }),
    };
    
    // Create enhanced orchestrator
    let orchestrator = Orchestrator::new(config)?;
    
    // Start orchestrator
    info!("Starting enhanced orchestrator...");
    let handles = orchestrator.start().await?;
    info!("Enhanced orchestrator started with {} background tasks", handles.len());
    
    // Register example compute services
    info!("Registering compute services...");
    for i in 1..=5 {
        let service_id = format!("compute-service-{:03}", i);
        let service_info = ServiceInfo {
            service_id: service_id.clone(),
            service_type: "compute".to_string(),
            description: format!("Example compute service {}", i),
            version: "1.0.0".to_string(),
            dependencies: vec![],
            resource_requirements: Default::default(),
        };
        
        let service = Box::new(ExampleComputeService::new(service_id.clone()));
        orchestrator.register_enhanced_service(service_info, service).await?;
        
        info!("Registered service: {}", service_id);
    }
    
    // Demonstrate system status monitoring
    info!("Demonstrating system status monitoring...");
    for i in 0..5 {
        sleep(Duration::from_secs(2)).await;
        let status = orchestrator.get_system_status().await;
        info!("System Status #{}: Health={:?}, Services={}, Load={:.2}, Ports={}/{}, WebSockets={}",
              i + 1,
              status.overall_health,
              status.total_services,
              status.system_load,
              status.port_allocation_stats.allocated_ports,
              status.port_allocation_stats.total_ports,
              status.websocket_connections
        );
    }
    
    // Demonstrate recovery capabilities
    info!("Demonstrating recovery capabilities...");
    if let Err(e) = orchestrator.health_monitoring.trigger_recovery(
        "compute-service-001",
        RecoveryType::Restart
    ).await {
        warn!("Failed to trigger recovery: {}", e);
    } else {
        info!("Recovery triggered for compute-service-001");
    }
    
    // Demonstrate testing infrastructure
    if let Some(testing_infra) = &orchestrator.testing_infrastructure {
        info!("Demonstrating testing infrastructure...");
        
        // Create mock services for testing
        let mock_config = MockServiceConfig {
            failure_rate: 0.1,
            response_time: Duration::from_millis(100),
            resource_usage: 0.5,
        };
        
        for i in 1..=3 {
            let mock_id = format!("mock-service-{:03}", i);
            testing_infra.create_mock_service(mock_id, mock_config.clone());
        }
        
        // Create test scenario
        let test_scenario = TestScenario {
            name: "Basic Service Test".to_string(),
            description: "Test basic service operations".to_string(),
            steps: vec![
                TestStep::StartService { service_id: "mock-service-001".to_string() },
                TestStep::WaitFor { duration: Duration::from_secs(1) },
                TestStep::SendLoad { 
                    target: "mock-service-001".to_string(), 
                    duration: Duration::from_secs(5), 
                    rps: 10 
                },
                TestStep::AssertHealth { 
                    service_id: "mock-service-001".to_string(), 
                    expected_health: ServiceHealth::Healthy 
                },
                TestStep::InjectChaos { 
                    chaos_type: ChaosType::ServiceFailure { 
                        service_id: "mock-service-001".to_string(), 
                        duration: Duration::from_secs(2) 
                    } 
                },
                TestStep::WaitFor { duration: Duration::from_secs(3) },
                TestStep::StopService { service_id: "mock-service-001".to_string() },
            ],
        };
        
        testing_infra.test_scenarios.insert("basic-test".to_string(), test_scenario);
        
        // Run tests
        info!("Running integration tests...");
        match testing_infra.run_integration_tests().await {
            Ok(results) => {
                info!("Integration tests completed - Passed: {}, Failed: {}", 
                      results.passed, results.failed);
            }
            Err(e) => {
                error!("Integration tests failed: {}", e);
            }
        }
        
        // Run stress tests
        info!("Running stress tests...");
        let stress_config = StressTestConfig {
            target: "localhost:8090".to_string(),
            concurrent_connections: 25,
            duration: Duration::from_secs(10),
        };
        
        match testing_infra.run_stress_tests(stress_config).await {
            Ok(results) => {
                info!("Stress tests completed - RPS: {:.2}, Error Rate: {:.2}%", 
                      results.requests_per_second, results.error_rate * 100.0);
            }
            Err(e) => {
                error!("Stress tests failed: {}", e);
            }
        }
    }
    
    // Demonstrate inter-tower communication
    info!("Demonstrating inter-tower communication...");
    let tower_message = nestgate_orchestrator::TowerMessage {
        message_id: uuid::Uuid::new_v4().to_string(),
        source_tower: "demo-tower-001".to_string(),
        target_tower: "demo-tower-002".to_string(),
        message_type: nestgate_orchestrator::TowerMessageType::HealthCheck,
        payload: serde_json::json!({
            "timestamp": chrono::Utc::now(),
            "status": "healthy"
        }),
        timestamp: chrono::Utc::now(),
        priority: 1,
    };
    
    // In a real scenario, this would send to another tower
    info!("Would send message to tower: {}", tower_message.target_tower);
    
    // Final system status
    info!("Final system status check...");
    let final_status = orchestrator.get_system_status().await;
    info!("Final Status: Health={:?}, Services={}, Load={:.2}",
          final_status.overall_health,
          final_status.total_services,
          final_status.system_load
    );
    
    info!("Enhanced Orchestrator Demo completed successfully!");
    
    // Keep running for a bit to see background tasks
    info!("Monitoring system for 30 seconds...");
    sleep(Duration::from_secs(30)).await;
    
    // Graceful shutdown
    info!("Shutting down enhanced orchestrator...");
    for handle in handles {
        handle.abort();
    }
    
    Ok(())
} 