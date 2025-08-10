#!/bin/bash

echo "🔧 Fixing duplicated function signature syntax errors..."

# Fix the pattern: ") -> Result<(), Box<dyn std::error::Error>> {) -> Result<(), Box<dyn std::error::Error>> {) {"
# Should be: ") -> Result<(), Box<dyn std::error::Error>> {"

find code/ -name "*.rs" -type f -exec sed -i 's/) -> Result<(), Box<dyn std::error::Error>> {) -> Result<(), Box<dyn std::error::Error>> {) {/) -> Result<(), Box<dyn std::error::Error>> {/g' {} \;

echo "✅ Fixed duplicated function signatures"

# Also fix any similar patterns with other return types
find code/ -name "*.rs" -type f -exec sed -i 's/) -> Result<(), Box<dyn std::error::Error>> {) /) -> Result<(), Box<dyn std::error::Error>> {/g' {} \;

echo "✅ Fixed additional syntax patterns"

# Check for any remaining issues
echo "🔍 Checking for remaining syntax issues..."
if grep -r ") -> Result<(), Box<dyn std::error::Error>> {) " code/ --include="*.rs"; then
    echo "⚠️  Found remaining issues - may need manual review"
else
    echo "✅ All duplicated function signatures appear to be fixed"
fi 