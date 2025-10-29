#!/bin/bash
# Find Orphaned Test Files Script
# Identifies test files that are not imported into the module tree

set -e

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

OUTPUT_DIR="$REPO_ROOT/test-wiring-audit"
mkdir -p "$OUTPUT_DIR"

echo "🔍 Scanning for orphaned test files..."
echo "Output directory: $OUTPUT_DIR"
echo ""

# Find all test files
find code/crates -type f -name "*.rs" \( -name "*test*.rs" -o -name "*tests.rs" \) > "$OUTPUT_DIR/all_test_files.txt"

TOTAL_TEST_FILES=$(wc -l < "$OUTPUT_DIR/all_test_files.txt")
echo "📁 Found $TOTAL_TEST_FILES test files"

# Find files with inline test modules
echo "🔎 Finding files with inline #[cfg(test)] modules..."
grep -r "#\[cfg(test)\]" code/crates --include="*.rs" -l > "$OUTPUT_DIR/inline_test_files.txt" 2>/dev/null || touch "$OUTPUT_DIR/inline_test_files.txt"
INLINE_TEST_FILES=$(wc -l < "$OUTPUT_DIR/inline_test_files.txt")
echo "📝 Found $INLINE_TEST_FILES files with inline test modules"

# Check each test file to see if it's imported
echo "🔍 Checking which test files are imported..."
> "$OUTPUT_DIR/orphaned_tests.txt"
> "$OUTPUT_DIR/wired_tests.txt"

while IFS= read -r test_file; do
    # Get the module name (filename without .rs)
    module_name=$(basename "$test_file" .rs)
    parent_dir=$(dirname "$test_file")
    
    # Check if this module is imported in parent mod.rs or lib.rs
    is_imported=false
    
    # Check parent directory's mod.rs
    if [ -f "$parent_dir/mod.rs" ]; then
        if grep -q "mod $module_name" "$parent_dir/mod.rs" 2>/dev/null; then
            is_imported=true
        fi
    fi
    
    # Check parent directory's lib.rs
    if [ -f "$parent_dir/lib.rs" ]; then
        if grep -q "mod $module_name" "$parent_dir/lib.rs" 2>/dev/null; then
            is_imported=true
        fi
    fi
    
    # Check crate root lib.rs
    crate_root=$(echo "$test_file" | sed 's|code/crates/\([^/]*\)/.*|\1|')
    if [ -f "code/crates/$crate_root/src/lib.rs" ]; then
        if grep -q "mod $module_name" "code/crates/$crate_root/src/lib.rs" 2>/dev/null; then
            is_imported=true
        fi
    fi
    
    if [ "$is_imported" = false ]; then
        echo "$test_file" >> "$OUTPUT_DIR/orphaned_tests.txt"
    else
        echo "$test_file" >> "$OUTPUT_DIR/wired_tests.txt"
    fi
done < "$OUTPUT_DIR/all_test_files.txt"

ORPHANED_COUNT=$(wc -l < "$OUTPUT_DIR/orphaned_tests.txt" 2>/dev/null || echo "0")
WIRED_COUNT=$(wc -l < "$OUTPUT_DIR/wired_tests.txt" 2>/dev/null || echo "0")

echo ""
echo "📊 Results:"
echo "  ✅ Wired test files:    $WIRED_COUNT"
echo "  ❌ Orphaned test files: $ORPHANED_COUNT"
echo "  📝 Inline test modules: $INLINE_TEST_FILES"
echo ""

# Generate per-crate breakdown
echo "📦 Per-crate breakdown..."
> "$OUTPUT_DIR/crate_breakdown.txt"

for crate_dir in code/crates/nestgate-*/; do
    crate_name=$(basename "$crate_dir")
    
    # Count tests in this crate
    total_in_crate=$(grep "^$crate_dir" "$OUTPUT_DIR/all_test_files.txt" 2>/dev/null | wc -l)
    orphaned_in_crate=$(grep "^$crate_dir" "$OUTPUT_DIR/orphaned_tests.txt" 2>/dev/null | wc -l)
    wired_in_crate=$(grep "^$crate_dir" "$OUTPUT_DIR/wired_tests.txt" 2>/dev/null | wc -l)
    
    if [ "$total_in_crate" -gt 0 ]; then
        echo "$crate_name: $wired_in_crate wired, $orphaned_in_crate orphaned, $total_in_crate total" >> "$OUTPUT_DIR/crate_breakdown.txt"
    fi
done

echo ""
cat "$OUTPUT_DIR/crate_breakdown.txt"
echo ""

# Count actual test functions
echo "🧪 Counting test functions..."
TOTAL_TEST_FUNCS=$(grep -r "#\[test\]" code/crates --include="*.rs" | wc -l)
TOTAL_TOKIO_TESTS=$(grep -r "#\[tokio::test\]" code/crates --include="*.rs" | wc -l)
TOTAL_ALL_TESTS=$((TOTAL_TEST_FUNCS + TOTAL_TOKIO_TESTS))

echo "  Total #[test] functions:        $TOTAL_TEST_FUNCS"
echo "  Total #[tokio::test] functions: $TOTAL_TOKIO_TESTS"
echo "  Combined total:                 $TOTAL_ALL_TESTS"
echo ""

# Generate summary report
cat > "$OUTPUT_DIR/SUMMARY.txt" << EOF
# Test Wiring Audit Summary
Generated: $(date)

## Overview
- Total test files found: $TOTAL_TEST_FILES
- Files with inline tests: $INLINE_TEST_FILES
- Wired test files: $WIRED_COUNT
- Orphaned test files: $ORPHANED_COUNT
- Orphan rate: $(awk "BEGIN {printf \"%.1f%%\", ($ORPHANED_COUNT / $TOTAL_TEST_FILES) * 100}")

## Test Function Counts
- #[test] functions: $TOTAL_TEST_FUNCS
- #[tokio::test] functions: $TOTAL_TOKIO_TESTS
- Total test functions: $TOTAL_ALL_TESTS

## Per-Crate Breakdown
$(cat "$OUTPUT_DIR/crate_breakdown.txt")

## Next Steps
1. Review orphaned_tests.txt for files to wire up
2. Start with highest-impact crates (see crate_breakdown.txt)
3. Use wire_up_test.sh to systematically add module imports
4. Verify compilation and test execution after each crate

## Files Generated
- all_test_files.txt: Complete list of test files
- inline_test_files.txt: Files with #[cfg(test)] blocks
- orphaned_tests.txt: Test files not imported
- wired_tests.txt: Test files properly imported
- crate_breakdown.txt: Per-crate statistics
- SUMMARY.txt: This file
EOF

echo "✅ Audit complete!"
echo "📂 Results saved to: $OUTPUT_DIR/"
echo ""
echo "📋 Summary:"
cat "$OUTPUT_DIR/SUMMARY.txt"

