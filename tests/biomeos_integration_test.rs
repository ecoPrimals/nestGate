//! biomeOS Integration Tests
//!
//! Tests for biomeOS manifest parsing and volume provisioning

use nestgate_core::biomeos::{
    BiomeContext, BiomeManifest, SecurityContext, SecurityLevel, VolumeSpec,
};
use std::collections::HashMap;

#[tokio::test]
async fn test_biome_manifest_parsing() {
    let yaml_content = r#"
apiVersion: biomeos/v1
kind: Biome
metadata:
  name: test-biome
  version: "1.0.0"
  description: "Test biome for integration testing"
  author: "Test Author"

primals:
  nestgate:
    primal_type: nestgate
    version: "2.0.0"
    config:
      pool_name: "testpool"
      encryption: true

services:
  test-service:
    service_type: storage
    version: "1.0.0"
    endpoints: []
    capabilities: []
    metadata: {}

resources:
  storage:
    total_gb: 100
    volumes:
      - name: test-volume
        size: "100Gi"
        tier: "hot"
        provisioner: "nestgate"
        mount_path: "/test/volume"
        access_mode: "ReadWriteOnce"

security:
  security_level: Standard
  
networking:
  service_mesh:
    enabled: false
    provider: "none"
    config: {}

storage:
  volumes:
    - name: test-volume
      size: "100Gi"
      tier: "hot"
      provisioner: "nestgate"
      mount_path: "/test/volume"
      access_mode: "ReadWriteOnce"
"#;

    let manifest = BiomeManifest::from_yaml(yaml_content).unwrap();

    assert_eq!(manifest.metadata.name, "test-biome");
    assert_eq!(manifest.metadata.version, "1.0.0");
    assert_eq!(manifest.storage.volumes.len(), 1);

    let volume = &manifest.storage.volumes[0];
    assert_eq!(volume.name, "test-volume");
    assert_eq!(volume.size, "100Gi");
    assert_eq!(volume.tier, "hot");
    assert_eq!(volume.provisioner, "nestgate");
}

#[tokio::test]
async fn test_volume_spec_size_parsing() {
    let volume_spec = VolumeSpec {
        name: "test-volume".to_string(),
        size: "100Gi".to_string(),
        tier: "hot".to_string(),
        provisioner: "nestgate".to_string(),
        mount_path: Some("/test/path".to_string()),
        access_mode: Some("ReadWriteOnce".to_string()),
        options: None,
    };

    let size_bytes = volume_spec.size_bytes().unwrap();
    assert_eq!(size_bytes, 100 * 1024 * 1024 * 1024); // 100 GiB in bytes

    let tier = volume_spec.storage_tier().unwrap();
    assert_eq!(tier, nestgate_core::StorageTier::Hot);
}

#[tokio::test]
async fn test_biome_context_creation() {
    let biome_context = BiomeContext {
        biome_id: "test-biome".to_string(),
        node_id: "test-node".to_string(),
        environment: "development".to_string(),
        security_context: SecurityContext {
            user_id: "test-user".to_string(),
            auth_token: Some("test-token".to_string()),
            permissions: vec!["read".to_string(), "write".to_string()],
            security_level: SecurityLevel::Standard,
        },
        resource_constraints: nestgate_core::biomeos::ResourceConstraints {
            max_cpu_cores: Some(8.0),
            max_memory_mb: Some(16384),
            max_storage_gb: Some(1000),
            quotas: HashMap::new(),
        },
        integration_endpoints: HashMap::new(),
    };

    assert_eq!(biome_context.biome_id, "test-biome");
    assert_eq!(biome_context.environment, "development");
    assert_eq!(biome_context.security_context.user_id, "test-user");
}

#[tokio::test]
async fn test_biome_templates() {
    let yaml_content = r#"
apiVersion: biomeos/v1
kind: Biome
metadata:
  name: template-test-biome
  version: "1.0.0"

primals:
  toadstool:
    primal_type: toadstool
    version: "1.0.0"
    config: {}

services:
  test-service:
    service_type: compute
    version: "1.0.0"
    endpoints: []
    capabilities: []
    metadata: {}

resources:
  storage:
    total_gb: 10
    volumes: []

storage:
  volumes: []

templates:
  toadstool_runtime:
    - name: "pytorch-training"
      resources: "8Gi"
      config:
        image: "pytorch/pytorch:latest"
        gpu: true
        
  squirrel_agents:
    - name: "model-optimizer"
      resources: "16Gi"
      config:
        optimization_level: "aggressive"

security:
  security_level: Basic
  
networking:
  service_mesh:
    enabled: false
    provider: "none"
    config: {}
"#;

    let manifest = BiomeManifest::from_yaml(yaml_content).unwrap();

    let toadstool_templates = manifest.get_primal_templates("toadstool");
    assert_eq!(toadstool_templates.len(), 1);
    assert_eq!(toadstool_templates[0].name, "pytorch-training");

    let squirrel_templates = manifest.get_primal_templates("squirrel");
    assert_eq!(squirrel_templates.len(), 1);
    assert_eq!(squirrel_templates[0].name, "model-optimizer");

    let unknown_templates = manifest.get_primal_templates("unknown");
    assert_eq!(unknown_templates.len(), 0);
}

#[tokio::test]
async fn test_nestgate_volume_filtering() {
    let yaml_content = r#"
apiVersion: biomeos/v1
kind: Biome
metadata:
  name: volume-filter-test
  version: "1.0.0"

primals:
  nestgate:
    primal_type: nestgate
    version: "1.0.0"
    config: {}

services:
  test-service:
    service_type: storage
    version: "1.0.0"
    endpoints: []
    capabilities: []
    metadata: {}

resources:
  storage:
    total_gb: 150
    volumes:
      - name: nestgate-volume
        size: "100Gi"
        tier: "hot"
        provisioner: "nestgate"
        
      - name: other-volume
        size: "50Gi"
        tier: "warm"
        provisioner: "other-provider"

storage:
  volumes:
    - name: nestgate-volume
      size: "100Gi"
      tier: "hot"
      provisioner: "nestgate"
      
    - name: other-volume
      size: "50Gi"
      tier: "warm"
      provisioner: "other-provider"

security:
  security_level: Basic
  
networking:
  service_mesh:
    enabled: false
    provider: "none"
    config: {}
"#;

    let manifest = BiomeManifest::from_yaml(yaml_content).unwrap();

    let nestgate_volumes = manifest.get_nestgate_volumes();
    assert_eq!(nestgate_volumes.len(), 1);
    assert_eq!(nestgate_volumes[0].name, "nestgate-volume");
    assert_eq!(nestgate_volumes[0].provisioner, "nestgate");
}
