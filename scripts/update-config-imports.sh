#!/bin/bash

# Configuration Import Consolidation Script
# Updates all canonical_unified imports to use the new unified configuration system

set -e

echo "🔧 Starting Configuration Import Consolidation..."
echo "📊 Target: Convert canonical_unified → unified imports"

# Count current occurrences
BEFORE_COUNT=$(grep -r "config::canonical_unified" --include="*.rs" . | wc -l)
echo "📈 Found $BEFORE_COUNT files using canonical_unified imports"

# Update common patterns
echo "🔄 Updating import patterns..."

# Pattern 1: NestGateCanonicalUnifiedConfig → NestGateUnifiedConfig
find . -name "*.rs" -not -path "./target/*" -exec sed -i 's/config::canonical_unified::NestGateCanonicalUnifiedConfig/config::unified::NestGateUnifiedConfig/g' {} +

# Pattern 2: CanonicalConfig → NestGateUnifiedConfig  
find . -name "*.rs" -not -path "./target/*" -exec sed -i 's/config::canonical_unified::CanonicalConfig/config::unified::NestGateUnifiedConfig/g' {} +

# Pattern 3: General canonical_unified → unified
find . -name "*.rs" -not -path "./target/*" -exec sed -i 's/config::canonical_unified::/config::unified::/g' {} +

# Pattern 4: Update as clauses to use consistent naming
find . -name "*.rs" -not -path "./target/*" -exec sed -i 's/as NestGateCanonicalUnifiedConfig/as NestGateUnifiedConfig/g' {} +
find . -name "*.rs" -not -path "./target/*" -exec sed -i 's/as NestGateFinalConfig/as NestGateUnifiedConfig/g' {} +

# Count after changes
AFTER_COUNT=$(grep -r "config::canonical_unified" --include="*.rs" . | wc -l || echo "0")
echo "📉 Remaining canonical_unified imports: $AFTER_COUNT"

UPDATED_COUNT=$((BEFORE_COUNT - AFTER_COUNT))
echo "✅ Updated $UPDATED_COUNT import statements"

# Verify unified imports
UNIFIED_COUNT=$(grep -r "config::unified" --include="*.rs" . | wc -l)
echo "📊 Now using unified imports: $UNIFIED_COUNT"

echo ""
echo "🎯 Configuration Import Consolidation Summary:"
echo "   • Eliminated: $UPDATED_COUNT canonical_unified imports"  
echo "   • Unified imports: $UNIFIED_COUNT"
echo "   • Remaining old imports: $AFTER_COUNT"

if [ "$AFTER_COUNT" -eq 0 ]; then
    echo "🏆 SUCCESS: All imports successfully consolidated!"
else
    echo "⚠️  Manual review needed for remaining $AFTER_COUNT imports"
    echo "📋 Remaining imports:"
    grep -r "config::canonical_unified" --include="*.rs" . | head -10
fi

echo ""
echo "🚀 Testing compilation with unified imports..."
if cargo check --workspace > /dev/null 2>&1; then
    echo "✅ Compilation successful with unified configuration!"
else
    echo "⚠️  Compilation issues detected - may need type alignment"
fi

echo ""
echo "🔧 Configuration Import Consolidation Complete!" 