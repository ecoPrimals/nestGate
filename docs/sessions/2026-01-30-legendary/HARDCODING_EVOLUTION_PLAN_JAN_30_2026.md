# 🔧 Hardcoding Evolution Plan - Phase 4

**Date**: January 30, 2026  
**Priority**: **HIGH** - Highest Impact (+4 bonus points)  
**Status**: Executing  
**Target**: 2,069 hardcoded values → Environment + Discovery

---

## 🎯 Goal

**Evolve from hardcoded values to:**
- ✅ Environment variables (configurable)
- ✅ Capability-based discovery (runtime)
- ✅ XDG standards (portable)
- ✅ Primal self-knowledge only (sovereignty)

**Impact**: +4 bonus points toward A+++ 110/100

---

## 📊 Hardcoding Inventory

### **Network Hardcoding** (1,899 instances, 255 files)

**Categories**:
```
Localhost variants: 127.0.0.1, localhost, 0.0.0.0, ::1
Ports: 8080, 8081, 8082, 3000, 5000, 6379, 5432
URLs: http://localhost:*
```

**High Priority Files**:
- `config/network_defaults.rs` (65 instances)
- `constants/network_hardcoded.rs` (24 instances)
- `constants/network_smart.rs` (33 instances)
- `constants/consolidated.rs` (30 instances)
- Test files (majority of instances)

### **Path Hardcoding** (170 instances, 49 files)

**Categories**:
```
System paths: /var/lib/, /var/log/, /tmp/, /etc/
User paths: /home/
Runtime: /run/user/{uid}/
```

**High Priority Files**:
- `rpc/unix_socket_server.rs` (8 instances - storage paths)
- `rpc/socket_config.rs` (18 instances - already has fallback!)
- `services/storage/config.rs` (5 instances)
- `utils/completely_safe_system.rs` (6 instances)

---

## 🚀 Evolution Strategy

### **Strategy 1: Test Hardcoding** (Keep ✅)

**Status**: Most instances are in tests - **INTENTIONAL**

**Rationale**:
- Tests need predictable values
- Test isolation requires fixed ports/paths
- No action needed for test hardcoding

**Examples**:
```rust
#[cfg(test)]
mod tests {
    const TEST_PORT: u16 = 8080; // ✅ OK in tests
    const TEST_HOST: &str = "127.0.0.1"; // ✅ OK in tests
}
```

---

### **Strategy 2: Configuration Defaults** (Evolve 🔄)

**Current**: Hardcoded defaults in constants
**Target**: Environment-aware defaults with fallback

**Pattern**:
```rust
// ❌ BEFORE:
const DEFAULT_PORT: u16 = 8080;

// ✅ AFTER:
pub fn get_default_port() -> u16 {
    std::env::var("NESTGATE_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080) // Fallback
}
```

**Files to Evolve**:
- `config/defaults.rs`
- `constants/ports.rs`
- `constants/network_defaults.rs`

---

### **Strategy 3: Storage Paths** (Evolve 🔄)

**Current**: Hardcoded `/var/lib/nestgate/`
**Target**: XDG-compliant with fallback

**Pattern**:
```rust
// ❌ BEFORE:
const STORAGE_PATH: &str = "/var/lib/nestgate/storage";

// ✅ AFTER:
pub fn get_storage_path() -> PathBuf {
    // 1. Environment variable (highest priority)
    if let Ok(path) = std::env::var("NESTGATE_STORAGE_PATH") {
        return PathBuf::from(path);
    }
    
    // 2. XDG data directory (standard)
    if let Ok(xdg_data) = std::env::var("XDG_DATA_HOME") {
        return PathBuf::from(xdg_data).join("nestgate");
    }
    
    // 3. Home directory fallback
    if let Ok(home) = std::env::var("HOME") {
        return PathBuf::from(home).join(".local/share/nestgate");
    }
    
    // 4. System fallback (requires permissions)
    PathBuf::from("/var/lib/nestgate")
}
```

**Files to Evolve**:
- `rpc/unix_socket_server.rs` (already has good patterns!)
- `services/storage/config.rs`

---

### **Strategy 4: Discovery Endpoints** (Evolve 🔄)

**Current**: Some hardcoded primal endpoints
**Target**: Runtime capability-based discovery

**Pattern**:
```rust
// ❌ BEFORE:
const BEARDOG_URL: &str = "http://localhost:9000";

// ✅ AFTER:
pub async fn discover_beardog() -> Result<Endpoint> {
    // Use Universal Primal Discovery
    let discovery = UniversalPrimalDiscovery::new().await?;
    
    discovery
        .find_by_capability(Capability::Security)
        .await?
        .first()
        .ok_or_else(|| Error::PrimalNotFound("beardog"))
}
```

**Pattern Already Exists!**:
- `universal_primal_discovery/` module ✅
- Capability-based discovery ✅
- Just need to USE it instead of hardcoding

---

## 📋 Execution Plan

### **Phase 4.1: Analyze & Categorize** (1 day) ✅ COMPLETE

**Results**:
- 1,899 network hardcodes (255 files)
- 170 path hardcodes (49 files)
- Most in tests (intentional) ✅
- ~200 production hardcodes to evolve

---

### **Phase 4.2: Test Hardcoding Review** (0.5 days)

**Goal**: Verify test hardcoding is intentional

**Actions**:
1. Review test files with hardcoding
2. Confirm isolation and predictability needs
3. Document rationale
4. Mark as "intentional - keep"

**Expected Outcome**: 
- 80%+ of hardcoding is in tests (keep)
- ~200 production instances to evolve

---

### **Phase 4.3: Configuration Defaults Evolution** (2 days)

**Goal**: Environment-aware defaults

**Files to Update**:
```
High Priority:
- config/defaults.rs
- config/network_defaults.rs
- constants/ports.rs
- constants/network_defaults.rs

Medium Priority:
- config/runtime/network.rs
- config/runtime/services.rs
- constants/consolidated.rs
```

**Pattern**: Add `get_*()` functions with env fallback

**Testing**: Verify with environment variables set/unset

---

### **Phase 4.4: Storage Path Evolution** (1 day)

**Goal**: XDG-compliant storage paths

**Files to Update**:
```
- services/storage/config.rs
- rpc/unix_socket_server.rs (storage paths only)
- utils/fs.rs
```

**Pattern**: XDG > HOME > /var/lib fallback

**Testing**: Verify across environments

---

### **Phase 4.5: Discovery Evolution** (2 days)

**Goal**: Eliminate hardcoded primal endpoints

**Strategy**:
1. Find hardcoded primal endpoints
2. Replace with discovery calls
3. Use existing `universal_primal_discovery/`

**Pattern**:
```rust
// Replace:
let beardog_url = "http://localhost:9000";

// With:
let beardog = discover_security_provider().await?;
```

**Files**: Search for primal endpoint patterns

---

### **Phase 4.6: Documentation** (0.5 days)

**Goal**: Document environment variables

**Create**:
- `ENVIRONMENT_VARIABLES.md` - Complete reference
- Update README.md with env vars
- Update config examples

**Content**:
- All NESTGATE_* variables
- XDG variables used
- Fallback behavior
- Examples

---

### **Phase 4.7: Testing** (1 day)

**Goal**: Verify evolution works

**Tests**:
1. Run with default environment
2. Run with custom environment variables
3. Run with XDG variables
4. Run in restricted environment (no /var/lib)
5. Verify discovery works

**Success Criteria**:
- All tests pass ✅
- Works with env vars ✅
- Works without env vars (fallback) ✅
- No hardcoded endpoints remain ✅

---

## 📈 Success Metrics

### **Quantitative**:
- ✅ 0 production hardcoded IPs/localhost
- ✅ 0 production hardcoded ports (use env vars)
- ✅ 0 production hardcoded paths (use XDG)
- ✅ 0 hardcoded primal endpoints (use discovery)
- ✅ ~200 production instances evolved
- ✅ ~1,700 test instances kept (intentional)

### **Qualitative**:
- ✅ Environment-configurable
- ✅ XDG-compliant
- ✅ Works in containers
- ✅ Works without permissions
- ✅ Capability-based discovery
- ✅ Primal self-knowledge only

---

## 🎯 Grade Impact

**Current**: A++ 100/100  
**This Phase**: +4 points  
**After Phase 4**: A++ 104/100

**Remaining to A+++ 110/100**: +6 points
- Phase 3 (Smart refactoring): +2
- Phase 6 (Tech debt): +2  
- Documentation: +2

---

## ⏱️ Timeline

**Total**: 8 days (1.5 weeks)

```
Day 1: ✅ Analysis & Categorization (COMPLETE)
Day 2: Test hardcoding review (0.5d) + Start config evolution (0.5d)
Day 3-4: Configuration defaults evolution
Day 5: Storage path evolution
Day 6-7: Discovery evolution
Day 8: Documentation + Testing
```

**Start**: January 30, 2026  
**Target Completion**: February 7, 2026

---

## 🚀 Next Actions

**Today** (January 30):
1. ✅ Create this plan (DONE)
2. 🔄 Start Phase 4.2: Test hardcoding review
3. 🔄 Begin Phase 4.3: Config defaults evolution

**This Week**:
- Complete Phase 4.2 (test review)
- Complete Phase 4.3 (config evolution)
- Start Phase 4.4 (storage paths)

**Next Week**:
- Complete Phase 4.4-4.7
- Full testing
- Documentation

---

## 📚 Principles Reinforced

✅ **Primal Self-Knowledge**: Know yourself, discover others  
✅ **Sovereignty**: No hardcoded infrastructure  
✅ **Agnostic**: Works anywhere (dev, staging, production)  
✅ **Buildable**: XDG-compliant, portable  
✅ **Capability-Based**: Runtime discovery, not hardcoding  
✅ **Deep Solutions**: Not quick fixes, proper architecture  

---

**Status**: Plan created ✅  
**Next**: Execute Phase 4.2 (Test review)  
**Impact**: +4 points toward A+++ 110/100  
**Timeline**: 8 days to completion

🦀 **Hardcoding Evolution · Capability-Based · Primal Self-Knowledge** 🦀
