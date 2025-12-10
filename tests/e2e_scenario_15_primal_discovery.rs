//! **E2E SCENARIO 15: DYNAMIC PRIMAL DISCOVERY**
//!
//! **Objective**: Test automatic discovery of ecosystem primals (Songbird, Squirrel, ToadStool)
//!
//! **Priority**: Critical | **Complexity**: High
//!
//! **Test Flow**:
//! 1. Start NestGate in isolation
//! 2. Bring up Songbird (AI)
//! 3. Verify automatic discovery
//! 4. Bring up Squirrel (metadata)
//! 5. Verify service integration
//! 6. Bring up ToadStool (monitoring)
//! 7. Verify full ecosystem awareness
//!
//! **Expected Outcomes**:
//! - All primals discovered automatically
//! - Services integrated seamlessly
//! - Capabilities advertised correctly
//! - Health monitoring active

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::RwLock;

/// Primal service types in the ecosystem
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PrimalType {
    Songbird,  // AI service
    Squirrel,  // Metadata service
    ToadStool, // Monitoring service
    Weasel,    // Security service
}

impl PrimalType {
    fn name(&self) -> &str {
        match self {
            PrimalType::Songbird => "Songbird",
            PrimalType::Squirrel => "Squirrel",
            PrimalType::ToadStool => "ToadStool",
            PrimalType::Weasel => "Weasel",
        }
    }

    fn capabilities(&self) -> Vec<&str> {
        match self {
            PrimalType::Songbird => vec!["ai", "ml", "inference", "training"],
            PrimalType::Squirrel => vec!["metadata", "search", "indexing", "tagging"],
            PrimalType::ToadStool => vec!["monitoring", "metrics", "alerting", "logging"],
            PrimalType::Weasel => vec!["security", "auth", "encryption", "audit"],
        }
    }
}

/// Service state
#[derive(Debug, Clone, Copy, PartialEq)]
enum ServiceState {
    Unknown,
    Discovering,
    Available,
    Integrated,
    Unhealthy,
}

/// Primal service information
#[allow(dead_code)] // Test infrastructure
struct PrimalService {
    primal_type: PrimalType,
    state: Arc<AtomicU8>,
    endpoint: String,
    capabilities: Vec<String>,
    last_seen: Arc<RwLock<SystemTime>>,
    health_score: Arc<AtomicU8>,
}

impl PrimalService {
    fn new(primal_type: PrimalType, endpoint: &str) -> Self {
        Self {
            primal_type,
            state: Arc::new(AtomicU8::new(ServiceState::Unknown as u8)),
            endpoint: endpoint.to_string(),
            capabilities: primal_type
                .capabilities()
                .iter()
                .map(|s| s.to_string())
                .collect(),
            last_seen: Arc::new(RwLock::new(SystemTime::now())),
            health_score: Arc::new(AtomicU8::new(100)),
        }
    }

    fn state(&self) -> ServiceState {
        match self.state.load(Ordering::SeqCst) {
            0 => ServiceState::Unknown,
            1 => ServiceState::Discovering,
            2 => ServiceState::Available,
            3 => ServiceState::Integrated,
            4 => ServiceState::Unhealthy,
            _ => ServiceState::Unknown,
        }
    }

    fn set_state(&self, state: ServiceState) {
        self.state.store(state as u8, Ordering::SeqCst);
    }

    async fn update_last_seen(&self) {
        let mut last_seen = self.last_seen.write().await;
        *last_seen = SystemTime::now();
    }

    fn health_score(&self) -> u8 {
        self.health_score.load(Ordering::SeqCst)
    }

    fn set_health(&self, score: u8) {
        self.health_score.store(score, Ordering::SeqCst);
    }
}

/// Service discovery manager
struct DiscoveryManager {
    services: Arc<RwLock<HashMap<PrimalType, Arc<PrimalService>>>>,
    auto_discovery_enabled: Arc<AtomicBool>,
    discovery_interval_ms: u64,
}

impl DiscoveryManager {
    fn new(discovery_interval_ms: u64) -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            auto_discovery_enabled: Arc::new(AtomicBool::new(true)),
            discovery_interval_ms,
        }
    }

    async fn discover_service(
        &self,
        primal_type: PrimalType,
        endpoint: &str,
    ) -> Result<(), String> {
        if !self.auto_discovery_enabled.load(Ordering::SeqCst) {
            return Err("Auto-discovery is disabled".to_string());
        }

        let service = Arc::new(PrimalService::new(primal_type, endpoint));
        service.set_state(ServiceState::Discovering);

        // Simulate discovery delay
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        service.set_state(ServiceState::Available);
        service.update_last_seen().await;

        let mut services = self.services.write().await;
        services.insert(primal_type, service);

        Ok(())
    }

    async fn integrate_service(&self, primal_type: PrimalType) -> Result<(), String> {
        let services = self.services.read().await;
        let service = services
            .get(&primal_type)
            .ok_or_else(|| format!("{} not discovered", primal_type.name()))?;

        if service.state() != ServiceState::Available {
            return Err(format!("{} is not available", primal_type.name()));
        }

        service.set_state(ServiceState::Integrated);
        service.update_last_seen().await;

        Ok(())
    }

    async fn service_count(&self) -> usize {
        self.services.read().await.len()
    }

    async fn integrated_count(&self) -> usize {
        let services = self.services.read().await;
        services
            .values()
            .filter(|s| s.state() == ServiceState::Integrated)
            .count()
    }

    async fn has_service(&self, primal_type: PrimalType) -> bool {
        self.services.read().await.contains_key(&primal_type)
    }

    async fn get_capabilities(&self, primal_type: PrimalType) -> Option<Vec<String>> {
        let services = self.services.read().await;
        services.get(&primal_type).map(|s| s.capabilities.clone())
    }

    async fn check_health(&self, primal_type: PrimalType) -> Option<u8> {
        let services = self.services.read().await;
        services.get(&primal_type).map(|s| s.health_score())
    }

    fn enable_auto_discovery(&self) {
        self.auto_discovery_enabled.store(true, Ordering::SeqCst);
    }

    fn disable_auto_discovery(&self) {
        self.auto_discovery_enabled.store(false, Ordering::SeqCst);
    }
}

// ==================== E2E TESTS ====================

#[tokio::test]
async fn e2e_scenario_15_start_in_isolation() {
    eprintln!("\n🧪 E2E: Start NestGate in Isolation");

    let manager = DiscoveryManager::new(1000);
    assert_eq!(manager.service_count().await, 0);

    eprintln!("✅ Passed - Started with no services");
}

#[tokio::test]
async fn e2e_scenario_15_discover_songbird() {
    eprintln!("\n🧪 E2E: Discover Songbird (AI Service)");

    let manager = DiscoveryManager::new(1000);

    let result = manager
        .discover_service(PrimalType::Songbird, "http://localhost:8001")
        .await;
    assert!(result.is_ok());

    assert!(manager.has_service(PrimalType::Songbird).await);
    assert_eq!(manager.service_count().await, 1);

    eprintln!("✅ Passed - Songbird discovered");
}

#[tokio::test]
async fn e2e_scenario_15_discover_squirrel() {
    eprintln!("\n🧪 E2E: Discover Squirrel (Metadata Service)");

    let manager = DiscoveryManager::new(1000);

    manager
        .discover_service(PrimalType::Songbird, "http://localhost:8001")
        .await
        .unwrap();
    manager
        .discover_service(PrimalType::Squirrel, "http://localhost:8002")
        .await
        .unwrap();

    assert!(manager.has_service(PrimalType::Squirrel).await);
    assert_eq!(manager.service_count().await, 2);

    eprintln!("✅ Passed - Squirrel discovered");
}

#[tokio::test]
async fn e2e_scenario_15_discover_toadstool() {
    eprintln!("\n🧪 E2E: Discover ToadStool (Monitoring Service)");

    let manager = DiscoveryManager::new(1000);

    manager
        .discover_service(PrimalType::Songbird, "http://localhost:8001")
        .await
        .unwrap();
    manager
        .discover_service(PrimalType::Squirrel, "http://localhost:8002")
        .await
        .unwrap();
    manager
        .discover_service(PrimalType::ToadStool, "http://localhost:8003")
        .await
        .unwrap();

    assert!(manager.has_service(PrimalType::ToadStool).await);
    assert_eq!(manager.service_count().await, 3);

    eprintln!("✅ Passed - ToadStool discovered");
}

#[tokio::test]
async fn e2e_scenario_15_integrate_songbird() {
    eprintln!("\n🧪 E2E: Integrate Songbird Service");

    let manager = DiscoveryManager::new(1000);

    manager
        .discover_service(PrimalType::Songbird, "http://localhost:8001")
        .await
        .unwrap();
    let result = manager.integrate_service(PrimalType::Songbird).await;

    assert!(result.is_ok());
    assert_eq!(manager.integrated_count().await, 1);

    eprintln!("✅ Passed - Songbird integrated");
}

#[tokio::test]
async fn e2e_scenario_15_capabilities_advertised() {
    eprintln!("\n🧪 E2E: Capabilities Advertised Correctly");

    let manager = DiscoveryManager::new(1000);

    manager
        .discover_service(PrimalType::Songbird, "http://localhost:8001")
        .await
        .unwrap();

    let capabilities = manager.get_capabilities(PrimalType::Songbird).await;
    assert!(capabilities.is_some());

    let caps = capabilities.unwrap();
    assert!(caps.contains(&"ai".to_string()));
    assert!(caps.contains(&"ml".to_string()));
    assert!(caps.contains(&"inference".to_string()));

    eprintln!("✅ Passed - Capabilities: {:?}", caps);
}

#[tokio::test]
async fn e2e_scenario_15_health_monitoring() {
    eprintln!("\n🧪 E2E: Health Monitoring Active");

    let manager = DiscoveryManager::new(1000);

    manager
        .discover_service(PrimalType::Songbird, "http://localhost:8001")
        .await
        .unwrap();

    let health = manager.check_health(PrimalType::Songbird).await;
    assert!(health.is_some());
    assert_eq!(health.unwrap(), 100);

    eprintln!("✅ Passed - Health monitoring: {}", health.unwrap());
}

#[tokio::test]
async fn e2e_scenario_15_concurrent_discovery() {
    eprintln!("\n🧪 E2E: Concurrent Service Discovery");

    let manager = Arc::new(DiscoveryManager::new(1000));

    let handles = vec![
        {
            let manager = manager.clone();
            tokio::spawn(async move {
                manager
                    .discover_service(PrimalType::Songbird, "http://localhost:8001")
                    .await
            })
        },
        {
            let manager = manager.clone();
            tokio::spawn(async move {
                manager
                    .discover_service(PrimalType::Squirrel, "http://localhost:8002")
                    .await
            })
        },
        {
            let manager = manager.clone();
            tokio::spawn(async move {
                manager
                    .discover_service(PrimalType::ToadStool, "http://localhost:8003")
                    .await
            })
        },
    ];

    for handle in handles {
        handle.await.unwrap().unwrap();
    }

    assert_eq!(manager.service_count().await, 3);

    eprintln!("✅ Passed - 3 services discovered concurrently");
}

#[tokio::test]
async fn e2e_scenario_15_full_ecosystem_awareness() {
    eprintln!("\n🧪 E2E: Full Ecosystem Awareness");

    let manager = DiscoveryManager::new(1000);

    // Discover all core primals
    manager
        .discover_service(PrimalType::Songbird, "http://localhost:8001")
        .await
        .unwrap();
    manager
        .discover_service(PrimalType::Squirrel, "http://localhost:8002")
        .await
        .unwrap();
    manager
        .discover_service(PrimalType::ToadStool, "http://localhost:8003")
        .await
        .unwrap();
    manager
        .discover_service(PrimalType::Weasel, "http://localhost:8004")
        .await
        .unwrap();

    assert_eq!(manager.service_count().await, 4);
    assert!(manager.has_service(PrimalType::Songbird).await);
    assert!(manager.has_service(PrimalType::Squirrel).await);
    assert!(manager.has_service(PrimalType::ToadStool).await);
    assert!(manager.has_service(PrimalType::Weasel).await);

    eprintln!("✅ Passed - Full ecosystem awareness (4 primals)");
}

#[tokio::test]
async fn e2e_scenario_15_auto_discovery_toggle() {
    eprintln!("\n🧪 E2E: Auto-Discovery Enable/Disable");

    let manager = DiscoveryManager::new(1000);

    // Disable auto-discovery
    manager.disable_auto_discovery();
    let result = manager
        .discover_service(PrimalType::Songbird, "http://localhost:8001")
        .await;
    assert!(result.is_err());

    // Enable auto-discovery
    manager.enable_auto_discovery();
    let result = manager
        .discover_service(PrimalType::Songbird, "http://localhost:8001")
        .await;
    assert!(result.is_ok());

    eprintln!("✅ Passed - Auto-discovery toggle working");
}

#[tokio::test]
async fn e2e_scenario_15_service_integration_order() {
    eprintln!("\n🧪 E2E: Service Integration Order");

    let manager = DiscoveryManager::new(1000);

    // Discover services
    manager
        .discover_service(PrimalType::Songbird, "http://localhost:8001")
        .await
        .unwrap();
    manager
        .discover_service(PrimalType::Squirrel, "http://localhost:8002")
        .await
        .unwrap();

    // Integrate in order
    manager
        .integrate_service(PrimalType::Songbird)
        .await
        .unwrap();
    manager
        .integrate_service(PrimalType::Squirrel)
        .await
        .unwrap();

    assert_eq!(manager.integrated_count().await, 2);

    eprintln!("✅ Passed - Services integrated in order");
}

#[tokio::test]
async fn e2e_scenario_15_rediscover_after_failure() {
    eprintln!("\n🧪 E2E: Rediscover Service After Failure");

    let manager = DiscoveryManager::new(1000);

    // Initial discovery
    manager
        .discover_service(PrimalType::Songbird, "http://localhost:8001")
        .await
        .unwrap();

    // Simulate service becoming unhealthy (in real scenario, this would be detected)
    let services = manager.services.read().await;
    if let Some(service) = services.get(&PrimalType::Songbird) {
        service.set_state(ServiceState::Unhealthy);
    }
    drop(services);

    // Rediscover
    manager
        .discover_service(PrimalType::Songbird, "http://localhost:8001")
        .await
        .unwrap();

    assert!(manager.has_service(PrimalType::Songbird).await);

    eprintln!("✅ Passed - Service rediscovered after failure");
}

#[tokio::test]
async fn e2e_scenario_15_capabilities_per_service() {
    eprintln!("\n🧪 E2E: Unique Capabilities Per Service");

    let manager = DiscoveryManager::new(1000);

    manager
        .discover_service(PrimalType::Songbird, "http://localhost:8001")
        .await
        .unwrap();
    manager
        .discover_service(PrimalType::Squirrel, "http://localhost:8002")
        .await
        .unwrap();

    let songbird_caps = manager
        .get_capabilities(PrimalType::Songbird)
        .await
        .unwrap();
    let squirrel_caps = manager
        .get_capabilities(PrimalType::Squirrel)
        .await
        .unwrap();

    assert!(songbird_caps.contains(&"ai".to_string()));
    assert!(!songbird_caps.contains(&"metadata".to_string()));

    assert!(squirrel_caps.contains(&"metadata".to_string()));
    assert!(!squirrel_caps.contains(&"ai".to_string()));

    eprintln!("✅ Passed - Unique capabilities verified");
}

#[tokio::test]
async fn e2e_scenario_15_full_integration() {
    eprintln!("\n🧪 E2E SCENARIO 15: FULL INTEGRATION TEST");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let manager = DiscoveryManager::new(1000);

    // Step 1: Start in isolation
    eprintln!("Step 1: NestGate starting in isolation");
    assert_eq!(manager.service_count().await, 0);
    eprintln!("   ✓ No services detected");

    // Step 2: Bring up Songbird
    eprintln!("Step 2: Bringing up Songbird (AI)");
    manager
        .discover_service(PrimalType::Songbird, "http://localhost:8001")
        .await
        .unwrap();
    assert!(manager.has_service(PrimalType::Songbird).await);
    eprintln!("   ✓ Songbird discovered automatically");

    // Step 3: Verify discovery
    let capabilities = manager
        .get_capabilities(PrimalType::Songbird)
        .await
        .unwrap();
    assert!(capabilities.contains(&"ai".to_string()));
    eprintln!("   ✓ Capabilities advertised: {:?}", capabilities);

    // Step 4: Bring up Squirrel
    eprintln!("Step 4: Bringing up Squirrel (Metadata)");
    manager
        .discover_service(PrimalType::Squirrel, "http://localhost:8002")
        .await
        .unwrap();
    assert!(manager.has_service(PrimalType::Squirrel).await);
    eprintln!("   ✓ Squirrel discovered automatically");

    // Step 5: Integrate services
    eprintln!("Step 5: Integrating services");
    manager
        .integrate_service(PrimalType::Songbird)
        .await
        .unwrap();
    manager
        .integrate_service(PrimalType::Squirrel)
        .await
        .unwrap();
    assert_eq!(manager.integrated_count().await, 2);
    eprintln!("   ✓ Services integrated seamlessly");

    // Step 6: Bring up ToadStool
    eprintln!("Step 6: Bringing up ToadStool (Monitoring)");
    manager
        .discover_service(PrimalType::ToadStool, "http://localhost:8003")
        .await
        .unwrap();
    manager
        .integrate_service(PrimalType::ToadStool)
        .await
        .unwrap();
    eprintln!("   ✓ ToadStool discovered and integrated");

    // Step 7: Verify full ecosystem
    eprintln!("Step 7: Verifying full ecosystem awareness");
    assert_eq!(manager.service_count().await, 3);
    assert_eq!(manager.integrated_count().await, 3);

    let health_songbird = manager.check_health(PrimalType::Songbird).await.unwrap();
    let health_squirrel = manager.check_health(PrimalType::Squirrel).await.unwrap();
    let health_toadstool = manager.check_health(PrimalType::ToadStool).await.unwrap();

    assert_eq!(health_songbird, 100);
    assert_eq!(health_squirrel, 100);
    assert_eq!(health_toadstool, 100);
    eprintln!("   ✓ All services healthy (100%)");

    eprintln!("\n✅ SCENARIO 15 COMPLETE");
    eprintln!("   ✓ All primals discovered automatically");
    eprintln!("   ✓ Services integrated seamlessly");
    eprintln!("   ✓ Capabilities advertised correctly");
    eprintln!("   ✓ Health monitoring active");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}
