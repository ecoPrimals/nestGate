#!/bin/bash
# Check for unwrap usage in production code (excluding tests)

echo "🔍 Checking for unwrap usage in production code..."

UNWRAP_COUNT=$(find code/crates/*/src -name "*.rs" -not -path "*/tests/*" -exec grep -l "\.unwrap()" {} \; | wc -l)
EXPECT_COUNT=$(find code/crates/*/src -name "*.rs" -not -path "*/tests/*" -exec grep -l "\.expect(" {} \; | wc -l)

echo "Production files with .unwrap(): $UNWRAP_COUNT"
echo "Production files with .expect(): $EXPECT_COUNT"

if [ $UNWRAP_COUNT -gt 0 ] || [ $EXPECT_COUNT -gt 0 ]; then
    echo "⚠️  Found unwrap/expect usage in production code"
    echo "Consider using proper error handling instead"
    exit 1
else
    echo "✅ No unwrap/expect found in production code"
    exit 0
fi
