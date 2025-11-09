#!/bin/bash
# 🚀 ASYNC_TRAIT ELIMINATION SCRIPT
# Converts remaining async_trait usages to native async (RPITIT)
# Part of 97% → 100% Unification Plan - Priority 2

set -euo pipefail

echo "🚀 **NESTGATE ASYNC_TRAIT ELIMINATION**"
echo "========================================"
echo ""

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo "📊 **PHASE 1: ANALYSIS**"
echo "-----------------------"

# Find all async_trait usages
ASYNC_TRAIT_FILES=$(grep -rl "#\[async_trait\]" code/crates --include="*.rs" 2>/dev/null || true)
ASYNC_TRAIT_COUNT=$(echo "$ASYNC_TRAIT_FILES" | grep -c . || echo "0")

echo "Found async_trait in $ASYNC_TRAIT_COUNT files:"
echo ""
echo "$ASYNC_TRAIT_FILES" | while read -r file; do
    if [ -n "$file" ]; then
        COUNT=$(grep -c "#\[async_trait\]" "$file" 2>/dev/null || echo "0")
        echo "  - $file ($COUNT usages)"
    fi
done
echo ""

if [ "$ASYNC_TRAIT_COUNT" -eq 0 ]; then
    echo -e "${GREEN}✅ No async_trait usages found - already 100% native async!${NC}"
    exit 0
fi

echo "🎯 **TARGET**: Convert to native async (RPITIT pattern)"
echo ""
echo "Pattern transformation:"
echo "  FROM: #[async_trait] trait Foo { async fn bar(&self) -> Result<T>; }"
echo "  TO:   trait Foo { fn bar(&self) -> impl Future<Output = Result<T>> + Send; }"
echo ""

echo -e "${YELLOW}⚠️  NOTE: async_trait elimination requires manual conversion${NC}"
echo -e "${YELLOW}   This script identifies files - manual migration recommended${NC}"
echo ""

echo "📋 **FILES REQUIRING MANUAL CONVERSION**:"
echo "------------------------------------------"

echo "$ASYNC_TRAIT_FILES" | while read -r file; do
    if [ -n "$file" ]; then
        echo ""
        echo "FILE: $file"
        grep -n "#\[async_trait\]" "$file" 2>/dev/null | head -5
    fi
done

echo ""
echo "✅ **ANALYSIS COMPLETE**"
echo "======================="
echo ""
echo "Next steps:"
echo "1. Review each file listed above"
echo "2. Convert async_trait to native async (RPITIT)"
echo "3. Run: cargo check --workspace"
echo "4. Run: cargo test --workspace --lib"
echo ""
echo "Example conversion pattern:"
echo ""
echo "  // BEFORE (async_trait):"
echo "  #[async_trait]"
echo "  trait MyService {"
echo "      async fn process(&self, data: Data) -> Result<Output>;"
echo "  }"
echo ""
echo "  // AFTER (native async - RPITIT):"
echo "  trait MyService {"
echo "      fn process(&self, data: Data) -> impl Future<Output = Result<Output>> + Send;"
echo "  }"
echo ""

