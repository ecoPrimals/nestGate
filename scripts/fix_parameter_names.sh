#!/bin/bash
# 🔧 FIX PARAMETER NAME ISSUES
# Fixes all malformed parameter declarations

set -euo pipefail

echo "🔧 FIXING PARAMETER NAME ISSUES"
echo "==============================="

# Fix the main pattern: _endpoint: endpoint: &strstr -> _endpoint: &str
find code/ -name "*.rs" -exec sed -i 's/_endpoint: endpoint: &strstr/_endpoint: \&str/g' {} \;

# Fix variations with different underscores
find code/ -name "*.rs" -exec sed -i 's/__endpoint: endpoint: &strstr/__endpoint: \&str/g' {} \;
find code/ -name "*.rs" -exec sed -i 's/_capability__endpoint: endpoint: &strstr/_capability__endpoint: \&str/g' {} \;
find code/ -name "*.rs" -exec sed -i 's/_orchestration__endpoint: endpoint: &strstr/_orchestration__endpoint: \&str/g' {} \;
find code/ -name "*.rs" -exec sed -i 's/_compute__endpoint: endpoint: &strstr/_compute__endpoint: \&str/g' {} \;

echo "✅ Fixed parameter name issues"

# Test compilation
echo "🔄 Testing compilation..."
if cargo check --workspace --quiet 2>/dev/null; then
    echo "✅ Compilation successful!"
else
    echo "⚠️ Some issues remain, checking details..."
    cargo check --workspace 2>&1 | grep -E "(error|warning)" | head -5
fi 