# 🧹 NestGate Archive Cleanup Plan - January 18, 2026

**Goal**: Remove obsolete archive code while preserving documentation as fossil record  
**Status**: 🔄 IN PROGRESS

---

## 📋 Items to Clean

### 1. Obsolete Source Files ❌ DELETE

**Files to Remove**:
- ✅ `code/crates/nestgate-bin/src/main_new.rs` - Old main file (370 lines)
- ✅ `code/crates/nestgate-bin/src/bin/nestgate-client.rs` - Obsolete (UniBin replaced)

**Reason**: 
- `main_new.rs`: Old implementation, current `main.rs` is active
- `nestgate-client.rs`: Separate binary removed in UniBin consolidation

**Impact**: No functional impact, these are not compiled

---

### 2. Archive Documentation ✅ KEEP

**Files to Preserve** (fossil record):
- ✅ `docs/archive/old-status/CURRENT_STATUS.md`
- ✅ `docs/archive/old-status/HARDCODING_ELIMINATION_STRATEGY.md`
- ✅ `docs/archive/old-status/CAPABILITY_DISCOVERY_MIGRATION_GUIDE.md`

**Reason**: Historical documentation, shows evolution

---

### 3. TODO/FIXME Audit 🔍 REVIEW

**Found**: 25 instances across 15 files

**Categories**:
1. **Strategic TODOs** (keep) - Future features, architectural notes
2. **Completed TODOs** (remove) - Already implemented
3. **Obsolete TODOs** (remove) - No longer relevant

**Action**: Review each individually

---

### 4. Test Files 🧪 KEEP

**Files**:
- ✅ `tests/e2e_scenario_60_64_backup_recovery.rs` - Active test
- ✅ `tests/e2e_scenario_39_backup_restore.rs` - Active test

**Reason**: "backup" in name refers to backup/restore functionality, not backup files

---

## 🎯 Cleanup Actions

### Phase 1: Remove Obsolete Source (SAFE) ✅

```bash
# Remove old main file
rm code/crates/nestgate-bin/src/main_new.rs

# Remove obsolete client binary
rm code/crates/nestgate-bin/src/bin/nestgate-client.rs
```

**Impact**: None - these files are not referenced in Cargo.toml or compiled

---

### Phase 2: TODO/FIXME Review (CAREFUL) 🔍

Review 25 instances:
- Keep strategic/architectural TODOs
- Remove completed TODOs
- Remove obsolete TODOs

**Files to Review**:
1. `code/crates/nestgate-core/src/zero_cost_security_provider/authentication.rs` (1)
2. `code/crates/nestgate-core/src/lib.rs` (1)
3. `code/crates/nestgate-bin/src/commands/service.rs` (1)
4. `code/crates/nestgate-api/src/transport/server.rs` (1)
5. `code/crates/nestgate-api/src/transport/security.rs` (1)
6. `code/crates/nestgate-core/src/universal_storage/types/config.rs` (1)
7. `code/crates/nestgate-core/src/storage/pipeline.rs` (1)
8. `code/crates/nestgate-core/src/services/storage/mod.rs` (2)
9. `code/crates/nestgate-core/src/rpc/tarpc_client.rs` (1)
10. `code/crates/nestgate-zfs/src/backends/azure.rs` (3)
11. `code/crates/nestgate-core/src/universal_primal_discovery/production_capability_bridge.rs` (3)
12. `code/crates/nestgate-core/src/universal_primal_discovery/backends/mdns.rs` (2)
13. `code/crates/nestgate-core/src/temporal_storage/device.rs` (3)
14. `code/crates/nestgate-core/src/error/strategic_error_tests_phase1.rs` (2)
15. `code/crates/nestgate-api/src/dev_stubs/zfs/types.rs` (2)

---

### Phase 3: Git Cleanup (FINAL) 🚀

After cleanup:
```bash
# Stage changes
git add -A

# Commit with clear message
git commit -m "chore: remove obsolete archive code (main_new.rs, nestgate-client.rs)

- Remove old main_new.rs (370 lines, obsolete)
- Remove nestgate-client.rs binary (UniBin replaced)
- Preserve documentation as fossil record
- No functional impact, cleanup only"

# Push via SSH
git push origin main
```

---

## 📊 Cleanup Summary

| Category | Action | Count | Impact |
|----------|--------|-------|--------|
| **Obsolete Source** | ❌ DELETE | 2 files | None (not compiled) |
| **Archive Docs** | ✅ KEEP | 3 files | Fossil record |
| **Test Files** | ✅ KEEP | 2 files | Active tests |
| **TODOs** | 🔍 REVIEW | 25 instances | TBD |

---

## ✅ Execution Status

- [ ] Phase 1: Remove obsolete source files
- [ ] Phase 2: Review and clean TODOs
- [ ] Phase 3: Git commit and push

---

**Created**: January 18, 2026  
**Status**: Ready for execution
