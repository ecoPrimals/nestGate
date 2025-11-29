#!/usr/bin/env bash
# Fix Immediate Blockers - November 28, 2025
# This script fixes the critical issues preventing test execution

set -e

echo "🚨 NESTGATE IMMEDIATE BLOCKER FIX SCRIPT"
echo "========================================="
echo ""

# Change to project root
cd "$(dirname "$0")"

echo "📍 Working directory: $(pwd)"
echo ""

# Step 1: Fix Formatting
echo "Step 1: Fixing formatting violations..."
echo "---------------------------------------"
cargo fmt --all
echo "✅ Formatting fixed"
echo ""

# Step 2: Check test compilation
echo "Step 2: Checking test compilation..."
echo "------------------------------------"
if cargo test --workspace --lib --no-run 2>&1 | tee /tmp/nestgate_test_compile.log; then
    echo "✅ Tests compile successfully!"
else
    echo "❌ Tests still don't compile. Errors saved to /tmp/nestgate_test_compile.log"
    echo ""
    echo "Common fixes needed:"
    echo "1. Check code/crates/nestgate-network/src/lib.rs for missing module declarations"
    echo "2. Verify unified_network_config/network_settings.rs exists"
    echo "3. Verify unified_network_extensions/orchestration_tests.rs exists"
    echo ""
    echo "Manual intervention required. See error log above."
    exit 1
fi
echo ""

# Step 3: Run tests
echo "Step 3: Running library tests..."
echo "---------------------------------"
if cargo test --workspace --lib 2>&1 | tee /tmp/nestgate_test_run.log | grep -E "test result:|running"; then
    echo "✅ Tests ran (see results above)"
else
    echo "⚠️ Some tests may have failed. Full log: /tmp/nestgate_test_run.log"
fi
echo ""

# Step 4: Measure coverage
echo "Step 4: Measuring test coverage..."
echo "-----------------------------------"
if cargo llvm-cov --workspace --lib --summary-only 2>&1 | tee /tmp/nestgate_coverage.log; then
    echo "✅ Coverage measured successfully!"
    echo ""
    echo "For detailed HTML report, run:"
    echo "  cargo llvm-cov --workspace --lib --html"
    echo "  open target/llvm-cov/html/index.html"
else
    echo "⚠️ Coverage measurement failed. Log: /tmp/nestgate_coverage.log"
    echo ""
    echo "This might be due to:"
    echo "1. llvm-cov not installed (cargo install cargo-llvm-cov)"
    echo "2. Tests still failing"
    echo "3. Missing documentation (check warnings)"
fi
echo ""

# Step 5: Verify linting
echo "Step 5: Verifying library linting..."
echo "--------------------------------------"
if cargo clippy --workspace --lib -- -D warnings; then
    echo "✅ Library code passes clippy with -D warnings"
else
    echo "⚠️ Clippy warnings detected in library code"
fi
echo ""

# Summary
echo "========================================="
echo "🎯 SUMMARY"
echo "========================================="
echo ""
echo "Logs saved to:"
echo "  - Test compilation: /tmp/nestgate_test_compile.log"
echo "  - Test execution: /tmp/nestgate_test_run.log"
echo "  - Coverage: /tmp/nestgate_coverage.log"
echo ""
echo "Next steps:"
echo "1. Review any errors in the logs above"
echo "2. Update documentation with REAL numbers"
echo "3. Address technical debt systematically"
echo ""
echo "See COMPREHENSIVE_REALITY_AUDIT_NOV_28_2025.md for full analysis"
echo "See URGENT_REALITY_CHECK_NOV_28.md for quick summary"
echo ""
echo "Done! 🎉"

