#!/usr/bin/env bash
# Find Duplicate Constant Definitions
# Identifies repeated constants across the codebase

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$REPO_ROOT"

echo "🔍 Finding Duplicate Constant Definitions..."
echo ""

# Common duplicated constants
DUPLICATES=(
    "MODULE_VERSION"
    "DEFAULT_TIMEOUT_MS"
    "DEFAULT_BUFFER_SIZE"
    "DEFAULT_MAX_CONNECTIONS"
    "DEFAULT_API_PORT"
    "DEFAULT_HTTP_PORT"
    "LOCALHOST"
)

OUTPUT="constant-duplication-report.txt"

{
    echo "Constant Duplication Report"
    echo "Generated: $(date)"
    echo "========================================"
    echo ""
    
    for const_name in "${DUPLICATES[@]}"; do
        echo "=== $const_name ==="
        count=$(rg "pub const $const_name" --type rust | wc -l)
        echo "Occurrences: $count"
        
        if [ "$count" -gt 1 ]; then
            echo "⚠️  DUPLICATE FOUND - Locations:"
            rg "pub const $const_name" --type rust -n
        fi
        echo ""
    done
    
    echo "=== All Duplicate Patterns ==="
    echo "Finding constants defined in 3+ files:"
    rg "pub const [A-Z_]+" --type rust -o | sort | uniq -c | sort -rn | awk '$1 >= 3'
    
} | tee "$OUTPUT"

echo ""
echo "✅ Report saved to: $OUTPUT"
echo ""
echo "🎯 Recommendation:"
echo "   Create code/crates/nestgate-core/src/constants/shared.rs"
echo "   Move duplicate constants there"
echo "   Update all references to use shared module" 