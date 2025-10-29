#!/bin/bash
# 🎯 NESTGATE UNIFICATION - PHASE 1: CONFIGURATION FOUNDATION
# Week 1: Establish THE Canonical Config System
#
# This script implements Week 1 of the unification roadmap:
# - Mark old config systems as deprecated
# - Update config/mod.rs exports
# - Find and catalog all config duplicates
# - Generate migration reports

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

TIMESTAMP=$(date +%Y%m%d_%H%M%S)
REPORT_DIR="docs/unification-reports"
mkdir -p "$REPORT_DIR"

echo "🎯 NESTGATE UNIFICATION - PHASE 1: CONFIGURATION FOUNDATION"
echo "============================================================"
echo "Date: $(date)"
echo "Project: $PROJECT_ROOT"
echo ""

# ============================================================
# STEP 1: ANALYZE CURRENT CONFIG FRAGMENTATION
# ============================================================

echo "📊 STEP 1: Analyzing current configuration fragmentation..."
echo ""

echo "Finding all Config struct definitions..."
CONFIG_FILES="$REPORT_DIR/config_structs_${TIMESTAMP}.txt"
find code/crates -name "*.rs" -type f -exec grep -l "pub struct.*Config" {} \; > "$CONFIG_FILES"
CONFIG_COUNT=$(wc -l < "$CONFIG_FILES")
echo "  ✅ Found $CONFIG_COUNT files with Config structs"

echo "Finding NetworkConfig variants..."
NETWORK_CONFIG_FILE="$REPORT_DIR/network_config_variants_${TIMESTAMP}.txt"
rg "pub struct.*NetworkConfig" --type rust code/crates/ > "$NETWORK_CONFIG_FILE" || true
NETWORK_COUNT=$(wc -l < "$NETWORK_CONFIG_FILE")
echo "  ✅ Found $NETWORK_COUNT NetworkConfig definitions"

echo "Finding StorageConfig variants..."
STORAGE_CONFIG_FILE="$REPORT_DIR/storage_config_variants_${TIMESTAMP}.txt"
rg "pub struct.*StorageConfig" --type rust code/crates/ > "$STORAGE_CONFIG_FILE" || true
STORAGE_COUNT=$(wc -l < "$STORAGE_CONFIG_FILE")
echo "  ✅ Found $STORAGE_COUNT StorageConfig definitions"

echo "Finding SecurityConfig variants..."
SECURITY_CONFIG_FILE="$REPORT_DIR/security_config_variants_${TIMESTAMP}.txt"
rg "pub struct.*SecurityConfig" --type rust code/crates/ > "$SECURITY_CONFIG_FILE" || true
SECURITY_COUNT=$(wc -l < "$SECURITY_CONFIG_FILE")
echo "  ✅ Found $SECURITY_COUNT SecurityConfig definitions"

echo ""
echo "Summary:"
echo "  - Total config files: $CONFIG_COUNT"
echo "  - NetworkConfig variants: $NETWORK_COUNT"
echo "  - StorageConfig variants: $STORAGE_COUNT"
echo "  - SecurityConfig variants: $SECURITY_COUNT"
echo ""

# ============================================================
# STEP 2: IDENTIFY CANONICAL VS DEPRECATED CONFIG SYSTEMS
# ============================================================

echo "🔍 STEP 2: Identifying canonical vs deprecated config systems..."
echo ""

CANONICAL_REPORT="$REPORT_DIR/config_system_classification_${TIMESTAMP}.txt"

cat > "$CANONICAL_REPORT" << 'EOF'
# CONFIGURATION SYSTEM CLASSIFICATION
# Generated: $(date)

## ✅ CANONICAL SYSTEM (THE ONE TO USE)
Location: code/crates/nestgate-core/src/config/canonical_master/
Main Type: NestGateCanonicalConfig
Status: ✅ CANONICAL - All new code should use this

## ❌ DEPRECATED SYSTEMS (TO BE REMOVED)

### 1. config/canonical/types.rs
Type: CanonicalConfig
Status: ❌ DEPRECATED - Use canonical_master::NestGateCanonicalConfig instead
Action: Add #[deprecated] marker

### 2. unified_config_consolidation.rs
Type: StandardDomainConfig<T>
Status: ❌ DEPRECATED - Use canonical_master::NestGateCanonicalConfig instead
Action: Add #[deprecated] marker

### 3. config/canonical_config/
Status: ❌ DEPRECATED - Old implementation
Action: Add #[deprecated] markers to mod.rs

### 4. config/canonical_unified/
Status: ❌ DEPRECATED - Superseded by canonical_master
Action: Add #[deprecated] markers to mod.rs

### 5. config/unified_types/
Status: ❌ DEPRECATED - Types moved to canonical_master
Action: Add #[deprecated] markers to mod.rs

## 🔄 MIGRATION HELPERS (TEMPORARY)
Location: code/crates/nestgate-core/src/config/migration_helpers/
Status: ⏳ TEMPORARY - Remove after migration complete
Action: Keep for now, remove in Week 4

## 📋 PER-CRATE CONFIGS (TO BE CONSOLIDATED)
Each of the 15 crates has local config definitions that should be removed
or converted to extensions of the canonical config.

Crates to update:
- nestgate-api
- nestgate-automation
- nestgate-bin
- nestgate-canonical
- nestgate-fsmonitor
- nestgate-installer
- nestgate-mcp
- nestgate-middleware
- nestgate-nas
- nestgate-network
- nestgate-performance
- nestgate-zfs

EOF

echo "  ✅ Created classification report: $CANONICAL_REPORT"
echo ""

# ============================================================
# STEP 3: GENERATE DEPRECATION MARKERS (DRY RUN)
# ============================================================

echo "🏷️  STEP 3: Generating deprecation markers (DRY RUN)..."
echo ""

DEPRECATION_SCRIPT="$REPORT_DIR/add_deprecation_markers_${TIMESTAMP}.sh"

cat > "$DEPRECATION_SCRIPT" << 'SCRIPTEOF'
#!/bin/bash
# Auto-generated script to add deprecation markers
# Review this script before running!

set -euo pipefail

PROJECT_ROOT="$1"
cd "$PROJECT_ROOT"

echo "Adding deprecation markers to old config systems..."

# Function to add deprecation marker to a file
add_deprecation() {
    local file="$1"
    local item_type="$2"
    local item_name="$3"
    local note="$4"
    
    if [ ! -f "$file" ]; then
        echo "  ⚠️  File not found: $file"
        return
    fi
    
    echo "  Processing: $file"
    
    # Check if already deprecated
    if grep -q "#\[deprecated" "$file"; then
        echo "    ℹ️  Already has deprecation markers"
        return
    fi
    
    # Add deprecation marker before the item
    # This is a simplified version - manual review recommended
    sed -i "/$item_type $item_name/i\\#[deprecated(since = \"0.7.0\", note = \"$note\")]" "$file"
    echo "    ✅ Added deprecation marker"
}

echo ""
echo "1. Deprecating config/canonical/types.rs"
add_deprecation \
    "code/crates/nestgate-core/src/config/canonical/types.rs" \
    "pub struct" \
    "CanonicalConfig" \
    "Use canonical_master::NestGateCanonicalConfig instead"

echo ""
echo "2. Deprecating unified_config_consolidation.rs"
add_deprecation \
    "code/crates/nestgate-core/src/config/unified_config_consolidation.rs" \
    "pub struct" \
    "StandardDomainConfig" \
    "Use canonical_master::NestGateCanonicalConfig instead"

echo ""
echo "3. Deprecating config/canonical_config/mod.rs"
if [ -f "code/crates/nestgate-core/src/config/canonical_config/mod.rs" ]; then
    # Add deprecation to the module itself
    sed -i '1i\#![deprecated(since = "0.7.0", note = "Use canonical_master instead")]' \
        "code/crates/nestgate-core/src/config/canonical_config/mod.rs"
    echo "  ✅ Deprecated canonical_config module"
fi

echo ""
echo "4. Deprecating config/canonical_unified/mod.rs"
if [ -f "code/crates/nestgate-core/src/config/canonical_unified/mod.rs" ]; then
    sed -i '1i\#![deprecated(since = "0.7.0", note = "Use canonical_master instead")]' \
        "code/crates/nestgate-core/src/config/canonical_unified/mod.rs"
    echo "  ✅ Deprecated canonical_unified module"
fi

echo ""
echo "5. Deprecating config/unified_types/mod.rs"
if [ -f "code/crates/nestgate-core/src/config/unified_types/mod.rs" ]; then
    sed -i '1i\#![deprecated(since = "0.7.0", note = "Use canonical_master instead")]' \
        "code/crates/nestgate-core/src/config/unified_types/mod.rs"
    echo "  ✅ Deprecated unified_types module"
fi

echo ""
echo "✅ Deprecation markers added!"
echo ""
echo "⚠️  IMPORTANT: Review the changes before committing:"
echo "  git diff code/crates/nestgate-core/src/config/"
echo ""
echo "Next steps:"
echo "  1. Review all deprecation markers"
echo "  2. Run: cargo check --workspace"
echo "  3. Fix any new warnings"
echo "  4. Commit changes"
SCRIPTEOF

chmod +x "$DEPRECATION_SCRIPT"
echo "  ✅ Created deprecation script: $DEPRECATION_SCRIPT"
echo "  ⚠️  Review this script before running!"
echo ""

# ============================================================
# STEP 4: UPDATE CONFIG MOD.RS EXPORTS (DRY RUN)
# ============================================================

echo "📦 STEP 4: Generating config/mod.rs update (DRY RUN)..."
echo ""

CONFIG_MOD_UPDATE="$REPORT_DIR/config_mod_update_${TIMESTAMP}.rs"

cat > "$CONFIG_MOD_UPDATE" << 'MODEOF'
//! Configuration system module
//! 
//! **PRIMARY SYSTEM**: Use `canonical_master::NestGateCanonicalConfig`
//! All other config systems are deprecated.

// ==================== THE CANONICAL CONFIG SYSTEM ====================

/// **THE** canonical configuration system - use this for all new code
pub mod canonical_master;

// Re-export the canonical config and domain configs
pub use canonical_master::{
    NestGateCanonicalConfig,
    ConsolidatedDomainConfigs,
    domains::*,
};

// ==================== DEPRECATED CONFIG SYSTEMS ====================

#[deprecated(note = "Use canonical_master instead")]
pub mod canonical;

#[deprecated(note = "Use canonical_master instead")]
pub mod canonical_config;

#[deprecated(note = "Use canonical_master instead")]
pub mod canonical_unified;

#[deprecated(note = "Use canonical_master instead")]
pub mod unified_types;

#[deprecated(note = "Use canonical_master instead")]
pub mod unified_config_consolidation;

// ==================== HELPER MODULES ====================

/// Domain-specific configuration types (to be consolidated)
pub mod domains;

/// Configuration validation utilities
pub mod validation;

/// Builder patterns for configuration
pub mod builders;

/// Configuration defaults
pub mod defaults;

/// Dynamic configuration updates
pub mod dynamic_config;

/// Migration helpers (temporary - remove after migration)
pub mod migration_helpers;

// ==================== CONVENIENCE RE-EXPORTS ====================

/// Type alias for the canonical config
pub type Config = NestGateCanonicalConfig;

/// Result type for configuration operations
pub type ConfigResult<T> = Result<T, crate::error::NestGateUnifiedError>;
MODEOF

echo "  ✅ Created updated config/mod.rs: $CONFIG_MOD_UPDATE"
echo ""
echo "To apply this update:"
echo "  1. Review: $CONFIG_MOD_UPDATE"
echo "  2. Backup: cp code/crates/nestgate-core/src/config/mod.rs code/crates/nestgate-core/src/config/mod.rs.backup"
echo "  3. Apply: cp $CONFIG_MOD_UPDATE code/crates/nestgate-core/src/config/mod.rs"
echo "  4. Test: cargo check --workspace"
echo ""

# ============================================================
# STEP 5: GENERATE MIGRATION PLAN FOR EACH CRATE
# ============================================================

echo "📋 STEP 5: Generating per-crate migration plans..."
echo ""

CRATE_MIGRATION_PLAN="$REPORT_DIR/crate_migration_plan_${TIMESTAMP}.md"

cat > "$CRATE_MIGRATION_PLAN" << 'PLANEOF'
# PER-CRATE MIGRATION PLAN

This document provides specific migration steps for each of the 15 crates.

## Migration Pattern

For each crate, follow this pattern:

### 1. Identify Local Config Structs
```bash
# Find config structs in the crate
rg "pub struct.*Config" code/crates/nestgate-{CRATE}/src/
```

### 2. Determine if Extension Needed
- If the config is truly crate-specific → Create extension
- If the config duplicates canonical → Remove entirely

### 3. Update Imports
```rust
// OLD
pub struct ApiConfig { ... }

// NEW
use nestgate_core::config::canonical_master::{
    NestGateCanonicalConfig,
    domains::ApiDomainConfig,
};

// Only if truly needed:
pub struct ApiExtensions {
    // Crate-specific fields only
}
```

### 4. Update Usage Sites
- Replace local config with canonical config
- Update method signatures
- Update initialization code

---

## Crate-Specific Plans

### nestgate-api

**Current Config Files**:
- `src/config.rs` - Defines ApiConfig
- `src/unified_api_config/` - Directory of config modules

**Analysis**:
- Most fields already exist in canonical ApiDomainConfig
- Some API-specific routing config may need extension

**Action**:
1. Remove `ApiConfig` struct
2. Import `ApiDomainConfig` from canonical_master
3. Create `ApiExtensions` for routing-specific config if needed
4. Update all usage sites

**Estimated Effort**: 2-3 hours

---

### nestgate-network

**Current Config Files**:
- `src/config.rs` - Defines NetworkConfig
- `src/types.rs` - Defines NetworkConfig again (duplicate!)
- `src/unified_network_config/` - Another network config

**Analysis**:
- THREE NetworkConfig definitions in one crate!
- Clear consolidation target
- Most fields exist in canonical NetworkServicesDomainConfig

**Action**:
1. Remove all three NetworkConfig definitions
2. Import NetworkServicesDomainConfig from canonical_master
3. Create NetworkExtensions ONLY if needed
4. Update all usage sites

**Estimated Effort**: 3-4 hours

---

### nestgate-zfs

**Current Config Files**:
- `src/config.rs` - Defines ZfsConfig
- Multiple config structs in various modules

**Analysis**:
- ZFS-specific config is appropriate
- Should extend canonical StorageDomainConfig

**Action**:
1. Import StorageDomainConfig from canonical_master
2. Keep ZfsExtensions for ZFS-specific features
3. Ensure no duplication with canonical storage config
4. Update all usage sites

**Estimated Effort**: 2-3 hours

---

### nestgate-mcp

**Current Config Files**:
- `src/config/` - Directory with MCP config modules

**Analysis**:
- MCP-specific config is appropriate
- Should use canonical McpDomainConfig

**Action**:
1. Import McpDomainConfig from canonical_master
2. Keep MCP-specific extensions
3. Remove any duplicated fields
4. Update all usage sites

**Estimated Effort**: 2 hours

---

### nestgate-automation

**Current Config Files**:
- `src/types/mod.rs` - Defines AutomationConfig

**Analysis**:
- Should use canonical AutomationDomainConfig

**Action**:
1. Remove AutomationConfig
2. Import AutomationDomainConfig from canonical_master
3. Create extensions only if needed
4. Update usage sites

**Estimated Effort**: 1-2 hours

---

### nestgate-installer

**Current Config Files**:
- `src/config.rs` - Defines InstallerConfig

**Analysis**:
- Installer-specific, but may duplicate system config

**Action**:
1. Review overlap with canonical InstallerDomainConfig
2. Remove duplicates
3. Keep installer-specific extensions
4. Update usage sites

**Estimated Effort**: 2 hours

---

### nestgate-fsmonitor

**Current Config Files**:
- `src/config.rs` - Defines FsMonitorConfig

**Analysis**:
- Should use canonical FsMonitorDomainConfig

**Action**:
1. Import FsMonitorDomainConfig from canonical_master
2. Create extensions if needed
3. Update usage sites

**Estimated Effort**: 1-2 hours

---

### nestgate-performance

**Current Config Files**:
- `src/config/` - Performance monitoring config

**Analysis**:
- Should use canonical PerformanceDomainConfig

**Action**:
1. Import PerformanceDomainConfig
2. Remove duplicates
3. Update usage sites

**Estimated Effort**: 1-2 hours

---

### Other Crates

**nestgate-bin**, **nestgate-canonical**, **nestgate-middleware**, **nestgate-nas**:

Similar pattern:
1. Identify local configs
2. Map to canonical equivalents
3. Create extensions if truly needed
4. Remove duplicates

**Total Estimated Effort**: 6-8 hours

---

## Total Migration Timeline

**Week 3 Schedule**:
- Day 1: nestgate-api, nestgate-network (largest configs)
- Day 2: nestgate-zfs, nestgate-mcp
- Day 3: nestgate-automation, nestgate-installer, nestgate-fsmonitor
- Day 4: nestgate-performance + 4 smaller crates
- Day 5: Testing, fixes, documentation

**Success Criteria**:
- All 15 crates import from canonical_master
- Zero local config struct duplicates
- All tests pass
- Documentation updated
PLANEOF

echo "  ✅ Created per-crate migration plan: $CRATE_MIGRATION_PLAN"
echo ""

# ============================================================
# STEP 6: GENERATE VALIDATION SCRIPTS
# ============================================================

echo "✅ STEP 6: Generating validation scripts..."
echo ""

VALIDATION_SCRIPT="$REPORT_DIR/validate_config_unification_${TIMESTAMP}.sh"

cat > "$VALIDATION_SCRIPT" << 'VALEOF'
#!/bin/bash
# Configuration Unification Validation Script

set -euo pipefail

PROJECT_ROOT="$1"
cd "$PROJECT_ROOT"

echo "🔍 CONFIGURATION UNIFICATION VALIDATION"
echo "========================================"
echo ""

PASSED=0
FAILED=0

# Test 1: Only canonical_master should have NetworkConfig
echo "Test 1: NetworkConfig consolidation..."
NON_CANONICAL=$(rg "pub struct.*NetworkConfig" --type rust code/crates/ | grep -v canonical_master | wc -l || true)
if [ "$NON_CANONICAL" -eq 0 ]; then
    echo "  ✅ PASS: NetworkConfig only in canonical_master"
    PASSED=$((PASSED + 1))
else
    echo "  ❌ FAIL: Found $NON_CANONICAL NetworkConfig definitions outside canonical_master"
    FAILED=$((FAILED + 1))
fi

# Test 2: Only canonical_master should have StorageConfig
echo "Test 2: StorageConfig consolidation..."
NON_CANONICAL=$(rg "pub struct.*StorageConfig" --type rust code/crates/ | grep -v canonical_master | wc -l || true)
if [ "$NON_CANONICAL" -eq 0 ]; then
    echo "  ✅ PASS: StorageConfig only in canonical_master"
    PASSED=$((PASSED + 1))
else
    echo "  ❌ FAIL: Found $NON_CANONICAL StorageConfig definitions outside canonical_master"
    FAILED=$((FAILED + 1))
fi

# Test 3: No usage of deprecated CanonicalConfig
echo "Test 3: No deprecated CanonicalConfig usage..."
DEPRECATED_USAGE=$(rg "use.*canonical::types::CanonicalConfig" --type rust code/crates/ | wc -l || true)
if [ "$DEPRECATED_USAGE" -eq 0 ]; then
    echo "  ✅ PASS: No usage of deprecated CanonicalConfig"
    PASSED=$((PASSED + 1))
else
    echo "  ❌ FAIL: Found $DEPRECATED_USAGE uses of deprecated CanonicalConfig"
    FAILED=$((FAILED + 1))
fi

# Test 4: No usage of deprecated StandardDomainConfig
echo "Test 4: No deprecated StandardDomainConfig usage..."
DEPRECATED_USAGE=$(rg "use.*StandardDomainConfig" --type rust code/crates/ | wc -l || true)
if [ "$DEPRECATED_USAGE" -eq 0 ]; then
    echo "  ✅ PASS: No usage of deprecated StandardDomainConfig"
    PASSED=$((PASSED + 1))
else
    echo "  ❌ FAIL: Found $DEPRECATED_USAGE uses of deprecated StandardDomainConfig"
    FAILED=$((FAILED + 1))
fi

# Test 5: Build should succeed
echo "Test 5: Clean build..."
if cargo check --workspace 2>&1 | grep -q "error"; then
    echo "  ❌ FAIL: Build has errors"
    FAILED=$((FAILED + 1))
else
    echo "  ✅ PASS: Build succeeds"
    PASSED=$((PASSED + 1))
fi

# Summary
echo ""
echo "VALIDATION SUMMARY"
echo "=================="
echo "  Passed: $PASSED"
echo "  Failed: $FAILED"
echo ""

if [ "$FAILED" -eq 0 ]; then
    echo "🎉 ALL TESTS PASSED! Configuration unification complete!"
    exit 0
else
    echo "❌ Some tests failed. Continue working on unification."
    exit 1
fi
VALEOF

chmod +x "$VALIDATION_SCRIPT"
echo "  ✅ Created validation script: $VALIDATION_SCRIPT"
echo ""

# ============================================================
# FINAL REPORT
# ============================================================

echo "📊 PHASE 1 ANALYSIS COMPLETE"
echo "============================"
echo ""
echo "Generated Reports:"
echo "  1. Config files list: $CONFIG_FILES"
echo "  2. NetworkConfig variants: $NETWORK_CONFIG_FILE"
echo "  3. StorageConfig variants: $STORAGE_CONFIG_FILE"
echo "  4. SecurityConfig variants: $SECURITY_CONFIG_FILE"
echo "  5. System classification: $CANONICAL_REPORT"
echo "  6. Deprecation script: $DEPRECATION_SCRIPT"
echo "  7. Updated mod.rs: $CONFIG_MOD_UPDATE"
echo "  8. Crate migration plan: $CRATE_MIGRATION_PLAN"
echo "  9. Validation script: $VALIDATION_SCRIPT"
echo ""
echo "Summary Statistics:"
echo "  - Total config files: $CONFIG_COUNT"
echo "  - NetworkConfig variants: $NETWORK_COUNT"
echo "  - StorageConfig variants: $STORAGE_COUNT"
echo "  - SecurityConfig variants: $SECURITY_COUNT"
echo ""
echo "Next Steps:"
echo "  1. Review all generated reports in: $REPORT_DIR"
echo "  2. Review and run: $DEPRECATION_SCRIPT \"$PROJECT_ROOT\""
echo "  3. Apply updated config/mod.rs"
echo "  4. Run validation: $VALIDATION_SCRIPT \"$PROJECT_ROOT\""
echo ""
echo "See UNIFICATION_ROADMAP_2025_Q4.md for complete 4-week plan"
echo ""
echo "✅ Phase 1 Analysis Complete! Ready to begin implementation." 