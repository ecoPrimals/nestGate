#!/bin/bash
# NestGate ZFS Test Environment Setup
# Creates test ZFS pools using loop devices for development and testing

set -e

# Configuration
POOL_NAME="nestpool"
POOL_SIZE="2G"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LOOP_DIR="/tmp/nestgate-zfs-loops"
LOOP_FILES=("$LOOP_DIR/disk1.img" "$LOOP_DIR/disk2.img")

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

check_requirements() {
    log_info "Checking ZFS requirements..."
    
    if ! command -v zpool &> /dev/null; then
        log_error "zpool command not found. Please install ZFS utilities."
        exit 1
    fi
    
    if ! command -v zfs &> /dev/null; then
        log_error "zfs command not found. Please install ZFS utilities."
        exit 1
    fi
    
    # Check if ZFS module is loaded
    if ! lsmod | grep -q zfs; then
        log_error "ZFS kernel module not loaded. Please load with: sudo modprobe zfs"
        exit 1
    fi
    
    log_success "ZFS tools and kernel module are available"
}

cleanup_existing() {
    log_info "Cleaning up any existing test environment..."
    
    # Destroy existing pool if it exists
    if zpool list "$POOL_NAME" &> /dev/null; then
        log_warn "Destroying existing pool: $POOL_NAME"
        sudo zpool destroy "$POOL_NAME" || true
    fi
    
    # Clean up loop devices
    for loop_file in "${LOOP_FILES[@]}"; do
        # Find associated loop device
        loop_dev=$(losetup -j "$loop_file" 2>/dev/null | cut -d: -f1 || true)
        if [ -n "$loop_dev" ]; then
            log_warn "Detaching loop device: $loop_dev"
            sudo losetup -d "$loop_dev" || true
        fi
    done
    
    # Remove loop files
    if [ -d "$LOOP_DIR" ]; then
        log_warn "Removing loop files directory: $LOOP_DIR"
        rm -rf "$LOOP_DIR"
    fi
    
    log_success "Cleanup completed"
}

create_loop_devices() {
    log_info "Creating loop devices for ZFS testing..."
    
    # Create directory for loop files
    mkdir -p "$LOOP_DIR"
    
    # Create loop files
    for i in "${!LOOP_FILES[@]}"; do
        loop_file="${LOOP_FILES[$i]}"
        log_info "Creating loop file: $loop_file ($POOL_SIZE)"
        dd if=/dev/zero of="$loop_file" bs=1M count=2048 status=progress
        
        # Set up loop device
        loop_dev=$(sudo losetup --find --show "$loop_file")
        log_success "Created loop device: $loop_dev -> $loop_file"
        
        # Store loop device for pool creation
        LOOP_DEVICES[$i]="$loop_dev"
    done
}

create_zfs_pool() {
    log_info "Creating ZFS pool: $POOL_NAME"
    
    # Create pool with mirror configuration
    sudo zpool create "$POOL_NAME" mirror "${LOOP_DEVICES[@]}"
    
    # Verify pool creation
    if zpool list "$POOL_NAME" &> /dev/null; then
        log_success "ZFS pool created successfully: $POOL_NAME"
        zpool status "$POOL_NAME"
    else
        log_error "Failed to create ZFS pool: $POOL_NAME"
        exit 1
    fi
}

create_tier_datasets() {
    log_info "Creating tier datasets..."
    
    # Hot tier - high performance
    sudo zfs create "$POOL_NAME/hot"
    sudo zfs set compression=lz4 "$POOL_NAME/hot"
    sudo zfs set recordsize=128K "$POOL_NAME/hot"
    sudo zfs set atime=off "$POOL_NAME/hot"
    sudo zfs set primarycache=all "$POOL_NAME/hot"
    sudo zfs set secondarycache=all "$POOL_NAME/hot"
    log_success "Created hot tier dataset: $POOL_NAME/hot"
    
    # Warm tier - balanced
    sudo zfs create "$POOL_NAME/warm"
    sudo zfs set compression=zstd "$POOL_NAME/warm"
    sudo zfs set recordsize=1M "$POOL_NAME/warm"
    sudo zfs set atime=on "$POOL_NAME/warm"
    sudo zfs set primarycache=metadata "$POOL_NAME/warm"
    sudo zfs set secondarycache=metadata "$POOL_NAME/warm"
    log_success "Created warm tier dataset: $POOL_NAME/warm"
    
    # Cold tier - high compression
    sudo zfs create "$POOL_NAME/cold"
    sudo zfs set compression=gzip-9 "$POOL_NAME/cold"
    sudo zfs set recordsize=1M "$POOL_NAME/cold"
    sudo zfs set atime=off "$POOL_NAME/cold"
    sudo zfs set primarycache=metadata "$POOL_NAME/cold"
    sudo zfs set secondarycache=none "$POOL_NAME/cold"
    log_success "Created cold tier dataset: $POOL_NAME/cold"
    
    log_info "Tier datasets created successfully"
    zfs list -t filesystem "$POOL_NAME"
}

create_test_data() {
    log_info "Creating test data..."
    
    # Create some test files in each tier
    for tier in hot warm cold; do
        tier_path="/$POOL_NAME/$tier"
        sudo mkdir -p "$tier_path/test"
        
        # Create test files
        sudo dd if=/dev/urandom of="$tier_path/test/random_${tier}.dat" bs=1M count=10 2>/dev/null
        sudo touch "$tier_path/test/empty_${tier}.txt"
        echo "Test data for $tier tier" | sudo tee "$tier_path/test/info_${tier}.txt" > /dev/null
        
        log_success "Created test data in $tier tier"
    done
}

create_test_snapshots() {
    log_info "Creating test snapshots..."
    
    # Create snapshots for each tier
    for tier in hot warm cold; do
        snapshot_name="$POOL_NAME/$tier@initial"
        sudo zfs snapshot "$snapshot_name"
        log_success "Created snapshot: $snapshot_name"
    done
}

show_status() {
    log_info "ZFS Test Environment Status:"
    echo
    echo "=== Pool Status ==="
    zpool status "$POOL_NAME"
    echo
    echo "=== Dataset List ==="
    zfs list -t all "$POOL_NAME"
    echo
    echo "=== Pool Properties ==="
    zpool get all "$POOL_NAME" | head -20
    echo
    log_success "ZFS test environment is ready!"
    echo
    log_info "To clean up later, run: $0 cleanup"
}

main() {
    case "${1:-setup}" in
        "setup")
            log_info "Setting up NestGate ZFS test environment..."
            check_requirements
            cleanup_existing
            
            # Array to store loop devices
            declare -a LOOP_DEVICES
            
            create_loop_devices
            create_zfs_pool
            create_tier_datasets
            create_test_data
            create_test_snapshots
            show_status
            ;;
        
        "cleanup")
            log_info "Cleaning up NestGate ZFS test environment..."
            cleanup_existing
            log_success "Cleanup completed"
            ;;
        
        "status")
            if zpool list "$POOL_NAME" &> /dev/null; then
                show_status
            else
                log_warn "Test pool '$POOL_NAME' not found. Run '$0 setup' to create it."
            fi
            ;;
        
        *)
            echo "Usage: $0 {setup|cleanup|status}"
            echo "  setup   - Create ZFS test environment"
            echo "  cleanup - Remove ZFS test environment"
            echo "  status  - Show current status"
            exit 1
            ;;
    esac
}

# Make sure we're not running as root directly (sudo will be used when needed)
if [ "$EUID" -eq 0 ]; then
    log_error "Please run this script as a regular user (sudo will be used when needed)"
    exit 1
fi

main "$@" 