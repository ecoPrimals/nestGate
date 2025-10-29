#!/bin/bash
# 🔧 FINAL COMPILATION FIXES
# Fixes all remaining compilation issues comprehensively

set -euo pipefail

echo "🔧 FINAL COMPILATION FIXES - COMPREHENSIVE CLEANUP"
echo "=================================================="

# 1. Fix remaining parameter name issues with snapshot_name
echo "🔄 Fixing snapshot_name parameter issues..."
find code/ -name "*.rs" -exec sed -i 's/_snapshot_name: snapshot_name: &strstr/_snapshot_name: \&str/g' {} \;

# 2. Fix underscore variable usage (remove underscores where variables are used)
echo "🔄 Fixing underscore variable usage..."
find code/ -name "*.rs" -exec sed -i 's/_endpoint: \&str/endpoint: \&str/g' {} \;
find code/ -name "*.rs" -exec sed -i 's/_value/value/g' {} \;

# 3. Fix error helper function calls
echo "🔄 Fixing error helper function calls..."
find code/ -name "*.rs" -exec sed -i 's/NestGateUnifiedError::configuration(/NestGateUnifiedError::configuration_error(/g' {} \;
find code/ -name "*.rs" -exec sed -i 's/NestGateUnifiedError::storage_error(message\.into())/NestGateUnifiedError::storage_error(\&message.into())/g' {} \;

echo "✅ Applied comprehensive fixes"

# 4. Test compilation
echo "🔄 Testing compilation..."
if cargo check --workspace --quiet 2>/dev/null; then
    echo "🎉 COMPILATION SUCCESSFUL!"
    RESULT="SUCCESS"
else
    echo "⚠️ Some issues may remain..."
    ERROR_COUNT=$(cargo check --workspace 2>&1 | grep -c "error\[" || echo "0")
    WARNING_COUNT=$(cargo check --workspace 2>&1 | grep -c "warning:" || echo "0")
    echo "📊 Errors: $ERROR_COUNT | Warnings: $WARNING_COUNT"
    RESULT="PARTIAL"
fi

echo ""
echo "🎯 FINAL RESULT: $RESULT" 