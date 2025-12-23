#!/usr/bin/env bash
# Reorganize root documentation - December 16, 2025
# Clean root directory by moving session docs to archive and detailed docs to docs/

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

echo "🧹 Root Documentation Reorganization"
echo "======================================"
echo ""

# Step 1: Archive December 16 session documents
echo "📦 Step 1: Archiving session documents..."
mkdir -p archive/dec16_session

SESSION_DOCS=(
    "AUDIT_EXECUTIVE_SUMMARY_DEC_16_2025.md"
    "AUDIT_QUICK_REFERENCE_DEC_16_2025.md"
    "COMPREHENSIVE_AUDIT_DEC_16_2025_FINAL.md"
    "COMPREHENSIVE_AUDIT_REPORT_DEC_16_2025.md"
    "COVERAGE_BASELINE_DEC_16_2025.md"
    "DEEP_EVOLUTION_PROGRESS_DEC_16_2025.md"
    "EVOLUTION_EXECUTION_PLAN_DEC_16_2025.md"
    "EVOLUTION_MILESTONE_DEC_16_2025.md"
    "EVOLUTION_PROGRESS_DEC_16_2025_CONTINUED.md"
    "EVOLUTION_PROGRESS_DEC_16_2025.md"
    "FINAL_AUDIT_HANDOFF_DEC_16_2025.md"
    "FINAL_SESSION_SUMMARY_DEC_16_2025.md"
    "NEXT_STEPS_DEC_16_2025.md"
    "SESSION_ACHIEVEMENTS_DEC_16_2025.md"
    "SESSION_COMPLETE_DEC_16_2025_FINAL.md"
    "SESSION_COMPLETE_DEC_16_2025.md"
    "SESSION_HANDOFF_DEC_16_2025_FINAL.md"
    "SESSION_SUMMARY_DEC_16_2025.md"
    "ROOT_DOCS_CLEANED_DEC_16.md"
    "ROOT_DOCS_INDEX.md"
    "ROOT_DOCS_UPDATED_DEC_16_2025.md"
    "ROOT_DOCS_ANALYSIS_DEC_16.md"
)

for doc in "${SESSION_DOCS[@]}"; do
    if [[ -f "$doc" ]]; then
        echo "  → $doc → archive/dec16_session/"
        mv "$doc" archive/dec16_session/
    fi
done

echo "✅ Archived ${#SESSION_DOCS[@]} session documents"
echo ""

# Step 2: Move architecture docs to docs/architecture/
echo "📚 Step 2: Moving architecture documentation..."
mkdir -p docs/architecture

ARCH_DOCS=(
    "PRIMAL_SOVEREIGNTY_VERIFIED.md:docs/architecture/PRIMAL_SOVEREIGNTY.md"
    "UNIVERSAL_AGNOSTIC_STORAGE_DESIGN.md:docs/architecture/UNIVERSAL_STORAGE_DESIGN.md"
    "UNIVERSAL_STORAGE_IMPLEMENTATION_COMPLETE.md:docs/architecture/UNIVERSAL_STORAGE_STATUS.md"
    "VENDOR_AGNOSTIC_INFRASTRUCTURE_PLAN.md:docs/architecture/VENDOR_AGNOSTIC_PLAN.md"
)

for mapping in "${ARCH_DOCS[@]}"; do
    src="${mapping%%:*}"
    dst="${mapping##*:}"
    if [[ -f "$src" ]]; then
        echo "  → $src → $dst"
        mv "$src" "$dst"
    fi
done

echo "✅ Moved 4 architecture documents"
echo ""

# Step 3: Move development guides to docs/development/
echo "🔧 Step 3: Moving development guides..."
mkdir -p docs/development

if [[ -f "ERROR_HANDLING_STRATEGY.md" ]]; then
    echo "  → ERROR_HANDLING_STRATEGY.md → docs/development/"
    mv ERROR_HANDLING_STRATEGY.md docs/development/
fi

echo "✅ Moved development guides"
echo ""

# Step 4: Move migration guides to docs/migration/
echo "🚀 Step 4: Moving migration guides..."
mkdir -p docs/migration

MIGRATION_DOCS=(
    "HARDCODING_MIGRATION_PROGRESS.md:docs/migration/HARDCODING_MIGRATION.md"
    "MIGRATION_GUIDE_CAPABILITY_DISCOVERY.md:docs/migration/CAPABILITY_DISCOVERY.md"
    "UNWRAP_MIGRATION_GUIDE.md:docs/migration/UNWRAP_MIGRATION.md"
)

for mapping in "${MIGRATION_DOCS[@]}"; do
    src="${mapping%%:*}"
    dst="${mapping##*:}"
    if [[ -f "$src" ]]; then
        echo "  → $src → $dst"
        mv "$src" "$dst"
    fi
done

echo "✅ Moved 3 migration guides"
echo ""

# Step 5: Move testing docs to docs/testing/
echo "🧪 Step 5: Moving testing documentation..."
mkdir -p docs/testing

if [[ -f "ERROR_PATH_TESTS_NETWORK_CONFIG.md" ]]; then
    echo "  → ERROR_PATH_TESTS_NETWORK_CONFIG.md → docs/testing/ERROR_PATH_TESTS.md"
    mv ERROR_PATH_TESTS_NETWORK_CONFIG.md docs/testing/ERROR_PATH_TESTS.md
fi

echo "✅ Moved testing documentation"
echo ""

# Step 6: Move general docs
echo "📖 Step 6: Moving general documentation..."

GENERAL_DOCS=(
    "CHEAT_SHEET.md:docs/CHEAT_SHEET.md"
    "DOCUMENTATION_MAP.md:docs/DOCUMENTATION_MAP.md"
    "QUICK_STATUS.md:docs/QUICK_STATUS.md"
)

for mapping in "${GENERAL_DOCS[@]}"; do
    src="${mapping%%:*}"
    dst="${mapping##*:}"
    if [[ -f "$src" ]]; then
        echo "  → $src → $dst"
        mv "$src" "$dst"
    fi
done

echo "✅ Moved general documentation"
echo ""

# Step 7: Consolidate start files
echo "🔗 Step 7: Consolidating start files..."

# Check if duplicates exist and remove them (00_START_HERE.md is canonical)
if [[ -f "00_READ_ME_FIRST.md" ]]; then
    echo "  → Removing 00_READ_ME_FIRST.md (consolidated into 00_START_HERE.md)"
    rm 00_READ_ME_FIRST.md
fi

if [[ -f "START_HERE.md" ]]; then
    echo "  → Removing START_HERE.md (consolidated into 00_START_HERE.md)"
    rm START_HERE.md
fi

echo "✅ Consolidated start files"
echo ""

# Summary
echo "======================================"
echo "✨ Root Documentation Cleanup Complete!"
echo ""
echo "📊 Summary:"
echo "  • Session docs archived: archive/dec16_session/"
echo "  • Architecture docs: docs/architecture/ (4 files)"
echo "  • Development guides: docs/development/ (1 file)"
echo "  • Migration guides: docs/migration/ (3 files)"
echo "  • Testing docs: docs/testing/ (1 file)"
echo "  • General docs: docs/ (3 files)"
echo "  • Duplicates removed: 2 files"
echo ""
echo "📁 Clean root directory with 10-12 essential docs"
echo "🎯 All detailed documentation in logical subdirectories"
echo ""

# Show current root docs
echo "Current root documentation files:"
ls -1 *.md 2>/dev/null | head -15 || echo "  (none)"
echo ""
echo "✅ Reorganization complete!"

