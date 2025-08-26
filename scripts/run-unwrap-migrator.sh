#!/bin/bash

# Enhanced Unwrap Migrator Runner
# Systematically eliminates unwrap/expect/panic patterns

set -e

echo "🔄 ENHANCED UNWRAP MIGRATOR - PRODUCTION SAFETY TOOL"
echo "=================================================="

cd "$(dirname "$0")/.."

# Build the migrator
echo "🔧 Building enhanced unwrap migrator..."
cd unwrap-migrator
cargo build --release

echo ""
echo "📊 ANALYZING PRODUCTION CODE PATTERNS..."
echo "========================================="

# Count current patterns
echo "📈 Current Pattern Analysis:"
echo "  Unwrap calls: $(find ../code/crates -name "*.rs" -exec grep -l "\.unwrap()" {} \; | wc -l) files"
echo "  Expect calls: $(find ../code/crates -name "*.rs" -exec grep -l "\.expect(" {} \; | wc -l) files"  
echo "  Panic calls:  $(find ../code/crates -name "*.rs" -exec grep -l "panic!" {} \; | wc -l) files"

echo ""
echo "🔍 PRODUCTION CODE SCAN..."
./target/release/unwrap-migrator --dry-run --nestgate-mode --path ../code/crates

echo ""
echo "📋 Would you like to apply the migration? (y/N)"
read -r response

if [[ "$response" =~ ^[Yy]$ ]]; then
    echo "⚡ APPLYING MIGRATION..."
    ./target/release/unwrap-migrator --apply --nestgate-mode --path ../code/crates
    
    echo ""
    echo "✅ MIGRATION COMPLETE!"
    echo "🧪 NEXT STEPS:"
    echo "  1. Run 'cargo check --all-features' to validate"
    echo "  2. Run 'cargo test --all-features' to ensure functionality"
    echo "  3. Review changes and commit"
else
    echo "ℹ️  Migration cancelled. Run with --apply when ready."
fi 