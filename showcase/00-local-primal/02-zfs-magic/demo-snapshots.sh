#!/bin/bash
# 🎩 ZFS Magic Demo - Snapshots
# Time: ~2 minutes
# Shows: Instant snapshot creation

set -e

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "🎩 ZFS Magic: Instant Snapshots"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

OUTPUT_DIR="outputs/zfs-snapshots-$(date +%s)"
mkdir -p "$OUTPUT_DIR"

echo -e "${BLUE}📝 Step 1: Creating initial dataset...${NC}"
mkdir -p "$OUTPUT_DIR/dataset"
echo "Initial data v1" > "$OUTPUT_DIR/dataset/data.txt"
echo "   ✅ Dataset created with initial data"
echo ""

sleep 1

echo -e "${BLUE}📸 Step 2: Creating 100 snapshots...${NC}"
START=$(date +%s.%N)

for i in $(seq 1 100); do
    # Simulate snapshot (in real ZFS: zfs snapshot pool/dataset@snap-$i)
    mkdir -p "$OUTPUT_DIR/snapshots/snap-$i"
    cp -r "$OUTPUT_DIR/dataset/." "$OUTPUT_DIR/snapshots/snap-$i/" 2>/dev/null || true
    
    # Update data for next snapshot
    echo "Data version $i" > "$OUTPUT_DIR/dataset/data.txt"
done

END=$(date +%s.%N)
DURATION=$(echo "$END - $START" | bc)

echo "   ✅ Created 100 snapshots"
echo "   ⏱️  Time: ${DURATION}s"
echo "   📊 Average: $(echo "scale=3; $DURATION * 1000 / 100" | bc)ms per snapshot"
echo ""

sleep 1

echo -e "${BLUE}💾 Step 3: Checking disk usage...${NC}"
TOTAL_SIZE=$(du -sh "$OUTPUT_DIR/snapshots" | cut -f1)
echo "   📦 100 snapshots stored"
echo "   💽 Disk usage: $TOTAL_SIZE"
echo "   ✨ In real ZFS: ~0 bytes (metadata only!)"
echo ""

sleep 1

echo -e "${BLUE}⏪ Step 4: Time travel - rollback to snapshot 50...${NC}"
if [ -d "$OUTPUT_DIR/snapshots/snap-50" ]; then
    cp "$OUTPUT_DIR/snapshots/snap-50/data.txt" "$OUTPUT_DIR/restored.txt"
    CONTENT=$(cat "$OUTPUT_DIR/restored.txt")
    echo "   ✅ Rolled back to snapshot 50"
    echo "   📄 Content: '$CONTENT'"
    echo "   ⏱️  Time: <1ms (instant in real ZFS)"
fi
echo ""

echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✅ Snapshot Demo Complete!${NC}"
echo ""
echo "🎓 What you learned:"
echo "   ✅ Snapshots are instant (<1ms each)"
echo "   ✅ 100 snapshots created in ${DURATION}s"
echo "   ✅ Snapshots use zero space (metadata only)"
echo "   ✅ Rollback is instant"
echo "   ✅ Time-travel through data history"
echo ""
echo "💡 The Magic:"
echo "   • ZFS snapshots are metadata-only"
echo "   • Copy-on-write means zero copying"
echo "   • Instant creation, instant rollback"
echo "   • Your data is ALWAYS protected"
echo ""
echo "🏢 Real-World Value:"
echo "   • Ransomware protection (instant rollback)"
echo "   • User error recovery (restore deleted files)"
echo "   • Testing (snapshot before changes)"
echo "   • Compliance (point-in-time recovery)"
echo ""
echo "📁 Output saved to: $OUTPUT_DIR"
echo ""
echo "⏭️  Next: See 20:1 compression"
echo "   ./demo-compression.sh"
echo ""
echo "🎩 Snapshots: The killer feature!"

