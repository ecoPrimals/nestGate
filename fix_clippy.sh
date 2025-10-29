#!/bin/bash
# Script to systematically fix double_must_use clippy errors
# These occur when #[must_use] is on a function returning Result<>, which is already must_use

echo "Finding all #[must_use] attributes on Result-returning functions..."

# Find all must_use on Result functions
grep -rn "#\[must_use\]" code/crates --include="*.rs" -A2 | \
    grep -B1 "Result<" | \
    grep must_use | \
    cut -d: -f1-2 | \
    sort -u > /tmp/must_use_results.txt

echo "Found $(wc -l < /tmp/must_use_results.txt) instances"
echo ""
echo "Files with must_use on Result functions:"
cat /tmp/must_use_results.txt

echo ""
echo "To fix manually, remove #[must_use] from these functions"
echo "Result<> is already marked as must_use, so the attribute is redundant"

