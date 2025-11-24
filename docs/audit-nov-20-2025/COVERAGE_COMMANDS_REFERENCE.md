# 📊 Coverage Commands Reference

Quick reference for measuring test coverage correctly in NestGate.

---

## ✅ CORRECT COMMANDS

### Full Coverage Report (Recommended)
```bash
cargo llvm-cov --workspace --all-features --lib --tests --html --output-dir coverage
```

**What it measures**:
- ✅ All workspace crates
- ✅ All features enabled
- ✅ Library code
- ✅ Integration tests execution
- ✅ Complete picture

**Result**: **66.64%** coverage

### Quick Summary (Fast)
```bash
cargo llvm-cov --workspace --all-features --lib --tests --summary-only
```

**Output**:
```
Function Coverage: 66.64% (9,689/14,539)
Line Coverage:     65.90% (71,151/107,963)
Region Coverage:   67.79% (98,756/145,685)
```

### Using Makefile (Easiest)
```bash
# Full HTML report
make -f Makefile.coverage coverage

# Quick summary
make -f Makefile.coverage coverage-summary

# Generate and open in browser
make -f Makefile.coverage coverage-open
```

---

## ❌ INCORRECT COMMANDS

### Too Limited (4.44% - Wrong!)
```bash
cargo llvm-cov --html
```

**Problem**: Only measures library code, no integration tests.  
**Result**: **4.44%** (incomplete!)

### Fails to Compile
```bash
cargo llvm-cov --workspace --all-features --all-targets --html
```

**Problem**: Some test targets don't compile with `--all-targets`.  
**Result**: Compilation error

---

## 📊 WHAT GETS MEASURED

### Included in Coverage ✅
- Production code in `code/crates/*/src/`
- Library functions and methods
- Public APIs
- Internal implementations
- Code executed by unit tests
- Code executed by integration tests

### Excluded from Coverage ❌
- Test code itself (`tests/`, `*_test.rs`)
- Benchmark code (`benches/`)
- Example code (`examples/`)
- Build scripts (`build.rs`)
- Archive/fossil code

**Note**: This is CORRECT! We measure production code, not test infrastructure.

---

## 🎯 COVERAGE TARGETS

| Metric | Current | Target | Gap |
|--------|---------|--------|-----|
| **Lines** | 65.90% | 90% | 24.1% |
| **Functions** | 66.64% | 90% | 23.4% |
| **Regions** | 67.79% | 90% | 22.2% |

**Lines to add**: ~26,000 more lines need tests  
**Tests to add**: ~1,000-1,500 more tests  
**Timeline**: 4-8 weeks

---

## 🔧 CONFIGURATION FILES

### .llvm-cov.toml
Created in project root for consistent coverage measurement.

**Key settings**:
- Workspace: true
- All features: true
- HTML output: true
- Precision: 2 decimal places
- Excludes: tests/, benches/, examples/

### Makefile.coverage
Created in project root for convenient commands.

**Targets**:
- `coverage` - Generate HTML report
- `coverage-summary` - Text summary
- `coverage-open` - Generate and open
- `coverage-clean` - Clean artifacts
- `coverage-weekly` - Log weekly progress

---

## 📈 TRACKING PROGRESS

### Weekly Check
```bash
make -f Makefile.coverage coverage-weekly
```

Appends to `coverage-history.log`:
```
Fri Nov 20 2025
Function Coverage: 66.64%
Line Coverage:     65.90%
Region Coverage:   67.79%
---
```

### Before Each PR
```bash
# Check coverage didn't decrease
make -f Makefile.coverage coverage-summary
```

### Monthly Deep Dive
```bash
# Full HTML analysis
make -f Makefile.coverage coverage-open

# Identify uncovered modules
# Add tests for critical paths
```

---

## 🎯 FINDING GAPS

### View HTML Report
```bash
make -f Makefile.coverage coverage-open
```

Look for:
- ❌ Red files (0-50% coverage)
- ⚠️ Yellow files (50-80% coverage)
- ✅ Green files (80-100% coverage)

### Focus Areas
**Priority 1 - 0% Coverage**:
- `network/client.rs`
- `network/native_async/production.rs`
- `services/storage/service.rs`
- `observability/` modules
- `performance/advanced_optimizations.rs`

**Priority 2 - Low Coverage (<50%)**:
- Various error paths
- Edge cases
- Async operations

**Priority 3 - Moderate Coverage (50-70%)**:
- Universal adapter edge cases
- Storage backend operations
- Security provider paths

---

## 🚀 CI/CD INTEGRATION

### GitHub Actions (Future)
```yaml
- name: Coverage
  run: |
    cargo llvm-cov --workspace --all-features --lib --tests --lcov --output-path lcov.info
    
- name: Upload to Codecov
  uses: codecov/codecov-action@v3
  with:
    file: ./lcov.info
```

### Local CI Simulation
```bash
make -f Makefile.coverage ci-coverage
```

Generates:
- `lcov.info` - For coverage services
- `coverage.json` - For programmatic analysis

---

## 📋 QUICK REFERENCE CARD

```bash
# Daily development
cargo test                           # Run tests
make -f Makefile.coverage coverage-summary  # Check coverage

# Before PR
cargo test --workspace               # All tests pass
make -f Makefile.coverage coverage-summary  # Coverage maintained

# Weekly tracking
make -f Makefile.coverage coverage-weekly   # Log progress

# Deep analysis
make -f Makefile.coverage coverage-open     # Full HTML report
```

---

## ✅ BEST PRACTICES

1. **Run Coverage Regularly**
   - Weekly minimum
   - Before each PR
   - After major features

2. **Focus on Critical Paths**
   - Core business logic first
   - Error handling second
   - Edge cases third

3. **Don't Chase 100%**
   - 90% is the target (excellent)
   - 95%+ is optional (diminishing returns)
   - 100% is unrealistic (some code is uncoverable)

4. **Track Trends**
   - Coverage should increase over time
   - Never let it decrease significantly
   - Monitor weekly logs

5. **Use Correct Command**
   - Always use `--lib --tests`
   - Always use `--workspace --all-features`
   - Use Makefile for consistency

---

## 🎓 UNDERSTANDING THE METRICS

### Function Coverage (66.64%)
**Measures**: How many functions have at least one line executed by tests.

**Example**:
```rust
// This function has 66.64% coverage across codebase
fn example() {
    if condition {  // Executed in test
        do_work();  // Executed in test
    } else {
        panic!();   // NOT executed (33.36% uncovered)
    }
}
```

### Line Coverage (65.90%)
**Measures**: How many lines of code are executed by tests.

**Most granular** and **most important** metric.

### Region Coverage (67.79%)
**Measures**: How many execution regions (branches, conditions) are covered.

**Most detailed** - tracks all possible execution paths.

---

## 🔍 TROUBLESHOOTING

### "Coverage seems too low"
✅ Make sure you're using `--lib --tests` flags!

### "Compilation error with llvm-cov"
❌ Remove `--all-targets` flag, use `--lib --tests` instead.

### "Numbers don't match previous reports"
✅ Check if you're comparing lib-only vs lib+tests measurements.

### "Coverage report missing files"
✅ Make sure files are included in workspace members.

---

**Status**: ✅ **COMPLETE**  
**Current Coverage**: **66.64%**  
**Measurement**: **Verified Accurate**  
**Commands**: **Tested and Working**

*Use `Makefile.coverage` for all future measurements!*

