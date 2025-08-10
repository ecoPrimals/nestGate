#!/bin/bash
# Systematic ZfsConfig to UnifiedZfsConfig migration

echo "🔄 MIGRATING ZfsConfig to UnifiedZfsConfig"
echo "========================================="

# Backup files first
echo "📋 Creating backups..."
find . -name "*.rs" -path "*/tests/*" -exec cp {} {}.backup \;
find . -name "*.rs" -path "*/fuzz/*" -exec cp {} {}.backup \;
find . -name "*.rs" -path "*/code/*" -exec cp {} {}.backup \;

echo "🔧 Step 1: Update imports"
# Update import statements
find . -name "*.rs" -exec sed -i 's/use nestgate_zfs::{config::ZfsConfig/use nestgate_zfs::{config::UnifiedZfsConfig/g' {} \;
find . -name "*.rs" -exec sed -i 's/use nestgate_zfs::config::ZfsConfig/use nestgate_zfs::config::UnifiedZfsConfig/g' {} \;
find . -name "*.rs" -exec sed -i 's/nestgate_zfs::{ZfsConfig/nestgate_zfs::{UnifiedZfsConfig/g' {} \;

echo "🔧 Step 2: Update variable declarations"
# Update variable type declarations  
find . -name "*.rs" -exec sed -i 's/: ZfsConfig/: UnifiedZfsConfig/g' {} \;
find . -name "*.rs" -exec sed -i 's/<ZfsConfig>/<UnifiedZfsConfig>/g' {} \;

echo "🔧 Step 3: Update constructor calls"
# Update ZfsConfig::default() calls
find . -name "*.rs" -exec sed -i 's/ZfsConfig::default()/UnifiedZfsConfig::default()/g' {} \;

echo "🔧 Step 4: Update function parameters and returns"
# Update function signatures
find . -name "*.rs" -exec sed -i 's/config: ZfsConfig/config: UnifiedZfsConfig/g' {} \;
find . -name "*.rs" -exec sed -i 's/-> ZfsConfig/-> UnifiedZfsConfig/g' {} \;

echo "🔧 Step 5: Update struct field types"
# Update struct fields
find . -name "*.rs" -exec sed -i 's/config: ZfsConfig,/config: UnifiedZfsConfig,/g' {} \;

echo "✅ MIGRATION COMPLETE"
echo "📊 Checking for remaining ZfsConfig references..."

REMAINING=$(grep -r "ZfsConfig" --include="*.rs" . | grep -v ".backup" | grep -v "UnifiedZfsConfig" | wc -l)
echo "🔍 Remaining ZfsConfig references: $REMAINING"

if [ "$REMAINING" -eq 0 ]; then
    echo "🎉 SUCCESS: All ZfsConfig references migrated to UnifiedZfsConfig"
else
    echo "⚠️  Manual review needed for remaining references"
    grep -r "ZfsConfig" --include="*.rs" . | grep -v ".backup" | grep -v "UnifiedZfsConfig" | head -10
fi 