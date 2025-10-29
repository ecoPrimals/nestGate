#!/bin/bash
# 🔧 FINAL SYNTAX FIXES SCRIPT
# Fixes all remaining syntax issues for clean compilation

set -euo pipefail

echo "🔧 **FINAL SYNTAX FIXES**"
echo "========================"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

# Fix generic constraint syntax issues
echo "📝 Fixing generic constraint syntax..."

# Fix network_discovery.rs generic constraints
find code/crates -name "*.rs" -exec sed -i 's/impl std::future::Future<Output = Result<[^>]*> + Send, [^>]*>/std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<CapabilityInfo>, NestGateError>> + Send>>/g' {} \;

# Fix infant_discovery/mod.rs generic constraints  
find code/crates -name "*.rs" -exec sed -i 's/impl std::future::Future<Output = Result<[^>]*> + Send, [^>]*>/std::pin::Pin<Box<dyn std::future::Future<Output = Result<InfantDiscoveryResult, NestGateError>> + Send>>/g' {} \;

# Fix universal_adapter/mod.rs generic constraints
find code/crates -name "*.rs" -exec sed -i 's/impl std::future::Future<Output = Result<[^>]*> + Send, [^>]*>/std::pin::Pin<Box<dyn std::future::Future<Output = Result<UniversalAdapterResult, NestGateError>> + Send>>/g' {} \;

echo "📝 Adding missing async block closures..."

# Add Box::pin(async move { to functions that need it
find code/crates -name "*.rs" -exec grep -l "std::pin::Pin<Box<dyn std::future::Future" {} \; | while read file; do
    # Add async block if missing
    if grep -q "std::pin::Pin<Box<dyn std::future::Future" "$file" && ! grep -q "Box::pin(async move {" "$file"; then
        sed -i 's/std::pin::Pin<Box<dyn std::future::Future<Output = \([^>]*\)> + Send>> {/std::pin::Pin<Box<dyn std::future::Future<Output = \1> + Send>> {\n        Box::pin(async move {/' "$file"
    fi
done

echo "📝 Fixing remaining import issues..."

# Fix remaining import issues
find code/crates -name "*.rs" -exec sed -i 's/use super::{Result, Error};/use crate::error::{Result, NestGateError};/g' {} \;

echo "✅ **FINAL SYNTAX FIXES APPLIED**"
echo "================================="

# Quick compilation check
echo "🧪 Testing compilation..."
if cargo check --package nestgate-core --message-format short 2>&1 | head -5; then
    echo "✅ Compilation check completed"
else
    echo "⚠️  Some issues may remain"
fi 