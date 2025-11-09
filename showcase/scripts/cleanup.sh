#!/usr/bin/env bash
# Cleanup script for NestGate showcase
# Safely removes all demo resources

set -euo pipefail

SHOWCASE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$SHOWCASE_ROOT"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

print_step() { echo -e "▶ $1"; }
print_success() { echo -e "  ${GREEN}✓${NC} $1"; }
print_warning() { echo -e "  ${YELLOW}⚠${NC} $1"; }

echo "═══════════════════════════════════════════════════════"
echo "NestGate Showcase Cleanup"
echo "═══════════════════════════════════════════════════════"
echo ""

# ═══════════════════════════════════════════════════════════
# 1. STOP RUNNING SERVERS
# ═══════════════════════════════════════════════════════════

print_step "Stopping running servers..."

if pgrep -f "nestgate.*server" > /dev/null; then
    pkill -f "nestgate.*server" || true
    sleep 1
    print_success "Servers stopped"
else
    print_success "No servers running"
fi

# ═══════════════════════════════════════════════════════════
# 2. CLEANUP ZFS RESOURCES
# ═══════════════════════════════════════════════════════════

print_step "Cleaning up ZFS resources..."

if command -v zpool &> /dev/null; then
    # Check if demo pool exists
    if sudo zpool list nestgate_demo &> /dev/null; then
        print_warning "Found ZFS pool 'nestgate_demo'"
        
        # Ask for confirmation
        read -p "  Destroy pool 'nestgate_demo'? [y/N] " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            sudo zpool destroy nestgate_demo
            print_success "Pool destroyed"
        else
            print_warning "Pool preserved (manual cleanup needed)"
        fi
    else
        print_success "No ZFS pools to cleanup"
    fi
    
    # Remove backing files
    if ls /tmp/nestgate_pool_*.img &> /dev/null; then
        rm -f /tmp/nestgate_pool_*.img
        print_success "Backing files removed"
    fi
else
    print_success "ZFS not installed, skipping"
fi

# ═══════════════════════════════════════════════════════════
# 3. CLEANUP FILESYSTEM RESOURCES
# ═══════════════════════════════════════════════════════════

print_step "Cleaning up filesystem resources..."

# Remove test directories
if [[ -d /tmp/nestgate_showcase ]]; then
    sudo rm -rf /tmp/nestgate_showcase
    print_success "Mount point removed"
fi

if ls /tmp/nestgate_demo_* &> /dev/null 2>&1; then
    rm -rf /tmp/nestgate_demo_*
    print_success "Test directories removed"
fi

# Remove test files
if ls /tmp/nestgate_perf_test_*.dat &> /dev/null 2>&1; then
    rm -f /tmp/nestgate_perf_test_*.dat
    print_success "Test files removed"
fi

# ═══════════════════════════════════════════════════════════
# 4. CLEANUP OUTPUT DIRECTORIES
# ═══════════════════════════════════════════════════════════

print_step "Cleaning output directories..."

if [[ -d outputs ]]; then
    # Ask before cleaning
    read -p "  Remove output logs and metrics? [y/N] " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        rm -rf outputs/logs/* outputs/metrics/* outputs/screenshots/*
        print_success "Outputs cleaned"
    else
        print_warning "Outputs preserved"
    fi
else
    print_success "No outputs to clean"
fi

# ═══════════════════════════════════════════════════════════
# 5. CLEANUP GENERATED DATA
# ═══════════════════════════════════════════════════════════

print_step "Cleaning generated data..."

if [[ -d data/sample_files ]]; then
    rm -rf data/sample_files/*
    print_success "Sample files removed"
fi

# ═══════════════════════════════════════════════════════════
# SUMMARY
# ═══════════════════════════════════════════════════════════

echo ""
print_success "Cleanup complete!"
echo ""
echo "Remaining resources (if any):"
echo "  ZFS pools:    sudo zpool list | grep nestgate"
echo "  Mount points: ls /tmp/nestgate_*"
echo "  Processes:    ps aux | grep nestgate"
echo ""

