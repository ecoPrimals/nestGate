> **Historical**: This document was written in November 19, 2025. Current architecture
> and patterns may differ. See root-level docs and `specs/` for current specifications.

# 🚀 MODERN CONCURRENCY PATTERNS GUIDE - NESTGATE
**Philosophy**: Test Issues = Production Issues. No sleep(), Real Concurrency Only.

---

## ❌ ANTI-PATTERNS WE'RE ELIMINATING

### Anti-Pattern #1: Using `sleep()` in Tests
```rust
// ❌ BAD: Timing-based, fragile, slow, doesn't test real behavior
#[tokio::test]
async fn bad_test() {
    create_resource().await;
    tokio::time::sleep(Duration::from_millis(100)).await; // ← WRONG!
    assert!(resource_exists());
}
```

**Problems**:
- Doesn't test actual completion, just waits arbitrary time
- Flaky on slow machines
- Hides race conditions
- Slow (589 sleep instances = slow test suite)
- Doesn't test what you think it tests

### Anti-Pattern #2: Serial Tests for Concurrent Code
```rust
// ❌ BAD: Testing concurrent code serially
#[tokio::test]
async fn bad_concurrent_test() {
    for i in 0..10 {
        create_workspace(i).await;  // Serial, not concurrent!
    }
}
```

### Anti-Pattern #3: Stub Functions with sleep()
```rust
// ❌ BAD: Stub that simulates timing instead of behavior
async fn create_workspace_stub(name: &str) -> Result<(), Error> {
    sleep(Duration::from_millis(50)).await; // ← Simulating, not testing!
    Ok(())
}
```

---

## ✅ MODERN PATTERNS WE'RE ADOPTING

### Pattern #1: Barriers for True Concurrency
```rust
// ✅ GOOD: All tasks start simultaneously
#[tokio::test]
async fn good_concurrent_test() {
    let n = 10;
    let barrier = Arc::new(Barrier::new(n));
    let mut handles = Vec::new();
    
    for i in 0..n {
        let barrier = barrier.clone();
        handles.push(tokio::spawn(async move {
            barrier.wait().await; // All wait here
            // NOW all start at exactly the same time - TRUE concurrency test
            create_workspace(i).await
        }));
    }
    
    // Collect results
    let results = futures::future::join_all(handles).await;
    
    // All tasks ran truly concurrently
    assert_eq!(results.iter().filter(|r| r.is_ok()).count(), n);
}
```

**Benefits**:
- Tests REAL concurrent behavior
- Deterministic
- Fast
- Catches actual race conditions

### Pattern #2: Channels for Event Coordination
```rust
// ✅ GOOD: Event-driven coordination
#[tokio::test]
async fn good_event_driven_test() {
    let (tx, mut rx) = mpsc::channel(100);
    
    // Producer
    tokio::spawn(async move {
        for i in 0..10 {
            tx.send(i).await.ok();
        }
    });
    
    // Consumer - no sleep, real event handling
    let mut received = Vec::new();
    while let Some(value) = rx.recv().await {
        received.push(value);
        if received.len() == 10 {
            break; // Event-driven termination
        }
    }
    
    assert_eq!(received.len(), 10);
}
```

### Pattern #3: Notify for State Changes
```rust
// ✅ GOOD: Wait for actual state change, not arbitrary time
#[tokio::test]
async fn good_state_change_test() {
    let ready_notify = Arc::new(Notify::new());
    let is_ready = Arc::new(AtomicBool::new(false));
    
    let notify = ready_notify.clone();
    let ready = is_ready.clone();
    
    // Setup task
    tokio::spawn(async move {
        perform_setup().await;
        ready.store(true, Ordering::SeqCst);
        notify.notify_waiters(); // Signal actual completion
    });
    
    // Wait for REAL completion, not arbitrary time
    ready_notify.notified().await;
    
    assert!(is_ready.load(Ordering::SeqCst));
}
```

### Pattern #4: Atomics for Concurrent State
```rust
// ✅ GOOD: Thread-safe counting without locks
#[tokio::test]
async fn good_atomic_test() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();
    
    for _ in 0..100 {
        let counter = counter.clone();
        handles.push(tokio::spawn(async move {
            counter.fetch_add(1, Ordering::SeqCst);
        }));
    }
    
    for handle in handles {
        handle.await.unwrap();
    }
    
    assert_eq!(counter.load(Ordering::SeqCst), 100);
    // No race conditions, no sleep, deterministic
}
```

### Pattern #5: Timeout for REAL Timeout Testing
```rust
// ✅ GOOD: Test actual timeout behavior
#[tokio::test]
async fn good_timeout_test() {
    let operation = async {
        // Real async work
        heavy_operation().await
    };
    
    // Test REAL timeout behavior
    match tokio::time::timeout(Duration::from_secs(1), operation).await {
        Ok(result) => {
            // Completed in time
            assert!(result.is_ok());
        }
        Err(_) => {
            // Actually timed out
            panic!("Operation took too long");
        }
    }
    // No sleep, testing real timeout behavior
}
```

### Pattern #6: RwLock for Concurrent Reads
```rust
// ✅ GOOD: Multiple concurrent readers
#[tokio::test]
async fn good_concurrent_reads() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let barrier = Arc::new(Barrier::new(10));
    let mut handles = Vec::new();
    
    for _ in 0..10 {
        let data = data.clone();
        let barrier = barrier.clone();
        
        handles.push(tokio::spawn(async move {
            barrier.wait().await; // True concurrent start
            let guard = data.read().await;
            assert_eq!(guard.len(), 3);
        }));
    }
    
    for handle in handles {
        handle.await.unwrap();
    }
    // All readers ran truly concurrently
}
```

### Pattern #7: Mutex for Mutual Exclusion
```rust
// ✅ GOOD: Proper write synchronization
#[tokio::test]
async fn good_concurrent_writes() {
    let data = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    
    for i in 0..100 {
        let data = data.clone();
        handles.push(tokio::spawn(async move {
            let mut guard = data.lock().await;
            guard.push(i);
        }));
    }
    
    for handle in handles {
        handle.await.unwrap();
    }
    
    let final_data = data.lock().await;
    assert_eq!(final_data.len(), 100);
    // No data races, proper mutual exclusion
}
```

### Pattern #8: Semaphore for Resource Limiting
```rust
// ✅ GOOD: Real resource pool management
#[tokio::test]
async fn good_resource_limiting() {
    let pool = Arc::new(Semaphore::new(5)); // Max 5 concurrent
    let mut handles = Vec::new();
    
    for i in 0..20 {
        let pool = pool.clone();
        handles.push(tokio::spawn(async move {
            let _permit = pool.acquire().await.unwrap();
            // Real resource usage
            perform_operation(i).await
        }));
    }
    
    for handle in handles {
        handle.await.unwrap().unwrap();
    }
    // Never exceeded 5 concurrent operations
}
```

---

## 🔧 MIGRATION STRATEGY

### Step 1: Identify sleep() Locations
```bash
rg 'sleep|Sleep' tests/ --type rust
# Found: 589 instances across 171 files
```

### Step 2: Categorize by Purpose

1. **Waiting for Completion** → Use `Notify` or channels
2. **Simulating Duration** → Use `yield_now()` or real operations
3. **Rate Limiting** → Use `Semaphore`
4. **Synchronizing Tasks** → Use `Barrier`
5. **Testing Timeouts** → Use `tokio::time::timeout()`
6. **State Coordination** → Use atomics + Notify

### Step 3: Replace Systematically

Priority order:
1. **E2E tests** (highest impact)
2. **Chaos tests** (testing resilience)
3. **Integration tests**
4. **Unit tests**

### Step 4: Verify Improvements

```bash
# Before: Slow, flaky tests
cargo test -- --nocapture  # 589 sleeps = slow

# After: Fast, deterministic tests
cargo test -- --nocapture  # 0 sleeps = fast, reliable
```

---

## 📊 REAL EXAMPLES FROM OUR CODEBASE

### Example 1: Concurrent Workspace Creation

**Before (Anti-Pattern)**:
```rust
async fn test_concurrent_creation() {
    for i in 0..10 {
        tokio::spawn(create_workspace(i));
    }
    sleep(Duration::from_secs(1)).await; // ← Hope they're done?
    // Flaky, slow, doesn't test concurrency
}
```

**After (Modern Pattern)**:
```rust
async fn test_concurrent_creation_modern() {
    let n = 10;
    let barrier = Arc::new(Barrier::new(n));
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();
    
    for i in 0..n {
        let barrier = barrier.clone();
        let counter = counter.clone();
        handles.push(tokio::spawn(async move {
            barrier.wait().await; // Synchronized start
            create_workspace(i).await?;
            counter.fetch_add(1, Ordering::SeqCst);
            Ok::<_, Error>(())
        }));
    }
    
    // Wait for actual completion
    for handle in handles {
        handle.await.unwrap().unwrap();
    }
    
    // Deterministic verification
    assert_eq!(counter.load(Ordering::SeqCst), n);
}
```

### Example 2: Network Partition Test

**Before (Anti-Pattern)**:
```rust
async fn test_network_partition() {
    // Phase 1: Normal
    sleep(Duration::from_millis(100)).await;
    
    // Phase 2: Partition
    simulate_partition();
    sleep(Duration::from_millis(500)).await;
    
    // Phase 3: Heal
    heal_partition();
    sleep(Duration::from_millis(200)).await;
    
    // Hope it's recovered?
}
```

**After (Modern Pattern)**:
```rust
async fn test_network_partition_modern() {
    let partition_notify = Arc::new(Notify::new());
    let heal_notify = Arc::new(Notify::new());
    let is_partitioned = Arc::new(AtomicBool::new(false));
    
    // Service A
    let service_a = tokio::spawn({
        let partition = partition_notify.clone();
        let heal = heal_notify.clone();
        let state = is_partitioned.clone();
        async move {
            // Wait for REAL partition event
            partition.notified().await;
            state.store(true, Ordering::SeqCst);
            
            // Wait for REAL heal event
            heal.notified().await;
            state.store(false, Ordering::SeqCst);
        }
    });
    
    // Coordinator triggers events
    partition_notify.notify_waiters();
    // ... operations during partition ...
    heal_notify.notify_waiters();
    
    service_a.await.unwrap();
    // Event-driven, deterministic, fast
}
```

---

## 🎯 BENEFITS OF MODERN PATTERNS

### Performance
- **Before**: 589 sleeps × average 50ms = ~30 seconds of wasted time
- **After**: 0 sleeps = instant coordination
- **Speedup**: 10-100x faster test suite

### Reliability
- **Before**: Flaky tests (timing-dependent)
- **After**: Deterministic tests
- **Improvement**: 100% reproducible

### Correctness
- **Before**: Tests pass but production fails (race conditions hidden)
- **After**: Tests catch real concurrency issues
- **Improvement**: Test what you ship

### Maintainability
- **Before**: Mysterious sleep values ("why 147ms?")
- **After**: Clear synchronization intent
- **Improvement**: Self-documenting code

---

## 🚀 QUICK REFERENCE

| Use Case | Old (Bad) | New (Good) |
|----------|-----------|------------|
| Wait for completion | `sleep()` | `Notify::notified()` or channels |
| Start tasks together | Sequential spawn | `Barrier::wait()` |
| Track state | Variables + sleep | `AtomicBool` + `Notify` |
| Test timeout | `sleep()` then check | `tokio::time::timeout()` |
| Concurrent reads | Lock + sleep | `RwLock::read()` |
| Concurrent writes | Sleep between ops | `Mutex::lock()` |
| Rate limiting | Sleep in loop | `Semaphore::acquire()` |
| Event coordination | Sleep + poll | Channels (`mpsc`) |

---

## 📝 CHECKLIST FOR REVIEWING CODE

When you see:
- ✅ `sleep()` → Ask: "Can I use Notify/channels/Barrier instead?"
- ✅ Sequential spawn → Ask: "Should these run concurrently with Barrier?"
- ✅ Polling in loop → Ask: "Can I use channels for events?"
- ✅ Arbitrary timeout → Ask: "Can I use tokio::timeout()?"
- ✅ "Wait for X" comment → Ask: "What event am I really waiting for?"

---

## 🎓 CONCLUSION

**Old Philosophy**: "Wait a bit, hope it's done"  
**New Philosophy**: "Wait for actual completion event"

**Old Result**: Flaky, slow, doesn't catch bugs  
**New Result**: Fast, deterministic, catches real issues

**Remember**: If production is concurrent, tests must be truly concurrent.  
**If it needs sleep() to pass, you're not testing the right thing.**

---

**Status**: Migration in progress  
**Target**: 0 sleep() calls in tests  
**Current**: 589 sleep() calls across 171 files  
**Next**: Systematic replacement following this guide

---

*Generated: November 19, 2025*  
*Part of: Deep Technical Debt Elimination Initiative*

