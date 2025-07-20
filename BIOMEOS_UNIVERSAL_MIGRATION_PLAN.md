# BiomeOS Universal Primal Architecture Migration Plan

**Target**: Migrate biomeOS from name-based to capability-based primal integration  
**Timeline**: 2-3 weeks  
**Impact**: Foundation for entire ecosystem universal architecture

---

## 🎯 **Migration Objectives**

1. **Replace Hardcoded Types**: Remove all hardcoded primal name strings
2. **Capability-Based Discovery**: Implement universal capability matching  
3. **Universal Integration**: Standardize primal provider interfaces
4. **Backward Compatibility**: Maintain existing functionality during transition

---

## 📋 **Implementation Plan**

### **Step 1: Update Core Primal System**

**File**: `biomeOS/crates/biomeos-core/src/primal.rs`

```rust
// ✅ REPLACE: Hardcoded PrimalType string
// OLD:
pub type PrimalType = String;

// NEW: 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalPrimalIdentity {
    /// Dynamic service identifier (UUID-based)
    pub service_id: Uuid,
    /// Human-readable service name (informational only)
    pub display_name: String,
    /// Provider category for grouping (not used for discovery)
    pub category: ServiceCategory,
    /// Core capabilities this service provides
    pub capabilities: Vec<Capability>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceCategory {
    /// Storage and data management services
    Storage,
    /// Orchestration and coordination services  
    Orchestration,
    /// AI and compute services
    Compute,
    /// Security and authentication services
    Security,
    /// Custom category with domain specification
    Custom { domain: String },
}

// ✅ ENHANCED: Universal Primal trait
#[async_trait]
pub trait UniversalPrimal: Send + Sync {
    /// Get unique service identity
    fn identity(&self) -> UniversalPrimalIdentity;
    
    /// Check if this primal supports a specific capability
    fn supports_capability(&self, capability: &str) -> bool {
        self.identity().capabilities
            .iter()
            .any(|cap| cap.name == capability)
    }
    
    /// Execute capability with universal parameter system
    async fn execute_capability(
        &self,
        capability: &str,
        parameters: CapabilityParameters,
    ) -> BiomeResult<CapabilityResponse>;
    
    /// Register for capability-based discovery
    async fn register_with_ecosystem(
        &self,
        discovery_service: &dyn CapabilityDiscoveryService,
    ) -> BiomeResult<()>;
    
    /// Health check with capability status
    async fn health_check(&self) -> BiomeResult<ServiceHealth>;
}
```

### **Step 2: Implement Universal Service Registry**

**File**: `biomeOS/crates/biomeos-core/src/universal_registry.rs`

```rust
//! Universal Service Registry - Capability-Based Discovery

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Universal service registry for capability-based discovery
#[derive(Debug)]
pub struct UniversalServiceRegistry {
    /// Services indexed by capability
    capability_index: Arc<RwLock<HashMap<String, Vec<Arc<dyn UniversalPrimal>>>>>,
    /// Services indexed by service ID  
    service_index: Arc<RwLock<HashMap<Uuid, Arc<dyn UniversalPrimal>>>>,
    /// Category groupings (for efficiency, not discovery)
    category_index: Arc<RwLock<HashMap<ServiceCategory, Vec<Uuid>>>>,
}

impl UniversalServiceRegistry {
    pub fn new() -> Self {
        Self {
            capability_index: Arc::new(RwLock::new(HashMap::new())),
            service_index: Arc::new(RwLock::new(HashMap::new())),
            category_index: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a service by its capabilities (replaces name-based registration)
    pub async fn register_service(&self, service: Arc<dyn UniversalPrimal>) -> BiomeResult<()> {
        let identity = service.identity();
        let service_id = identity.service_id;
        
        // Index by each capability
        let mut cap_index = self.capability_index.write().await;
        for capability in &identity.capabilities {
            cap_index
                .entry(capability.name.clone())
                .or_insert_with(Vec::new)
                .push(service.clone());
        }
        
        // Index by service ID
        self.service_index.write().await.insert(service_id, service.clone());
        
        // Index by category (for grouping)
        self.category_index.write().await
            .entry(identity.category)
            .or_insert_with(Vec::new)
            .push(service_id);
            
        info!("Registered service {} with capabilities: {:?}", 
              identity.display_name, 
              identity.capabilities.iter().map(|c| &c.name).collect::<Vec<_>>());
        
        Ok(())
    }

    /// Find services by required capability (universal discovery)
    pub async fn find_services_with_capability(&self, capability: &str) -> Vec<Arc<dyn UniversalPrimal>> {
        self.capability_index
            .read()
            .await
            .get(capability)
            .cloned()
            .unwrap_or_default()
    }

    /// Find best service for capability with load balancing
    pub async fn get_best_service_for_capability(&self, capability: &str) -> Option<Arc<dyn UniversalPrimal>> {
        let services = self.find_services_with_capability(capability).await;
        
        if services.is_empty() {
            return None;
        }

        // TODO: Implement health-based selection similar to nestgate
        // For now, return first available service
        services.into_iter().next()
    }

    /// Legacy compatibility: find service by name (deprecated)
    #[deprecated(note = "Use find_services_with_capability instead")]
    pub async fn get_primal(&self, name: &str) -> Option<Arc<dyn UniversalPrimal>> {
        warn!("Using deprecated get_primal - migrate to capability-based discovery");
        
        // Try to map common legacy names to capabilities
        let capability = match name {
            "toadstool" => "compute.execution",
            "songbird" => "orchestration.service_coordination", 
            "nestgate" => "storage.zfs_management",
            "beardog" => "security.authentication",
            _ => return None,
        };
        
        self.get_best_service_for_capability(capability).await
    }
}
```

### **Step 3: Update BiomeOS Core Integration**

**File**: `biomeOS/crates/biomeos-core/src/biome.rs`

```rust
// ✅ REPLACE: Hardcoded primal management
impl Biome {
    // OLD: Hardcoded primal registration
    // pub fn add_primal(&mut self, primal: Box<dyn Primal>) -> BiomeResult<()>
    
    // NEW: Universal capability-based registration
    pub async fn register_service(&self, service: Arc<dyn UniversalPrimal>) -> BiomeResult<()> {
        self.service_registry.register_service(service).await
    }

    // OLD: Name-based primal lookup
    // pub fn get_primal(&self, primal_type: &str) -> Option<&dyn Primal>
    
    // NEW: Capability-based service discovery
    pub async fn get_service_for_capability(&self, capability: &str) -> Option<Arc<dyn UniversalPrimal>> {
        self.service_registry.get_best_service_for_capability(capability).await
    }

    /// Execute workflow step with universal capability system
    pub async fn execute_workflow_step(&self, step: &WorkflowStep) -> BiomeResult<WorkflowResult> {
        match &step.action {
            WorkflowAction::RequiresCapability { capability, parameters } => {
                // Universal capability-based execution
                if let Some(service) = self.get_service_for_capability(capability).await {
                    service.execute_capability(capability, parameters.clone()).await
                        .map(|response| WorkflowResult::Success { 
                            service_id: service.identity().service_id,
                            response 
                        })
                } else {
                    Err(BiomeError::CapabilityNotAvailable {
                        capability: capability.clone(),
                        available_services: self.list_available_capabilities().await,
                    })
                }
            }
        }
    }
}
```

### **Step 4: Migrate Ecosystem Integration**

**File**: `biomeOS/crates/biomeos-core/src/ecosystem_integration.rs`

```rust
// ✅ REPLACE: Hardcoded primal type enum with capability-based discovery

// OLD:
// pub primal_type: PrimalType,

// NEW:
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalServiceRegistration {
    /// Dynamic service identifier (replaces hardcoded names)
    pub service_id: Uuid,
    /// Service identity information
    pub identity: UniversalPrimalIdentity,
    /// Capabilities this service provides
    pub capabilities: Vec<ServiceCapability>,
    /// Service endpoints
    pub endpoints: EcosystemEndpoints,
    /// Health and metrics configuration
    pub health_config: HealthCheckConfig,
    /// Registration timestamp
    pub registered_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceCapability {
    /// Capability identifier (e.g., "storage.zfs", "ai.text_generation")
    pub name: String,
    /// Capability version
    pub version: semver::Version,
    /// Performance characteristics
    pub performance: PerformanceProfile,
    /// Resource requirements
    pub resources: ResourceRequirements,
    /// Supported parameters
    pub parameters: Vec<CapabilityParameter>,
}

/// Universal ecosystem coordinator (replaces hardcoded primal coordination)
#[derive(Debug)]
pub struct UniversalEcosystemCoordinator {
    service_registry: Arc<UniversalServiceRegistry>,
    capability_router: Arc<CapabilityRouter>,
    health_monitor: Arc<HealthMonitor>,
}

impl UniversalEcosystemCoordinator {
    /// Coordinate workflow execution across discovered services
    pub async fn execute_workflow(&self, workflow: EcosystemWorkflow) -> BiomeResult<WorkflowResult> {
        let mut results = Vec::new();
        
        for step in workflow.steps {
            // Find service by capability requirement, not hardcoded name
            let service = self.service_registry
                .get_best_service_for_capability(&step.required_capability)
                .await
                .ok_or_else(|| BiomeError::CapabilityNotAvailable {
                    capability: step.required_capability.clone(),
                    available_services: vec![], // TODO: List available
                })?;
            
            let result = service.execute_capability(
                &step.required_capability,
                step.parameters
            ).await?;
            
            results.push(result);
        }
        
        Ok(WorkflowResult::Success { results })
    }
}
```

### **Step 5: Create Universal Adapters for Existing Primals**

**File**: `biomeOS/crates/biomeos-core/src/primal_adapters.rs`

```rust
//! Universal adapters for existing primal services during migration

/// Adapter for ToadStool compute service
pub struct ToadStoolUniversalAdapter {
    toadstool_client: ToadStoolClient,
    identity: UniversalPrimalIdentity,
}

#[async_trait]
impl UniversalPrimal for ToadStoolUniversalAdapter {
    fn identity(&self) -> UniversalPrimalIdentity {
        self.identity.clone()
    }
    
    async fn execute_capability(
        &self,
        capability: &str,
        parameters: CapabilityParameters,
    ) -> BiomeResult<CapabilityResponse> {
        match capability {
            "compute.wasm_execution" => {
                // Translate universal parameters to ToadStool-specific format
                let toadstool_request = self.translate_wasm_request(parameters)?;
                let result = self.toadstool_client.execute_wasm(toadstool_request).await?;
                Ok(CapabilityResponse::WasmExecution { result })
            },
            "compute.container_orchestration" => {
                let container_request = self.translate_container_request(parameters)?;
                let result = self.toadstool_client.deploy_container(container_request).await?;
                Ok(CapabilityResponse::ContainerDeployment { result })
            },
            _ => Err(BiomeError::UnsupportedCapability { 
                capability: capability.to_string(),
                supported: vec!["compute.wasm_execution", "compute.container_orchestration"],
            })
        }
    }
}

/// Factory for creating universal adapters from legacy integrations
pub struct UniversalAdapterFactory;

impl UniversalAdapterFactory {
    /// Create universal adapter for discovered ToadStool instance
    pub async fn create_toadstool_adapter(endpoint: &str) -> BiomeResult<Arc<dyn UniversalPrimal>> {
        let client = ToadStoolClient::new(endpoint).await?;
        
        let identity = UniversalPrimalIdentity {
            service_id: Uuid::new_v4(),
            display_name: "ToadStool Compute Service".to_string(),
            category: ServiceCategory::Compute,
            capabilities: vec![
                Capability {
                    name: "compute.wasm_execution".to_string(),
                    version: "1.0.0".to_string(),
                    description: "WebAssembly execution environment".to_string(),
                    parameters: Self::wasm_parameters(),
                },
                Capability {
                    name: "compute.container_orchestration".to_string(),
                    version: "1.0.0".to_string(), 
                    description: "Container deployment and management".to_string(),
                    parameters: Self::container_parameters(),
                },
            ],
        };
        
        Ok(Arc::new(ToadStoolUniversalAdapter {
            toadstool_client: client,
            identity,
        }))
    }
}
```

---

## 🧪 **Testing Strategy**

### **Migration Tests**
```rust
#[tokio::test]
async fn test_universal_service_registration() {
    let registry = UniversalServiceRegistry::new();
    
    // Test capability-based registration
    let mock_service = create_mock_compute_service().await;
    registry.register_service(mock_service).await.unwrap();
    
    // Test capability-based discovery
    let services = registry.find_services_with_capability("compute.wasm_execution").await;
    assert!(!services.is_empty());
    
    // Test legacy compatibility (deprecated)
    let legacy_service = registry.get_primal("toadstool").await;
    assert!(legacy_service.is_some());
}

#[tokio::test] 
async fn test_ecosystem_workflow_execution() {
    let coordinator = UniversalEcosystemCoordinator::new().await;
    
    let workflow = EcosystemWorkflow {
        steps: vec![
            WorkflowStep {
                required_capability: "storage.dataset_create".to_string(),
                parameters: CapabilityParameters::new()
                    .with("name", "test-dataset")
                    .with("size", "1GB"),
            },
            WorkflowStep {
                required_capability: "compute.wasm_execution".to_string(), 
                parameters: CapabilityParameters::new()
                    .with("module", "data-processor.wasm")
                    .with("input", "test-dataset"),
            },
        ],
    };
    
    let result = coordinator.execute_workflow(workflow).await;
    assert!(result.is_ok());
}
```

---

## 📊 **Migration Timeline**

| Week | Task | Deliverables |
|------|------|-------------|
| **Week 1** | Universal Primal System | `UniversalPrimal` trait, `UniversalServiceRegistry` |
| **Week 2** | Adapter Creation | Universal adapters for ToadStool, Songbird, NestGate |
| **Week 3** | Integration & Testing | Full capability-based workflows, compatibility testing |

---

## 🎯 **Success Metrics**

- ✅ Zero hardcoded primal name references in production code
- ✅ All services discoverable by capability, not name
- ✅ Legacy compatibility maintained during transition  
- ✅ Universal workflow execution working end-to-end
- ✅ Performance equivalent or better than name-based system

This migration will establish biomeOS as the **universal ecosystem coordinator** that can work with any primal service, regardless of implementation details! 