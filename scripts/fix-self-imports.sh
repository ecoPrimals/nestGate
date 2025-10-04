#!/bin/bash
# 🔧 **FIX SELF-REFERENCING IMPORTS**
# Systematically fixes self-referencing nestgate_core imports

set -euo pipefail

echo "🔧 **FIXING SELF-REFERENCING IMPORTS**"
echo "====================================="

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

echo "📝 Fixing self-referencing nestgate_core imports..."

# Fix self-referencing imports
find code/crates/nestgate-core/src -name "*.rs" -type f -exec sed -i \
  's/use nestgate_core::/use crate::/g' {} \;

echo "✅ Fixed self-referencing imports"

echo "📊 Running cargo check to verify fixes..."
cargo check --workspace --quiet && echo "✅ All import errors fixed!" || echo "⚠️  Some issues remain" 