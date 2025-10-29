#!/bin/bash
# 🔧 **FIX MATCH ARM SYNTAX ERRORS**
# Systematically fixes malformed match arms using = instead of =>

set -euo pipefail

echo "🔧 **FIXING MATCH ARM SYNTAX ERRORS**"
echo "===================================="

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

echo "📝 Fixing match arms with incorrect syntax..."

# Fix match arms: Self::Something = write!(f, "something") -> Self::Something => write!(f, "something")
find code/crates/nestgate-core/src/unified_enums -name "*.rs" -type f -exec sed -i \
  's/Self::\([^=]*\) = write!(f, \([^)]*\))/Self::\1 => write!(f, \2)/g' {} \;

# Fix string match arms: Self::Something = "something" -> Self::Something => "something"
find code/crates/nestgate-core/src/unified_enums -name "*.rs" -type f -exec sed -i \
  's/Self::\([^=]*\) = "\([^"]*\)"/Self::\1 => "\2"/g' {} \;

# Fix custom variant match arms: Self::Custom(name) = ... -> Self::Custom(name) => ...
find code/crates/nestgate-core/src/unified_enums -name "*.rs" -type f -exec sed -i \
  's/Self::Custom(\([^)]*\)) = \(.*\)/Self::Custom(\1) => \2/g' {} \;

# Fix other pattern match arms
find code/crates/nestgate-core/src/unified_enums -name "*.rs" -type f -exec sed -i \
  's/custom: =>/custom =>/g' {} \;

echo "📝 Fixing empty vector syntax..."
# Fix empty vector with trailing comma
find code/crates/nestgate-core/src -name "*.rs" -type f -exec sed -i \
  's/vec!\[,/vec![/g' {} \;

echo "✅ Fixed match arm syntax errors"

echo "📊 Running cargo check to verify fixes..."
cargo check --workspace --quiet && echo "✅ All syntax errors fixed!" || echo "⚠️  Some issues remain" 