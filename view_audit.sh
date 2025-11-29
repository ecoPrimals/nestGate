#!/usr/bin/env bash
#
# Quick Audit Viewer - November 27, 2025
# Display comprehensive audit results
#

set -euo pipefail

AUDIT_DIR="audits/nov-27-2025"
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

cd "$ROOT_DIR"

echo "================================================================================"
echo "  NESTGATE AUDIT VIEWER"
echo "================================================================================"
echo ""

if [ $# -eq 0 ]; then
    echo "Available audit documents:"
    echo ""
    echo "  Quick Reference:"
    echo "    1) console       - Console-friendly summary (this file)"
    echo "    2) followup      - Follow-up comprehensive review"
    echo "    3) quick         - Quick status (00_READ_ME_FIRST)"
    echo ""
    echo "  Detailed Audits:"
    echo "    4) full          - Full comprehensive audit"
    echo "    5) hardcoding    - Hardcoding audit + migration plan"
    echo "    6) errors        - Error handling migration plan"
    echo "    7) execution     - Week 1-4 execution summary"
    echo ""
    echo "Usage: ./view_audit.sh [option]"
    echo "Example: ./view_audit.sh console"
    echo ""
    exit 0
fi

case "$1" in
    console|1)
        cat AUDIT_RESPONSE_NOV_27_FOLLOWUP.txt
        ;;
    followup|2)
        less "$AUDIT_DIR/COMPREHENSIVE_REVIEW_NOV_27_FOLLOWUP.md"
        ;;
    quick|3)
        less 00_READ_ME_FIRST_NOV_27.md
        ;;
    full|4)
        less "$AUDIT_DIR/COMPREHENSIVE_AUDIT_NOV_27_2025.md"
        ;;
    hardcoding|5)
        less "$AUDIT_DIR/HARDCODING_AUDIT_NOV_27.md"
        ;;
    errors|6)
        less "$AUDIT_DIR/ERROR_HANDLING_MIGRATION_PLAN_NOV_27.md"
        ;;
    execution|7)
        less "$AUDIT_DIR/WEEK_1_4_EXECUTION_COMPLETE_NOV_27.md"
        ;;
    *)
        echo "Error: Unknown option '$1'"
        echo "Run './view_audit.sh' without arguments to see available options"
        exit 1
        ;;
esac

