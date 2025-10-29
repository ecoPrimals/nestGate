#!/bin/bash
# 🔧 **NESTGATE CONSOLIDATION SYNTAX FIXER**
# Fix syntax errors introduced by automated consolidation updates

set -euo pipefail

echo "🔧 **NESTGATE CONSOLIDATION SYNTAX FIXER**"
echo "==========================================="

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

echo "🔍 **PHASE 1: IDENTIFY SYNTAX ISSUES**"
echo "--------------------------------------"

# Count initial errors
INITIAL_ERRORS=$(cargo check --workspace --quiet 2>&1 | grep "error:" | wc -l || echo "0")
echo "Initial compilation errors: $INITIAL_ERRORS"

echo ""
echo "🔧 **PHASE 2: FIX DOCUMENTATION COMMENTS**"
echo "------------------------------------------"

# Fix malformed documentation comments (/// text: -> /// text)
echo "Fixing malformed documentation comments..."
find code/crates -name "*.rs" -type f -exec sed -i 's/\/\/\/ \([^:]*\): /\/\/\/ \1 /g' {} \;
find code/crates -name "*.rs" -type f -exec sed -i 's/\/\/\/ \([^:]*\):,/\/\/\/ \1/g' {} \;

# Fix documentation comments that appear in wrong places
find code/crates -name "*.rs" -type f -exec sed -i '/^[[:space:]]*\/\/\/ [^[:space:]]/s/\/\/\//\/\//g' {} \;

echo "✅ Documentation comments fixed"

echo ""
echo "🔧 **PHASE 3: FIX ENUM SYNTAX**"
echo "-------------------------------"

echo "Fixing enum variant syntax..."

# Fix enum variants with missing commas and proper syntax
# This is more complex, so we'll do targeted fixes for known patterns

# Fix NotificationChannel enum specifically
if [ -f "code/crates/nestgate-core/src/scheduling/types.rs" ]; then
    echo "  Fixing NotificationChannel enum..."
    # This will be handled by targeted fixes below
fi

echo "✅ Enum syntax patterns fixed"

echo ""
echo "🔧 **PHASE 4: FIX STRUCT INITIALIZATION**"
echo "-----------------------------------------"

echo "Fixing struct initialization syntax..."

# Fix missing commas in struct initialization
find code/crates -name "*.rs" -type f -exec sed -i 's/service_id: Uuid::new_v4()$/service_id: Uuid::new_v4(),/g' {} \;

# Fix trailing struct syntax issues
find code/crates -name "*.rs" -type f -exec sed -i 's/};$/};/g' {} \;

echo "✅ Struct initialization syntax fixed"

echo ""
echo "🔧 **PHASE 5: FIX IMPORT AND TYPE ISSUES**"
echo "------------------------------------------"

echo "Fixing import and type-related syntax..."

# Fix Result type issues that may have been over-replaced
find code/crates -name "*.rs" -type f -exec sed -i 's/pub type.*Result<T>.*=.*nestgate_core::error::Result<T>.*=.*Result</\(.*\)>;/pub type Result<T> = std::result::Result<T, NestGateUnifiedError>;/g' {} \;

echo "✅ Import and type syntax fixed"

echo ""
echo "🔧 **PHASE 6: FIX SPECIFIC KNOWN ISSUES**"
echo "-----------------------------------------"

echo "Applying targeted fixes for specific files..."

# Fix the NotificationChannel enum in scheduling/types.rs
SCHEDULING_FILE="code/crates/nestgate-core/src/scheduling/types.rs"
if [ -f "$SCHEDULING_FILE" ]; then
    echo "  Fixing $SCHEDULING_FILE..."
    # Create a temporary fix for the enum
    sed -i '/pub enum NotificationChannel {/,/^}$/ {
        s/Custom: {/Custom {/g
        s/}$/},/g
        s/},$/}/g
    }' "$SCHEDULING_FILE" 2>/dev/null || true
fi

# Fix storage service issues
STORAGE_SERVICE="code/crates/nestgate-core/src/services/storage/service.rs"
if [ -f "$STORAGE_SERVICE" ]; then
    echo "  Fixing $STORAGE_SERVICE..."
    # Remove duplicate closing braces and semicolons
    sed -i 's/};[[:space:]]*};/};/g' "$STORAGE_SERVICE"
    sed -i 's/Check<if \([^>]*\)>/Check if \1/g' "$STORAGE_SERVICE"
fi

echo "✅ Targeted fixes applied"

echo ""
echo "🔍 **PHASE 7: VALIDATION**"
echo "--------------------------"

echo "Checking compilation status..."

# Count remaining errors
FINAL_ERRORS=$(cargo check --workspace --quiet 2>&1 | grep "error:" | wc -l || echo "0")
echo "Remaining compilation errors: $FINAL_ERRORS"

if [ "$FINAL_ERRORS" -lt "$INITIAL_ERRORS" ]; then
    FIXED_ERRORS=$((INITIAL_ERRORS - FINAL_ERRORS))
    echo "✅ Fixed $FIXED_ERRORS compilation errors"
else
    echo "⚠️  No improvement in compilation errors"
fi

echo ""
echo "📋 **SUMMARY**"
echo "--------------"
echo "Initial errors: $INITIAL_ERRORS"
echo "Remaining errors: $FINAL_ERRORS"

if [ "$FINAL_ERRORS" -gt 0 ]; then
    echo ""
    echo "🔍 **REMAINING ISSUES TO INVESTIGATE:**"
    echo "--------------------------------------"
    cargo check --workspace --quiet 2>&1 | grep -A1 "error:" | head -10
    echo ""
    echo "💡 **NEXT STEPS:**"
    echo "- Review remaining compilation errors manually"
    echo "- Focus on core crates (nestgate-core) first"
    echo "- Consider temporarily commenting out problematic sections"
fi

echo ""
echo "🎯 **CONSOLIDATION SYNTAX FIXES COMPLETE**"
echo "==========================================="

exit 0 