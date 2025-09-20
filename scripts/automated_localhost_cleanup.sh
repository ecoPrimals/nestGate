#!/bin/bash

# **AUTOMATED LOCALHOST CLEANUP SCRIPT**
# Systematically eliminates remaining hardcoded localhost patterns
# Date: September 12, 2025
# Status: Final cleanup phase

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TIMESTAMP=$(date +%Y%m%d-%H%M%S)
CLEANUP_LOG="$PROJECT_ROOT/localhost_cleanup_$TIMESTAMP.log"

echo "🧹 **AUTOMATED LOCALHOST CLEANUP - FINAL PHASE**" | tee "$CLEANUP_LOG"
echo "Project: $PROJECT_ROOT" | tee -a "$CLEANUP_LOG"
echo "Started: $(date)" | tee -a "$CLEANUP_LOG"
echo "=========================================" | tee -a "$CLEANUP_LOG"

# Function to replace localhost patterns with dynamic resolution
replace_localhost_patterns() {
    local file="$1"
    local backup_file="${file}.backup-$TIMESTAMP"
    
    echo "Processing: $file" | tee -a "$CLEANUP_LOG"
    
    # Create backup
    cp "$file" "$backup_file"
    
    # Replace various localhost patterns
    sed -i \
        -e 's/"http:\/\/localhost:8080"/"crate::service_discovery::resolve_service_endpoint(\"api\").await.unwrap_or_else(|_| crate::constants::canonical_defaults::network::build_api_url())"/g' \
        -e 's/"http:\/\/localhost:8081"/"crate::service_discovery::resolve_service_endpoint(\"admin\").await.unwrap_or_else(|_| crate::constants::canonical_defaults::network::build_admin_url())"/g' \
        -e 's/"http:\/\/localhost:8082"/"crate::service_discovery::resolve_service_endpoint(\"metrics\").await.unwrap_or_else(|_| crate::constants::canonical_defaults::network::build_metrics_url())"/g' \
        -e 's/"ws:\/\/localhost:8080"/"crate::service_discovery::resolve_service_endpoint(\"websocket\").await.unwrap_or_else(|_| crate::constants::canonical_defaults::network::build_websocket_url())"/g' \
        -e 's/"127\.0\.0\.1:8080"/"std::env::var(\"NESTGATE_API_ENDPOINT\").unwrap_or_else(|_| crate::constants::canonical_defaults::network::build_api_url())"/g' \
        "$file"
    
    # Check if changes were made
    if ! diff -q "$file" "$backup_file" > /dev/null; then
        echo "  ✅ Updated: $file" | tee -a "$CLEANUP_LOG"
    else
        echo "  ⚪ No changes: $file" | tee -a "$CLEANUP_LOG"
        rm "$backup_file"  # Remove backup if no changes
    fi
}

# Function to add deprecation warnings to files
add_deprecation_warnings() {
    local file="$1"
    
    # Check if deprecation warning already exists
    if ! grep -q "LOCALHOST HARDCODING DEPRECATION" "$file"; then
        # Add deprecation warning at the top
        temp_file=$(mktemp)
        cat > "$temp_file" << 'EOF'
// **LOCALHOST HARDCODING DEPRECATION NOTICE**
//!
//! ⚠️  DEPRECATION WARNING: This file contains hardcoded localhost patterns
//! that are being migrated to dynamic endpoint resolution.
//!
//! **MIGRATION STATUS**: 🔄 FINAL CLEANUP PHASE
//! **TARGET**: Replace with environment-driven endpoint resolution
//! **TIMELINE**: Immediate migration recommended
//!
//! **MIGRATION PATTERN**:
//! ```rust
//! // OLD: "http://localhost:8080"
//! // NEW: resolve_service_endpoint("api").await.unwrap_or_else(|_| build_api_url())
//! ```

EOF
        cat "$file" >> "$temp_file"
        mv "$temp_file" "$file"
        echo "  📝 Added deprecation warning to: $file" | tee -a "$CLEANUP_LOG"
    fi
}

echo "🔍 **PHASE 1: IDENTIFYING REMAINING PATTERNS**" | tee -a "$CLEANUP_LOG"

# Find all files with localhost patterns
LOCALHOST_FILES=$(find "$PROJECT_ROOT/code" -name "*.rs" -type f -exec grep -l "localhost:808[0-9]" {} \;)

if [ -z "$LOCALHOST_FILES" ]; then
    echo "✅ No localhost patterns found - cleanup complete!" | tee -a "$CLEANUP_LOG"
    exit 0
fi

echo "Found $(echo "$LOCALHOST_FILES" | wc -l) files with localhost patterns:" | tee -a "$CLEANUP_LOG"
echo "$LOCALHOST_FILES" | tee -a "$CLEANUP_LOG"

echo "" | tee -a "$CLEANUP_LOG"
echo "🔧 **PHASE 2: AUTOMATED PATTERN REPLACEMENT**" | tee -a "$CLEANUP_LOG"

# Process each file
while IFS= read -r file; do
    if [ -f "$file" ]; then
        # Skip test files for now (different migration strategy)
        if [[ "$file" == *"test"* ]] || [[ "$file" == *"spec"* ]]; then
            echo "📋 Marking test file for manual review: $file" | tee -a "$CLEANUP_LOG"
            add_deprecation_warnings "$file"
        else
            # Apply automated replacements to production code
            replace_localhost_patterns "$file"
        fi
    fi
done <<< "$LOCALHOST_FILES"

echo "" | tee -a "$CLEANUP_LOG"
echo "🧪 **PHASE 3: VALIDATION**" | tee -a "$CLEANUP_LOG"

# Count remaining patterns
REMAINING_COUNT=$(find "$PROJECT_ROOT/code" -name "*.rs" -type f -exec grep -l "localhost:808[0-9]" {} \; | wc -l)
echo "Remaining hardcoded patterns: $REMAINING_COUNT" | tee -a "$CLEANUP_LOG"

if [ "$REMAINING_COUNT" -eq 0 ]; then
    echo "🎉 **SUCCESS: ALL LOCALHOST PATTERNS ELIMINATED!**" | tee -a "$CLEANUP_LOG"
else
    echo "📋 Remaining patterns (likely in test files):" | tee -a "$CLEANUP_LOG"
    find "$PROJECT_ROOT/code" -name "*.rs" -type f -exec grep -l "localhost:808[0-9]" {} \; | tee -a "$CLEANUP_LOG"
fi

echo "" | tee -a "$CLEANUP_LOG"
echo "🔍 **PHASE 4: ENVIRONMENT VARIABLE VALIDATION**" | tee -a "$CLEANUP_LOG"

# Count files using environment variables
ENV_VAR_COUNT=$(find "$PROJECT_ROOT/code" -name "*.rs" -type f -exec grep -l "std::env::var.*NESTGATE" {} \; | wc -l)
echo "Files using NESTGATE environment variables: $ENV_VAR_COUNT" | tee -a "$CLEANUP_LOG"

# Check for canonical defaults usage
CANONICAL_COUNT=$(find "$PROJECT_ROOT/code" -name "*.rs" -type f -exec grep -l "canonical_defaults::network" {} \; | wc -l)
echo "Files using canonical defaults: $CANONICAL_COUNT" | tee -a "$CLEANUP_LOG"

echo "" | tee -a "$CLEANUP_LOG"
echo "📊 **CLEANUP SUMMARY**" | tee -a "$CLEANUP_LOG"
echo "- Processed files: $(echo "$LOCALHOST_FILES" | wc -l)" | tee -a "$CLEANUP_LOG"
echo "- Remaining patterns: $REMAINING_COUNT" | tee -a "$CLEANUP_LOG"
echo "- Environment variable adoption: $ENV_VAR_COUNT files" | tee -a "$CLEANUP_LOG"
echo "- Canonical defaults usage: $CANONICAL_COUNT files" | tee -a "$CLEANUP_LOG"
echo "- Backup files created: $(find "$PROJECT_ROOT/code" -name "*.backup-$TIMESTAMP" | wc -l)" | tee -a "$CLEANUP_LOG"

echo "" | tee -a "$CLEANUP_LOG"
echo "✅ **CLEANUP COMPLETED**" | tee -a "$CLEANUP_LOG"
echo "Completed: $(date)" | tee -a "$CLEANUP_LOG"
echo "Log saved: $CLEANUP_LOG" | tee -a "$CLEANUP_LOG"

# Test compilation to ensure no syntax errors
echo "" | tee -a "$CLEANUP_LOG"
echo "🧪 **COMPILATION VALIDATION**" | tee -a "$CLEANUP_LOG"

if cd "$PROJECT_ROOT" && cargo check --lib -p nestgate-core --quiet 2>/dev/null; then
    echo "✅ Compilation successful - no syntax errors introduced" | tee -a "$CLEANUP_LOG"
else
    echo "⚠️  Compilation issues detected - manual review needed" | tee -a "$CLEANUP_LOG"
    echo "Run 'cargo check' to see specific errors" | tee -a "$CLEANUP_LOG"
fi

echo ""
echo "🎯 **NEXT STEPS**:"
echo "1. Review backup files if needed: find code -name '*.backup-$TIMESTAMP'"
echo "2. Test the changes: cargo test"
echo "3. Remove backups when satisfied: find code -name '*.backup-$TIMESTAMP' -delete"
echo "4. Update environment variables for production deployment" 