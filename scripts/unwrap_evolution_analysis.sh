#!/bin/bash
# Unwrap Evolution Script
# Systematically migrates .unwrap() and .expect() to proper Result handling

set -euo pipefail

REPO_ROOT="/home/westgate/Development/ecoPrimals/nestgate"
cd "$REPO_ROOT"

echo "🚀 Starting Unwrap Evolution"
echo "================================"

# Phase 1: Find all unwrap/expect usages in production code (exclude tests)
echo ""
echo "📊 Phase 1: Analyzing unwrap/expect usage..."

# Count total unwraps in production code
TOTAL_UNWRAPS=$(rg '\.unwrap\(\)|\.expect\(' code/crates --type rust -c | awk -F: '{sum+=$2} END {print sum}')
echo "Total unwrap/expect in production code: $TOTAL_UNWRAPS"

# Show top offenders
echo ""
echo "🔝 Top 10 files with most unwrap/expect:"
rg '\.unwrap\(\)|\.expect\(' code/crates --type rust -c | sort -t: -k2 -rn | head -10

# Phase 2: Evolution examples
echo ""
echo "📝 Phase 2: Evolution Patterns"
echo "================================"

cat << 'EOF'

## Evolution Pattern 1: Option → Result

❌ OLD (Panic-prone):
    let value = some_option.unwrap();
    let data = map.get(&key).unwrap();

✅ NEW (Safe):
    let value = some_option.ok_or(Error::MissingValue)?;
    let data = map.get(&key).ok_or(Error::KeyNotFound(key.clone()))?;

## Evolution Pattern 2: Result → Contextual Error

❌ OLD (Lost context):
    let file = File::open(path).unwrap();
    let config = toml::from_str(&contents).expect("valid config");

✅ NEW (With context):
    let file = File::open(path)
        .with_context(|| format!("Failed to open file: {}", path.display()))?;
    let config = toml::from_str(&contents)
        .context("Failed to parse configuration")?;

## Evolution Pattern 3: Test Assertions

❌ OLD (Tests, but verbose):
    let result = do_something();
    assert!(result.is_ok());
    let value = result.unwrap();

✅ NEW (Clean):
    let value = do_something()?;
    // Or for tests:
    let value = do_something().expect("test should succeed");

EOF

# Phase 3: Priority files for migration
echo ""
echo "🎯 Phase 3: Priority Files for Migration"
echo "================================"

echo ""
echo "API Handlers (user-facing, highest priority):"
find code/crates/nestgate-api/src/handlers -name "*.rs" ! -name "*_test*.rs" -exec sh -c '
    count=$(rg "\.unwrap\(\)|\.expect\(" "$1" -c 2>/dev/null || echo "0")
    if [ "$count" -gt 0 ]; then
        echo "  $1: $count occurrences"
    fi
' _ {} \; | sort -t: -k2 -rn | head -10

echo ""
echo "Core Services (critical paths):"
find code/crates/nestgate-core/src/services -name "*.rs" ! -name "*_test*.rs" -exec sh -c '
    count=$(rg "\.unwrap\(\)|\.expect\(" "$1" -c 2>/dev/null || echo "0")
    if [ "$count" -gt 0 ]; then
        echo "  $1: $count occurrences"
    fi
' _ {} \; | sort -t: -k2 -rn | head -10

# Phase 4: Automated evolution (safe patterns only)
echo ""
echo "🤖 Phase 4: Automated Safe Evolution"
echo "================================"
echo ""
echo "Would evolve the following safe patterns:"
echo "  1. .unwrap_or_default() → already safe ✅"
echo "  2. .unwrap() in test code → keep or change to expect with message"
echo "  3. .expect(\"msg\") → analyze if context is sufficient"
echo ""
echo "⚠️  Manual review required for:"
echo "  - Production code unwrap()"
echo "  - expect() without clear context"
echo "  - Nested error handling"

# Phase 5: Progress tracking
echo ""
echo "📈 Progress Tracking"
echo "================================"
echo "Baseline: $TOTAL_UNWRAPS unwrap/expect calls in production code"
echo "Target Week 1: $(($TOTAL_UNWRAPS - 500)) (migrate 500)"
echo "Target Week 4: $(($TOTAL_UNWRAPS * 20 / 100)) (80% reduction)"
echo ""
echo "Next Steps:"
echo "1. Review priority files listed above"
echo "2. Apply evolution patterns"
echo "3. Run tests after each batch"
echo "4. Update progress tracking"

echo ""
echo "✅ Analysis Complete!"
echo ""
echo "To start evolution, review files above and apply patterns manually"
echo "or use tools/unwrap-migrator/ for automated migration."
