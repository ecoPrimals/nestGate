# 🌍 Ecosystem Integration Guide
## Using NestGate's Unified Capability System

**Target Audience**: Other primal development teams  
**Purpose**: How to integrate with NestGate using unified capabilities  
**Last Updated**: December 13, 2025

---

## 🎯 OVERVIEW

NestGate provides a **unified capability discovery system** that enables:
- ✅ Runtime discovery of services by capability (not by name)
- ✅ Type-safe capability matching
- ✅ Pluggable discovery backends
- ✅ Primal sovereignty (no hardcoded dependencies)
- ✅ Graceful degradation (optional integrations)

---

## 🏗️ ARCHITECTURE

### **Core Components**

1. **UnifiedCapability** - Single enum for all capability types
2. **CapabilityResolver** - Trait for discovery implementations
3. **CapabilityMapper** - Bidirectional capability translation
4. **ResolvedService** - Discovery result with endpoint info

### **Location**

```
code/crates/nestgate-core/src/
├── unified_capabilities.rs   - Capability definitions
└── capability_resolver.rs    - Discovery interface
```

---

## 🚀 QUICK START

### **1. Add Dependency**

```toml
# Cargo.toml
[dependencies]
nestgate-core = { path = "../nestgate/code/crates/nestgate-core" }
```

### **2. Discover NestGate Storage**

```rust
use nestgate_core::{UnifiedCapability, CapabilityResolver};
use nestgate_core::unified_capabilities::StorageCapability;

async fn find_storage<R: CapabilityResolver>(
    resolver: &R
) -> Result<ResolvedService> {
    // Discover any service offering file storage
    resolver
        .resolve_capability(&UnifiedCapability::Storage(
            StorageCapability::FileSystem
        ))
        .await
}

// Usage
let resolver = create_resolver(); // Your resolver implementation
let storage = find_storage(&resolver).await?;

println!("Found storage at: {}", storage.url());
// Output: Found storage at: http://nestgate.local:8080
```

### **3. Discover Multiple Services**

```rust
async fn find_all_storage<R: CapabilityResolver>(
    resolver: &R
) -> Result<Vec<ResolvedService>> {
    // Find all services offering object storage
    resolver
        .resolve_capability_all(&UnifiedCapability::Storage(
            StorageCapability::ObjectStorage
        ))
        .await
}

// Usage
let all_storage = find_all_storage(&resolver).await?;
for service in all_storage {
    println!("Storage: {} at {}", service.id, service.url());
}
```

---

## 📚 CAPABILITY TYPES

### **Available Capabilities**

```rust
pub enum UnifiedCapability {
    // Storage capabilities
    Storage(StorageCapability),
    
    // Networking capabilities
    Networking(NetworkingCapability),
    
    // Security capabilities
    Security(SecurityCapability),
    
    // AI capabilities
    AI(AICapability),
    
    // Orchestration capabilities
    Orchestration(OrchestrationCapability),
    
    // Custom capabilities
    CustomTaxonomy(String),
    CustomPrimal(String),
}
```

### **Storage Capabilities**

```rust
pub enum StorageCapability {
    FileSystem,           // POSIX filesystem
    ObjectStorage,        // S3-like
    BlockStorage,         // iSCSI, NBD
    NetworkFileSystem,    // NFS, SMB
    Backup,              // Backup services
    Archive,             // Cold storage
}
```

### **Networking Capabilities**

```rust
pub enum NetworkingCapability {
    RestApi,             // HTTP REST API
    GraphQL,             // GraphQL endpoint
    gRPC,                // gRPC service
    WebSocket,           // WebSocket server
    ServiceMesh,         // Service mesh node
}
```

### **Orchestration Capabilities**

```rust
pub enum OrchestrationCapability {
    TaskManagement,      // Task scheduling
    WorkflowEngine,      // Workflow orchestration
    ResourceAllocation,  // Resource management
    DistributedExecution,// Distributed computing
}
```

---

## 🔌 IMPLEMENTING CAPABILITYRESOLVER

### **Basic Implementation**

```rust
use nestgate_core::{CapabilityResolver, UnifiedCapability, ResolvedService};
use std::future::Future;
use std::pin::Pin;

pub struct MyServiceRegistry {
    services: HashMap<UnifiedCapability, Vec<ResolvedService>>,
}

impl CapabilityResolver for MyServiceRegistry {
    fn resolve_capability(
        &self,
        capability: &UnifiedCapability,
    ) -> Pin<Box<dyn Future<Output = Result<ResolvedService>> + Send + '_>> {
        let capability = capability.clone();
        Box::pin(async move {
            self.services
                .get(&capability)
                .and_then(|services| services.first())
                .cloned()
                .ok_or_else(|| {
                    NestGateError::internal_error(
                        &format!("No service found for capability: {:?}", capability),
                        "my_resolver"
                    )
                })
        })
    }

    fn resolve_capability_all(
        &self,
        capability: &UnifiedCapability,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ResolvedService>>> + Send + '_>> {
        let capability = capability.clone();
        Box::pin(async move {
            self.services
                .get(&capability)
                .cloned()
                .ok_or_else(|| {
                    NestGateError::internal_error(
                        &format!("No services found for capability: {:?}", capability),
                        "my_resolver"
                    )
                })
        })
    }
}
```

---

## 🎯 COMMON PATTERNS

### **Pattern 1: Discover with Fallback**

```rust
use nestgate_core::capability_resolver::CompositeResolver;

async fn discover_with_fallback() -> Result<ResolvedService> {
    // Try registry first, then environment variables
    let resolver = CompositeResolver::new()
        .with_resolver(Box::new(my_registry))
        .with_resolver(Box::new(EnvironmentResolver::new()));
    
    resolver
        .resolve_capability(&UnifiedCapability::Storage(
            StorageCapability::FileSystem
        ))
        .await
}
```

### **Pattern 2: Capability-Based Configuration**

```rust
pub struct MyServiceConfig {
    storage_capability: UnifiedCapability,
    orchestration_capability: UnifiedCapability,
}

impl MyServiceConfig {
    pub async fn resolve_endpoints<R: CapabilityResolver>(
        &self,
        resolver: &R
    ) -> Result<(ResolvedService, ResolvedService)> {
        let storage = resolver.resolve_capability(&self.storage_capability).await?;
        let orchestrator = resolver.resolve_capability(&self.orchestration_capability).await?;
        Ok((storage, orchestrator))
    }
}
```

### **Pattern 3: Primal Self-Registration**

```rust
use nestgate_core::universal_primal_discovery::ServiceRegistry;

pub struct MyPrimal {
    registry: ServiceRegistry,
}

impl MyPrimal {
    pub async fn register_capabilities(&self) -> Result<()> {
        // Register what capabilities THIS primal provides
        self.registry.register(
            "my-primal-compute",
            vec![
                UnifiedCapability::Orchestration(
                    OrchestrationCapability::TaskManagement
                ),
                UnifiedCapability::Orchestration(
                    OrchestrationCapability::DistributedExecution
                ),
            ],
            "http://my-primal.local:9000"
        ).await
    }
    
    pub async fn discover_storage<R: CapabilityResolver>(
        &self,
        resolver: &R
    ) -> Result<ResolvedService> {
        // Discover OTHER primals by capability (not by name!)
        resolver
            .resolve_capability(&UnifiedCapability::Storage(
                StorageCapability::FileSystem
            ))
            .await
    }
}
```

---

## 🏛️ SOVEREIGNTY PRINCIPLES

### **DO** ✅

1. **Know Only Yourself**
   ```rust
   // ✅ GOOD: Register your own capabilities
   registry.register("my-service", my_capabilities, my_endpoint).await;
   ```

2. **Discover Others at Runtime**
   ```rust
   // ✅ GOOD: Discover by capability
   let storage = resolver.resolve_capability(&storage_capability).await?;
   ```

3. **Use Graceful Degradation**
   ```rust
   // ✅ GOOD: Optional integration
   let storage = resolver.resolve_capability(&cap).await.ok();
   if let Some(storage) = storage {
       // Use storage
   } else {
       // Work without it
   }
   ```

### **DON'T** ❌

1. **Hardcode Primal Names**
   ```rust
   // ❌ BAD: Hardcoded primal reference
   let nestgate_url = "http://nestgate.local:8080";
   ```

2. **Hardcode Ports**
   ```rust
   // ❌ BAD: Hardcoded port
   const STORAGE_PORT: u16 = 8080;
   ```

3. **Force Dependencies**
   ```rust
   // ❌ BAD: Required primal integration
   let storage = resolver.resolve_capability(&cap).await.expect("NestGate must be available");
   ```

---

## 🧪 TESTING

### **Unit Tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use nestgate_core::capability_resolver::EnvironmentResolver;

    #[tokio::test]
    async fn test_discover_storage() {
        std::env::set_var(
            "NESTGATE_CAPABILITY_STORAGE_FILESYSTEM_ENDPOINT",
            "http://test-storage:8080"
        );
        
        let resolver = EnvironmentResolver::new();
        let storage = resolver
            .resolve_capability(&UnifiedCapability::Storage(
                StorageCapability::FileSystem
            ))
            .await;
        
        assert!(storage.is_ok());
        let service = storage.unwrap();
        assert_eq!(service.host, "test-storage");
        assert_eq!(service.port, 8080);
    }
}
```

### **Integration Tests**

```rust
#[tokio::test]
async fn test_cross_primal_discovery() {
    // Start NestGate (or use mock)
    let nestgate = start_nestgate_test_instance().await;
    
    // Create resolver pointing to test registry
    let resolver = create_test_resolver(nestgate.registry_url);
    
    // Discover NestGate's storage
    let storage = resolver
        .resolve_capability(&UnifiedCapability::Storage(
            StorageCapability::FileSystem
        ))
        .await?;
    
    // Verify discovery worked
    assert_eq!(storage.id, "nestgate-storage");
    assert!(storage.is_healthy);
    
    // Try using the discovered endpoint
    let client = reqwest::Client::new();
    let response = client.get(&storage.url()).send().await?;
    assert!(response.status().is_success());
}
```

---

## 🔧 TROUBLESHOOTING

### **Problem: Service Not Found**

```rust
Err(DiscoveryError::ServiceNotFound { 
    capability: "Storage(FileSystem)" 
})
```

**Solutions**:
1. Check service is registered: `registry.list_all().await`
2. Verify capability match exactly
3. Check environment variables if using `EnvironmentResolver`
4. Confirm network connectivity

### **Problem: Multiple Services Found**

```rust
// Use resolve_capability_all to see all matches
let all = resolver.resolve_capability_all(&cap).await?;
for service in all {
    println!("Found: {} at {}", service.id, service.url());
}

// Select specific one
let preferred = all.into_iter()
    .find(|s| s.id.contains("primary"))
    .ok_or("No primary service")?;
```

### **Problem: Capability Type Mismatch**

```rust
// ❌ Wrong enum variant
UnifiedCapability::Storage(StorageCapability::ObjectStorage)

// ✅ Correct variant
UnifiedCapability::Storage(StorageCapability::FileSystem)
```

---

## 📊 EXAMPLES

### **Example 1: SongBird Discovering NestGate**

```rust
// In SongBird codebase
use nestgate_core::{CapabilityResolver, UnifiedCapability};
use nestgate_core::unified_capabilities::StorageCapability;

pub struct SongBirdOrchestrator<R: CapabilityResolver> {
    resolver: R,
}

impl<R: CapabilityResolver> SongBirdOrchestrator<R> {
    pub async fn provision_storage(&self) -> Result<String> {
        // Discover storage by capability (not by name!)
        let storage = self.resolver
            .resolve_capability(&UnifiedCapability::Storage(
                StorageCapability::FileSystem
            ))
            .await?;
        
        // Use discovered storage
        let mount_point = format!("nfs://{}:{}/data", storage.host, storage.port);
        Ok(mount_point)
    }
}
```

### **Example 2: ToadStool Using NestGate + SongBird**

```rust
// In ToadStool codebase
pub struct ToadStoolCompute<R: CapabilityResolver> {
    resolver: R,
}

impl<R: CapabilityResolver> ToadStoolCompute<R> {
    pub async fn execute_task(&self, task: Task) -> Result<TaskResult> {
        // Discover orchestrator
        let orchestrator = self.resolver
            .resolve_capability(&UnifiedCapability::Orchestration(
                OrchestrationCapability::TaskManagement
            ))
            .await?;
        
        // Discover storage
        let storage = self.resolver
            .resolve_capability(&UnifiedCapability::Storage(
                StorageCapability::FileSystem
            ))
            .await?;
        
        // Execute task with discovered services
        let result = execute_with_storage(
            &task,
            &orchestrator.url(),
            &storage.url()
        ).await?;
        
        Ok(result)
    }
}
```

---

## 🎓 BEST PRACTICES

### **1. Always Use Capabilities, Never Names**

```rust
// ✅ GOOD
let service = resolver.resolve_capability(&capability).await?;

// ❌ BAD
let service = find_service_by_name("nestgate").await?;
```

### **2. Make Integrations Optional**

```rust
// ✅ GOOD
let storage = resolver.resolve_capability(&cap).await.ok();

// ❌ BAD
let storage = resolver.resolve_capability(&cap).await.expect("Must have NestGate");
```

### **3. Provide Clear Error Messages**

```rust
// ✅ GOOD
Err(DiscoveryError::ServiceNotFound {
    capability: "Storage(FileSystem)".to_string(),
})

// ❌ BAD
Err("Not found".into())
```

### **4. Document Capability Requirements**

```rust
/// Requires a service providing Storage(FileSystem) capability
/// 
/// If no storage is available, returns Err(DiscoveryError::ServiceNotFound)
pub async fn process_files<R: CapabilityResolver>(
    resolver: &R
) -> Result<()> {
    // ...
}
```

---

## 📚 FURTHER READING

- `DEEP_MODERNIZATION_SESSION_COMPLETE.md` - Architectural details
- `SOVEREIGNTY_MOCK_VERIFICATION_DEC_13_2025.md` - Sovereignty compliance
- `unified_capabilities.rs` - Full capability definitions
- `capability_resolver.rs` - Resolver implementation details

---

## 🆘 SUPPORT

**Questions?** Check:
1. This guide's examples
2. `code/crates/nestgate-core/src/config/port_migration.rs` - Real usage example
3. Test files in `code/crates/nestgate-core/src/capability_resolver.rs`

**Issues?** File in NestGate repository with:
- Capability you're trying to discover
- Error message received
- Your resolver implementation
- Expected vs actual behavior

---

**Last Updated**: December 13, 2025  
**Version**: 1.0 (Unified Capabilities)  
**Status**: Production Ready ✅

*"Discover by capability, not by name."* 🌍✨

