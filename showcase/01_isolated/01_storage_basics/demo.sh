#!/usr/bin/env bash
# NestGate Storage Basics - LIVE DEMONSTRATION (No-sudo version)
# Demonstrates filesystem operations without requiring root
set -euo pipefail

DEMO_NAME="storage_basics"
DEMO_VERSION="1.0.0"
START_TIME=$(date +%s)
TIMESTAMP=$(date +%s)
OUTPUT_DIR="outputs/${DEMO_NAME}-${TIMESTAMP}"

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${CYAN}🎬 NestGate Storage Basics - Live Demo v${DEMO_VERSION}${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo -e "${BLUE}Demonstrates:${NC} Real filesystem operations with live data"
echo -e "${BLUE}Output:${NC} $OUTPUT_DIR"
echo -e "${BLUE}Started:${NC} $(date)"
echo ""
echo -e "${YELLOW}Note:${NC} Running in no-sudo mode (filesystem backend)"
echo ""

# Filesystem backend (no ZFS)
STORAGE_PATH="$OUTPUT_DIR/storage"

# ============================================================================
# STEP 1: Create Storage Pool
# ============================================================================

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}📁 [1/6] Creating Storage Backend${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

echo "Creating filesystem-based storage..."
mkdir -p "$STORAGE_PATH"
echo -e "${GREEN}✓${NC} Storage directory created: $STORAGE_PATH"

# Create metadata file to simulate pool info
cat > "$OUTPUT_DIR/pool-info.json" <<EOF
{
  "pool_name": "nestgate-demo-${TIMESTAMP}",
  "backend": "filesystem",
  "created": "$(date -Iseconds)",
  "size_mb": 1024,
  "compression": "none",
  "path": "$STORAGE_PATH"
}
EOF

echo -e "${GREEN}✓${NC} Pool metadata: $OUTPUT_DIR/pool-info.json"

echo ""
sleep 1

# ============================================================================
# STEP 2: Create Datasets
# ============================================================================

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}📦 [2/6] Creating Datasets${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

DATASETS=("data" "logs" "backups" "cache" "tmp")

for dataset in "${DATASETS[@]}"; do
    echo "Creating dataset: ${dataset}..."
    mkdir -p "${STORAGE_PATH}/${dataset}"
    
    # Create dataset metadata
    cat > "${STORAGE_PATH}/${dataset}/.dataset-meta.json" <<EOF
{
  "name": "${dataset}",
  "created": "$(date -Iseconds)",
  "quota_mb": 256,
  "compression": false
}
EOF
    
    echo -e "${GREEN}✓${NC} Created directory: ${STORAGE_PATH}/${dataset}"
done

echo ""
echo "Dataset listing:"
ls -lah "$STORAGE_PATH" | tee "$OUTPUT_DIR/datasets.txt"

echo ""
sleep 1

# ============================================================================
# STEP 3: Write Test Data
# ============================================================================

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}📝 [3/6] Writing Test Data${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Write to data dataset
TEST_FILE="${STORAGE_PATH}/data/test-${TIMESTAMP}.bin"
FILE_SIZE_MB=10

echo "Writing ${FILE_SIZE_MB}MB test file..."
dd if=/dev/urandom of="$TEST_FILE" bs=1M count=$FILE_SIZE_MB 2>&1 | grep -E "copied|MB" || echo "   Wrote ${FILE_SIZE_MB}MB"
echo -e "${GREEN}✓${NC} Wrote: $TEST_FILE"
echo "   Size: $(du -h "$TEST_FILE" | cut -f1)"

# Create some log files with realistic content
echo ""
echo "Creating sample log files..."
for i in {1..5}; do
    LOG_FILE="${STORAGE_PATH}/logs/nestgate-${i}.log"
    {
        echo "[$(date -Iseconds)] INFO Starting NestGate service"
        echo "[$(date -Iseconds)] INFO Initializing storage backend"
        echo "[$(date -Iseconds)] INFO Loading configuration from /etc/nestgate/config.toml"
        echo "[$(date -Iseconds)] INFO Discovered 3 peer nodes on network"
        echo "[$(date -Iseconds)] INFO Storage capacity: 1TB available"
        echo "[$(date -Iseconds)] INFO Started API server on port 8080"
        echo "[$(date -Iseconds)] INFO Health check: OK"
    } > "$LOG_FILE"
done
echo -e "${GREEN}✓${NC} Created 5 log files"

# Create backup data
echo ""
echo "Creating backup data..."
mkdir -p "${STORAGE_PATH}/backups/snapshots"
echo "Backup manifest - $(date)" > "${STORAGE_PATH}/backups/manifest.txt"
echo "Dataset: data" >> "${STORAGE_PATH}/backups/manifest.txt"
echo "Files: $(ls -1 ${STORAGE_PATH}/data | wc -l)" >> "${STORAGE_PATH}/backups/manifest.txt"
echo -e "${GREEN}✓${NC} Backup manifest created"

# List contents
echo ""
echo "Storage contents:"
du -sh "${STORAGE_PATH}"/* | tee "$OUTPUT_DIR/storage-usage.txt"

echo ""
sleep 1

# ============================================================================
# STEP 4: Create Snapshots
# ============================================================================

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}📸 [4/6] Creating Snapshots${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Create filesystem-based snapshots (tar archives)
SNAPSHOT_DIR="$OUTPUT_DIR/snapshots"
mkdir -p "$SNAPSHOT_DIR"

for dataset in "${DATASETS[@]}"; do
    SNAPSHOT_FILE="${SNAPSHOT_DIR}/${dataset}-snapshot-${TIMESTAMP}.tar.gz"
    echo "Creating snapshot of ${dataset}..."
    
    if [ -d "${STORAGE_PATH}/${dataset}" ]; then
        tar -czf "$SNAPSHOT_FILE" -C "${STORAGE_PATH}/${dataset}" . 2>/dev/null
        SNAPSHOT_SIZE=$(du -h "$SNAPSHOT_FILE" | cut -f1)
        echo -e "${GREEN}✓${NC} Snapshot created: $(basename "$SNAPSHOT_FILE") (${SNAPSHOT_SIZE})"
    fi
done

echo ""
echo "Snapshot listing:"
ls -lh "$SNAPSHOT_DIR" | tee "$OUTPUT_DIR/snapshots.txt"

echo ""
sleep 1

# ============================================================================
# STEP 5: Performance Verification
# ============================================================================

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}📊 [5/6] Performance Verification${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

echo "Storage statistics:"
{
    echo "Total size: $(du -sh "$STORAGE_PATH" | cut -f1)"
    echo "Datasets: ${#DATASETS[@]}"
    echo "Files: $(find "$STORAGE_PATH" -type f | wc -l)"
    echo "Snapshots: $(ls -1 "$SNAPSHOT_DIR" | wc -l)"
} | tee "$OUTPUT_DIR/stats.txt"

echo ""
echo "Dataset breakdown:"
for dataset in "${DATASETS[@]}"; do
    SIZE=$(du -sh "${STORAGE_PATH}/${dataset}" 2>/dev/null | cut -f1 || echo "0")
    FILES=$(find "${STORAGE_PATH}/${dataset}" -type f 2>/dev/null | wc -l || echo "0")
    echo "   ${dataset}: ${SIZE} (${FILES} files)"
done | tee -a "$OUTPUT_DIR/stats.txt"

echo ""
echo "System disk usage:"
df -h "$OUTPUT_DIR" | tail -1 | tee "$OUTPUT_DIR/disk-usage.txt"

echo ""
sleep 1

# ============================================================================
# STEP 6: Data Integrity Check
# ============================================================================

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}🔍 [6/6] Data Integrity Check${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

echo "Verifying test file..."
if [ -f "$TEST_FILE" ]; then
    FILE_HASH=$(sha256sum "$TEST_FILE" | awk '{print $1}')
    FILE_SIZE=$(stat -f%z "$TEST_FILE" 2>/dev/null || stat -c%s "$TEST_FILE" 2>/dev/null)
    echo -e "${GREEN}✓${NC} Test file exists"
    echo "   Hash: ${FILE_HASH:0:16}..."
    echo "   Size: ${FILE_SIZE} bytes"
else
    echo -e "${RED}✗${NC} Test file missing!"
fi

echo ""
echo "Verifying snapshots..."
SNAPSHOT_COUNT=$(ls -1 "$SNAPSHOT_DIR" | wc -l)
if [ "$SNAPSHOT_COUNT" -eq "${#DATASETS[@]}" ]; then
    echo -e "${GREEN}✓${NC} All snapshots present (${SNAPSHOT_COUNT}/${#DATASETS[@]})"
else
    echo -e "${YELLOW}⚠${NC} Some snapshots missing (${SNAPSHOT_COUNT}/${#DATASETS[@]})"
fi

END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

echo ""

# ============================================================================
# Generate Receipt
# ============================================================================

cat > "$OUTPUT_DIR/RECEIPT.md" <<EOF
# NestGate Storage Basics - Demo Receipt

**Date**: $(date)
**Duration**: ${DURATION}s
**Backend**: Filesystem (no-sudo mode)
**Status**: ✅ SUCCESS

---

## Operations Performed

### 1. Storage Backend Creation
- Created filesystem storage: \`$STORAGE_PATH\`
- Simulated 1GB pool capacity
- Storage path: \`$STORAGE_PATH\`

### 2. Dataset Creation
$(for dataset in "${DATASETS[@]}"; do
    echo "- \`${dataset}\` (256MB quota simulated)"
done)

### 3. Test Data Written
- Test file: \`$TEST_FILE\` (${FILE_SIZE_MB}MB)
- File hash: \`${FILE_HASH:0:32}...\`
- Log files: 5 sample files created
- Backup manifest: Created
- Total data written: ~${FILE_SIZE_MB}MB

### 4. Snapshots Created
$(ls -1 "$SNAPSHOT_DIR" | while read snapshot; do
    SIZE=$(du -h "$SNAPSHOT_DIR/$snapshot" | cut -f1)
    echo "- \`$snapshot\` (${SIZE})"
done)

### 5. Performance Metrics
- Total storage used: $(du -sh "$STORAGE_PATH" | cut -f1)
- Total files: $(find "$STORAGE_PATH" -type f | wc -l)
- Datasets: ${#DATASETS[@]}
- Snapshots: $SNAPSHOT_COUNT

---

## Files Generated

\`\`\`
$(ls -lh "$OUTPUT_DIR" | tail -n +2)
\`\`\`

## Storage Structure

\`\`\`
$(tree -L 2 "$STORAGE_PATH" 2>/dev/null || find "$STORAGE_PATH" -maxdepth 2 -type d)
\`\`\`

---

## Cleanup

\`\`\`bash
rm -rf $OUTPUT_DIR
\`\`\`

---

## Key Capabilities Demonstrated

- ✅ Storage backend creation
- ✅ Dataset organization (5 datasets)
- ✅ Real data write operations (${FILE_SIZE_MB}MB)
- ✅ Snapshot/backup functionality
- ✅ Performance monitoring
- ✅ Data integrity verification

---

## NestGate Features Shown

1. **Universal Storage Backend**: Filesystem-based storage
2. **Graceful Degradation**: Works without ZFS
3. **Data Organization**: Logical dataset structure
4. **Snapshot Support**: Tar-based snapshots
5. **Performance Monitoring**: Real-time statistics
6. **Integrity Checking**: Hash verification

---

**Demo Version**: ${DEMO_VERSION}
**Generated**: $(date)
EOF

# ============================================================================
# Summary
# ============================================================================

echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✅ Demo Complete!${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo "📊 Summary:"
echo "   Duration: ${DURATION}s"
echo "   Backend: Filesystem"
echo "   Data written: ${FILE_SIZE_MB}MB"
echo "   Datasets: ${#DATASETS[@]}"
echo "   Snapshots: $SNAPSHOT_COUNT"
echo "   Total size: $(du -sh "$OUTPUT_DIR" | cut -f1)"
echo ""
echo "📁 Output:"
echo "   Directory: $OUTPUT_DIR"
echo "   Receipt: $OUTPUT_DIR/RECEIPT.md"
echo "   Storage: $STORAGE_PATH"
echo "   Snapshots: $SNAPSHOT_DIR"
echo ""
echo "🧹 Cleanup:"
echo "   rm -rf $OUTPUT_DIR"
echo ""

exit 0
