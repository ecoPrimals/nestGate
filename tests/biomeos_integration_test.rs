//! biomeOS Integration Tests
//!
//! Tests for biomeOS manifest parsing, volume provisioning, agent runtime support,
//! and universal coordination patterns

use nestgate_core::biomeos::{
    AgentSpec, BiomeContext, BiomeManifest, CoordinationConfig, DiscoveryConfig,
    EventCoordinationConfig, HealthChecksConfig, ResourceLimits, RetryConfig, SecurityContext,
    SecurityLevel, VolumeSpec,
};
use std::collections::HashMap;

#[tokio::test]
async fn test_enhanced_biome_manifest_parsing() {
    let yaml_content = r#"
apiVersion: biomeos/v1
kind: Biome
metadata:
  name: enhanced-test-biome
  version: "2.0.0"
  description: "Enhanced biome for testing universal patterns"
  author: "NestGate Team"
  labels:
    tier: "production"
    environment: "test"
  annotations:
    coordination.enabled: "true"
    universal.patterns: "true"

primals:
  nestgate:
    primal_type: nestgate
    version: "2.0.0"
    config:
      pool_name: "nestpool"
      encryption: true
      universal_storage: true
      real_time_sync: true
    dependencies:
      - "beardog"
      - "songbird"

  squirrel:
    primal_type: squirrel
    version: "1.5.0"
    config:
      mcp_enabled: true
      agent_runtime: true
      universal_coordination: true
    dependencies:
      - "nestgate"
      - "toadstool"

  toadstool:
    primal_type: toadstool
    version: "2.1.0"
    config:
      runtime_engines: ["wasm", "container", "native"]
      universal_execution: true
    dependencies:
      - "nestgate"

  songbird:
    primal_type: songbird
    version: "1.8.0"
    config:
      service_mesh: true
      universal_discovery: true
    dependencies: []

  beardog:
    primal_type: beardog
    version: "1.3.0"
    config:
      universal_security: true
      crypto_locks: true
    dependencies: []

services:
  nestgate-storage:
    service_type: storage
    version: "2.0.0"
    endpoints:
      - "http://nestgate:8080/api/v1/zfs"
      - "http://nestgate:8080/api/v1/zfs/biomeos"
      - "http://nestgate:8080/api/v1/agents/provision"
    capabilities:
      - "zfs-pools"
      - "tiered-storage"
      - "snapshots"
      - "volume-provisioning"
      - "agent-runtime-provisioning"
      - "universal-coordination"
    metadata:
      provider: "nestgate"
      service_id: "primal-nestgate-1"
      coordination_enabled: "true"
    health_checks:
      - name: "storage-health"
        path: "/health"
        expected_status: 200
      - name: "zfs-health"
        path: "/api/v1/zfs/health"
        expected_status: 200
    dependencies:
      - "beardog-security"
      - "songbird-discovery"

resources:
  storage:
    total_gb: 1000
    volumes:
      - name: ai-models
        size: "100Gi"
        tier: "hot"
        provisioner: "nestgate"
        mount_path: "/biome/ai-models"
        access_mode: "ReadWriteMany"
        protocols: ["nfs", "coordination"]
      - name: training-data
        size: "500Gi"
        tier: "warm"
        provisioner: "nestgate"
        mount_path: "/biome/training-data"
        access_mode: "ReadWriteMany"
        protocols: ["nfs", "smb", "coordination"]

security:
  security_level: High
  encryption_policies:
    at_rest: true
    in_transit: true
    key_rotation_days: 30
    provider: "beardog"
  access_controls:
    default_access: "deny"
    user_permissions:
      admin: ["read", "write", "delete"]
      user: ["read", "write"]
  audit_requirements:
    level: "detailed"
    retention_days: 90
    destinations: ["nestgate-audit", "beardog-audit"]

networking:
  service_mesh:
    enabled: true
    provider: "songbird"
    config:
      discovery_timeout: 30
      health_check_interval: 15
  load_balancing:
    strategy: "round_robin"
    health_check:
      interval_seconds: 30
      timeout_seconds: 10
      path: "/health"

storage:
  default_class: "zfs-hot"
  volumes:
    - name: ai-models
      size: "100Gi"
      tier: "hot"
      provisioner: "nestgate"
      mount_path: "/biome/ai-models"
      access_mode: "ReadWriteMany"
      protocols: ["nfs", "coordination"]
      options:
        compression: "lz4"
        deduplication: "off"

    - name: training-data
      size: "500Gi"
      tier: "warm"
      provisioner: "nestgate"
      mount_path: "/biome/training-data"
      access_mode: "ReadWriteMany"
      protocols: ["nfs", "smb", "coordination"]
      options:
        compression: "gzip"
        deduplication: "on"

# Enhanced agent support for Squirrel MCP integration
agents:
  - name: "data-analyst"
    runtime: "wasm"
    capabilities: ["data_analysis", "visualization", "reporting"]
    executor: "squirrel"
    resource_limits:
      memory_mb: 512
      cpu_percent: 25
      timeout_seconds: 300
    ai_provider: "openai"
    model: "gpt-4"
    env:
      OPENAI_API_KEY: "sk-test-key"
      LOG_LEVEL: "info"

  - name: "model-trainer"
    runtime: "container"
    capabilities: ["model_training", "data_processing", "gpu_acceleration"]
    executor: "toadstool"
    resource_limits:
      memory_mb: 4096
      cpu_percent: 80
      timeout_seconds: 3600
    ai_provider: "huggingface"
    model: "transformers/pytorch"
    env:
      HF_TOKEN: "hf-test-token"
      CUDA_VISIBLE_DEVICES: "0"

# Universal coordination configuration
coordination:
  service_mesh:
    enabled: true
    provider: "songbird"
    config:
      discovery_timeout: 30
      load_balancing: "round_robin"
      health_checks: true
  discovery:
    provider: "songbird"
    timeout_seconds: 30
    retry:
      max_retries: 3
      interval_seconds: 5
      backoff: "exponential"
  health_checks:
    interval_seconds: 30
    timeout_seconds: 10
    endpoints:
      - name: "nestgate-health"
        path: "/health"
        expected_status: 200
      - name: "agents-health"
        path: "/api/v1/agents/health"
        expected_status: 200
  events:
    provider: "songbird"
    topics: ["volume.provisioned", "agent.deployed", "storage.events"]
    retention_hours: 24

specialization:
  specialization_type: "ai-research"
  parameters:
    model_types: ["llm", "vision", "multimodal"]
    frameworks: ["pytorch", "tensorflow", "jax"]
    optimization_targets: ["throughput", "latency", "accuracy"]

templates:
  toadstool_runtime:
    - name: "pytorch-training"
      resources: "8Gi"
      config:
        image: "pytorch/pytorch:latest"
        gpu: true

  squirrel_agents:
    - name: "model-optimizer"
      resources: "4Gi"
      config:
        optimization_level: "aggressive"

  custom:
    nestgate_storage:
      - name: "high-performance-storage"
        resources: "100Gi"
        config:
          tier: "hot"
          compression: "lz4"
          deduplication: false
"#;

    let manifest = BiomeManifest::from_yaml(yaml_content).unwrap();

    // Test enhanced metadata
    assert_eq!(manifest.api_version, "biomeos/v1");
    assert_eq!(manifest.kind, "Biome");
    assert_eq!(manifest.metadata.name, "enhanced-test-biome");
    assert_eq!(manifest.metadata.version, "2.0.0");
    assert!(manifest.metadata.labels.is_some());
    assert!(manifest.metadata.annotations.is_some());

    // Test enhanced primal configurations
    assert_eq!(manifest.primals.len(), 5);
    let nestgate_primal = &manifest.primals["nestgate"];
    assert_eq!(nestgate_primal.primal_type, "nestgate");
    assert!(nestgate_primal.dependencies.is_some());
    assert!(nestgate_primal
        .dependencies
        .as_ref()
        .unwrap()
        .contains(&"beardog".to_string()));

    // Test agent specifications
    assert!(manifest.agents.is_some());
    let agents = manifest.agents.as_ref().unwrap();
    assert_eq!(agents.len(), 2);

    let data_analyst = &agents[0];
    assert_eq!(data_analyst.name, "data-analyst");
    assert_eq!(data_analyst.runtime, "wasm");
    assert_eq!(data_analyst.executor, "squirrel");
    assert!(data_analyst
        .capabilities
        .contains(&"data_analysis".to_string()));
    assert!(data_analyst.resource_limits.is_some());
    assert_eq!(data_analyst.ai_provider, Some("openai".to_string()));
    assert_eq!(data_analyst.model, Some("gpt-4".to_string()));

    let model_trainer = &agents[1];
    assert_eq!(model_trainer.name, "model-trainer");
    assert_eq!(model_trainer.runtime, "container");
    assert_eq!(model_trainer.executor, "toadstool");
    assert!(model_trainer
        .capabilities
        .contains(&"model_training".to_string()));

    // Test universal coordination configuration
    assert!(manifest.coordination.is_some());
    let coordination = manifest.coordination.as_ref().unwrap();
    assert!(coordination.service_mesh.is_some());
    assert!(coordination.discovery.is_some());
    assert!(coordination.health_checks.is_some());
    assert!(coordination.events.is_some());

    let discovery = coordination.discovery.as_ref().unwrap();
    assert_eq!(discovery.provider, "songbird");
    assert_eq!(discovery.timeout_seconds, Some(30));
    assert!(discovery.retry.is_some());

    let events = coordination.events.as_ref().unwrap();
    assert_eq!(events.provider, "songbird");
    assert_eq!(events.topics.len(), 3);
    assert!(events.topics.contains(&"volume.provisioned".to_string()));
    assert!(events.topics.contains(&"agent.deployed".to_string()));

    // Test storage configuration
    assert_eq!(manifest.storage.volumes.len(), 2);
    let ai_models_volume = &manifest.storage.volumes[0];
    assert_eq!(ai_models_volume.name, "ai-models");
    assert_eq!(ai_models_volume.size, "100Gi");
    assert_eq!(ai_models_volume.tier, "hot");
    assert_eq!(ai_models_volume.provisioner, "nestgate");
    assert!(ai_models_volume
        .protocols
        .contains(&"coordination".to_string()));

    // Test enhanced service configuration
    let nestgate_service = &manifest.services["nestgate-storage"];
    assert!(nestgate_service
        .capabilities
        .contains(&"agent-runtime-provisioning".to_string()));
    assert!(nestgate_service
        .capabilities
        .contains(&"universal-coordination".to_string()));
    assert!(nestgate_service.health_checks.is_some());
    assert!(nestgate_service.dependencies.is_some());

    // Test templates
    assert!(manifest.templates.is_some());
    let templates = manifest.templates.as_ref().unwrap();
    assert!(templates.toadstool_runtime.is_some());
    assert!(templates.squirrel_agents.is_some());
    assert!(templates.custom.is_some());

    let toadstool_templates = templates.toadstool_runtime.as_ref().unwrap();
    assert_eq!(toadstool_templates.len(), 1);
    assert_eq!(toadstool_templates[0].name, "pytorch-training");

    let squirrel_templates = templates.squirrel_agents.as_ref().unwrap();
    assert_eq!(squirrel_templates.len(), 1);
    assert_eq!(squirrel_templates[0].name, "model-optimizer");

    let custom_templates = templates.custom.as_ref().unwrap();
    assert!(custom_templates.contains_key("nestgate_storage"));
}

#[tokio::test]
async fn test_enhanced_biome_context_creation() {
    let biome_context = BiomeContext {
        biome_id: "test-biome-enhanced".to_string(),
        node_id: "node-001".to_string(),
        environment: "production".to_string(),
        security_context: SecurityContext {
            security_level: SecurityLevel::High,
            encryption_enabled: true,
            audit_enabled: true,
            access_controls: HashMap::new(),
        },
        resource_constraints: nestgate_core::biomeos::ResourceConstraints {
            max_cpu_cores: 16.0,
            max_memory_mb: 32768,
            max_storage_gb: 1000,
            max_network_bandwidth_mbps: 10000,
        },
        integration_endpoints: {
            let mut endpoints = HashMap::new();
            endpoints.insert("songbird".to_string(), "http://songbird:8080".to_string());
            endpoints.insert("beardog".to_string(), "http://beardog:8080".to_string());
            endpoints.insert("squirrel".to_string(), "http://squirrel:8080".to_string());
            endpoints.insert("toadstool".to_string(), "http://toadstool:8080".to_string());
            endpoints.insert(
                "coordination".to_string(),
                "http://coordination:8080".to_string(),
            );
            endpoints
        },
    };

    // Test enhanced biome context
    assert_eq!(biome_context.biome_id, "test-biome-enhanced");
    assert_eq!(biome_context.environment, "production");
    assert!(biome_context.security_context.encryption_enabled);
    assert!(biome_context.security_context.audit_enabled);
    assert_eq!(biome_context.resource_constraints.max_cpu_cores, 16.0);
    assert_eq!(biome_context.integration_endpoints.len(), 5);
    assert!(biome_context
        .integration_endpoints
        .contains_key("coordination"));
}

#[tokio::test]
async fn test_volume_provisioning_with_universal_patterns() {
    let volume_spec = VolumeSpec {
        name: "ai-training-data".to_string(),
        size: "1Ti".to_string(),
        tier: "warm".to_string(),
        provisioner: "nestgate".to_string(),
        mount_path: Some("/biome/ai-training".to_string()),
        access_mode: Some("ReadWriteMany".to_string()),
        protocols: vec![
            "nfs".to_string(),
            "smb".to_string(),
            "coordination".to_string(),
        ],
        options: Some({
            let mut options = HashMap::new();
            options.insert("compression".to_string(), "gzip".to_string());
            options.insert("deduplication".to_string(), "on".to_string());
            options.insert("coordination_enabled".to_string(), "true".to_string());
            options.insert("universal_patterns".to_string(), "true".to_string());
            options
        }),
    };

    // Test volume spec parsing
    assert_eq!(volume_spec.name, "ai-training-data");
    assert_eq!(volume_spec.size, "1Ti");
    assert_eq!(volume_spec.tier, "warm");
    assert_eq!(volume_spec.provisioner, "nestgate");
    assert!(volume_spec.protocols.contains(&"coordination".to_string()));
    assert!(volume_spec.options.is_some());

    // Test size parsing
    let size_bytes = volume_spec.size_bytes().unwrap();
    assert_eq!(size_bytes, 1024 * 1024 * 1024 * 1024u64); // 1Ti in bytes

    // Test tier parsing
    let storage_tier = volume_spec.storage_tier().unwrap();
    assert_eq!(storage_tier, nestgate_core::StorageTier::Warm);

    // Test options parsing
    let options = volume_spec.options.as_ref().unwrap();
    assert_eq!(options.get("compression"), Some(&"gzip".to_string()));
    assert_eq!(options.get("deduplication"), Some(&"on".to_string()));
    assert_eq!(
        options.get("coordination_enabled"),
        Some(&"true".to_string())
    );
    assert_eq!(options.get("universal_patterns"), Some(&"true".to_string()));
}

#[tokio::test]
async fn test_agent_runtime_specification() {
    let agent_spec = AgentSpec {
        name: "advanced-ai-agent".to_string(),
        runtime: "wasm".to_string(),
        capabilities: vec![
            "data_analysis".to_string(),
            "visualization".to_string(),
            "ml_training".to_string(),
            "real_time_inference".to_string(),
        ],
        executor: "squirrel".to_string(),
        resource_limits: Some(ResourceLimits {
            memory_mb: Some(1024),
            cpu_percent: Some(50),
            timeout_seconds: Some(600),
        }),
        ai_provider: Some("openai".to_string()),
        model: Some("gpt-4".to_string()),
        env: Some({
            let mut env = HashMap::new();
            env.insert("OPENAI_API_KEY".to_string(), "sk-test-key".to_string());
            env.insert("LOG_LEVEL".to_string(), "debug".to_string());
            env.insert("COORDINATION_ENABLED".to_string(), "true".to_string());
            env
        }),
    };

    // Test agent specification
    assert_eq!(agent_spec.name, "advanced-ai-agent");
    assert_eq!(agent_spec.runtime, "wasm");
    assert_eq!(agent_spec.executor, "squirrel");
    assert_eq!(agent_spec.capabilities.len(), 4);
    assert!(agent_spec
        .capabilities
        .contains(&"data_analysis".to_string()));
    assert!(agent_spec
        .capabilities
        .contains(&"real_time_inference".to_string()));

    // Test resource limits
    let resource_limits = agent_spec.resource_limits.as_ref().unwrap();
    assert_eq!(resource_limits.memory_mb, Some(1024));
    assert_eq!(resource_limits.cpu_percent, Some(50));
    assert_eq!(resource_limits.timeout_seconds, Some(600));

    // Test AI provider configuration
    assert_eq!(agent_spec.ai_provider, Some("openai".to_string()));
    assert_eq!(agent_spec.model, Some("gpt-4".to_string()));

    // Test environment variables
    let env = agent_spec.env.as_ref().unwrap();
    assert_eq!(env.get("OPENAI_API_KEY"), Some(&"sk-test-key".to_string()));
    assert_eq!(env.get("LOG_LEVEL"), Some(&"debug".to_string()));
    assert_eq!(env.get("COORDINATION_ENABLED"), Some(&"true".to_string()));
}

#[tokio::test]
async fn test_universal_coordination_configuration() {
    let coordination_config = CoordinationConfig {
        service_mesh: Some(nestgate_core::biomeos::ServiceMeshConfig {
            enabled: true,
            provider: "songbird".to_string(),
            config: {
                let mut config = HashMap::new();
                config.insert(
                    "discovery_timeout".to_string(),
                    serde_json::Value::Number(30.into()),
                );
                config.insert(
                    "load_balancing".to_string(),
                    serde_json::Value::String("round_robin".to_string()),
                );
                config.insert("health_checks".to_string(), serde_json::Value::Bool(true));
                config
            },
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
            timeout_seconds: 10,
            endpoints: vec![
                nestgate_core::biomeos::HealthCheckEndpoint {
                    name: "nestgate-health".to_string(),
                    path: "/health".to_string(),
                    expected_status: Some(200),
                },
                nestgate_core::biomeos::HealthCheckEndpoint {
                    name: "storage-health".to_string(),
                    path: "/api/v1/zfs/health".to_string(),
                    expected_status: Some(200),
                },
            ],
        }),
        events: Some(EventCoordinationConfig {
            provider: "songbird".to_string(),
            topics: vec![
                "volume.provisioned".to_string(),
                "agent.deployed".to_string(),
                "storage.events".to_string(),
                "coordination.events".to_string(),
            ],
            retention_hours: Some(24),
        }),
    };

    // Test service mesh configuration
    let service_mesh = coordination_config.service_mesh.as_ref().unwrap();
    assert!(service_mesh.enabled);
    assert_eq!(service_mesh.provider, "songbird");
    assert_eq!(service_mesh.config.len(), 3);

    // Test discovery configuration
    let discovery = coordination_config.discovery.as_ref().unwrap();
    assert_eq!(discovery.provider, "songbird");
    assert_eq!(discovery.timeout_seconds, Some(30));

    let retry = discovery.retry.as_ref().unwrap();
    assert_eq!(retry.max_retries, 3);
    assert_eq!(retry.interval_seconds, 5);
    assert_eq!(retry.backoff, Some("exponential".to_string()));

    // Test health checks configuration
    let health_checks = coordination_config.health_checks.as_ref().unwrap();
    assert_eq!(health_checks.interval_seconds, 30);
    assert_eq!(health_checks.timeout_seconds, 10);
    assert_eq!(health_checks.endpoints.len(), 2);
    assert_eq!(health_checks.endpoints[0].name, "nestgate-health");
    assert_eq!(health_checks.endpoints[1].path, "/api/v1/zfs/health");

    // Test event coordination configuration
    let events = coordination_config.events.as_ref().unwrap();
    assert_eq!(events.provider, "songbird");
    assert_eq!(events.topics.len(), 4);
    assert!(events.topics.contains(&"volume.provisioned".to_string()));
    assert!(events.topics.contains(&"agent.deployed".to_string()));
    assert!(events.topics.contains(&"coordination.events".to_string()));
    assert_eq!(events.retention_hours, Some(24));
}

#[tokio::test]
async fn test_primal_templates_with_universal_patterns() {
    let manifest = BiomeManifest::from_yaml(
        r#"
apiVersion: biomeos/v1
kind: Biome
metadata:
  name: template-test
  version: "1.0.0"
primals:
  nestgate:
    primal_type: nestgate
    version: "2.0.0"
    config: {}
services:
  test-service:
    service_type: test
    version: "1.0.0"
    endpoints: []
    capabilities: []
    metadata: {}
resources:
  storage:
    total_gb: 100
    volumes: []
security:
  security_level: Basic
networking:
  service_mesh:
    enabled: true
    provider: "songbird"
    config: {}
storage:
  volumes: []
templates:
  toadstool_runtime:
    - name: "pytorch-gpu-training"
      resources: "16Gi"
      config:
        image: "pytorch/pytorch:latest-gpu"
        gpu: true
        coordination_enabled: true
        universal_patterns: true

    - name: "tensorflow-distributed"
      resources: "32Gi"
      config:
        image: "tensorflow/tensorflow:latest-gpu"
        distributed: true
        coordination_enabled: true

  squirrel_agents:
    - name: "llm-inference-agent"
      resources: "8Gi"
      config:
        model_type: "llm"
        optimization_level: "high"
        coordination_enabled: true
        universal_patterns: true

    - name: "vision-analysis-agent"
      resources: "4Gi"
      config:
        model_type: "vision"
        optimization_level: "balanced"
        coordination_enabled: true

  custom:
    nestgate_storage:
      - name: "ultra-high-performance"
        resources: "500Gi"
        config:
          tier: "hot"
          nvme_cache: true
          compression: "lz4"
          coordination_enabled: true
          universal_patterns: true

    beardog_security:
      - name: "enterprise-encryption"
        resources: "10Gi"
        config:
          encryption_level: "enterprise"
          key_rotation: "daily"
          coordination_enabled: true
"#,
    )
    .unwrap();

    // Test Toadstool runtime templates
    let toadstool_templates = manifest.get_primal_templates("toadstool");
    assert_eq!(toadstool_templates.len(), 2);
    assert_eq!(toadstool_templates[0].name, "pytorch-gpu-training");
    assert_eq!(toadstool_templates[1].name, "tensorflow-distributed");

    // Test Squirrel agent templates
    let squirrel_templates = manifest.get_primal_templates("squirrel");
    assert_eq!(squirrel_templates.len(), 2);
    assert_eq!(squirrel_templates[0].name, "llm-inference-agent");
    assert_eq!(squirrel_templates[1].name, "vision-analysis-agent");

    // Test custom templates
    let nestgate_templates = manifest.get_primal_templates("nestgate_storage");
    assert_eq!(nestgate_templates.len(), 1);
    assert_eq!(nestgate_templates[0].name, "ultra-high-performance");

    let beardog_templates = manifest.get_primal_templates("beardog_security");
    assert_eq!(beardog_templates.len(), 1);
    assert_eq!(beardog_templates[0].name, "enterprise-encryption");

    // Test unknown templates
    let unknown_templates = manifest.get_primal_templates("unknown");
    assert_eq!(unknown_templates.len(), 0);
}

#[tokio::test]
async fn test_nestgate_volume_filtering() {
    let manifest = BiomeManifest::from_yaml(
        r#"
apiVersion: biomeos/v1
kind: Biome
metadata:
  name: volume-filter-test
  version: "1.0.0"
primals:
  nestgate:
    primal_type: nestgate
    version: "2.0.0"
    config: {}
services:
  test-service:
    service_type: test
    version: "1.0.0"
    endpoints: []
    capabilities: []
    metadata: {}
resources:
  storage:
    total_gb: 100
    volumes: []
security:
  security_level: Basic
networking:
  service_mesh:
    enabled: true
    provider: "songbird"
    config: {}
storage:
  volumes:
    - name: "nestgate-volume-1"
      size: "100Gi"
      tier: "hot"
      provisioner: "nestgate"
      protocols: ["nfs", "coordination"]

    - name: "other-volume"
      size: "50Gi"
      tier: "warm"
      provisioner: "other-provider"
      protocols: ["smb"]

    - name: "nestgate-volume-2"
      size: "200Gi"
      tier: "cold"
      provisioner: "nestgate"
      protocols: ["s3", "coordination"]

    - name: "third-party-volume"
      size: "75Gi"
      tier: "warm"
      provisioner: "third-party"
      protocols: ["nfs"]
"#,
    )
    .unwrap();

    // Test NestGate volume filtering
    let nestgate_volumes = manifest.get_nestgate_volumes();
    assert_eq!(nestgate_volumes.len(), 2);
    assert_eq!(nestgate_volumes[0].name, "nestgate-volume-1");
    assert_eq!(nestgate_volumes[1].name, "nestgate-volume-2");

    // Verify all returned volumes have nestgate as provisioner
    for volume in nestgate_volumes {
        assert_eq!(volume.provisioner, "nestgate");
        assert!(volume.protocols.contains(&"coordination".to_string()));
    }
}

#[tokio::test]
async fn test_integration_endpoints_validation() {
    let biome_context = BiomeContext {
        biome_id: "test-integration".to_string(),
        node_id: "node-001".to_string(),
        environment: "production".to_string(),
        security_context: SecurityContext {
            security_level: SecurityLevel::Enterprise,
            encryption_enabled: true,
            audit_enabled: true,
            access_controls: HashMap::new(),
        },
        resource_constraints: nestgate_core::biomeos::ResourceConstraints {
            max_cpu_cores: 32.0,
            max_memory_mb: 65536,
            max_storage_gb: 2000,
            max_network_bandwidth_mbps: 10000,
        },
        integration_endpoints: {
            let mut endpoints = HashMap::new();
            endpoints.insert("nestgate".to_string(), "http://nestgate:8080".to_string());
            endpoints.insert("songbird".to_string(), "http://songbird:8080".to_string());
            endpoints.insert("beardog".to_string(), "https://beardog:8443".to_string());
            endpoints.insert("squirrel".to_string(), "http://squirrel:8080".to_string());
            endpoints.insert("toadstool".to_string(), "http://toadstool:8080".to_string());
            endpoints.insert(
                "coordination".to_string(),
                "http://coordination:8080".to_string(),
            );
            endpoints.insert(
                "universal-storage".to_string(),
                "http://universal-storage:8080".to_string(),
            );
            endpoints.insert("events".to_string(), "ws://events:8080".to_string());
            endpoints
        },
    };

    // Test integration endpoints
    assert_eq!(biome_context.integration_endpoints.len(), 8);
    assert_eq!(
        biome_context.integration_endpoints.get("nestgate"),
        Some(&"http://nestgate:8080".to_string())
    );
    assert_eq!(
        biome_context.integration_endpoints.get("beardog"),
        Some(&"https://beardog:8443".to_string())
    );
    assert_eq!(
        biome_context.integration_endpoints.get("coordination"),
        Some(&"http://coordination:8080".to_string())
    );
    assert_eq!(
        biome_context.integration_endpoints.get("universal-storage"),
        Some(&"http://universal-storage:8080".to_string())
    );
    assert_eq!(
        biome_context.integration_endpoints.get("events"),
        Some(&"ws://events:8080".to_string())
    );

    // Test security context for enterprise level
    assert_eq!(
        biome_context.security_context.security_level,
        SecurityLevel::Enterprise
    );
    assert!(biome_context.security_context.encryption_enabled);
    assert!(biome_context.security_context.audit_enabled);

    // Test resource constraints for enterprise deployment
    assert_eq!(biome_context.resource_constraints.max_cpu_cores, 32.0);
    assert_eq!(biome_context.resource_constraints.max_memory_mb, 65536);
    assert_eq!(biome_context.resource_constraints.max_storage_gb, 2000);
    assert_eq!(
        biome_context
            .resource_constraints
            .max_network_bandwidth_mbps,
        10000
    );
}
