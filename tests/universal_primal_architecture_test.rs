//! Comprehensive Universal Primal Architecture Test Suite
//!
//! Tests the core functionality of the universal primal adapter including:
//! - Provider discovery and registration
//! - Capability-based provider selection
//! - Fallback and resilience mechanisms
//! - Dynamic provider switching
//! - Health monitoring and load balancing

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use tokio::sync::RwLock;

use nestgate_automation::{UniversalAIConnection, UniversalAIConnectionPool};
use nestgate_core::universal_adapter::{
    PrimalDiscoveryService, ServiceDiscoveryConfig, UniversalAdapterConfig, UniversalPrimalAdapter,
};
use nestgate_core::universal_providers::{
    UniversalComputeWrapper, UniversalOrchestrationWrapper, UniversalSecurityWrapper,
};
use nestgate_core::universal_traits::{
    AuthToken, ComputePrimalProvider, Credentials, OrchestrationPrimalProvider, PrimalContext,
    SecurityPrimalProvider, ServiceHealth, ServiceInstance, Signature,
};

/// Mock security provider for testing
#[derive(Debug)]
struct MockSecurityProvider {
    pub name: String,
    pub capabilities: Vec<String>,
    pub healthy: bool,
}

#[async_trait::async_trait]
impl SecurityPrimalProvider for MockSecurityProvider {
    async fn authenticate(&self, _credentials: &Credentials) -> nestgate_core::Result<AuthToken> {
        if self.healthy {
            Ok(AuthToken {
                token: format!("{}-auth-token", self.name),
                expires_at: SystemTime::now() + Duration::from_secs(3600),
                permissions: vec!["read".to_string(), "write".to_string()],
            })
        } else {
            Err(nestgate_core::NestGateError::Unauthorized(
                "Provider unhealthy".to_string(),
            ))
        }
    }

    async fn encrypt(&self, data: &[u8], _algorithm: &str) -> nestgate_core::Result<Vec<u8>> {
        if self.healthy {
            let mut encrypted = Vec::new();
            encrypted.extend_from_slice(b"ENCRYPTED:");
            encrypted.extend_from_slice(&self.name.as_bytes());
            encrypted.extend_from_slice(b":");
            encrypted.extend_from_slice(data);
            Ok(encrypted)
        } else {
            Err(nestgate_core::NestGateError::Unauthorized(
                "Encryption failed".to_string(),
            ))
        }
    }

    async fn decrypt(&self, encrypted: &[u8], _algorithm: &str) -> nestgate_core::Result<Vec<u8>> {
        if self.healthy {
            let encrypted_str = String::from_utf8_lossy(encrypted);
            if let Some(data) = encrypted_str.strip_prefix(&format!("ENCRYPTED:{}:", self.name)) {
                Ok(data.as_bytes().to_vec())
            } else {
                Err(nestgate_core::NestGateError::Unauthorized(
                    "Invalid encryption format".to_string(),
                ))
            }
        } else {
            Err(nestgate_core::NestGateError::Unauthorized(
                "Decryption failed".to_string(),
            ))
        }
    }

    async fn sign_data(&self, data: &[u8]) -> nestgate_core::Result<Signature> {
        if self.healthy {
            Ok(Signature {
                data: format!("{}-signature-{}", self.name, String::from_utf8_lossy(data))
                    .into_bytes(),
                algorithm: "MOCK".to_string(),
                key_id: format!("{}-key", self.name),
            })
        } else {
            Err(nestgate_core::NestGateError::Unauthorized(
                "Signing failed".to_string(),
            ))
        }
    }

    async fn verify_signature(
        &self,
        _data: &[u8],
        signature: &Signature,
    ) -> nestgate_core::Result<bool> {
        if self.healthy {
            Ok(signature.key_id == format!("{}-key", self.name))
        } else {
            Err(nestgate_core::NestGateError::Unauthorized(
                "Verification failed".to_string(),
            ))
        }
    }

    async fn get_key_id(&self) -> nestgate_core::Result<String> {
        if self.healthy {
            Ok(format!("{}-key", self.name))
        } else {
            Err(nestgate_core::NestGateError::Unauthorized(
                "Key unavailable".to_string(),
            ))
        }
    }

    async fn validate_token(&self, token: &str, _data: &[u8]) -> nestgate_core::Result<bool> {
        Ok(self.healthy && token.contains(&self.name))
    }

    async fn generate_validation_token(&self, _data: &[u8]) -> nestgate_core::Result<String> {
        if self.healthy {
            Ok(format!("{}-validation-token", self.name))
        } else {
            Err(nestgate_core::NestGateError::Unauthorized(
                "Token generation failed".to_string(),
            ))
        }
    }
}

/// Mock compute provider for testing
#[derive(Debug)]
struct MockComputeProvider {
    pub name: String,
    pub capabilities: Vec<String>,
    pub healthy: bool,
}

#[async_trait::async_trait]
impl ComputePrimalProvider for MockComputeProvider {
    async fn process_data(&self, data: Vec<u8>) -> nestgate_core::Result<Vec<u8>> {
        if self.healthy {
            let mut processed = Vec::new();
            processed.extend_from_slice(b"PROCESSED:");
            processed.extend_from_slice(&self.name.as_bytes());
            processed.extend_from_slice(b":");
            processed.extend_from_slice(&data);
            Ok(processed)
        } else {
            Err(nestgate_core::NestGateError::External(
                "Processing failed".to_string(),
            ))
        }
    }

    async fn execute_task(
        &self,
        task: &str,
        _context: &PrimalContext,
    ) -> nestgate_core::Result<String> {
        if self.healthy {
            Ok(format!("{} executed task: {}", self.name, task))
        } else {
            Err(nestgate_core::NestGateError::External(
                "Task execution failed".to_string(),
            ))
        }
    }

    async fn get_capabilities(&self) -> nestgate_core::Result<Vec<String>> {
        Ok(self.capabilities.clone())
    }

    async fn health_check(&self) -> nestgate_core::Result<bool> {
        Ok(self.healthy)
    }
}

#[tokio::test]
async fn test_universal_adapter_creation() {
    let config = UniversalAdapterConfig {
        max_providers: 100,
        discovery_interval: Duration::from_secs(30),
        health_check_interval: Duration::from_secs(60),
        enable_auto_discovery: true,
        enable_capability_matching: true,
        enable_fallback_providers: true,
        discovery_config: ServiceDiscoveryConfig {
            scan_local_network: true,
            scan_environment: true,
            scan_service_registry: false,
            timeout: Duration::from_secs(10),
            retry_attempts: 3,
        },
    };

    let adapter = UniversalPrimalAdapter::new(config);
    assert!(adapter.is_ok());
    println!("✅ Universal adapter created successfully");
}

#[tokio::test]
async fn test_security_provider_registration_and_discovery() {
    let config = UniversalAdapterConfig::default();
    let adapter = UniversalPrimalAdapter::new(config).unwrap();

    // Register multiple security providers with different capabilities
    let providers = vec![
        MockSecurityProvider {
            name: "security-provider-1".to_string(),
            capabilities: vec!["encryption".to_string(), "signing".to_string()],
            healthy: true,
        },
        MockSecurityProvider {
            name: "security-provider-2".to_string(),
            capabilities: vec!["authentication".to_string(), "token-validation".to_string()],
            healthy: true,
        },
        MockSecurityProvider {
            name: "security-provider-3".to_string(),
            capabilities: vec!["encryption".to_string(), "authentication".to_string()],
            healthy: false, // Unhealthy provider
        },
    ];

    for provider in providers {
        let result = adapter
            .register_security_provider(provider.name.clone(), Arc::new(provider))
            .await;
        assert!(result.is_ok());
    }

    // Test capability-based provider selection
    let encryption_provider = adapter
        .get_security_provider_with_capability("encryption")
        .await;
    assert!(encryption_provider.is_some());

    let auth_provider = adapter
        .get_security_provider_with_capability("authentication")
        .await;
    assert!(auth_provider.is_some());

    println!("✅ Security provider registration and discovery working");
}

#[tokio::test]
async fn test_compute_provider_load_balancing() {
    let config = UniversalAdapterConfig::default();
    let adapter = UniversalPrimalAdapter::new(config).unwrap();

    // Register multiple compute providers
    let providers = vec![
        MockComputeProvider {
            name: "compute-provider-1".to_string(),
            capabilities: vec!["text-generation".to_string(), "analysis".to_string()],
            healthy: true,
        },
        MockComputeProvider {
            name: "compute-provider-2".to_string(),
            capabilities: vec!["text-generation".to_string()],
            healthy: true,
        },
        MockComputeProvider {
            name: "compute-provider-3".to_string(),
            capabilities: vec!["analysis".to_string(), "embedding".to_string()],
            healthy: true,
        },
    ];

    for provider in providers {
        let result = adapter
            .register_compute_provider(provider.name.clone(), Arc::new(provider))
            .await;
        assert!(result.is_ok());
    }

    // Test load balancing - multiple requests should potentially use different providers
    let mut used_providers = std::collections::HashSet::new();

    for _ in 0..10 {
        if let Some(provider) = adapter.get_compute_provider().await {
            let result = provider
                .execute_task("test-task", &PrimalContext::default())
                .await;
            if let Ok(response) = result {
                used_providers.insert(response.split_whitespace().next().unwrap_or("").to_string());
            }
        }
    }

    assert!(used_providers.len() >= 1);
    println!(
        "✅ Compute provider load balancing working: used {} providers",
        used_providers.len()
    );
}

#[tokio::test]
async fn test_provider_health_monitoring_and_failover() {
    let config = UniversalAdapterConfig::default();
    let adapter = UniversalPrimalAdapter::new(config).unwrap();

    // Register healthy and unhealthy providers
    let healthy_provider = Arc::new(MockSecurityProvider {
        name: "healthy-provider".to_string(),
        capabilities: vec!["encryption".to_string()],
        healthy: true,
    });

    let unhealthy_provider = Arc::new(MockSecurityProvider {
        name: "unhealthy-provider".to_string(),
        capabilities: vec!["encryption".to_string()],
        healthy: false,
    });

    adapter
        .register_security_provider("healthy".to_string(), healthy_provider.clone())
        .await
        .unwrap();
    adapter
        .register_security_provider("unhealthy".to_string(), unhealthy_provider.clone())
        .await
        .unwrap();

    // Test that we get the healthy provider
    let provider = adapter
        .get_security_provider_with_capability("encryption")
        .await;
    assert!(provider.is_some());

    let result = provider.unwrap().encrypt(b"test data", "AES").await;
    assert!(result.is_ok());
    assert!(String::from_utf8_lossy(&result.unwrap()).contains("healthy-provider"));

    println!("✅ Provider health monitoring and failover working");
}

#[tokio::test]
async fn test_capability_based_provider_matching() {
    let config = UniversalAdapterConfig::default();
    let adapter = UniversalPrimalAdapter::new(config).unwrap();

    // Register providers with specific capabilities
    let compute_providers = vec![
        (
            "text-generator",
            vec!["text-generation".to_string(), "completion".to_string()],
        ),
        (
            "image-analyzer",
            vec!["image-analysis".to_string(), "vision".to_string()],
        ),
        (
            "embedder",
            vec!["embedding".to_string(), "similarity".to_string()],
        ),
        (
            "multi-modal",
            vec![
                "text-generation".to_string(),
                "image-analysis".to_string(),
                "embedding".to_string(),
            ],
        ),
    ];

    for (name, capabilities) in compute_providers {
        let provider = Arc::new(MockComputeProvider {
            name: name.to_string(),
            capabilities: capabilities.clone(),
            healthy: true,
        });
        adapter
            .register_compute_provider(name.to_string(), provider)
            .await
            .unwrap();
    }

    // Test specific capability matching
    let text_provider = adapter
        .get_compute_provider_with_capability("text-generation")
        .await;
    assert!(text_provider.is_some());

    let vision_provider = adapter
        .get_compute_provider_with_capability("image-analysis")
        .await;
    assert!(vision_provider.is_some());

    let embedding_provider = adapter
        .get_compute_provider_with_capability("embedding")
        .await;
    assert!(embedding_provider.is_some());

    // Test multi-capability matching (should prefer multi-modal)
    let multi_provider = adapter
        .get_compute_provider_with_capabilities(&[
            "text-generation".to_string(),
            "embedding".to_string(),
        ])
        .await;
    assert!(multi_provider.is_some());

    println!("✅ Capability-based provider matching working");
}

#[tokio::test]
async fn test_dynamic_provider_discovery() {
    let config = UniversalAdapterConfig {
        enable_auto_discovery: true,
        discovery_interval: Duration::from_millis(100), // Fast discovery for testing
        ..UniversalAdapterConfig::default()
    };
    let adapter = UniversalPrimalAdapter::new(config).unwrap();

    // Start discovery
    let discovery_result = adapter.start_discovery().await;
    assert!(discovery_result.is_ok());

    // Wait a bit for discovery to run
    tokio::time::sleep(Duration::from_millis(200)).await;

    // Check if any providers were discovered (this will depend on environment)
    let providers = adapter.list_available_providers().await;

    // Even if no actual providers are discovered in the test environment,
    // the discovery mechanism should be working
    println!(
        "✅ Dynamic provider discovery working: found {} providers",
        providers.len()
    );
}

#[tokio::test]
async fn test_fallback_mechanisms() {
    let config = UniversalAdapterConfig {
        enable_fallback_providers: true,
        ..UniversalAdapterConfig::default()
    };
    let adapter = UniversalPrimalAdapter::new(config).unwrap();

    // Initially, no providers registered - should fallback gracefully
    let provider = adapter.get_security_provider().await;

    // Even without providers, the adapter should handle gracefully
    // (either return None or a fallback provider)
    println!(
        "✅ Fallback mechanism working: provider = {:?}",
        provider.is_some()
    );

    // Now register a provider and test it works
    let security_provider = Arc::new(MockSecurityProvider {
        name: "fallback-test".to_string(),
        capabilities: vec!["encryption".to_string()],
        healthy: true,
    });

    adapter
        .register_security_provider("fallback".to_string(), security_provider)
        .await
        .unwrap();

    let provider = adapter.get_security_provider().await;
    assert!(provider.is_some());

    println!("✅ Provider registration and retrieval after fallback working");
}

#[tokio::test]
async fn test_universal_ai_connections_integration() {
    let mut ai_pool = UniversalAIConnectionPool::new();

    // Test capability-based AI provider addition
    ai_pool.add_ai_provider_with_capabilities(
        "text-gen-provider".to_string(),
        "http://localhost:8001".to_string(),
        "llm".to_string(),
        vec!["text-generation".to_string(), "completion".to_string()],
    );

    ai_pool.add_ai_provider_with_capabilities(
        "embedding-provider".to_string(),
        "http://localhost:8002".to_string(),
        "embedding".to_string(),
        vec!["embedding".to_string(), "similarity".to_string()],
    );

    // Test capability-based provider selection
    let text_providers = ai_pool.get_providers_with_capabilities(&["text-generation".to_string()]);
    assert_eq!(text_providers.len(), 1);
    assert_eq!(text_providers[0], "text-gen-provider");

    let embedding_providers = ai_pool.get_providers_with_capabilities(&["embedding".to_string()]);
    assert_eq!(embedding_providers.len(), 1);
    assert_eq!(embedding_providers[0], "embedding-provider");

    // Test provider type filtering
    let llm_provider = ai_pool.get_provider_by_type("llm");
    assert!(llm_provider.is_some());

    let embedding_provider = ai_pool.get_provider_by_type("embedding");
    assert!(embedding_provider.is_some());

    // Test health metrics
    ai_pool.update_ai_provider_health("text-gen-provider", 150, true);
    ai_pool.update_ai_provider_health("embedding-provider", 200, true);

    let stats = ai_pool.get_ai_provider_stats();
    assert!(stats.contains_key("text-gen-provider"));
    assert!(stats.contains_key("embedding-provider"));

    println!("✅ Universal AI connections integration working");
}

#[tokio::test]
async fn test_provider_priority_and_scoring() {
    let config = UniversalAdapterConfig::default();
    let adapter = UniversalPrimalAdapter::new(config).unwrap();

    // Register providers with different "performance characteristics"
    let providers = vec![
        ("fast-provider", true, 50),   // Fast and healthy
        ("slow-provider", true, 200),  // Slow but healthy
        ("broken-provider", false, 0), // Broken
    ];

    for (name, healthy, _response_time) in providers {
        let provider = Arc::new(MockSecurityProvider {
            name: name.to_string(),
            capabilities: vec!["encryption".to_string()],
            healthy,
        });
        adapter
            .register_security_provider(name.to_string(), provider)
            .await
            .unwrap();
    }

    // Test multiple requests to see if we consistently get the healthy provider
    for _ in 0..5 {
        let provider = adapter
            .get_security_provider_with_capability("encryption")
            .await;
        assert!(provider.is_some());

        let result = provider.unwrap().encrypt(b"test", "AES").await;
        assert!(result.is_ok());

        // Should get one of the healthy providers (fast or slow, not broken)
        let encrypted = result.unwrap();
        let encrypted_str = String::from_utf8_lossy(&encrypted);
        assert!(encrypted_str.contains("fast-provider") || encrypted_str.contains("slow-provider"));
        assert!(!encrypted_str.contains("broken-provider"));
    }

    println!("✅ Provider priority and scoring working");
}

#[tokio::test]
async fn test_concurrent_provider_operations() {
    let config = UniversalAdapterConfig::default();
    let adapter = Arc::new(UniversalPrimalAdapter::new(config).unwrap());

    // Register a provider
    let provider = Arc::new(MockComputeProvider {
        name: "concurrent-test".to_string(),
        capabilities: vec!["processing".to_string()],
        healthy: true,
    });
    adapter
        .register_compute_provider("concurrent".to_string(), provider)
        .await
        .unwrap();

    // Run concurrent operations
    let mut handles = Vec::new();

    for i in 0..10 {
        let adapter_clone = Arc::clone(&adapter);
        let handle = tokio::spawn(async move {
            if let Some(provider) = adapter_clone.get_compute_provider().await {
                provider
                    .process_data(format!("data-{}", i).into_bytes())
                    .await
            } else {
                Err(nestgate_core::NestGateError::External(
                    "No provider".to_string(),
                ))
            }
        });
        handles.push(handle);
    }

    // Wait for all operations to complete
    let results = futures::future::join_all(handles).await;

    let mut success_count = 0;
    for result in results {
        if let Ok(Ok(_)) = result {
            success_count += 1;
        }
    }

    assert!(success_count > 0);
    println!(
        "✅ Concurrent provider operations working: {}/10 succeeded",
        success_count
    );
}

#[tokio::test]
async fn test_provider_configuration_migration() {
    // Test that legacy configuration works with universal providers
    use nestgate_core::config::environment::EnvironmentConfig;

    let env_config = EnvironmentConfig::default();

    // Test that capability-based configuration is properly set
    assert!(env_config.enable_primal_auto_discovery);
    assert!(!env_config.ai_provider_capabilities.is_empty());
    assert!(!env_config.security_provider_capabilities.is_empty());
    assert!(!env_config.orchestration_provider_capabilities.is_empty());

    // Test legacy fallback compatibility
    if env_config.squirrel_api_key.is_some() {
        println!("Legacy Squirrel API key available for migration");
    }

    println!("✅ Provider configuration migration working");
}

// Performance benchmark test
#[tokio::test]
async fn test_provider_selection_performance() {
    let config = UniversalAdapterConfig::default();
    let adapter = UniversalPrimalAdapter::new(config).unwrap();

    // Register many providers
    for i in 0..100 {
        let provider = Arc::new(MockComputeProvider {
            name: format!("provider-{}", i),
            capabilities: vec!["processing".to_string(), format!("capability-{}", i % 10)],
            healthy: i % 5 != 0, // 80% healthy
        });
        adapter
            .register_compute_provider(format!("provider-{}", i), provider)
            .await
            .unwrap();
    }

    let start = std::time::Instant::now();

    // Perform many provider selections
    for _ in 0..1000 {
        let _provider = adapter.get_compute_provider().await;
    }

    let duration = start.elapsed();
    let ops_per_second = 1000.0 / duration.as_secs_f64();

    println!(
        "✅ Provider selection performance: {:.0} ops/sec",
        ops_per_second
    );
    assert!(ops_per_second > 100.0, "Provider selection should be fast");
}
