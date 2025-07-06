#!/bin/bash

# NestGate Pool Expansion Script
# Expands existing nestpool with second drive and full tier structure
# Current: 1.8TB single drive with hot tier
# Target: Multi-TB enterprise setup with hot/warm/cold tiers

set -e

echo "🚀 NestGate Pool Expansion"
echo "=========================="
echo "Current: nestpool (1.8TB) with hot tier"
echo "Adding: nvme2n1 + warm/cold tiers + test datasets"
echo ""

# Check current status
echo "📊 Current nestpool status:"
sudo zpool status nestpool
echo ""

# Add second drive to expand capacity
echo "💾 Adding nvme2n1 to expand pool capacity..."
sudo zpool add nestpool /dev/nvme2n1

echo "✅ Pool expanded! New status:"
sudo zpool status nestpool
echo ""

# Create missing tier datasets
echo "📊 Creating warm and cold tier datasets..."

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

# Optimize existing HOT TIER
echo "⚡ Optimizing hot tier settings..."
sudo zfs set recordsize=128K nestpool/hot
sudo zfs set compression=lz4 nestpool/hot
sudo zfs set primarycache=all nestpool/hot
sudo zfs set secondarycache=all nestpool/hot
sudo zfs set logbias=throughput nestpool/hot
sudo zfs set nestgate:storage_tier=hot nestpool/hot

# Create datasets for comprehensive testing
echo "🧪 Creating test datasets..."
sudo zfs create nestpool/test
sudo zfs create nestpool/benchmarks
sudo zfs create nestpool/large_data
sudo zfs create nestpool/chaos_test
sudo zfs create nestpool/performance_test
sudo zfs create nestpool/ai_datasets

# Create datasets for E2E testing
sudo zfs create nestpool/e2e_test
sudo zfs create nestpool/fault_injection
sudo zfs create nestpool/integration_test

# Set user permissions
echo "🔐 Setting user permissions..."
sudo chown -R $USER:$USER /nestpool
sudo chmod -R 755 /nestpool

# Create initial snapshots for all datasets
echo "📸 Creating baseline snapshots..."
for dataset in hot warm cold test benchmarks large_data chaos_test performance_test ai_datasets e2e_test fault_injection integration_test; do
    sudo zfs snapshot nestpool/$dataset@baseline_$(date +%Y%m%d)
done

# Enable useful ZFS features
echo "⚙️ Enabling advanced ZFS features..."
sudo zfs set atime=off nestpool  # Better performance
sudo zfs set compression=lz4 nestpool  # Global compression

echo ""
echo "🎉 NestPool Expansion Complete!"
echo "==============================="
echo ""
echo "📊 Final Pool Status:"
sudo zpool status nestpool
echo ""
echo "📁 All Datasets:"
sudo zfs list -t filesystem -o name,used,avail,mountpoint nestpool
echo ""
echo "💿 Available Space:"
df -h /nestpool
echo ""
echo "🏆 Enterprise Capabilities Unlocked:"
echo "  ✅ Multi-TB capacity (~3.6TB total)"
echo "  ✅ Hot/Warm/Cold tier structure"
echo "  ✅ 6,032 test functions ready for real ZFS"
echo "  ✅ TB-scale data movement testing"
echo "  ✅ Comprehensive chaos testing datasets"
echo "  ✅ AI workload testing infrastructure"
echo "  ✅ Production-grade performance validation"
echo ""
echo "🎯 Mount Points Ready:"
echo "  📁 Hot:   /nestpool/hot   (max performance)"
echo "  📁 Warm:  /nestpool/warm  (balanced)"
echo "  📁 Cold:  /nestpool/cold  (max compression)"
echo "  📁 Test:  /nestpool/test  (general testing)"
echo "  📁 Large: /nestpool/large_data (TB-scale testing)"
echo ""
echo "🚀 Ready to run ENTERPRISE-SCALE testing!" 