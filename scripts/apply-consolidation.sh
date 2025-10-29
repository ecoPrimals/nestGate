#!/bin/bash
# 🔄 **NESTGATE CONSOLIDATION APPLICATION SCRIPT**
# Apply consolidated systems across all crates systematically

set -euo pipefail

echo "🚀 **NESTGATE CONSOLIDATION APPLICATION**"
echo "=========================================="

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

# Function to show progress
show_progress() {
    echo "📊 Checking consolidation progress..."
    LEGACY_CONFIG_COUNT=$(find code/crates -name "*.rs" -exec grep -l "UnifiedCanonicalConfig" {} \; | wc -l || echo "0")
    LEGACY_ERROR_COUNT=$(find code/crates -name "*.rs" -exec grep -l "pub enum.*Error" {} \; | wc -l || echo "0")
    ASYNC_TRAIT_COUNT=$(find code/crates -name "*.rs" -exec grep -l "#\[async_trait\]" {} \; | wc -l || echo "0")
    echo "   Legacy config usage: $LEGACY_CONFIG_COUNT files"
    echo "   Legacy error enums: $LEGACY_ERROR_COUNT files"
    echo "   Remaining async_trait: $ASYNC_TRAIT_COUNT files"
}

echo "🔍 **PHASE 1: CONSOLIDATION STATUS CHECK**"
echo "-------------------------------------------"
show_progress

echo ""
echo "🔧 **PHASE 2: APPLY CONFIGURATION CONSOLIDATION**"
echo "--------------------------------------------------"

# Update imports to use consolidated configuration
echo "Updating configuration imports..."

# Find and update configuration imports
find code/crates -name "*.rs" -type f -exec sed -i 's/use nestgate_core::config::UnifiedCanonicalConfig/use nestgate_core::config::Config/g' {} \;
find code/crates -name "*.rs" -type f -exec sed -i 's/UnifiedCanonicalConfig::/Config::/g' {} \;
find code/crates -name "*.rs" -type f -exec sed -i 's/UnifiedCanonicalConfig</Config</g' {} \;

echo "✅ Configuration imports updated"

echo ""
echo "🛡️ **PHASE 3: APPLY ERROR CONSOLIDATION**"
echo "------------------------------------------"

# Update error imports
echo "Updating error system imports..."

# Update error imports across crates
find code/crates -name "*.rs" -type f -exec sed -i 's/use.*::error::{[^}]*}/use nestgate_core::error::{NestGateError, Result}/g' {} \;

# Update Result type usage
find code/crates -name "*.rs" -type f -exec sed -i 's/pub type.*Result<T>/pub type Result<T> = nestgate_core::error::Result<T>/g' {} \;

echo "✅ Error system imports updated"

echo ""
echo "🚀 **PHASE 4: REMOVE ASYNC_TRAIT WHERE POSSIBLE**"
echo "--------------------------------------------------"

# Check which crates still actually use async_trait
echo "Checking async_trait usage..."

# Create a temporary file to store crates that still need async_trait
ASYNC_TRAIT_NEEDED=$(mktemp)

for crate_dir in code/crates/*/; do
    crate_name=$(basename "$crate_dir")
    
    # Check if crate still has actual async_trait usage (not just comments)
    if find "$crate_dir" -name "*.rs" -exec grep -l "#\[async_trait\]" {} \; | head -1 | grep -q .; then
        echo "   $crate_name: Still needs async_trait"
        echo "$crate_name" >> "$ASYNC_TRAIT_NEEDED"
    else
        echo "   $crate_name: Can remove async_trait dependency"
        
        # Remove async_trait from Cargo.toml if it exists and is not needed
        if [ -f "$crate_dir/Cargo.toml" ] && grep -q "async-trait" "$crate_dir/Cargo.toml"; then
            echo "     Removing async_trait dependency from $crate_name"
            sed -i '/^async-trait = /d' "$crate_dir/Cargo.toml"
        fi
    fi
done

echo ""
echo "🧹 **PHASE 5: REMOVE DEPRECATED MODULES**"
echo "------------------------------------------"

echo "Marking deprecated modules..."

# Add deprecation warnings to legacy modules
find code/crates -path "*/config/unified_canonical_config.rs" -exec sed -i '1i//! **DEPRECATED**: Use nestgate_core::config::ConsolidatedCanonicalConfig instead' {} \;
find code/crates -path "*/config/canonical_config.rs" -exec sed -i '1i//! **DEPRECATED**: Use nestgate_core::config::ConsolidatedCanonicalConfig instead' {} \;

# Mark legacy error modules as deprecated
find code/crates -name "error.rs" -not -path "*/nestgate-core/*" -exec sed -i '1i//! **DEPRECATED**: Use nestgate_core::error::{NestGateError, Result} instead' {} \;

echo "✅ Deprecated modules marked"

echo ""
echo "🔍 **PHASE 6: VALIDATION**"
echo "--------------------------"

echo "Running basic validation..."

# Check if the project compiles with basic check
if cargo check --workspace --quiet 2>/dev/null; then
    echo "✅ Basic compilation check passed"
else
    echo "⚠️  Compilation issues detected - manual review needed"
    echo "   Run 'cargo check --workspace' for details"
fi

# Show final progress
echo ""
echo "📊 **CONSOLIDATION RESULTS**"
echo "-----------------------------"
show_progress

# Count consolidated usage
CONSOLIDATED_CONFIG_COUNT=$(find code/crates -name "*.rs" -exec grep -l "nestgate_core::config::Config" {} \; | wc -l || echo "0")
CONSOLIDATED_ERROR_COUNT=$(find code/crates -name "*.rs" -exec grep -l "nestgate_core::error::NestGateError" {} \; | wc -l || echo "0")

echo "   Consolidated config usage: $CONSOLIDATED_CONFIG_COUNT files"
echo "   Consolidated error usage: $CONSOLIDATED_ERROR_COUNT files"

echo ""
echo "🎯 **NEXT STEPS**"
echo "-----------------"
echo "1. Review compilation output: cargo check --workspace"
echo "2. Run tests: cargo test --workspace"
echo "3. Update remaining async_trait usage in:"
while IFS= read -r crate_name; do
    echo "   - $crate_name"
done < "$ASYNC_TRAIT_NEEDED"

echo "4. Remove deprecated modules once migration is complete"
echo "5. Update documentation to reflect consolidated systems"

# Cleanup
rm -f "$ASYNC_TRAIT_NEEDED"

echo ""
echo "🎉 **CONSOLIDATION APPLICATION COMPLETE**"
echo "========================================="
echo "The consolidated systems have been applied across the codebase."
echo "Review the output above and address any compilation issues."

exit 0 