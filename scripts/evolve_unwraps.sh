#!/bin/bash
# Unwrap Evolution Script - Migrate unwraps to modern error handling
# Usage: ./scripts/evolve_unwraps.sh --module <module_path> --count <number>

set -e

MODULE=""
COUNT=20
DRY_RUN=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --module)
            MODULE="$2"
            shift 2
            ;;
        --count)
            COUNT="$2"
            shift 2
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

if [ -z "$MODULE" ]; then
    echo "Usage: $0 --module <module_path> [--count <number>] [--dry-run]"
    echo "Example: $0 --module code/crates/nestgate-core/src/network/client --count 30"
    exit 1
fi

echo "🔍 Analyzing unwraps in $MODULE..."

# Find unwraps in production code (exclude tests)
unwrap_files=$(find "$MODULE" -name "*.rs" ! -name "*test*.rs" ! -path "*/tests/*" -exec grep -l "\.unwrap()" {} \;)

if [ -z "$unwrap_files" ]; then
    echo "✅ No unwraps found in production code!"
    exit 0
fi

echo "📊 Found unwraps in:"
echo "$unwrap_files"

echo ""
echo "🎯 Evolution Strategy:"
echo "1. Replace .unwrap() with proper ? operator"
echo "2. Add context with .context() or .with_context()"
echo "3. Update function signatures to return Result<>"
echo ""

if [ "$DRY_RUN" = true ]; then
    echo "🏃 DRY RUN - No files will be modified"
    echo ""
    echo "Would evolve $COUNT unwraps in:"
    echo "$unwrap_files" | head -n 5
    exit 0
fi

echo "⚠️  This will modify files. Make sure you have committed your changes."
read -p "Continue? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Aborted."
    exit 1
fi

echo "🚀 Evolution in progress..."
echo "See UNWRAP_EVOLUTION_EXAMPLES.md for patterns"

# TODO: Implement automated refactoring
# For now, this is a tracking/analysis tool
# Manual evolution following documented patterns

echo "✅ Analysis complete. Ready for manual evolution."
echo "Follow patterns in docs/migration/UNWRAP_MIGRATION.md"

