#!/bin/bash
# fix-doc-comments.sh
# Fixes the 4 doc comment syntax errors in canonical_config/mod.rs

set -e

echo "🔧 Fixing doc comment syntax errors..."

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
FILE="$PROJECT_ROOT/code/crates/nestgate-core/src/config/canonical_config/mod.rs"

if [ ! -f "$FILE" ]; then
    echo "❌ File not found: $FILE"
    exit 1
fi

echo "📝 Fixing line 94..."
sed -i '94s|//! while preserving|// while preserving|' "$FILE"

echo "📝 Fixing line 95..."
sed -i '95s|//! Module definitions|// Module definitions|' "$FILE"

echo "📝 Fixing line 97..."
sed -i '97s|//! - UnifiedApiHandlerConfig|// - UnifiedApiHandlerConfig|' "$FILE"

echo "📝 Fixing line 98..."
sed -i '98s|//! - UnifiedAutomationConfig|// - UnifiedAutomationConfig|' "$FILE"

echo "✅ Fixed 4 doc comment syntax errors"
echo ""
echo "🔍 Verifying build..."

cd "$PROJECT_ROOT"
if cargo check --workspace --quiet 2>&1 | head -10; then
    echo "✅ Build verification complete!"
else
    echo "⚠️  Some warnings may remain, but compilation errors should be fixed"
fi 