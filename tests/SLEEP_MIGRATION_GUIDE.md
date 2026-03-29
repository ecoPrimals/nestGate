# 📖 SLEEP PATTERN MIGRATION GUIDE

## When to Keep Sleep vs. Replace

### ✅ **KEEP SLEEP** (Legitimate Uses)

#### **1. Chaos/Fault Injection Tests**
```rust
// ✅ GOOD - Simulating real network latency
tokio::time::sleep(Duration::from_millis(latency_ms)).await;
```
**Why**: These tests measure system behavior under realistic delays.

#### **2. Benchmarks/Performance Tests**
```rust
// ✅ GOOD - Measuring actual time
let start = Instant::now();
tokio::time::sleep(Duration::from_secs(1)).await;
let elapsed = start.elapsed();
```
**Why**: We're testing time-related functionality.

#### **3. Rate Limiting Implementation**
```rust
// ✅ GOOD - Actual rate limiting logic
tokio::time::sleep(Duration::from_millis(rate_limit_delay)).await;
```
**Why**: This is production code implementing delays.

---

### ❌ **REPLACE SLEEP** (Anti-Patterns)

#### **1. Waiting for Async Completion**
```rust
// ❌ BAD - Hope operation completes
tokio::spawn(async { do_work().await });
tokio::time::sleep(Duration::from_millis(100)).await;
assert!(work_done());

// ✅ GOOD - Event-driven (test binary must `mod common` → tests/common)
use crate::common::modern_sync::TestCoordinator;

let coord = TestCoordinator::new();
let c = coord.clone();
tokio::spawn(async move {
    do_work().await;
    c.signal_ready().await;
});
coord.wait_ready().await;
assert!(work_done());
```

#### **2. Coordination Between Tasks**
```rust
// ❌ BAD - Arbitrary delay for synchronization
tokio::time::sleep(Duration::from_millis(50)).await; // Hope other task ready

// ✅ GOOD - Barrier-based sync
use crate::common::modern_sync::TestBarrier;
use std::sync::Arc;

let barrier = Arc::new(TestBarrier::new(2));
let b = Arc::clone(&barrier);

tokio::spawn(async move {
    // Do work
    b.arrive();
});

barrier.arrive();
barrier.wait().await;
```

#### **3. Polling Without Backoff**
```rust
// ❌ BAD - Fixed interval polling
while !condition() {
    tokio::time::sleep(Duration::from_millis(10)).await;
}

// ✅ ACCEPTABLE - Polling with timeout (if no event mechanism)
tokio::time::timeout(Duration::from_secs(1), async {
    while !condition() {
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}).await?;

// ✅ BETTER - Condition wait with timeout (no fixed sleep loop)
use crate::common::sync_utils::wait_for_condition;

wait_for_condition(|| condition(), Duration::from_secs(1)).await?;
```

#### **4. Holding Locks**
```rust
// ❌ BAD - Sleep while holding lock
let _lock = mutex.lock().await;
tokio::time::sleep(Duration::from_millis(100)).await; // Blocks everyone!

// ✅ GOOD - Signal-based coordination
use tokio::sync::oneshot;

let (tx, rx) = oneshot::channel();
let handle = tokio::spawn(async move {
    let _lock = mutex.lock().await;
    // Do work...
    tx.send(()).ok(); // Signal when done
});
rx.await?; // Wait for signal, not arbitrary time
```

#### **5. Staggering Task Launch**
```rust
// ❌ BAD - Sleep between spawns
for i in 0..100 {
    tokio::spawn(async move { task(i).await });
    tokio::time::sleep(Duration::from_millis(10)).await; // Why?
}

// ✅ GOOD - Just spawn (executor handles scheduling)
for i in 0..100 {
    tokio::spawn(async move { task(i).await });
}

// ✅ OR use a semaphore if you truly need bounded concurrency
use tokio::sync::Semaphore;
use std::sync::Arc;

let sem = Arc::new(Semaphore::new(10));
for i in 0..100 {
    let sem = Arc::clone(&sem);
    tokio::spawn(async move {
        let _permit = sem.acquire().await.unwrap();
        task(i).await
    });
}
```

---

## 🎯 Decision Tree

```
Is this a sleep call?
  ↓
  ├─ In chaos/fault test? → KEEP (simulating real delays)
  ├─ In benchmark? → KEEP (measuring time)
  ├─ Rate limiting implementation? → KEEP (production logic)
  │
  └─ Otherwise, ask:
      ├─ Waiting for condition? → REPLACE with `sync_utils::wait_for_*` or `modern_sync::TestEventStream`
      ├─ Coordinating tasks? → REPLACE with `modern_sync::TestCoordinator` / `TestBarrier`
      ├─ Holding a lock? → REPLACE with channels/signals
      ├─ "Giving time" to something? → REPLACE with yield_now() or remove
      └─ Polling? → ADD timeout, consider event-driven

```

---

## 📊 Our Findings

### **Sleep Calls by Category**:
```
Total identified:          337
Legitimate (keep):         ~60 (18%)
  - Chaos tests:           ~40
  - Benchmarks:            ~15
  - Rate limiting:         ~5

Problematic (replace):     ~270 (80%)
  - Async coordination:    ~120
  - Lock holding:          ~15
  - Task staggering:       ~50
  - Polling loops:         ~85

Already fixed:             7 (2%)
```

---

## 🔧 Common Patterns

### **Pattern 1: Test Coordination**
**Before**: `sleep(100ms)` and hope  
**After**: `crate::common::modern_sync::TestCoordinator::wait_ready()` (after `signal_ready()`), or `crate::common::sync_utils::wait_for_condition`

### **Pattern 2: Multi-Task Sync**
**Before**: `sleep(time * task_id)` for staggering  
**After**: `crate::common::modern_sync::TestBarrier` (each task `arrive()`, one waits with `wait()`)

### **Pattern 3: Timeout Testing**
**Before**: `sleep(10s)` in operation  
**After**: `std::future::pending()` + timeout wrapper

### **Pattern 4: Polling**
**Before**: Bare `while !cond() { sleep(10ms) }`  
**After**: Wrapped in `timeout()`, or event-driven

---

## ✅ Checklist

Before replacing a sleep:
- [ ] Understand why it's there
- [ ] Is it simulating real-world delay? (keep)
- [ ] Is it waiting for something? (event-driven)
- [ ] Is it coordinating tasks? (barrier/sync)
- [ ] Can I remove it entirely? (try first!)
- [ ] Test the replacement works
- [ ] Document the change

---

## Module paths (2026)

- Prefer **`crate::common::modern_sync`** for coordinators, barriers, event streams, and result channels (`tests/common/modern_sync.rs`, exported from `tests/common/mod.rs`).
- Prefer **`crate::common::sync_utils`** for condition polling with timeout (`tests/common/sync_utils.rs`).
- The file `tests/common/concurrent_sync.rs` is **not** wired into `tests/common/mod.rs`; do not import it as a stable module path—use `modern_sync` / `sync_utils` (or plain `tokio::sync` primitives) instead.

Integration test binaries that use these helpers typically declare `mod common;` at the top of `tests/<suite>.rs`, which resolves to `tests/common/mod.rs` (same pattern as `tests/biomeos_integration_tests.rs`).

**Last Updated**: March 29, 2026

