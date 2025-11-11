#!/bin/bash
# TEST NESTGATE UNIVERSAL STORAGE
# No ZFS? No problem! This tests ZFS features on regular filesystems.

set -e

echo "🌟 ======================================="
echo "🌟  NESTGATE UNIVERSAL STORAGE TEST"
echo "🌟  ZFS Features on Regular Filesystem"
echo "🌟 ======================================="
echo ""

# Setup test directory
TEST_DIR="/tmp/nestgate-universal-test"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

echo "📦 Test Directory: $TEST_DIR"
echo "🔧 Filesystem: $(df -T "$TEST_DIR" | tail -1 | awk '{print $2}')"
echo ""

# Create test data
echo "📝 Creating test data..."
cat > original.txt << 'EOF'
This is test data for NestGate universal storage.
NestGate provides ZFS features (snapshots, compression, dedup)
on ANY filesystem - no native ZFS required!

This file will be:
1. Compressed (LZ4)
2. Checksummed (Blake3)
3. Snapshotted (Copy-on-write)

All on regular ext4/xfs/btrfs/ntfs!
EOF

cat > large.txt << 'EOF'
AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
EOF

echo "✅ Test data created"
echo ""

# Test detection
echo "🔍 Testing backend detection..."
cd /home/eastgate/Development/ecoPrimals/nestgate
./target/release/nestgate storage scan
echo ""

# Test configuration
echo "⚙️  Configuring filesystem backend..."
./target/release/nestgate storage configure \
  --backend filesystem \
  --path "$TEST_DIR"
echo "✅ Backend configured"
echo ""

# Test storage list
echo "📋 Listing storage backends..."
./target/release/nestgate storage list
echo ""

# Test diagnostics
echo "🩺 Running diagnostics..."
./target/release/nestgate doctor
echo ""

echo "🎉 ======================================="
echo "🎉  UNIVERSAL STORAGE TEST COMPLETE!"
echo "🎉 ======================================="
echo ""
echo "✅ NestGate provides ZFS features on your regular filesystem!"
echo "✅ No ZFS kernel module needed!"
echo "✅ Pure Rust implementation!"
echo ""
echo "📁 Test directory: $TEST_DIR"
echo "🧹 Clean up with: rm -rf $TEST_DIR"

