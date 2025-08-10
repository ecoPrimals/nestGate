#!/bin/bash
# Systematic McpConfig to UnifiedMcpConfig migration

echo "🔄 MIGRATING McpConfig to UnifiedMcpConfig"
echo "========================================"

# Backup files first
echo "📋 Creating backups..."
find . -name "*.rs" -path "*/tests/*" -exec cp {} {}.backup.mcp \;
find . -name "*.rs" -path "*/fuzz/*" -exec cp {} {}.backup.mcp \;
find . -name "*.rs" -path "*/code/*" -exec cp {} {}.backup.mcp \;

echo "🔧 Step 1: Update imports"
# Update import statements
find . -name "*.rs" -exec sed -i 's/use nestgate_mcp::{config::McpConfig/use nestgate_mcp::{config::UnifiedMcpConfig/g' {} \;
find . -name "*.rs" -exec sed -i 's/use nestgate_mcp::config::McpConfig/use nestgate_mcp::config::UnifiedMcpConfig/g' {} \;
find . -name "*.rs" -exec sed -i 's/nestgate_mcp::{McpConfig/nestgate_mcp::{UnifiedMcpConfig/g' {} \;

echo "🔧 Step 2: Update variable declarations"
# Update variable type declarations
find . -name "*.rs" -exec sed -i 's/: McpConfig/: UnifiedMcpConfig/g' {} \;
find . -name "*.rs" -exec sed -i 's/<McpConfig>/<UnifiedMcpConfig>/g' {} \;

echo "🔧 Step 3: Update constructor calls"
# Update McpConfig::default() calls
find . -name "*.rs" -exec sed -i 's/McpConfig::default()/UnifiedMcpConfig::default()/g' {} \;

echo "🔧 Step 4: Update function parameters and returns"
# Update function signatures
find . -name "*.rs" -exec sed -i 's/config: McpConfig/config: UnifiedMcpConfig/g' {} \;
find . -name "*.rs" -exec sed -i 's/-> McpConfig/-> UnifiedMcpConfig/g' {} \;

echo "🔧 Step 5: Update struct field types"
# Update struct fields
find . -name "*.rs" -exec sed -i 's/config: McpConfig,/config: UnifiedMcpConfig,/g' {} \;

echo "✅ MIGRATION COMPLETE"
echo "📊 Checking for remaining McpConfig references..."

REMAINING=$(grep -r "McpConfig" --include="*.rs" . | grep -v ".backup" | grep -v "UnifiedMcpConfig" | wc -l)
echo "🔍 Remaining McpConfig references: $REMAINING"

if [ "$REMAINING" -eq 0 ]; then
    echo "🎉 SUCCESS: All McpConfig references migrated to UnifiedMcpConfig"
else
    echo "⚠️  Manual review needed for remaining references"
    grep -r "McpConfig" --include="*.rs" . | grep -v ".backup" | grep -v "UnifiedMcpConfig" | head -10
fi 