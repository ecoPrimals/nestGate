#!/bin/bash
# Documentation Cleanup and Consolidation Script
# Organizes redundant documentation files and creates clean archive structure

echo "🧹 NESTGATE DOCUMENTATION CLEANUP"
echo "=================================="

# Create organized archive structure
mkdir -p docs/archive/historical-consolidated
mkdir -p docs/archive/superseded-reports

# Count files before cleanup
BEFORE_COUNT=$(find docs/archive/ -name "*.md" | wc -l)
echo "📊 Documentation files before cleanup: $BEFORE_COUNT"

# Move redundant completion reports
echo "📁 Consolidating completion reports..."
find docs/archive/ -name "*COMPLETION*" -name "*.md" -exec mv {} docs/archive/superseded-reports/ \; 2>/dev/null
find docs/archive/ -name "*COMPLETE*" -name "*.md" -exec mv {} docs/archive/superseded-reports/ \; 2>/dev/null
find docs/archive/ -name "*SUCCESS*" -name "*.md" -exec mv {} docs/archive/superseded-reports/ \; 2>/dev/null

# Move redundant achievement reports  
echo "📁 Consolidating achievement reports..."
find docs/archive/achievement-reports/ -name "*.md" -exec mv {} docs/archive/superseded-reports/ \; 2>/dev/null

# Move redundant status reports
echo "📁 Consolidating status reports..."
find docs/archive/status-reports/ -name "*FINAL*" -name "*.md" -exec mv {} docs/archive/superseded-reports/ \; 2>/dev/null
find docs/archive/status-reports/ -name "*ULTIMATE*" -name "*.md" -exec mv {} docs/archive/superseded-reports/ \; 2>/dev/null

# Create index of preserved current documentation
echo "📋 Creating current documentation index..."
cat > docs/CURRENT_DOCUMENTATION_INDEX.md << 'EOF'
# 📚 **NESTGATE CURRENT DOCUMENTATION INDEX**

**Date**: January 30, 2025  
**Status**: ✅ **ACTIVE REFERENCE** - Current and maintained documentation  
**Purpose**: Quick access to essential NestGate documentation

---

## 🎯 **ESSENTIAL CURRENT DOCS**

### **Architecture & Specifications**
- [Architecture Overview](current/ARCHITECTURE_DIAGRAMS.md) - System design overview
- [API Reference](current/API_REFERENCE.md) - Complete API documentation  
- [Canonical Modernization Guide](current/CANONICAL_MODERNIZATION_GUIDE.md) - Implementation guide

### **Implementation Guides**
- [Canonical Config Migration Guide](guides/CANONICAL_CONFIG_MIGRATION_GUIDE.md) - Configuration migration
- [Error Standardization Migration Plan](guides/ERROR_STANDARDIZATION_MIGRATION_PLAN.md) - Error system migration
- [Phase 2 Migration Guide](guides/PHASE_2_MIGRATION_GUIDE.md) - Phase 2 implementation

### **Development & Planning**
- [Development Guide](current/DEVELOPMENT_GUIDE.md) - Developer setup and workflow
- [Next Phase Development Plan](planning/NEXT_PHASE_DEVELOPMENT_PLAN.md) - Future roadmap
- [Test Coverage Improvement Plan](planning/TEST_COVERAGE_IMPROVEMENT_PLAN.md) - Testing strategy

### **Reports & Analysis**
- [Build Optimization Report](BUILD_OPTIMIZATION_REPORT.md) - Build system analysis
- [Canonical Modernization Complete](CANONICAL_MODERNIZATION_COMPLETE.md) - Achievement summary
- [Comprehensive Unification Report](COMPREHENSIVE_UNIFICATION_MODERNIZATION_REPORT.md) - Current status

---

## 📂 **ARCHIVE ORGANIZATION**

### **Active Archives** (Maintained)
- `docs/current/` - Current active documentation
- `docs/guides/` - Implementation and migration guides  
- `docs/planning/` - Future development plans
- `docs/implementation-reports/` - Key implementation summaries

### **Historical Archives** (Reference Only)
- `docs/archive/historical-consolidated/` - Consolidated historical summaries
- `docs/archive/superseded-reports/` - Individual historical reports
- `docs/archive/` - General historical documentation

---

*Total Active Documentation: ~53 current files | Historical Archive: ~298 files*
EOF

# Count files after cleanup
AFTER_COUNT=$(find docs/archive/superseded-reports/ -name "*.md" 2>/dev/null | wc -l)
CURRENT_COUNT=$(find docs/current/ docs/guides/ docs/planning/ -name "*.md" 2>/dev/null | wc -l)

echo "📊 CLEANUP SUMMARY:"
echo "  📁 Files moved to superseded: $AFTER_COUNT"
echo "  📋 Current active docs: $CURRENT_COUNT"
echo "  🧹 Cleanup completed successfully"

# Create README for superseded reports
cat > docs/archive/superseded-reports/README.md << 'EOF'
# 📚 Superseded Documentation Archive

This directory contains historical documentation that has been superseded by consolidated reports and current documentation.

## Purpose
These files are preserved for historical reference but are no longer actively maintained. The information they contain has been consolidated into:

- `docs/archive/historical-consolidated/CONSOLIDATED_ACHIEVEMENT_SUMMARY.md`
- Current documentation in `docs/current/`
- Active specifications in `specs/`

## Organization
- Achievement reports documenting completed milestones
- Completion reports from various development phases  
- Status reports from the modernization journey
- Success reports celebrating major accomplishments

All essential information has been preserved in the consolidated summaries.
EOF

echo "✅ Documentation cleanup completed successfully!"
echo "📋 See docs/CURRENT_DOCUMENTATION_INDEX.md for active documentation" 