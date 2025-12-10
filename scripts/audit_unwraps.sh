#!/usr/bin/env bash
#
# Unwrap/Expect Audit Tool
#
# Identifies unwrap() and expect() calls in production code that should
# be converted to proper Result<T, E> error handling.
#
# Usage: ./scripts/audit_unwraps.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

echo "🔍 Auditing unwrap/expect usage..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Total count
TOTAL=$(rg '\.unwrap\(\)|\.expect\(' code/crates --type rust | wc -l)
echo "📊 Total unwrap/expect calls: $TOTAL"
echo ""

# Count excluding tests
PRODUCTION=$(rg '\.unwrap\(\)|\.expect\(' code/crates --type rust \
    | grep -v test \
    | grep -v "tests/" \
    | grep -v "_tests.rs" \
    | wc -l)
echo "⚠️  Production code (non-test): $PRODUCTION"
echo ""

# Count with "Safe:" justification
JUSTIFIED=$(rg '// Safe:' code/crates --type rust | wc -l)
echo "✅ Justified (with // Safe: comment): $JUSTIFIED"
echo ""

# Calculate unjustified
UNJUSTIFIED=$((PRODUCTION - JUSTIFIED))
echo "🔴 Unjustified (needs review): $UNJUSTIFIED"
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📁 Top files needing attention (production code):"
echo ""

rg '\.unwrap\(\)|\.expect\(' code/crates --type rust \
    | grep -v test \
    | grep -v "tests/" \
    | grep -v "_tests.rs" \
    | cut -d: -f1 \
    | sort | uniq -c | sort -rn | head -20

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "💡 Conversion Pattern:"
echo ""
echo "  BEFORE: let value = parse_config().unwrap();"
echo "  AFTER:  let value = parse_config()"
echo "              .context(\"Failed to parse configuration\")?;"
echo ""
echo "  Requires: use anyhow::{Context, Result};"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

