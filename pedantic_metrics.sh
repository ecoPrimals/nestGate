#!/bin/bash

echo "📊 PEDANTIC METRICS REPORT"
echo "=========================="

# Constants elimination
hardcoded_count=$(grep -r "localhost:8080\|\"8080\"\|127\.0\.0\.1:8080" code/ --include="*.rs" | wc -l)
echo "🎯 Hardcoded references: $hardcoded_count (TARGET: 0)"

# NetworkConfig consolidation
networkconfig_count=$(find code/ -name "*.rs" -exec grep -l "struct.*NetworkConfig" {} \; | wc -l)
echo "🌐 NetworkConfig instances: $networkconfig_count (TARGET: 1)"

# Migration helper usage
migration_helper_count=$(grep -r "ConstantsMigrationHelper" code/ --include="*.rs" | wc -l)
echo "🔧 Migration helper usage: $migration_helper_count"

# Compilation status
if cargo check --quiet 2>/dev/null; then
    echo "✅ Compilation: PERFECT"
else
    echo "❌ Compilation: FAILED"
fi

# Progress percentage
total_configs=1135
remaining_configs=$(grep -r "struct.*Config" code/ --include="*.rs" | wc -l)
progress=$(( (total_configs - remaining_configs) * 100 / total_configs ))
echo "📈 Overall progress: $progress% ($remaining_configs remaining)"

echo ""
echo "🏆 PEDANTIC STANDARD: 100% or FAILURE"
