#!/bin/bash
# 🎯 NESTGATE PERFECTION ACHIEVEMENT SCRIPT
# Lower entropy code through systematic quality improvements

set -euo pipefail

REPORT_DIR="reports/perfection-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$REPORT_DIR"

echo "🎯 NESTGATE PERFECTION PROTOCOL"
echo "==============================="

echo "Phase 1: COMPILATION"
if cargo check --all-targets --all-features > "$REPORT_DIR/compilation.log" 2>&1; then
    echo "✅ Compilation: CLEAN"
else
    echo "❌ Compilation: NEEDS FIXES"
    head -20 "$REPORT_DIR/compilation.log"
fi

echo ""
echo "Phase 2: LINTING"
cargo clippy --all-targets --all-features -- -D warnings > "$REPORT_DIR/clippy.log" 2>&1 || true
CLIPPY_ISSUES=$(grep -c "error:\|warning:" "$REPORT_DIR/clippy.log" || echo "0")
echo "�� Clippy issues: $CLIPPY_ISSUES"

echo ""
echo "Phase 3: FORMATTING"
if cargo fmt --all -- --check > "$REPORT_DIR/formatting.log" 2>&1; then
    echo "✅ Formatting: PERFECT"
else
    cargo fmt --all
    echo "✅ Formatting: FIXED"
fi

echo ""
echo "Phase 4: COVERAGE"
cargo tarpaulin --all-features --workspace --out Html --output-dir "$REPORT_DIR" --timeout 300 > "$REPORT_DIR/coverage.log" 2>&1 || true
COVERAGE=$(grep -oP "Coverage: \K[0-9.]+%" "$REPORT_DIR/coverage.log" | tail -1 || echo "0%")
echo "📊 Coverage: $COVERAGE"

echo ""
echo "🚀 Report: $REPORT_DIR"
