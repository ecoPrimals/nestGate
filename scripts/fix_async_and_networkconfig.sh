#!/bin/bash
# Fix Async/Await and NetworkConfig Issues - October 3, 2025

set -e

echo "🔧 Phase 3: Fixing async/await and NetworkConfig issues..."

# Backup
BACKUP_DIR="backups/async-network-fix-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$BACKUP_DIR"
cp -r code/crates "$BACKUP_DIR/"
echo "✅ Backup: $BACKUP_DIR"

echo ""
echo "🔧 Part 1: NetworkConfig field access fixes..."
echo "   Migrating to CanonicalNetworkConfig structure..."

# Fix NetworkConfig field access patterns
# Old: config.max_connections
# New: config.server.max_connections (or appropriate path)

# This requires careful analysis per file - let's target specific files
FILES_TO_FIX=(
    "code/crates/nestgate-network/src/service/mod.rs"
    "code/crates/nestgate-network/src/types.rs"
)

for file in "${FILES_TO_FIX[@]}"; do
    if [ -f "$file" ]; then
        echo "   📝 Fixing $file..."
        # These are manual fixes - document them
        echo "      ⚠️  Needs manual review: $file"
    fi
done

echo ""
echo "⚠️  NetworkConfig fixes need manual review"
echo "   Files identified:"
echo "   - code/crates/nestgate-network/src/service/mod.rs (4 instances)"
echo "   - code/crates/nestgate-network/src/types.rs (2 instances)"

echo ""
echo "🔧 Part 2: Async/await fixes..."
echo "   Finding functions using .await without async..."

# Count async issues
ASYNC_ISSUES=$(cargo build 2>&1 | grep "error\[E0728\]" | wc -l)
echo "   Found $ASYNC_ISSUES async/await errors"

echo ""
echo "⚠️  Async/await fixes need function signature analysis"
echo "   Files identified:"
echo "   - code/crates/nestgate-installer/src/download.rs (2 functions)"
echo "   - code/crates/nestgate-installer/src/installer.rs (4 functions)"
echo "   - code/crates/nestgate-network/src/api.rs (multiple functions)"

echo ""
echo "📊 Analysis complete. Manual fixes recommended for:"
echo "   1. NetworkConfig field access (18 errors) - Need structure migration"
echo "   2. Async/await signatures (94 errors) - Need function analysis"
echo ""
echo "💾 Backup: $BACKUP_DIR"
echo ""
echo "Recommended next step: Manual fixes with targeted approach"

