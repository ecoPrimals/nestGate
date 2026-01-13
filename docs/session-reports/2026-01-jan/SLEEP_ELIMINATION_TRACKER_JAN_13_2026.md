# 🎯 Sleep Elimination Tracker - January 13, 2026

**Philosophy**: Test issues ARE production issues. No sleeps, no serial tests (except chaos).

---

## 📊 BASELINE METRICS

**Total Sleep Calls**: 329 across 81 test files  
**Target**: 0 sleeps in tests (except intentional delays for demos)  
**Timeline**: 2-3 weeks systematic elimination

---

## 🔍 SLEEP CATEGORIES

### Category 1: Service Startup Waits (Highest Priority)
**Pattern**: `sleep()` after starting service, hoping it's ready  
**Files**: ~30 files  
**Fix**: Add `wait_ready()` methods with proper signaling

**Example Files**:
- `biomeos_integration_tests.rs` (10 sleeps)
- `e2e_scenario_65_70_final.rs` (20 sleeps)
- `e2e_scenario_60_64_backup_recovery.rs` (14 sleeps)
- `e2e_scenario_48_53_operations.rs` (14 sleeps)

### Category 2: Async Operation Completion (Medium Priority)
**Pattern**: `spawn()` then `sleep()`, hoping task completes  
**Files**: ~25 files  
**Fix**: Use `JoinHandle::await` or channels for completion

**Example Files**:
- `chaos_test_19_24_edge_cases.rs` (14 sleeps)
- `e2e/intermittent_network_connectivity.rs` (15 sleeps)
- `e2e_scenario_54_59_observability.rs` (11 sleeps)

### Category 3: Condition Polling (Medium Priority)
**Pattern**: Loop with `sleep()` checking condition  
**Files**: ~15 files  
**Fix**: Use `tokio::sync::Notify` or condition variables

**Example Files**:
- `e2e/network_bandwidth_saturation.rs` (12 sleeps)
- `chaos_test_25_28_final.rs` (9 sleeps)
- `chaos_test_13_18_advanced.rs` (9 sleeps)

### Category 4: Chaos/Timing Tests (Keep Some)
**Pattern**: Intentional delays for fault injection  
**Files**: ~10 files (chaos tests)  
**Decision**: Keep sleeps in chaos tests when simulating timing issues

**Example Files**:
- `chaos/` directory tests (legitimate use)
- `fault_injection_*` tests (may need some delays)

---

## 🛠️ REPLACEMENT PATTERNS

### Pattern 1: Service Ready Signaling

```rust
// ❌ OLD: Sleep and hope
async fn start_service() -> Service {
    let service = Service::new();
    tokio::spawn(async move { service.run().await });
    tokio::time::sleep(Duration::from_millis(100)).await; // Hope it's ready!
    service
}

// ✅ NEW: Explicit readiness
pub struct Service {
    ready_tx: Option<tokio::sync::oneshot::Sender<()>>,
    ready_rx: tokio::sync::oneshot::Receiver<()>,
}

impl Service {
    async fn run(&mut self) {
        // ... initialization ...
        if let Some(tx) = self.ready_tx.take() {
            let _ = tx.send(()); // Signal ready
        }
        // ... main loop ...
    }
    
    pub async fn wait_ready(&mut self) -> Result<()> {
        tokio::time::timeout(
            Duration::from_secs(5),
            &mut self.ready_rx
        ).await??;
        Ok(())
    }
}

// Usage
let mut service = Service::new();
tokio::spawn(async move { service.run().await });
service.wait_ready().await?; // Actually waits!
```

### Pattern 2: Task Completion

```rust
// ❌ OLD: Spawn and sleep
tokio::spawn(async { do_work().await });
tokio::time::sleep(Duration::from_millis(50)).await; // Hope it's done!

// ✅ NEW: Await the handle
let handle = tokio::spawn(async { do_work().await });
handle.await??; // Actually waits for completion
```

### Pattern 3: Condition Wait

```rust
// ❌ OLD: Poll with sleep
for _ in 0..100 {
    if check_condition() { break; }
    tokio::time::sleep(Duration::from_millis(10)).await;
}

// ✅ NEW: Event-driven with timeout
let notify = Arc::new(tokio::sync::Notify::new());
tokio::time::timeout(
    Duration::from_secs(1),
    notify.notified()
).await?;
```

### Pattern 4: Channel-Based Coordination

```rust
// ❌ OLD: Multiple spawns with sleep
tokio::spawn(async { task1().await });
tokio::spawn(async { task2().await });
tokio::time::sleep(Duration::from_millis(200)).await; // Hope both done!

// ✅ NEW: Channels for coordination
let (tx, mut rx) = tokio::sync::mpsc::channel(2);

tokio::spawn(async move {
    task1().await;
    tx.send(()).await.unwrap();
});

tokio::spawn(async move {
    task2().await;
    tx.send(()).await.unwrap();
});

// Wait for both
rx.recv().await;
rx.recv().await;
```

---

## 📋 HIGH-PRIORITY FILES TO FIX

### Week 1 Targets (Top 10 Files, 143 sleeps)

1. **e2e_scenario_65_70_final.rs** - 20 sleeps
   - Service startup waits
   - Operation completion waits
   - **Priority**: CRITICAL

2. **e2e_scenario_60_64_backup_recovery.rs** - 14 sleeps
   - Backup completion waits
   - Recovery verification waits
   - **Priority**: HIGH

3. **e2e_scenario_48_53_operations.rs** - 14 sleeps
   - Operation sequencing
   - State verification
   - **Priority**: HIGH

4. **e2e/intermittent_network_connectivity.rs** - 15 sleeps
   - Network simulation delays
   - Reconnection waits
   - **Priority**: HIGH

5. **chaos_test_19_24_edge_cases.rs** - 14 sleeps
   - Fault injection timing
   - Recovery verification
   - **Priority**: MEDIUM (some sleeps legitimate for chaos)

6. **e2e/network_bandwidth_saturation.rs** - 12 sleeps
   - Load generation pacing
   - Saturation detection
   - **Priority**: MEDIUM

7. **e2e_scenario_54_59_observability.rs** - 11 sleeps
   - Metrics collection waits
   - Monitoring stabilization
   - **Priority**: MEDIUM

8. **e2e_scenario_44_47_advanced.rs** - 12 sleeps
   - Complex workflow coordination
   - Multi-step operations
   - **Priority**: MEDIUM

9. **biomeos_integration_tests.rs** - 10 sleeps
   - External service waits
   - Integration synchronization
   - **Priority**: MEDIUM

10. **chaos_test_25_28_final.rs** - 9 sleeps
    - Chaos scenario timing
    - System recovery
    - **Priority**: LOW (chaos test, some sleeps OK)

**Total for Week 1**: 143 sleeps (43% of all sleeps)

---

## 🎯 EXECUTION STRATEGY

### Phase 1: Create Utilities (2-3 days)

```rust
// tests/common/sync_utils.rs

/// Wait for a condition with timeout
pub async fn wait_for_condition<F>(
    mut check: F,
    timeout: Duration,
) -> Result<()>
where
    F: FnMut() -> bool + Send,
{
    tokio::time::timeout(timeout, async {
        while !check() {
            tokio::task::yield_now().await;
        }
    }).await.map_err(|_| anyhow::anyhow!("Timeout waiting for condition"))?;
    Ok(())
}

/// Wait for async condition with timeout
pub async fn wait_for_async<F, Fut>(
    mut check: F,
    timeout: Duration,
) -> Result<()>
where
    F: FnMut() -> Fut + Send,
    Fut: std::future::Future<Output = bool> + Send,
{
    tokio::time::timeout(timeout, async {
        while !check().await {
            tokio::task::yield_now().await;
        }
    }).await.map_err(|_| anyhow::anyhow!("Timeout waiting for async condition"))?;
    Ok(())
}

/// Service ready tracker
pub struct ReadySignal {
    notify: Arc<tokio::sync::Notify>,
}

impl ReadySignal {
    pub fn new() -> (Self, ReadyWaiter) {
        let notify = Arc::new(tokio::sync::Notify::new());
        (
            Self { notify: Arc::clone(&notify) },
            ReadyWaiter { notify },
        )
    }
    
    pub fn signal(&self) {
        self.notify.notify_waiters();
    }
}

pub struct ReadyWaiter {
    notify: Arc<tokio::sync::Notify>,
}

impl ReadyWaiter {
    pub async fn wait(&self, timeout: Duration) -> Result<()> {
        tokio::time::timeout(timeout, self.notify.notified())
            .await
            .map_err(|_| anyhow::anyhow!("Timeout waiting for ready signal"))?;
        Ok(())
    }
}
```

### Phase 2: Fix High-Priority Files (1 week)

- **Day 1-2**: e2e_scenario_65_70_final.rs (20 sleeps)
- **Day 3**: e2e_scenario_60_64_backup_recovery.rs (14 sleeps)
- **Day 4**: e2e_scenario_48_53_operations.rs (14 sleeps)
- **Day 5**: e2e/intermittent_network_connectivity.rs (15 sleeps)

### Phase 3: Fix Remaining Files (1 week)

- **Day 1-2**: Medium priority files (40-50 sleeps)
- **Day 3-4**: Low priority files (30-40 sleeps)
- **Day 5**: Review and cleanup

### Phase 4: Validation (3-4 days)

- Run full test suite concurrently
- Identify any remaining timing issues
- Performance benchmark comparison
- Documentation updates

---

## 📊 PROGRESS TRACKING

### Week 1: Setup + High Priority
- [ ] Create sync_utils.rs with helper functions
- [ ] Fix e2e_scenario_65_70_final.rs (20 sleeps)
- [ ] Fix e2e_scenario_60_64_backup_recovery.rs (14 sleeps)
- [ ] Fix e2e_scenario_48_53_operations.rs (14 sleeps)
- [ ] Fix e2e/intermittent_network_connectivity.rs (15 sleeps)
- [ ] Run tests concurrently, verify no flakes

**Target**: 63 sleeps eliminated (19% of total)

### Week 2: Medium Priority
- [ ] Fix chaos_test_19_24_edge_cases.rs (keep legitimate delays)
- [ ] Fix e2e/network_bandwidth_saturation.rs (12 sleeps)
- [ ] Fix e2e_scenario_54_59_observability.rs (11 sleeps)
- [ ] Fix e2e_scenario_44_47_advanced.rs (12 sleeps)
- [ ] Fix biomeos_integration_tests.rs (10 sleeps)
- [ ] Fix remaining medium priority files

**Target**: 100+ sleeps eliminated (50%+ of total)

### Week 3: Cleanup + Validation
- [ ] Fix all remaining non-chaos sleeps
- [ ] Document legitimate sleep usage in chaos tests
- [ ] Full concurrent test run
- [ ] Performance benchmarks
- [ ] Update testing documentation

**Target**: 250+ sleeps eliminated (75%+ of total)

---

## 🧪 TESTING VALIDATION

### Concurrent Execution Test

```bash
# Run tests with high parallelism
RUST_TEST_THREADS=32 cargo test --tests

# Should complete without flakes
# Target: 50% faster than serial execution
```

### Timing Validation

```bash
# Before sleep elimination
time cargo test --tests
# ~10 minutes (with sleeps)

# After sleep elimination
time cargo test --tests
# Target: ~5 minutes (proper sync, concurrent)
```

### Flaky Test Detection

```bash
# Run tests 10 times
for i in {1..10}; do
    cargo test --tests || echo "FLAKE DETECTED: Run $i"
done

# Target: 0 flakes
```

---

## 🎯 SUCCESS METRICS

### Quantitative
- **Sleeps Eliminated**: 250+ / 329 (75%+)
- **Test Execution Time**: 50% reduction
- **Flaky Tests**: 0
- **Concurrent Tests**: 95%+ (except chaos)

### Qualitative
- Tests are deterministic and robust
- Production code demonstrates proper async patterns
- No timing assumptions in test logic
- Clear synchronization boundaries

---

## 📚 REFERENCE FILES

### Existing Guides
- `tests/SLEEP_MIGRATION_GUIDE.md` - Migration patterns
- `docs/guides/TESTING_MODERN.md` - Modern test patterns
- `tests/common/modern_sync.rs` - Existing sync utilities

### Created During This Session
- `DEEP_DEBT_EVOLUTION_JAN_13_2026.md` - Overall evolution plan
- `COMPREHENSIVE_CODEBASE_AUDIT_JAN_13_2026.md` - Full audit report

---

**Status**: Phase 1 Complete (Build Fixed), Phase 2 Ready (Sleep Elimination)  
**Next Action**: Create sync utilities and start fixing high-priority files  
**Commitment**: No shortcuts, no timing assumptions, proper concurrency
