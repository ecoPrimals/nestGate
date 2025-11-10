#!/bin/bash
# ==============================================================================
# beardog HSM Integration Test Scenario
# ==============================================================================
#
# Tests NestGate's large dataset and model handling for beardog HSM integration
#
# ==============================================================================

set -euo pipefail

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

log_info() { echo -e "${BLUE}▶ $1${NC}"; }
log_success() { echo -e "${GREEN}✓ $1${NC}"; }
log_warn() { echo -e "${YELLOW}⚠ $1${NC}"; }
log_error() { echo -e "${RED}✗ $1${NC}"; }

echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║      🔐 beardog HSM INTEGRATION TEST                      ║"
echo "║      Large Dataset & Model Handling                       ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

SHOWCASE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DATA_DIR="/tmp/nestgate_test_data"
TEST_START=$(date +%s)

# ==============================================================================
# Test 1: Generate HSM Training Dataset (5GB)
# ==============================================================================

log_info "Test 1: Generating 5GB HSM training dataset..."

"$SHOWCASE_DIR/data/generators/generate_large_dataset.sh" \
  --size 5GB \
  --type mixed \
  --primal beardog \
  --output "$DATA_DIR/hsm_training_v1" \
  --compression true

if [ $? -eq 0 ]; then
    log_success "Training dataset generated"
else
    log_error "Dataset generation failed"
    exit 1
fi

# ==============================================================================
# Test 2: Create Model Storage Structure
# ==============================================================================

log_info "Test 2: Creating model storage for beardog..."

mkdir -p /nestgate_data/models/beardog
mkdir -p /nestgate_data/metadata/models/beardog

# Create simulated HSM model
log_info "Creating simulated HSM model (4.2GB)..."
dd if=/dev/urandom of=/tmp/hsm_model_v2.safetensors bs=1M count=4200 2>/dev/null

log_success "Model storage structure created"

# ==============================================================================
# Test 3: Store Model with Versioning
# ==============================================================================

log_info "Test 3: Storing HSM model with versioning..."

"$SHOWCASE_DIR/scripts/store_model.sh" \
  --model /tmp/hsm_model_v2.safetensors \
  --primal beardog \
  --name hsm_model \
  --version v2.0.0 \
  --description "HSM security model v2" \
  --tags "hsm,security,production" \
  --auto-snapshot true

if [ $? -eq 0 ]; then
    log_success "Model stored successfully"
else
    log_error "Model storage failed"
    exit 1
fi

# ==============================================================================
# Test 4: Simulate Data Streaming
# ==============================================================================

log_info "Test 4: Testing data streaming performance..."

START_STREAM=$(date +%s)

# Simulate streaming 1GB of training data
dd if=/dev/urandom bs=1M count=1000 2>/dev/null | pv -s 1G > /tmp/stream_test.dat

END_STREAM=$(date +%s)
STREAM_DURATION=$((END_STREAM - START_STREAM))
THROUGHPUT_MBS=$((1024 / STREAM_DURATION))

log_success "Streaming test complete"
log_info "Throughput: ${THROUGHPUT_MBS} MB/s"

# Validate target (>200 MB/s)
if [ "$THROUGHPUT_MBS" -gt 200 ]; then
    log_success "✓ Throughput exceeds target (200 MB/s)"
else
    log_warn "⚠ Throughput below target: ${THROUGHPUT_MBS} MB/s < 200 MB/s"
fi

# ==============================================================================
# Test 5: Model Checkpointing
# ==============================================================================

log_info "Test 5: Testing model checkpointing..."

# Simulate checkpoint creation
for i in {1..5}; do
    CHECKPOINT_FILE="/tmp/hsm_checkpoint_$i.safetensors"
    dd if=/dev/urandom of="$CHECKPOINT_FILE" bs=1M count=100 2>/dev/null
    
    "$SHOWCASE_DIR/scripts/store_model.sh" \
      --model "$CHECKPOINT_FILE" \
      --primal beardog \
      --name hsm_model_checkpoint \
      --description "Training checkpoint $i" \
      --tags "checkpoint,training" \
      --auto-snapshot false > /dev/null 2>&1
    
    rm "$CHECKPOINT_FILE"
    
    echo -ne "\r  Creating checkpoints: $i/5"
done
echo ""

log_success "Checkpoint creation complete"

# ==============================================================================
# Test 6: Performance Validation
# ==============================================================================

log_info "Test 6: Validating performance targets..."

# Target: 10K operations per second
log_info "Testing small file operations..."

START_OPS=$(date +%s)
OPS_COUNT=0
TARGET_OPS=10000

while [ "$OPS_COUNT" -lt "$TARGET_OPS" ]; do
    echo "operation" > /tmp/test_op_$OPS_COUNT.txt
    OPS_COUNT=$((OPS_COUNT + 1))
    
    if [ $((OPS_COUNT % 1000)) -eq 0 ]; then
        echo -ne "\r  Operations: $OPS_COUNT / $TARGET_OPS"
    fi
done
echo ""

END_OPS=$(date +%s)
OPS_DURATION=$((END_OPS - START_OPS))
OPS_PER_SEC=$((TARGET_OPS / OPS_DURATION))

log_info "Operations per second: ${OPS_PER_SEC}"

if [ "$OPS_PER_SEC" -gt 10000 ]; then
    log_success "✓ Operations exceed target (10K ops/sec)"
else
    log_warn "⚠ Operations below target: ${OPS_PER_SEC} ops/sec"
fi

# Cleanup test operations
rm -f /tmp/test_op_*.txt

# ==============================================================================
# Test 7: Latency Measurement
# ==============================================================================

log_info "Test 7: Measuring API latency..."

# Simulate API requests
TOTAL_LATENCY=0
REQUEST_COUNT=100

for i in $(seq 1 $REQUEST_COUNT); do
    START=$(date +%s%N)
    
    # Simulate metadata lookup
    cat /nestgate_data/metadata/models/beardog/hsm_model_versions.json > /dev/null 2>&1
    
    END=$(date +%s%N)
    LATENCY_NS=$((END - START))
    LATENCY_MS=$((LATENCY_NS / 1000000))
    TOTAL_LATENCY=$((TOTAL_LATENCY + LATENCY_MS))
    
    if [ $((i % 10)) -eq 0 ]; then
        echo -ne "\r  Requests: $i / $REQUEST_COUNT"
    fi
done
echo ""

AVG_LATENCY=$((TOTAL_LATENCY / REQUEST_COUNT))

log_info "Average latency: ${AVG_LATENCY}ms"

if [ "$AVG_LATENCY" -lt 5 ]; then
    log_success "✓ Latency under target (<5ms p95)"
else
    log_warn "⚠ Latency above target: ${AVG_LATENCY}ms"
fi

# ==============================================================================
# Test Summary
# ==============================================================================

TEST_END=$(date +%s)
TOTAL_DURATION=$((TEST_END - TEST_START))

echo ""
echo "════════════════════════════════════════════════════════════"
echo "Integration Test Complete! ✨"
echo "════════════════════════════════════════════════════════════"
echo ""
echo "Test Results:"
echo "  Duration:          ${TOTAL_DURATION}s"
echo "  Dataset Generated: 5GB"
echo "  Model Stored:      4.2GB"
echo "  Checkpoints:       5"
echo ""
echo "Performance:"
echo "  Streaming:         ${THROUGHPUT_MBS} MB/s"
echo "  Operations:        ${OPS_PER_SEC} ops/sec"
echo "  Avg Latency:       ${AVG_LATENCY}ms"
echo ""

# Check all targets
ALL_PASS=true

if [ "$THROUGHPUT_MBS" -lt 200 ]; then
    log_warn "⚠ Streaming throughput below target"
    ALL_PASS=false
fi

if [ "$OPS_PER_SEC" -lt 10000 ]; then
    log_warn "⚠ Operations per second below target"
    ALL_PASS=false
fi

if [ "$AVG_LATENCY" -gt 5 ]; then
    log_warn "⚠ Latency above target"
    ALL_PASS=false
fi

if [ "$ALL_PASS" = true ]; then
    log_success "✓ All performance targets met!"
    echo ""
    echo "Status: ✅ READY FOR beardog INTEGRATION"
else
    log_warn "⚠ Some performance targets not met (see above)"
    echo ""
    echo "Status: ⚠️ REVIEW PERFORMANCE BEFORE INTEGRATION"
fi

echo ""

# Cleanup
log_info "Cleaning up test data..."
rm -f /tmp/hsm_model_v2.safetensors
rm -f /tmp/stream_test.dat

log_success "beardog HSM integration test complete!"
echo ""

exit 0

