#!/usr/bin/env bash
# Workload Generator for NestGate Showcase
# Generates realistic data patterns for demonstrations

set -euo pipefail

# Configuration
OUTPUT_DIR="${1:-.}"
SIZE_MB="${2:-100}"

# Colors
GREEN='\033[0;32m'
CYAN='\033[0;36m'
NC='\033[0m'

print_step() { echo -e "${CYAN}▶${NC} $1"; }
print_success() { echo -e "  ${GREEN}✓${NC} $1"; }

# ═══════════════════════════════════════════════════════════
# WORKLOAD PATTERNS
# ═══════════════════════════════════════════════════════════

generate_text_workload() {
    local output="$OUTPUT_DIR/text_data"
    mkdir -p "$output"
    
    print_step "Generating text workload..."
    
    # Generate log files (highly compressible)
    for i in $(seq 1 10); do
        cat > "$output/app_log_$i.log" << EOF
$(date -Iseconds) INFO  Starting application...
$(date -Iseconds) INFO  Loading configuration...
$(date -Iseconds) INFO  Connecting to database...
$(date -Iseconds) INFO  Server started on port 8080
$(date -Iseconds) DEBUG Processing request GET /api/health
$(date -Iseconds) DEBUG Processing request GET /api/metrics
$(date -Iseconds) INFO  Request completed in 15ms
EOF
        # Repeat log patterns to make files larger
        for j in $(seq 1 100); do
            cat "$output/app_log_$i.log" >> "$output/app_log_$i.log.tmp"
        done
        mv "$output/app_log_$i.log.tmp" "$output/app_log_$i.log"
    done
    
    print_success "Text workload generated (highly compressible)"
}

generate_binary_workload() {
    local output="$OUTPUT_DIR/binary_data"
    mkdir -p "$output"
    
    print_step "Generating binary workload..."
    
    # Generate binary files (low compressibility)
    for i in $(seq 1 5); do
        dd if=/dev/urandom of="$output/binary_file_$i.bin" bs=1M count=$((SIZE_MB / 5)) status=none
    done
    
    print_success "Binary workload generated (low compressibility)"
}

generate_mixed_workload() {
    local output="$OUTPUT_DIR/mixed_data"
    mkdir -p "$output"
    
    print_step "Generating mixed workload..."
    
    # Mix of different file types
    # JSON files
    for i in $(seq 1 3); do
        cat > "$output/config_$i.json" << EOF
{
  "version": "1.0.0",
  "server": {
    "host": "0.0.0.0",
    "port": 8080,
    "threads": 4
  },
  "database": {
    "url": "postgresql://localhost:5432/nestgate",
    "pool_size": 10,
    "timeout": 30
  },
  "logging": {
    "level": "info",
    "format": "json",
    "output": "/var/log/nestgate.log"
  }
}
EOF
    done
    
    # CSV files
    for i in $(seq 1 3); do
        cat > "$output/metrics_$i.csv" << EOF
timestamp,cpu_usage,memory_mb,disk_io_ops,network_kb
$(date +%s),25.5,1024,150,450
$(date +%s),30.2,1156,175,520
$(date +%s),28.7,1089,162,485
EOF
        # Repeat for larger files
        for j in $(seq 1 100); do
            tail -1 "$output/metrics_$i.csv" >> "$output/metrics_$i.csv"
        done
    done
    
    print_success "Mixed workload generated"
}

generate_incremental_workload() {
    local output="$OUTPUT_DIR/incremental_data"
    mkdir -p "$output"
    
    print_step "Generating incremental workload (for snapshot demos)..."
    
    # Create base dataset
    for i in $(seq 1 5); do
        echo "Base data - version 1.0" > "$output/data_$i.txt"
        for j in $(seq 1 100); do
            echo "Line $j: $(date +%s) - Sample data entry" >> "$output/data_$i.txt"
        done
    done
    
    # Mark snapshot point
    echo "SNAPSHOT_1" > "$output/.snapshot_marker"
    
    # Add incremental changes
    for i in $(seq 1 3); do
        echo "Modified data - version 1.1" >> "$output/data_$i.txt"
        echo "Additional line: $(date +%s)" >> "$output/data_$i.txt"
    done
    
    print_success "Incremental workload generated (shows snapshot efficiency)"
}

# ═══════════════════════════════════════════════════════════
# MAIN EXECUTION
# ═══════════════════════════════════════════════════════════

main() {
    echo "═══════════════════════════════════════════════════════"
    echo "NestGate Workload Generator"
    echo "═══════════════════════════════════════════════════════"
    echo ""
    echo "Output directory: $OUTPUT_DIR"
    echo "Target size:      ${SIZE_MB}MB"
    echo ""
    
    mkdir -p "$OUTPUT_DIR"
    
    # Generate all workload types
    generate_text_workload
    generate_binary_workload
    generate_mixed_workload
    generate_incremental_workload
    
    echo ""
    print_step "Summary:"
    
    local total_size=$(du -sh "$OUTPUT_DIR" | awk '{print $1}')
    local file_count=$(find "$OUTPUT_DIR" -type f | wc -l)
    
    echo "  Total size:  $total_size"
    echo "  File count:  $file_count"
    echo ""
    
    print_success "Workload generation complete!"
}

main "$@"

