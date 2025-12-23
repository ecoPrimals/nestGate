# 🚀 NestGate Quick Reference

**Fast commands and common operations for daily development.**

---

## 🔥 Most Common Commands

### Build & Test
```bash
# Full workspace build
cargo build --workspace

# Quick library test
cargo test --workspace --lib

# Watch mode (if installed)
cargo watch -x "test --lib"
```

### Quality Checks
```bash
# Clippy
cargo clippy --workspace --lib -- -D warnings

# Format
cargo fmt --check
cargo fmt  # Apply formatting

# All checks
cargo build && cargo test --workspace --lib && cargo clippy --workspace --lib
```

### Coverage
```bash
# Generate HTML coverage
cargo llvm-cov --workspace --lib --html

# Open report
open target/llvm-cov/html/index.html  # macOS
xdg-open target/llvm-cov/html/index.html  # Linux
```

---

## 🔍 Finding Work

### Hardcoding
```bash
# Find hardcoded ports
rg "8080|8081|8091|9090" code/crates/nestgate-{core,api}/src --type rust

# Find hardcoded IPs
rg "127\.0\.0\.1|0\.0\.0\.0|localhost" code/crates/nestgate-{core,api}/src --type rust

# Find hardcoded primal names
rg "beardog|songbird|squirrel|toadstool" code/crates/nestgate-{core,api}/src --type rust -i
```

### Unwraps
```bash
# Find production unwraps
rg "\.unwrap\(\)" code/crates/nestgate-api/src/handlers --type rust

# Find expects
rg "\.expect\(" code/crates/nestgate-api/src --type rust

# Find all unwraps with context
rg "\.unwrap\(\)" code/crates/nestgate-{core,api}/src --type rust -C 2
```

### TODOs
```bash
# Find all TODOs
rg "TODO|FIXME|XXX|HACK" code/crates --type rust

# Count by priority
rg "TODO" code/crates --type rust | wc -l
```

### Large Files
```bash
# Files over 1000 lines (should be 0)
find code/crates -name "*.rs" -exec wc -l {} + | awk '$1 > 1000 {print $1, $2}'

# Largest files
find code/crates -name "*.rs" -exec wc -l {} + | sort -rn | head -20
```

---

## 📊 Quick Status

### Metrics
```bash
# Test count
rg "#\[test\]" code/crates --type rust | wc -l

# Unsafe blocks
rg "unsafe \{" code/crates --type rust | wc -l

# File count
find code/crates -name "*.rs" | wc -l

# Line count
find code/crates -name "*.rs" -exec wc -l {} + | tail -1
```

### Status Files
```bash
# Quick status (5 min)
cat AUDIT_QUICK_SUMMARY_DEC_19_2025.md

# Current metrics
cat STATUS.md

# Next steps
cat START_NEXT_SESSION_DEC_19_2025.md
```

---

## 🛠️ Development

### Running Services
```bash
# Start local development
./start_local_dev.sh

# Start with Songbird integration
./start_with_songbird.sh

# Stop services
./stop_local_dev.sh
```

### Specific Tests
```bash
# Run specific package tests
cargo test --package nestgate-core --lib

# Run specific test
cargo test --package nestgate-core --lib test_name

# Run with output
cargo test --package nestgate-core --lib -- --nocapture
```

### Benchmarks
```bash
# Run all benchmarks
cargo bench --workspace

# Run specific benchmark
cargo bench --bench simple_perf_benchmark
```

---

## 🎯 Common Tasks

### Apply Port Discovery Pattern
```bash
# 1. Find hardcoded port
rg "let.*port.*=.*8080" code/crates/nestgate-core/src

# 2. Replace with discovery
# Before:
let api_port = 8080;

# After:
use nestgate_core::config::capability_port_discovery;
let api_port = capability_port_discovery::discover_api_port().await?;
```

### Eliminate Unwrap
```bash
# Before:
let value = map.get("key").unwrap();

# After:
let value = map.get("key")
    .ok_or_else(|| NestGateError::missing_key("key", "context"))?;
```

### Add Error Path Test
```rust
#[test]
fn test_error_case() {
    // Setup error condition
    std::env::remove_var("CRITICAL_VAR");
    
    // Execute
    let result = function_under_test();
    
    // Verify error
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("expected error"));
}
```

---

## 📂 Important Files

### Entry Points
- `00_START_HERE.md` - Project overview
- `README.md` - Quick start
- `STATUS.md` - Current metrics
- `DOCUMENTATION_INDEX.md` - All docs

### Current Work
- `START_NEXT_SESSION_DEC_19_2025.md` - Getting started
- `READINESS_CHECKLIST_DEC_19_2025.md` - Week-by-week tasks
- `DEEP_DEBT_SOLUTIONS_APPLIED_DEC_19_2025.md` - Patterns

### Technical Details
- `HARDCODING_ELIMINATION.md` - Hardcoding strategy
- `UNWRAP_MIGRATION_PROGRESS.md` - Unwrap strategy
- `UNSAFE_CODE_AUDIT.md` - Unsafe analysis
- `TEST_COVERAGE_PLAN.md` - Testing strategy

---

## 🎨 Code Patterns

### Capability Discovery
```rust
// 3-layer discovery: Capability → Env → Default
use nestgate_core::config::capability_port_discovery;

let port = capability_port_discovery::discover_api_port().await?;
```

### Error Handling
```rust
// Rich context errors
let value = map.get("key")
    .ok_or_else(|| NestGateError::missing_key("key", "user context"))?;

// Parse with context
let port: u16 = port_str.parse()
    .map_err(|e| NestGateError::parse("port", port_str, e))?;
```

### Feature Gates
```rust
// Development-only code
#[cfg(feature = "dev-stubs")]
pub mod stubs {
    // Mock implementations
}
```

### Safe Alternatives
```rust
// Prefer safe alternatives
use nestgate_core::performance::safe_optimizations::SafeRingBuffer;
// Instead of: unsafe { ... }
```

---

## 📈 Progress Tracking

### Daily Checklist
- [ ] Check `STATUS.md` for current grade
- [ ] Review `READINESS_CHECKLIST_DEC_19_2025.md` for tasks
- [ ] Run `cargo clippy --workspace --lib`
- [ ] Run `cargo test --workspace --lib`
- [ ] Apply 5-10 pattern migrations
- [ ] Update progress docs

### Weekly Goals
- [ ] Review comprehensive audit
- [ ] Complete 50-100 migrations
- [ ] Add 20-30 tests
- [ ] Improve coverage by 2-3%
- [ ] Update roadmap

---

## 🔧 Troubleshooting

### Build Issues
```bash
# Clean build
cargo clean
cargo build --workspace

# Check for conflicts
cargo tree --duplicates
```

### Test Failures
```bash
# Run with output
cargo test --workspace --lib -- --nocapture

# Run single test
cargo test --package nestgate-core --lib test_name -- --nocapture
```

### Clippy Issues
```bash
# See all warnings
cargo clippy --workspace --lib

# Auto-fix where possible
cargo clippy --workspace --fix --allow-dirty --lib

# Explain specific error
cargo clippy --explain E0308
```

---

## 📞 Quick Links

**Documentation**:
- [Start Here](00_START_HERE.md)
- [Documentation Index](DOCUMENTATION_INDEX.md)
- [Architecture](ARCHITECTURE_OVERVIEW.md)

**Current Work**:
- [Quick Summary](AUDIT_QUICK_SUMMARY_DEC_19_2025.md)
- [Next Steps](START_NEXT_SESSION_DEC_19_2025.md)
- [Checklist](READINESS_CHECKLIST_DEC_19_2025.md)

**Technical**:
- [Comprehensive Audit](COMPREHENSIVE_AUDIT_REPORT_DEC_19_2025.md)
- [Deep Debt Solutions](DEEP_DEBT_SOLUTIONS_APPLIED_DEC_19_2025.md)
- [Patterns](code/crates/nestgate-core/examples/migration_before_after.rs)

**Operations**:
- [Runbook](OPERATIONS_RUNBOOK.md)
- [Deploy](QUICK_DEPLOY.sh)

---

## 💡 Pro Tips

1. **Start Small**: Pick 5-10 easy migrations to build momentum
2. **Use Patterns**: Copy from `migration_before_after.rs` and `DEEP_DEBT_SOLUTIONS_APPLIED_DEC_19_2025.md`
3. **Test Early**: Run tests after each migration batch
4. **Track Progress**: Update `STATUS.md` weekly
5. **Stay Focused**: One pattern at a time (ports, then unwraps, then tests)

---

**Last Updated**: December 19, 2025  
**Quick Help**: See [`00_START_HERE.md`](00_START_HERE.md) or [`DOCUMENTATION_INDEX.md`](DOCUMENTATION_INDEX.md)
