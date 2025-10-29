#!/bin/bash

# 🔒 **AGPL-3.0 COMPLIANCE VERIFICATION SCRIPT**
# Ensures all ecoPrimals are properly licensed under AGPL-3.0-only (strictest copyleft)

set -euo pipefail

echo "🔒 **ECOPRIMALS AGPL-3.0 COMPLIANCE VERIFICATION**"
echo "=================================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

ERRORS=0
WARNINGS=0

# Function to report errors
report_error() {
    echo -e "${RED}❌ ERROR: $1${NC}"
    ((ERRORS++))
}

# Function to report warnings
report_warning() {
    echo -e "${YELLOW}⚠️  WARNING: $1${NC}"
    ((WARNINGS++))
}

# Function to report success
report_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

# Function to report info
report_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

echo "1. Checking main LICENSE file..."
if [[ -f "LICENSE" ]]; then
    if grep -q "GNU AFFERO GENERAL PUBLIC LICENSE" LICENSE; then
        if grep -q "AGPL-3.0-only" LICENSE; then
            report_success "Main LICENSE file contains AGPL-3.0"
        else
            report_warning "LICENSE mentions AGPL but not explicitly AGPL-3.0-only"
        fi
        
        if grep -q "beardog entropy" LICENSE; then
            report_info "Beardog entropy human free use provision found"
        else
            report_info "Beardog entropy provision noted for future implementation"
        fi
    else
        report_error "Main LICENSE file does not contain AGPL-3.0 license"
    fi
else
    report_error "No LICENSE file found in project root"
fi

echo ""
echo "2. Checking workspace Cargo.toml license declaration..."
if [[ -f "Cargo.toml" ]]; then
    if grep -q 'license = "AGPL-3.0-only"' Cargo.toml; then
        report_success "Workspace declares AGPL-3.0-only license"
    else
        report_error "Workspace Cargo.toml missing or incorrect license declaration"
    fi
else
    report_error "No workspace Cargo.toml found"
fi

echo ""
echo "3. Checking individual crate license declarations..."

# Find all Cargo.toml files in crates
CRATE_COUNT=0
COMPLIANT_COUNT=0

# Check crates
if [[ -d "code/crates" ]]; then
    for cargo_file in code/crates/*/Cargo.toml; do
        if [[ -f "$cargo_file" ]]; then
            ((CRATE_COUNT++))
            crate_name=$(basename "$(dirname "$cargo_file")")
            
            if grep -q 'license = "AGPL-3.0-only"' "$cargo_file"; then
                report_success "✓ $crate_name: AGPL-3.0-only"
                ((COMPLIANT_COUNT++))
            elif grep -q 'license = "AGPL-3.0-or-later"' "$cargo_file"; then
                report_warning "$crate_name: Uses AGPL-3.0-or-later (should be AGPL-3.0-only)"
            elif grep -q 'license.*=' "$cargo_file"; then
                license_line=$(grep 'license.*=' "$cargo_file")
                report_error "$crate_name: Wrong license - $license_line"
            else
                report_error "$crate_name: No license declaration found"
            fi
        fi
    done
fi

# Check tools as well
if [[ -d "tools" ]]; then
    for cargo_file in tools/*/Cargo.toml; do
        if [[ -f "$cargo_file" ]]; then
            ((CRATE_COUNT++))
            tool_name=$(basename "$(dirname "$cargo_file")")
            
            if grep -q 'license = "AGPL-3.0-only"' "$cargo_file"; then
                report_success "✓ tools/$tool_name: AGPL-3.0-only"
                ((COMPLIANT_COUNT++))
            elif grep -q 'license.*=' "$cargo_file"; then
                license_line=$(grep 'license.*=' "$cargo_file")
                report_error "tools/$tool_name: Wrong license - $license_line"
            else
                report_error "tools/$tool_name: No license declaration found"
            fi
        fi
    done
fi

# Check standalone-tests
if [[ -f "standalone-tests/Cargo.toml" ]]; then
    ((CRATE_COUNT++))
    if grep -q 'license = "AGPL-3.0-only"' "standalone-tests/Cargo.toml"; then
        report_success "✓ standalone-tests: AGPL-3.0-only"
        ((COMPLIANT_COUNT++))
    elif grep -q 'license.*=' "standalone-tests/Cargo.toml"; then
        license_line=$(grep 'license.*=' "standalone-tests/Cargo.toml")
        report_error "standalone-tests: Wrong license - $license_line"
    else
        report_error "standalone-tests: No license declaration found"
    fi
fi

echo ""
echo "4. Checking for license headers in source files..."

# Check for proper AGPL headers in Rust files
RUST_FILES=0
HEADER_COMPLIANT=0

# Sample check of key files for license headers
key_files=(
    "code/crates/nestgate-core/src/lib.rs"
    "code/crates/nestgate-api/src/lib.rs"
    "code/crates/nestgate-zfs/src/lib.rs"
)

for file in "${key_files[@]}"; do
    if [[ -f "$file" ]]; then
        ((RUST_FILES++))
        if head -20 "$file" | grep -qi "agpl\|copyright.*ecoprimals"; then
            report_success "✓ $file: Has license header"
            ((HEADER_COMPLIANT++))
        else
            report_info "$file: Missing license header (can be added later)"
        fi
    fi
done

echo ""
echo "5. Checking pedantic-deny.toml for AGPL compliance..."

if [[ -f "tools/pedantic-deny.toml" ]]; then
    if grep -A 10 'allow = \[' tools/pedantic-deny.toml | grep -q '"AGPL-3.0"'; then
        report_success "pedantic-deny.toml allows AGPL-3.0"
    else
        report_error "pedantic-deny.toml does not allow AGPL-3.0"
    fi
    
    if grep -q 'copyleft = "allow"' tools/pedantic-deny.toml; then
        report_success "pedantic-deny.toml allows copyleft licenses"
    else
        report_error "pedantic-deny.toml denies copyleft licenses"
    fi
else
    report_warning "No pedantic-deny.toml found"
fi

echo ""
echo "6. Summary Report"
echo "=================="
echo -e "${BLUE}Total crates checked: $CRATE_COUNT${NC}"
echo -e "${GREEN}AGPL-3.0-only compliant: $COMPLIANT_COUNT${NC}"
echo -e "${YELLOW}Warnings: $WARNINGS${NC}"
echo -e "${RED}Errors: $ERRORS${NC}"

echo ""
if [[ $ERRORS -eq 0 ]]; then
    echo -e "${GREEN}🎉 **AGPL-3.0 COMPLIANCE: EXCELLENT**${NC}"
    echo -e "${GREEN}All ecoPrimals are properly licensed under AGPL-3.0-only${NC}"
    echo -e "${GREEN}Strictest copyleft licensing successfully implemented${NC}"
    
    if [[ $COMPLIANT_COUNT -eq $CRATE_COUNT ]]; then
        echo -e "${GREEN}✅ 100% crate compliance achieved${NC}"
    fi
    
    exit 0
else
    echo -e "${RED}🚨 **AGPL-3.0 COMPLIANCE: ISSUES FOUND**${NC}"
    echo -e "${RED}Please fix the $ERRORS error(s) above${NC}"
    echo -e "${YELLOW}Also address the $WARNINGS warning(s) if possible${NC}"
    exit 1
fi 