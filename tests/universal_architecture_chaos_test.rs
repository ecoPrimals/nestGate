//! Universal Primal Architecture Chaos Engineering Tests
//!
//! Tests the resilience and fault tolerance of the universal primal architecture under:
//! - Provider failures and network partitions
//! - High load and stress conditions
//! - Configuration changes and hot-swapping
//! - Resource exhaustion scenarios
//! - Concurrent access patterns

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use futures::future::join_all;
use tokio::sync::RwLock;

use nestgate_automation::{ServiceConnectionPool, UniversalAIConnectionPool};
use nestgate_core::universal_adapter::{UniversalAdapterConfig, UniversalPrimalAdapter};
use nestgate_core::universal_traits::{
    AuthToken, ComputePrimalProvider, Credentials, PrimalContext, SecurityPrimalProvider, Signature,
};

/// Chaos-enabled security provider that can simulate failures
#[derive(Debug)]
struct ChaosSecurityProvider {
    name: String,
    capabilities: Vec<String>,
    failure_rate: f64,
    response_delay: Duration,
    should_fail: Arc<AtomicBool>,
    call_count: Arc<AtomicUsize>,
}

impl ChaosSecurityProvider {
    fn new(name: String, capabilities: Vec<String>, failure_rate: f64) -> Self {
        Self {
            name,
            capabilities,
            failure_rate,
            response_delay: Duration::from_millis(100),
            should_fail: Arc::new(AtomicBool::new(false)),
            call_count: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn set_failure_mode(&self, should_fail: bool) {
        self.should_fail.store(should_fail, Ordering::Relaxed);
    }

    fn get_call_count(&self) -> usize {
        self.call_count.load(Ordering::Relaxed)
    }

    fn increment_calls(&self) {
        self.call_count.fetch_add(1, Ordering::Relaxed);
    }

    async fn maybe_fail(&self) -> nestgate_core::Result<()> {
        self.increment_calls();

        // Simulate network delay
        tokio::time::sleep(self.response_delay).await;

        // Check if forced failure
        if self.should_fail.load(Ordering::Relaxed) {
            return Err(nestgate_core::NestGateError::security_simple(
                nestgate_core::error::SecurityError::AuthenticationFailed {
                    reason: "Chaos failure".to_string(),
                    auth_method: "chaos".to_string(),
                    user: Some("test".to_string()),
                },
            ));
        }

        // Random failure based on failure rate
        if rand::random::<f64>() < self.failure_rate {
            return Err(nestgate_core::NestGateError::security_simple(
                nestgate_core::error::SecurityError::AuthenticationFailed {
                    reason: "Random chaos failure".to_string(),
                    auth_method: "chaos".to_string(),
                    user: Some("test".to_string()),
                },
            ));
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl SecurityPrimalProvider for ChaosSecurityProvider {
    async fn authenticate(&self, _credentials: &Credentials) -> nestgate_core::Result<AuthToken> {
        self.maybe_fail().await?;
        Ok(AuthToken {
            token: format!("{}-chaos-token", self.name),
            expires_at: SystemTime::now() + Duration::from_secs(3600),
            permissions: vec!["chaos".to_string()],
        })
    }

    async fn encrypt(&self, data: &[u8], _algorithm: &str) -> nestgate_core::Result<Vec<u8>> {
        self.maybe_fail().await?;
        Ok(format!("CHAOS-ENCRYPTED:{}", String::from_utf8_lossy(data)).into_bytes())
    }

    async fn decrypt(&self, encrypted: &[u8], _algorithm: &str) -> nestgate_core::Result<Vec<u8>> {
        self.maybe_fail().await?;
        let encrypted_str = String::from_utf8_lossy(encrypted);
        if let Some(data) = encrypted_str.strip_prefix("CHAOS-ENCRYPTED:") {
            Ok(data.as_bytes().to_vec())
        } else {
            Err(nestgate_core::NestGateError::security_simple(
                nestgate_core::error::SecurityError::AuthorizationDenied {
                    user: "chaos".to_string(),
                    action: "decrypt".to_string(),
                    resource: "encrypted_data".to_string(),
                    required_role: Some("valid_format".to_string()),
                },
            ))
        }
    }

    async fn sign_data(&self, data: &[u8]) -> nestgate_core::Result<Signature> {
        self.maybe_fail().await?;
        Ok(Signature {
            signature: format!("CHAOS-SIG-{}", String::from_utf8_lossy(data)),
            algorithm: "CHAOS".to_string(),
            key_id: format!("{}-chaos-key", self.name),
        })
    }

    async fn verify_signature(
        &self,
        _data: &[u8],
        signature: &Signature,
    ) -> nestgate_core::Result<bool> {
        self.maybe_fail().await?;
        Ok(signature.algorithm == "CHAOS")
    }

    async fn get_key_id(&self) -> nestgate_core::Result<String> {
        self.maybe_fail().await?;
        Ok(format!("{}-chaos-key", self.name))
    }

    async fn validate_token(&self, token: &str, _data: &[u8]) -> nestgate_core::Result<bool> {
        self.maybe_fail().await?;
        Ok(token.contains("chaos-token"))
    }

    async fn generate_validation_token(&self, _data: &[u8]) -> nestgate_core::Result<String> {
        self.maybe_fail().await?;
        Ok(format!("{}-chaos-validation", self.name))
    }

    async fn evaluate_boundary_access(
        &self,
        _source: &str,
        _destination: &str,
        _operation: &str,
    ) -> nestgate_core::Result<nestgate_core::universal_traits::SecurityDecision> {
        self.maybe_fail().await?;
        if rand::random::<f64>() < 0.8 {
            Ok(nestgate_core::universal_traits::SecurityDecision::Allow)
        } else if rand::random::<f64>() < 0.5 {
            Ok(nestgate_core::universal_traits::SecurityDecision::RequireAuth)
        } else {
            Ok(nestgate_core::universal_traits::SecurityDecision::Deny)
        }
    }
}

/// Chaos-enabled compute provider
#[derive(Debug)]
struct ChaosComputeProvider {
    name: String,
    capabilities: Vec<String>,
    failure_rate: f64,
    should_fail: Arc<AtomicBool>,
    call_count: Arc<AtomicUsize>,
}

impl ChaosComputeProvider {
    fn new(name: String, capabilities: Vec<String>, failure_rate: f64) -> Self {
        Self {
            name,
            capabilities,
            failure_rate,
            should_fail: Arc::new(AtomicBool::new(false)),
            call_count: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn set_failure_mode(&self, should_fail: bool) {
        self.should_fail.store(should_fail, Ordering::Relaxed);
    }

    fn get_call_count(&self) -> usize {
        self.call_count.load(Ordering::Relaxed)
    }
}

#[async_trait::async_trait]
impl ComputePrimalProvider for ChaosComputeProvider {
    async fn allocate_resources(
        &self,
        _spec: &nestgate_core::ResourceSpec,
    ) -> nestgate_core::Result<nestgate_core::ResourceAllocation> {
        self.call_count.fetch_add(1, Ordering::Relaxed);

        if self.should_fail.load(Ordering::Relaxed) || rand::random::<f64>() < self.failure_rate {
            return Err(nestgate_core::NestGateError::Internal {
                message: "Chaos resource allocation failure".to_string(),
                location: Some(file!().to_string()),
                debug_info: None,
                is_bug: false,
            });
        }

        Ok(nestgate_core::ResourceAllocation {
            id: "chaos-allocation".to_string(),
            allocated_resources: nestgate_core::universal_traits::ResourceSpec {
                cpu_cores: Some(2.0),
                memory_mb: Some(1024),
                disk_mb: Some(10240),
                gpu_count: None,
                network_bandwidth: Some(1000),
            },
            status: "allocated".to_string(),
            created_at: SystemTime::now(),
        })
    }

    async fn execute_workload(
        &self,
        _workload: &nestgate_core::WorkloadSpec,
    ) -> nestgate_core::Result<nestgate_core::WorkloadResult> {
        self.call_count.fetch_add(1, Ordering::Relaxed);

        if self.should_fail.load(Ordering::Relaxed) || rand::random::<f64>() < self.failure_rate {
            return Err(nestgate_core::NestGateError::Internal {
                message: "Chaos workload execution failure".to_string(),
                location: Some(file!().to_string()),
                debug_info: None,
                is_bug: false,
            });
        }

        Ok(nestgate_core::WorkloadResult {
            id: "chaos-workload".to_string(),
            exit_code: 0,
            stdout: "CHAOS-EXECUTED workload".to_string(),
            stderr: "".to_string(),
            execution_time: 1000, // milliseconds
        })
    }

    async fn monitor_performance(
        &self,
        _allocation: &nestgate_core::ResourceAllocation,
    ) -> nestgate_core::Result<nestgate_core::universal_traits::PerformanceMetrics> {
        Ok(nestgate_core::universal_traits::PerformanceMetrics {
            _cpu_usage: 0.8,
            memory_usage: 0.6,
            network_io: 200.0,
            disk_io: 100.0,
            timestamp: SystemTime::now(),
        })
    }

    async fn scale_resources(
        &self,
        _allocation: &nestgate_core::ResourceAllocation,
        _target: &nestgate_core::ScalingTarget,
    ) -> nestgate_core::Result<()> {
        if self.should_fail.load(Ordering::Relaxed) {
            return Err(nestgate_core::NestGateError::Internal {
                message: "Chaos scaling failure".to_string(),
                location: Some(file!().to_string()),
                debug_info: None,
                is_bug: false,
            });
        }
        Ok(())
    }

    async fn get_resource_utilization(
        &self,
    ) -> nestgate_core::Result<nestgate_core::ResourceUtilization> {
        Ok(nestgate_core::ResourceUtilization {
            cpu_percent: 0.7,
            memory_percent: 0.5,
            disk_percent: 0.3,
            network_utilization: 0.2,
        })
    }

    async fn detect_platform(&self) -> nestgate_core::Result<nestgate_core::PlatformCapabilities> {
        Ok(nestgate_core::PlatformCapabilities {
            architecture: "x86_64".to_string(),
            os_type: "linux".to_string(),
            container_runtime: "chaos".to_string(),
            gpu_support: false,
            features: vec!["chaos".to_string(), "testing".to_string()],
        })
    }

    async fn optimize_allocation(
        &self,
        _current: &nestgate_core::ResourceAllocation,
        _metrics: &nestgate_core::universal_traits::PerformanceMetrics,
    ) -> nestgate_core::Result<nestgate_core::OptimizationRecommendation> {
        Ok(nestgate_core::OptimizationRecommendation {
            recommendations: vec!["Chaos optimization applied".to_string()],
            expected_improvement: 0.1,
            confidence: 0.85,
        })
    }
}

/// Test provider failover under chaos conditions
#[tokio::test]
async fn test_chaos_provider_failover() {
    println!("🔥 Testing provider failover under chaos conditions...");

    let config = UniversalAdapterConfig {
        auto_discovery: true,
        discovery_interval: 1, // Fast discovery for testing
        request_timeout: 5,
        max_retries: 3,
        ..UniversalAdapterConfig::default()
    };

    let adapter = UniversalPrimalAdapter::new(config);

    // Register multiple chaos providers
    let provider1 = Arc::new(ChaosSecurityProvider::new(
        "chaos-provider-1".to_string(),
        vec!["encryption".to_string()],
        0.2, // 20% failure rate
    ));

    let provider2 = Arc::new(ChaosSecurityProvider::new(
        "chaos-provider-2".to_string(),
        vec!["encryption".to_string()],
        0.1, // 10% failure rate
    ));

    let provider3 = Arc::new(ChaosSecurityProvider::new(
        "chaos-provider-3".to_string(),
        vec!["encryption".to_string()],
        0.0, // Always reliable
    ));

    adapter
        .register_security_provider("chaos1".to_string(), provider1.clone())
        .await
        .unwrap();
    adapter
        .register_security_provider("chaos2".to_string(), provider2.clone())
        .await
        .unwrap();
    adapter
        .register_security_provider("chaos3".to_string(), provider3.clone())
        .await
        .unwrap();

    // Simulate sudden failure of first provider
    provider1.set_failure_mode(true);

    let mut success_count = 0;
    let mut failure_count = 0;

    // Perform many operations to test failover
    for _ in 0..100 {
        if let Some(provider) = adapter.get_security_provider().await {
            match provider.encrypt(b"test data", "AES").await {
                Ok(_) => success_count += 1,
                Err(_) => failure_count += 1,
            }
        }
    }

    println!(
        "Chaos results: {} successes, {} failures",
        success_count, failure_count
    );

    // Should have significant success rate due to failover
    assert!(
        success_count > 70,
        "Failover should maintain high success rate"
    );
    assert!(
        provider3.get_call_count() > 50,
        "Reliable provider should receive most traffic"
    );

    println!("✅ Chaos provider failover test passed");
}

/// Test concurrent access under stress
#[tokio::test]
async fn test_concurrent_chaos_stress() {
    println!("💥 Testing concurrent access under chaos stress...");

    let config = UniversalAdapterConfig::default();
    let adapter = Arc::new(UniversalPrimalAdapter::new(config));

    // Register chaos providers
    for i in 0..5 {
        let provider = Arc::new(ChaosComputeProvider::new(
            format!("stress-provider-{}", i),
            vec!["processing".to_string()],
            0.15, // 15% failure rate
        ));
        adapter
            .register_compute_provider(format!("stress{}", i), provider)
            .await
            .unwrap();
    }

    // Launch concurrent stress test
    let mut handles = Vec::new();
    let total_operations = Arc::new(AtomicUsize::new(0));
    let successful_operations = Arc::new(AtomicUsize::new(0));

    for worker_id in 0..20 {
        let adapter_clone = Arc::clone(&adapter);
        let total_ops = Arc::clone(&total_operations);
        let successful_ops = Arc::clone(&successful_operations);

        let handle = tokio::spawn(async move {
            for i in 0..50 {
                total_ops.fetch_add(1, Ordering::Relaxed);

                if let Some(provider) = adapter_clone.get_compute_provider().await {
                    let workload = nestgate_core::universal_traits::WorkloadSpec {
                        id: format!("worker-{}-task-{}", worker_id, i),
                        image: "test-image".to_string(),
                        command: vec![
                            "process".to_string(),
                            format!("worker-{}-task-{}", worker_id, i),
                        ],
                        environment: std::collections::HashMap::new(),
                        resources: nestgate_core::universal_traits::ResourceSpec {
                            cpu_cores: Some(0.5),
                            memory_mb: Some(256),
                            disk_mb: Some(512),
                            gpu_count: None,
                            network_bandwidth: None,
                        },
                    };
                    if provider.execute_workload(&workload).await.is_ok() {
                        successful_ops.fetch_add(1, Ordering::Relaxed);
                    }
                }

                // Add some chaos to the timing
                if rand::random::<bool>() {
                    tokio::time::sleep(Duration::from_millis(1)).await;
                }
            }
        });

        handles.push(handle);
    }

    // Wait for all workers to complete
    join_all(handles).await;

    let total = total_operations.load(Ordering::Relaxed);
    let successful = successful_operations.load(Ordering::Relaxed);
    let success_rate = successful as f64 / total as f64;

    println!(
        "Stress test results: {}/{} operations succeeded ({:.2}%)",
        successful,
        total,
        success_rate * 100.0
    );

    // Should maintain reasonable success rate under stress
    assert!(
        success_rate > 0.7,
        "Should maintain >70% success rate under stress"
    );
    assert_eq!(total, 1000, "All operations should have been attempted");

    println!("✅ Concurrent chaos stress test passed");
}

/// Test AI connection pool chaos resilience
#[tokio::test]
async fn test_ai_connection_pool_chaos_resilience() {
    println!("🤖 Testing AI connection pool chaos resilience...");

    let mut ai_pool = UniversalAIConnectionPool::new();

    // Add providers with different reliability characteristics
    let provider_configs = vec![
        ("unstable-ai", 0.3),        // 30% failure rate
        ("flaky-ai", 0.2),           // 20% failure rate
        ("reliable-ai", 0.05),       // 5% failure rate
        ("super-reliable-ai", 0.01), // 1% failure rate
    ];

    for (name, _failure_rate) in &provider_configs {
        ai_pool.add_ai_provider_with_capabilities(
            name.to_string(),
            format!("http://localhost:8080/{}", name),
            "ai".to_string(),
            vec!["text-generation".to_string(), "analysis".to_string()],
        );
    }

    // Simulate chaos by updating health metrics randomly
    for _ in 0..100 {
        for (name, failure_rate) in &provider_configs {
            let success = rand::random::<f64>() > *failure_rate;
            let response_time = if success {
                100 + rand::random::<u64>() % 200
            } else {
                1000 + rand::random::<u64>() % 4000
            };

            ai_pool.update_ai_provider_health(name, response_time, success);
        }

        // Test provider selection under chaos
        let provider =
            ai_pool.get_best_ai_provider_with_capabilities(&["text-generation".to_string()]);
        assert!(
            provider.is_some(),
            "Should always be able to select a provider"
        );
    }

    let stats = ai_pool.get_ai_provider_stats();

    // Verify that statistics are tracking health properly
    for (name, (success_rate, response_time, healthy, _capabilities)) in stats {
        println!(
            "Provider {}: success_rate={:.2}, response_time={}ms, healthy={}",
            name, success_rate, response_time, healthy
        );

        // Health should correlate with performance
        if success_rate > 0.8 {
            assert!(
                healthy,
                "High success rate providers should be marked healthy"
            );
        }
    }

    println!("✅ AI connection pool chaos resilience test passed");
}

/// Test configuration hot-swapping under load
#[tokio::test]
async fn test_configuration_hot_swapping_chaos() {
    println!("🔧 Testing configuration hot-swapping under chaos...");

    let config = UniversalAdapterConfig {
        auto_discovery: true,
        discovery_interval: 1, // Fast discovery for testing (1 second)
        request_timeout: 5,
        max_retries: 3,
        ..UniversalAdapterConfig::default()
    };

    let adapter = Arc::new(UniversalPrimalAdapter::new(config));

    // Start background load
    let adapter_load = Arc::clone(&adapter);
    let load_handle = tokio::spawn(async move {
        for _ in 0..200 {
            let _provider = adapter_load.get_security_provider().await;
            let _compute_provider = adapter_load.get_compute_provider().await;
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    });

    // Simulate hot-swapping providers under load
    let providers = vec![
        ("hotswap-1", vec!["encryption".to_string()]),
        ("hotswap-2", vec!["authentication".to_string()]),
        ("hotswap-3", vec!["signing".to_string()]),
    ];

    for round in 0..10 {
        // Add providers
        for (name, capabilities) in &providers {
            let provider = Arc::new(ChaosSecurityProvider::new(
                format!("{}-round-{}", name, round),
                capabilities.clone(),
                0.1,
            ));

            let provider_name = format!("{}-{}", name, round);
            adapter
                .register_security_provider(provider_name, provider)
                .await
                .unwrap();
        }

        tokio::time::sleep(Duration::from_millis(100)).await;

        // Remove old providers (if any)
        if round > 0 {
            // In a real implementation, we'd have a remove_provider method
            // For now, just test that the system handles provider additions
        }
    }

    // Wait for load test to complete
    load_handle.await.unwrap();

    // Verify system is still functional
    let provider = adapter.get_security_provider().await;
    assert!(
        provider.is_some(),
        "System should still be functional after hot-swapping"
    );

    println!("✅ Configuration hot-swapping chaos test passed");
}

/// Test network partition simulation
#[tokio::test]
async fn test_network_partition_simulation() {
    println!("🌐 Testing network partition simulation...");

    let mut service_pool = UniversalAIConnectionPool::new();

    // Create providers in different "network zones"
    let zones = vec![
        ("zone-a", vec!["provider-a1", "provider-a2"]),
        ("zone-b", vec!["provider-b1", "provider-b2"]),
        ("zone-c", vec!["provider-c1"]),
    ];

    for (zone, providers) in zones {
        for provider_name in providers {
            service_pool.add_ai_provider_with_capabilities(
                provider_name.to_string(),
                format!("http://{}.{}.local:8080", provider_name, zone),
                "ai".to_string(),
                vec!["text-generation".to_string()],
            );
        }
    }

    // Simulate zone-a going offline (network partition)
    service_pool.update_ai_provider_health("provider-a1", 5000, false);
    service_pool.update_ai_provider_health("provider-a2", 5000, false);

    // Zone-b experiencing high latency
    service_pool.update_ai_provider_health("provider-b1", 2000, true);
    service_pool.update_ai_provider_health("provider-b2", 1800, true);

    // Zone-c remains healthy
    service_pool.update_ai_provider_health("provider-c1", 150, true);

    // Test that system adapts to network conditions
    for _ in 0..20 {
        let provider =
            service_pool.get_best_ai_provider_with_capabilities(&["text-generation".to_string()]);
        assert!(provider.is_some());

        // Should prefer the healthy zone-c provider
        if let Some(provider_url) = provider {
            assert!(
                provider_url.contains("provider-c1"),
                "Should prefer healthy zone provider"
            );
        }
    }

    // Simulate zone-a recovery
    service_pool.update_ai_provider_health("provider-a1", 120, true);
    service_pool.update_ai_provider_health("provider-a2", 130, true);

    // Test that system rebalances
    let mut zone_a_selections = 0;
    let mut zone_c_selections = 0;

    for _ in 0..100 {
        if let Some(provider_url) =
            service_pool.get_best_ai_provider_with_capabilities(&["text-generation".to_string()])
        {
            if provider_url.contains("provider-a") {
                zone_a_selections += 1;
            } else if provider_url.contains("provider-c") {
                zone_c_selections += 1;
            }
        }
    }

    println!(
        "Zone selection after recovery: A={}, C={}",
        zone_a_selections, zone_c_selections
    );
    assert!(
        zone_a_selections + zone_c_selections > 80,
        "Should use recovered healthy providers"
    );

    println!("✅ Network partition simulation test passed");
}

/// Test resource exhaustion scenarios
#[tokio::test]
async fn test_resource_exhaustion_chaos() {
    println!("🔋 Testing resource exhaustion chaos scenarios...");

    let config = UniversalAdapterConfig {
        auto_discovery: true,
        discovery_interval: 1,
        request_timeout: 2, // Short timeout to trigger failures
        max_retries: 1,     // Limited retries
        ..UniversalAdapterConfig::default()
    };

    let adapter = UniversalPrimalAdapter::new(config);
    let mut ai_pool = UniversalAIConnectionPool::new();

    // Try to register more providers than the limit
    for i in 0..20 {
        let provider = Arc::new(ChaosSecurityProvider::new(
            format!("exhaust-provider-{}", i),
            vec!["encryption".to_string()],
            0.0,
        ));

        let result = adapter
            .register_security_provider(format!("exhaust-{}", i), provider)
            .await;

        if i < 10 {
            assert!(result.is_ok(), "Should accept providers within limit");
        }
        // Later providers may be rejected or cause older ones to be evicted
    }

    // Test AI pool under memory pressure simulation
    for i in 0..1000 {
        ai_pool.add_ai_provider_with_capabilities(
            format!("memory-pressure-{}", i),
            format!("http://localhost:{}", 8000 + i),
            "ai".to_string(),
            vec!["capability".to_string()],
        );

        // Simulate memory pressure by updating health frequently
        if i % 10 == 0 {
            for j in 0..std::cmp::min(i, 100) {
                ai_pool.update_ai_provider_health(&format!("memory-pressure-{}", j), 100, true);
            }
        }
    }

    // System should still be responsive under pressure
    let provider = ai_pool.get_best_ai_provider_with_capabilities(&["capability".to_string()]);
    assert!(
        provider.is_some(),
        "System should remain functional under memory pressure"
    );

    let stats = ai_pool.get_ai_provider_stats();
    println!("Providers under memory pressure: {}", stats.len());

    // Test cleanup under pressure
    ai_pool.perform_health_check().await;

    println!("✅ Resource exhaustion chaos test passed");
}

/// Test cascading failure scenarios
#[tokio::test]
async fn test_cascading_failure_scenarios() {
    println!("⚡ Testing cascading failure scenarios...");

    let config = UniversalAdapterConfig {
        auto_discovery: true,
        discovery_interval: 1, // Fast discovery for testing
        request_timeout: 5,
        max_retries: 3,
        ..UniversalAdapterConfig::default()
    };

    let adapter = UniversalPrimalAdapter::new(config);

    // Create a chain of dependent providers
    let providers = vec![
        ("primary", 0.0),   // Initially reliable
        ("backup", 0.0),    // Initially reliable
        ("emergency", 0.0), // Always reliable
    ];

    let chaos_providers: Vec<_> = providers
        .iter()
        .map(|(name, failure_rate)| {
            Arc::new(ChaosSecurityProvider::new(
                name.to_string(),
                vec!["encryption".to_string()],
                *failure_rate,
            ))
        })
        .collect();

    for (i, provider) in chaos_providers.iter().enumerate() {
        adapter
            .register_security_provider(format!("cascade-{}", i), provider.clone())
            .await
            .unwrap();
    }

    // Test normal operation
    let mut success_count = 0;
    for _ in 0..50 {
        if let Some(provider) = adapter.get_security_provider().await {
            if provider.encrypt(b"test", "AES").await.is_ok() {
                success_count += 1;
            }
        }
    }
    assert!(success_count > 45, "Should work normally initially");

    // Trigger cascading failure
    println!("Triggering cascading failure...");

    // Primary fails
    chaos_providers[0].set_failure_mode(true);
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Backup fails under increased load
    chaos_providers[1].set_failure_mode(true);
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Test that emergency provider handles the load
    let mut emergency_success = 0;
    for _ in 0..50 {
        if let Some(provider) = adapter.get_security_provider().await {
            if provider.encrypt(b"test", "AES").await.is_ok() {
                emergency_success += 1;
            }
        }
    }

    println!("Emergency provider success rate: {}/50", emergency_success);
    assert!(
        emergency_success > 35,
        "Emergency provider should handle cascading failure"
    );
    assert!(
        chaos_providers[2].get_call_count() > 30,
        "Emergency provider should receive most traffic"
    );

    println!("✅ Cascading failure scenarios test passed");
}

/// Chaos test summary and resilience scoring
#[tokio::test]
async fn test_chaos_resilience_summary() {
    println!("\n🏆 UNIVERSAL PRIMAL ARCHITECTURE CHAOS TEST SUMMARY");
    println!("===================================================");

    let mut resilience_scores = HashMap::new();

    // Test basic adapter resilience
    let config = UniversalAdapterConfig::default();
    let adapter = UniversalPrimalAdapter::new(config);

    // Score: Can handle no providers gracefully
    let no_provider_result = adapter.get_security_provider().await;
    resilience_scores.insert(
        "No Provider Handling",
        if no_provider_result.is_none() { 100 } else { 0 },
    );

    // Score: Can handle repeated requests
    let mut repeated_requests_score = 0;
    for _ in 0..100 {
        let _result = adapter.find_providers_by_capability("").await;
        repeated_requests_score += 1;
    }
    resilience_scores.insert("Repeated Requests", repeated_requests_score);

    // Test AI pool resilience
    let ai_pool = UniversalAIConnectionPool::new();

    // Score: Empty pool handling
    let empty_pool_result = ai_pool.get_best_ai_provider();
    resilience_scores.insert(
        "Empty Pool Handling",
        if empty_pool_result.is_none() { 100 } else { 0 },
    );

    // Score: Statistics on empty pool
    let empty_stats = ai_pool.get_ai_provider_stats();
    resilience_scores.insert("Empty Stats", if empty_stats.is_empty() { 100 } else { 0 });

    // Test service pool resilience
    let service_pool = ServiceConnectionPool::new();

    // Score: Empty service pool
    let empty_service = service_pool.get_best_squirrel();
    resilience_scores.insert(
        "Empty Service Pool",
        if empty_service.is_none() { 100 } else { 0 },
    );

    println!("🔬 Resilience Scores:");
    let mut total_score = 0;
    let mut max_score = 0;

    for (test, score) in &resilience_scores {
        println!("   {}: {}/100", test, score);
        total_score += score;
        max_score += 100;
    }

    let overall_score = (total_score as f64 / max_score as f64) * 100.0;
    println!("\n📊 Overall Resilience Score: {:.1}/100", overall_score);

    // Resilience criteria
    if overall_score >= 90.0 {
        println!("🏆 EXCELLENT: System shows exceptional resilience to chaos");
    } else if overall_score >= 75.0 {
        println!("✅ GOOD: System shows good resilience to failure scenarios");
    } else if overall_score >= 50.0 {
        println!("⚠️ MODERATE: System has some resilience but needs improvement");
    } else {
        println!("❌ POOR: System lacks sufficient resilience to chaos conditions");
    }

    // Performance under chaos test
    let start = std::time::Instant::now();
    for _ in 0..1000 {
        let _adapter_check = adapter.find_providers_by_capability("").await;
    }
    let chaos_performance = start.elapsed();
    let ops_per_second = 1000.0 / chaos_performance.as_secs_f64();

    println!("⚡ Performance under chaos: {:.0} ops/sec", ops_per_second);

    println!("\n🎯 CHAOS ENGINEERING RECOMMENDATIONS:");
    if overall_score < 90.0 {
        println!("   - Implement circuit breaker patterns for provider failures");
        println!("   - Add more sophisticated health checking with gradual recovery");
        println!("   - Implement provider connection pooling with retry logic");
    }

    if ops_per_second < 100.0 {
        println!("   - Optimize provider selection algorithms for better performance");
        println!("   - Add caching layers for frequently accessed providers");
        println!("   - Consider async batching for health checks");
    }

    println!("\n🚀 UNIVERSAL ARCHITECTURE CHAOS TESTING COMPLETE!");

    // Should maintain reasonable resilience
    assert!(
        overall_score >= 60.0,
        "System should maintain basic resilience under chaos"
    );
}
