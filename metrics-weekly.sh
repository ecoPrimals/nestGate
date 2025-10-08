#!/bin/bash
# Weekly Metrics Tracker
# Run this at the end of each week

WEEK=$(date +%U)
DATE=$(date +%Y-%m-%d)

echo "========================================"
echo "NESTGATE METRICS - Week $WEEK"
echo "Date: $DATE"
echo "========================================"

# Coverage
echo ""
echo "📊 TEST COVERAGE:"
cargo tarpaulin --lib --timeout 120 2>&1 | grep "coverage" || echo "Coverage measurement in progress..."

# Tests
echo ""
echo "✅ TESTS:"
cargo test --lib --workspace 2>&1 | grep "test result" || echo "Tests running..."

# Technical Debt
echo ""
echo "📉 TECHNICAL DEBT:"
echo "  Mocks: $(grep -r 'mock\|Mock\|MOCK' code/ --include='*.rs' | wc -l)"
echo "  Unwraps: $(grep -r 'unwrap\(\|expect\(' code/ --include='*.rs' | wc -l)"
echo "  Hardcoded: $(grep -r '8080\|8443\|3000\|localhost\|127\.0\.0\.1' code/ --include='*.rs' | wc -l)"
echo "  Unsafe undocumented: $(comm -13 <(grep -r 'SAFETY:' code/ --include='*.rs' | wc -l) <(grep -r 'unsafe' code/ --include='*.rs' | wc -l))"

# Clippy
echo ""
echo "🔧 CODE QUALITY:"
echo "  Clippy warnings: $(cargo clippy --workspace --lib 2>&1 | grep 'warning:' | wc -l 2>/dev/null || echo 'TBD')"
echo "  Format issues: $(cargo fmt --check 2>&1 | wc -l)"

echo ""
echo "========================================"
echo "Metrics saved to METRICS_LOG.txt"
echo "========================================"

# Append to log
{
    echo ""
    echo "=== Week $WEEK: $DATE ==="
    echo "Coverage: $(cargo tarpaulin --lib --timeout 120 2>&1 | grep '%' | head -1 || echo 'TBD')"
    echo "Tests: $(cargo test --lib --workspace 2>&1 | grep 'test result' || echo 'TBD')"
    echo "Mocks: $(grep -r 'mock\|Mock\|MOCK' code/ --include='*.rs' | wc -l)"
    echo "Unwraps: $(grep -r 'unwrap\(\|expect\(' code/ --include='*.rs' | wc -l)"
    echo "Hardcoded: $(grep -r '8080\|8443\|3000\|localhost\|127\.0\.0\.1' code/ --include='*.rs' | wc -l)"
    echo "Clippy: $(cargo clippy --workspace --lib 2>&1 | grep 'warning:' | wc -l 2>/dev/null || echo 'TBD')"
} >> METRICS_LOG.txt

