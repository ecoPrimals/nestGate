#!/usr/bin/env bash
# Find Duplicate Config Struct Definitions
# Helps identify fragmented configuration structures

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$REPO_ROOT"

echo "🔍 Finding Duplicate Config Struct Definitions..."
echo ""

# Find all Config struct definitions
echo "📊 Analyzing Config struct patterns..."
echo ""

# Find by pattern
echo "=== NetworkConfig Definitions ==="
rg "pub struct.*NetworkConfig" --type rust -n | head -20
echo ""

echo "=== StorageConfig Definitions ==="
rg "pub struct.*StorageConfig" --type rust -n | head -20
echo ""

echo "=== SecurityConfig Definitions ==="
rg "pub struct.*SecurityConfig" --type rust -n | head -20
echo ""

echo "=== ApiConfig Definitions ==="
rg "pub struct.*ApiConfig" --type rust -n | head -20
echo ""

echo "=== PerformanceConfig Definitions ==="
rg "pub struct.*PerformanceConfig" --type rust -n | head -20
echo ""

# Count total Config structs
echo "📈 Total Config Struct Count:"
rg "pub struct.*Config" --type rust code/crates/nestgate-core/src | wc -l
echo ""

# Generate detailed report
OUTPUT="config-duplication-report.txt"
echo "📝 Generating detailed report: $OUTPUT"

{
    echo "Config Duplication Report"
    echo "Generated: $(date)"
    echo "========================================"
    echo ""
    
    echo "NetworkConfig occurrences:"
    rg "pub struct.*NetworkConfig" --type rust -n || echo "  None found"
    echo ""
    
    echo "StorageConfig occurrences:"
    rg "pub struct.*StorageConfig" --type rust -n || echo "  None found"
    echo ""
    
    echo "SecurityConfig occurrences:"
    rg "pub struct.*SecurityConfig" --type rust -n || echo "  None found"
    echo ""
    
    echo "All Config structs by file:"
    rg "pub struct.*Config" --type rust -n --stats || echo "  None found"
    
} > "$OUTPUT"

echo "✅ Report saved to: $OUTPUT"
echo ""
echo "🎯 Next Steps:"
echo "   1. Review the report to identify duplicates"
echo "   2. Choose THE canonical config system"
echo "   3. Create migration plan for each duplicate"
echo "   4. Run migrate-to-canonical.sh after decisions" 