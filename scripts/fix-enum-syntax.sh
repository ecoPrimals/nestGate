#!/bin/bash
# 🔧 **FIX ENUM SYNTAX ERRORS**
# Systematically fixes malformed enum default implementations

set -euo pipefail

echo "🔧 **FIXING ENUM SYNTAX ERRORS**"
echo "================================="

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

echo "📝 Fixing malformed enum default implementations..."

# Fix all instances of Self::Something> to Self::Something
find code/crates/nestgate-core/src/unified_enums -name "*.rs" -type f -exec sed -i \
  's/Self::\([^>]*\)>/Self::\1/g' {} \;

echo "✅ Fixed enum syntax errors"

echo "📊 Running cargo check to verify fixes..."
cargo check --workspace --quiet && echo "✅ All syntax errors fixed!" || echo "⚠️  Some issues remain" 