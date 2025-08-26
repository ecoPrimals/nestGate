#!/bin/bash

# Fix RwLock Panics Script
# Systematically replaces panic! calls in RwLock handling with proper error handling

set -e

echo "🔧 FIXING RWLOCK PANICS IN ZERO COST SERVICE EXAMPLES"
echo "====================================================="

FILE="code/crates/nestgate-core/src/services/zero_cost_service_examples.rs"

# Create backup
cp "$FILE" "${FILE}.backup"

echo "📁 Processing: $FILE"

# Replace all RwLock panic patterns with proper error handling
sed -i 's/panic!("Critical concurrency error - RwLock poisoned: {e:?}")/return Err(NestGateError::system_error("RwLock poisoned - concurrent access error", Some("zero_cost_service_examples")))/g' "$FILE"

# Replace unwrap_or_else patterns with proper match statements
# This is more complex and would need manual review

echo "✅ Basic panic patterns replaced"
echo "⚠️  Manual review needed for unwrap_or_else patterns"
echo "📋 Backup saved as: ${FILE}.backup"

echo ""
echo "🧪 TESTING CHANGES..."
cd "$(dirname "$0")/.."
cargo check --package nestgate-core

if [ $? -eq 0 ]; then
    echo "✅ Changes compile successfully!"
else
    echo "❌ Compilation failed - review needed"
    echo "🔄 Restore backup with: mv ${FILE}.backup $FILE"
fi 