#!/bin/bash
# Live Demo: Energy Cost Comparison
#
# Compares energy/time costs of different transport strategies

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m'

echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║     ⚡ Energy Analysis: Compression vs Transfer           ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUTPUT_DIR="$SCRIPT_DIR/output"
mkdir -p "$OUTPUT_DIR"

# Create test data (1MB for quick demo, scale up for production)
TEST_SIZE_MB=1
TEST_FILE="$OUTPUT_DIR/test-data.txt"

echo -e "${BLUE}📄 Creating test data (${TEST_SIZE_MB}MB compressible text)...${NC}"

# Generate compressible text (genomic-like patterns)
python3 << EOF > "$TEST_FILE"
import random
import sys

# Genomic sequences have patterns (compressible)
bases = ['A', 'T', 'C', 'G']
patterns = [
    'ATCGATCG',
    'GCTAGCTA',
    'TTAATTAA',
    'CCGGCCGG',
]

size_bytes = $TEST_SIZE_MB * 1024 * 1024
generated = 0

while generated < size_bytes:
    # 70% patterns, 30% random (mimics real genomic data)
    if random.random() < 0.7:
        pattern = random.choice(patterns)
        sys.stdout.write(pattern)
        generated += len(pattern)
    else:
        base = random.choice(bases)
        sys.stdout.write(base)
        generated += 1
EOF

ACTUAL_SIZE=$(stat -f%z "$TEST_FILE" 2>/dev/null || stat -c%s "$TEST_FILE")
ACTUAL_SIZE_MB=$(echo "scale=2; $ACTUAL_SIZE / 1048576" | bc)

echo -e "${GREEN}✅ Created: ${ACTUAL_SIZE_MB}MB${NC}"
echo ""

# Calculate entropy
echo -e "${BLUE}📊 Analyzing data...${NC}"
ENTROPY=$(python3 << EOF
import math
from collections import Counter

with open("$TEST_FILE", "rb") as f:
    data = f.read()

if len(data) == 0:
    print(0.0)
else:
    counts = Counter(data)
    entropy = -sum(count/len(data) * math.log2(count/len(data)) 
                   for count in counts.values())
    print(f"{entropy:.2f}")
EOF
)

echo "   Entropy: $ENTROPY bits/byte"
echo "   (2.0 = highly compressible, 8.0 = random)"
echo ""

# Strategy 1: Uncompressed Transfer
echo -e "${CYAN}═══════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}Strategy 1: Uncompressed Transfer${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════════${NC}"

START_TIME=$(date +%s%N)
# Simulate network transfer (copy file)
cp "$TEST_FILE" "$OUTPUT_DIR/uncompressed-received.txt"
END_TIME=$(date +%s%N)

TRANSFER_TIME_NS=$((END_TIME - START_TIME))
TRANSFER_TIME_MS=$(echo "scale=2; $TRANSFER_TIME_NS / 1000000" | bc)

# Energy estimation (0.004 kWh/GB for internet)
ENERGY_TRANSFER=$(echo "scale=6; $ACTUAL_SIZE_MB * 0.004 / 1024" | bc)

echo -e "${BLUE}Results:${NC}"
echo "   Data size:       ${ACTUAL_SIZE_MB}MB"
echo "   Transfer time:   ${TRANSFER_TIME_MS}ms"
echo "   Energy cost:     ${ENERGY_TRANSFER} kWh"
echo ""

# Strategy 2: Compress → Transfer → Decompress (Zstd-6)
echo -e "${CYAN}═══════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}Strategy 2: Compress → Transfer → Decompress (Zstd-6)${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════════${NC}"

# Compression
echo -e "${BLUE}Step 1: Compress...${NC}"
START_COMPRESS=$(date +%s%N)
zstd -6 -q "$TEST_FILE" -o "$OUTPUT_DIR/compressed.zst"
END_COMPRESS=$(date +%s%N)

COMPRESS_TIME_NS=$((END_COMPRESS - START_COMPRESS))
COMPRESS_TIME_MS=$(echo "scale=2; $COMPRESS_TIME_NS / 1000000" | bc)
COMPRESSED_SIZE=$(stat -f%z "$OUTPUT_DIR/compressed.zst" 2>/dev/null || stat -c%s "$OUTPUT_DIR/compressed.zst")
COMPRESSED_SIZE_MB=$(echo "scale=2; $COMPRESSED_SIZE / 1048576" | bc)
RATIO=$(echo "scale=2; $ACTUAL_SIZE / $COMPRESSED_SIZE" | bc)

echo "   Compressed:  ${COMPRESSED_SIZE_MB}MB (${RATIO}:1)"
echo "   Time:        ${COMPRESS_TIME_MS}ms"

# Energy estimation (0.0002 kWh/GB for Zstd-6)
ENERGY_COMPRESS=$(echo "scale=6; $ACTUAL_SIZE_MB * 0.0002 / 1024" | bc)

# Transfer
echo -e "${BLUE}Step 2: Transfer compressed...${NC}"
START_TRANSFER=$(date +%s%N)
cp "$OUTPUT_DIR/compressed.zst" "$OUTPUT_DIR/compressed-received.zst"
END_TRANSFER=$(date +%s%N)

TRANSFER2_TIME_NS=$((END_TRANSFER - START_TRANSFER))
TRANSFER2_TIME_MS=$(echo "scale=2; $TRANSFER2_TIME_NS / 1000000" | bc)

echo "   Time:        ${TRANSFER2_TIME_MS}ms"

ENERGY_TRANSFER2=$(echo "scale=6; $COMPRESSED_SIZE_MB * 0.004 / 1024" | bc)

# Decompress
echo -e "${BLUE}Step 3: Decompress...${NC}"
START_DECOMPRESS=$(date +%s%N)
zstd -d -q "$OUTPUT_DIR/compressed-received.zst" -o "$OUTPUT_DIR/decompressed.txt"
END_DECOMPRESS=$(date +%s%N)

DECOMPRESS_TIME_NS=$((END_DECOMPRESS - START_DECOMPRESS))
DECOMPRESS_TIME_MS=$(echo "scale=2; $DECOMPRESS_TIME_NS / 1000000" | bc)

echo "   Time:        ${DECOMPRESS_TIME_MS}ms"

# Energy estimation (0.00002 kWh/GB for decompression)
ENERGY_DECOMPRESS=$(echo "scale=6; $ACTUAL_SIZE_MB * 0.00002 / 1024" | bc)

# Total
TOTAL_TIME_MS=$(echo "scale=2; $COMPRESS_TIME_MS + $TRANSFER2_TIME_MS + $DECOMPRESS_TIME_MS" | bc)
ENERGY_TOTAL2=$(echo "scale=6; $ENERGY_COMPRESS + $ENERGY_TRANSFER2 + $ENERGY_DECOMPRESS" | bc)

echo ""
echo -e "${GREEN}Summary:${NC}"
echo "   Total time:      ${TOTAL_TIME_MS}ms"
echo "   Energy cost:     ${ENERGY_TOTAL2} kWh"
echo ""

# Strategy 3: Compress → Transfer → Decompress (LZ4)
echo -e "${CYAN}═══════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}Strategy 3: Compress → Transfer → Decompress (LZ4)${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════════${NC}"

# Compression
echo -e "${BLUE}Step 1: Compress (LZ4)...${NC}"
START_LZ4=$(date +%s%N)
lz4 -q "$TEST_FILE" "$OUTPUT_DIR/compressed.lz4"
END_LZ4=$(date +%s%N)

LZ4_COMPRESS_TIME_NS=$((END_LZ4 - START_LZ4))
LZ4_COMPRESS_TIME_MS=$(echo "scale=2; $LZ4_COMPRESS_TIME_NS / 1000000" | bc)
LZ4_SIZE=$(stat -f%z "$OUTPUT_DIR/compressed.lz4" 2>/dev/null || stat -c%s "$OUTPUT_DIR/compressed.lz4")
LZ4_SIZE_MB=$(echo "scale=2; $LZ4_SIZE / 1048576" | bc)
LZ4_RATIO=$(echo "scale=2; $ACTUAL_SIZE / $LZ4_SIZE" | bc)

echo "   Compressed:  ${LZ4_SIZE_MB}MB (${LZ4_RATIO}:1)"
echo "   Time:        ${LZ4_COMPRESS_TIME_MS}ms"

# Energy (0.00005 kWh/GB for LZ4)
ENERGY_LZ4_COMPRESS=$(echo "scale=6; $ACTUAL_SIZE_MB * 0.00005 / 1024" | bc)

# Transfer
START_LZ4_TRANSFER=$(date +%s%N)
cp "$OUTPUT_DIR/compressed.lz4" "$OUTPUT_DIR/compressed-received.lz4"
END_LZ4_TRANSFER=$(date +%s%N)

LZ4_TRANSFER_TIME_NS=$((END_LZ4_TRANSFER - START_LZ4_TRANSFER))
LZ4_TRANSFER_TIME_MS=$(echo "scale=2; $LZ4_TRANSFER_TIME_NS / 1000000" | bc)

ENERGY_LZ4_TRANSFER=$(echo "scale=6; $LZ4_SIZE_MB * 0.004 / 1024" | bc)

# Decompress
START_LZ4_DECOMPRESS=$(date +%s%N)
lz4 -d -q "$OUTPUT_DIR/compressed-received.lz4" "$OUTPUT_DIR/decompressed-lz4.txt"
END_LZ4_DECOMPRESS=$(date +%s%N)

LZ4_DECOMPRESS_TIME_NS=$((END_LZ4_DECOMPRESS - START_LZ4_DECOMPRESS))
LZ4_DECOMPRESS_TIME_MS=$(echo "scale=2; $LZ4_DECOMPRESS_TIME_NS / 1000000" | bc)

ENERGY_LZ4_DECOMPRESS=$(echo "scale=6; $ACTUAL_SIZE_MB * 0.00001 / 1024" | bc)

# Total
LZ4_TOTAL_TIME_MS=$(echo "scale=2; $LZ4_COMPRESS_TIME_MS + $LZ4_TRANSFER_TIME_MS + $LZ4_DECOMPRESS_TIME_MS" | bc)
ENERGY_LZ4_TOTAL=$(echo "scale=6; $ENERGY_LZ4_COMPRESS + $ENERGY_LZ4_TRANSFER + $ENERGY_LZ4_DECOMPRESS" | bc)

echo ""
echo -e "${GREEN}Summary:${NC}"
echo "   Total time:      ${LZ4_TOTAL_TIME_MS}ms"
echo "   Energy cost:     ${ENERGY_LZ4_TOTAL} kWh"
echo ""

# Final Comparison
echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║                    FINAL COMPARISON                        ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

echo -e "${BLUE}Strategy 1: Uncompressed${NC}"
echo "   Time:        ${TRANSFER_TIME_MS}ms"
echo "   Energy:      ${ENERGY_TRANSFER} kWh"
echo "   Bandwidth:   ${ACTUAL_SIZE_MB}MB"
echo ""

echo -e "${BLUE}Strategy 2: Zstd-6 (Balanced)${NC}"
echo "   Time:        ${TOTAL_TIME_MS}ms"
echo "   Energy:      ${ENERGY_TOTAL2} kWh"
echo "   Bandwidth:   ${COMPRESSED_SIZE_MB}MB"
ZSTD_ENERGY_SAVINGS=$(echo "scale=1; 100 * ($ENERGY_TRANSFER - $ENERGY_TOTAL2) / $ENERGY_TRANSFER" | bc)
echo -e "   ${GREEN}Savings:     ${ZSTD_ENERGY_SAVINGS}% energy${NC}"
echo ""

echo -e "${BLUE}Strategy 3: LZ4 (Fast)${NC}"
echo "   Time:        ${LZ4_TOTAL_TIME_MS}ms"
echo "   Energy:      ${ENERGY_LZ4_TOTAL} kWh"
echo "   Bandwidth:   ${LZ4_SIZE_MB}MB"
LZ4_ENERGY_SAVINGS=$(echo "scale=1; 100 * ($ENERGY_TRANSFER - $ENERGY_LZ4_TOTAL) / $ENERGY_TRANSFER" | bc)
echo -e "   ${GREEN}Savings:     ${LZ4_ENERGY_SAVINGS}% energy${NC}"
echo ""

# Determine winner
echo -e "${MAGENTA}🏆 Recommendation:${NC}"
echo ""

if (( $(echo "$ENTROPY > 7.5" | bc -l) )); then
    echo -e "${YELLOW}   ⚠️  High entropy data (${ENTROPY})${NC}"
    echo "   → Strategy 1: Uncompressed (skip compression)"
    echo "   → Compression would waste energy"
elif (( $(echo "$LZ4_SIZE_MB < $COMPRESSED_SIZE_MB" | bc -l) )); then
    echo -e "${GREEN}   ✅ Strategy 3: LZ4${NC}"
    echo "   → Fastest compression/decompression"
    echo "   → Good ratio (${LZ4_RATIO}:1)"
    echo "   → ${LZ4_ENERGY_SAVINGS}% energy savings"
else
    echo -e "${GREEN}   ✅ Strategy 2: Zstd-6${NC}"
    echo "   → Best compression ratio (${RATIO}:1)"
    echo "   → ${ZSTD_ENERGY_SAVINGS}% energy savings"
    echo "   → Balanced speed/compression"
fi

echo ""
echo -e "${CYAN}Output saved to: $OUTPUT_DIR${NC}"

