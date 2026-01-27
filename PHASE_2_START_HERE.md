# 🚀 Phase 2 Start Here - Capability Discovery Migration

**Date**: January 27, 2026  
**Status**: Ready to Execute  
**Phase**: Phase 2a - TRUE PRIMAL Compliance  
**Target Grade**: A (93/100)

---

## ✅ FOUNDATION COMPLETE

- ✅ Phase 1 Complete - A- (90/100)
- ✅ All critical blockers resolved
- ✅ Build/test infrastructure solid
- ✅ CapabilityDiscovery module ready (348 lines, 81 tests)
- ✅ Patterns established and documented

---

## 🎯 FIRST MIGRATION: Songbird Registration

**File**: `code/crates/nestgate-core/src/rpc/songbird_registration.rs`

**Current State**:
- ⚠️ 73 hardcoded "songbird" references
- ✅ Already marked DEPRECATED since v2.3.0
- ✅ Migration path documented in deprecation notice

**Goal**: Replace direct Unix socket registration with capability-based IPC discovery

---

## 📋 STEP-BY-STEP EXECUTION

### **Step 1: Examine Current Implementation** (15 min)

```bash
# Read the deprecated module
code code/crates/nestgate-core/src/rpc/songbird_registration.rs

# Check usage sites
rg "SongbirdRegistration" --type rust
```

**What to Look For**:
- Registration flow
- Call sites
- Error handling patterns
- Test dependencies

---

### **Step 2: Review Replacement Module** (15 min)

```bash
# Read the new capability discovery module
code code/crates/nestgate-core/src/capability_discovery.rs

# Review Songbird IPC bootstrap
code code/crates/nestgate-core/src/capability_discovery/songbird_ipc_bootstrap.rs

# Check existing tests
cargo test capability_discovery
```

**Key Functions**:
- `CapabilityDiscovery::discover_songbird_ipc()` - Discovers Songbird IPC service
- `query_capability(capability: &str)` - Queries for capability providers
- `call_method(method: &str, params: Value)` - Calls JSON-RPC methods

---

### **Step 3: Update Call Sites** (1-2 hours)

**Pattern to Apply**:

```rust
// ❌ OLD: Direct Unix socket registration
use crate::rpc::songbird_registration::SongbirdRegistration;

let registration = SongbirdRegistration::new(&family_id).await?;
registration.register().await?;

// ✅ NEW: Capability-based IPC discovery
use crate::capability_discovery::CapabilityDiscovery;

let discovery = CapabilityDiscovery::discover_songbird_ipc().await?;
// Register via JSON-RPC call to "service.register" semantic method
discovery.call_method(
    "service.register",
    serde_json::json!({
        "service_id": family_id,
        "capabilities": ["storage", "encryption"],
        "socket_path": "/primal/nestgate"
    })
).await?;
```

**Files to Update**:
1. Find all uses: `rg "SongbirdRegistration" --type rust -l`
2. Update imports
3. Replace initialization
4. Replace method calls
5. Update error handling

---

### **Step 4: Remove Deprecated Module** (15 min)

```bash
# After all call sites updated
rm code/crates/nestgate-core/src/rpc/songbird_registration.rs

# Update mod.rs
# Remove: pub mod songbird_registration;
# Remove: pub use songbird_registration::*;
```

---

### **Step 5: Verify** (30 min)

```bash
# Build
cargo build --all-targets --all-features

# Test
cargo test

# Clippy
cargo clippy --all-targets --all-features -- -D warnings

# Format
cargo fmt --all
```

**Expected Results**:
- ✅ All builds succeed
- ✅ All tests pass
- ✅ Zero clippy errors
- ✅ Code formatted

---

### **Step 6: Document** (15 min)

Update `EXECUTION_PROGRESS_JAN_27_2026.md`:

```markdown
## Phase 2a - Capability Discovery Migration

### Batch 1: Songbird Registration (COMPLETE)

**Changes**:
- ✅ Removed `rpc/songbird_registration.rs` (308 lines)
- ✅ Updated X call sites to use CapabilityDiscovery
- ✅ Eliminated 73 hardcoded "songbird" references
- ✅ All tests passing

**Impact**:
- Hardcoded primal names: 378 → 305 (-73)
- TRUE PRIMAL compliance: +19%
- Grade: A- (90/100) → A- (90.5/100)
```

---

## 📊 SUCCESS METRICS

### **After This Batch**:
- 🎯 Hardcoded primal names: **562 → 489** (-73, 13% reduction)
- 🎯 Pattern established: **Replicable for remaining 305 refs**
- 🎯 Grade: **A- (90/100) → A- (90.5/100)**

### **After Full Capability Migration** (5-6 more batches):
- 🎯 Hardcoded primal names: **562 → 0** (100% elimination)
- 🎯 TRUE PRIMAL compliance: **Complete**
- 🎯 Grade: **A- (90/100) → A (93/100)** (+3 points)

---

## 🔄 NEXT BATCHES (After Batch 1 Complete)

### **Batch 2**: Service Metadata Storage (51 refs, 2-3 hours)
- File: `service_metadata/mod.rs`
- Add discovery layer for runtime resolution
- Cache discovered endpoints

### **Batch 3**: External Services Config (50 refs, 2-3 hours)
- Files: `config/external/services.rs`, `config/external/services_config.rs`
- Environment-driven discovery hints
- Remove hardcoded paths

### **Batch 4**: Examples & Tests (48 refs, 2-3 hours)
- Update documentation examples
- Fix test mocks to use discovery
- Remove deprecated examples

### **Batch 5-8**: Remaining files (2-3 hours each)
- Systematic cleanup of remaining references
- Establish patterns for each context
- Comprehensive verification

---

## 💡 TIPS FOR SUCCESS

### **Go Slow, Go Deep**:
- Don't rush through the migration
- Understand each use case
- Apply deep solutions, not quick fixes
- Document patterns as you discover them

### **Test Continuously**:
- Run tests after each file update
- Don't let errors accumulate
- Fix issues immediately
- Keep main branch clean

### **Ask Questions**:
- If a pattern is unclear, stop and analyze
- Check the wateringHole/ standards
- Review existing capability_discovery tests
- Document ambiguities

### **Document Everything**:
- Update progress tracking in real-time
- Note unexpected patterns
- Record decisions made
- Make it easy for next session

---

## 📚 KEY REFERENCES

1. **DEEP_DEBT_MIGRATION_ROADMAP_JAN_27_2026.md** - Full strategy
2. **COMPREHENSIVE_COMPLIANCE_AUDIT_JAN_27_2026.md** - Gap analysis
3. **UNIVERSAL_IPC_EVOLUTION_PLAN_JAN_19_2026.md** - IPC architecture
4. **wateringHole/PRIMAL_IPC_PROTOCOL.md** - IPC standard
5. **wateringHole/SEMANTIC_METHOD_NAMING_STANDARD.md** - Naming standard

---

## 🎯 EXECUTE

**Ready?** Run this to start:

```bash
# Start with examination
code code/crates/nestgate-core/src/rpc/songbird_registration.rs
code code/crates/nestgate-core/src/capability_discovery.rs

# Find call sites
rg "SongbirdRegistration" --type rust
```

**Time Estimate**: **3-4 hours** for complete Batch 1

**Confidence**: **VERY HIGH** - Foundation ready, pattern clear

---

**🦀 Foundation solid. Pattern clear. Execute with confidence. 🚀**
