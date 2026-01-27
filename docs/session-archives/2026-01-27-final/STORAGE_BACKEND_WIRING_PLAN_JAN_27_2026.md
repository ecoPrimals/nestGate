# 🏗️ Storage Backend Wiring Plan - January 27, 2026

**Purpose**: Complete architecture plan for wiring tarpc RPC to StorageManagerService  
**Priority**: Week 3-4 remaining work  
**Estimated**: 6-10 hours  
**Blocker**: rustup issue (prevents testing, must fix first)  
**Grade Impact**: A+ (95.0) → A+ (96.0) when complete

---

## 📊 **CURRENT STATE**

### **Phase 1: In-Memory Storage** (Current)

**File**: `code/crates/nestgate-core/src/rpc/tarpc_server.rs`

```rust
pub struct NestGateRpcService {
    /// In-memory datasets (DashMap for lock-free concurrency)
    datasets: Arc<DashMap<String, DatasetInfo>>,
    
    /// In-memory objects (DashMap for lock-free concurrency)
    objects: Arc<DashMap<String, ObjectStorage>>,
    
    /// Start time for uptime
    start_time: SystemTime,
}
```

**Status**: ✅ Works, but ephemeral (data lost on restart)

---

### **Phase 2: Real Storage** (Target)

**File**: `code/crates/nestgate-core/src/services/storage/service.rs`

```rust
pub struct StorageManagerService {
    service_id: Uuid,
    zfs_config: ZfsConfig,
    pools: StoragePoolMap,
    quotas: StorageQuotaMap,
    cache_configs: CacheConfigMap,
    stats: Arc<RwLock<StorageServiceStats>>,
    config: StorageServiceConfig,
    // Real ZFS integration available
}
```

**Status**: ✅ Implemented with real ZFS support, needs wiring

---

## 🎯 **ARCHITECTURE PLAN**

### **Option A: Direct Integration** (Recommended)

**Approach**: Replace in-memory storage with `StorageManagerService` calls

```rust
pub struct NestGateRpcService {
    /// Real storage manager (ZFS backend!)
    storage_manager: Arc<StorageManagerService>,
    
    /// Start time for uptime
    start_time: SystemTime,
}

impl NestGateRpcService {
    pub async fn new() -> Result<Self> {
        let storage_manager = Arc::new(
            StorageManagerService::new().await?
        );
        
        Ok(Self {
            storage_manager,
            start_time: SystemTime::now(),
        })
    }
}
```

**Benefits**:
- ✅ Simple, direct integration
- ✅ Leverages existing ZFS code
- ✅ Persistent storage (survives restarts)
- ✅ Production-ready with real ZFS

**Complexity**: Medium (6-8 hours)

---

### **Option B: Hybrid Approach** (Fallback)

**Approach**: Use `StorageManagerService` when available, fallback to in-memory

```rust
pub struct NestGateRpcService {
    /// Optional storage manager (if ZFS available)
    storage_manager: Option<Arc<StorageManagerService>>,
    
    /// Fallback: In-memory storage (if ZFS unavailable)
    in_memory_datasets: Arc<DashMap<String, DatasetInfo>>,
    in_memory_objects: Arc<DashMap<String, ObjectStorage>>,
    
    start_time: SystemTime,
}
```

**Benefits**:
- ✅ Works without ZFS (development/testing)
- ✅ Graceful degradation
- ✅ Backwards compatible

**Complexity**: High (8-10 hours)  
**Recommendation**: Only if ZFS not available in production

---

## 📋 **IMPLEMENTATION STEPS**

### **Step 1: Update NestGateRpcService Structure** (1-2h)

**File**: `code/crates/nestgate-core/src/rpc/tarpc_server.rs`

**Changes**:
```rust
pub struct NestGateRpcService {
    // OLD: Remove in-memory storage
    // datasets: Arc<DashMap<String, DatasetInfo>>,
    // objects: Arc<DashMap<String, ObjectStorage>>,
    
    // NEW: Add storage manager
    storage_manager: Arc<StorageManagerService>,
    start_time: SystemTime,
}

impl NestGateRpcService {
    pub async fn new() -> Result<Self> {
        info!("🚀 Creating NestGate RPC service with real storage");
        
        let storage_manager = Arc::new(
            StorageManagerService::new().await
                .map_err(|e| {
                    warn!("Failed to initialize storage manager: {}", e);
                    e
                })?
        );
        
        Ok(Self {
            storage_manager,
            start_time: SystemTime::now(),
        })
    }
}
```

**Testing**:
```bash
cargo build --release
# Should compile without errors
```

---

### **Step 2: Wire create_dataset** (1h)

**Current** (In-memory):
```rust
async fn create_dataset(
    self,
    _context: Context,
    name: String,
    params: DatasetParams,
) -> std::result::Result<DatasetInfo, NestGateRpcError> {
    // In-memory: DashMap insert
    self.datasets.insert(name.clone(), dataset.clone());
    self.objects.insert(name, HashMap::new());
    Ok(dataset)
}
```

**New** (Real storage):
```rust
async fn create_dataset(
    self,
    _context: Context,
    name: String,
    params: DatasetParams,
) -> std::result::Result<DatasetInfo, NestGateRpcError> {
    debug!("RPC: create_dataset({}) → StorageManagerService", name);
    
    // Delegate to storage manager
    self.storage_manager
        .create_dataset(&name, params)
        .await
        .map_err(|e| {
            warn!("Storage manager create_dataset failed: {}", e);
            NestGateRpcError::from(e)
        })
}
```

**Note**: Need to add `create_dataset` method to `StorageManagerService` if missing

---

### **Step 3: Wire list_datasets** (30min)

**Current**:
```rust
async fn list_datasets(
    self,
    _context: Context,
) -> std::result::Result<Vec<DatasetInfo>, NestGateRpcError> {
    // In-memory: DashMap iteration
    Ok(self.datasets.iter().map(|e| e.value().clone()).collect())
}
```

**New**:
```rust
async fn list_datasets(
    self,
    _context: Context,
) -> std::result::Result<Vec<DatasetInfo>, NestGateRpcError> {
    debug!("RPC: list_datasets() → StorageManagerService");
    
    self.storage_manager
        .list_datasets()
        .await
        .map_err(|e| NestGateRpcError::from(e))
}
```

---

### **Step 4: Wire store_object** (1h)

**Current**:
```rust
async fn store_object(
    self,
    _context: Context,
    dataset: String,
    key: String,
    data: Vec<u8>,
) -> std::result::Result<ObjectInfo, NestGateRpcError> {
    // In-memory: HashMap insert
    dataset_objects.insert(key.clone(), (data.clone(), object_info.clone()));
    Ok(object_info)
}
```

**New**:
```rust
async fn store_object(
    self,
    _context: Context,
    dataset: String,
    key: String,
    data: Vec<u8>,
) -> std::result::Result<ObjectInfo, NestGateRpcError> {
    debug!("RPC: store_object({}/{}) → StorageManagerService", dataset, key);
    
    self.storage_manager
        .store_object(&dataset, &key, data)
        .await
        .map_err(|e| {
            warn!("Storage manager store_object failed: {}", e);
            NestGateRpcError::from(e)
        })
}
```

---

### **Step 5: Wire retrieve_object** (30min)

**Current**:
```rust
async fn retrieve_object(
    self,
    _context: Context,
    dataset: String,
    key: String,
) -> std::result::Result<(Vec<u8>, ObjectInfo), NestGateRpcError> {
    // In-memory: HashMap lookup
    dataset_objects.get(&key).map(|e| e.value().clone())
        .ok_or(NestGateRpcError::ObjectNotFound { ... })
}
```

**New**:
```rust
async fn retrieve_object(
    self,
    _context: Context,
    dataset: String,
    key: String,
) -> std::result::Result<(Vec<u8>, ObjectInfo), NestGateRpcError> {
    debug!("RPC: retrieve_object({}/{}) → StorageManagerService", dataset, key);
    
    self.storage_manager
        .retrieve_object(&dataset, &key)
        .await
        .map_err(|e| NestGateRpcError::from(e))
}
```

---

### **Step 6: Wire delete_object** (30min)

**Current**:
```rust
async fn delete_object(
    self,
    _context: Context,
    dataset: String,
    key: String,
) -> std::result::Result<OperationResult, NestGateRpcError> {
    // In-memory: HashMap remove
    dataset_objects.remove(&key);
    Ok(OperationResult { success: true, ... })
}
```

**New**:
```rust
async fn delete_object(
    self,
    _context: Context,
    dataset: String,
    key: String,
) -> std::result::Result<OperationResult, NestGateRpcError> {
    debug!("RPC: delete_object({}/{}) → StorageManagerService", dataset, key);
    
    self.storage_manager
        .delete_object(&dataset, &key)
        .await
        .map(|_| OperationResult {
            success: true,
            message: format!("Object {}/{} deleted", dataset, key),
        })
        .map_err(|e| NestGateRpcError::from(e))
}
```

---

### **Step 7: Wire get_storage_metrics** (1h)

**Current**:
```rust
async fn get_storage_metrics(
    self,
    _context: Context,
) -> std::result::Result<StorageMetrics, NestGateRpcError> {
    // In-memory: Calculate from DashMap
    Ok(self.calculate_metrics().await)
}
```

**New**:
```rust
async fn get_storage_metrics(
    self,
    _context: Context,
) -> std::result::Result<StorageMetrics, NestGateRpcError> {
    debug!("RPC: get_storage_metrics() → StorageManagerService");
    
    self.storage_manager
        .get_metrics()
        .await
        .map_err(|e| NestGateRpcError::from(e))
}
```

**Note**: Real ZFS metrics from `zpool iostat`, `zfs get` commands

---

### **Step 8: Error Conversion** (1h)

**Add error conversion**:

**File**: `code/crates/nestgate-core/src/rpc/tarpc_types.rs`

```rust
impl From<NestGateError> for NestGateRpcError {
    fn from(err: NestGateError) -> Self {
        match err {
            NestGateError::NotFound { resource, .. } => {
                if resource.contains("dataset") {
                    NestGateRpcError::DatasetNotFound {
                        dataset: resource.clone(),
                    }
                } else {
                    NestGateRpcError::ObjectNotFound {
                        dataset: "unknown".to_string(),
                        key: resource.clone(),
                    }
                }
            }
            NestGateError::AlreadyExists { resource, .. } => {
                NestGateRpcError::DatasetAlreadyExists {
                    dataset: resource.clone(),
                }
            }
            _ => NestGateRpcError::InternalError {
                message: err.to_string(),
            },
        }
    }
}
```

---

### **Step 9: Add Missing Methods to StorageManagerService** (2-3h)

**Check if these methods exist**, add if missing:

```rust
impl StorageManagerService {
    /// Create a new dataset with ZFS
    pub async fn create_dataset(
        &self,
        name: &str,
        params: DatasetParams,
    ) -> Result<DatasetInfo> {
        info!("📦 Creating dataset: {}", name);
        
        // Create ZFS dataset (if ZFS enabled)
        if self.config.enable_zfs {
            self.create_zfs_dataset(name, &params).await?;
        }
        
        // Create dataset info
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
            
        let dataset = DatasetInfo {
            name: name.to_string(),
            description: params.description.clone(),
            created_at: now,
            modified_at: now,
            size_bytes: 0,
            object_count: 0,
            compression_ratio: 1.0,
            params,
            status: "active".to_string(),
        };
        
        info!("✅ Dataset created: {}", name);
        Ok(dataset)
    }
    
    /// Store object to ZFS-backed storage
    pub async fn store_object(
        &self,
        dataset: &str,
        key: &str,
        data: Vec<u8>,
    ) -> Result<ObjectInfo> {
        info!("💾 Storing object: {}/{} ({} bytes)", dataset, key, data.len());
        
        // Get dataset path
        let path = self.get_dataset_path(dataset)?;
        let object_path = path.join(key);
        
        // Write to filesystem (ZFS-backed)
        tokio::fs::write(&object_path, &data).await
            .map_err(|e| NestGateError::io_error(&format!(
                "Failed to write object {}/{}: {}",
                dataset, key, e
            )))?;
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
            
        let object_info = ObjectInfo {
            key: key.to_string(),
            size_bytes: data.len() as u64,
            created_at: now,
            modified_at: now,
            content_type: "application/octet-stream".to_string(),
            checksum: "".to_string(), // TODO: Calculate checksum
        };
        
        info!("✅ Object stored: {}/{}", dataset, key);
        Ok(object_info)
    }
    
    /// Retrieve object from ZFS-backed storage
    pub async fn retrieve_object(
        &self,
        dataset: &str,
        key: &str,
    ) -> Result<(Vec<u8>, ObjectInfo)> {
        info!("📖 Retrieving object: {}/{}", dataset, key);
        
        let path = self.get_dataset_path(dataset)?;
        let object_path = path.join(key);
        
        // Read from filesystem
        let data = tokio::fs::read(&object_path).await
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    NestGateError::not_found(&format!("object {}/{}", dataset, key))
                } else {
                    NestGateError::io_error(&format!(
                        "Failed to read object {}/{}: {}",
                        dataset, key, e
                    ))
                }
            })?;
        
        // Get metadata
        let metadata = tokio::fs::metadata(&object_path).await?;
        let modified = metadata.modified()?;
        let modified_at = modified
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
        
        let object_info = ObjectInfo {
            key: key.to_string(),
            size_bytes: data.len() as u64,
            created_at: modified_at, // Use modified as created (approx)
            modified_at,
            content_type: "application/octet-stream".to_string(),
            checksum: "".to_string(),
        };
        
        info!("✅ Object retrieved: {}/{} ({} bytes)", dataset, key, data.len());
        Ok((data, object_info))
    }
    
    /// Delete object from ZFS-backed storage
    pub async fn delete_object(
        &self,
        dataset: &str,
        key: &str,
    ) -> Result<()> {
        info!("🗑️  Deleting object: {}/{}", dataset, key);
        
        let path = self.get_dataset_path(dataset)?;
        let object_path = path.join(key);
        
        tokio::fs::remove_file(&object_path).await
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    NestGateError::not_found(&format!("object {}/{}", dataset, key))
                } else {
                    NestGateError::io_error(&format!(
                        "Failed to delete object {}/{}: {}",
                        dataset, key, e
                    ))
                }
            })?;
        
        info!("✅ Object deleted: {}/{}", dataset, key);
        Ok(())
    }
    
    /// Helper: Get dataset filesystem path
    fn get_dataset_path(&self, dataset: &str) -> Result<std::path::PathBuf> {
        let base_path = std::path::PathBuf::from(&self.config.base_path);
        let dataset_path = base_path.join(dataset);
        
        // Create directory if it doesn't exist
        std::fs::create_dir_all(&dataset_path)
            .map_err(|e| NestGateError::io_error(&format!(
                "Failed to create dataset directory: {}",
                e
            )))?;
        
        Ok(dataset_path)
    }
}
```

---

### **Step 10: Testing** (1-2h)

**Prerequisites**: Fix rustup issue first!

**Unit Tests**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_create_dataset_with_storage_manager() {
        let service = NestGateRpcService::new().await.unwrap();
        
        let params = DatasetParams {
            description: Some("Test dataset".to_string()),
            ..Default::default()
        };
        
        let result = service.clone()
            .create_dataset(Context::current(), "test-dataset".to_string(), params)
            .await;
        
        assert!(result.is_ok());
        let dataset = result.unwrap();
        assert_eq!(dataset.name, "test-dataset");
    }
    
    #[tokio::test]
    async fn test_store_retrieve_object() {
        let service = NestGateRpcService::new().await.unwrap();
        
        // Create dataset first
        let params = DatasetParams::default();
        service.clone()
            .create_dataset(Context::current(), "test-ds".to_string(), params)
            .await
            .unwrap();
        
        // Store object
        let data = b"Hello, ZFS!".to_vec();
        let object = service.clone()
            .store_object(
                Context::current(),
                "test-ds".to_string(),
                "test-key".to_string(),
                data.clone(),
            )
            .await
            .unwrap();
        
        assert_eq!(object.size_bytes, data.len() as u64);
        
        // Retrieve object
        let (retrieved_data, info) = service.clone()
            .retrieve_object(
                Context::current(),
                "test-ds".to_string(),
                "test-key".to_string(),
            )
            .await
            .unwrap();
        
        assert_eq!(retrieved_data, data);
        assert_eq!(info.key, "test-key");
    }
}
```

**Integration Tests**:
```bash
# Requires ZFS available
cargo test --release storage_manager_integration
```

---

## 🚧 **BLOCKERS & DEPENDENCIES**

### **Critical Blocker**: rustup Environment

**Issue**: `rustup could not choose a version of cargo to run`

**Impact**: Prevents all testing (`cargo test`, `cargo build`)

**Fix Required**:
```bash
rustup default stable
cargo --version  # Verify working
```

**Priority**: **MUST FIX FIRST** before any storage wiring

---

### **Dependency 1**: ZFS Availability

**Check**:
```bash
# Check if ZFS is available
which zpool
zpool list

# Check if ZFS kernel module is loaded
lsmod | grep zfs
cat /proc/modules | grep zfs
```

**If Missing**:
- Install ZFS: `sudo apt install zfsutils-linux` (Debian/Ubuntu)
- Load module: `sudo modprobe zfs`

**Alternative**: Use filesystem backend without ZFS (fallback to regular filesystem)

---

### **Dependency 2**: Storage Configuration

**File**: `config/storage.toml` (or create if missing)

```toml
[storage]
base_path = "/var/lib/nestgate/storage"
enable_zfs = true
auto_discover_pools = true
enable_quotas = true
enable_caching = true
enable_monitoring = true

[storage.zfs]
pool_name = "nestgate-pool"
compression = "lz4"
dedup = false
recordsize = "128K"
```

---

## 📊 **ESTIMATED TIMELINE**

### **With rustup Fix**:

| Step | Task | Time | Difficulty |
|------|------|------|------------|
| 0 | Fix rustup | 15min | Easy |
| 1 | Update structure | 1-2h | Medium |
| 2 | Wire create_dataset | 1h | Easy |
| 3 | Wire list_datasets | 30min | Easy |
| 4 | Wire store_object | 1h | Medium |
| 5 | Wire retrieve_object | 30min | Easy |
| 6 | Wire delete_object | 30min | Easy |
| 7 | Wire get_metrics | 1h | Medium |
| 8 | Error conversion | 1h | Easy |
| 9 | Add missing methods | 2-3h | Medium |
| 10 | Testing | 1-2h | Medium |
| **Total** | **8-12h** | **Medium-High** |

---

## 🎯 **SUCCESS CRITERIA**

### **Functional**:
- ✅ All tarpc RPC methods use `StorageManagerService`
- ✅ Data persists across restarts (ZFS-backed)
- ✅ Real ZFS metrics reported
- ✅ Zero compilation errors
- ✅ All tests pass (unit + integration)

### **Performance**:
- ✅ Similar latency to in-memory (ZFS is fast!)
- ✅ Metrics show real disk usage
- ✅ Compression/dedup working (if ZFS enabled)

### **Quality**:
- ✅ Comprehensive error handling
- ✅ Detailed logging (info, debug, warn)
- ✅ Clean code (no hacks or workarounds)

---

## 🚀 **DEPLOYMENT PLAN**

### **Phase 1: Development Testing**
1. Fix rustup
2. Implement wiring
3. Unit tests pass
4. Integration tests pass

### **Phase 2: Staging Deployment**
1. Deploy to staging environment
2. Test with real workloads
3. Monitor metrics
4. Verify data persistence

### **Phase 3: Production Deployment**
1. Deploy to production
2. Monitor closely for 24h
3. Verify ZFS metrics
4. Validate data integrity

---

## 📚 **REFERENCES**

### **Key Files**:
- `code/crates/nestgate-core/src/rpc/tarpc_server.rs` - RPC server (to modify)
- `code/crates/nestgate-core/src/services/storage/service.rs` - Storage manager
- `code/crates/nestgate-core/src/rpc/tarpc_types.rs` - RPC types
- `config/storage.toml` - Storage configuration

### **Documentation**:
- ZFS on Linux: https://openzfs.github.io/openzfs-docs/
- tarpc documentation: https://docs.rs/tarpc/
- Storage service design: `docs/storage/ARCHITECTURE.md`

---

## ✅ **NEXT STEPS**

### **For Current Session** (Blocked):
- ❌ Cannot proceed without rustup fix
- ✅ Created comprehensive wiring plan
- ✅ Documented architecture and steps

### **For Next Developer**:
1. Fix rustup: `rustup default stable`
2. Verify cargo works: `cargo --version`
3. Follow this plan step-by-step
4. Test thoroughly at each step
5. Commit after each working step

---

## 🎓 **LEARNING OUTCOMES**

This wiring demonstrates:
- ✅ **Layered Architecture**: RPC layer → Service layer → Storage layer
- ✅ **Separation of Concerns**: Transport (tarpc) vs Logic (StorageManager)
- ✅ **Progressive Enhancement**: In-memory → Real storage
- ✅ **Production-Ready**: Error handling, logging, metrics

---

**Created**: January 27, 2026  
**Author**: Deep Debt Execution Session  
**Status**: Ready for implementation (after rustup fix)  
**Priority**: High (completes Week 3-4 work)

---

*🦀 Comprehensive plan · Ready to implement · Blocked by rustup · Week 3-4 completion 🏗️*
