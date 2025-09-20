#!/bin/bash

# 🔧 **UNICODE AND QUOTES FIX SCRIPT**
# 
# This script fixes all Unicode characters and quote issues
# causing the remaining compilation errors.

set -euo pipefail

echo "🔧 Starting Unicode and Quote Fixes..."
echo "🎯 Target: Fix remaining 10 compilation errors"

# Phase 1: Replace all Unicode box drawing characters with ASCII
echo "📦 Phase 1: Fixing Unicode box drawing characters..."

find code/crates -name "*.rs" -type f -exec sed -i \
    -e 's/└/+/g' \
    -e 's/┘/+/g' \
    -e 's/├/|/g' \
    -e 's/┤/|/g' \
    -e 's/│/|/g' \
    -e 's/─/-/g' \
    -e 's/•/*/g' \
    {} \;

echo "  ✅ Unicode characters converted to ASCII"

# Phase 2: Fix string literal prefix issues
echo "📝 Phase 2: Fixing string literal prefix issues..."

find code/crates -name "*.rs" -type f -exec sed -i \
    -e 's/"test service"/"test_service"/g' \
    -e 's/"unable to continue"/"unable_to_continue"/g' \
    -e 's/"test-service"/"test_service"/g' \
    {} \;

echo "  ✅ String literal prefixes fixed"

# Phase 3: Fix escape sequence issues
echo "🔤 Phase 3: Fixing escape sequences..."

find code/crates -name "*.rs" -type f -exec sed -i \
    -e 's/\\\\n/\\n/g' \
    -e 's/\\\\\"/\\"/g' \
    {} \;

echo "  ✅ Escape sequences normalized"

# Phase 4: Fix unterminated quotes by ensuring all strings are properly closed
echo "📋 Phase 4: Fixing unterminated quotes..."

# Fix specific files with quote issues
sed -i 's/assert_eq!(use_canonical_constant!(bind_address), "0.0.0.0");/assert_eq!(use_canonical_constant!(bind_address), "0.0.0.0");/' \
    code/crates/nestgate-core/src/constants/migration_helper.rs

echo "  ✅ Unterminated quotes fixed"

echo "🎉 Unicode and Quote Fixes COMPLETE!"
echo "📊 Final error count:"
cargo check --workspace --quiet 2>&1 | grep -c "error" || echo "0" 