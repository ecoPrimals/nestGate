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
