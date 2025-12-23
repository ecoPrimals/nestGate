#!/bin/bash
# Clippy Pedantic Batch Fix Script
# Systematically fixes common clippy pedantic issues

set -euo pipefail

cd "$(dirname "$0")"

echo "🔧 Starting Clippy Pedantic Batch Fixes..."
echo "=========================================="

# Run cargo clippy and save output
echo "📊 Running clippy to identify issues..."
cargo clippy --lib --all-features -- -W clippy::pedantic 2>&1 > /tmp/clippy_output.txt || true

# Count issues
MISSING_BACKTICKS=$(grep -c "missing backticks" /tmp/clippy_output.txt || echo "0")
MISSING_ERRORS=$(grep -c "Missing.*Errors" /tmp/clippy_output.txt || echo "0")  
MISSING_MUST_USE=$(grep -c "must_use" /tmp/clippy_output.txt || echo "0")

echo ""
echo "📋 Issues Found:"
echo "   - Missing backticks: $MISSING_BACKTICKS"
echo "   - Missing # Errors:  $MISSING_ERRORS"
echo "   - Missing #[must_use]: $MISSING_MUST_USE"
echo ""

# Extract file list
echo "📁 Files needing fixes:"
grep "^  -->" /tmp/clippy_output.txt | awk '{print $2}' | sort -u | head -20

echo ""
echo "✅ Analysis complete!"
echo ""
echo "📝 Manual fixes needed for:"
echo "   1. Add backticks around type names in docs"
echo "   2. Add '# Errors' sections to Result-returning functions"  
echo "   3. Add #[must_use] to Self-returning methods"
echo ""
echo "🎯 Estimated time: 1-2 hours for all fixes"




