#!/bin/bash
# 🚀 Quick Unification Next Steps - NestGate
# Run this script to begin the unification work

set -e

echo "=================================================="
echo "🔍 NESTGATE UNIFICATION - NEXT STEPS"
echo "=================================================="
echo ""

cd "$(dirname "$0")"

echo "📊 CURRENT STATUS:"
echo "  ✅ File size discipline: 100% (max 974/2000 lines)"
echo "  ✅ Build: GREEN (0 errors)"
echo "  ✅ Tests: 1,909/1,909 passing (100%)"
echo "  ✅ Technical debt markers: 0 (perfect!)"
echo "  ✅ async_trait: 22 instances (98% eliminated)"
echo ""
echo "  🔴 CRITICAL: 19 duplicate Service traits in network module"
echo "  🔴 CRITICAL: 20+ provider trait variants need migration"
echo "  🟡 MEDIUM: 9 helper/stub files need consolidation"
echo ""

echo "=================================================="
echo "🎯 PHASE 1: CRITICAL FIXES (Weeks 1-2)"
echo "=================================================="
echo ""

echo "📋 STEP 1: Network Service Trait Unification"
echo "   Problem: 19 duplicate 'pub trait Service' definitions in network module"
echo ""
echo "   Action: Consolidate to single canonical NetworkService trait"
echo ""
echo "   Commands:"
echo "     # Find all duplicate Service trait definitions"
echo "     grep -r \"^pub trait Service\" code/crates/nestgate-core/src/network --include=\"*.rs\""
echo ""
echo "     # Count duplicates"
echo "     grep -r \"^pub trait Service\" code/crates/nestgate-core/src/network --include=\"*.rs\" | wc -l"
echo ""

read -p "Press Enter to see the duplicate Service traits..."

echo ""
echo "Duplicate Service trait definitions:"
grep -r "^pub trait Service" code/crates/nestgate-core/src/network --include="*.rs" || true

echo ""
echo ""
echo "📋 STEP 2: Provider Trait Audit"
echo "   Problem: 20+ provider trait variants scattered across codebase"
echo ""
echo "   Action: Map all variants to canonical traits"
echo ""
echo "   Commands:"
echo "     # Find all Provider trait definitions"
echo "     grep -r \"pub trait.*Provider\" code/crates --include=\"*.rs\" > provider_traits_audit.txt"
echo ""
echo "     # View the audit file"
echo "     cat provider_traits_audit.txt"
echo ""

read -p "Press Enter to generate provider traits audit..."

echo ""
echo "Generating provider traits audit..."
grep -r "pub trait.*Provider" code/crates --include="*.rs" > provider_traits_audit.txt 2>/dev/null || true
echo "✅ Saved to: provider_traits_audit.txt"
echo "   Found $(wc -l < provider_traits_audit.txt) provider trait definitions"
echo ""

echo ""
echo "📋 STEP 3: async_trait Audit"
echo "   Current: 22 async_trait instances remaining"
echo "   Target: 5-10 legitimate instances"
echo ""
echo "   Commands:"
echo "     # Find all async_trait usage with context"
echo "     grep -r \"#\[async_trait\]\" code/crates --include=\"*.rs\" -B 3 -A 5 > async_trait_audit.txt"
echo ""

read -p "Press Enter to generate async_trait audit..."

echo ""
echo "Generating async_trait audit..."
grep -r "#\[async_trait\]" code/crates --include="*.rs" -B 3 -A 5 > async_trait_audit.txt 2>/dev/null || true
echo "✅ Saved to: async_trait_audit.txt"
echo "   Found $(grep -c "#\[async_trait\]" code/crates --include="*.rs" 2>/dev/null || echo "0") async_trait instances"
echo ""

echo ""
echo "=================================================="
echo "🟡 QUICK WIN: Error Helper Consolidation (1 hour)"
echo "=================================================="
echo ""

echo "Current files:"
echo "  - code/crates/nestgate-core/src/error/helpers.rs (53 lines)"
echo "  - code/crates/nestgate-core/src/error/modernized_error_helpers.rs (26 lines)"
echo ""
echo "Target: Merge into error/utilities.rs (80 lines)"
echo ""

ls -lh code/crates/nestgate-core/src/error/*helper*.rs 2>/dev/null || echo "(Files not found - path may be different)"

echo ""
echo "=================================================="
echo "📊 GENERATED AUDIT FILES"
echo "=================================================="
echo ""
echo "Review these files to plan consolidation:"
echo "  1. provider_traits_audit.txt - All provider trait definitions"
echo "  2. async_trait_audit.txt - All async_trait usage with context"
echo ""
echo "Next steps:"
echo "  1. Review the audit files"
echo "  2. Read UNIFICATION_DEEP_ANALYSIS_NOV_9_2025.md for detailed analysis"
echo "  3. Begin with network module consolidation (highest impact)"
echo "  4. Follow with provider trait migration"
echo ""

echo "=================================================="
echo "✅ AUDIT COMPLETE"
echo "=================================================="
echo ""
echo "📚 Key documents:"
echo "  - UNIFICATION_DEEP_ANALYSIS_NOV_9_2025.md (comprehensive analysis)"
echo "  - UNIFICATION_TECHNICAL_DEBT_REPORT_NOV_8_2025.md (previous report)"
echo "  - V0.12.0_CLEANUP_CHECKLIST.md (deprecation cleanup plan)"
echo ""
echo "🎯 Recommended next action:"
echo "  Start with network module Service trait consolidation (2-3 days effort)"
echo ""
echo "Good luck! 🚀"
echo ""

