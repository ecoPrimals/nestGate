# File Split Plan: canonical_unified_traits.rs

**Current**: 1,100 lines (10% over limit)  
**Target**: Multiple files < 1000 lines each  
**Priority**: Medium (documented exception exists, but should still fix)

## Proposed Structure

```
traits/
├── canonical_unified_traits.rs  (DEPRECATED - remove after migration)
└── canonical/
    ├── mod.rs              (~150 lines) - Re-exports + CanonicalService
    ├── provider.rs         (~100 lines) - CanonicalProvider trait
    ├── storage.rs          (~290 lines) - CanonicalStorage trait
    ├── network.rs          (~200 lines) - CanonicalNetwork trait
    ├── security.rs         (~300 lines) - CanonicalSecurity trait
    └── types.rs            (~180 lines) - Supporting types
```

## Section Breakdown

### 1. mod.rs (Lines 1-151)
- Module documentation
- CanonicalService trait
- Core service operations
- Re-exports all other modules

### 2. provider.rs (Lines 152-254)
- CanonicalProvider trait
- Provider operations
- Provider types

### 3. storage.rs (Lines 255-463 + storage types)
- CanonicalStorage trait
- Storage operations
- StorageUsageStats
- StorageBackendType
- StorageCapability
- SnapshotInfo

### 4. network.rs (Lines 464-636)
- CanonicalNetwork trait
- Network operations
- ConnectionHandle
- ConnectionStatus

### 5. security.rs (Lines 637-917)
- CanonicalSecurity trait
- Security operations
- SecurityCredentials

### 6. types.rs (Lines 941-1100)
- ServiceCapabilities
- ProviderHealth
- ProviderCapabilities
- HealthStatus
- CronSchedule
- ScheduleId
- ScheduleInfo
- DataStream
- WriteStream

## Migration Steps

1. Create `canonical/` directory ✅
2. Create `types.rs` with all supporting types
3. Create `provider.rs` with CanonicalProvider
4. Create `storage.rs` with CanonicalStorage + types
5. Create `network.rs` with CanonicalNetwork
6. Create `security.rs` with CanonicalSecurity
7. Create `mod.rs` with CanonicalService + re-exports
8. Update all imports in codebase
9. Test compilation
10. Remove old file

## Testing Strategy

```bash
# After each module creation:
cargo build --lib

# After all modules created:
cargo test --lib
cargo clippy --lib

# Verify no regressions:
cargo test --all
```

## Timeline

**Estimated**: 3-4 hours for safe migration  
**Risk**: Medium (affects many files)  
**Benefit**: 100% file size compliance

## Notes

- File currently has documented exception (Nov 12, 2025)
- Exception was for 1,075 lines (7.5% over)
- Current state is 1,100 lines (10% over)
- Exception rationale still valid but limit exceeded

## Status

**Status**: ⏳ **PLANNED** - Ready to execute when time available  
**Priority**: Medium (exception exists but should fix)  
**Blocked**: No  
**Next**: Execute during dedicated refactoring session

