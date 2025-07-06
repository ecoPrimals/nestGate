#!/bin/bash

# NestGate Production ZFS Pool Setup Script
# Creates enterprise-grade nestpool with hot/warm/cold tiers
# Uses nvme1n1 and nvme2n1 (the free drives)
# RESPECTS existing boot drive and project space

set -e  # Exit on any error

echo "🚀 NestGate ZFS Pool Setup"
echo "=========================="
echo "Creating production nestpool with 2x 1.8TB NVMe drives"
echo ""

# Safety checks
echo "🔍 Pre-flight safety checks..."

# Verify we're not touching the boot drive
if mount | grep -q "/dev/nvme0n1"; then
    echo "✅ Boot drive nvme0n1 detected - will NOT touch"
else
    echo "❌ Boot drive detection failed"
    exit 1
fi

# Check available drives
if [ ! -b "/dev/nvme1n1" ] || [ ! -b "/dev/nvme2n1" ]; then
    echo "❌ Required drives nvme1n1 or nvme2n1 not found"
    exit 1
fi

# Verify drives are not mounted
if mount | grep -q "/dev/nvme1n1\|/dev/nvme2n1"; then
    echo "❌ One of the target drives is already mounted"
    mount | grep "/dev/nvme[12]n1" || true
    exit 1
fi

echo "✅ Target drives nvme1n1 and nvme2n1 available and safe"
echo ""

# Create ZFS pool with mirror for redundancy
echo "💾 Creating mirrored ZFS pool 'nestpool'..."
sudo zpool create nestpool mirror /dev/nvme1n1 /dev/nvme2n1

# Enable compression globally
echo "🗜️ Enabling LZ4 compression..."
sudo zfs set compression=lz4 nestpool

# Create tier datasets with optimized settings
echo "📊 Creating performance-optimized tier datasets..."

# HOT TIER - Maximum performance
sudo zfs create nestpool/hot
sudo zfs set recordsize=128K nestpool/hot
sudo zfs set compression=lz4 nestpool/hot
sudo zfs set primarycache=all nestpool/hot
sudo zfs set secondarycache=all nestpool/hot
sudo zfs set logbias=throughput nestpool/hot
sudo zfs set nestgate:storage_tier=hot nestpool/hot

# WARM TIER - Balanced performance/compression
sudo zfs create nestpool/warm  
sudo zfs set recordsize=1M nestpool/warm
sudo zfs set compression=zstd nestpool/warm
sudo zfs set primarycache=all nestpool/warm
sudo zfs set secondarycache=all nestpool/warm
sudo zfs set nestgate:storage_tier=warm nestpool/warm

# COLD TIER - Maximum compression
sudo zfs create nestpool/cold
sudo zfs set recordsize=1M nestpool/cold
sudo zfs set compression=gzip-9 nestpool/cold
sudo zfs set primarycache=metadata nestpool/cold
sudo zfs set secondarycache=none nestpool/cold
sudo zfs set nestgate:storage_tier=cold nestpool/cold

# Create datasets for testing
echo "🧪 Creating test datasets..."
sudo zfs create nestpool/test
sudo zfs create nestpool/benchmarks
sudo zfs create nestpool/large_data

# Set permissions for user access
echo "🔐 Configuring permissions..."
sudo chown -R $USER:$USER /nestpool
sudo chmod -R 755 /nestpool

# Create initial snapshots
echo "📸 Creating initial snapshots..."
sudo zfs snapshot nestpool/hot@initial
sudo zfs snapshot nestpool/warm@initial  
sudo zfs snapshot nestpool/cold@initial
sudo zfs snapshot nestpool/test@initial

echo ""
echo "🎉 NestPool Setup Complete!"
echo "=========================="
echo "📊 Pool Status:"
sudo zpool status nestpool
echo ""
echo "📁 Datasets:"
sudo zfs list -t filesystem -o name,used,avail,mountpoint nestpool
echo ""
echo "🏆 Ready for:"
echo "  ✅ 6,032 test functions with real ZFS"
echo "  ✅ TB-scale data movement testing"
echo "  ✅ Production-grade chaos testing"
echo "  ✅ Hot/Warm/Cold tier validation"
echo "  ✅ Enterprise-grade performance testing"
echo ""
echo "🎯 Mount points:"
echo "  Hot:  /nestpool/hot"
echo "  Warm: /nestpool/warm" 
echo "  Cold: /nestpool/cold"
echo "  Test: /nestpool/test" 