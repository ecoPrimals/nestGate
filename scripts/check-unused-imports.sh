#!/bin/bash
# Helper script to check for unused imports
# Run this manually to identify unused imports

echo "🔍 Checking for unused imports..."
echo "Note: Run 'cargo clippy -- -W unused-imports' for detailed analysis"

# Check for common unused import patterns
find code/crates -name "*.rs" -exec grep -l "use.*;" {} \; | head -10 | while read -r file; do
    echo "Checking $file..."
    # This would need cargo clippy for accurate detection
done

echo "Run: cargo clippy --workspace -- -W unused-imports"
