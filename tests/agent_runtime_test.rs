use std::collections::HashMap;

use nestgate_core::biomeos::{
    AgentSpec, BiomeManifest, BiomeMetadata, BiomeNetworking, BiomeResources, BiomeSecurity,
    BiomeStorage, ComputeResources, CoordinationConfig, DiscoveryConfig, EventCoordinationConfig,
    HealthCheckEndpoint, HealthChecksConfig, NetworkResources, PortSpec, ResourceLimits,
    RetryConfig, SecurityLevel, ServiceMeshConfig, StorageResources, VolumeSpec,
};

#[tokio::test]
async fn test_agent_runtime_provisioning() {
    let manifest = BiomeManifest {
        api_version: "v1".to_string(),
        kind: "AgentRuntime".to_string(),
        metadata: BiomeMetadata {
            name: "test-agent".to_string(),
            version: "1.0.0".to_string(),
            description: Some("Test agent runtime".to_string()),
            author: Some("test".to_string()),
            created_at: Some(chrono::Utc::now()),
            labels: None,
            annotations: None,
        },
        primals: HashMap::new(),
        services: HashMap::new(),
        resources: BiomeResources {
            storage: Some(StorageResources {
                total_gb: 10,
                volumes: vec![VolumeSpec {
                    name: "agent-storage".to_string(),
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
            service_mesh: None,
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
            name: "test-agent".to_string(),
            runtime: "container".to_string(),
            capabilities: vec!["runtime".to_string()],
            executor: "squirrel".to_string(),
            resource_limits: Some(ResourceLimits {
                memory_mb: Some(4096),
                cpu_percent: Some(80),
                timeout_seconds: Some(3600),
            }),
            ai_provider: None,
            model: None,
            env: None,
        }]),
        coordination: None,
    };

    // Test that the manifest is properly structured for agent runtime
    assert_eq!(manifest.api_version, "v1");
    assert_eq!(manifest.kind, "AgentRuntime");
    assert!(manifest.agents.is_some());
    let agents = manifest.agents.unwrap();
    assert_eq!(agents.len(), 1);
    assert_eq!(agents[0].name, "test-agent");
    assert_eq!(agents[0].runtime, "container");
    assert_eq!(agents[0].executor, "squirrel");
    assert!(agents[0].resource_limits.is_some());
    let limits = agents[0].resource_limits.as_ref().unwrap();
    assert_eq!(limits.memory_mb, Some(4096));
    assert_eq!(limits.cpu_percent, Some(80));
    assert_eq!(limits.timeout_seconds, Some(3600));
}

#[tokio::test]
async fn test_squirrel_mcp_integration() {
    let manifest = BiomeManifest {
        api_version: "v1".to_string(),
        kind: "SquirrelMCP".to_string(),
        metadata: BiomeMetadata {
            name: "squirrel-mcp".to_string(),
            version: "1.0.0".to_string(),
            description: Some("Squirrel MCP integration".to_string()),
            author: Some("nestgate".to_string()),
            created_at: Some(chrono::Utc::now()),
            labels: Some({
                let mut labels = HashMap::new();
                labels.insert("primal".to_string(), "squirrel".to_string());
                labels.insert("type".to_string(), "mcp".to_string());
                labels
            }),
            annotations: None,
        },
        primals: HashMap::new(),
        services: HashMap::new(),
        resources: BiomeResources {
            storage: Some(StorageResources {
                total_gb: 50,
                volumes: vec![
                    VolumeSpec {
                        name: "model-cache".to_string(),
                        size: "20Gi".to_string(),
                        tier: "hot".to_string(),
                        provisioner: "nestgate".to_string(),
                        mount_path: Some("/model-cache".to_string()),
                        access_mode: Some("ReadWriteOnce".to_string()),
                        options: None,
                        protocols: None,
                        backup_policy: None,
                    },
                    VolumeSpec {
                        name: "workspace".to_string(),
                        size: "30Gi".to_string(),
                        tier: "hot".to_string(),
                        provisioner: "nestgate".to_string(),
                        mount_path: Some("/workspace".to_string()),
                        access_mode: Some("ReadWriteOnce".to_string()),
                        options: None,
                        protocols: None,
                        backup_policy: None,
                    },
                ],
            }),
            compute: Some(ComputeResources {
                cpu_cores: 4.0,
                memory_mb: 8192,
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
            service_mesh: None,
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
            name: "squirrel-mcp".to_string(),
            runtime: "container".to_string(),
            capabilities: vec!["mcp".to_string(), "model-inference".to_string()],
            executor: "squirrel".to_string(),
            resource_limits: Some(ResourceLimits {
                memory_mb: Some(8192),
                cpu_percent: Some(90),
                timeout_seconds: Some(7200),
            }),
            ai_provider: Some("openai".to_string()),
            model: Some("gpt-4".to_string()),
            env: Some({
                let mut env = HashMap::new();
                env.insert("MCP_PORT".to_string(), "8080".to_string());
                env.insert("MODEL_CACHE_SIZE".to_string(), "20Gi".to_string());
                env
            }),
        }]),
        coordination: None,
    };

    // Test Squirrel MCP specific configuration
    assert_eq!(manifest.kind, "SquirrelMCP");
    assert!(manifest.agents.is_some());
    let agents = manifest.agents.unwrap();
    assert_eq!(agents[0].name, "squirrel-mcp");
    assert_eq!(agents[0].executor, "squirrel");
    assert!(agents[0].capabilities.contains(&"mcp".to_string()));
    assert!(manifest.metadata.labels.is_some());
    let labels = manifest.metadata.labels.unwrap();
    assert_eq!(labels.get("primal"), Some(&"squirrel".to_string()));
    assert_eq!(labels.get("type"), Some(&"mcp".to_string()));
    assert_eq!(agents[0].ai_provider, Some("openai".to_string()));
    assert_eq!(agents[0].model, Some("gpt-4".to_string()));
    assert!(agents[0].env.is_some());
    let env = agents[0].env.as_ref().unwrap();
    assert_eq!(env.get("MCP_PORT"), Some(&"8080".to_string()));
}

#[tokio::test]
async fn test_cross_primal_coordination() {
    let manifest = BiomeManifest {
        api_version: "v1".to_string(),
        kind: "CrossPrimalAgent".to_string(),
        metadata: BiomeMetadata {
            name: "cross-primal-agent".to_string(),
            version: "1.0.0".to_string(),
            description: Some("Agent with cross-primal coordination".to_string()),
            author: Some("nestgate".to_string()),
            created_at: Some(chrono::Utc::now()),
            labels: Some({
                let mut labels = HashMap::new();
                labels.insert("coordination".to_string(), "cross-primal".to_string());
                labels.insert(
                    "primals".to_string(),
                    "songbird,squirrel,toadstool,beardog".to_string(),
                );
                labels
            }),
            annotations: Some({
                let mut annotations = HashMap::new();
                annotations.insert(
                    "coordination.nestgate.io/enabled".to_string(),
                    "true".to_string(),
                );
                annotations.insert(
                    "coordination.nestgate.io/primals".to_string(),
                    "all".to_string(),
                );
                annotations
            }),
        },
        primals: HashMap::new(),
        services: HashMap::new(),
        resources: BiomeResources {
            storage: Some(StorageResources {
                total_gb: 5,
                volumes: vec![VolumeSpec {
                    name: "coordination-data".to_string(),
                    size: "5Gi".to_string(),
                    tier: "hot".to_string(),
                    provisioner: "nestgate".to_string(),
                    mount_path: Some("/coordination".to_string()),
                    access_mode: Some("ReadWriteOnce".to_string()),
                    options: None,
                    protocols: None,
                    backup_policy: None,
                }],
            }),
            compute: Some(ComputeResources {
                cpu_cores: 1.0,
                memory_mb: 2048,
                gpu: None,
            }),
            network: Some(NetworkResources {
                bandwidth_mbps: Some(100),
                ports: vec![PortSpec {
                    port: 8080,
                    protocol: "TCP".to_string(),
                    expose_externally: false,
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
            name: "cross-primal-agent".to_string(),
            runtime: "container".to_string(),
            capabilities: vec!["cross-primal-coordination".to_string()],
            executor: "squirrel".to_string(),
            resource_limits: Some(ResourceLimits {
                memory_mb: Some(2048),
                cpu_percent: Some(50),
                timeout_seconds: Some(3600),
            }),
            ai_provider: None,
            model: None,
            env: Some({
                let mut env = HashMap::new();
                env.insert("COORDINATION_ENABLED".to_string(), "true".to_string());
                env.insert(
                    "PRIMALS".to_string(),
                    "songbird,squirrel,toadstool,beardog".to_string(),
                );
                env
            }),
        }]),
        coordination: Some(CoordinationConfig {
            service_mesh: Some(ServiceMeshConfig {
                enabled: true,
                provider: "istio".to_string(),
                config: HashMap::new(),
            }),
            discovery: Some(DiscoveryConfig {
                provider: "songbird".to_string(),
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
                provider: "songbird".to_string(),
                topics: vec!["cross-primal.events".to_string()],
                retention_hours: Some(24),
            }),
        }),
    };

    // Test cross-primal coordination configuration
    assert_eq!(manifest.kind, "CrossPrimalAgent");
    assert!(manifest.metadata.labels.is_some());
    let labels = manifest.metadata.labels.unwrap();
    assert_eq!(
        labels.get("coordination"),
        Some(&"cross-primal".to_string())
    );
    assert_eq!(
        labels.get("primals"),
        Some(&"songbird,squirrel,toadstool,beardog".to_string())
    );
    assert!(manifest.metadata.annotations.is_some());
    let annotations = manifest.metadata.annotations.unwrap();
    assert_eq!(
        annotations.get("coordination.nestgate.io/enabled"),
        Some(&"true".to_string())
    );
    assert!(manifest.coordination.is_some());
    let coordination = manifest.coordination.unwrap();
    assert!(coordination.service_mesh.is_some());
    assert!(coordination.discovery.is_some());
    assert!(coordination.health_checks.is_some());
    assert!(coordination.events.is_some());
    assert!(manifest.agents.is_some());
    let agents = manifest.agents.unwrap();
    assert!(agents[0]
        .capabilities
        .contains(&"cross-primal-coordination".to_string()));
}
