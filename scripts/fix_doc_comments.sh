#!/bin/bash
# 🔧 **DOC COMMENT FIXES**
# Fix doc comment structure issues caused by import modifications

set -euo pipefail

echo "🔧 **FIXING DOC COMMENT STRUCTURE**"
echo "==================================="

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

echo "Fixing doc comment structure issues..."

# Remove problematic import lines that were added at the beginning of files
find code/crates -name "*.rs" -type f -exec sed -i '1{/^use nestgate_core::unified_enums/d;}' {} \;
find code/crates -name "*.rs" -type f -exec sed -i '1{/^use nestgate_core::constants/d;}' {} \;
find code/crates -name "*.rs" -type f -exec sed -i '1{/^use nestgate_core::config/d;}' {} \;

# Fix files where the doc comment structure was broken
find code/crates -name "*.rs" -type f -exec sed -i '1{/^$/d;}' {} \;

# Ensure proper doc comment structure for crate-level documentation
for file in $(find code/crates -name "lib.rs" -o -name "mod.rs" | head -10); do
    if [[ -f "$file" ]]; then
        # Check if the file starts with a comment that should be a doc comment
        if head -n 3 "$file" | grep -q "^//[^!]"; then
            sed -i '1s|^//|//!|' "$file"
        fi
    fi
done

echo "✅ Fixed doc comment structure"

echo ""
echo "📊 Checking compilation status..."
ERROR_COUNT=$(cargo check --workspace --message-format short 2>&1 | grep -c "error:" || echo "0")
WARNING_COUNT=$(cargo check --workspace --message-format short 2>&1 | grep -c "warning:" || echo "0")
echo "   Current errors: $ERROR_COUNT, warnings: $WARNING_COUNT"

echo ""
echo "✅ **DOC COMMENT FIXES COMPLETE**" 