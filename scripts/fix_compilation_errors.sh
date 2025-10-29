#!/bin/bash
# 🔧 COMPILATION ERROR FIXING SCRIPT
# Systematically fixes compilation errors from modernization

set -euo pipefail

echo "🔧 COMPILATION ERROR FIXING - SYSTEMATIC CLEANUP"
echo "================================================"

# Create error fixing log
ERROR_FIX_LOG="compilation-fixes-$(date +%Y%m%d-%H%M%S).log"
echo "📋 Creating error fix log: $ERROR_FIX_LOG"

# 1. FIX STORAGE ERROR REFERENCES
echo "🔄 Phase 1: Fixing storage error references..."

# Replace deprecated storage() calls with storage_error()
find code/ -name "*.rs" -exec sed -i 's/NestGateError::storage(/NestGateError::storage_error(/g' {} \;
echo "✅ Fixed storage() → storage_error() calls" | tee -a "$ERROR_FIX_LOG"

# 2. FIX ERROR STRUCT FIELD REFERENCES
echo "🔄 Phase 2: Fixing error struct field references..."

# Fix Configuration error usage
find code/ -name "*.rs" -exec sed -i '
/NestGateError::Configuration {/,/}/ {
    s/NestGateError::Configuration {/NestGateError::configuration(/
    /field:/d
    /message:/d
    /current_value:/d
    /expected:/d
    /user_error:/d
    s/}.into()/)/
    s/}/)/
}
' {} \;

# Fix Validation error usage
find code/ -name "*.rs" -exec sed -i '
/NestGateError::Validation {/,/}/ {
    s/NestGateError::Validation {/NestGateError::validation(/
    /field:/d
    /message:/d
    /expected:/d
    /actual:/d
    /context:/d
    s/}.into()/)/
    s/}/)/
}
' {} \;

# Fix Internal error usage
find code/ -name "*.rs" -exec sed -i '
/NestGateError::Internal {/,/}/ {
    s/NestGateError::Internal {/NestGateError::internal_error(/
    /message:/d
    /component:/d
    /location:/d
    /context:/d
    /is_bug:/d
    s/}.into()/)/
    s/}/)/
}
' {} \;

echo "✅ Fixed error struct field references" | tee -a "$ERROR_FIX_LOG"

# 3. FIX STRING PATTERN ISSUES
echo "🔄 Phase 3: Fixing string pattern issues..."

# Fix string contains patterns
find code/ -name "*.rs" -exec sed -i 's/\.contains("container_runtime"\.to_string())/\.contains("container_runtime")/g' {} \;
find code/ -name "*.rs" -exec sed -i 's/\.contains("service_discovery"\.to_string())/\.contains("service_discovery")/g' {} \;
find code/ -name "*.rs" -exec sed -i 's/\.contains("key_value_store"\.to_string())/\.contains("key_value_store")/g' {} \;

echo "✅ Fixed string pattern issues" | tee -a "$ERROR_FIX_LOG"

# 4. FIX FORMAT STRING RETURN TYPES
echo "🔄 Phase 4: Fixing format string return types..."

# Fix format! macro return type issues
find code/ -name "*.rs" -exec sed -i 's/format!("Invalid command argument detected: {}", "placeholder")/format!("Invalid command argument detected: {}", "placeholder").as_str()/g' {} \;
find code/ -name "*.rs" -exec sed -i 's/format!("Unsafe ZFS command: {}", "placeholder")/format!("Unsafe ZFS command: {}", "placeholder").as_str()/g' {} \;

echo "✅ Fixed format string return types" | tee -a "$ERROR_FIX_LOG"

# 5. FIX UNUSED VARIABLE WARNINGS
echo "🔄 Phase 5: Fixing unused variable warnings..."

# Add underscores to unused variables
find code/ -name "*.rs" -exec sed -i '
s/orchestration_endpoint:/_orchestration_endpoint:/g
s/compute_endpoint:/_compute_endpoint:/g
s/endpoint: &str/_endpoint: &str/g
s/snapshot_name: &str/_snapshot_name: &str/g
s/for (key, value)/for (key, _value)/g
' {} \;

# Remove unnecessary mut keywords
find code/ -name "*.rs" -exec sed -i 's/let mut config = zfs_default();/let config = zfs_default();/g' {} \;
find code/ -name "*.rs" -exec sed -i 's/let mut config = ZfsConfig::default();/let config = ZfsConfig::default();/g' {} \;

echo "✅ Fixed unused variable warnings" | tee -a "$ERROR_FIX_LOG"

# 6. CREATE ERROR HANDLING MODERNIZATION
echo "🔄 Phase 6: Creating error handling modernization..."

cat > code/crates/nestgate-core/src/error/modernized_error_helpers.rs << 'EOF'
//! 🔧 MODERNIZED ERROR HANDLING HELPERS
//! 
//! Provides simplified error creation for common patterns

use crate::error::NestGateUnifiedError;

/// Create a storage error with modern pattern
pub fn storage_error(message: impl Into<String>) -> NestGateUnifiedError {
    NestGateUnifiedError::storage_error(message.into())
}

/// Create a configuration error with modern pattern  
pub fn configuration_error(message: impl Into<String>) -> NestGateUnifiedError {
    NestGateUnifiedError::configuration(message.into())
}

/// Create a validation error with modern pattern
pub fn validation_error(message: impl Into<String>) -> NestGateUnifiedError {
    NestGateUnifiedError::validation(message.into())
}

/// Create an internal error with modern pattern
pub fn internal_error(message: impl Into<String>, component: &str) -> NestGateUnifiedError {
    NestGateUnifiedError::internal_error(&message.into(), component)
}
EOF

echo "✅ Created modernized error helpers" | tee -a "$ERROR_FIX_LOG"

# 7. UPDATE ERROR MODULE EXPORTS
echo "🔄 Phase 7: Updating error module exports..."

# Add the new error helpers to the error module
if ! grep -q "modernized_error_helpers" code/crates/nestgate-core/src/error/mod.rs; then
    echo "pub mod modernized_error_helpers;" >> code/crates/nestgate-core/src/error/mod.rs
    echo "pub use modernized_error_helpers::*;" >> code/crates/nestgate-core/src/error/mod.rs
fi

echo "✅ Updated error module exports" | tee -a "$ERROR_FIX_LOG"

# 8. COMPILATION TEST
echo "🔄 Phase 8: Testing compilation fixes..."

if cargo check --workspace --quiet 2>/dev/null; then
    echo "✅ Compilation successful after fixes!" | tee -a "$ERROR_FIX_LOG"
    COMPILATION_STATUS="SUCCESS"
else
    echo "⚠️ Some compilation issues remain - logging details" | tee -a "$ERROR_FIX_LOG"
    cargo check --workspace 2>&1 | head -50 >> "$ERROR_FIX_LOG"
    COMPILATION_STATUS="PARTIAL"
fi

# 9. VERIFICATION AND REPORTING
echo "🔄 Phase 9: Verification and reporting..."

# Count remaining errors
ERROR_COUNT=$(cargo check --workspace 2>&1 | grep -c "error\[" || echo "0")
WARNING_COUNT=$(cargo check --workspace 2>&1 | grep -c "warning:" || echo "0")

echo "📊 COMPILATION FIX RESULTS:" | tee -a "$ERROR_FIX_LOG"
echo "  - Compilation status: $COMPILATION_STATUS" | tee -a "$ERROR_FIX_LOG"
echo "  - Remaining errors: $ERROR_COUNT" | tee -a "$ERROR_FIX_LOG"
echo "  - Remaining warnings: $WARNING_COUNT" | tee -a "$ERROR_FIX_LOG"
echo "  - Fix log: $ERROR_FIX_LOG" | tee -a "$ERROR_FIX_LOG"

echo ""
echo "🔧 COMPILATION ERROR FIXING COMPLETE"
echo "===================================="
echo "✅ Storage error calls → Modernized to storage_error()"
echo "✅ Error struct fields → Converted to function calls"
echo "✅ String patterns → Fixed contains() calls"
echo "✅ Format strings → Fixed return type issues"
echo "✅ Unused variables → Added underscores and removed mut"
echo "✅ Error helpers → Created modernized helper functions"
echo ""
echo "🎯 Status: $COMPILATION_STATUS"
echo "📊 Errors: $ERROR_COUNT | Warnings: $WARNING_COUNT"
echo "📋 Detailed log: $ERROR_FIX_LOG" 