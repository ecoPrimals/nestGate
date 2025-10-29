//! Comprehensive BiomeOS Configuration Tests
//!
//! These tests provide extensive coverage of NestGate's BiomeOS configuration system,
//! focusing on service discovery, capability configuration, and manifest parsing.

use std::collections::HashMap;
use nestgate_core::{
    biomeos::{
        BiomeManifest, ServiceConfig, CapabilityConfig, ResourceRequirements, 
        DiscoveryPreferences, BiomeMetadata
    },
    constants::canonical::network::DEFAULT_API_PORT,
    Result,
};

/// Test comprehensive ServiceConfig functionality
#[test]
fn test_service_config_comprehensive() -> Result<(), Box<dyn std::error::Error>> {
    // Test minimal service config
    let minimal_config = ServiceConfig {
        service_type: "storage".to_string(),
        version: "1.0.0".to_string(),
        endpoints: vec![format!("http://storage:{}", DEFAULT_API_PORT)],
        capabilities: vec!["file_system".to_string()],
        metadata: HashMap::new(),
        health_checks: None,
        dependencies: None,
    };
    
    // Test field access
    assert_eq!(minimal_config.service_type, "storage");
    assert_eq!(minimal_config.version, "1.0.0");
    assert_eq!(minimal_config.endpoints.len(), 1);
    assert_eq!(minimal_config.capabilities.len(), 1);
    assert!(minimal_config.metadata.is_empty());
    assert!(minimal_config.health_checks.is_none());
    assert!(minimal_config.dependencies.is_none());
    
    // Test full service config
    let mut metadata = HashMap::new();
    metadata.insert("environment".to_string(), "production".to_string());
    metadata.insert("region".to_string(), "us-west-2".to_string());
    
    let full_config = ServiceConfig {
        service_type: "universal_adapter".to_string(),
        version: "2.1.0".to_string(),
        endpoints: vec![
            format!("http://primary:{}", DEFAULT_API_PORT),
            format!("http://secondary:{}", DEFAULT_API_PORT),
            format!("http://tertiary:{}", DEFAULT_API_PORT),
        ],
        capabilities: vec![
            "routing".to_string(),
            "discovery".to_string(),
            "load_balancing".to_string(),
            "health_monitoring".to_string(),
        ],
        metadata: metadata.clone(),
        health_checks: Some(vec!["/health".to_string(), "/ready".to_string()]),
        dependencies: Some(vec!["storage".to_string(), "security".to_string(), "monitoring".to_string()]),
    };
    
    // Test all fields
    assert_eq!(full_config.service_type, "universal_adapter");
    assert_eq!(full_config.version, "2.1.0");
    assert_eq!(full_config.endpoints.len(), 3);
    assert_eq!(full_config.capabilities.len(), 4);
    assert_eq!(full_config.metadata.len(), 2);
    assert_eq!(full_config.health_checks.as_ref()?.len(), 2);
    assert_eq!(full_config.dependencies.as_ref()?.len(), 3);
    
    // Test specific values
    assert!(full_config.endpoints.contains(&format!("http://primary:{}", DEFAULT_API_PORT)));
    assert!(full_config.capabilities.contains(&"routing".to_string()));
    assert_eq!(full_config.metadata.get("environment")?, "production");
    assert!(full_config.health_checks.as_ref()?.contains(&"/health".to_string()));
    assert!(full_config.dependencies.as_ref()?.contains(&"storage".to_string()));
    
    // Test clone functionality
    let cloned = full_config.clone();
    assert_eq!(full_config.service_type, cloned.service_type);
    assert_eq!(full_config.endpoints.len(), cloned.endpoints.len());
    assert_eq!(full_config.metadata, cloned.metadata);
}

/// Test comprehensive CapabilityConfig functionality
#[test]
fn test_capability_config_comprehensive() -> Result<(), Box<dyn std::error::Error>> {
    // Test minimal capability config
    let minimal_config = CapabilityConfig {
        capability_type: "basic_storage".to_string(),
        config: HashMap::new(),
        resources: None,
        discovery: None,
    };
    
    assert_eq!(minimal_config.capability_type, "basic_storage");
    assert!(minimal_config.config.is_empty());
    assert!(minimal_config.resources.is_none());
    assert!(minimal_config.discovery.is_none());
    
    // Test full capability config
    let mut config_map = HashMap::new();
    config_map.insert("provider_type".to_string(), serde_json::Value::String("universal".to_string()));
    config_map.insert("max_connections".to_string(), serde_json::Value::Number(serde_json::Number::from(100)));
    config_map.insert("enable_caching".to_string(), serde_json::Value::Bool(true));
    config_map.insert("cache_ttl".to_string(), serde_json::Value::Number(serde_json::Number::from(3600)));
    
    let full_config = CapabilityConfig {
        capability_type: "advanced_storage".to_string(),
        config: config_map.clone(),
        resources: Some(ResourceRequirements {
            cpu: Some("4".to_string()),
            memory: Some("8Gi".to_string()),
            storage: Some("500Gi".to_string()),
            custom: Some({
                let mut custom = HashMap::new();
                custom.insert("gpu".to_string(), "1".to_string());
                custom.insert("network_bandwidth".to_string(), "10Gbps".to_string());
                custom
            }),
        }),
        discovery: Some(DiscoveryPreferences {
            preferred_regions: Some(vec!["us-west-2".to_string(), "us-east-1".to_string()]),
            required_capabilities: Some(vec!["high_performance".to_string(), "encryption".to_string()]),
            excluded_providers: Some(vec!["legacy_provider".to_string()]),
            priority_scoring: Some({
                let mut scoring = HashMap::new();
                scoring.insert("latency".to_string(), 0.4);
                scoring.insert("reliability".to_string(), 0.6);
                scoring
            }),
        }),
    };
    
    // Test capability type
    assert_eq!(full_config.capability_type, "advanced_storage");
    
    // Test config values
    assert_eq!(full_config.config.len(), 4);
    assert_eq!(
        full_config.config.get("provider_type")?.as_str()?,
        "universal"
    );
    assert_eq!(
        full_config.config.get("max_connections")?.as_u64()?,
        100
    );
    assert_eq!(
        full_config.config.get("enable_caching")?.as_bool()?,
        true
    );
    assert_eq!(
        full_config.config.get("cache_ttl")?.as_u64()?,
        3600
    );
    
    // Test resource requirements
    let resources = full_config.resources.as_ref()?;
    assert_eq!(resources.cpu.as_ref()?, "4");
    assert_eq!(resources.memory.as_ref()?, "8Gi");
    assert_eq!(resources.storage.as_ref()?, "500Gi");
    
    let custom = resources.custom.as_ref()?;
    assert_eq!(custom.get("gpu")?, "1");
    assert_eq!(custom.get("network_bandwidth")?, "10Gbps");
    
    // Test discovery preferences
    let discovery = full_config.discovery.as_ref()?;
    assert_eq!(discovery.preferred_regions.as_ref()?.len(), 2);
    assert_eq!(discovery.required_capabilities.as_ref()?.len(), 2);
    assert_eq!(discovery.excluded_providers.as_ref()?.len(), 1);
    assert_eq!(discovery.priority_scoring.as_ref()?.len(), 2);
    
    assert!(discovery.preferred_regions.as_ref()?.contains(&"us-west-2".to_string()));
    assert!(discovery.required_capabilities.as_ref()?.contains(&"encryption".to_string()));
    assert!(discovery.excluded_providers.as_ref()?.contains(&"legacy_provider".to_string()));
    assert_eq!(*discovery.priority_scoring.as_ref()?.get("latency")?, 0.4);
}

/// Test comprehensive ResourceRequirements functionality
#[test]
fn test_resource_requirements_comprehensive() -> Result<(), Box<dyn std::error::Error>> {
    // Test minimal resource requirements
    let minimal = ResourceRequirements {
        cpu: None,
        memory: None,
        storage: None,
        custom: None,
    };
    
    assert!(minimal.cpu.is_none());
    assert!(minimal.memory.is_none());
    assert!(minimal.storage.is_none());
    assert!(minimal.custom.is_none());
    
    // Test standard resource requirements
    let standard = ResourceRequirements {
        cpu: Some("2".to_string()),
        memory: Some("4Gi".to_string()),
        storage: Some("100Gi".to_string()),
        custom: None,
    };
    
    assert_eq!(standard.cpu.as_ref()?, "2");
    assert_eq!(standard.memory.as_ref()?, "4Gi");
    assert_eq!(standard.storage.as_ref()?, "100Gi");
    assert!(standard.custom.is_none());
    
    // Test custom resource requirements
    let mut custom_resources = HashMap::new();
    custom_resources.insert("gpu".to_string(), "2".to_string());
    custom_resources.insert("fpga".to_string(), "1".to_string());
    custom_resources.insert("network_bandwidth".to_string(), "40Gbps".to_string());
    custom_resources.insert("iops".to_string(), "10000".to_string());
    
    let custom = ResourceRequirements {
        cpu: Some("16".to_string()),
        memory: Some("64Gi".to_string()),
        storage: Some("2Ti".to_string()),
        custom: Some(custom_resources.clone()),
    };
    
    assert_eq!(custom.cpu.as_ref()?, "16");
    assert_eq!(custom.memory.as_ref()?, "64Gi");
    assert_eq!(custom.storage.as_ref()?, "2Ti");
    
    let custom_map = custom.custom.as_ref()?;
    assert_eq!(custom_map.len(), 4);
    assert_eq!(custom_map.get("gpu")?, "2");
    assert_eq!(custom_map.get("fpga")?, "1");
    assert_eq!(custom_map.get("network_bandwidth")?, "40Gbps");
    assert_eq!(custom_map.get("iops")?, "10000");
    
    // Test clone functionality
    let cloned = custom.clone();
    assert_eq!(custom.cpu, cloned.cpu);
    assert_eq!(custom.memory, cloned.memory);
    assert_eq!(custom.storage, cloned.storage);
    assert_eq!(custom.custom, cloned.custom);
}

/// Test comprehensive DiscoveryPreferences functionality
#[test]
fn test_discovery_preferences_comprehensive() -> Result<(), Box<dyn std::error::Error>> {
    // Test minimal discovery preferences
    let minimal = DiscoveryPreferences {
        preferred_regions: None,
        required_capabilities: None,
        excluded_providers: None,
        priority_scoring: None,
    };
    
    assert!(minimal.preferred_regions.is_none());
    assert!(minimal.required_capabilities.is_none());
    assert!(minimal.excluded_providers.is_none());
    assert!(minimal.priority_scoring.is_none());
    
    // Test full discovery preferences
    let mut scoring = HashMap::new();
    scoring.insert("performance".to_string(), 0.3);
    scoring.insert("cost".to_string(), 0.2);
    scoring.insert("reliability".to_string(), 0.5);
    
    let full = DiscoveryPreferences {
        preferred_regions: Some(vec![
            "us-west-1".to_string(),
            "us-west-2".to_string(),
            "us-east-1".to_string(),
            "eu-west-1".to_string(),
        ]),
        required_capabilities: Some(vec![
            "encryption_at_rest".to_string(),
            "encryption_in_transit".to_string(),
            "backup_support".to_string(),
            "monitoring".to_string(),
        ]),
        excluded_providers: Some(vec![
            "unreliable_provider".to_string(),
            "deprecated_service".to_string(),
        ]),
        priority_scoring: Some(scoring.clone()),
    };
    
    // Test preferred regions
    let regions = full.preferred_regions.as_ref()?;
    assert_eq!(regions.len(), 4);
    assert!(regions.contains(&"us-west-1".to_string()));
    assert!(regions.contains(&"eu-west-1".to_string()));
    
    // Test required capabilities
    let capabilities = full.required_capabilities.as_ref()?;
    assert_eq!(capabilities.len(), 4);
    assert!(capabilities.contains(&"encryption_at_rest".to_string()));
    assert!(capabilities.contains(&"monitoring".to_string()));
    
    // Test excluded providers
    let excluded = full.excluded_providers.as_ref()?;
    assert_eq!(excluded.len(), 2);
    assert!(excluded.contains(&"unreliable_provider".to_string()));
    assert!(excluded.contains(&"deprecated_service".to_string()));
    
    // Test priority scoring
    let priority = full.priority_scoring.as_ref()?;
    assert_eq!(priority.len(), 3);
    assert_eq!(*priority.get("performance")?, 0.3);
    assert_eq!(*priority.get("cost")?, 0.2);
    assert_eq!(*priority.get("reliability")?, 0.5);
    
    // Test that scores sum to 1.0
    let total_score: f64 = priority.values().sum();
    assert!((total_score - 1.0).abs() < 0.001);
    
    // Test clone functionality
    let cloned = full.clone();
    assert_eq!(full.preferred_regions, cloned.preferred_regions);
    assert_eq!(full.required_capabilities, cloned.required_capabilities);
    assert_eq!(full.excluded_providers, cloned.excluded_providers);
    assert_eq!(full.priority_scoring, cloned.priority_scoring);
}

/// Test comprehensive BiomeMetadata functionality
#[test]
fn test_biome_metadata_comprehensive() -> Result<(), Box<dyn std::error::Error>> {
    // Test minimal metadata
    let minimal = BiomeMetadata {
        name: "test_biome".to_string(),
        version: "1.0.0".to_string(),
        description: Some("Test biome description".to_string()),
        labels: None,
        annotations: None,
    };
    
    assert_eq!(minimal.name, "test_biome");
    assert_eq!(minimal.version, "1.0.0");
    assert_eq!(minimal.description.as_ref()?, "Test biome description");
    assert!(minimal.labels.is_none());
    assert!(minimal.annotations.is_none());
    
    // Test full metadata
    let mut labels = HashMap::new();
    labels.insert("environment".to_string(), "production".to_string());
    labels.insert("team".to_string(), "platform".to_string());
    labels.insert("service_tier".to_string(), "critical".to_string());
    
    let mut annotations = HashMap::new();
    annotations.insert("deployment.timestamp".to_string(), "2025-01-30T10:00:00Z".to_string());
    annotations.insert("deployment.version".to_string(), "v2.1.0".to_string());
    annotations.insert("contact.team".to_string(), "platform@company.com".to_string());
    annotations.insert("documentation.url".to_string(), "https://docs.company.com/biome".to_string());
    
    let full = BiomeMetadata {
        name: "production_biome".to_string(),
        version: "2.1.0".to_string(),
        description: Some("Production biome with full capabilities".to_string()),
        labels: Some(labels.clone()),
        annotations: Some(annotations.clone()),
    };
    
    // Test basic fields
    assert_eq!(full.name, "production_biome");
    assert_eq!(full.version, "2.1.0");
    assert_eq!(full.description.as_ref()?, "Production biome with full capabilities");
    
    // Test labels
    let metadata_labels = full.labels.as_ref()?;
    assert_eq!(metadata_labels.len(), 3);
    assert_eq!(metadata_labels.get("environment")?, "production");
    assert_eq!(metadata_labels.get("team")?, "platform");
    assert_eq!(metadata_labels.get("service_tier")?, "critical");
    
    // Test annotations
    let metadata_annotations = full.annotations.as_ref()?;
    assert_eq!(metadata_annotations.len(), 4);
    assert_eq!(metadata_annotations.get("deployment.timestamp")?, "2025-01-30T10:00:00Z");
    assert_eq!(metadata_annotations.get("contact.team")?, "platform@company.com");
    assert!(metadata_annotations.get("documentation.url")?.contains("docs.company.com"));
    
    // Test clone functionality
    let cloned = full.clone();
    assert_eq!(full.name, cloned.name);
    assert_eq!(full.version, cloned.version);
    assert_eq!(full.labels, cloned.labels);
    assert_eq!(full.annotations, cloned.annotations);
}

/// Test comprehensive BiomeManifest functionality
#[test]
fn test_biome_manifest_comprehensive() -> Result<(), Box<dyn std::error::Error>> {
    // Create test metadata
    let mut labels = HashMap::new();
    labels.insert("env".to_string(), "test".to_string());
    
    let metadata = BiomeMetadata {
        name: "test_manifest".to_string(),
        version: "1.0.0".to_string(),
        description: Some("Test manifest".to_string()),
        labels: Some(labels),
        annotations: None,
    };
    
    // Create test service config
    let service_config = ServiceConfig {
        service_type: "test_service".to_string(),
        version: "1.0.0".to_string(),
        endpoints: vec![format!("http://test:{}", DEFAULT_API_PORT)],
        capabilities: vec!["testing".to_string()],
        metadata: HashMap::new(),
        health_checks: None,
        dependencies: None,
    };
    
    // Create test capability config
    let mut capability_config_map = HashMap::new();
    capability_config_map.insert("test_key".to_string(), serde_json::Value::String("test_value".to_string()));
    
    let capability_config = CapabilityConfig {
        capability_type: "test_capability".to_string(),
        config: capability_config_map,
        resources: None,
        discovery: None,
    };
    
    // Create full manifest
    let manifest = BiomeManifest {
        metadata: metadata.clone(),
        services: vec![service_config.clone()],
        capabilities: vec![capability_config.clone()],
    };
    
    // Test metadata
    assert_eq!(manifest.metadata.name, "test_manifest");
    assert_eq!(manifest.metadata.version, "1.0.0");
    
    // Test services
    assert_eq!(manifest.services.len(), 1);
    assert_eq!(manifest.services[0].service_type, "test_service");
    assert_eq!(manifest.services[0].endpoints.len(), 1);
    
    // Test capabilities
    assert_eq!(manifest.capabilities.len(), 1);
    assert_eq!(manifest.capabilities[0].capability_type, "test_capability");
    assert_eq!(manifest.capabilities[0].config.len(), 1);
    
    // Test clone functionality
    let cloned = manifest.clone();
    assert_eq!(manifest.metadata.name, cloned.metadata.name);
    assert_eq!(manifest.services.len(), cloned.services.len());
    assert_eq!(manifest.capabilities.len(), cloned.capabilities.len());
    
    // Test complex manifest with multiple services and capabilities
    let mut complex_services = Vec::new();
    let mut complex_capabilities = Vec::new();
    
    // Add multiple services
    for i in 0..3 {
        let service = ServiceConfig {
            service_type: format!("service_{}", i),
            version: "1.0.0".to_string(),
            endpoints: vec![format!("http://service{}:{}", i, DEFAULT_API_PORT)],
            capabilities: vec![format!("capability_{}", i)],
            metadata: HashMap::new(),
            health_checks: None,
            dependencies: None,
        };
        complex_services.push(service);
    }
    
    // Add multiple capabilities
    for i in 0..3 {
        let mut config = HashMap::new();
        config.insert(format!("key_{}", i), serde_json::Value::String(format!("value_{}", i)));
        
        let capability = CapabilityConfig {
            capability_type: format!("capability_{}", i),
            config,
            resources: None,
            discovery: None,
        };
        complex_capabilities.push(capability);
    }
    
    let complex_manifest = BiomeManifest {
        metadata: metadata.clone(),
        services: complex_services,
        capabilities: complex_capabilities,
    };
    
    // Test complex manifest
    assert_eq!(complex_manifest.services.len(), 3);
    assert_eq!(complex_manifest.capabilities.len(), 3);
    
    // Test that each service has unique properties
    for (i, service) in complex_manifest.services.iter().enumerate() {
        assert_eq!(service.service_type, format!("service_{}", i));
        assert_eq!(service.endpoints[0], format!("http://service{}:{}", i, DEFAULT_API_PORT));
    }
    
    // Test that each capability has unique properties
    for (i, capability) in complex_manifest.capabilities.iter().enumerate() {
        assert_eq!(capability.capability_type, format!("capability_{}", i));
        assert_eq!(
            capability.config.get(&format!("key_{}", i))?.as_str()?,
            format!("value_{}", i)
        );
    }
} 