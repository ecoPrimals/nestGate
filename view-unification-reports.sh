#!/bin/bash
# 📚 Unification Reports Viewer
# Quick access to all unification assessment documentation

set -euo pipefail

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}📚 NESTGATE UNIFICATION REPORTS${NC}"
echo -e "${CYAN}================================${NC}"
echo ""

# Function to display menu
show_menu() {
    echo -e "${GREEN}Available Reports:${NC}"
    echo ""
    echo -e "${YELLOW}Quick Access (5-10 minutes):${NC}"
    echo "  1) Session Summary (SESSION_COMPLETE)"
    echo "  2) Final Status Overview (FINAL_STATUS)"
    echo "  3) Quick Reference (2 min)"
    echo ""
    echo -e "${YELLOW}Comprehensive Reports (20-30 minutes):${NC}"
    echo "  4) Main Execution Report ⭐"
    echo "  5) Milestone Achievement 🏆"
    echo "  6) Technical Deep Dive"
    echo "  7) Executive Summary"
    echo ""
    echo -e "${YELLOW}Project Status:${NC}"
    echo "  8) Project Status Master"
    echo "  9) Documentation Index"
    echo ""
    echo -e "${YELLOW}Other Options:${NC}"
    echo "  l) List all reports"
    echo "  s) Search reports"
    echo "  v) Validate build & tests"
    echo "  q) Quit"
    echo ""
}

# Function to view report
view_report() {
    local file=$1
    if [ -f "$file" ]; then
        echo -e "${GREEN}📄 Viewing: $file${NC}"
        echo ""
        if command -v bat &> /dev/null; then
            bat --style=plain --paging=always "$file"
        elif command -v less &> /dev/null; then
            less "$file"
        else
            cat "$file"
        fi
    else
        echo -e "${YELLOW}⚠️  File not found: $file${NC}"
    fi
}

# Main loop
while true; do
    show_menu
    read -p "Select option: " choice
    echo ""
    
    case $choice in
        1)
            view_report "SESSION_COMPLETE_NOV_8_2025_EVENING.md"
            ;;
        2)
            view_report "FINAL_STATUS_NOV_8_2025_EVENING.md"
            ;;
        3)
            view_report "UNIFICATION_QUICK_SUMMARY_NOV_8_2025.md"
            ;;
        4)
            view_report "UNIFICATION_EXECUTION_REPORT_NOV_8_2025.md"
            ;;
        5)
            view_report "UNIFICATION_COMPLETE_NOV_8_2025.md"
            ;;
        6)
            view_report "UNIFICATION_STATUS_COMPREHENSIVE_NOV_8_2025_EVENING.md"
            ;;
        7)
            view_report "UNIFICATION_FINDINGS_EXECUTIVE_SUMMARY.md"
            ;;
        8)
            view_report "PROJECT_STATUS_MASTER.md"
            ;;
        9)
            view_report "UNIFICATION_DOCUMENTATION_INDEX.md"
            ;;
        l|L)
            echo -e "${GREEN}📋 All Unification Reports:${NC}"
            echo ""
            ls -lh *UNIFICATION*.md *NOV_8_2025*.md SESSION_COMPLETE*.md FINAL_STATUS*.md 2>/dev/null || echo "No reports found"
            echo ""
            read -p "Press Enter to continue..."
            ;;
        s|S)
            echo -e "${GREEN}🔍 Search Reports${NC}"
            read -p "Enter search term: " term
            echo ""
            echo -e "${GREEN}Results for '$term':${NC}"
            grep -n "$term" *UNIFICATION*.md *NOV_8_2025*.md SESSION_COMPLETE*.md FINAL_STATUS*.md 2>/dev/null | head -20
            echo ""
            read -p "Press Enter to continue..."
            ;;
        v|V)
            echo -e "${GREEN}✅ Validating Build & Tests${NC}"
            echo ""
            echo -e "${BLUE}Running: cargo check --workspace${NC}"
            cargo check --workspace 2>&1 | grep -E "(Finished|error\[)" | head -5
            echo ""
            echo -e "${BLUE}Running: cargo test --workspace --lib${NC}"
            cargo test --workspace --lib 2>&1 | grep "test result" | tail -5
            echo ""
            read -p "Press Enter to continue..."
            ;;
        q|Q)
            echo -e "${GREEN}✅ Thank you for using the Unification Reports Viewer!${NC}"
            exit 0
            ;;
        *)
            echo -e "${YELLOW}Invalid option. Please try again.${NC}"
            sleep 1
            ;;
    esac
done

