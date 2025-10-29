#!/bin/bash
# 🔧 **FINAL IMPORT FIXES**
# Fix all remaining import and type issues

set -euo pipefail

echo "🔧 **FINAL IMPORT FIXES**"
echo "========================"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

echo "1. Adding NestGateUnifiedError imports where missing..."

# Add imports to files that use NestGateUnifiedError but don't import it
files_needing_import=(
    "code/crates/nestgate-core/src/traits/universal.rs"
    "code/crates/nestgate-core/src/canonical_types/storage.rs"
    "code/crates/nestgate-core/src/utils.rs"
    "code/crates/nestgate-core/src/network/mod.rs"
    "code/crates/nestgate-core/src/scheduling/mod.rs"
    "code/crates/nestgate-core/src/services/storage/types.rs"
    "code/crates/nestgate-core/src/services/native_async/traits.rs"
    "code/crates/nestgate-core/src/services/native_async/development.rs"
    "code/crates/nestgate-core/src/services/native_async/production.rs"
    "code/crates/nestgate-core/src/storage/traits.rs"
)

for file in "${files_needing_import[@]}"; do
    if [[ -f "$file" ]]; then
        # Check if import already exists
        if ! grep -q "use.*NestGateUnifiedError" "$file"; then
            # Add import after existing use statements or at the top
            if grep -q "^use " "$file"; then
                sed -i '/^use /a use crate::error::NestGateUnifiedError;' "$file"
            else
                sed -i '1i use crate::error::NestGateUnifiedError;' "$file"
            fi
        fi
    fi
done

echo "2. Fixing type parameter issues in unified_result_system.rs..."

# Fix the unused type parameter issues
sed -i 's/pub type OptionalResult<T, NestGateUnifiedError>/pub type OptionalResult<T>/g' code/crates/nestgate-core/src/error/unified_result_system.rs
sed -i 's/pub type CollectionResult<T, NestGateUnifiedError>/pub type CollectionResult<T>/g' code/crates/nestgate-core/src/error/unified_result_system.rs
sed -i 's/pub type OptionResult<T, NestGateUnifiedError>/pub type OptionResult<T>/g' code/crates/nestgate-core/src/error/unified_result_system.rs

echo "3. Fixing self-referencing constants..."

# Fix remaining self-referencing constants
sed -i 's/crate::constants::system::DEFAULT_BUFFER_SIZE/65536/g' code/crates/nestgate-core/src/constants/system.rs
sed -i 's/crate::constants::network::DEFAULT_TIMEOUT_MS/30000/g' code/crates/nestgate-core/src/constants/system.rs

echo "4. Fixing async function issues..."

# Remove await calls from non-async functions that were converted incorrectly
find code/crates -name "*.rs" -type f -exec sed -i '/^[[:space:]]*fn /,/^[[:space:]]*}/ { s/\.await//g }' {} \;

echo "5. Adding missing imports to all trait files..."

# Ensure all trait files have the necessary imports
find code/crates/nestgate-core/src/traits -name "*.rs" -type f | while read -r file; do
    if grep -q "NestGateUnifiedError" "$file" && ! grep -q "use.*NestGateUnifiedError" "$file"; then
        sed -i '1i use crate::error::NestGateUnifiedError;' "$file"
    fi
done

echo "6. Fixing remaining circular references..."

# Fix any remaining circular references in constants
find code/crates/nestgate-core/src/constants -name "*.rs" -type f -exec sed -i 's/crate::constants::[^:]*::\([A-Z_]*\)/\1_VALUE/g' {} \;

echo "✅ Applied final import fixes"

echo ""
echo "📊 Checking compilation status..."
ERROR_COUNT=$(cargo check --workspace --message-format short 2>&1 | grep -c "error:" || echo "0")
WARNING_COUNT=$(cargo check --workspace --message-format short 2>&1 | grep -c "warning:" || echo "0")
echo "   Current errors: $ERROR_COUNT, warnings: $WARNING_COUNT"

if [ "$ERROR_COUNT" -eq "0" ]; then
    echo "🎉 **COMPILATION SUCCESSFUL!**"
    echo "🏆 **UNIFICATION & MODERNIZATION COMPLETE!**"
else
    echo "⚠️ Still $ERROR_COUNT errors remaining"
    echo "📋 Most common error types:"
    cargo check --workspace 2>&1 | grep "error\[" | cut -d':' -f1 | sort | uniq -c | sort -nr | head -5
fi

echo ""
echo "✅ **FINAL IMPORT FIXES COMPLETE**" 