#!/bin/bash
# Find test modules that aren't referenced in mod.rs files
# Part of Test Modernization Plan - Oct 28, 2025

set -e

echo "========================================"
echo "  MISSING TEST MODULE FINDER"
echo "  Finding tests not in module tree"
echo "========================================"
echo ""

total_missing=0

for crate_dir in code/crates/*/; do
    crate_name=$(basename "$crate_dir")
    echo "=== Analyzing: $crate_name ==="
    crate_missing=0
    
    # Find all test files
    find "$crate_dir/src" -name "*test*.rs" -type f 2>/dev/null | while read test_file; do
        test_name=$(basename "$test_file" .rs)
        parent_dir=$(dirname "$test_file")
        mod_file="$parent_dir/mod.rs"
        
        # Check if referenced in mod.rs
        if [ -f "$mod_file" ]; then
            # Look for "mod test_name;" with optional #[cfg(test)]
            if ! grep -q "mod $test_name" "$mod_file"; then
                echo "  ❌ NOT REFERENCED: $(realpath --relative-to=. "$test_file")"
                echo "     → Add to: $(realpath --relative-to=. "$mod_file")"
                echo "     → Line: #[cfg(test)] mod $test_name;"
                echo ""
                ((crate_missing++)) || true
                ((total_missing++)) || true
            fi
        else
            # No mod.rs exists in parent directory
            if [ "$test_name" != "lib" ] && [ "$test_name" != "main" ]; then
                echo "  ⚠️  NO MOD.RS in: $(realpath --relative-to=. "$parent_dir")"
                echo "     File: $(realpath --relative-to=. "$test_file")"
                echo ""
            fi
        fi
    done
    
    if [ $crate_missing -eq 0 ]; then
        echo "  ✅ All test modules referenced"
    fi
    echo ""
done

echo "========================================"
echo "  SUMMARY"
echo "========================================"
echo "Total test files NOT referenced: $total_missing"
echo ""
echo "Estimated tests not running: $((total_missing * 20)) - $((total_missing * 50))"
echo "(assuming 20-50 tests per file)"
echo ""
echo "Next step: Run scripts/auto_add_test_modules.sh to fix"
echo "========================================"

