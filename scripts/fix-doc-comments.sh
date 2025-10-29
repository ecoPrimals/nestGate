#!/bin/bash
# 🔧 DOC COMMENTS FIX SCRIPT
# Fixes inner doc comment ordering issues

set -euo pipefail

echo "🔧 **FIXING DOC COMMENT ORDERING**"
echo "=================================="

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

# Fix service_discovery/mod.rs
if [ -f "code/crates/nestgate-core/src/service_discovery/mod.rs" ]; then
    echo "📝 Fixing service_discovery/mod.rs..."
    sed -i '1i//! Service Discovery Module' code/crates/nestgate-core/src/service_discovery/mod.rs
fi

# Fix zero_cost/mod.rs
if [ -f "code/crates/nestgate-core/src/zero_cost/mod.rs" ]; then
    echo "📝 Fixing zero_cost/mod.rs..."
    # Move all inner doc comments to the top
    temp_file=$(mktemp)
    echo "//! Zero-cost abstractions and performance optimizations" > "$temp_file"
    echo "//!" >> "$temp_file"
    grep -v "^//!" code/crates/nestgate-core/src/zero_cost/mod.rs >> "$temp_file"
    mv "$temp_file" code/crates/nestgate-core/src/zero_cost/mod.rs
fi

echo "✅ **DOC COMMENTS FIXED**"
echo "========================" 