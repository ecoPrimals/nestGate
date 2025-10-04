#!/bin/bash
# 🔧 **FIX CONST GENERIC PARAMETERS**
# Systematically fixes const generic parameter syntax issues

set -euo pipefail

echo "🔧 **FIXING CONST GENERIC PARAMETERS**"
echo "======================================"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

echo "📝 Fixing const generic parameter expressions..."

# Fix all instances of const generic parameters that need braces
find code/crates -name "*.rs" -type f -exec sed -i \
  's/const BUFFER_SIZE: usize = crate::constants::system::DEFAULT_BUFFER_SIZE/const BUFFER_SIZE: usize = { crate::constants::system::DEFAULT_BUFFER_SIZE }/g' {} \;

find code/crates -name "*.rs" -type f -exec sed -i \
  's/const TIMEOUT_MS: u64 = crate::constants::network::connection_timeout().as_millis() as u64/const TIMEOUT_MS: u64 = { crate::constants::network::connection_timeout().as_millis() as u64 }/g' {} \;

find code/crates -name "*.rs" -type f -exec sed -i \
  's/const API_PORT: u16 = crate::constants::network::api_port()/const API_PORT: u16 = { crate::constants::network::api_port() }/g' {} \;

echo "✅ Const generic parameter fixes applied!"

echo ""
echo "🔧 Running cargo check to validate fixes..."
if cargo check --workspace --quiet 2>/dev/null; then
  echo "✅ All const generic fixes successful!"
else
  echo "⚠️  Still some issues remaining"
  cargo check --workspace 2>&1 | head -10
fi 