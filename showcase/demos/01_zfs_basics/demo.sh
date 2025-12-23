#!/usr/bin/env bash
# Demo 1: ZFS Basics with Live Operations
# Demonstrates pool creation, datasets, snapshots, and compression

set -euo pipefail

DEMO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SHOWCASE_ROOT="$(dirname "$(dirname "$DEMO_ROOT")")"
cd "$DEMO_ROOT"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

# Configuration
POOL_NAME="nestgate_demo"
POOL_SIZE_MB=2048  # 2GB
DATA_SIZE_MB=100
MOUNT_POINT="/tmp/nestgate_showcase"

# Helper functions
print_step() { echo -e "${CYAN}▶${NC} $1"; }
print_success() { echo -e "  ${GREEN}✓${NC} $1"; }
print_info() { echo "  → $1"; }

format_bytes() {
    local bytes=$1
    if [[ $bytes -lt 1024 ]]; then
        echo "${bytes}B"
    elif [[ $bytes -lt 1048576 ]]; then
        echo "$(awk "BEGIN {printf \"%.1f\", $bytes/1024}")KB"
    elif [[ $bytes -lt 1073741824 ]]; then
        echo "$(awk "BEGIN {printf \"%.1f\", $bytes/1048576}")MB"
    else
        echo "$(awk "BEGIN {printf \"%.1f\", $bytes/1073741824}")GB"
    fi
}

# ═══════════════════════════════════════════════════════════
# DEMO EXECUTION
# ═══════════════════════════════════════════════════════════

main() {
    echo "═══════════════════════════════════════════════════════"
    echo "Demo 1: ZFS Fundamentals"
    echo "═══════════════════════════════════════════════════════"
    echo ""

    # Check if we're using ZFS or filesystem backend
    if [[ "${NESTGATE_BACKEND:-auto}" == "filesystem" ]] || ! command -v zpool &> /dev/null; then
        echo "⚠️  ZFS not available - using filesystem backend demonstration"
        run_filesystem_demo
        return 0
    fi

    # ═══════════════════════════════════════════════════════
    # 1. CREATE TEST POOL
    # ═══════════════════════════════════════════════════════

    print_step "Creating test pool '$POOL_NAME'..."
    
    # Create backing files for the pool (safe, no real disks)
    local pool_file="/tmp/nestgate_pool_${RANDOM}.img"
    dd if=/dev/zero of="$pool_file" bs=1M count=$POOL_SIZE_MB status=none
    
    # Create pool from file
    if sudo zpool create -f "$POOL_NAME" "$pool_file" -m "$MOUNT_POINT" 2>/dev/null; then
        local pool_size=$(sudo zpool list -H -o size "$POOL_NAME")
        print_success "Pool created: $pool_size total"
        
        # Check pool status
        local pool_status=$(sudo zpool status "$POOL_NAME" | grep state: | awk '{print $2}')
        print_success "Pool status: $pool_status"
    else
        echo "  ℹ️  Pool might already exist, using existing pool"
    fi

    # ═══════════════════════════════════════════════════════
    # 2. CREATE DATASET
    # ═══════════════════════════════════════════════════════

    print_step "Creating dataset '$POOL_NAME/data'..."
    
    if sudo zfs create "$POOL_NAME/data" 2>/dev/null || true; then
        print_success "Dataset created"
        
        local mount_point=$(sudo zfs get -H -o value mountpoint "$POOL_NAME/data")
        print_success "Mounted at: $mount_point"
        
        # Set compression
        sudo zfs set compression=lz4 "$POOL_NAME/data"
        print_info "Compression enabled: lz4"
    fi

    # ═══════════════════════════════════════════════════════
    # 3. WRITE TEST DATA
    # ═══════════════════════════════════════════════════════

    print_step "Writing test data (${DATA_SIZE_MB}MB)..."
    
    local data_dir="$MOUNT_POINT/data"
    local start_time=$(date +%s.%N)
    
    # Generate compressible test data (text files)
    for i in $(seq 1 10); do
        # Create files with repetitive data (compresses well)
        sudo bash -c "head -c $((DATA_SIZE_MB * 102400)) /dev/urandom | base64 | head -c $((DATA_SIZE_MB * 102400)) > $data_dir/testfile_$i.dat"
        echo -n "."
    done
    echo ""
    
    local end_time=$(date +%s.%N)
    local duration=$(awk "BEGIN {printf \"%.1f\", $end_time - $start_time}")
    local throughput=$(awk "BEGIN {printf \"%.1f\", $DATA_SIZE_MB / $duration}")
    
    print_success "Write completed in ${duration}s (${throughput} MB/s)"

    # ═══════════════════════════════════════════════════════
    # 4. CREATE SNAPSHOT
    # ═══════════════════════════════════════════════════════

    print_step "Creating snapshot '$POOL_NAME/data@demo1'..."
    
    if sudo zfs snapshot "$POOL_NAME/data@demo1"; then
        print_success "Snapshot created"
        
        # Show snapshot space usage (should be ~0 due to copy-on-write)
        local snap_used=$(sudo zfs list -H -o used "$POOL_NAME/data@demo1")
        print_success "Space used: $snap_used (copy-on-write)"
    fi

    # ═══════════════════════════════════════════════════════
    # 5. COMPRESSION ANALYSIS
    # ═══════════════════════════════════════════════════════

    print_step "Analyzing compression..."
    
    # Get compression stats
    local used=$(sudo zfs get -H -o value used "$POOL_NAME/data" | sed 's/[^0-9.]//g')
    local referenced=$(sudo zfs get -H -o value referenced "$POOL_NAME/data" | sed 's/[^0-9.]//g')
    local compressratio=$(sudo zfs get -H -o value compressratio "$POOL_NAME/data")
    
    echo ""
    print_info "Original size:  ${DATA_SIZE_MB}.0 MB (written)"
    print_info "Compressed:     ~${used}MB (on disk)"
    print_info "Ratio:          $compressratio"
    
    if [[ "$compressratio" != "1.00x" ]]; then
        local saved=$(awk "BEGIN {printf \"%.1f\", $DATA_SIZE_MB - $used}")
        print_success "Space saved: ~${saved}MB"
    fi

    # ═══════════════════════════════════════════════════════
    # 6. DEMONSTRATE SNAPSHOTS
    # ═══════════════════════════════════════════════════════

    print_step "Demonstrating snapshot functionality..."
    
    # Modify data
    sudo bash -c "echo 'Modified data' > $data_dir/modified.txt"
    print_info "Modified data in dataset"
    
    # Create another snapshot
    sudo zfs snapshot "$POOL_NAME/data@demo2"
    print_info "Created second snapshot: $POOL_NAME/data@demo2"
    
    # List all snapshots
    echo ""
    print_step "Available snapshots:"
    sudo zfs list -t snapshot -r "$POOL_NAME/data" | tail -n +2 | while read line; do
        print_info "$line"
    done

    # ═══════════════════════════════════════════════════════
    # 7. DATASET PROPERTIES
    # ═══════════════════════════════════════════════════════

    echo ""
    print_step "Dataset properties:"
    
    local props=(
        "used"
        "available"
        "referenced"
        "compressratio"
        "compression"
        "mountpoint"
    )
    
    for prop in "${props[@]}"; do
        local value=$(sudo zfs get -H -o value "$prop" "$POOL_NAME/data")
        printf "  %-15s %s\n" "$prop:" "$value"
    done

    # ═══════════════════════════════════════════════════════
    # 8. PERFORMANCE METRICS
    # ═══════════════════════════════════════════════════════

    echo ""
    print_step "Performance metrics:"
    
    # I/O stats
    if command -v zpool &> /dev/null; then
        echo ""
        sudo zpool iostat "$POOL_NAME" 1 2 | tail -1 | awk '{
            print "  Bandwidth (read):  " $5
            print "  Bandwidth (write): " $6
            print "  Operations (read): " $7
            print "  Operations (write):" $8
        }'
    fi

    echo ""
    print_success "Demo 1 completed successfully!"
    
    # Cleanup instructions
    echo ""
    echo "To inspect the pool:"
    echo "  sudo zpool status $POOL_NAME"
    echo "  sudo zfs list -r $POOL_NAME"
    echo "  ls -lah $MOUNT_POINT/data"
    echo ""
    echo "To manually cleanup:"
    echo "  sudo zpool destroy $POOL_NAME"
    echo "  rm -f /tmp/nestgate_pool_*.img"
    echo ""
}

# ═══════════════════════════════════════════════════════════
# FILESYSTEM BACKEND DEMO (fallback when ZFS unavailable)
# ═══════════════════════════════════════════════════════════

run_filesystem_demo() {
    echo ""
    print_step "Creating test directory..."
    
    local test_dir="/tmp/nestgate_demo_$$"
    mkdir -p "$test_dir/data"
    print_success "Directory created: $test_dir"
    
    print_step "Writing test data (${DATA_SIZE_MB}MB)..."
    
    local start_time=$(date +%s.%N)
    for i in $(seq 1 10); do
        head -c $((DATA_SIZE_MB * 102400)) /dev/urandom | base64 | head -c $((DATA_SIZE_MB * 102400)) > "$test_dir/data/testfile_$i.dat"
        echo -n "."
    done
    echo ""
    
    local end_time=$(date +%s.%N)
    local duration=$(awk "BEGIN {printf \"%.1f\", $end_time - $start_time}")
    local throughput=$(awk "BEGIN {printf \"%.1f\", $DATA_SIZE_MB / $duration}")
    
    print_success "Write completed in ${duration}s (${throughput} MB/s)"
    
    print_step "Analyzing disk usage..."
    
    local disk_usage=$(du -sh "$test_dir" | awk '{print $1}')
    print_info "Total size: $disk_usage"
    
    local file_count=$(find "$test_dir" -type f | wc -l)
    print_info "Files created: $file_count"
    
    print_step "Creating 'snapshot' (directory copy)..."
    cp -r "$test_dir/data" "$test_dir/snapshot_demo1"
    print_success "Snapshot created: $test_dir/snapshot_demo1"
    
    echo ""
    print_success "Filesystem demo completed!"
    print_info "Test directory: $test_dir"
    print_info "Cleanup: rm -rf $test_dir"
    echo ""
}

# Run main
main "$@"

