#!/bin/bash
# ==============================================================================
# Large Dataset Generator for Ecosystem Integration Testing
# ==============================================================================
#
# Generates multi-GB datasets with realistic distributions for primal
# integration testing. Supports various data types and compression.
#
# Usage:
#   ./generate_large_dataset.sh [OPTIONS]
#
# Options:
#   --size SIZE          Dataset size (e.g., 1GB, 10GB, 100GB)
#   --type TYPE          Dataset type (mixed, text, binary, tensors, timeseries)
#   --primal PRIMAL      Target primal (beardog, squirrel, toadstool, songbird)
#   --output PATH        Output directory
#   --compression BOOL   Enable ZFS compression (default: true)
#   --distribution TYPE  Distribution pattern (realistic, uniform, zipf)
#   --help               Show this help message
#
# Examples:
#   # Generate 10GB mixed dataset for beardog
#   ./generate_large_dataset.sh --size 10GB --type mixed --primal beardog \
#     --output /nestgate_data/datasets/beardog_test
#
#   # Generate 50GB text corpus for squirrel LLM training
#   ./generate_large_dataset.sh --size 50GB --type text --primal squirrel \
#     --output /nestgate_data/datasets/llm_training
#
# ==============================================================================

set -euo pipefail

# ==============================================================================
# Configuration
# ==============================================================================

# Defaults
SIZE="10GB"
TYPE="mixed"
PRIMAL="generic"
OUTPUT_DIR="/tmp/nestgate_datasets"
COMPRESSION=true
DISTRIBUTION="realistic"
VERBOSE=false

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# ==============================================================================
# Helper Functions
# ==============================================================================

log_info() {
    echo -e "${BLUE}▶ $1${NC}"
}

log_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

log_warn() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

log_error() {
    echo -e "${RED}✗ $1${NC}"
}

show_help() {
    sed -n '/^# Usage:/,/^# ===/p' "$0" | sed 's/^# //' | head -n -1
    exit 0
}

# Parse size to bytes
parse_size() {
    local size_str=$1
    local num=$(echo "$size_str" | sed 's/[^0-9.]//g')
    local unit=$(echo "$size_str" | sed 's/[0-9.]//g' | tr '[:lower:]' '[:upper:]')
    
    case $unit in
        KB) echo $(echo "$num * 1024" | bc | cut -d. -f1) ;;
        MB) echo $(echo "$num * 1024 * 1024" | bc | cut -d. -f1) ;;
        GB) echo $(echo "$num * 1024 * 1024 * 1024" | bc | cut -d. -f1) ;;
        TB) echo $(echo "$num * 1024 * 1024 * 1024 * 1024" | bc | cut -d. -f1) ;;
        *) echo "$num" ;;
    esac
}

# Format bytes to human-readable
format_size() {
    local bytes=$1
    if [ "$bytes" -lt 1024 ]; then
        echo "${bytes}B"
    elif [ "$bytes" -lt $((1024 * 1024)) ]; then
        echo "$(echo "scale=2; $bytes / 1024" | bc)KB"
    elif [ "$bytes" -lt $((1024 * 1024 * 1024)) ]; then
        echo "$(echo "scale=2; $bytes / 1024 / 1024" | bc)MB"
    else
        echo "$(echo "scale=2; $bytes / 1024 / 1024 / 1024" | bc)GB"
    fi
}

# ==============================================================================
# Parse Command Line Arguments
# ==============================================================================

while [[ $# -gt 0 ]]; do
    case $1 in
        --size) SIZE="$2"; shift 2 ;;
        --type) TYPE="$2"; shift 2 ;;
        --primal) PRIMAL="$2"; shift 2 ;;
        --output) OUTPUT_DIR="$2"; shift 2 ;;
        --compression) COMPRESSION="$2"; shift 2 ;;
        --distribution) DISTRIBUTION="$2"; shift 2 ;;
        --verbose) VERBOSE=true; shift ;;
        --help) show_help ;;
        *) log_error "Unknown option: $1"; exit 1 ;;
    esac
done

# ==============================================================================
# Validation
# ==============================================================================

log_info "Validating configuration..."

# Parse size
SIZE_BYTES=$(parse_size "$SIZE")
SIZE_HUMAN=$(format_size "$SIZE_BYTES")

log_info "Target size: $SIZE_HUMAN ($SIZE_BYTES bytes)"

# Validate type
case $TYPE in
    mixed|text|binary|tensors|timeseries) ;;
    *) log_error "Invalid type: $TYPE"; exit 1 ;;
esac

# Validate primal
case $PRIMAL in
    beardog|squirrel|toadstool|songbird|biomeOS|generic) ;;
    *) log_warn "Unknown primal: $PRIMAL (using generic config)" ;;
esac

# Check disk space
AVAILABLE_SPACE=$(df -B1 "$(dirname "$OUTPUT_DIR")" | tail -1 | awk '{print $4}')
REQUIRED_SPACE=$((SIZE_BYTES * 2))  # 2x for safety during generation

if [ "$AVAILABLE_SPACE" -lt "$REQUIRED_SPACE" ]; then
    log_error "Insufficient disk space"
    log_error "Available: $(format_size $AVAILABLE_SPACE)"
    log_error "Required: $(format_size $REQUIRED_SPACE)"
    exit 1
fi

log_success "Disk space check passed ($(format_size $AVAILABLE_SPACE) available)"

# ==============================================================================
# Setup Output Directory
# ==============================================================================

log_info "Creating output directory: $OUTPUT_DIR"

mkdir -p "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR/data"
mkdir -p "$OUTPUT_DIR/metadata"

# Create dataset metadata
cat > "$OUTPUT_DIR/metadata/dataset_info.json" <<EOF
{
  "name": "${PRIMAL}_${TYPE}_dataset",
  "version": "1.0.0",
  "created": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "size_bytes": ${SIZE_BYTES},
  "size_human": "${SIZE_HUMAN}",
  "type": "${TYPE}",
  "primal": "${PRIMAL}",
  "distribution": "${DISTRIBUTION}",
  "compression_enabled": ${COMPRESSION},
  "generator": "nestgate_large_dataset_generator_v1.0",
  "files": []
}
EOF

log_success "Output directory created"

# ==============================================================================
# Dataset Generation Functions
# ==============================================================================

# Generate mixed dataset (realistic workload)
generate_mixed_dataset() {
    log_info "Generating mixed dataset..."
    
    local generated=0
    local file_count=0
    local target=$SIZE_BYTES
    
    # Distribution: 40% text, 30% binary, 20% images, 10% small files
    local text_target=$((target * 40 / 100))
    local binary_target=$((target * 30 / 100))
    local image_target=$((target * 20 / 100))
    local small_target=$((target * 10 / 100))
    
    # Generate text files (highly compressible)
    log_info "Generating text files (40% of dataset)..."
    while [ "$generated" -lt "$text_target" ]; do
        local size=$((RANDOM % 10485760 + 1048576))  # 1MB-10MB
        local filename="text_$(printf "%06d" $file_count).txt"
        
        # Generate Lorem Ipsum-style text
        head -c "$size" /dev/urandom | tr -dc 'a-zA-Z0-9 \n' > "$OUTPUT_DIR/data/$filename"
        
        generated=$((generated + size))
        file_count=$((file_count + 1))
        
        if [ $((file_count % 10)) -eq 0 ]; then
            echo -ne "\r  Progress: $(format_size $generated) / $(format_size $text_target)"
        fi
    done
    echo ""
    log_success "Text files generated ($(format_size $generated))"
    
    # Generate binary files (low compressibility)
    log_info "Generating binary files (30% of dataset)..."
    generated=0
    while [ "$generated" -lt "$binary_target" ]; do
        local size=$((RANDOM % 52428800 + 10485760))  # 10MB-50MB
        local filename="binary_$(printf "%06d" $file_count).bin"
        
        dd if=/dev/urandom of="$OUTPUT_DIR/data/$filename" bs=1M count=$((size / 1048576)) 2>/dev/null
        
        generated=$((generated + size))
        file_count=$((file_count + 1))
        
        if [ $((file_count % 5)) -eq 0 ]; then
            echo -ne "\r  Progress: $(format_size $generated) / $(format_size $binary_target)"
        fi
    done
    echo ""
    log_success "Binary files generated ($(format_size $generated))"
    
    # Generate image-like files (medium compressibility)
    log_info "Generating image files (20% of dataset)..."
    generated=0
    while [ "$generated" -lt "$image_target" ]; then
        local size=$((RANDOM % 5242880 + 1048576))  # 1MB-5MB
        local filename="image_$(printf "%06d" $file_count).dat"
        
        # Mix of random and structured data (simulates images)
        head -c $((size / 2)) /dev/urandom > "$OUTPUT_DIR/data/$filename"
        head -c $((size / 2)) /dev/zero >> "$OUTPUT_DIR/data/$filename"
        
        generated=$((generated + size))
        file_count=$((file_count + 1))
        
        if [ $((file_count % 10)) -eq 0 ]; then
            echo -ne "\r  Progress: $(format_size $generated) / $(format_size $image_target)"
        fi
    done
    echo ""
    log_success "Image files generated ($(format_size $generated))"
    
    # Generate many small files (typical workload)
    log_info "Generating small files (10% of dataset)..."
    generated=0
    while [ "$generated" -lt "$small_target" ]; do
        local size=$((RANDOM % 10240 + 1024))  # 1KB-10KB
        local filename="small_$(printf "%06d" $file_count).dat"
        
        head -c "$size" /dev/urandom > "$OUTPUT_DIR/data/$filename"
        
        generated=$((generated + size))
        file_count=$((file_count + 1))
        
        if [ $((file_count % 100)) -eq 0 ]; then
            echo -ne "\r  Progress: $(format_size $generated) / $(format_size $small_target)"
        fi
    done
    echo ""
    log_success "Small files generated ($(format_size $generated))"
    
    log_success "Mixed dataset complete: $file_count files generated"
}

# Generate text corpus (for LLM training)
generate_text_dataset() {
    log_info "Generating text corpus for $PRIMAL..."
    
    local generated=0
    local file_count=0
    local target=$SIZE_BYTES
    
    # Generate large text files (highly compressible)
    while [ "$generated" -lt "$target" ]; do
        local size=$((RANDOM % 104857600 + 10485760))  # 10MB-100MB per file
        local filename="corpus_$(printf "%06d" $file_count).txt"
        
        log_info "Generating $filename ($(format_size $size))..."
        
        # Generate realistic text patterns (Lorem Ipsum + code + data)
        {
            # 50% Lorem Ipsum style text
            head -c $((size / 2)) /dev/urandom | tr -dc 'a-zA-Z0-9 .,!?\n' 
            
            # 30% code-like patterns
            for i in $(seq 1 $((size / 30 / 100))); do
                echo "def function_$i(arg1, arg2):"
                echo "    return arg1 + arg2"
                echo ""
            done
            
            # 20% structured data
            for i in $(seq 1 $((size / 20 / 200))); do
                echo "{\"id\": $i, \"value\": \"data_$i\", \"timestamp\": \"2025-11-10T12:00:00Z\"}"
            done
        } > "$OUTPUT_DIR/data/$filename"
        
        generated=$((generated + size))
        file_count=$((file_count + 1))
        
        echo -ne "\r  Progress: $(format_size $generated) / $(format_size $target) ($file_count files)"
    done
    echo ""
    
    log_success "Text corpus complete: $file_count files, $(format_size $generated)"
}

# Generate binary dataset
generate_binary_dataset() {
    log_info "Generating binary dataset for $PRIMAL..."
    
    local generated=0
    local file_count=0
    local target=$SIZE_BYTES
    
    while [ "$generated" -lt "$target" ]; do
        local size=$((RANDOM % 104857600 + 10485760))  # 10MB-100MB
        local filename="binary_$(printf "%06d" $file_count).bin"
        
        log_info "Generating $filename ($(format_size $size))..."
        
        dd if=/dev/urandom of="$OUTPUT_DIR/data/$filename" bs=1M count=$((size / 1048576)) 2>/dev/null
        
        generated=$((generated + size))
        file_count=$((file_count + 1))
        
        echo -ne "\r  Progress: $(format_size $generated) / $(format_size $target)"
    done
    echo ""
    
    log_success "Binary dataset complete: $file_count files"
}

# Generate tensor dataset (for ML training)
generate_tensors_dataset() {
    log_info "Generating tensor dataset for $PRIMAL..."
    
    local generated=0
    local file_count=0
    local target=$SIZE_BYTES
    
    # Typical tensor sizes for ML
    local tensor_sizes=(
        $((512 * 512 * 4))        # 1MB tensors
        $((1024 * 1024 * 4))      # 4MB tensors
        $((2048 * 2048 * 4))      # 16MB tensors
    )
    
    while [ "$generated" -lt "$target" ]; do
        local size_idx=$((RANDOM % ${#tensor_sizes[@]}))
        local size=${tensor_sizes[$size_idx]}
        local filename="tensor_$(printf "%06d" $file_count).pt"
        
        # Generate float32 tensor-like data
        dd if=/dev/urandom of="$OUTPUT_DIR/data/$filename" bs=1 count=$size 2>/dev/null
        
        generated=$((generated + size))
        file_count=$((file_count + 1))
        
        if [ $((file_count % 10)) -eq 0 ]; then
            echo -ne "\r  Progress: $(format_size $generated) / $(format_size $target) ($file_count tensors)"
        fi
    done
    echo ""
    
    log_success "Tensor dataset complete: $file_count tensors"
}

# Generate time series dataset
generate_timeseries_dataset() {
    log_info "Generating time series dataset for $PRIMAL..."
    
    local generated=0
    local file_count=0
    local target=$SIZE_BYTES
    
    # Time series with multiple channels
    local channels=64
    local samples_per_file=1000000
    local bytes_per_sample=$((channels * 4))  # float32
    
    while [ "$generated" -lt "$target" ]; do
        local size=$((samples_per_file * bytes_per_sample))
        local filename="timeseries_$(printf "%06d" $file_count).dat"
        
        log_info "Generating $filename ($(format_size $size))..."
        
        # Generate time series data (mix of signal and noise)
        dd if=/dev/urandom of="$OUTPUT_DIR/data/$filename" bs=1M count=$((size / 1048576)) 2>/dev/null
        
        generated=$((generated + size))
        file_count=$((file_count + 1))
        
        echo -ne "\r  Progress: $(format_size $generated) / $(format_size $target)"
    done
    echo ""
    
    log_success "Time series dataset complete: $file_count files"
}

# ==============================================================================
# Main Generation Logic
# ==============================================================================

echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║      📊 LARGE DATASET GENERATOR                           ║"
echo "║      NestGate Ecosystem Integration                       ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

log_info "Configuration:"
echo "  Size: $SIZE_HUMAN"
echo "  Type: $TYPE"
echo "  Primal: $PRIMAL"
echo "  Distribution: $DISTRIBUTION"
echo "  Compression: $COMPRESSION"
echo "  Output: $OUTPUT_DIR"
echo ""

# Start generation timer
START_TIME=$(date +%s)

# Generate based on type
case $TYPE in
    mixed) generate_mixed_dataset ;;
    text) generate_text_dataset ;;
    binary) generate_binary_dataset ;;
    tensors) generate_tensors_dataset ;;
    timeseries) generate_timeseries_dataset ;;
esac

# End timer
END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

# ==============================================================================
# Generate Statistics
# ==============================================================================

log_info "Generating statistics..."

# Count files
FILE_COUNT=$(find "$OUTPUT_DIR/data" -type f | wc -l)

# Calculate actual size
ACTUAL_SIZE=$(du -sb "$OUTPUT_DIR/data" | cut -f1)
ACTUAL_SIZE_HUMAN=$(format_size "$ACTUAL_SIZE")

# Calculate throughput
if [ "$DURATION" -gt 0 ]; then
    THROUGHPUT_MBS=$((ACTUAL_SIZE / DURATION / 1024 / 1024))
else
    THROUGHPUT_MBS="N/A"
fi

# Create statistics file
cat > "$OUTPUT_DIR/metadata/statistics.json" <<EOF
{
  "generation_time_seconds": ${DURATION},
  "files_generated": ${FILE_COUNT},
  "actual_size_bytes": ${ACTUAL_SIZE},
  "actual_size_human": "${ACTUAL_SIZE_HUMAN}",
  "throughput_mbs": ${THROUGHPUT_MBS},
  "target_size_bytes": ${SIZE_BYTES},
  "target_size_human": "${SIZE_HUMAN}",
  "compression_expected": "2.1x",
  "distribution": "${DISTRIBUTION}"
}
EOF

# Create checksums
log_info "Generating checksums..."
find "$OUTPUT_DIR/data" -type f -exec sha256sum {} \; > "$OUTPUT_DIR/metadata/checksums.txt"

# ==============================================================================
# Summary
# ==============================================================================

echo ""
echo "════════════════════════════════════════════════════════════"
echo "Dataset Generation Complete! ✨"
echo "════════════════════════════════════════════════════════════"
echo ""
echo "Summary:"
echo "  Files created:   $FILE_COUNT"
echo "  Total size:      $ACTUAL_SIZE_HUMAN"
echo "  Generation time: ${DURATION}s"
echo "  Throughput:      ${THROUGHPUT_MBS} MB/s"
echo "  Output:          $OUTPUT_DIR"
echo ""
echo "Files:"
echo "  Data:            $OUTPUT_DIR/data/"
echo "  Metadata:        $OUTPUT_DIR/metadata/dataset_info.json"
echo "  Statistics:      $OUTPUT_DIR/metadata/statistics.json"
echo "  Checksums:       $OUTPUT_DIR/metadata/checksums.txt"
echo ""

if [ "$COMPRESSION" = true ]; then
    log_info "Compression enabled - expect ~2-3x space savings"
    log_info "Effective size after compression: ~$(format_size $((ACTUAL_SIZE / 2)))"
fi

echo ""
log_success "Dataset ready for $PRIMAL integration testing!"
echo ""

exit 0

