#!/bin/bash
# 🔧 SIMPLE COMPILATION FIXES
# Targets the most critical compilation errors

set -euo pipefail

echo "🔧 SIMPLE COMPILATION FIXES - CRITICAL ERRORS ONLY"
echo "=================================================="

# 1. Fix storage error calls
echo "🔄 Fixing storage error calls..."
find code/ -name "*.rs" -exec sed -i 's/NestGateError::storage(/NestGateError::storage_error(/g' {} \;

# 2. Fix string contains patterns
echo "🔄 Fixing string contains patterns..."
find code/ -name "*.rs" -exec sed -i 's/\.contains("container_runtime"\.to_string())/\.contains("container_runtime")/g' {} \;

# 3. Fix unused variables
echo "🔄 Fixing unused variables..."
find code/ -name "*.rs" -exec sed -i 's/let mut config = zfs_default();/let config = zfs_default();/g' {} \;
find code/ -name "*.rs" -exec sed -i 's/let mut config = ZfsConfig::default();/let config = ZfsConfig::default();/g' {} \;

# 4. Test compilation
echo "🔄 Testing compilation..."
if cargo check --workspace --quiet 2>/dev/null; then
    echo "✅ Compilation successful!"
else
    echo "⚠️ Some issues remain, but major fixes applied"
fi

echo "✅ Simple compilation fixes complete" 