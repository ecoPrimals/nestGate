#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;
    use uuid::Uuid;
    use chrono::Utc;
    use std::collections::HashMap;

    /// Mock Toadstool client for testing
    #[derive(Debug, Clone)]
    pub struct MockToadstoolComputeClient {
        /// Mock base URL
        pub base_url: String,
        /// Mock responses
        pub mock_responses: Arc<RwLock<HashMap<String, serde_json::Value>>>,
        /// Mock allocations
        pub mock_allocations: Arc<RwLock<HashMap<String, ComputeAllocation>>>,
        /// Mock metrics
        pub mock_metrics: Arc<RwLock<LiveHardwareMetrics>>,
    }

    impl MockToadstoolComputeClient {
        pub fn new() -> Self {
            let mut mock_responses = HashMap::new();
            mock_responses.insert(
                "register_service".to_string(),
                serde_json::json!({"status": "registered"}),
            );

            let mock_allocation = ComputeAllocation {
                allocation_id: "mock-allocation-1".to_string(),
                cpu_cores: 8,
                memory_gb: 16,
                gpu_allocation: Some(GpuAllocation {
                    gpu_count: 1,
                    gpu_type: "NVIDIA RTX 4090".to_string(),
                    memory_gb: 24,
                }),
                expires_at: Utc::now() + chrono::Duration::hours(1),
                compute_node: "node-1".to_string(),
            };

            let mut mock_allocations = HashMap::new();
            mock_allocations.insert("mock-allocation-1".to_string(), mock_allocation);

            let mock_metrics = LiveHardwareMetrics {
                timestamp: Utc::now(),
                cpu_usage: 42.5,
                memory_usage: 68.3,
                gpu_usage: Some(25.7),
                temperature: 65.2,
                power_consumption: 150.0,
                network_io: NetworkIoMetrics {
                    bytes_sent: 1024000,
                    bytes_received: 2048000,
                    packets_sent: 5000,
                    packets_received: 7500,
                },
                disk_io: DiskIoMetrics {
                    read_bytes: 512000,
                    write_bytes: 256000,
                    read_ops: 100,
                    write_ops: 50,
                },
            };

            Self {
                base_url: "http://mock-toadstool:8080".to_string(),
                mock_responses: Arc::new(RwLock::new(mock_responses)),
                mock_allocations: Arc::new(RwLock::new(mock_allocations)),
                mock_metrics: Arc::new(RwLock::new(mock_metrics)),
            }
        }

        pub async fn register_tuning_service(&self, service: &TuningServiceRegistration) -> Result<()> {
            println!("🧪 Mock: Registering service: {}", service.name);
            Ok(())
        }

        pub async fn request_compute_resources(&self, request: &ComputeResourceRequest) -> Result<ComputeAllocation> {
            println!("🧪 Mock: Requesting {} cores, {} GB RAM", request.cpu_cores, request.memory_gb);

            let allocation = ComputeAllocation {
                allocation_id: format!("mock-{}", Uuid::new_v4()),
                cpu_cores: request.cpu_cores,
                memory_gb: request.memory_gb,
                gpu_allocation: if request.gpu_required {
                    Some(GpuAllocation {
                        gpu_count: 1,
                        gpu_type: "Mock GPU".to_string(),
                        memory_gb: 8,
                    })
                } else {
                    None
                },
                expires_at: Utc::now() + chrono::Duration::hours(1),
                compute_node: "mock-node".to_string(),
            };

            Ok(allocation)
        }

        pub async fn get_live_hardware_metrics(&self) -> Result<LiveHardwareMetrics> {
            let metrics = self.mock_metrics.read().await;
            Ok(metrics.clone())
        }

        pub async fn subscribe_to_hardware_feed(&self, _callback: Box<dyn Fn(HardwareEvent) + Send + Sync>) -> Result<()> {
            println!("🧪 Mock: Subscribed to hardware feed");
            Ok(())
        }

        pub async fn release_compute_resources(&self, allocation_id: &str) -> Result<()> {
            println!("🧪 Mock: Released allocation: {}", allocation_id);
            Ok(())
        }
    }

    /// Mock hardware tuning handler for testing
    pub struct MockHardwareTuningHandler {
        pub mock_client: MockToadstoolComputeClient,
        pub sessions: Arc<RwLock<HashMap<Uuid, TuningSession>>>,
    }

    impl MockHardwareTuningHandler {
        pub fn new() -> Self {
            Self {
                mock_client: MockToadstoolComputeClient::new(),
                sessions: Arc::new(RwLock::new(HashMap::new())),
            }
        }

        pub async fn mock_auto_tune(&self) -> Result<TuningResult> {
            let mock_result = TuningResult {
                profile_name: "mock_performance".to_string(),
                optimizations_applied: vec![
                    "cpu_frequency_scaling".to_string(),
                    "memory_prefetch_tuning".to_string(),
                    "cache_optimization".to_string(),
                ],
                estimated_performance_gain: 25.5,
            };

            Ok(mock_result)
        }

        pub async fn mock_benchmark(&self, benchmark_name: &str) -> Result<BenchmarkResult> {
            let mock_result = BenchmarkResult {
                name: benchmark_name.to_string(),
                timestamp: Utc::now(),
                hardware_config: HardwareConfiguration {
                    cpu_cores: 8,
                    memory_gb: 16,
                    storage_devices: vec![],
                    network_interfaces: vec![],
                    accelerators: vec![],
                },
                metrics: PerformanceMetrics {
                    cpu_score: 85.2,
                    memory_score: 91.7,
                    storage_score: 76.3,
                    network_score: 88.9,
                    overall_score: 85.5,
                    latency_ms: 1.8,
                    throughput_mbps: 1150.0,
                    iops: 12000,
                },
                baseline_comparison: Some(15.2),
            };

            Ok(mock_result)
        }
    }

    #[test]
    async fn test_mock_toadstool_client_registration() {
        let mock_client = MockToadstoolComputeClient::new();

        let registration = TuningServiceRegistration {
            name: "test-tuning-service".to_string(),
            service_type: "hardware_optimization".to_string(),
            capabilities: vec!["cpu_tuning".to_string()],
            resource_requirements: ResourceRequirements {
                min_cpu_cores: 2,
                min_memory_gb: 4,
                preferred_cpu_cores: 4,
                preferred_memory_gb: 8,
                gpu_required: false,
            },
            health_check_url: "http://test:3000/health".to_string(),
        };

        let result = mock_client.register_tuning_service(&registration).await;
        assert!(result.is_ok());
    }

    #[test]
    async fn test_mock_compute_resource_allocation() {
        let mock_client = MockToadstoolComputeClient::new();

        let request = ComputeResourceRequest {
            session_id: Uuid::new_v4(),
            cpu_cores: 4,
            memory_gb: 8,
            gpu_required: true,
            duration_minutes: Some(30),
            priority: ComputePriority::Normal,
        };

        let result = mock_client.request_compute_resources(&request).await;
        assert!(result.is_ok());

        let allocation = result.expect("Failed to get compute allocation from mock client");
        assert_eq!(allocation.cpu_cores, 4);
        assert_eq!(allocation.memory_gb, 8);
        assert!(allocation.gpu_allocation.is_some());
    }

    #[test]
    async fn test_mock_live_hardware_metrics() {
        let mock_client = MockToadstoolComputeClient::new();

        let result = mock_client.get_live_hardware_metrics().await;
        assert!(result.is_ok());

        let metrics = result.expect("Failed to get hardware metrics from mock client");
        assert!(metrics.cpu_usage > 0.0);
        assert!(metrics.memory_usage > 0.0);
        assert!(metrics.gpu_usage.is_some());
        assert!(metrics.temperature > 0.0);
    }

    #[test]
    async fn test_mock_hardware_tuning_handler() {
        let mock_handler = MockHardwareTuningHandler::new();

        // Test auto-tuning
        let tuning_result = mock_handler.mock_auto_tune().await;
        assert!(tuning_result.is_ok());

        let result = tuning_result.expect("Failed to get tuning result");
        assert_eq!(result.profile_name, "mock_performance");
        assert!(!result.optimizations_applied.is_empty());
        assert!(result.estimated_performance_gain > 0.0);
    }

    #[test]
    async fn test_mock_benchmark() {
        let mock_handler = MockHardwareTuningHandler::new();

        let benchmark_result = mock_handler.mock_benchmark("cpu_intensive").await;
        assert!(benchmark_result.is_ok());

        let result = benchmark_result.expect("Failed to get benchmark result");
        assert_eq!(result.name, "cpu_intensive");
        assert!(result.metrics.overall_score > 0.0);
        assert!(result.baseline_comparison.is_some());
    }

    #[test]
    async fn test_mock_resource_lifecycle() {
        let mock_client = MockToadstoolComputeClient::new();

        // Request resources
        let request = ComputeResourceRequest {
            session_id: Uuid::new_v4(),
            cpu_cores: 2,
            memory_gb: 4,
            gpu_required: false,
            duration_minutes: Some(15),
            priority: ComputePriority::Low,
        };

        let allocation = mock_client.request_compute_resources(&request).await.expect("Failed to allocate compute resources");
        assert!(!allocation.allocation_id.is_empty());

        // Release resources
        let release_result = mock_client.release_compute_resources(&allocation.allocation_id).await;
        assert!(release_result.is_ok());
    }

    #[test]
    async fn test_mock_hardware_feed_subscription() {
        let mock_client = MockToadstoolComputeClient::new();

        let callback = Box::new(|event: HardwareEvent| {
            println!("Received mock event: {:?}", event.event_type);
        });

        let result = mock_client.subscribe_to_hardware_feed(callback).await;
        assert!(result.is_ok());
    }

    #[test]
    async fn test_mock_tuning_session_flow() {
        let mock_handler = MockHardwareTuningHandler::new();
        let session_id = Uuid::new_v4();

        // Create mock session
        let session = TuningSession {
            session_id,
            started_at: Utc::now(),
            hardware_config: HardwareConfiguration {
                cpu_cores: 4,
                memory_gb: 8,
                storage_devices: vec![],
                network_interfaces: vec![],
                accelerators: vec![],
            },
            tuning_profile: None,
            result: None,
            status: SessionStatus::Started,
            external_access_log: vec![],
        };

        // Store session
        {
            let mut sessions = mock_handler.sessions.write().await;
            sessions.insert(session_id, session);
        }

        // Verify session exists
        {
            let sessions = mock_handler.sessions.read().await;
            assert!(sessions.contains_key(&session_id));
        }

        // Mock tuning process
        let tuning_result = mock_handler.mock_auto_tune().await.expect("Failed to auto-tune hardware");
        assert!(tuning_result.estimated_performance_gain > 0.0);
    }
}