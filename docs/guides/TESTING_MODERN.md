> **Historical**: This document was written in December 7, 2025. Current architecture
> and patterns may differ. See root-level docs and `specs/` for current specifications.

# 🧪 NestGate Testing Guide - Modern Concurrent Edition
**Version**: 2.0  
**Last Updated**: December 7, 2025  
**Status**: Modern Concurrent Testing System Active

---

## 🎯 Philosophy

**"Test issues = Production issues"**

Our tests verify real concurrent behavior, not artificial timing. We use:
- ✅ Event-driven coordination (not sleep-based timing)
- ✅ Environment isolation (not global state)
- ✅ Dynamic resource allocation (not hardcoded ports)

---

## 📊 Quick Status

```
Tests:          3,083+ PASSING (99.94% pass rate)
Coverage:       73.65%
Target:         90%
Concurrent:     96% (targeting 99.7%)
Infrastructure: Unit, Integration, E2E, Chaos, Fault, Byzantine
Build:          ✅ Clean
```

---

## 🚀 Quick Start

### Run All Tests Concurrently
```bash
# Run all tests with maximum concurrency
cargo test --workspace --lib --no-fail-fast

# Run with high concurrency (verify robustness)
cargo test --workspace --lib -- --test-threads=16

# Run single-threaded (debugging only)
cargo test --workspace --lib -- --test-threads=1
```

### Generate Coverage Report
```bash
# Generate HTML coverage report
cargo llvm-cov --workspace --lib --all-features --html

# View the report
open target/llvm-cov/html/index.html

# Get coverage summary
cargo llvm-cov --workspace --lib --all-features --summary-only
```

---

## ✨ Modern Concurrent Patterns

### Pattern 1: Environment Isolation

**❌ Anti-Pattern: Global State Pollution**
```rust
#[test]
fn old_test() {
    std::env::set_var("PORT", "8080");
    // Test code
    std::env::remove_var("PORT"); // May not run if panic!
}
```

**✅ Modern: Isolated Environment**
```rust
#[test]
fn modern_test() {
    use temp_env::with_var;
    
    with_var("PORT", Some("8080"), || {
        // Test code
    }); // Auto-restored, even on panic!
}

#[tokio::test]
async fn modern_async_test() {
    use temp_env::async_with_vars;
    
    async_with_vars(
        vec![
            ("PORT", Some("8080")),
            ("HOST", Some("localhost")),
        ],
        async {
            // Async test code
        }
    ).await; // Auto-restored!
}
```

### Pattern 2: Event-Driven Coordination

**❌ Anti-Pattern: Sleep-Based Timing**
```rust
#[tokio::test]
async fn old_test() {
    start_service().await;
    tokio::time::sleep(Duration::from_millis(100)).await; // Hope it's ready!
    send_request().await; // May fail due to timing
}
```

**✅ Modern: Explicit Signals**
```rust
use tests::test_utils::coordination::ReadySignal;

#[tokio::test]
async fn modern_test() {
    let signal = ReadySignal::new();
    
    tokio::spawn({
        let signal = signal.clone();
        async move {
            start_service().await;
            signal.notify_ready().await; // Explicit signal
        }
    });
    
    signal.wait_ready().await; // Wait for actual readiness
    send_request().await; // Guaranteed ready!
}
```

### Pattern 3: Dynamic Port Allocation

**❌ Anti-Pattern: Hardcoded Ports**
```rust
#[tokio::test]
async fn old_test() {
    let server = Server::bind("127.0.0.1:8080").await?; // Port conflict!
    // Test server
}
```

**✅ Modern: OS-Assigned Ports**
```rust
use tests::test_utils::ports::DynamicPort;

#[tokio::test]
async fn modern_test() {
    let port = DynamicPort::new();
    let server = Server::bind(format!("127.0.0.1:{}", port.get())).await?;
    // Test server with unique port
}
```

### Pattern 4: Concurrent Stress Testing

**✅ Modern: Test Real Concurrency**
```rust
#[tokio::test]
async fn concurrent_stress_test() {
    let config = Arc::new(Config::new());
    
    // Spawn 100 concurrent operations
    let handles: Vec<_> = (0..100)
        .map(|i| {
            let config = Arc::clone(&config);
            tokio::spawn(async move {
                // Each task operates concurrently
                assert!(config.is_valid());
            })
        })
        .collect();
    
    // All should complete successfully
    for handle in handles {
        handle.await.unwrap();
    }
}
```

---

## 🛠️ Test Utilities

### Available Utilities

```rust
use tests::test_utils::*;

// Coordination primitives
use coordination::{ReadySignal, CompletionBarrier, StateWatcher};

// Dynamic ports
use ports::{DynamicPort, allocate_ports};

// Environment isolation
use environment::{IsolatedEnv, with_var, async_with_vars};
```

### ReadySignal - Single Readiness Notification

```rust
let signal = ReadySignal::new();

tokio::spawn({
    let signal = signal.clone();
    async move {
        setup().await;
        signal.notify_ready().await;
    }
});

signal.wait_ready().await; // Blocks until ready
```

### CompletionBarrier - Wait for Multiple Tasks

```rust
let barrier = CompletionBarrier::new(3);

for i in 0..3 {
    let barrier = barrier.clone();
    tokio::spawn(async move {
        do_work(i).await;
        barrier.arrive().await;
    });
}

barrier.wait_all().await; // Waits for all 3
```

### StateWatcher - Observe State Changes

```rust
let watcher = StateWatcher::new("initializing");

tokio::spawn({
    let watcher = watcher.clone();
    async move {
        setup().await;
        watcher.update("running").await;
    }
});

watcher.wait_for("running").await; // Waits for specific state
```

### DynamicPort - Conflict-Free Ports

```rust
// Single port
let port = DynamicPort::new();
let addr = port.bind_addr(); // "127.0.0.1:54321"
let url = port.url("/health"); // "http://127.0.0.1:54321/health"

// Multiple ports
let [api, ws, metrics] = allocate_ports::<3>();
```

---

## 📂 Test Organization

```
nestgate/
├── code/crates/
│   ├── nestgate-core/src/
│   │   ├── *_tests.rs              # Unit tests (inline)
│   │   └── */mod.rs                # Module tests
│   ├── nestgate-api/src/handlers/
│   │   └── *_tests.rs              # Handler tests
│   └── nestgate-zfs/src/
│       └── *_tests.rs              # ZFS tests
├── tests/
│   ├── test_utils/                 # ✨ NEW: Test utilities
│   │   ├── coordination.rs         # Event-driven primitives
│   │   ├── ports.rs                # Dynamic port allocation
│   │   └── environment.rs          # Env isolation
│   ├── e2e/                        # End-to-end scenarios
│   ├── chaos/                      # Chaos engineering
│   └── *.rs                        # Integration tests
└── benches/                        # Performance benchmarks
```

---

## 🧪 Test Types

### 1. Unit Tests (2,500+)

**Pattern**: Test individual functions/modules

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculation() {
        assert_eq!(add(2, 2), 4);
    }

    #[tokio::test]
    async fn test_async_operation() {
        let result = fetch_data().await;
        assert!(result.is_ok());
    }
}
```

### 2. Integration Tests (300+)

**Pattern**: Test interactions between components

```rust
#[tokio::test]
async fn test_service_integration() {
    let port = DynamicPort::new();
    let service = Service::new(port.get()).await;
    
    let response = service.handle_request().await?;
    assert_eq!(response.status, 200);
}
```

### 3. E2E Tests (43 scenarios)

**Pattern**: Test complete workflows

```rust
#[tokio::test]
async fn test_complete_workflow() {
    // Setup
    let system = TestSystem::new().await;
    
    // Execute workflow
    system.create_pool("test-pool").await?;
    system.create_dataset("test-dataset").await?;
    system.write_data(b"test").await?;
    
    // Verify
    let data = system.read_data().await?;
    assert_eq!(data, b"test");
    
    // Cleanup
    system.cleanup().await;
}
```

### 4. Chaos Tests (11+ suites)

**Pattern**: Test resilience under adverse conditions

```rust
#[tokio::test]
async fn chaos_network_partition() {
    let system = System::new().await;
    
    // Simulate network partition
    system.partition_network().await;
    
    // Verify graceful degradation
    let result = system.execute_operation().await;
    assert!(result.is_err());
    
    // Restore and verify recovery
    system.restore_network().await;
    let result = system.execute_operation().await;
    assert!(result.is_ok());
}
```

### 5. Byzantine Tests (11 scenarios)

**Pattern**: Test malicious/arbitrary behavior

```rust
#[tokio::test]
async fn byzantine_conflicting_messages() {
    let msg1 = Message { sender: 1, value: 100, seq: 1 };
    let msg2 = Message { sender: 1, value: 200, seq: 1 }; // Conflict!
    
    let is_byzantine = detect_conflict(&msg1, &msg2);
    assert!(is_byzantine);
}
```

---

## 📊 Coverage Guidelines

### Target Coverage
- **Overall**: 90%
- **Core modules**: 95%+
- **Critical paths**: 100%
- **Error paths**: 90%+

### Measure Coverage
```bash
# Generate coverage report
cargo llvm-cov --workspace --lib --all-features --html

# Check specific module
cargo llvm-cov --package nestgate-core --html

# Coverage summary
cargo llvm-cov --workspace --lib --summary-only
```

### Current Status
```
Total Coverage:     73.65%
High Coverage (95%+): nestgate-zfs/types
Medium Coverage (70-95%): Most modules
Low Coverage (<70%): Some newer features
```

---

## ⚡ Performance

### Concurrent Execution

```bash
# Default (auto-detect cores)
cargo test --workspace --lib

# High concurrency (stress test)
cargo test --workspace --lib -- --test-threads=32

# Serial (debugging only)
cargo test --workspace --lib -- --test-threads=1
```

### Test Runtime

```
Current: 38.34s (concurrent)
Target:  15s (optimized)
Improvement: 2.5x faster
```

---

## 🐛 Debugging

### Show Test Output

```bash
# Show all output
cargo test --workspace --lib -- --nocapture

# Show specific test output
cargo test test_name -- --nocapture

# Show only failures
cargo test --workspace --lib --no-fail-fast
```

### Isolate Failures

```bash
# Run specific test
cargo test --package nestgate-core test_specific_function

# Run tests matching pattern
cargo test --lib config::

# Run single-threaded (for debugging race conditions)
cargo test --lib -- --test-threads=1
```

---

## 📝 Best Practices

### ✅ DO

1. **Use event-driven coordination**
   ```rust
   let signal = ReadySignal::new();
   signal.wait_ready().await; // Not sleep!
   ```

2. **Isolate environment variables**
   ```rust
   with_var("VAR", Some("value"), || { /* test */ });
   ```

3. **Use dynamic ports**
   ```rust
   let port = DynamicPort::new();
   ```

4. **Test concurrent behavior**
   ```rust
   let handles = (0..100).map(|_| tokio::spawn(test())).collect();
   ```

5. **Clean up resources**
   ```rust
   let _cleanup = TestCleanup::new(); // RAII pattern
   ```

### ❌ DON'T

1. **Don't use sleep for coordination**
   ```rust
   tokio::time::sleep(Duration::from_millis(100)).await; // ❌
   ```

2. **Don't pollute environment**
   ```rust
   std::env::set_var("VAR", "value"); // ❌ Without cleanup
   ```

3. **Don't hardcode ports**
   ```rust
   Server::bind("127.0.0.1:8080").await; // ❌ Port conflict
   ```

4. **Don't serialize tests unnecessarily**
   ```rust
   #[serial] // ❌ Only if absolutely required
   ```

5. **Don't ignore timing issues**
   ```rust
   // If test is flaky, fix the race condition!
   ```

---

## 🔧 Troubleshooting

### Tests Fail Randomly

**Cause**: Race condition or timing dependency  
**Fix**: Use event-driven coordination instead of sleeps

```rust
// ❌ BAD
tokio::time::sleep(Duration::from_millis(10)).await;

// ✅ GOOD
signal.wait_ready().await;
```

### Port Already in Use

**Cause**: Hardcoded port conflicts  
**Fix**: Use dynamic port allocation

```rust
// ❌ BAD
let port = 8080;

// ✅ GOOD
let port = DynamicPort::new().get();
```

### Environment Variable Pollution

**Cause**: Global env var modification  
**Fix**: Use isolated environment

```rust
// ❌ BAD
std::env::set_var("VAR", "value");

// ✅ GOOD
with_var("VAR", Some("value"), || { /* test */ });
```

---

## 📚 Resources

- **Test Utils**: `tests/test_utils/mod.rs`
- **Examples**: `tests/e2e_scenario_43_configuration_lifecycle.rs`
- **Patterns**: `CONCURRENT_EVOLUTION_EXECUTION_DEC_7_2025.md`
- **Coverage**: `cargo llvm-cov --workspace --lib --html`

---

## 🎯 Current Status

```
✅ 3,083/3,085 tests passing (99.94%)
✅ 73.65% coverage (target 90%)
✅ 96% concurrent tests
✅ Event-driven patterns established
✅ Environment isolation implemented
✅ Dynamic port allocation available
✅ Comprehensive test utilities

Target for Next Month:
- 99.7% concurrent tests
- 90% coverage
- 15s test runtime (2.5x faster)
- Zero sleep-based coordination
```

---

*Last Updated: December 7, 2025*  
*Version: 2.0 - Modern Concurrent Edition*  
*Philosophy: "Test issues = Production issues"*

