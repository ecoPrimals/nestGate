# Test Concurrency Analysis - November 20, 2025

## Executive Summary

The `test_sustained_performance` test failure is caused by **resource exhaustion when run concurrently with other tests**. This analysis identifies the root causes and provides a roadmap for building robust, concurrent-safe testing infrastructure.

## Root Cause Analysis

### 1. **Resource Contention** ⚠️ PRIMARY ISSUE

The performance test allocates significant resources:
- **Memory**: Up to 50MB in 1MB chunks per test
- **CPU**: Intensive computation loops (5000 iterations × continuous)
- **Tokio Tasks**: 15+ concurrent spawn operations
- **Duration**: 5 seconds of sustained load

**Problem**: When `llvm-cov` runs the full test suite, multiple performance tests execute concurrently, causing:
- Memory exhaustion (50MB × N tests)
- CPU starvation (competing for threads)
- Tokio runtime overload (too many spawned tasks)

```
Tests/performance_stress_battery.rs:240-268 (run_memory_stress_test)
├── Allocates 1MB chunks continuously
├── Holds up to 50 chunks (50MB)
└── When N tests run concurrently → N × 50MB

Tests/performance_stress_battery.rs:216-237 (run_cpu_stress_test)
├── CPU-intensive loops (5000 iterations)
├── Runs for 5 seconds
└── When N tests run concurrently → Thread pool saturation
```

### 2. **Environment Variable Conflicts** 🔴 CRITICAL

**Found**: Environment variables are used without synchronization locks.

```rust
// tests/common/test_environment.rs:96-104
pub fn from_environment() -> Self {
    let host = env::var("NESTGATE_TEST_HOST").unwrap_or_else(...);
    let port = env::var("NESTGATE_TEST_PORT").unwrap_or_else(...);
    // ... more env vars
}
```

**Problem**: Concurrent tests reading the same environment variables without locking can cause:
- Race conditions when tests modify env vars
- Inconsistent configuration between tests
- Non-deterministic failures

**Good Pattern Found** (not used everywhere):
```rust
// code/crates/nestgate-core/src/constants/system.rs:77
static ENV_TEST_LOCK: Mutex<()> = Mutex::new(());
```

### 3. **No Test Isolation** 🟡 MEDIUM

Tests share:
- **Global Tokio Runtime**: All async tests use the same `#[tokio::test]` runtime
- **Process Resources**: No namespacing or containerization
- **File System**: Tests may write to shared temp directories

### 4. **Tokio Runtime Overload** 🟡 MEDIUM

Each performance test spawns **4+ long-running tasks**:
```rust
// tests/performance_stress_battery.rs:117-132
handles.push(self.run_performance_operations().await);
handles.push(self.run_cpu_stress_test().await);
handles.push(self.run_memory_stress_test().await);
handles.push(self.run_io_stress_test().await);
```

When multiple tests run concurrently:
- 4 tests × 4 tasks = 16+ concurrent tasks
- Each task runs for 5 seconds
- Tokio's default thread pool may be overwhelmed

## Modern Concurrent Testing Architecture

### **Level 1: Immediate Fixes** (1-2 days)

#### 1.1 Serialize Resource-Intensive Tests

```rust
// tests/performance_stress_battery.rs
use std::sync::Mutex;

// Global lock for performance tests
static PERFORMANCE_TEST_LOCK: Mutex<()> = Mutex::new(());

#[tokio::test]
async fn test_sustained_performance() -> Result<()> {
    let _lock = PERFORMANCE_TEST_LOCK
        .lock()
        .expect("Failed to acquire performance test lock");
    
    // ... existing test code
}
```

**Benefit**: Prevents concurrent resource exhaustion
**Tradeoff**: Tests run sequentially (slower but reliable)

#### 1.2 Environment Variable Guards

```rust
// tests/common/test_environment.rs
use std::sync::Mutex;

static ENV_TEST_LOCK: Mutex<()> = Mutex::new(());

impl TestEnvironment {
    pub fn from_environment_with_lock() -> (Self, MutexGuard<'static, ()>) {
        let lock = ENV_TEST_LOCK.lock().expect("Failed to acquire env lock");
        (Self::from_environment(), lock)
    }
}

// Usage:
#[tokio::test]
async fn test_with_env() {
    let (_env, _lock) = TestEnvironment::from_environment_with_lock();
    // Lock held until end of scope
}
```

#### 1.3 Resource Limits

```rust
// tests/performance_stress_battery.rs
pub struct PerformanceConfig {
    // Add resource limits
    pub max_memory_mb: usize,        // Default: 20MB (down from 50MB)
    pub max_concurrent_tasks: usize, // Default: 4
    pub cpu_intensity: f64,          // 0.0-1.0, scale down for concurrent
}

impl PerformanceStressBattery {
    pub fn new_concurrent_safe(config: PerformanceConfig) -> Self {
        let mut safe_config = config;
        // Scale down for concurrent execution
        safe_config.concurrent_threads = safe_config.concurrent_threads.min(5);
        safe_config.max_memory_mb = safe_config.max_memory_mb.min(20);
        Self { config: safe_config, ... }
    }
}
```

### **Level 2: Modern Test Infrastructure** (1 week)

#### 2.1 Test Isolation Framework

```rust
// tests/isolation/test_runner.rs
use std::sync::Arc;
use tokio::sync::Semaphore;

/// Modern test runner with resource control
pub struct IsolatedTestRunner {
    /// Limit concurrent resource-intensive tests
    resource_semaphore: Arc<Semaphore>,
    /// Dedicated tokio runtime for heavy tests
    heavy_runtime: Arc<tokio::runtime::Runtime>,
}

impl IsolatedTestRunner {
    pub fn new() -> Self {
        let cpu_count = num_cpus::get();
        Self {
            // Allow only 2 resource-intensive tests at once
            resource_semaphore: Arc::new(Semaphore::new(2)),
            // Dedicated runtime with controlled thread pool
            heavy_runtime: Arc::new(
                tokio::runtime::Builder::new_multi_thread()
                    .worker_threads(cpu_count / 2) // Reserve half for other tests
                    .thread_name("heavy-test-worker")
                    .enable_all()
                    .build()
                    .expect("Failed to create heavy test runtime")
            ),
        }
    }
    
    pub async fn run_heavy_test<F, Fut>(&self, test: F) -> Result<()>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = Result<()>> + Send,
    {
        let _permit = self.resource_semaphore.acquire().await;
        self.heavy_runtime.spawn(test()).await?
    }
}

// Global runner
static TEST_RUNNER: OnceLock<IsolatedTestRunner> = OnceLock::new();

pub fn get_test_runner() -> &'static IsolatedTestRunner {
    TEST_RUNNER.get_or_init(IsolatedTestRunner::new)
}
```

#### 2.2 Test Resource Manager

```rust
// tests/isolation/resource_manager.rs
use std::sync::Arc;
use tokio::sync::RwLock;

/// Track and limit test resource usage
pub struct TestResourceManager {
    allocated_memory: Arc<AtomicU64>,
    active_tasks: Arc<AtomicUsize>,
    max_memory_bytes: u64,
    max_concurrent_tasks: usize,
}

impl TestResourceManager {
    pub fn new() -> Self {
        let system_mem = sys_info::mem_info().unwrap().total * 1024;
        let max_test_memory = system_mem / 4; // Use max 25% of system RAM
        
        Self {
            allocated_memory: Arc::new(AtomicU64::new(0)),
            active_tasks: Arc::new(AtomicUsize::new(0)),
            max_memory_bytes: max_test_memory,
            max_concurrent_tasks: num_cpus::get() * 4,
        }
    }
    
    pub async fn allocate_memory(&self, bytes: u64) -> Result<MemoryGuard> {
        let current = self.allocated_memory.fetch_add(bytes, Ordering::SeqCst);
        if current + bytes > self.max_memory_bytes {
            self.allocated_memory.fetch_sub(bytes, Ordering::SeqCst);
            return Err(NestGateError::resource_exhausted(
                "Test memory limit exceeded".to_string()
            ));
        }
        Ok(MemoryGuard {
            manager: self.clone(),
            bytes,
        })
    }
}

pub struct MemoryGuard {
    manager: TestResourceManager,
    bytes: u64,
}

impl Drop for MemoryGuard {
    fn drop(&mut self) {
        self.manager.allocated_memory.fetch_sub(self.bytes, Ordering::SeqCst);
    }
}
```

#### 2.3 Environment Variable Isolation

```rust
// tests/isolation/env_isolation.rs
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Per-test environment variable isolation
pub struct IsolatedEnvironment {
    original_vars: HashMap<String, Option<String>>,
    test_vars: HashMap<String, String>,
    _lock: MutexGuard<'static, ()>,
}

impl IsolatedEnvironment {
    pub fn new(test_name: &str) -> Self {
        let lock = ENV_TEST_LOCK.lock().expect("Failed to acquire env lock");
        
        // Capture original environment
        let original_vars = NESTGATE_ENV_VARS
            .iter()
            .map(|key| (key.to_string(), std::env::var(key).ok()))
            .collect();
        
        // Set test-specific environment
        let test_vars = HashMap::new();
        
        Self {
            original_vars,
            test_vars,
            _lock: lock,
        }
    }
    
    pub fn set(&mut self, key: &str, value: &str) {
        std::env::set_var(key, value);
        self.test_vars.insert(key.to_string(), value.to_string());
    }
}

impl Drop for IsolatedEnvironment {
    fn drop(&mut self) {
        // Restore original environment
        for (key, value) in &self.original_vars {
            match value {
                Some(v) => std::env::set_var(key, v),
                None => std::env::remove_var(key),
            }
        }
    }
}

const NESTGATE_ENV_VARS: &[&str] = &[
    "NESTGATE_TEST_HOST",
    "NESTGATE_TEST_PORT",
    "NESTGATE_TIMEOUT_MS",
    "NESTGATE_TEST_ZFS_DATASET",
    // ... all nestgate env vars
];
```

### **Level 3: Advanced Patterns** (2-3 weeks)

#### 3.1 Test Containers/Namespaces

```rust
// tests/isolation/container_test.rs
use testcontainers::*;

/// Run tests in isolated containers
pub struct ContainerizedTest {
    container: Container<'static, GenericImage>,
}

impl ContainerizedTest {
    pub async fn new(test_name: &str) -> Result<Self> {
        let image = GenericImage::new("nestgate-test-env", "latest")
            .with_env_var("TEST_NAME", test_name);
        
        let container = clients::Cli::default().run(image);
        
        Ok(Self { container })
    }
    
    pub async fn run_test<F, Fut>(&self, test: F) -> Result<()>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<()>>,
    {
        // Execute test in container context
        test().await
    }
}
```

#### 3.2 Property-Based Concurrent Testing

```rust
// tests/property/concurrent_safety.rs
use proptest::prelude::*;
use tokio::sync::RwLock;

proptest! {
    #[test]
    fn test_concurrent_config_access(
        num_readers in 1..100usize,
        num_writers in 1..10usize
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let config = Arc::new(RwLock::new(TestConfig::default()));
            
            // Spawn concurrent readers and writers
            let mut handles = vec![];
            
            for _ in 0..num_readers {
                let config = config.clone();
                handles.push(tokio::spawn(async move {
                    let _ = config.read().await;
                }));
            }
            
            for _ in 0..num_writers {
                let config = config.clone();
                handles.push(tokio::spawn(async move {
                    let mut cfg = config.write().await;
                    cfg.update_something();
                }));
            }
            
            for handle in handles {
                handle.await.unwrap();
            }
        });
    }
}
```

#### 3.3 Chaos Testing Integration

```rust
// tests/chaos/concurrent_chaos.rs
use chaos_mesh::*;

#[tokio::test]
async fn test_concurrent_chaos_resilience() {
    let chaos = ChaosManager::new()
        .with_scenario(NetworkChaos::packet_loss(0.1))
        .with_scenario(CPUChaos::stress(0.8))
        .with_scenario(MemoryChaos::limit_bytes(100_000_000));
    
    chaos.inject().await;
    
    // Run tests under chaos
    let results = tokio::join!(
        test_performance_under_chaos(),
        test_correctness_under_chaos(),
        test_recovery_under_chaos()
    );
    
    chaos.recover().await;
    
    assert!(results.0.is_ok());
    assert!(results.1.is_ok());
    assert!(results.2.is_ok());
}
```

## Implementation Roadmap

### Phase 1: Stabilization (This Week)

- [x] Identify root causes
- [ ] Add `PERFORMANCE_TEST_LOCK` to serialize resource-intensive tests
- [ ] Add resource limits to performance tests
- [ ] Document concurrent testing requirements

### Phase 2: Infrastructure (Next 2 Weeks)

- [ ] Implement `IsolatedTestRunner`
- [ ] Implement `TestResourceManager`
- [ ] Implement `IsolatedEnvironment`
- [ ] Migrate existing tests to use new infrastructure

### Phase 3: Advanced (Month 2)

- [ ] Container-based test isolation
- [ ] Property-based concurrent testing
- [ ] Chaos testing integration
- [ ] Automated concurrency analysis

## Recommended Testing Patterns

### Pattern 1: Lightweight Tests (Default)

```rust
#[tokio::test]
async fn test_lightweight_operation() {
    // Fast, no significant resource usage
    // Can run concurrently with others
}
```

### Pattern 2: Resource-Intensive Tests (Serialized)

```rust
#[tokio::test]
async fn test_heavy_operation() {
    let _lock = PERFORMANCE_TEST_LOCK.lock().unwrap();
    // Resource-intensive work
    // Runs alone
}
```

### Pattern 3: Isolated Tests (Dedicated Runtime)

```rust
#[tokio::test]
async fn test_isolated_operation() {
    get_test_runner()
        .run_heavy_test(|| async {
            // Runs in dedicated runtime
            // Better isolation
        })
        .await
        .unwrap();
}
```

### Pattern 4: Containerized Tests (Maximum Isolation)

```rust
#[tokio::test]
async fn test_fully_isolated() {
    let container = ContainerizedTest::new("my_test").await.unwrap();
    container
        .run_test(|| async {
            // Runs in isolated container
            // Complete resource isolation
        })
        .await
        .unwrap();
}
```

## Dependencies to Add

```toml
[dev-dependencies]
# Test isolation
testcontainers = "0.15"
proptest = "1.4"
quickcheck = "1.0"

# Resource management
sys-info = "0.9"
num_cpus = "1.16"

# Chaos testing
chaos-mesh = "0.3"  # If using Kubernetes
tokio-chaosmonkey = "0.1"  # For in-process chaos

# Test orchestration
nextest = "0.9"  # Modern test runner with better concurrency
```

## Metrics & Monitoring

### Test Execution Metrics

```rust
pub struct TestMetrics {
    pub concurrent_tests: usize,
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
    pub test_duration_ms: u64,
    pub tokio_tasks_spawned: usize,
}

impl TestMetrics {
    pub fn report(&self) {
        println!("📊 Test Metrics:");
        println!("   Concurrent: {}", self.concurrent_tests);
        println!("   Memory: {:.2} MB", self.memory_usage_mb);
        println!("   CPU: {:.1}%", self.cpu_usage_percent);
        println!("   Duration: {} ms", self.test_duration_ms);
        println!("   Tasks: {}", self.tokio_tasks_spawned);
    }
}
```

## Summary

**Immediate Action**: Add serialization lock to `test_sustained_performance`

**Short Term**: Implement resource limits and environment isolation

**Long Term**: Build modern concurrent testing infrastructure with containers, property-based testing, and chaos engineering

This will make our tests:
- ✅ **Robust**: No flaky failures from resource contention
- ✅ **Concurrent**: Safe parallel execution
- ✅ **Modern**: Following 2025 best practices
- ✅ **Fast**: Better test parallelization where safe
- ✅ **Reliable**: Deterministic results every time

