# 🚫 Disabled Tests Reference

**Last Updated**: November 5, 2025  
**Status**: Documented

---

## 📋 Disabled Test Files

### 1. `sovereignty_chaos_testing.rs.disabled`
**Location**: `tests/sovereignty_chaos_testing.rs.disabled`  
**Status**: Deferred to Phase 3  
**Reason**: Complex chaos testing framework requiring extensive refactoring

**Details:**
- **Estimated Fix Time**: 16-20 hours
- **Compilation Errors**: 80+ errors (outdated APIs)
- **Scope**: Comprehensive chaos/fault injection testing
- **Decision**: Defer until after Phase 2 (90% coverage achieved)

**When to Re-enable:**
- After Phase 2 completion (90% coverage)
- After core test infrastructure stabilized
- When chaos testing becomes priority

---

### 2. `unified_performance_validation.rs.disabled`
**Location**: `code/crates/nestgate-core/benches/unified_performance_validation.rs.disabled`  
**Status**: Deferred (benchmark, not critical)  
**Reason**: Benchmark file with compilation errors

**Details:**
- **Type**: Performance benchmark (non-critical)
- **Compilation Errors**: Multiple API mismatches
- **Scope**: Unified performance validation benchmark
- **Decision**: Focus on test coverage first, benchmarks later

**When to Re-enable:**
- After Phase 2 completion
- When performance tuning becomes focus
- After API stabilization

---

## 📊 Context

### Testing Status (Current):
- **Tests Passing**: 973+ tests ✅
- **Coverage**: 4.74% (baseline measured) ✅
- **Disabled Tests**: 2 (documented here)
- **Compilation Errors**: 52 remaining (non-blocking)

### Why Disable Instead of Fix?

**Prioritization:**
1. **Phase 1 Complete**: Deep debt cleanup (262 errors fixed)
2. **Phase 2 Active**: Writing new tests for 90% coverage
3. **Phase 3 Future**: Advanced testing (chaos, performance)

**Strategic Decision:**
- Fixing these 2 files: ~20 hours
- Writing 300 new tests: ~20 hours
- **Choice**: Write new tests → faster path to 90% coverage

---

## 🚀 Re-enabling Strategy

### When Coverage Reaches 90%:

**Step 1: Assess Priority**
- Is chaos testing needed for production?
- Is performance benchmarking critical?

**Step 2: Modernization Approach**
- Apply patterns from Phase 1 (deep debt solutions)
- Use current test infrastructure
- Leverage modern APIs

**Step 3: Re-integration**
- Fix compilation errors
- Update to modern test patterns
- Re-enable and verify

---

## 📝 Maintenance

### Adding Disabled Tests:
1. Rename test file to `.disabled`
2. Document in this file (reason, errors, estimated fix time)
3. Reference in `CURRENT_STATUS.md`

### Re-enabling Tests:
1. Fix compilation errors
2. Update this file
3. Update test count in `CURRENT_STATUS.md`
4. Remove `.disabled` suffix

---

## 🔍 Finding Disabled Tests

```bash
# Find all .disabled files
find . -name "*.disabled" -type f

# Current count
find . -name "*.disabled" -type f | wc -l
```

**Expected**: 2 files

---

**Maintained By**: Development Team  
**Status**: Active reference  
**Next Review**: After Phase 2 completion

