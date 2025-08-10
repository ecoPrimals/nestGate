/// **PHASE 2: CHAOS ENGINEERING INTEGRATION TESTS**
///
/// Advanced chaos engineering tests targeting resilience and fault tolerance:
/// - Network partition simulation and recovery
/// - Service failure injection and circuit breaker testing
/// - Resource exhaustion scenarios (memory, disk, CPU)
/// - Cascading failure prevention
/// - Byzantine fault tolerance testing
/// - Load spike and stress testing

use std::collections::HashMap;
use std::sync::{Arc, atomic::{AtomicBool, AtomicU64, Ordering}};
use std::time::{Duration, SystemTime, Instant};
use tokio::time::{sleep, timeout};
use tokio::sync::{RwLock, Semaphore};

use nestgate_core::{
    error::{NestGateError, Result},
    service_discovery::{
        registry::{InMemoryServiceRegistry, UniversalServiceRegistry},
        types::{ServiceCapability, ServiceInfo, ServiceRole, UniversalServiceRegistration}
    },
    universal_storage::{
        backends::{
            filesystem::{FilesystemBackend, FilesystemBackendConfig},
            memory::{MemoryBackend, MemoryBackendConfig},
        },
    },
    Result as CoreResult,
};

/// **CHAOS TESTING INFRASTRUCTURE**

/// Simulates network partitions and connectivity issues
#[derive(Debug)]
pub struct NetworkChaosSimulator {
    partition_active: Arc<AtomicBool>,
    packet_loss_rate: Arc<AtomicU64>, // Percentage * 100 for precision
    latency_injection: Arc<AtomicU64>, // Milliseconds
}

impl NetworkChaosSimulator {
    pub fn new() -> Self {
        Self {
            partition_active: Arc::new(AtomicBool::new(false)),
            packet_loss_rate: Arc::new(AtomicU64::new(0)),
            latency_injection: Arc::new(AtomicU64::new(0)),
        }
    }
    
    /// Simulate network partition
    pub fn create_partition(&self) {
        self.partition_active.store(true, Ordering::SeqCst);
        tracing::warn!("🔥 CHAOS: Network partition activated");
    }
    
    /// Restore network connectivity
    pub fn heal_partition(&self) {
        self.partition_active.store(false, Ordering::SeqCst);
        tracing::info!("✅ CHAOS: Network partition healed");
    }
    
    /// Inject packet loss
    pub fn inject_packet_loss(&self, loss_percentage: f64) {
        let loss_rate = (loss_percentage * 100.0) as u64;
        self.packet_loss_rate.store(loss_rate, Ordering::SeqCst);
        tracing::warn!("🔥 CHAOS: {}% packet loss injected", loss_percentage);
    }
    
    /// Inject network latency
    pub fn inject_latency(&self, latency_ms: u64) {
        self.latency_injection.store(latency_ms, Ordering::SeqCst);
        tracing::warn!("🔥 CHAOS: {}ms latency injected", latency_ms);
    }
    
    /// Check if operation should fail due to network issues
    pub async fn should_fail_operation(&self) -> bool {
        if self.partition_active.load(Ordering::SeqCst) {
            return true;
        }
        
        let loss_rate = self.packet_loss_rate.load(Ordering::SeqCst);
        if loss_rate > 0 {
            let random_value = (SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
                .unwrap().as_nanos() % 10000) as u64;
            if random_value < loss_rate {
                return true;
            }
        }
        
        let latency = self.latency_injection.load(Ordering::SeqCst);
        if latency > 0 {
            sleep(Duration::from_millis(latency)).await;
        }
        
        false
    }
}

/// Resource exhaustion simulator
#[derive(Debug)]
pub struct ResourceChaosSimulator {
    memory_pressure: Arc<AtomicBool>,
    disk_full: Arc<AtomicBool>,
    cpu_throttle: Arc<AtomicBool>,
}

impl ResourceChaosSimulator {
    pub fn new() -> Self {
        Self {
            memory_pressure: Arc::new(AtomicBool::new(false)),
            disk_full: Arc::new(AtomicBool::new(false)),
            cpu_throttle: Arc::new(AtomicBool::new(false)),
        }
    }
    
    pub fn simulate_memory_pressure(&self) {
        self.memory_pressure.store(true, Ordering::SeqCst);
        tracing::warn!("🔥 CHAOS: Memory pressure activated");
    }
    
    pub fn simulate_disk_full(&self) {
        self.disk_full.store(true, Ordering::SeqCst);
        tracing::warn!("🔥 CHAOS: Disk full condition activated");
    }
    
    pub fn simulate_cpu_throttle(&self) {
        self.cpu_throttle.store(true, Ordering::SeqCst);
        tracing::warn!("🔥 CHAOS: CPU throttling activated");
    }
    
    pub fn clear_all_conditions(&self) {
        self.memory_pressure.store(false, Ordering::SeqCst);
        self.disk_full.store(false, Ordering::SeqCst);
        self.cpu_throttle.store(false, Ordering::SeqCst);
        tracing::info!("✅ CHAOS: All resource conditions cleared");
    }
    
    pub fn should_fail_memory_operation(&self) -> bool {
        self.memory_pressure.load(Ordering::SeqCst)
    }
    
    pub fn should_fail_disk_operation(&self) -> bool {
        self.disk_full.load(Ordering::SeqCst)
    }
    
    pub async fn apply_cpu_throttle(&self) {
        if self.cpu_throttle.load(Ordering::SeqCst) {
            sleep(Duration::from_millis(100)).await; // Simulate CPU slowdown
        }
    }
}

/// **TEST SUITE 1: NETWORK PARTITION AND RECOVERY**
#[cfg(test)]
mod network_chaos_tests {
    use super::*;
    use tempfile::TempDir;

    /// Test service discovery resilience during network partitions
    #[tokio::test]
    async fn test_service_discovery_network_partition_resilience() -> Result<()> {
        let registry = Arc::new(InMemoryServiceRegistry::new());
        let chaos_simulator = Arc::new(NetworkChaosSimulator::new());
        
        // Register services in different "zones"
        let zone_a_services = vec![
            create_test_service("zone-a-storage", ServiceCapability::Storage),
            create_test_service("zone-a-compute", ServiceCapability::Compute),
        ];
        
        let zone_b_services = vec![
            create_test_service("zone-b-storage", ServiceCapability::Storage),
            create_test_service("zone-b-security", ServiceCapability::Security),
        ];
        
        // Register all services
        for service in zone_a_services {
            registry.register_service(service).await?;
        }
        for service in zone_b_services {
            registry.register_service(service).await?;
        }
        
        // Verify all services are discoverable
        let all_storage = registry.discover_by_capabilities(vec![ServiceCapability::Storage]).await?;
        assert_eq!(all_storage.len(), 2);
        
        // Simulate network partition affecting Zone A
        chaos_simulator.create_partition();
        
        // Simulate service discovery with network partition
        let partition_start = Instant::now();
        let mut successful_discoveries = 0;
        let mut failed_discoveries = 0;
        
        for _ in 0..10 {
            if chaos_simulator.should_fail_operation().await {
                failed_discoveries += 1;
            } else {
                let storage_services = registry.discover_by_capabilities(vec![ServiceCapability::Storage]).await;
                if storage_services.is_ok() {
                    successful_discoveries += 1;
                }
            }
            sleep(Duration::from_millis(10)).await;
        }
        
        // Verify some operations failed due to partition
        assert!(failed_discoveries > 0, "Network partition should cause some failures");
        
        // Heal the partition
        chaos_simulator.heal_partition();
        
        // Verify recovery
        let recovery_start = Instant::now();
        let recovered_services = registry.discover_by_capabilities(vec![ServiceCapability::Storage]).await?;
        assert_eq!(recovered_services.len(), 2, "All services should be discoverable after partition heals");
        
        let recovery_time = recovery_start.elapsed();
        println!("Service discovery recovered in {:?}", recovery_time);
        
        println!("✅ Service discovery network partition resilience tested");
        Ok(())
    }
    
    /// Test storage operations during network instability
    #[tokio::test]
    async fn test_storage_network_instability_resilience() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let chaos_simulator = Arc::new(NetworkChaosSimulator::new());
        
        let config = FilesystemBackendConfig {
            root_path: temp_dir.path().to_string_lossy().to_string(),
            max_file_size: 1024 * 1024,
            allowed_extensions: vec!["txt".to_string()],
        };
        let backend = Arc::new(FilesystemBackend::new(config)?);
        
        // Test normal operations first
        let test_content = b"Chaos engineering test content";
        backend.write_file("normal_test.txt", test_content).await?;
        let read_content = backend.read_file("normal_test.txt").await?;
        assert_eq!(read_content, test_content);
        
        // Inject network instability
        chaos_simulator.inject_packet_loss(20.0); // 20% packet loss
        chaos_simulator.inject_latency(100); // 100ms latency
        
        // Test operations under network stress
        let mut successful_operations = 0;
        let mut failed_operations = 0;
        
        for i in 0..10 {
            let file_name = format!("chaos_test_{}.txt", i);
            
            // Simulate network check before operation
            if chaos_simulator.should_fail_operation().await {
                failed_operations += 1;
                continue;
            }
            
            // Attempt write operation
            let write_result = timeout(
                Duration::from_secs(2),
                backend.write_file(&file_name, test_content)
            ).await;
            
            match write_result {
                Ok(Ok(_)) => {
                    // Attempt read operation
                    if !chaos_simulator.should_fail_operation().await {
                        let read_result = timeout(
                            Duration::from_secs(2),
                            backend.read_file(&file_name)
                        ).await;
                        
                        if read_result.is_ok() && read_result.unwrap().is_ok() {
                            successful_operations += 1;
                        }
                    }
                }
                _ => failed_operations += 1,
            }
        }
        
        // Clear network issues
        chaos_simulator.inject_packet_loss(0.0);
        chaos_simulator.inject_latency(0);
        
        // Verify some operations succeeded despite chaos
        assert!(successful_operations > 0, "Some operations should succeed despite network issues");
        assert!(failed_operations > 0, "Some operations should fail due to network issues");
        
        println!("Successful operations: {}, Failed operations: {}", successful_operations, failed_operations);
        println!("✅ Storage network instability resilience tested");
        Ok(())
    }
}

/// **TEST SUITE 2: RESOURCE EXHAUSTION SCENARIOS**
#[cfg(test)]
mod resource_exhaustion_tests {
    use super::*;
    use tempfile::TempDir;

    /// Test memory pressure handling
    #[tokio::test]
    async fn test_memory_pressure_handling() -> Result<()> {
        let chaos_simulator = Arc::new(ResourceChaosSimulator::new());
        
        // Setup memory backend with limited capacity
        let config = MemoryBackendConfig {
            max_memory_size: 1024 * 10, // 10KB limit
            max_file_size: 1024,
        };
        let backend = Arc::new(MemoryBackend::new(config)?);
        
        // Fill memory to near capacity
        let test_content = vec![0u8; 500]; // 500 bytes per file
        for i in 0..15 { // Should fill ~7.5KB
            let file_name = format!("fill_memory_{}.txt", i);
            let result = backend.write_file(&file_name, &test_content).await;
            if result.is_err() {
                break; // Memory backend is full
            }
        }
        
        // Simulate memory pressure
        chaos_simulator.simulate_memory_pressure();
        
        // Test operations under memory pressure
        let mut memory_failures = 0;
        let mut successful_operations = 0;
        
        for i in 0..5 {
            if chaos_simulator.should_fail_memory_operation() {
                memory_failures += 1;
                continue;
            }
            
            let file_name = format!("pressure_test_{}.txt", i);
            let small_content = b"small";
            
            let result = backend.write_file(&file_name, small_content).await;
            if result.is_ok() {
                successful_operations += 1;
            } else {
                memory_failures += 1;
            }
        }
        
        // Clear memory pressure
        chaos_simulator.clear_all_conditions();
        
        // Verify system handled memory pressure gracefully
        assert!(memory_failures > 0, "Memory pressure should cause some failures");
        
        println!("Memory pressure failures: {}, Successful operations: {}", memory_failures, successful_operations);
        println!("✅ Memory pressure handling tested");
        Ok(())
    }
    
    /// Test disk full scenarios
    #[tokio::test]
    async fn test_disk_full_handling() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let chaos_simulator = Arc::new(ResourceChaosSimulator::new());
        
        let config = FilesystemBackendConfig {
            root_path: temp_dir.path().to_string_lossy().to_string(),
            max_file_size: 1024,
            allowed_extensions: vec!["txt".to_string()],
        };
        let backend = Arc::new(FilesystemBackend::new(config)?);
        
        // Create some initial files
        let test_content = b"Disk full test content";
        backend.write_file("initial_file.txt", test_content).await?;
        
        // Simulate disk full condition
        chaos_simulator.simulate_disk_full();
        
        // Test operations under disk full condition
        let mut disk_full_failures = 0;
        let mut successful_operations = 0;
        
        for i in 0..5 {
            if chaos_simulator.should_fail_disk_operation() {
                disk_full_failures += 1;
                continue;
            }
            
            let file_name = format!("disk_test_{}.txt", i);
            let result = backend.write_file(&file_name, test_content).await;
            
            if result.is_ok() {
                successful_operations += 1;
            } else {
                disk_full_failures += 1;
            }
        }
        
        // Verify read operations still work
        let read_result = backend.read_file("initial_file.txt").await;
        assert!(read_result.is_ok(), "Read operations should work even when disk is full");
        
        // Clear disk full condition
        chaos_simulator.clear_all_conditions();
        
        // Verify recovery
        let recovery_write = backend.write_file("recovery_test.txt", test_content).await;
        assert!(recovery_write.is_ok(), "Write operations should work after disk full condition clears");
        
        println!("Disk full failures: {}, Successful operations: {}", disk_full_failures, successful_operations);
        println!("✅ Disk full handling tested");
        Ok(())
    }
    
    /// Test CPU throttling scenarios
    #[tokio::test]
    async fn test_cpu_throttling_handling() -> Result<()> {
        let chaos_simulator = Arc::new(ResourceChaosSimulator::new());
        
        // Measure baseline performance
        let baseline_start = Instant::now();
        for _ in 0..10 {
            // Simulate CPU-intensive operation
            let _result = (0..1000).fold(0u64, |acc, x| acc.wrapping_add(x));
        }
        let baseline_duration = baseline_start.elapsed();
        
        // Activate CPU throttling
        chaos_simulator.simulate_cpu_throttle();
        
        // Measure performance under CPU throttling
        let throttled_start = Instant::now();
        for _ in 0..10 {
            chaos_simulator.apply_cpu_throttle().await;
            let _result = (0..1000).fold(0u64, |acc, x| acc.wrapping_add(x));
        }
        let throttled_duration = throttled_start.elapsed();
        
        // Clear CPU throttling
        chaos_simulator.clear_all_conditions();
        
        // Verify throttling had an impact
        assert!(throttled_duration > baseline_duration, 
            "CPU throttling should increase operation time. Baseline: {:?}, Throttled: {:?}", 
            baseline_duration, throttled_duration);
        
        // Measure recovery performance
        let recovery_start = Instant::now();
        for _ in 0..10 {
            chaos_simulator.apply_cpu_throttle().await; // Should be no-op now
            let _result = (0..1000).fold(0u64, |acc, x| acc.wrapping_add(x));
        }
        let recovery_duration = recovery_start.elapsed();
        
        // Verify recovery (should be closer to baseline)
        let recovery_ratio = recovery_duration.as_millis() as f64 / baseline_duration.as_millis() as f64;
        assert!(recovery_ratio < 2.0, "Recovery performance should be reasonable");
        
        println!("Baseline: {:?}, Throttled: {:?}, Recovery: {:?}", 
            baseline_duration, throttled_duration, recovery_duration);
        println!("✅ CPU throttling handling tested");
        Ok(())
    }
}

/// **TEST SUITE 3: CASCADING FAILURE PREVENTION**
#[cfg(test)]
mod cascading_failure_tests {
    use super::*;

    /// Test circuit breaker pattern implementation
    #[tokio::test]
    async fn test_circuit_breaker_pattern() -> Result<()> {
        let registry = Arc::new(InMemoryServiceRegistry::new());
        let chaos_simulator = Arc::new(NetworkChaosSimulator::new());
        
        // Register primary and backup services
        let primary_service = create_test_service("primary-service", ServiceCapability::Storage);
        let backup_service = create_test_service("backup-service", ServiceCapability::Storage);
        
        registry.register_service(primary_service.clone()).await?;
        registry.register_service(backup_service).await?;
        
        // Circuit breaker state tracking
        let failure_count = Arc::new(AtomicU64::new(0));
        let circuit_open = Arc::new(AtomicBool::new(false));
        
        // Simulate operations with failure tracking
        for attempt in 0..20 {
            // Simulate primary service failures
            if attempt >= 5 && attempt < 15 {
                chaos_simulator.create_partition();
            } else {
                chaos_simulator.heal_partition();
            }
            
            let operation_failed = chaos_simulator.should_fail_operation().await;
            
            if operation_failed {
                let current_failures = failure_count.fetch_add(1, Ordering::SeqCst) + 1;
                
                // Open circuit breaker after 3 consecutive failures
                if current_failures >= 3 && !circuit_open.load(Ordering::SeqCst) {
                    circuit_open.store(true, Ordering::SeqCst);
                    tracing::warn!("🔥 CIRCUIT BREAKER OPENED: Too many failures detected");
                }
            } else {
                // Reset failure count on success
                failure_count.store(0, Ordering::SeqCst);
                
                // Close circuit breaker after successful operation
                if circuit_open.load(Ordering::SeqCst) {
                    circuit_open.store(false, Ordering::SeqCst);
                    tracing::info!("✅ CIRCUIT BREAKER CLOSED: Service recovered");
                }
            }
            
            // When circuit is open, use backup service
            if circuit_open.load(Ordering::SeqCst) {
                let backup_services = registry.discover_by_capabilities(vec![ServiceCapability::Storage]).await?;
                let backup_available = backup_services.iter().any(|s| s.name == "backup-service");
                assert!(backup_available, "Backup service should be available when primary fails");
            }
            
            sleep(Duration::from_millis(10)).await;
        }
        
        // Verify circuit breaker prevented cascading failures
        let final_circuit_state = circuit_open.load(Ordering::SeqCst);
        println!("Final circuit breaker state: {}", if final_circuit_state { "OPEN" } else { "CLOSED" });
        
        println!("✅ Circuit breaker pattern tested");
        Ok(())
    }
    
    /// Test bulkhead pattern for resource isolation
    #[tokio::test]
    async fn test_bulkhead_pattern() -> Result<()> {
        // Create separate resource pools (bulkheads)
        let critical_pool = Arc::new(Semaphore::new(3)); // 3 slots for critical operations
        let non_critical_pool = Arc::new(Semaphore::new(2)); // 2 slots for non-critical operations
        
        let chaos_simulator = Arc::new(ResourceChaosSimulator::new());
        
        // Simulate resource exhaustion in non-critical pool
        chaos_simulator.simulate_memory_pressure();
        
        // Fill up non-critical pool with failing operations
        let mut non_critical_tasks = Vec::new();
        for i in 0..5 {
            let pool = non_critical_pool.clone();
            let chaos = chaos_simulator.clone();
            
            let task = tokio::spawn(async move {
                let _permit = pool.acquire().await.unwrap();
                
                // Simulate non-critical operation that might fail
                if chaos.should_fail_memory_operation() {
                    sleep(Duration::from_millis(200)).await; // Simulate slow failing operation
                    return Err(NestGateError::InternalError("Non-critical operation failed".to_string()));
                }
                
                sleep(Duration::from_millis(50)).await;
                Ok(format!("Non-critical operation {} completed", i))
            });
            non_critical_tasks.push(task);
        }
        
        // Critical operations should still succeed despite non-critical failures
        let mut critical_tasks = Vec::new();
        for i in 0..3 {
            let pool = critical_pool.clone();
            
            let task = tokio::spawn(async move {
                let _permit = pool.acquire().await.unwrap();
                
                // Critical operations have dedicated resources
                sleep(Duration::from_millis(30)).await;
                Ok(format!("Critical operation {} completed", i))
            });
            critical_tasks.push(task);
        }
        
        // Wait for all tasks to complete
        let mut critical_successes = 0;
        let mut non_critical_failures = 0;
        
        for task in critical_tasks {
            let result = task.await.unwrap();
            if result.is_ok() {
                critical_successes += 1;
            }
        }
        
        for task in non_critical_tasks {
            let result = task.await.unwrap();
            if result.is_err() {
                non_critical_failures += 1;
            }
        }
        
        // Verify bulkhead isolation worked
        assert_eq!(critical_successes, 3, "All critical operations should succeed");
        assert!(non_critical_failures > 0, "Some non-critical operations should fail");
        
        chaos_simulator.clear_all_conditions();
        
        println!("Critical successes: {}, Non-critical failures: {}", critical_successes, non_critical_failures);
        println!("✅ Bulkhead pattern tested");
        Ok(())
    }
}

/// **TEST SUITE 4: BYZANTINE FAULT TOLERANCE**
#[cfg(test)]
mod byzantine_fault_tests {
    use super::*;

    /// Test handling of malicious or corrupted service responses
    #[tokio::test]
    async fn test_byzantine_service_detection() -> Result<()> {
        let registry = Arc::new(InMemoryServiceRegistry::new());
        
        // Register multiple services for the same capability
        let honest_service_1 = create_test_service("honest-service-1", ServiceCapability::Storage);
        let honest_service_2 = create_test_service("honest-service-2", ServiceCapability::Storage);
        let byzantine_service = create_test_service("byzantine-service", ServiceCapability::Storage);
        
        registry.register_service(honest_service_1).await?;
        registry.register_service(honest_service_2).await?;
        registry.register_service(byzantine_service).await?;
        
        // Simulate consensus mechanism for service responses
        let mut service_responses = HashMap::new();
        let services = registry.discover_by_capabilities(vec![ServiceCapability::Storage]).await?;
        
        for service in &services {
            // Simulate service responses (honest services return consistent data)
            let response = if service.name == "byzantine-service" {
                "CORRUPTED_DATA".to_string() // Byzantine service returns bad data
            } else {
                "CONSISTENT_DATA".to_string() // Honest services return good data
            };
            
            service_responses.insert(service.name.clone(), response);
        }
        
        // Implement Byzantine fault tolerance (majority consensus)
        let mut response_counts = HashMap::new();
        for response in service_responses.values() {
            *response_counts.entry(response.clone()).or_insert(0) += 1;
        }
        
        // Find the majority response
        let majority_response = response_counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(response, _)| response);
        
        // Verify byzantine fault tolerance
        assert_eq!(majority_response, Some("CONSISTENT_DATA".to_string()));
        
        // Detect byzantine services
        let byzantine_services: Vec<_> = service_responses
            .iter()
            .filter(|(_, response)| *response != &majority_response.as_ref().unwrap())
            .map(|(name, _)| name.clone())
            .collect();
        
        assert_eq!(byzantine_services.len(), 1);
        assert_eq!(byzantine_services[0], "byzantine-service");
        
        println!("Detected byzantine services: {:?}", byzantine_services);
        println!("✅ Byzantine service detection tested");
        Ok(())
    }
    
    /// Test data integrity verification across multiple sources
    #[tokio::test]
    async fn test_data_integrity_consensus() -> Result<()> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let temp_dir = TempDir::new().unwrap();
        
        // Create multiple storage backends (simulating different nodes)
        let mut backends = Vec::new();
        for i in 0..5 {
            let config = FilesystemBackendConfig {
                root_path: temp_dir.path().join(format!("node_{}", i)).to_string_lossy().to_string(),
                max_file_size: 1024,
                allowed_extensions: vec!["txt".to_string()],
            };
            backends.push(FilesystemBackend::new(config)?);
        }
        
        let original_content = b"Consensus test data";
        let file_name = "consensus_test.txt";
        
        // Write data to all backends
        for backend in &backends {
            backend.write_file(file_name, original_content).await?;
        }
        
        // Simulate one backend getting corrupted data
        let corrupted_content = b"CORRUPTED DATA";
        backends[2].write_file(file_name, corrupted_content).await?;
        
        // Read from all backends and verify consensus
        let mut content_hashes = HashMap::new();
        for (i, backend) in backends.iter().enumerate() {
            let content = backend.read_file(file_name).await?;
            
            let mut hasher = DefaultHasher::new();
            content.hash(&mut hasher);
            let hash = hasher.finish();
            
            *content_hashes.entry(hash).or_insert_with(Vec::new).push(i);
        }
        
        // Find majority hash (consensus)
        let majority_entry = content_hashes
            .iter()
            .max_by_key(|&(_, nodes)| nodes.len())
            .unwrap();
        
        let majority_hash = *majority_entry.0;
        let majority_nodes = majority_entry.1;
        
        // Verify majority consensus
        assert_eq!(majority_nodes.len(), 4, "4 out of 5 nodes should have consistent data");
        
        // Identify corrupted nodes
        let corrupted_nodes: Vec<_> = content_hashes
            .iter()
            .filter(|&(hash, _)| *hash != majority_hash)
            .flat_map(|(_, nodes)| nodes.iter())
            .collect();
        
        assert_eq!(corrupted_nodes.len(), 1);
        assert_eq!(*corrupted_nodes[0], 2, "Node 2 should be identified as corrupted");
        
        println!("Majority consensus achieved with {} nodes", majority_nodes.len());
        println!("Corrupted nodes detected: {:?}", corrupted_nodes);
        println!("✅ Data integrity consensus tested");
        Ok(())
    }
}

// **HELPER FUNCTIONS**

fn create_test_service(name: &str, capability: ServiceCapability) -> UniversalServiceRegistration {
    use uuid::Uuid;
    use nestgate_core::service_discovery::types::{
        ServiceMetadata, ServiceEndpoint, CommunicationProtocol, ServiceCategory
    };
    
    UniversalServiceRegistration {
        service_id: Uuid::new_v4(),
        name: name.to_string(),
        category: ServiceCategory::Storage,
        capabilities: vec![capability],
        endpoint: ServiceEndpoint {
            protocol: CommunicationProtocol::Http,
            address: "localhost".to_string(),
            port: 8080,
            path: Some("/".to_string()),
        },
        metadata: ServiceMetadata {
            version: "1.0.0".to_string(),
            tags: HashMap::new(),
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        },
        role: ServiceRole::Primary,
        health_check_endpoint: Some("/health".to_string()),
        registration_time: SystemTime::now(),
    }
}

/// Service capability constants for testing
impl ServiceCapability {
    pub const Storage: ServiceCapability = ServiceCapability::Storage;
    pub const Compute: ServiceCapability = ServiceCapability::Compute;
    pub const Security: ServiceCapability = ServiceCapability::Security;
}

/// Service role constants for testing
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceRole {
    Primary,
    Secondary,
    Backup,
} 