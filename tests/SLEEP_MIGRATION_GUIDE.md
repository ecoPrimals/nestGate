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

// ✅ GOOD - Event-driven
use tests::common::concurrent_sync::EventSync;

let sync = EventSync::new();
let s = sync.clone_handle();
tokio::spawn(async move {
    do_work().await;
    s.record("work_done").await;
});
sync.wait_for_event("work_done", Duration::from_secs(1)).await?;
assert!(work_done());
```

#### **2. Coordination Between Tasks**
```rust
// ❌ BAD - Arbitrary delay for synchronization
tokio::time::sleep(Duration::from_millis(50)).await; // Hope other task ready

// ✅ GOOD - Barrier-based sync
use tests::common::concurrent_sync::TestCoordinator;

let coord = TestCoordinator::new(2);
let c1 = coord.clone_handle();

tokio::spawn(async move {
    // Do work
    c1.sync_point().await; // Wait for both tasks
});

coord.sync_point().await;
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

// ✅ BETTER - Event-driven if possible
sync.wait_for_event("condition_met", Duration::from_secs(1)).await?;
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

// ✅ OR use rate limiter if actually needed
use tests::common::concurrent_sync::ConcurrentRateLimiter;

let limiter = ConcurrentRateLimiter::new(10);
for i in 0..100 {
    let lim = limiter.clone();
    tokio::spawn(async move {
        let _permit = lim.acquire().await;
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
      ├─ Waiting for condition? → REPLACE with EventSync
      ├─ Coordinating tasks? → REPLACE with TestCoordinator
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
**After**: `EventSync::wait_for_event()`

### **Pattern 2: Multi-Task Sync**
**Before**: `sleep(time * task_id)` for staggering  
**After**: `TestCoordinator::sync_point()`

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

**Last Updated**: December 13, 2025  
**Files Using New Patterns**: 5 and growing  
**Sleep Calls Eliminated**: 7 so far

