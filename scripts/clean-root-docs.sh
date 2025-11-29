#!/bin/bash
# Clean and organize root documentation

set -e

echo "╔════════════════════════════════════════════════════════╗"
echo "║     CLEANING ROOT DOCUMENTATION                        ║"
echo "╚════════════════════════════════════════════════════════╝"
echo ""

cd "$(dirname "$0")/.."

# Create archive directories
mkdir -p archive/nov-2025-sessions
mkdir -p archive/old-progress-docs

echo "=== Archiving Old November Files ==="
count=0
for file in *NOV_*.md *NOV_*.txt; do
    if [ -f "$file" ] && [ "$file" != "00_CURRENT_STATUS_NOV_27.md" ]; then
        echo "  Archiving: $file"
        mv "$file" archive/nov-2025-sessions/
        count=$((count + 1))
    fi
done
echo "  Archived: $count files"
echo ""

echo "=== Archiving Old Progress Files ==="
count=0
for pattern in "MONTH_*.md" "DAY_*.md" "WEEK_*.md" "EXECUTION_COMPLETE*.md" "SESSION_COMPLETE_REALISTIC*.md" "SESSION_COMPLETE_NOV*.md"; do
    for file in $pattern; do
        if [ -f "$file" ]; then
            echo "  Archiving: $file"
            mv "$file" archive/old-progress-docs/
            count=$((count + 1))
        fi
    done
done
echo "  Archived: $count files"
echo ""

echo "=== Root Documentation Structure ==="
echo ""
echo "🎯 START HERE:"
ls -1 00_START_HERE.md README.md README_AUDIT_COMPLETE.txt 2>/dev/null || true
echo ""
echo "📦 AUDIT DELIVERABLES (Dec 2025):"
ls -1 SESSION_COMPLETE_DEC_2025.txt FINAL_HANDOFF.txt FINAL_AUDIT_SUMMARY.md COMPREHENSIVE_AUDIT_REPORT_DEC_2025.md 2>/dev/null || true
echo ""
echo "🛠️  MIGRATION PLANS:"
ls -1 *MIGRATION_PLAN.md 2>/dev/null || true
echo ""
echo "📋 INDEXES:"
ls -1 DELIVERABLES_MANIFEST.md AUDIT_COMPLETE_INDEX.txt 2>/dev/null || true
echo ""

echo "✅ Root documentation cleaned and organized!"
echo ""
echo "Next: cat 00_START_HERE.md"

