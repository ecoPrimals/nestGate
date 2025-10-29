#!/bin/bash
# Quick Build Fix Script - Add missing mod declarations
# Run from repository root

echo "🔧 Adding missing 'mod common;' declarations to test files..."

# List of files that need mod common; added
files_needing_common=(
    "tests/api_security_comprehensive.rs"
    "tests/canonical_test_framework.rs"
    "tests/e2e_comprehensive_workflows.rs"
    "tests/sovereignty_chaos_testing.rs"
    "tests/universal_architecture_validation.rs"
)

for file in "${files_needing_common[@]}"; do
    if [ -f "$file" ]; then
        # Check if file already has 'mod common;'
        if ! grep -q "^mod common;" "$file"; then
            echo "  Adding mod common; to $file"
            # Add mod common; after initial comments/doc comments
            sed -i '1a mod common;' "$file"
        else
            echo "  ✓ $file already has mod common;"
        fi
    else
        echo "  ⚠️  File not found: $file"
    fi
done

echo ""
echo "✅ Mod declarations added!"
echo "Run 'cargo build --workspace --tests' to verify fixes"

