#!/bin/bash
# Auto-add missing test module references to mod.rs files
# Part of Test Modernization Plan - Oct 28, 2025
# ⚠️  RUN WITH CAUTION - Make sure you have a clean git tree first!

set -e

echo "========================================"
echo "  AUTO-ADD TEST MODULES"
echo "  Adding test modules to mod.rs files"
echo "========================================"
echo ""

# Check for clean git state
if ! git diff --quiet 2>/dev/null; then
    echo "⚠️  WARNING: You have uncommitted changes!"
    echo "    Commit or stash changes before running this script"
    echo ""
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

added_count=0

for crate_dir in code/crates/*/; do
    crate_name=$(basename "$crate_dir")
    echo "=== Processing: $crate_name ==="
    
    # Find all test files
    find "$crate_dir/src" -name "*test*.rs" -type f 2>/dev/null | while read test_file; do
        test_name=$(basename "$test_file" .rs)
        parent_dir=$(dirname "$test_file")
        mod_file="$parent_dir/mod.rs"
        
        # Skip lib.rs and main.rs
        if [ "$test_name" == "lib" ] || [ "$test_name" == "main" ]; then
            continue
        fi
        
        # Check if mod.rs exists and test is not referenced
        if [ -f "$mod_file" ]; then
            if ! grep -q "mod $test_name" "$mod_file"; then
                echo "  ✅ Adding: $test_name to $(realpath --relative-to=. "$mod_file")"
                
                # Add test module reference
                echo "" >> "$mod_file"
                echo "#[cfg(test)]" >> "$mod_file"
                echo "mod $test_name;" >> "$mod_file"
                
                ((added_count++)) || true
            fi
        fi
    done
done

echo ""
echo "========================================"
echo "  COMPLETE"
echo "========================================"
echo "Added $added_count test module references"
echo ""
echo "Next steps:"
echo "  1. Run: cargo fmt --all"
echo "  2. Run: cargo test --workspace --lib"
echo "  3. Fix any compilation errors"
echo "  4. Commit changes"
echo "========================================"

