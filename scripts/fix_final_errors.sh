#!/bin/bash
# 🔧 **FINAL ERROR FIXES**
# Fix the last 10 compilation errors

set -euo pipefail

echo "🔧 **FIXING FINAL 10 ERRORS**"
echo "============================"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

echo "1. Fixing duplicate NestGateUnifiedError definitions..."

# Remove duplicate pub use statements for NestGateUnifiedError
sed -i '/^pub use.*NestGateUnifiedError;$/d' code/crates/nestgate-core/src/error/variants/mod.rs
sed -i '/^pub use.*NestGateUnifiedError;$/d' code/crates/nestgate-core/src/error/mod.rs

echo "2. Fixing self-referencing nestgate_core:: paths..."

# Fix all self-referencing paths to use crate:: instead
find code/crates/nestgate-core -name "*.rs" -type f -exec sed -i 's/nestgate_core::/crate::/g' {} \;

echo "3. Ensuring proper module re-exports..."

# Make sure the main error type is properly exported from the root
if ! grep -q "pub use crate::error::variants::core_errors::NestGateUnifiedError;" code/crates/nestgate-core/src/lib.rs; then
    echo "pub use crate::error::variants::core_errors::NestGateUnifiedError;" >> code/crates/nestgate-core/src/lib.rs
fi

echo "4. Cleaning up any remaining import issues..."

# Clean up any remaining problematic imports
find code/crates/nestgate-core -name "*.rs" -type f -exec sed -i '/^use nestgate_core::/d' {} \;

echo "✅ Applied final error fixes"

echo ""
echo "📊 Checking compilation status..."
ERROR_COUNT=$(cargo check --workspace --message-format short 2>&1 | grep -c "error:" || echo "0")
WARNING_COUNT=$(cargo check --workspace --message-format short 2>&1 | grep -c "warning:" || echo "0")
echo "   Current errors: $ERROR_COUNT, warnings: $WARNING_COUNT"

if [ "$ERROR_COUNT" -eq "0" ]; then
    echo "🎉 **COMPILATION SUCCESSFUL!**"
else
    echo "⚠️ Still $ERROR_COUNT errors remaining"
fi

echo ""
echo "✅ **FINAL ERROR FIXES COMPLETE**" 