# Hardcoding Removal - Batch 2

**Date**: December 10, 2025 (Continued)  
**Target**: Remaining `build_api_url()` usages  
**Status**: IN PROGRESS

---

## ANALYSIS

### Current Usages of `build_api_url()`

Found in 6 files:
1. `universal_adapter/capability_system.rs` - ✅ **ALREADY FIXED** (using ServiceRegistry)
2. `ecosystem_integration/mod.rs` - ⏳ To review
3. `constants/canonical_defaults.rs` - 📍 Definition site
4. `canonical_modernization/canonical_constants.rs` - ⏳ To review
5. `universal_primal_discovery/registry.rs` - ⏳ To review
6. `sovereignty_config.rs` - ⏳ To review

### Strategy

**Phase 1**: Mark `build_api_url()` as deprecated ✅  
**Phase 2**: Update remaining call sites  
**Phase 3**: Remove function entirely (future)

---

## DEPRECATION APPROACH

### Add Deprecation Warning
```rust
#[deprecated(
    since = "0.10.0",
    note = "Use ServiceRegistry for capability-based discovery instead of hardcoded URLs"
)]
pub fn build_api_url() -> String {
    safe_env_var_or_default("NESTGATE_API_URL", DEFAULT_API_BASE_URL).to_string()
}
```

### Migration Path
```rust
// OLD (hardcoded fallback)
let url = build_api_url();

// NEW (capability discovery)
let registry = ServiceRegistry::new(capabilities).await?;
let service = registry.find_by_capability(&capability).await?;
let url = service.url();

// TRANSITIONAL (environment only)
let url = std::env::var("NESTGATE_API_URL")
    .map_err(|_| NestGateError::configuration_error(
        "NESTGATE_API_URL must be set - no hardcoded defaults"
    ))?;
```

---

## CALL SITE ANALYSIS

### 1. capability_system.rs
**Status**: ✅ FIXED (already uses ServiceRegistry)
**Action**: None needed

### 2. ecosystem_integration/mod.rs
**Status**: ⏳ TO REVIEW
**Likely pattern**: Ecosystem discovery
**Action**: Check if can use ServiceRegistry

### 3. canonical_defaults.rs
**Status**: 📍 DEFINITION SITE
**Action**: Add deprecation warning

### 4. canonical_modernization/canonical_constants.rs
**Status**: ⏳ TO REVIEW
**Context**: Likely migration/compatibility code
**Action**: Review and mark deprecated if needed

### 5. universal_primal_discovery/registry.rs
**Status**: ⏳ TO REVIEW
**Context**: Discovery system - might be fallback
**Action**: Remove or make env-only

### 6. sovereignty_config.rs
**Status**: ⏳ TO REVIEW
**Context**: Configuration initialization
**Action**: Use environment variable directly

---

## EXECUTION PLAN

### Step 1: Deprecate Function (5 min)
- Add `#[deprecated]` to `build_api_url()`
- Add migration note
- Document ServiceRegistry alternative

### Step 2: Review Call Sites (15-20 min)
- Check each remaining usage
- Determine if can use ServiceRegistry
- If not, use environment variable directly
- Document why hardcoded fallback removed

### Step 3: Update Call Sites (20-30 min)
- Migrate to ServiceRegistry where possible
- Use `std::env::var()` for config initialization
- Return errors instead of falling back to localhost

### Step 4: Test (10 min)
- Verify compilation
- Run tests
- Check clippy

---

## EXPECTED IMPACT

### Before
- `build_api_url()`: 6 call sites
- Hardcoded fallback: "localhost:8080"
- Silent defaults: No error when misconfigured

### After
- `build_api_url()`: Deprecated
- Call sites: 0-2 (only where truly needed)
- Explicit errors: Configuration required
- Discovery-first: ServiceRegistry primary

### Benefits
- ✅ No silent hardcoded defaults
- ✅ Explicit configuration required
- ✅ Discovery-based routing
- ✅ Multi-instance ready
- ✅ Clear error messages

---

## MIGRATION GUIDE

### For Application Code
```rust
// Don't use build_api_url() anymore!
// ❌ let url = build_api_url();

// ✅ Use ServiceRegistry for service discovery
let registry = ServiceRegistry::new(vec![
    PrimalCapability::Storage,
]).await?;

let service = registry
    .find_by_capability(&PrimalCapability::ApiGateway)
    .await?;

let url = service.url();
```

### For Configuration Code
```rust
// ❌ Don't use build_api_url() as fallback
// let url = build_api_url();

// ✅ Require explicit configuration
let url = std::env::var("NESTGATE_API_URL")
    .map_err(|_| NestGateError::configuration_error(
        "NESTGATE_API_URL environment variable must be set"
    ))?;
```

---

## NEXT ACTIONS

1. ⏳ Deprecate `build_api_url()` function
2. ⏳ Review remaining 5 call sites
3. ⏳ Migrate to ServiceRegistry or env-only
4. ⏳ Test and verify
5. ⏳ Update documentation

**Time Estimate**: 40-60 minutes

---

**Status**: READY TO EXECUTE  
**Priority**: HIGH (removes hardcoded defaults)  
**Impact**: ~70 more hardcoded URLs indirectly affected

