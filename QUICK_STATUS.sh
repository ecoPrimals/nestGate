#!/bin/bash
# Quick status check for NestGate compilation progress

echo "=== NESTGATE COMPILATION STATUS ==="
echo ""

ERROR_COUNT=$(cargo build --lib --package nestgate-core 2>&1 | grep -c "^error" || echo "0")
echo "Current Errors: $ERROR_COUNT / 113 original"
echo "Progress: $((100 - ERROR_COUNT * 100 / 113))% fixed"
echo ""

echo "Error breakdown:"
cargo build --lib --package nestgate-core 2>&1 | grep "^error\[" | sort | uniq -c | head -10
