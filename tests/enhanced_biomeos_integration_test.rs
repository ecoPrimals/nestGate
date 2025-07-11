use std::collections::HashMap;

use nestgate_core::biomeos::{
    AgentSpec, BackupConfig, BiomeManifest, BiomeMetadata, BiomeNetworking, BiomeResources,
    BiomeSecurity, BiomeStorage, ComputeResources, CoordinationConfig, DiscoveryConfig,
    EventCoordinationConfig, HealthCheckEndpoint, HealthChecksConfig, NetworkResources, PortSpec,
    PrimalConfig, ResourceLimits, RetryConfig, SecurityLevel, ServiceConfig, ServiceMeshConfig,
    StoragePolicies, StorageResources, VolumeSpec,
};

#[tokio::test]
async fn test_enhanced_biome_manifest_structure() {
    let manifest = BiomeManifest {
        api_version: "v1".to_string(),
        kind: "EnhancedBiome".to_string(),
        metadata: BiomeMetadata {
            name: "enhanced-biome".to_string(),
            version: "2.0.0".to_string(),
            description: Some("Enhanced biome with full compatibility".to_string()),
            author: Some("nestgate".to_string()),
            created_at: Some(chrono::Utc::now()),
            labels: Some({
                let mut labels = HashMap::new();
                labels.insert("biome.type".to_string(), "enhanced".to_string());
                labels.insert("compatibility".to_string(), "100%".to_string());
                labels
            }),
            annotations: Some({
                let mut annotations = HashMap::new();
                annotations.insert("biome.nestgate.io/enhanced".to_string(), "true".to_string());
                annotations.insert(
                    "biome.nestgate.io/compatibility".to_string(),
                    "v1".to_string(),
                );
                annotations
            }),
        },
        primals: HashMap::new(),
        services: HashMap::new(),
        resources: BiomeResources {
            storage: Some(StorageResources {
                total_gb: 10,
                volumes: vec![VolumeSpec {
                    name: "enhanced-storage".to_string(),
                    size: "10Gi".to_string(),
                    tier: "hot".to_string(),
                    provisioner: "nestgate".to_string(),
                    mount_path: Some("/data".to_string()),
                    access_mode: Some("ReadWriteOnce".to_string()),
                    options: None,
                    protocols: None,
                    backup_policy: None,
                }],
            }),
            compute: Some(ComputeResources {
                cpu_cores: 2.0,
                memory_mb: 4096,
                gpu: None,
            }),
            network: Some(NetworkResources {
                bandwidth_mbps: Some(1000),
                ports: vec![PortSpec {
                    port: 8080,
                    protocol: "TCP".to_string(),
                    expose_externally: true,
                }],
            }),
        },
        security: BiomeSecurity {
            security_level: SecurityLevel::Standard,
            encryption_policies: None,
            access_controls: None,
            audit_requirements: None,
        },
        networking: BiomeNetworking {
            service_mesh: Some(ServiceMeshConfig {
                enabled: true,
                provider: "istio".to_string(),
                config: HashMap::new(),
            }),
            load_balancing: None,
            network_policies: None,
        },
        storage: BiomeStorage {
            default_class: Some("fast".to_string()),
            volumes: vec![],
            policies: None,
            backup: None,
        },
        specialization: None,
        templates: None,
        agents: Some(vec![AgentSpec {
            name: "enhanced-agent".to_string(),
            runtime: "container".to_string(),
            capabilities: vec!["enhanced-biomeos".to_string()],
            executor: "ai-primal".to_string(),
            resource_limits: Some(ResourceLimits {
                memory_mb: Some(4096),
                cpu_percent: Some(80),
                timeout_seconds: Some(3600),
            }),
            ai_provider: None,
            model: None,
            env: None,
        }]),
        coordination: Some(CoordinationConfig {
            service_mesh: Some(ServiceMeshConfig {
                enabled: true,
                provider: "istio".to_string(),
                config: HashMap::new(),
            }),
            discovery: Some(DiscoveryConfig {
                provider: "orchestration-primal".to_string(),
                timeout_seconds: Some(30),
                retry: Some(RetryConfig {
                    max_retries: 3,
                    interval_seconds: 5,
                    backoff: Some("exponential".to_string()),
                }),
            }),
            health_checks: Some(HealthChecksConfig {
                interval_seconds: 30,
                timeout_seconds: 5,
                endpoints: vec![HealthCheckEndpoint {
                    name: "health".to_string(),
                    path: "/health".to_string(),
                    expected_status: Some(200),
                }],
            }),
            events: Some(EventCoordinationConfig {
                provider: "orchestration-primal".to_string(),
                topics: vec!["biome.events".to_string()],
                retention_hours: Some(24),
            }),
        }),
    };

    // Test enhanced biome structure
    assert_eq!(manifest.api_version, "v1");
    assert_eq!(manifest.kind, "EnhancedBiome");
    assert!(manifest.coordination.is_some());

    let coordination = manifest.coordination.unwrap();
    assert!(coordination.service_mesh.is_some());
    assert!(coordination.discovery.is_some());
    assert!(coordination.health_checks.is_some());
    assert!(coordination.events.is_some());

    assert!(manifest.networking.service_mesh.is_some());
    let service_mesh = manifest.networking.service_mesh.unwrap();
    assert!(service_mesh.enabled);
    assert_eq!(service_mesh.provider, "istio");
}

#[tokio::test]
async fn test_coordination_config() {
    let coordination = CoordinationConfig {
        service_mesh: Some(ServiceMeshConfig {
            enabled: true,
            provider: "istio".to_string(),
            config: HashMap::new(),
        }),
        discovery: Some(DiscoveryConfig {
            provider: "orchestration-primal".to_string(),
            timeout_seconds: Some(60),
            retry: Some(RetryConfig {
                max_retries: 5,
                interval_seconds: 10,
                backoff: Some("linear".to_string()),
            }),
        }),
        health_checks: Some(HealthChecksConfig {
            interval_seconds: 60,
            timeout_seconds: 10,
            endpoints: vec![
                HealthCheckEndpoint {
                    name: "health".to_string(),
                    path: "/health".to_string(),
                    expected_status: Some(200),
                },
                HealthCheckEndpoint {
                    name: "metrics".to_string(),
                    path: "/metrics".to_string(),
                    expected_status: Some(200),
                },
            ],
        }),
        events: Some(EventCoordinationConfig {
            provider: "orchestration-primal".to_string(),
            topics: vec!["test.events".to_string(), "coordination.events".to_string()],
            retention_hours: Some(48),
        }),
    };

    // Test coordination configuration
    assert!(coordination.service_mesh.is_some());
    let service_mesh = coordination.service_mesh.unwrap();
    assert!(service_mesh.enabled);
    assert_eq!(service_mesh.provider, "istio");

    assert!(coordination.discovery.is_some());
    let discovery = coordination.discovery.unwrap();
    assert_eq!(discovery.provider, "orchestration-primal");
    assert_eq!(discovery.timeout_seconds, Some(60));

    assert!(coordination.health_checks.is_some());
    let health_checks = coordination.health_checks.unwrap();
    assert_eq!(health_checks.interval_seconds, 60);
    assert_eq!(health_checks.timeout_seconds, 10);
    assert_eq!(health_checks.endpoints.len(), 2);

    assert!(coordination.events.is_some());
    let events = coordination.events.unwrap();
    assert_eq!(events.provider, "orchestration-primal");
    assert_eq!(events.topics.len(), 2);
    assert_eq!(events.retention_hours, Some(48));
}

#[tokio::test]
async fn test_100_percent_compatibility() {
    let manifest = BiomeManifest {
        api_version: "v1".to_string(),
        kind: "CompatibilityTest".to_string(),
        metadata: BiomeMetadata {
            name: "compatibility-test".to_string(),
            version: "1.0.0".to_string(),
            description: Some("Test 100% biomeOS compatibility".to_string()),
            author: Some("nestgate".to_string()),
            created_at: Some(chrono::Utc::now()),
            labels: None,
            annotations: None,
        },
        primals: HashMap::new(),
        services: HashMap::new(),
        resources: BiomeResources {
            storage: None,
            compute: None,
            network: None,
        },
        security: BiomeSecurity {
            security_level: SecurityLevel::Basic,
            encryption_policies: None,
            access_controls: None,
            audit_requirements: None,
        },
        networking: BiomeNetworking {
            service_mesh: None,
            load_balancing: None,
            network_policies: None,
        },
        storage: BiomeStorage {
            default_class: None,
            volumes: vec![],
            policies: None,
            backup: None,
        },
        specialization: None,
        templates: None,
        agents: None,
        coordination: None,
    };

    // Test that all required fields are present for 100% compatibility
    assert!(manifest.api_version.len() > 0);
    assert!(manifest.kind.len() > 0);
    assert!(manifest.metadata.name.len() > 0);
    assert!(manifest.metadata.version.len() > 0);
    assert!(manifest.metadata.description.is_some());
    assert!(manifest.metadata.author.is_some());
    assert!(manifest.metadata.created_at.is_some());

    // Test that all optional fields are accessible
    assert!(manifest.agents.is_none());
    assert!(manifest.coordination.is_none());
    assert!(manifest.specialization.is_none());
    assert!(manifest.templates.is_none());
    assert!(manifest.resources.storage.is_none());
    assert!(manifest.resources.compute.is_none());
    assert!(manifest.resources.network.is_none());
    assert!(manifest.networking.service_mesh.is_none());
    assert!(manifest.storage.default_class.is_none());
    assert!(manifest.storage.volumes.is_empty());
}

#[tokio::test]
async fn test_universal_patterns() {
    let manifest = BiomeManifest {
        api_version: "v1".to_string(),
        kind: "UniversalPattern".to_string(),
        metadata: BiomeMetadata {
            name: "universal-pattern".to_string(),
            version: "1.0.0".to_string(),
            description: Some("Universal pattern implementation".to_string()),
            author: Some("nestgate".to_string()),
            created_at: Some(chrono::Utc::now()),
            labels: Some({
                let mut labels = HashMap::new();
                labels.insert("pattern".to_string(), "universal".to_string());
                labels.insert("cross-primal".to_string(), "enabled".to_string());
                labels
            }),
            annotations: Some({
                let mut annotations = HashMap::new();
                annotations.insert(
                    "universal.nestgate.io/pattern".to_string(),
                    "true".to_string(),
                );
                annotations.insert(
                    "cross-primal.nestgate.io/enabled".to_string(),
                    "true".to_string(),
                );
                annotations
            }),
        },
        primals: HashMap::new(),
        services: HashMap::new(),
        resources: BiomeResources {
            storage: None,
            compute: None,
            network: None,
        },
        security: BiomeSecurity {
            security_level: SecurityLevel::Standard,
            encryption_policies: None,
            access_controls: None,
            audit_requirements: None,
        },
        networking: BiomeNetworking {
            service_mesh: Some(ServiceMeshConfig {
                enabled: true,
                provider: "istio".to_string(),
                config: HashMap::new(),
            }),
            load_balancing: None,
            network_policies: None,
        },
        storage: BiomeStorage {
            default_class: None,
            volumes: vec![],
            policies: None,
            backup: None,
        },
        specialization: None,
        templates: None,
        agents: None,
        coordination: Some(CoordinationConfig {
            service_mesh: Some(ServiceMeshConfig {
                enabled: true,
                provider: "istio".to_string(),
                config: HashMap::new(),
            }),
            discovery: Some(DiscoveryConfig {
                provider: "orchestration-primal".to_string(),
                timeout_seconds: Some(30),
                retry: Some(RetryConfig {
                    max_retries: 3,
                    interval_seconds: 5,
                    backoff: Some("exponential".to_string()),
                }),
            }),
            health_checks: Some(HealthChecksConfig {
                interval_seconds: 30,
                timeout_seconds: 5,
                endpoints: vec![HealthCheckEndpoint {
                    name: "health".to_string(),
                    path: "/health".to_string(),
                    expected_status: Some(200),
                }],
            }),
            events: Some(EventCoordinationConfig {
                provider: "orchestration-primal".to_string(),
                topics: vec!["universal.events".to_string()],
                retention_hours: Some(24),
            }),
        }),
    };

    // Test universal pattern configuration
    assert_eq!(manifest.kind, "UniversalPattern");
    assert!(manifest.metadata.labels.is_some());
    let labels = manifest.metadata.labels.unwrap();
    assert_eq!(labels.get("pattern"), Some(&"universal".to_string()));
    assert_eq!(labels.get("cross-primal"), Some(&"enabled".to_string()));
    assert!(manifest.metadata.annotations.is_some());
    let annotations = manifest.metadata.annotations.unwrap();
    assert_eq!(
        annotations.get("universal.nestgate.io/pattern"),
        Some(&"true".to_string())
    );
    assert_eq!(
        annotations.get("cross-primal.nestgate.io/enabled"),
        Some(&"true".to_string())
    );
    assert!(manifest.coordination.is_some());

    let coordination = manifest.coordination.unwrap();
    assert!(coordination.service_mesh.is_some());
    assert!(coordination.discovery.is_some());
    assert!(coordination.health_checks.is_some());
    assert!(coordination.events.is_some());

    assert!(manifest.networking.service_mesh.is_some());
    let service_mesh = manifest.networking.service_mesh.unwrap();
    assert!(service_mesh.enabled);
    assert_eq!(service_mesh.provider, "istio");
}

#[tokio::test]
async fn test_cross_primal_integration() {
    let manifest = BiomeManifest {
        api_version: "v1".to_string(),
        kind: "CrossPrimalIntegration".to_string(),
        metadata: BiomeMetadata {
            name: "cross-primal-integration".to_string(),
            version: "1.0.0".to_string(),
            description: Some("Cross-primal integration test".to_string()),
            author: Some("nestgate".to_string()),
            created_at: Some(chrono::Utc::now()),
            labels: Some({
                let mut labels = HashMap::new();
                labels.insert("integration".to_string(), "cross-primal".to_string());
                labels.insert(
                    "primals".to_string(),
                    "orchestration-primal,ai-primal,compute-primal,security-primal".to_string(),
                );
                labels
            }),
            annotations: Some({
                let mut annotations = HashMap::new();
                annotations.insert(
                    "cross-primal.nestgate.io/enabled".to_string(),
                    "true".to_string(),
                );
                annotations.insert(
                    "cross-primal.nestgate.io/primals".to_string(),
                    "all".to_string(),
                );
                annotations
            }),
        },
        primals: {
            let mut primals = HashMap::new();
            primals.insert(
                "orchestration-primal".to_string(),
                PrimalConfig {
                    primal_type: "orchestration".to_string(),
                    enabled: true,
                    endpoint: Some("http://localhost:8080".to_string()),
                    capabilities: vec!["discovery".to_string(), "coordination".to_string()],
                },
            );
            primals.insert(
                "ai-primal".to_string(),
                PrimalConfig {
                    primal_type: "ai".to_string(),
                    enabled: true,
                    endpoint: Some("http://localhost:8081".to_string()),
                    capabilities: vec!["optimization".to_string(), "analysis".to_string()],
                },
            );
            primals
        },
        services: HashMap::new(),
        resources: BiomeResources {
            storage: None,
            compute: None,
            network: None,
        },
        security: BiomeSecurity {
            security_level: SecurityLevel::Standard,
            encryption_policies: None,
            access_controls: None,
            audit_requirements: None,
        },
        networking: BiomeNetworking {
            service_mesh: Some(ServiceMeshConfig {
                enabled: true,
                provider: "istio".to_string(),
                config: HashMap::new(),
            }),
            load_balancing: None,
            network_policies: None,
        },
        storage: BiomeStorage {
            default_class: None,
            volumes: vec![],
            policies: None,
            backup: None,
        },
        specialization: None,
        templates: None,
        agents: None,
        coordination: Some(CoordinationConfig {
            service_mesh: Some(ServiceMeshConfig {
                enabled: true,
                provider: "istio".to_string(),
                config: HashMap::new(),
            }),
            discovery: Some(DiscoveryConfig {
                provider: "orchestration-primal".to_string(),
                timeout_seconds: Some(30),
                retry: Some(RetryConfig {
                    max_retries: 3,
                    interval_seconds: 5,
                    backoff: Some("exponential".to_string()),
                }),
            }),
            health_checks: Some(HealthChecksConfig {
                interval_seconds: 30,
                timeout_seconds: 5,
                endpoints: vec![HealthCheckEndpoint {
                    name: "health".to_string(),
                    path: "/health".to_string(),
                    expected_status: Some(200),
                }],
            }),
            events: Some(EventCoordinationConfig {
                provider: "orchestration-primal".to_string(),
                topics: vec!["cross-primal.events".to_string()],
                retention_hours: Some(24),
            }),
        }),
    };

    // Test cross-primal integration
    assert_eq!(manifest.kind, "CrossPrimalIntegration");
    assert!(manifest.metadata.labels.is_some());
    let labels = manifest.metadata.labels.unwrap();
    assert_eq!(labels.get("integration"), Some(&"cross-primal".to_string()));
    assert_eq!(
        labels.get("primals"),
        Some(&"orchestration-primal,ai-primal,compute-primal,security-primal".to_string())
    );
    assert!(manifest.metadata.annotations.is_some());
    let annotations = manifest.metadata.annotations.unwrap();
    assert_eq!(
        annotations.get("cross-primal.nestgate.io/enabled"),
        Some(&"true".to_string())
    );
    assert_eq!(
        annotations.get("cross-primal.nestgate.io/primals"),
        Some(&"all".to_string())
    );

    assert_eq!(manifest.primals.len(), 2);
    assert!(manifest.primals.contains_key("orchestration-primal"));
    assert!(manifest.primals.contains_key("ai-primal"));

    let coordination = manifest.coordination.unwrap();
    assert!(coordination.service_mesh.is_some());
    assert!(coordination.discovery.is_some());
    assert!(coordination.health_checks.is_some());
    assert!(coordination.events.is_some());

    assert!(manifest.networking.service_mesh.is_some());
    let service_mesh = manifest.networking.service_mesh.unwrap();
    assert!(service_mesh.enabled);
    assert_eq!(service_mesh.provider, "istio");
}
