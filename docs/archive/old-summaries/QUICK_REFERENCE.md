# ⚡ **NestGate Quick Reference**

**Updated**: September 30, 2025  
**Status**: 88% Unified - 32 LegacyModuleError Remaining

---

## 🎯 **Current Status Check**

```bash
# Check LegacyModuleError count
grep -r "pub enum LegacyModuleError" code/crates --include="*.rs" | wc -l

# View latest progress
cat SESSION_FINAL_SUMMARY.md

# Check build status
cargo check --workspace 2>&1 | head -50
```

---

## 📖 **Essential Commands**

### **Documentation**
```bash
# Start here
cat START_HERE_UNIFICATION.md

# Quick overview
cat UNIFICATION_SUMMARY.md

# Latest progress
cat SESSION_FINAL_SUMMARY.md

# Detailed plan
cat UNIFICATION_NEXT_STEPS.md
```

### **Build & Test**
```bash
# Build workspace
cargo build --workspace

# Check for errors
cargo check --workspace

# Run tests
cargo test --workspace --lib

# Format code
cargo fmt --all

# Lint
cargo clippy --workspace -- -D warnings
```

---

## 🔧 **Unification Work Commands**

### **Find Remaining LegacyModuleError Files**
```bash
# List all files with LegacyModuleError
grep -rl "pub enum LegacyModuleError" code/crates/nestgate-core/src --include="*.rs"

# Count by directory
grep -r "pub enum LegacyModuleError" code/crates --include="*.rs" | cut -d: -f1 | xargs dirname | sort | uniq -c
```

### **File Size Compliance Check**
```bash
# Find files over 2000 lines
find code/crates -name "*.rs" -type f -exec wc -l {} \; | awk '$1 > 2000 {print $1, $2}' | sort -rn
```

### **Configuration Analysis**
```bash
# Count config structs
rg "^pub struct.*Config" code/crates --type rust | wc -l

# Find NetworkConfig usage
rg "NetworkConfig" code/crates/nestgate-core/src --type rust -l
```

### **Storage Trait Analysis**
```bash
# List storage traits
rg "^pub trait.*Storage" code/crates/nestgate-core/src/traits --type rust
```

---

## 🚀 **Common Tasks**

### **Clean Next LegacyModuleError File**

```bash
# 1. Pick a file
FILE="code/crates/nestgate-core/src/config/domains/FILENAME.rs"

# 2. Check the structure
grep -A 30 "pub enum LegacyModuleError" "$FILE"

# 3. Use search_replace tool to remove enum and From impl
# (See pattern in SESSION_FINAL_SUMMARY.md)
```

### **Test After Changes**
```bash
# Quick check
cargo check --package nestgate-core

# Full workspace check
cargo check --workspace

# Run tests
cargo test --package nestgate-core --lib
```

---

## 📊 **Progress Tracking**

### **Current Metrics** (as of Sep 30, 2025)
```
Overall Unification:        88%
LegacyModuleError Cleanup:  84% (32 remaining)
File Size Compliance:      100% 
Config Structs:         1,338 (target: <100)
Storage Traits:            31 (target: 2)
Build Status:          ✅ Stable
```

### **View Progress**
```bash
# Latest session summary
cat SESSION_FINAL_SUMMARY.md

# Progress tracker
cat PROGRESS_TRACKER.md

# View unification plan
cat UNIFICATION_NEXT_STEPS.md
```

---

## 🎯 **Next Actions**

### **Immediate (Next Session)**
```bash
# 1. Continue LegacyModuleError cleanup
#    Target: Clean 10-15 more files
#    Goal: Get below 20 instances

# 2. Files to clean next:
grep -rl "pub enum LegacyModuleError" code/crates/nestgate-core/src/config/domains --include="*.rs" | head -10
```

### **This Week**
- Complete LegacyModuleError cleanup (32 → 0)
- Establish canonical config exports
- Begin NetworkConfig consolidation

---

## 📁 **Key File Locations**

### **Canonical Implementations**
```bash
# Canonical config
code/crates/nestgate-core/src/config/canonical_master/mod.rs

# Canonical traits
code/crates/nestgate-core/src/traits/canonical_unified_traits.rs

# Unified error
code/crates/nestgate-core/src/error/mod.rs
```

### **Configuration Domains**
```bash
# Domain configs (where LegacyModuleError instances are)
code/crates/nestgate-core/src/config/domains/
```

---

## 🔍 **Search Patterns**

```bash
# Find deprecated items
rg "#\[deprecated" code/crates --type rust

# Find TODO comments
rg "TODO|FIXME" code/crates --type rust

# Find uses of old patterns
rg "LegacyModuleError" code/crates --type rust

# Find config structs
rg "pub struct.*Config" code/crates --type rust
```

---

## 🛠️ **Useful One-Liners**

```bash
# Count Rust files
find code/crates -name "*.rs" | wc -l

# Count lines of code
find code/crates -name "*.rs" -exec cat {} \; | wc -l

# Find largest files
find code/crates -name "*.rs" -exec wc -l {} \; | sort -rn | head -20

# Check for async_trait usage (should be rare/none)
rg "#\[async_trait\]" code/crates --type rust

# Find From implementations
rg "impl From<.*> for" code/crates --type rust
```

---

## 📚 **Documentation Quick Links**

| What | Where |
|------|-------|
| **Start Here** | [START_HERE_UNIFICATION.md](./START_HERE_UNIFICATION.md) |
| **Overview** | [README.md](./README.md) |
| **Latest Progress** | [SESSION_FINAL_SUMMARY.md](./SESSION_FINAL_SUMMARY.md) |
| **Action Guide** | [UNIFICATION_QUICK_ACTION_GUIDE.md](./UNIFICATION_QUICK_ACTION_GUIDE.md) |
| **Deep Dive** | [UNIFICATION_DEEP_ASSESSMENT_2025_09_30.md](./UNIFICATION_DEEP_ASSESSMENT_2025_09_30.md) |
| **Roadmap** | [UNIFICATION_NEXT_STEPS.md](./UNIFICATION_NEXT_STEPS.md) |
| **Architecture** | [ARCHITECTURE_OVERVIEW.md](./ARCHITECTURE_OVERVIEW.md) |
| **All Docs** | [DOCUMENTATION_INDEX.md](./DOCUMENTATION_INDEX.md) |

---

## 🎯 **Goals**

- **Immediate**: Complete LegacyModuleError cleanup (32 → 0)
- **Week 1-2**: Config consolidation (1,338 → <100)
- **Week 2-3**: Storage trait unification (31 → 2)
- **Week 3-4**: Final cleanup, 95% unification

---

**Need more detail?** See [DOCUMENTATION_INDEX.md](./DOCUMENTATION_INDEX.md) for complete documentation map.

---

**Current Status**: 88% Unified | 32 Remaining | 2.5 Hours to LegacyModuleError Completion  
**Last Updated**: September 30, 2025 