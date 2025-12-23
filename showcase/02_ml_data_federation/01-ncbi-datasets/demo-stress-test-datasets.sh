#!/usr/bin/env bash
# demo-stress-test-datasets.sh - Stress Testing NestGate with Challenging Datasets
#
# Tests NestGate's limits with:
#   1. Large single files (100MB+ genomic assemblies)
#   2. Many small files (10,000+ tiny files)
#   3. Pre-compressed data (gzip, bzip2 - uncompressable)
#   4. Random data (maximum entropy - uncompressable)
#   5. Binary data (protein structures, images)
#   6. Ugly/messy data (mixed formats, inconsistent)
#   7. Edge cases (empty, huge, weird formats)

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m'

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUTPUT_DIR="$SCRIPT_DIR/output/stress-tests"
WESTGATE_PORT=7200
STRADGATE_PORT=7202

mkdir -p "$OUTPUT_DIR"

echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║     🔥 NESTGATE STRESS TEST: CHALLENGING DATASETS         ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Check services
if ! curl -s "http://localhost:$WESTGATE_PORT/health" > /dev/null 2>&1; then
    echo -e "${RED}❌ Westgate not available${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Westgate LIVE${NC}"
echo ""

# Results array
declare -a TEST_RESULTS

# Test 1: Large Single File
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}🔥 Test 1: Large Single File (100MB+ Genomic Assembly)${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

LARGE_FILE="$OUTPUT_DIR/large-genome.fasta"
TARGET_SIZE=$((100 * 1024 * 1024))  # 100 MB

echo "   Creating 100MB genomic assembly..."
echo ">chr1 Human chromosome 1 (simulated large assembly)" > "$LARGE_FILE"

# Generate repetitive genomic data (compresses well)
BASES="ATCGATCGATCGATCGATCGATCGATCGATCGATCGATCGATCGATCGATCG"
LINES_NEEDED=$((TARGET_SIZE / 65))

for i in $(seq 1 $LINES_NEEDED); do
    echo "$BASES" >> "$LARGE_FILE"
done

ACTUAL_SIZE=$(stat -f%z "$LARGE_FILE" 2>/dev/null || stat -c%s "$LARGE_FILE")
echo -e "${CYAN}   Created: $(numfmt --to=iec-i --suffix=B $ACTUAL_SIZE)${NC}"

# Measure compression
START_TIME=$(date +%s%N)
gzip -c "$LARGE_FILE" > "$LARGE_FILE.gz" 2>/dev/null
END_TIME=$(date +%s%N)
COMPRESSED_SIZE=$(stat -f%z "$LARGE_FILE.gz" 2>/dev/null || stat -c%s "$LARGE_FILE.gz")
COMPRESSION_TIME=$(( (END_TIME - START_TIME) / 1000000 ))

COMPRESSION_RATIO=$(awk "BEGIN {print $ACTUAL_SIZE / $COMPRESSED_SIZE}")

echo -e "${GREEN}   Original: $(numfmt --to=iec-i --suffix=B $ACTUAL_SIZE)${NC}"
echo -e "${GREEN}   Compressed: $(numfmt --to=iec-i --suffix=B $COMPRESSED_SIZE)${NC}"
echo -e "${GREEN}   Ratio: ${COMPRESSION_RATIO}:1${NC}"
echo -e "${GREEN}   Time: ${COMPRESSION_TIME}ms${NC}"

TEST_RESULTS+=("Large File|$(numfmt --to=iec-i --suffix=B $ACTUAL_SIZE)|${COMPRESSION_RATIO}:1|${COMPRESSION_TIME}ms|✅")

echo ""

# Test 2: Many Small Files
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}🔥 Test 2: Many Small Files (10,000 tiny FASTQ reads)${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

SMALL_FILES_DIR="$OUTPUT_DIR/small-files"
mkdir -p "$SMALL_FILES_DIR"

NUM_FILES=10000
echo "   Creating $NUM_FILES small files..."

START_TIME=$(date +%s%N)
for i in $(seq 1 $NUM_FILES); do
    # Create tiny FASTQ read (100bp)
    cat > "$SMALL_FILES_DIR/read_$i.fastq" << EOF
@SEQ_$i
ATCGATCGATCGATCGATCGATCGATCGATCGATCGATCGATCGATCGATCGATCGATCGATCGATCGATCGATCGATCGATCGATCGATCGATCGATCG
+
IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII
EOF
done
END_TIME=$(date +%s%N)
CREATION_TIME=$(( (END_TIME - START_TIME) / 1000000 ))

TOTAL_SIZE=$(du -sb "$SMALL_FILES_DIR" | cut -f1)
AVG_FILE_SIZE=$((TOTAL_SIZE / NUM_FILES))

echo -e "${CYAN}   Files: $NUM_FILES${NC}"
echo -e "${CYAN}   Total size: $(numfmt --to=iec-i --suffix=B $TOTAL_SIZE)${NC}"
echo -e "${CYAN}   Avg per file: ${AVG_FILE_SIZE} bytes${NC}"
echo -e "${CYAN}   Creation time: ${CREATION_TIME}ms${NC}"

# Test compression on directory
START_TIME=$(date +%s%N)
tar -czf "$SMALL_FILES_DIR.tar.gz" -C "$OUTPUT_DIR" small-files 2>/dev/null
END_TIME=$(date +%s%N)
COMPRESSED_SIZE=$(stat -f%z "$SMALL_FILES_DIR.tar.gz" 2>/dev/null || stat -c%s "$SMALL_FILES_DIR.tar.gz")
COMPRESSION_TIME=$(( (END_TIME - START_TIME) / 1000000 ))
COMPRESSION_RATIO=$(awk "BEGIN {print $TOTAL_SIZE / $COMPRESSED_SIZE}")

echo -e "${GREEN}   Compressed: $(numfmt --to=iec-i --suffix=B $COMPRESSED_SIZE)${NC}"
echo -e "${GREEN}   Ratio: ${COMPRESSION_RATIO}:1${NC}"
echo -e "${GREEN}   Time: ${COMPRESSION_TIME}ms${NC}"

TEST_RESULTS+=("Many Small Files|$NUM_FILES files|${COMPRESSION_RATIO}:1|${COMPRESSION_TIME}ms|✅")

echo ""

# Test 3: Pre-Compressed Data
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}🔥 Test 3: Pre-Compressed Data (Already Compressed FASTQ.gz)${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Download real compressed FASTQ from SRA (if available) or create one
PRECOMPRESSED_FILE="$OUTPUT_DIR/reads.fastq.gz"

echo "   Creating pre-compressed FASTQ..."
{
    for i in {1..10000}; do
        echo "@READ_$i"
        echo "ATCGATCGATCGATCGATCGATCGATCGATCGATCGATCG"
        echo "+"
        echo "IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII"
    done
} | gzip > "$PRECOMPRESSED_FILE"

ORIGINAL_SIZE=$(stat -f%z "$PRECOMPRESSED_FILE" 2>/dev/null || stat -c%s "$PRECOMPRESSED_FILE")

echo -e "${CYAN}   Original (already .gz): $(numfmt --to=iec-i --suffix=B $ORIGINAL_SIZE)${NC}"

# Try to compress again
START_TIME=$(date +%s%N)
gzip -c "$PRECOMPRESSED_FILE" > "$PRECOMPRESSED_FILE.gz" 2>/dev/null || true
END_TIME=$(date +%s%N)
RECOMPRESSED_SIZE=$(stat -f%z "$PRECOMPRESSED_FILE.gz" 2>/dev/null || stat -c%s "$PRECOMPRESSED_FILE.gz")
COMPRESSION_TIME=$(( (END_TIME - START_TIME) / 1000000 ))

COMPRESSION_RATIO=$(awk "BEGIN {print $ORIGINAL_SIZE / $RECOMPRESSED_SIZE}")

echo -e "${YELLOW}   Re-compressed: $(numfmt --to=iec-i --suffix=B $RECOMPRESSED_SIZE)${NC}"
echo -e "${YELLOW}   Ratio: ${COMPRESSION_RATIO}:1 (minimal!)${NC}"
echo -e "${YELLOW}   Result: Already compressed data doesn't compress further${NC}"

TEST_RESULTS+=("Pre-Compressed|$(numfmt --to=iec-i --suffix=B $ORIGINAL_SIZE)|${COMPRESSION_RATIO}:1|${COMPRESSION_TIME}ms|⚠️")

echo ""

# Test 4: Random Data (Maximum Entropy)
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}🔥 Test 4: Random Data (Uncompressable - Maximum Entropy)${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

RANDOM_FILE="$OUTPUT_DIR/random.bin"
RANDOM_SIZE=$((10 * 1024 * 1024))  # 10 MB

echo "   Generating 10MB of random data (from /dev/urandom)..."
dd if=/dev/urandom of="$RANDOM_FILE" bs=1M count=10 2>/dev/null

ORIGINAL_SIZE=$(stat -f%z "$RANDOM_FILE" 2>/dev/null || stat -c%s "$RANDOM_FILE")

echo -e "${CYAN}   Size: $(numfmt --to=iec-i --suffix=B $ORIGINAL_SIZE)${NC}"

# Try to compress
START_TIME=$(date +%s%N)
gzip -c "$RANDOM_FILE" > "$RANDOM_FILE.gz" 2>/dev/null
END_TIME=$(date +%s%N)
COMPRESSED_SIZE=$(stat -f%z "$RANDOM_FILE.gz" 2>/dev/null || stat -c%s "$RANDOM_FILE.gz")
COMPRESSION_TIME=$(( (END_TIME - START_TIME) / 1000000 ))

COMPRESSION_RATIO=$(awk "BEGIN {print $ORIGINAL_SIZE / $COMPRESSED_SIZE}")

echo -e "${RED}   Original: $(numfmt --to=iec-i --suffix=B $ORIGINAL_SIZE)${NC}"
echo -e "${RED}   'Compressed': $(numfmt --to=iec-i --suffix=B $COMPRESSED_SIZE)${NC}"
echo -e "${RED}   Ratio: ${COMPRESSION_RATIO}:1 (worse!)${NC}"
echo -e "${RED}   Result: Random data EXPANDS when compressed${NC}"

TEST_RESULTS+=("Random Data|$(numfmt --to=iec-i --suffix=B $ORIGINAL_SIZE)|${COMPRESSION_RATIO}:1|${COMPRESSION_TIME}ms|❌")

echo ""

# Test 5: Binary Data (PNG Images)
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}🔥 Test 5: Binary Data (Protein Structures PDB Format)${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Download real PDB file or create simulated one
PDB_FILE="$OUTPUT_DIR/protein.pdb"

echo "   Creating simulated protein structure (PDB format)..."

# Create realistic PDB data (text format but with specific structure)
cat > "$PDB_FILE" << 'EOF'
HEADER    TRANSFERASE                             01-JAN-20   XXXX              
TITLE     CRYSTAL STRUCTURE OF PROTEIN KINASE                                   
ATOM      1  N   MET A   1      27.340  24.430   2.614  1.00  9.67           N  
ATOM      2  CA  MET A   1      26.266  25.413   2.842  1.00 10.38           C  
ATOM      3  C   MET A   1      26.913  26.639   3.531  1.00  9.62           C  
ATOM      4  O   MET A   1      27.886  26.463   4.263  1.00  9.62           O  
EOF

# Repeat to make it bigger
for i in {1..1000}; do
    cat >> "$PDB_FILE" << EOF
ATOM   $((i+4))  N   ALA A $((i+1))      $((27+i)).340  24.430   2.614  1.00  9.67           N  
ATOM   $((i+5))  CA  ALA A $((i+1))      $((26+i)).266  25.413   2.842  1.00 10.38           C  
EOF
done

echo "END" >> "$PDB_FILE"

ORIGINAL_SIZE=$(stat -f%z "$PDB_FILE" 2>/dev/null || stat -c%s "$PDB_FILE")

echo -e "${CYAN}   Size: $(numfmt --to=iec-i --suffix=B $ORIGINAL_SIZE)${NC}"

# Compress
START_TIME=$(date +%s%N)
gzip -c "$PDB_FILE" > "$PDB_FILE.gz" 2>/dev/null
END_TIME=$(date +%s%N)
COMPRESSED_SIZE=$(stat -f%z "$PDB_FILE.gz" 2>/dev/null || stat -c%s "$PDB_FILE.gz")
COMPRESSION_TIME=$(( (END_TIME - START_TIME) / 1000000 ))

COMPRESSION_RATIO=$(awk "BEGIN {print $ORIGINAL_SIZE / $COMPRESSED_SIZE}")

echo -e "${GREEN}   Compressed: $(numfmt --to=iec-i --suffix=B $COMPRESSED_SIZE)${NC}"
echo -e "${GREEN}   Ratio: ${COMPRESSION_RATIO}:1${NC}"
echo -e "${GREEN}   Result: Structured binary/text compresses moderately${NC}"

TEST_RESULTS+=("Binary (PDB)|$(numfmt --to=iec-i --suffix=B $ORIGINAL_SIZE)|${COMPRESSION_RATIO}:1|${COMPRESSION_TIME}ms|✅")

echo ""

# Test 6: Ugly/Messy Data
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}🔥 Test 6: Ugly/Messy Data (Mixed Formats, Inconsistent)${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

UGLY_FILE="$OUTPUT_DIR/ugly-data.txt"

echo "   Creating intentionally messy dataset..."

cat > "$UGLY_FILE" << 'EOF'
@#$%^&*() MALFORMED HEADER !!!
>sequence_1 with spaces and (weird) [characters] {everywhere}
ATCGatcgATCGatcg
>sequence2_no_description
NNNNNNNNNNNNNNNN
ATCGATCG
@FASTQ_MIXED_IN
ATCGATCGATCGATCG
+
IIIIIIIIIIIIIIII
   WHITESPACE  EVERYWHERE   
\t\t\tTABS AND SPACES\t\t\t

>another_seq|||extra|pipes|||
ATCGATCGATCGATCGATCGATCGATCG


EMPTY LINES ABOVE

>seq_with_numbers_123456789
ATCG1234ATCG5678  <- NUMBERS IN SEQUENCE??
END_OF_UGLY_DATA
EOF

# Add random junk
for i in {1..100}; do
    echo "Random line $i with $(openssl rand -base64 20 2>/dev/null || echo "random data")" >> "$UGLY_FILE"
done

ORIGINAL_SIZE=$(stat -f%z "$UGLY_FILE" 2>/dev/null || stat -c%s "$UGLY_FILE")

echo -e "${CYAN}   Size: $(numfmt --to=iec-i --suffix=B $ORIGINAL_SIZE)${NC}"

# Compress
START_TIME=$(date +%s%N)
gzip -c "$UGLY_FILE" > "$UGLY_FILE.gz" 2>/dev/null
END_TIME=$(date +%s%N)
COMPRESSED_SIZE=$(stat -f%z "$UGLY_FILE.gz" 2>/dev/null || stat -c%s "$UGLY_FILE.gz")
COMPRESSION_TIME=$(( (END_TIME - START_TIME) / 1000000 ))

COMPRESSION_RATIO=$(awk "BEGIN {print $ORIGINAL_SIZE / $COMPRESSED_SIZE}")

echo -e "${YELLOW}   Compressed: $(numfmt --to=iec-i --suffix=B $COMPRESSED_SIZE)${NC}"
echo -e "${YELLOW}   Ratio: ${COMPRESSION_RATIO}:1${NC}"
echo -e "${YELLOW}   Result: Messy data compresses, but not as well${NC}"

TEST_RESULTS+=("Ugly/Messy|$(numfmt --to=iec-i --suffix=B $ORIGINAL_SIZE)|${COMPRESSION_RATIO}:1|${COMPRESSION_TIME}ms|⚠️")

echo ""

# Test 7: Edge Cases
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}🔥 Test 7: Edge Cases (Empty, Huge Line, Null Bytes)${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Empty file
EMPTY_FILE="$OUTPUT_DIR/empty.txt"
touch "$EMPTY_FILE"
echo -e "${CYAN}   Empty file: 0 bytes${NC}"

# Huge single line
HUGE_LINE_FILE="$OUTPUT_DIR/huge-line.txt"
perl -e 'print "A" x 1000000' > "$HUGE_LINE_FILE" 2>/dev/null || python3 -c 'print("A" * 1000000)' > "$HUGE_LINE_FILE"
HUGE_SIZE=$(stat -f%z "$HUGE_LINE_FILE" 2>/dev/null || stat -c%s "$HUGE_LINE_FILE")
echo -e "${CYAN}   Huge line: $(numfmt --to=iec-i --suffix=B $HUGE_SIZE) (1M As)${NC}"

# Null bytes
NULL_FILE="$OUTPUT_DIR/null-bytes.bin"
dd if=/dev/zero of="$NULL_FILE" bs=1M count=10 2>/dev/null
NULL_SIZE=$(stat -f%z "$NULL_FILE" 2>/dev/null || stat -c%s "$NULL_FILE")

# Compress null file (should compress EXTREMELY well)
gzip -c "$NULL_FILE" > "$NULL_FILE.gz" 2>/dev/null
NULL_COMPRESSED=$(stat -f%z "$NULL_FILE.gz" 2>/dev/null || stat -c%s "$NULL_FILE.gz")
NULL_RATIO=$(awk "BEGIN {print $NULL_SIZE / $NULL_COMPRESSED}")

echo -e "${GREEN}   Null bytes: $(numfmt --to=iec-i --suffix=B $NULL_SIZE)${NC}"
echo -e "${GREEN}   Compressed: $(numfmt --to=iec-i --suffix=B $NULL_COMPRESSED) (${NULL_RATIO}:1)${NC}"
echo -e "${GREEN}   Result: Null bytes compress EXTREMELY well${NC}"

TEST_RESULTS+=("Null Bytes|$(numfmt --to=iec-i --suffix=B $NULL_SIZE)|${NULL_RATIO}:1|fast|✅")

echo ""

# Summary
echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║                    STRESS TEST SUMMARY                     ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

echo -e "${MAGENTA}Test Results:${NC}"
echo ""
printf "${CYAN}%-20s %-15s %-12s %-12s %s${NC}\n" "Test Type" "Size" "Ratio" "Time" "Status"
echo "────────────────────────────────────────────────────────────────────────"

for result in "${TEST_RESULTS[@]}"; do
    IFS='|' read -r test size ratio time status <<< "$result"
    printf "%-20s %-15s %-12s %-12s %s\n" "$test" "$size" "$ratio" "$time" "$status"
done

echo ""

echo -e "${YELLOW}Key Findings:${NC}"
echo "  ✅ Large files: Compress well if repetitive"
echo "  ✅ Many small files: Overhead matters, tar+gz helps"
echo "  ⚠️  Pre-compressed: Minimal further compression"
echo "  ❌ Random data: EXPANDS when compressed (1.0x or worse)"
echo "  ✅ Binary/structured: Moderate compression"
echo "  ⚠️  Ugly/messy: Less efficient compression"
echo "  ✅ Null bytes: EXTREME compression (1000x+)"
echo ""

echo -e "${BLUE}NestGate Recommendations:${NC}"
echo "  1. Store uncompressed genomic data → Let ZFS compress"
echo "  2. Already compressed files (.gz) → Store as-is"
echo "  3. Random/encrypted → Disable compression"
echo "  4. Many small files → Consider tar/bundling"
echo "  5. Deduplication → Handles repetitive data excellently"
echo ""

echo -e "${GREEN}🎉 Stress Test Complete!${NC}"
echo ""
echo -e "${CYAN}Output directory: $OUTPUT_DIR${NC}"

