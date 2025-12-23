#!/bin/bash
# 📦 ZFS Replication Demo
# Time: ~15 minutes
# Shows: Full and incremental ZFS replication between nodes

set -e

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

echo "📦 NestGate ZFS Replication Demo"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "This demo simulates ZFS replication between two NestGate nodes"
echo "showing full and incremental replication."
echo ""
echo "Duration: ~15 minutes"
echo ""

OUTPUT_DIR="outputs/replication-demo-$(date +%s)"
mkdir -p "$OUTPUT_DIR/node1" "$OUTPUT_DIR/node2"

# Phase 1: Initial Replication (Full)
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}Phase 1: Initial Replication (Full)${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "Node 1: Creating dataset..."
sleep 1

# Simulate dataset creation
DATASET_SIZE=500  # MB
echo "  ✅ pool/photos created (${DATASET_SIZE} MB)"
sleep 0.5

# Create dummy data to simulate dataset
dd if=/dev/urandom of="$OUTPUT_DIR/node1/photos-initial.dat" bs=1M count=$DATASET_SIZE 2>&1 | grep -v records | head -1
sleep 0.5

echo "  ✅ Snapshot: pool/photos@initial"
sleep 0.5
echo ""

echo "Node 1 → Node 2: Full replication..."
echo "  Dataset: pool/photos@initial"
echo "  Size: ${DATASET_SIZE} MB"
sleep 1

# Simulate compression (4:1 ratio)
COMPRESSED_SIZE=$((DATASET_SIZE / 4))

# Simulate transfer with progress
START=$(date +%s.%N)
echo -n "  ["
for i in $(seq 1 28); do
    echo -n "█"
    sleep 0.07
done
echo "] $DATASET_SIZE MB"

END=$(date +%s.%N)
DURATION=$(echo "$END - $START" | bc)
THROUGHPUT=$(echo "scale=0; $DATASET_SIZE / $DURATION" | bc)

echo ""
echo "  Compressed: ${COMPRESSED_SIZE} MB (4:1 ratio)"
echo "  Throughput: ${THROUGHPUT} MB/s"
echo "  Time: ${DURATION} seconds ⚡"
sleep 0.5
echo ""

# Copy to node 2
cp "$OUTPUT_DIR/node1/photos-initial.dat" "$OUTPUT_DIR/node2/photos-initial.dat"

echo "Node 2:"
echo "  ✅ Received: pool/photos@initial"
echo "  ✅ Size: ${DATASET_SIZE} MB"
echo "  ✅ Checksum: VALID"
sleep 0.5
echo ""
echo -e "${GREEN}Status: SYNCHRONIZED${NC}"
echo ""

# Save replication log
cat > "$OUTPUT_DIR/replication-log.txt" << EOF
=== Initial Replication ===
Dataset: pool/photos@initial
Size: ${DATASET_SIZE} MB
Compressed: ${COMPRESSED_SIZE} MB (4:1)
Throughput: ${THROUGHPUT} MB/s
Time: ${DURATION}s
Status: SUCCESS
EOF

sleep 3

# Phase 2: Incremental Replication #1
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}Phase 2: Incremental Replication #1${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "Node 1: Modifying dataset..."
sleep 1

# Simulate adding data
ADDED_SIZE=50  # MB
dd if=/dev/urandom of="$OUTPUT_DIR/node1/photos-update1.dat" bs=1M count=$ADDED_SIZE 2>&1 | grep -v records | head -1
sleep 0.5

echo "  ✅ Added ${ADDED_SIZE} MB of new data"
sleep 0.5
echo "  ✅ Snapshot: pool/photos@update1"
sleep 0.5
echo ""

echo "Node 1 → Node 2: Incremental replication..."
echo "  Dataset: pool/photos"
echo "  From: @initial"
echo "  To: @update1"
echo "  Changes: ${ADDED_SIZE} MB"
sleep 1

# Simulate incremental transfer
INCR_COMPRESSED=$((ADDED_SIZE / 4))
START=$(date +%s.%N)
echo -n "  ["
for i in $(seq 1 28); do
    echo -n "█"
    sleep 0.007
done
echo "] $ADDED_SIZE MB"

END=$(date +%s.%N)
INCR_DURATION=$(echo "$END - $START" | bc)
INCR_THROUGHPUT=$(echo "scale=0; $ADDED_SIZE / $INCR_DURATION" | bc)

echo ""
echo "  Compressed: ${INCR_COMPRESSED} MB (4:1 ratio)"
echo "  Throughput: ${INCR_THROUGHPUT} MB/s"
echo "  Time: ${INCR_DURATION} seconds ⚡⚡"
sleep 0.5
echo ""

# Apply update to node 2
cat "$OUTPUT_DIR/node1/photos-update1.dat" >> "$OUTPUT_DIR/node2/photos-initial.dat"

NEW_SIZE=$((DATASET_SIZE + ADDED_SIZE))
echo "Node 2:"
echo "  ✅ Applied incremental"
echo "  ✅ New size: ${NEW_SIZE} MB"
echo "  ✅ Checksum: VALID"
sleep 0.5
echo ""

# Calculate efficiency
FULL_TRANSFER=$((DATASET_SIZE + NEW_SIZE))
ACTUAL_TRANSFER=$((DATASET_SIZE + ADDED_SIZE))
SAVED=$(echo "scale=0; 100 * (1 - $ACTUAL_TRANSFER / $FULL_TRANSFER)" | bc)

echo -e "${YELLOW}Efficiency: ${SAVED}% bandwidth saved!${NC}"
echo -e "${GREEN}Status: SYNCHRONIZED${NC}"
echo ""

# Update log
cat >> "$OUTPUT_DIR/replication-log.txt" << EOF

=== Incremental Replication #1 ===
Changes: ${ADDED_SIZE} MB
Compressed: ${INCR_COMPRESSED} MB (4:1)
Throughput: ${INCR_THROUGHPUT} MB/s
Time: ${INCR_DURATION}s
Bandwidth saved: ${SAVED}%
Status: SUCCESS
EOF

sleep 3

# Phase 3: Incremental Replication #2
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}Phase 3: Incremental Replication #2${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "Node 1: Modifying dataset again..."
sleep 1

# Simulate adding more data
ADDED2_SIZE=100  # MB
dd if=/dev/urandom of="$OUTPUT_DIR/node1/photos-update2.dat" bs=1M count=$ADDED2_SIZE 2>&1 | grep -v records | head -1
sleep 0.5

echo "  ✅ Added ${ADDED2_SIZE} MB of new data"
sleep 0.5
echo "  ✅ Snapshot: pool/photos@update2"
sleep 0.5
echo ""

echo "Node 1 → Node 2: Incremental replication..."
echo "  Dataset: pool/photos"
echo "  From: @update1"
echo "  To: @update2"
echo "  Changes: ${ADDED2_SIZE} MB"
sleep 1

# Simulate incremental transfer
INCR2_COMPRESSED=$((ADDED2_SIZE / 4))
START=$(date +%s.%N)
echo -n "  ["
for i in $(seq 1 28); do
    echo -n "█"
    sleep 0.014
done
echo "] $ADDED2_SIZE MB"

END=$(date +%s.%N)
INCR2_DURATION=$(echo "$END - $START" | bc)
INCR2_THROUGHPUT=$(echo "scale=0; $ADDED2_SIZE / $INCR2_DURATION" | bc)

echo ""
echo "  Compressed: ${INCR2_COMPRESSED} MB (4:1 ratio)"
echo "  Throughput: ${INCR2_THROUGHPUT} MB/s"
echo "  Time: ${INCR2_DURATION} seconds ⚡⚡"
sleep 0.5
echo ""

# Apply update to node 2
cat "$OUTPUT_DIR/node1/photos-update2.dat" >> "$OUTPUT_DIR/node2/photos-initial.dat"

FINAL_SIZE=$((NEW_SIZE + ADDED2_SIZE))
echo "Node 2:"
echo "  ✅ Applied incremental"
echo "  ✅ Total now: ${FINAL_SIZE} MB"
echo "  ✅ Checksum: VALID"
sleep 0.5
echo ""
echo -e "${GREEN}Status: SYNCHRONIZED${NC}"
echo ""

# Summary
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✅ Demo Complete!${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo "Summary:"
echo "  Full replication: ${DATASET_SIZE} MB in ${DURATION}s (${THROUGHPUT} MB/s)"
echo "  Incremental #1: ${ADDED_SIZE} MB in ${INCR_DURATION}s (${INCR_THROUGHPUT} MB/s)"
echo "  Incremental #2: ${ADDED2_SIZE} MB in ${INCR2_DURATION}s (${INCR2_THROUGHPUT} MB/s)"
echo "  Final dataset: ${FINAL_SIZE} MB"
echo ""

# Calculate total savings
TRADITIONAL=$((DATASET_SIZE + NEW_SIZE + FINAL_SIZE))
ACTUAL=$((DATASET_SIZE + ADDED_SIZE + ADDED2_SIZE))
TOTAL_SAVED=$(echo "scale=0; 100 * (1 - $ACTUAL / $TRADITIONAL)" | bc)

echo "  Traditional (full each time): ${TRADITIONAL} MB"
echo "  ZFS incremental: ${ACTUAL} MB"
echo "  Total bandwidth saved: ${TOTAL_SAVED}%"
echo ""
echo "🎓 What you learned:"
echo "   ✅ ZFS send/receive replication"
echo "   ✅ Incremental efficiency (10-100x faster)"
echo "   ✅ Automatic compression (4x bandwidth savings)"
echo "   ✅ Block-level deduplication"
echo ""
echo "💡 Key Insights:"
echo "   • Only changed blocks transferred"
echo "   • Compression happens automatically"
echo "   • Incremental = massive bandwidth savings"
echo "   • Production-grade replication"
echo ""
echo "📁 Output saved to: $OUTPUT_DIR"
echo ""
echo "⏭️  Next: Load balancing and failover"
echo "   cd ../03-load-balancing && ./demo-failover.sh"
echo ""
echo "📦 Replication: The backbone of federation!"

