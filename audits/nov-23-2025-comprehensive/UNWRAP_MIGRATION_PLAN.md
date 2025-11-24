# 🔧 UNWRAP/EXPECT MIGRATION PLAN

**Created**: November 23, 2025  
**Status**: Active Migration  
**Target**: Remove all production unwrap/expect calls

---

## 📊 INITIAL AUDIT

### Discovery Summary
**Total Unwraps**: 1,090  
**Total Expects**: 1,949  
**Total Target**: ~1,400-1,700 production calls

### High-Risk Areas Identified
1. **Configuration Loading** (5+ unwraps)
   - `config/defaults_config.rs`
   - `config/network_defaults_v2_config.rs`
   - `config/sovereignty_config.rs`
   
2. **ZFS Operations** (4+ expects)
   - `orchestrator_integration.rs`
   - `types.rs`
   - `error.rs`

3. **API Handlers** (Mostly test files, low priority)

### Risk Classification
- 🔴 **HIGH**: API handlers, storage ops, config loading
- 🟡 **MEDIUM**: Service initialization, network operations
- 🟢 **LOW**: Builder patterns, test utilities
- ⚪ **IGNORE**: Test files (`*test*.rs`, `#[cfg(test)]`)

---

## 🎯 MIGRATION STRATEGY

### Phase 2A: Configuration Files (Week 1)
**Target**: Config loading unwraps  
**Files**: 3 config files  
**Calls**: ~5 unwraps  
**Risk**: 🔴 HIGH

**Pattern**:
```rust
// ❌ BEFORE
handle.await.unwrap();

// ✅ AFTER  
handle.await
    .map_err(|e| NestGateError::ConfigLoad {
        reason: format!("Task join failed: {:?}", e),
        context: "async config loading".to_string(),
    })?;
```

### Phase 2B: ZFS Operations (Week 1-2)
**Target**: Storage operation expects  
**Files**: 3 ZFS files  
**Calls**: ~4 expects  
**Risk**: 🔴 HIGH

**Pattern**:
```rust
// ❌ BEFORE
.expect("System time should be after UNIX epoch")

// ✅ AFTER
.map_err(|e| ZfsError::TimeError {
    reason: format!("System time error: {:?}", e),
    timestamp: SystemTime::now(),
})?
```

### Phase 2C: Core Operations (Week 2-3)
**Target**: Core library unwraps/expects  
**Files**: universal_adapter, discovery, etc.  
**Calls**: ~100-200  
**Risk**: 🟡 MEDIUM

### Phase 2D: Network & Services (Week 3-4)
**Target**: Network client, service discovery  
**Files**: network/, services/  
**Calls**: ~200-300  
**Risk**: 🟡 MEDIUM

---

## 🔨 MIGRATION PATTERNS

### Pattern 1: Task Join Errors
```rust
// ❌ OLD
handle.await.unwrap();

// ✅ NEW
handle.await.map_err(|e| Error::TaskJoinFailed {
    task: "config_loader",
    error: e.to_string(),
})?;
```

### Pattern 2: Time Operations
```rust
// ❌ OLD
SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards");

// ✅ NEW
SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .map_err(|e| Error::TimeError {
        reason: "System time before UNIX epoch".to_string(),
        details: e.to_string(),
    })?
```

### Pattern 3: Serialization
```rust
// ❌ OLD
let json = serde_json::to_string(&data).expect("Serialization failed");

// ✅ NEW
let json = serde_json::to_string(&data)
    .map_err(|e| Error::SerializationError {
        type_name: std::any::type_name::<Data>(),
        error: e.to_string(),
    })?;
```

### Pattern 4: Lock Poisoning
```rust
// ❌ OLD
let guard = mutex.lock().unwrap();

// ✅ NEW
let guard = mutex.lock().map_err(|e| Error::LockPoisoned {
    lock_name: "data_mutex",
    details: e.to_string(),
})?;
```

---

## 📋 WEEK 1 PLAN

### Day 1-2: Configuration Files
- [ ] Migrate `config/defaults_config.rs` (2 unwraps)
- [ ] Migrate `config/network_defaults_v2_config.rs` (1 unwrap)
- [ ] Migrate `config/sovereignty_config.rs` (2 unwraps)
- [ ] Add error variants to `NestGateError`
- [ ] Test error paths

### Day 3-4: ZFS Operations
- [ ] Migrate `orchestrator_integration.rs` (1 expect)
- [ ] Migrate `types.rs` (2 expects)
- [ ] Migrate `error.rs` (1 expect)
- [ ] Add error variants to `ZfsError`
- [ ] Test storage error handling

### Day 5: Review & Test
- [ ] Run full test suite
- [ ] Verify error messages
- [ ] Update documentation
- [ ] Code review
- [ ] Commit changes

**Goal**: 10-15 migrations complete ✅

---

## 🎯 SUCCESS METRICS

### Week 1
- [ ] 10-15 HIGH-risk unwraps/expects migrated
- [ ] 0 test failures introduced
- [ ] Error messages are clear and actionable
- [ ] Documentation updated

### Week 2-3
- [ ] 100-150 MEDIUM-risk migrations
- [ ] Core modules fully migrated
- [ ] Error handling patterns documented
- [ ] Team reviewed and approved

### Week 4
- [ ] All production unwraps <100
- [ ] All production expects <100
- [ ] Comprehensive error handling tests
- [ ] Migration guide published

---

## 🚫 WHAT NOT TO MIGRATE

### Test Code
- Files ending in `_test.rs`, `_tests.rs`
- Code in `#[cfg(test)]` blocks
- Code in `tests/` directories

**Reason**: Tests should fail fast. `.unwrap()` and `.expect()` are appropriate in tests.

### Builder Patterns (Some Cases)
```rust
// This is OK - builder should panic on invalid construction
SomeBuilder::new()
    .field(value)
    .build()
    .unwrap() // OK if builder validates
```

### Infallible Operations
```rust
// This is OK - Arc::try_unwrap when we know count=1
let inner = Arc::try_unwrap(arc).unwrap();
```

---

## 📝 COMMIT STRATEGY

### Atomic Commits
Each commit should:
1. Migrate one logical unit (1 file or related group)
2. Pass all tests
3. Include updated error types if needed
4. Have clear commit message

### Commit Message Template
```
refactor: migrate unwrap/expect in {module}

- Remove {N} unwrap() calls from {file}
- Add {ErrorVariant} to handle {specific_case}
- All tests passing
- Improves error context for {operation}

Part of: Safety Migration Phase 2
Issue: #unwrap-migration
```

---

## 🎓 LESSONS LEARNED (To Be Updated)

### Week 1
- TBD after first week

### Week 2-3
- TBD

### Week 4
- TBD

---

**Status**: Ready to begin  
**First Target**: `config/defaults_config.rs`  
**Next Review**: End of Week 1

