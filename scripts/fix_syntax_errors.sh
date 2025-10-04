#!/bin/bash
# 🔧 **SYNTAX ERROR FIXES**
# Fix specific syntax errors from regex replacements

set -euo pipefail

echo "🔧 **FIXING SYNTAX ERRORS**"
echo "=========================="

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

echo "1. Fixing duplicate async keywords..."

# Fix duplicate async keywords
sed -i 's/async async fn/async fn/g' code/crates/nestgate-core/src/services/storage/service.rs

echo "2. Fixing mangled function parameters..."

# Fix the mangled snapshot_name parameter
sed -i 's/__snapshot_name: __snapshot_name: _snapshot_name: &strstrstr/_snapshot_name: \&str/g' code/crates/nestgate-core/src/services/storage/service.rs

echo "3. Fixing other parameter issues..."

# Fix any other mangled parameters
find code/crates -name "*.rs" -type f -exec sed -i 's/&strstr/\&str/g' {} \;
find code/crates -name "*.rs" -type f -exec sed -i 's/&strstrstr/\&str/g' {} \;

echo "4. Fixing escaped ampersands..."

# Fix escaped ampersands that should be regular references
find code/crates -name "*.rs" -type f -exec sed -i 's/\\&/\&/g' {} \;

echo "5. Checking for other common syntax issues..."

# Fix any remaining parameter syntax issues
find code/crates -name "*.rs" -type f -exec sed -i 's/: : /: /g' {} \;
find code/crates -name "*.rs" -type f -exec sed -i 's/__\([a-zA-Z_][a-zA-Z0-9_]*\): __\1/_\1/g' {} \;

echo "✅ Fixed syntax errors"

echo ""
echo "📊 Checking compilation status..."
ERROR_COUNT=$(cargo check --workspace --message-format short 2>&1 | grep -c "error:" || echo "0")
WARNING_COUNT=$(cargo check --workspace --message-format short 2>&1 | grep -c "warning:" || echo "0")
echo "   Current errors: $ERROR_COUNT, warnings: $WARNING_COUNT"

echo ""
echo "✅ **SYNTAX ERROR FIXES COMPLETE**" 