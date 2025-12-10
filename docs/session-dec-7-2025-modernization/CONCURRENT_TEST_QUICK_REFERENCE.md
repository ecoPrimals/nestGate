# ⚡ QUICK REFERENCE - Concurrent Test Infrastructure
**NestGate Modernization** | Created: Dec 7, 2025

---

## 🚀 USAGE

### Basic Pattern:
```rust
use tests::common::{IsolatedTestContext, ConcurrentCoordinator};

#[tokio::test]
async fn test_my_service() {
    // Get isolated context
    let ctx = IsolatedTestContext::new().await.unwrap();
    
    // Allocate unique port (no conflicts)
    let port = ctx.allocate_port().await;
    
    // Start service
    let coord = ctx.coordinator();
    let coord_clone = coord.clone();
    
    tokio::spawn(async move {
        start_service(port).await;
        coord_clone.signal_ready();  // Signal when ready
    });
    
    // Wait for ready (event-driven, no sleep!)
    coord.wait_ready().await;
    
    // Test immediately when actually ready
    assert!(test_endpoint(port).await);
    
    // Cleanup automatic on drop
}
```

---

## 🔧 API

### IsolatedTestContext:
```rust
let ctx = IsolatedTestContext::new().await?;
let port = ctx.allocate_port().await;           // Unique port
let temp = ctx.temp_dir();                      // Isolated directory
let path = ctx.temp_path("subdir/file.txt");   // Path in temp
let coord = ctx.coordinator();                  // Get coordinator
```

### ConcurrentCoordinator:
```rust
let coord = ConcurrentCoordinator::new();
coord.wait_ready().await;                       // Wait for signal
coord.wait_ready_timeout(dur).await?;           // With timeout
coord.signal_ready();                           // Signal ready
coord.set_state(CoordinatorState::Ready);       // Set state
let state = coord.state();                      // Get state
coord.wait_for_state(target).await;             // Wait for state
```

### PortAllocator (Advanced):
```rust
let alloc = PortAllocator::shared();
let port = alloc.allocate().await;              // Allocate
alloc.release(port);                            // Release
```

---

## 🔄 MIGRATION PATTERNS

### Pattern 1: Sleep → Event
```rust
// ❌ OLD
tokio::time::sleep(Duration::from_millis(100)).await;
assert!(service.is_ready());

// ✅ NEW
coord.wait_ready().await;
assert!(service.is_ready());
```

### Pattern 2: Hardcoded Port → Dynamic
```rust
// ❌ OLD
let service = Service::new(8080).await;

// ✅ NEW
let ctx = IsolatedTestContext::new().await?;
let port = ctx.allocate_port().await;
let service = Service::new(port).await;
```

### Pattern 3: Manual Cleanup → Automatic
```rust
// ❌ OLD
let temp = create_temp();
// ... test ...
cleanup(temp);  // Might not run on panic!

// ✅ NEW
let ctx = IsolatedTestContext::new().await?;
// Cleanup automatic, even on panic
```

### Pattern 4: Polling → Watching
```rust
// ❌ OLD
loop {
    if check_ready() { break; }
    sleep(Duration::from_millis(10)).await;
}

// ✅ NEW
coord.wait_for_state(CoordinatorState::Ready).await;
```

---

## ✅ CHECKLIST

### Before Migration:
- [ ] Identify sleep calls
- [ ] Find hardcoded ports
- [ ] Note shared resources
- [ ] Check cleanup needs

### During Migration:
- [ ] Add `use tests::common::IsolatedTestContext;`
- [ ] Create context: `let ctx = IsolatedTestContext::new().await?;`
- [ ] Replace ports: `ctx.allocate_port().await`
- [ ] Replace sleeps: `coord.wait_ready().await`
- [ ] Remove manual cleanup

### After Migration:
- [ ] Run test: `cargo test --test TESTNAME`
- [ ] Verify passes
- [ ] Check performance: `hyperfine 'cargo test --test TESTNAME'`
- [ ] Document improvements

---

## 📊 EXPECTED IMPROVEMENTS

### Speed:
```
Before: ~50ms sleep × N sleeps
After:  Near-instant (<1ms)
Improvement: 50x+ per test
```

### Reliability:
```
Before: Flaky (timing-dependent)
After:  Deterministic (event-driven)
Improvement: 100% reliable
```

### Concurrency:
```
Before: Serial (port conflicts)
After:  Parallel (isolated)
Improvement: 10-16x throughput
```

---

## 🐛 TROUBLESHOOTING

### "Port in use":
```rust
// Problem: Reusing port
let port = 8080;  // Fixed port

// Solution: Dynamic allocation
let port = ctx.allocate_port().await;
```

### "Test timeout":
```rust
// Problem: Waiting forever
coord.wait_ready().await;

// Solution: Use timeout
coord.wait_ready_timeout(Duration::from_secs(5)).await?;
```

### "Cleanup not running":
```rust
// Problem: Manual cleanup after test
cleanup_resources();

// Solution: Register with context
ctx.cleanup().register(|| cleanup_resources()).await;
```

---

## 📚 FILES

```
tests/common/isolated_context.rs  - Implementation
START_HERE_NEXT_SESSION_MODERNIZATION.md - Quick start
MODERNIZATION_EXECUTION_PLAN_DEC_7_2025.md - Full plan
```

---

## 🎯 TARGETS (Week 1)

1. `concurrent_operations_comprehensive_tests.rs` (14 sleeps)
2. `e2e/intermittent_network_connectivity.rs` (16 sleeps)
3. `e2e/network_bandwidth_saturation.rs` (11 sleeps)
4. `common/concurrent_test_framework.rs` (10 sleeps)
5. `e2e/fault_tolerance_scenarios.rs` (9 sleeps)

**Total**: 60 sleeps to eliminate

---

## ⚡ QUICK COMMANDS

```bash
# Run test
cargo test --test TESTNAME

# Run with output
cargo test --test TESTNAME -- --nocapture

# Benchmark
hyperfine 'cargo test --test TESTNAME'

# Check races
RUSTFLAGS="-Z sanitizer=thread" cargo +nightly test --test TESTNAME
```

---

**STATUS**: Infrastructure ready  
**TESTS**: 6/6 passing  
**NEXT**: Migrate first file

