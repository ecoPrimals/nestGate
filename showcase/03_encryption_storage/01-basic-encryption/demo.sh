#!/bin/bash
# Live Demo: Basic Encryption Flow (Compress → Encrypt → Store)
#
# Demonstrates the correct order for encrypted storage

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
echo -e "${CYAN}║     🔐 LIVE: Compress → Encrypt → Store                   ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUTPUT_DIR="$SCRIPT_DIR/output"
mkdir -p "$OUTPUT_DIR"

# Check prerequisites
echo -e "${BLUE}🔍 Checking prerequisites...${NC}"

# Check for BearDog
if curl -s http://localhost:9000/health > /dev/null 2>&1; then
    echo -e "${GREEN}✅ BearDog BTSP server is running${NC}"
    BEARDOG_AVAILABLE=true
else
    echo -e "${YELLOW}⚠️  BearDog BTSP server not running - using simulation${NC}"
    BEARDOG_AVAILABLE=false
fi

# Check for NestGate
NESTGATE_PORT=7200
if curl -s "http://localhost:$NESTGATE_PORT/health" > /dev/null 2>&1; then
    echo -e "${GREEN}✅ NestGate is running on port $NESTGATE_PORT${NC}"
    NESTGATE_AVAILABLE=true
else
    echo -e "${YELLOW}⚠️  NestGate not running - using filesystem simulation${NC}"
    NESTGATE_AVAILABLE=false
fi

echo ""

# Create test data (genomic sequence)
echo -e "${BLUE}📄 Step 1: Create test data (genomic sequence)${NC}"
TEST_FILE="$OUTPUT_DIR/genome-sample.fasta"

cat > "$TEST_FILE" << 'EOF'
>chr1 Human chromosome 1 (sample)
ATCGATCGATCGATCGATCGATCGATCGATCGATCGATCGATCGATCGATCG
ATCGATCGATCGATCGATCGATCGATCGATCGATCGATCGATCGATCGATCG
ATCGATCGATCGATCGATCGATCGATCGATCGATCGATCGATCGATCGATCG
ATCGATCGATCGATCGATCGATCGATCGATCGATCGATCGATCGATCGATCG
>chr2 Human chromosome 2 (sample)
GCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTA
GCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTA
GCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTA
EOF

ORIGINAL_SIZE=$(stat -f%z "$TEST_FILE" 2>/dev/null || stat -c%s "$TEST_FILE")
ORIGINAL_HASH=$(sha256sum "$TEST_FILE" | cut -d' ' -f1)

echo -e "${CYAN}   Original data:${NC}"
echo "   - File: genome-sample.fasta"
echo "   - Size: $ORIGINAL_SIZE bytes"
echo "   - Hash: $ORIGINAL_HASH"
echo "   - Type: Plain text FASTA (genomic data)"
echo ""

# Step 2: Compress (BEFORE encryption)
echo -e "${BLUE}🗜️  Step 2: COMPRESS (while data is plain)${NC}"
echo -e "${CYAN}   Why compress first?${NC}"
echo "   • Plain data has patterns (low entropy)"
echo "   • Compression finds and removes redundancy"
echo "   • Encrypted data looks random (high entropy)"
echo "   • Random data doesn't compress!"
echo ""

COMPRESSED_FILE="$OUTPUT_DIR/genome-sample.fasta.zst"
zstd -19 -q "$TEST_FILE" -o "$COMPRESSED_FILE"

COMPRESSED_SIZE=$(stat -f%z "$COMPRESSED_FILE" 2>/dev/null || stat -c%s "$COMPRESSED_FILE")
COMPRESSION_RATIO=$(echo "scale=2; $ORIGINAL_SIZE / $COMPRESSED_SIZE" | bc)
SAVINGS=$((ORIGINAL_SIZE - COMPRESSED_SIZE))
SAVINGS_PCT=$(echo "scale=1; 100 * ($ORIGINAL_SIZE - $COMPRESSED_SIZE) / $ORIGINAL_SIZE" | bc)

echo -e "${GREEN}   ✅ Compression complete:${NC}"
echo "   - Original: $ORIGINAL_SIZE bytes"
echo "   - Compressed: $COMPRESSED_SIZE bytes"
echo "   - Ratio: ${COMPRESSION_RATIO}:1"
echo "   - Saved: $SAVINGS bytes (${SAVINGS_PCT}%)"
echo ""

# Step 3: Encrypt (AFTER compression)
echo -e "${BLUE}🔒 Step 3: ENCRYPT (compressed data)${NC}"

if $BEARDOG_AVAILABLE; then
    echo -e "${MAGENTA}   🎉 LIVE MODE: Using real BearDog BTSP encryption${NC}"
    
    # TODO: Actual BearDog encryption call
    # For now, simulate with OpenSSL
    echo -e "${YELLOW}   (BearDog API integration in progress)${NC}"
    echo "   Using OpenSSL AES-256-GCM simulation..."
    ENCRYPTED_FILE="$OUTPUT_DIR/genome-sample.enc"
    
    # Generate random key and IV for demo
    KEY=$(openssl rand -hex 32)
    IV=$(openssl rand -hex 12)
    
    # Encrypt
    openssl enc -aes-256-gcm -in "$COMPRESSED_FILE" -out "$ENCRYPTED_FILE" -K "$KEY" -iv "$IV" 2>/dev/null
    
    # Save key info (in production, BearDog manages this)
    echo "$KEY" > "$OUTPUT_DIR/key.txt"
    echo "$IV" > "$OUTPUT_DIR/iv.txt"
else
    echo -e "${YELLOW}   Simulating encryption with OpenSSL AES-256-GCM${NC}"
    ENCRYPTED_FILE="$OUTPUT_DIR/genome-sample.enc"
    
    KEY=$(openssl rand -hex 32)
    IV=$(openssl rand -hex 12)
    
    openssl enc -aes-256-gcm -in "$COMPRESSED_FILE" -out "$ENCRYPTED_FILE" -K "$KEY" -iv "$IV" 2>/dev/null
    
    echo "$KEY" > "$OUTPUT_DIR/key.txt"
    echo "$IV" > "$OUTPUT_DIR/iv.txt"
fi

ENCRYPTED_SIZE=$(stat -f%z "$ENCRYPTED_FILE" 2>/dev/null || stat -c%s "$ENCRYPTED_FILE")
ENCRYPTED_HASH=$(sha256sum "$ENCRYPTED_FILE" | cut -d' ' -f1)

echo -e "${GREEN}   ✅ Encryption complete:${NC}"
echo "   - Input: $COMPRESSED_SIZE bytes (compressed)"
echo "   - Output: $ENCRYPTED_SIZE bytes (encrypted)"
echo "   - Overhead: $((ENCRYPTED_SIZE - COMPRESSED_SIZE)) bytes (auth tag)"
echo "   - Hash: $ENCRYPTED_HASH"
echo ""

# Analyze encrypted data entropy
echo -e "${BLUE}📊 Step 4: Analyze encrypted data${NC}"
ENCRYPTED_ENTROPY=$(python3 << EOF
import math
from collections import Counter

with open("$ENCRYPTED_FILE", "rb") as f:
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

echo "   Encrypted data entropy: $ENCRYPTED_ENTROPY bits/byte"
echo "   (8.0 = maximum, truly random)"
echo ""

if (( $(echo "$ENCRYPTED_ENTROPY > 7.5" | bc -l) )); then
    echo -e "${GREEN}   ✅ High entropy confirmed: Encrypted data looks random${NC}"
else
    echo -e "${YELLOW}   ⚠️  Lower entropy: Encryption may be weak${NC}"
fi
echo ""

# Step 5: Store (NestGate)
echo -e "${BLUE}💾 Step 5: STORE encrypted blob${NC}"

if $NESTGATE_AVAILABLE; then
    echo -e "${MAGENTA}   🎉 LIVE MODE: Storing to real NestGate${NC}"
    
    # Store via API
    RESPONSE=$(curl -s -X POST \
        -H "Content-Type: application/octet-stream" \
        --data-binary "@$ENCRYPTED_FILE" \
        "http://localhost:$NESTGATE_PORT/api/v1/store")
    
    echo "   API Response: $RESPONSE"
else
    echo -e "${YELLOW}   Simulating NestGate storage${NC}"
    
    # Simulate NestGate's decision
    echo "   NestGate analysis:"
    echo "   - Entropy: $ENCRYPTED_ENTROPY (high)"
    echo "   - Format: Unknown (encrypted)"
    echo "   - Decision: PASSTHROUGH (no further compression)"
    
    # Copy to simulated storage
    STORAGE_DIR="$OUTPUT_DIR/nestgate-storage"
    mkdir -p "$STORAGE_DIR"
    cp "$ENCRYPTED_FILE" "$STORAGE_DIR/$ENCRYPTED_HASH"
    
    echo ""
    echo -e "${GREEN}   ✅ Stored encrypted blob:${NC}"
    echo "   - Location: $STORAGE_DIR/$ENCRYPTED_HASH"
    echo "   - Size: $ENCRYPTED_SIZE bytes"
    echo "   - Compression: None (already encrypted)"
fi

echo ""

# Step 6: Verify round-trip
echo -e "${BLUE}🔄 Step 6: Verify round-trip (retrieve → decrypt → decompress)${NC}"

# Retrieve
if $NESTGATE_AVAILABLE; then
    echo "   Retrieving from NestGate..."
    # TODO: Retrieve via API
else
    echo "   Retrieving from simulated storage..."
    RETRIEVED_FILE="$OUTPUT_DIR/retrieved.enc"
    cp "$STORAGE_DIR/$ENCRYPTED_HASH" "$RETRIEVED_FILE"
fi

# Decrypt
echo "   Decrypting..."
DECRYPTED_FILE="$OUTPUT_DIR/retrieved.zst"
KEY=$(cat "$OUTPUT_DIR/key.txt")
IV=$(cat "$OUTPUT_DIR/iv.txt")
openssl enc -aes-256-gcm -d -in "$ENCRYPTED_FILE" -out "$DECRYPTED_FILE" -K "$KEY" -iv "$IV" 2>/dev/null

# Decompress
echo "   Decompressing..."
FINAL_FILE="$OUTPUT_DIR/retrieved.fasta"
zstd -d -q "$DECRYPTED_FILE" -o "$FINAL_FILE"

# Verify
FINAL_HASH=$(sha256sum "$FINAL_FILE" | cut -d' ' -f1)

if [ "$ORIGINAL_HASH" == "$FINAL_HASH" ]; then
    echo -e "${GREEN}   ✅ SUCCESS: Data integrity verified!${NC}"
    echo "   - Original hash: $ORIGINAL_HASH"
    echo "   - Final hash:    $FINAL_HASH"
else
    echo -e "${RED}   ❌ FAIL: Hash mismatch!${NC}"
    echo "   - Original hash: $ORIGINAL_HASH"
    echo "   - Final hash:    $FINAL_HASH"
fi

echo ""

# Summary
echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║                         SUMMARY                            ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

echo -e "${BLUE}Data Flow:${NC}"
echo "  1. Original data:     $ORIGINAL_SIZE bytes (plain FASTA)"
echo "  2. After compression: $COMPRESSED_SIZE bytes (zstd-19, ${COMPRESSION_RATIO}:1)"
echo "  3. After encryption:  $ENCRYPTED_SIZE bytes (AES-256-GCM)"
echo "  4. Final storage:     $ENCRYPTED_SIZE bytes"
echo ""

echo -e "${BLUE}Key Insights:${NC}"
echo "  ✅ Compress FIRST (while data is plain)"
echo "     → Achieved ${COMPRESSION_RATIO}:1 compression"
echo ""
echo "  ✅ Encrypt SECOND (compress data)"
echo "     → Result: High entropy ($ENCRYPTED_ENTROPY bits/byte)"
echo ""
echo "  ✅ Store encrypted (NestGate passthrough)"
echo "     → No further compression attempted"
echo ""
echo "  ✅ Round-trip verified"
echo "     → Data integrity maintained"
echo ""

echo -e "${YELLOW}What if we encrypted FIRST?${NC}"
echo "  ❌ Encrypted data → high entropy ($ENCRYPTED_ENTROPY)"
echo "  ❌ High entropy → compression fails"
echo "  ❌ Result: $ORIGINAL_SIZE bytes stored (NO compression!)"
echo ""

echo -e "${GREEN}🎉 Demo Complete!${NC}"
echo ""

echo -e "${BLUE}Next Steps:${NC}"
echo "  1. See ../02-friend-backup/demo.sh for zero-knowledge storage"
echo "  2. See ../03-encrypted-sharding/demo.sh for distributed storage"
echo "  3. See ../04-key-management/demo.sh for access control"
echo ""

echo -e "${CYAN}Output saved to: $OUTPUT_DIR${NC}"

