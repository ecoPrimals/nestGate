#!/usr/bin/env bash
# Find Duplicate Trait Definitions
# Identifies trait fragmentation and suggests consolidation

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$REPO_ROOT"

echo "🔍 Finding Trait Definitions..."
echo ""

OUTPUT="trait-analysis-report.txt"

{
    echo "Trait Analysis Report"
    echo "Generated: $(date)"
    echo "========================================"
    echo ""
    
    echo "=== Total Trait Files ==="
    trait_count=$(find code/crates -name "*.rs" -exec grep -l "pub trait" {} \; | wc -l)
    echo "Files containing trait definitions: $trait_count"
    echo ""
    
    echo "=== Service-Related Traits ==="
    rg "pub trait.*Service" --type rust -n | head -30
    echo ""
    
    echo "=== Provider-Related Traits ==="
    rg "pub trait.*Provider" --type rust -n | head -30
    echo ""
    
    echo "=== Storage-Related Traits ==="
    rg "pub trait.*Storage" --type rust -n | head -30
    echo ""
    
    echo "=== Config-Related Traits ==="
    rg "pub trait.*Config" --type rust -n | head -30
    echo ""
    
    echo "=== Canonical Trait System ==="
    echo "Files in traits/ directory:"
    find code/crates/nestgate-core/src/traits -name "*.rs" -type f
    echo ""
    
    echo "=== Recommendation ==="
    echo "Traits should extend from canonical system:"
    echo "  - Services: CanonicalService (traits/canonical_unified_traits.rs)"
    echo "  - Storage: UnifiedStorage (traits/unified_storage.rs)"
    echo "  - Providers: CanonicalProvider (traits/canonical_unified_traits.rs)"
    echo ""
    
} | tee "$OUTPUT"

echo "✅ Report saved to: $OUTPUT"
echo ""
echo "🎯 Next Steps:"
echo "   1. Audit each trait to determine if it should inherit from canonical"
echo "   2. Look for semantic duplicates (e.g., multiple health check traits)"
echo "   3. Consolidate into domain extensions of canonical traits" 