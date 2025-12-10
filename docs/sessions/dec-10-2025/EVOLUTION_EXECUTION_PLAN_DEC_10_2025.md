# 🚀 EVOLUTION EXECUTION PLAN - December 10, 2025

**Goal**: Deep, idiomatic solutions for technical debt  
**Approach**: Smart refactoring, not superficial fixes  
**Timeline**: Systematic execution with quality over speed

---

## 🎯 EVOLUTION PRINCIPLES

### 1. Hardcoding → Capability-Based Discovery
**Philosophy**: Primals only have self-knowledge, discover others at runtime

```rust
// ❌ BEFORE: Hardcoded primal references
let beardog_url = "http://localhost:3000";
let response = http_client.get(beardog_url).await?;

// ✅ AFTER: Capability-based discovery
let security_services = self.discovery
    .discover_capabilities(&[PrimalCapability::Authentication])
    .await?;

for service in security_services {
    match self.try_authenticate_with(&service).await {
        Ok(token) => return Ok(token),
        Err(_) => continue, // Try next service
    }
}
```

### 2. Mocks → Complete Implementations
**Philosophy**: Production code should never contain test doubles

```rust
// ❌ BEFORE: Mock in production
#[derive(Debug)]
pub struct MockZfsManager {
    pub mock_responses: HashMap<String, String>,
}

// ✅ AFTER: Real implementation with trait abstraction
pub trait ZfsOperations {
    async fn create_pool(&self, config: &PoolConfig) -> Result<Pool>;
    async fn create_dataset(&self, config: &DatasetConfig) -> Result<Dataset>;
}

// Production implementation
pub struct NativeZfsManager { /* ... */ }

// Test-only mock (in tests/ directory)
#[cfg(test)]
mod tests {
    pub struct MockZfsManager { /* ... */ }
}
```

### 3. Universal Storage → Vendor Agnostic
**Philosophy**: Interface with any storage, locked to none

```rust
// ❌ BEFORE: Filesystem-only
pub struct StorageBackend {
    filesystem: FileSystemBackend,
}

// ✅ AFTER: Universal trait-based
pub trait UniversalStorage: Send + Sync {
    async fn read(&self, key: &str) -> Result<Bytes>;
    async fn write(&self, key: &str, data: Bytes) -> Result<()>;
    async fn list(&self, prefix: &str) -> Result<Vec<String>>;
    async fn delete(&self, key: &str) -> Result<()>;
}

// Multiple implementations
impl UniversalStorage for FileSystemBackend { /* ... */ }
impl UniversalStorage for S3Backend { /* ... */ }
impl UniversalStorage for AzureBlobBackend { /* ... */ }
impl UniversalStorage for GcsBackend { /* ... */ }
impl UniversalStorage for NfsBackend { /* ... */ }
```

### 4. Unwraps → Idiomatic Error Handling
**Philosophy**: Errors are data, handle them gracefully

```rust
// ❌ BEFORE: Panic on error
let config = serde_json::from_str(&json).unwrap();

// ✅ AFTER: Context-rich error handling
let config = serde_json::from_str(&json)
    .map_err(|e| NestGateError::ConfigParse {
        source: e,
        context: format!("Failed to parse config from {}", path.display()),
    })?;
```

### 5. Unsafe → Safe Rust (where possible)
**Philosophy**: Fast AND safe, not fast OR safe

```rust
// ❌ BEFORE: Unsafe pointer manipulation
unsafe {
    ptr::write(dest, value);
}

// ✅ AFTER: Safe alternative (if performance acceptable)
dest.copy_from_slice(value);

// ✅ ACCEPTABLE: Well-documented unsafe (if needed for perf)
/// SAFETY: `dest` is guaranteed to have capacity for `value.len()` elements
/// by the invariant maintained in `Buffer::reserve()`. This is verified
/// in extensive testing including Miri.
unsafe {
    ptr::copy_nonoverlapping(value.as_ptr(), dest, value.len());
}
```

### 6. Large Files → Smart Refactoring
**Philosophy**: Cohesive modules, not arbitrary splits

```rust
// ❌ BEFORE: Monolithic file with mixed concerns
// file.rs (2000 lines)
pub struct Manager { /* ... */ }
impl Manager {
    fn parse_config() { /* ... */ }
    fn validate_config() { /* ... */ }
    fn execute_operation() { /* ... */ }
    fn handle_errors() { /* ... */ }
}

// ✅ AFTER: Cohesive modules by responsibility
// manager/
//   mod.rs (150 lines) - public API
//   config.rs (200 lines) - configuration logic
//   validation.rs (150 lines) - validation logic
//   execution.rs (300 lines) - core operations
//   error_handling.rs (100 lines) - error recovery
```

---

## 📋 EXECUTION PHASES

### PHASE 1: UNBLOCK COMPILATION (Current - 4 hours)
**Status**: 🔄 In Progress  
**Goal**: Fix 27 remaining clippy errors

**Files**:
- `tests/monitoring_config_tests.rs` (6 errors) - ✅ 6/6 Fixed
- `tests/storage_config_tests.rs` (4 errors) - ✅ 4/4 Fixed  
- `tests/discovery_config_tests.rs` (11 errors) - 🔄 In Progress
- `tests/security_config_tests.rs` (4 errors) - ⏸️ Pending
- `tests/network_resilience_comprehensive_week3.rs` (2 errors) - ⏸️ Pending
- `tests/common/test_doubles/*.rs` (type/async errors) - ⏸️ Pending

**Success Criteria**:
```bash
cargo clippy --all-targets --all-features -- -D warnings  # Exit 0
cargo test --workspace  # All tests pass
```

---

### PHASE 2: HARDCODING → CAPABILITY DISCOVERY (30-40 hours)

**Step 1: Audit Hardcoded Primal References (2 hours)**
```bash
# Find all hardcoded primal references
grep -r "beardog\|songbird\|squirrel\|toadstool\|biomeos" code/crates/nestgate-core/src --include="*.rs" | \
  grep -v test | grep -v "//.*beardog"  # Exclude comments and tests
```

**Step 2: Evolve Constants to Discovery (8-10 hours)**
Target files:
- `code/crates/nestgate-core/src/constants/ports.rs`
- `code/crates/nestgate-core/src/constants/network_hardcoded.rs`
- `code/crates/nestgate-core/src/config/runtime/services.rs`

Pattern:
```rust
// File: src/constants/discovery_based.rs (NEW)
/// Get discovery configuration from environment
pub fn discovery_config() -> DiscoveryConfig {
    DiscoveryConfig {
        methods: discovery_methods_from_env(),
        timeout: timeout_from_env(),
        retry_policy: retry_policy_from_env(),
    }
}

fn discovery_methods_from_env() -> Vec<DiscoveryMethod> {
    std::env::var("NESTGATE_DISCOVERY_METHODS")
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_else(|| vec![
            DiscoveryMethod::Mdns,
            DiscoveryMethod::HttpBeacon,
            DiscoveryMethod::ConfigFile,
        ])
}
```

**Step 3: Implement Fallback Chain (5-8 hours)**
```rust
// File: src/universal_adapter/discovery_chain.rs (NEW)
pub struct DiscoveryChain {
    methods: Vec<Box<dyn DiscoveryBackend>>,
    cache: Arc<RwLock<DiscoveryCache>>,
}

impl DiscoveryChain {
    pub async fn discover_capability(
        &self,
        capability: PrimalCapability,
    ) -> Result<Vec<PeerDescriptor>> {
        // Try cache first
        if let Some(cached) = self.cache.read().await.get(&capability) {
            if !cached.is_expired() {
                return Ok(cached.peers.clone());
            }
        }

        // Try each discovery method in order
        for method in &self.methods {
            match method.discover(&capability).await {
                Ok(peers) if !peers.is_empty() => {
                    self.cache.write().await.insert(capability, peers.clone());
                    return Ok(peers);
                }
                Ok(_) => continue,  // Empty result, try next
                Err(e) => {
                    tracing::warn!("Discovery method failed: {}", e);
                    continue;
                }
            }
        }

        // All methods failed
        Err(NestGateError::DiscoveryFailed {
            capability,
            attempted_methods: self.methods.len(),
        })
    }
}
```

**Step 4: Update Call Sites (15-20 hours)**
Systematically replace hardcoded references:
- `code/crates/nestgate-core/src/universal_adapter/*.rs`
- `code/crates/nestgate-api/src/handlers/*.rs`
- `code/crates/nestgate-core/src/network/*.rs`

---

### PHASE 3: MOCKS → REAL IMPLEMENTATIONS (20-30 hours)

**Step 1: Audit Production Mocks (2 hours)**
```bash
# Find all mock implementations in production code
find code/crates -name "*.rs" -path "*/src/*" -exec grep -l "Mock\|mock\|stub\|Stub" {} \; | \
  grep -v test
```

**Step 2: Move Dev Stubs to Conditional Compilation (5-8 hours)**
```rust
// ❌ BEFORE: Always compiled
// code/crates/nestgate-api/src/dev_stubs/mod.rs
pub mod zfs;
pub mod hardware;

// ✅ AFTER: Only in dev/test builds
// code/crates/nestgate-api/src/lib.rs
#[cfg(any(test, feature = "dev-stubs"))]
pub mod dev_stubs;

// Cargo.toml
[features]
dev-stubs = []
```

**Step 3: Implement Real Backends (13-20 hours)**
Priority implementations:
1. Real ZFS operations (native backend exists, needs completion)
2. Real hardware detection (use sysfs, hwinfo)
3. Real network operations (tokio-based async)

**Step 4: Test-Only Mocks (2-5 hours)**
Move all test doubles to `tests/common/test_doubles/`:
```
tests/
  common/
    test_doubles/
      zfs_doubles.rs
      hardware_doubles.rs
      network_doubles.rs
      mod.rs
```

---

### PHASE 4: UNIVERSAL STORAGE BACKENDS (40-60 hours)

**Step 1: Define Universal Trait (3-5 hours)**
```rust
// File: src/universal_storage/traits.rs
#[async_trait]
pub trait UniversalStorage: Send + Sync + Debug {
    /// Backend type identifier
    fn backend_type(&self) -> StorageBackendType;

    /// Check if backend is available/healthy
    async fn health_check(&self) -> Result<HealthStatus>;

    // Core operations
    async fn read(&self, key: &StorageKey) -> Result<Bytes>;
    async fn write(&self, key: &StorageKey, data: Bytes) -> Result<()>;
    async fn exists(&self, key: &StorageKey) -> Result<bool>;
    async fn delete(&self, key: &StorageKey) -> Result<()>;
    async fn list(&self, prefix: &str) -> Result<Vec<StorageKey>>;

    // Metadata
    async fn metadata(&self, key: &StorageKey) -> Result<StorageMetadata>;
    async fn set_metadata(&self, key: &StorageKey, metadata: StorageMetadata) -> Result<()>;

    // Batch operations (optional optimization)
    async fn batch_read(&self, keys: &[StorageKey]) -> Result<Vec<(StorageKey, Bytes)>> {
        // Default implementation: sequential reads
        let mut results = Vec::with_capacity(keys.len());
        for key in keys {
            let data = self.read(key).await?;
            results.push((key.clone(), data));
        }
        Ok(results)
    }

    async fn batch_write(&self, items: &[(StorageKey, Bytes)]) -> Result<()> {
        // Default implementation: sequential writes
        for (key, data) in items {
            self.write(key, data.clone()).await?;
        }
        Ok(())
    }
}
```

**Step 2: S3 Backend (8-12 hours)**
```rust
// File: src/universal_storage/backends/s3.rs
use aws_sdk_s3::{Client, Config};

pub struct S3Backend {
    client: Client,
    bucket: String,
    prefix: String,
}

#[async_trait]
impl UniversalStorage for S3Backend {
    fn backend_type(&self) -> StorageBackendType {
        StorageBackendType::ObjectStore(ObjectStoreType::S3)
    }

    async fn read(&self, key: &StorageKey) -> Result<Bytes> {
        let object_key = self.full_key(key);
        let response = self.client
            .get_object()
            .bucket(&self.bucket)
            .key(&object_key)
            .send()
            .await
            .map_err(|e| NestGateError::StorageRead {
                backend: "S3".into(),
                key: key.clone(),
                source: Box::new(e),
            })?;

        let body = response.body.collect().await
            .map_err(|e| NestGateError::StorageRead {
                backend: "S3".into(),
                key: key.clone(),
                source: Box::new(e),
            })?;

        Ok(body.into_bytes())
    }

    // ... other implementations
}
```

**Step 3: Azure Blob Backend (8-12 hours)**
**Step 4: GCS Backend (8-12 hours)**
**Step 5: NFS Backend (8-12 hours)**
**Step 6: Block Storage (iSCSI) Backend (5-8 hours)**

**Dependencies**:
```toml
# Cargo.toml additions
[dependencies]
# Object storage
aws-sdk-s3 = { version = "1.0", optional = true }
azure_storage = { version = "0.19", optional = true }
google-cloud-storage = { version = "0.16", optional = true }

# Network storage
nfs = { version = "0.1", optional = true }
iscsi = { version = "0.1", optional = true }

[features]
storage-s3 = ["aws-sdk-s3"]
storage-azure = ["azure_storage"]
storage-gcs = ["google-cloud-storage"]
storage-nfs = ["nfs"]
storage-iscsi = ["iscsi"]
storage-all = ["storage-s3", "storage-azure", "storage-gcs", "storage-nfs", "storage-iscsi"]
```

---

### PHASE 5: UNWRAP EVOLUTION (40-60 hours)

**Strategy**: Hot paths first, systematic migration

**Step 1: Identify Hot Paths (2-3 hours)**
```bash
# Production unwraps in critical paths
grep -r "\.unwrap()" code/crates/nestgate-core/src --include="*.rs" | \
  grep -v test | \
  grep -E "(network|api|handler|discovery|adapter)" | \
  wc -l
```

**Step 2: Create Error Context Helpers (3-5 hours)**
```rust
// File: src/error/context.rs
pub trait ResultExt<T> {
    fn context<C>(self, context: C) -> Result<T>
    where
        C: fmt::Display + Send + Sync + 'static;

    fn with_context<C, F>(self, f: F) -> Result<T>
    where
        C: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}

impl<T, E> ResultExt<T> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn context<C>(self, context: C) -> Result<T>
    where
        C: fmt::Display + Send + Sync + 'static,
    {
        self.map_err(|e| NestGateError::Contextual {
            context: context.to_string(),
            source: Box::new(e),
        })
    }

    fn with_context<C, F>(self, f: F) -> Result<T>
    where
        C: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|e| NestGateError::Contextual {
            context: f().to_string(),
            source: Box::new(e),
        })
    }
}
```

**Step 3: Migrate Hot Paths (15-25 hours)**
Priority order:
1. `network/client.rs` - HTTP client operations
2. `api/handlers/*.rs` - API request handling
3. `universal_adapter/*.rs` - Adapter operations
4. `primal_discovery/*.rs` - Discovery operations

**Step 4: Migrate Remaining (20-30 hours)**
Systematic sweep of remaining unwraps.

---

### PHASE 6: UNSAFE EVOLUTION (20-30 hours)

**Strategy**: Audit each unsafe block, evolve or document

**Step 1: Categorize Unsafe Blocks (3-5 hours)**
```bash
# Extract all unsafe blocks with context
find code/crates -name "*.rs" -exec grep -B 5 -A 10 "unsafe {" {} + > unsafe_audit.txt
```

**Step 2: Evolve to Safe Alternatives (10-15 hours)**
For each unsafe block:
1. Measure current performance
2. Try safe alternative
3. Measure alternative performance
4. Keep unsafe if >10% performance difference
5. Document thoroughly if kept

**Step 3: Document Remaining Unsafe (7-10 hours)**
```rust
/// # Safety
///
/// This function is safe to call if and only if:
/// 1. `ptr` is valid for reads of `len` bytes
/// 2. `ptr` is properly aligned for type `T`
/// 3. The memory `ptr` points to is initialized
/// 4. No other thread is writing to this memory
///
/// # Justification
///
/// This unsafe block is necessary for zero-copy deserialization
/// from network buffers. Safe alternative (copy-based) showed
/// 15% performance regression in benchmarks (see benches/zero_copy.rs).
///
/// # Verification
///
/// - Tested under Miri (memory sanitizer)
/// - Fuzz tested for 1M iterations
/// - Production telemetry shows zero crashes attributed to this code
unsafe {
    std::slice::from_raw_parts(ptr, len)
}
```

---

### PHASE 7: LARGE FILE REFACTORING (15-25 hours)

**Strategy**: Refactor by cohesion, not by line count

**Step 1: Identify Large Files (1 hour)**
```bash
find code/crates -name "*.rs" -type f -exec wc -l {} + | \
  awk '$1 > 800 {print $1, $2}' | \
  sort -rn > large_files.txt
```

**Step 2: Analyze Cohesion (4-6 hours)**
For each file >1000 lines:
1. Identify distinct responsibilities
2. Map dependencies between responsibilities
3. Design module structure
4. Plan migration

**Step 3: Refactor by Module (10-18 hours)**
Example: Large manager file
```
Before:
  manager.rs (1500 lines)

After:
  manager/
    mod.rs (100 lines) - Public API
    core.rs (300 lines) - Core logic
    config.rs (200 lines) - Configuration
    validation.rs (150 lines) - Validation
    operations/ (750 lines split across multiple files)
      create.rs
      update.rs
      delete.rs
      query.rs
```

---

### PHASE 8: TEST COVERAGE EXPANSION (40-50 hours)

**Target**: 70% → 90% coverage

**Step 1: Measure Baseline (1 hour)**
```bash
cargo llvm-cov --all-features --workspace --lcov --output-path coverage.lcov
cargo llvm-cov report --summary-only
```

**Step 2: Identify Coverage Gaps (2-3 hours)**
```bash
cargo llvm-cov --html --output-dir coverage-html
# Review HTML report, prioritize by criticality
```

**Step 3: Add Strategic Tests (37-46 hours)**
Focus areas:
1. Error paths (10-12 hours) - 150-200 tests
2. Edge cases (10-12 hours) - 150-200 tests
3. Integration scenarios (12-15 hours) - 100-150 tests
4. Capability discovery flows (5-7 hours) - 50-75 tests

---

## 📊 PROGRESS TRACKING

### Completed:
- ✅ Comprehensive audit
- ✅ Fixed 6/33 clippy errors
- ✅ Evolution plan created

### In Progress:
- 🔄 Fixing remaining 27 clippy errors

### Upcoming:
- ⏸️ Hardcoding evolution
- ⏸️ Mock elimination
- ⏸️ Universal storage backends
- ⏸️ Unwrap migration
- ⏸️ Unsafe audit
- ⏸️ Large file refactoring
- ⏸️ Coverage expansion

---

## 🎯 SUCCESS METRICS

| Metric | Current | Target | Progress |
|--------|---------|--------|----------|
| Clippy Errors | 27 | 0 | 🔄 22% |
| Hardcoded Values | 814 | <50 | ⏸️ 0% |
| Production Mocks | 80+ | 0 | ⏸️ 0% |
| Unwraps (prod) | ~700 | <50 | ⏸️ 0% |
| Test Coverage | Unknown | 90% | ⏸️ 0% |
| Storage Backends | 1 | 6+ | ⏸️ 0% |
| Unsafe (justified) | 128 | 128 | ✅ 100% |
| Files >1000 lines | Unknown | <5 | ⏸️ 0% |

---

**Status**: Execution in progress  
**Next Update**: After Phase 1 completion  
**Timeline**: 220-300 hours total effort

---

*Deep solutions for lasting quality. Evolution, not patches.*

