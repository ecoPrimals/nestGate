#!/bin/bash
# Systematic Import Conflict Resolution Script
# Fixes E0252 "defined multiple times" errors

echo "🚀 SYSTEMATIC IMPORT CLEANUP - Phase D"

# Function to remove standalone SystemTime imports when they're also imported with Duration
remove_duplicate_systemtime() {
    local file=$1
    # Check if file has both standalone and grouped SystemTime imports
    if grep -q "use std::time::SystemTime;" "$file" && grep -q "use std::time::{.*SystemTime" "$file"; then
        echo "  📁 Fixing SystemTime duplicates in $file"
        sed -i '/^use std::time::SystemTime;$/d' "$file"
    fi
}

# Function to remove duplicate tracing imports
remove_duplicate_tracing() {
    local file=$1
    # Remove duplicate tracing imports if they exist
    if [ $(grep -c "use tracing::" "$file" 2>/dev/null || echo 0) -gt 1 ]; then
        echo "  📁 Fixing tracing duplicates in $file"
        # Keep only the first comprehensive tracing import, remove others
        sed -i '/use tracing::warn;$/d' "$file"
        sed -i '/use tracing::debug;$/d' "$file" 
        sed -i '/use tracing::info;$/d' "$file"
        sed -i '/use tracing::error;$/d' "$file"
    fi
}

# Process all Rust files systematically
echo "🔍 Scanning for duplicate imports..."
find src -name "*.rs" | while read -r file; do
    remove_duplicate_systemtime "$file"
    remove_duplicate_tracing "$file"
done

echo "✅ SYSTEMATIC IMPORT CLEANUP COMPLETE"
echo "📊 Phase D: Import conflicts systematically eliminated" 