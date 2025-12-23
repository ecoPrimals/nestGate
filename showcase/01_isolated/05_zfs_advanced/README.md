# Demo 1.5: ZFS Advanced Features

**Level**: 1 (Isolated)  
**Time**: 5 minutes  
**Complexity**: Beginner to Intermediate  
**Status**: 🆕 New demo

---

## 🎯 WHAT THIS DEMO SHOWS

This demo demonstrates NestGate's advanced ZFS features:

1. **Snapshots** - Point-in-time copies
2. **Compression** - Transparent data compression
3. **Deduplication** - Eliminate duplicate data
4. **Copy-on-Write** - Efficient data management
5. **Rollback & Recovery** - Time travel for your data

**Key Concept**: ZFS provides enterprise-grade features that NestGate exposes through simple APIs

---

## 🚀 QUICK RUN

```bash
# Make sure NestGate is running with ZFS backend
../../scripts/start_data_service.sh

# Run the demo
./demo.sh

# Expected runtime: ~2 minutes
```

---

## 📋 WHAT YOU'LL SEE

### Part 1: Snapshots (Time Travel!)
```bash
# Create a dataset
POST /api/v1/datasets/demo-zfs
{"name": "demo-zfs"}

# Add some data
PUT /api/v1/datasets/demo-zfs/objects/file1.txt
"Original content"

# Create a snapshot
POST /api/v1/datasets/demo-zfs/snapshots
{"name": "snap1"}

# Modify the data
PUT /api/v1/datasets/demo-zfs/objects/file1.txt
"Modified content"

# Rollback to snapshot
POST /api/v1/datasets/demo-zfs/rollback
{"snapshot": "snap1"}

# File is back to original!
GET /api/v1/datasets/demo-zfs/objects/file1.txt
→ "Original content"
```

### Part 2: Compression
```bash
# Enable compression
PUT /api/v1/datasets/demo-zfs/properties
{"compression": "lz4"}

# Store compressible data
PUT /api/v1/datasets/demo-zfs/objects/large.txt
"This is repeated text... (1000x)"

# Check actual storage used
GET /api/v1/datasets/demo-zfs/stats
→ {
    "logical_size": "100KB",
    "physical_size": "5KB",
    "compression_ratio": "20:1"
  }
```

### Part 3: Deduplication
```bash
# Enable dedup
PUT /api/v1/datasets/demo-zfs/properties
{"deduplication": "on"}

# Store duplicate data
PUT /api/v1/datasets/demo-zfs/objects/file1.txt
"Same content"

PUT /api/v1/datasets/demo-zfs/objects/file2.txt
"Same content"

# Check dedup stats
GET /api/v1/datasets/demo-zfs/stats
→ {
    "logical_size": "24 bytes",
    "physical_size": "12 bytes",
    "dedup_ratio": "2.0x"
  }
```

---

## 💡 WHY ZFS FEATURES MATTER

### 1. Snapshots = Time Machine
**Problem**: Accidental file deletion or corruption  
**Solution**: Instant recovery from snapshots

**Use Cases**:
- Before risky operations (software updates)
- Hourly backups (nearly free!)
- Testing changes safely
- Disaster recovery

### 2. Compression = Free Space
**Problem**: Running out of disk space  
**Solution**: Transparent compression

**Benefits**:
- 2-4x space savings typical
- No application changes needed
- Faster I/O (less data to read/write)
- LZ4 is extremely fast (~500 MB/s compression)

### 3. Deduplication = Efficiency
**Problem**: Many copies of same data  
**Solution**: Store once, reference many times

**Use Cases**:
- VM images (90% identical)
- Backups (incremental dedup)
- User data (duplicate documents)
- Container layers

### 4. Copy-on-Write = Safety
**Problem**: Corruption during writes  
**Solution**: Never overwrite existing data

**Benefits**:
- Atomic operations (all or nothing)
- No fsck needed ever
- Snapshot-based backups
- Data integrity guaranteed

---

## 🧪 EXPERIMENTS TO TRY

### Experiment 1: Snapshot Workflow
```bash
# Create dataset
curl -X POST ${NESTGATE_URL}/api/v1/datasets \
  -d '{"name": "experiment"}'

# Add initial data
for i in {1..10}; do
  curl -X PUT ${NESTGATE_URL}/api/v1/datasets/experiment/objects/file${i} \
    -d "Version 1"
done

# Snapshot
curl -X POST ${NESTGATE_URL}/api/v1/datasets/experiment/snapshots \
  -d '{"name": "v1"}'

# Make changes
for i in {1..10}; do
  curl -X PUT ${NESTGATE_URL}/api/v1/datasets/experiment/objects/file${i} \
    -d "Version 2"
done

# Snapshot again
curl -X POST ${NESTGATE_URL}/api/v1/datasets/experiment/snapshots \
  -d '{"name": "v2"}'

# List snapshots
curl ${NESTGATE_URL}/api/v1/datasets/experiment/snapshots

# Rollback to v1
curl -X POST ${NESTGATE_URL}/api/v1/datasets/experiment/rollback \
  -d '{"snapshot": "v1"}'
```

### Experiment 2: Compression Testing
```bash
# Create highly compressible data
python3 <<EOF
with open('/tmp/compressible.txt', 'w') as f:
    for i in range(10000):
        f.write("This is repeated text that compresses well.\n")
EOF

# Upload without compression
curl -X POST ${NESTGATE_URL}/api/v1/datasets/no-compress \
  -d '{"name": "no-compress", "compression": "off"}'

curl -X PUT ${NESTGATE_URL}/api/v1/datasets/no-compress/objects/data \
  --data-binary @/tmp/compressible.txt

# Upload with compression
curl -X POST ${NESTGATE_URL}/api/v1/datasets/with-compress \
  -d '{"name": "with-compress", "compression": "lz4"}'

curl -X PUT ${NESTGATE_URL}/api/v1/datasets/with-compress/objects/data \
  --data-binary @/tmp/compressible.txt

# Compare sizes
curl ${NESTGATE_URL}/api/v1/datasets/no-compress/stats
curl ${NESTGATE_URL}/api/v1/datasets/with-compress/stats
```

### Experiment 3: Snapshot Diff
```bash
# Create dataset with snapshots
curl -X POST ${NESTGATE_URL}/api/v1/datasets/diff-test \
  -d '{"name": "diff-test"}'

# Add files
echo "File A" | curl -X PUT ${NESTGATE_URL}/api/v1/datasets/diff-test/objects/a -d @-
echo "File B" | curl -X PUT ${NESTGATE_URL}/api/v1/datasets/diff-test/objects/b -d @-

# Snapshot
curl -X POST ${NESTGATE_URL}/api/v1/datasets/diff-test/snapshots -d '{"name": "before"}'

# Make changes
echo "Modified A" | curl -X PUT ${NESTGATE_URL}/api/v1/datasets/diff-test/objects/a -d @-
echo "File C" | curl -X PUT ${NESTGATE_URL}/api/v1/datasets/diff-test/objects/c -d @-
curl -X DELETE ${NESTGATE_URL}/api/v1/datasets/diff-test/objects/b

# Snapshot
curl -X POST ${NESTGATE_URL}/api/v1/datasets/diff-test/snapshots -d '{"name": "after"}'

# See what changed
curl ${NESTGATE_URL}/api/v1/datasets/diff-test/snapshots/diff?from=before&to=after
```

---

## 📊 ZFS FEATURE COMPARISON

### vs Traditional Filesystems

| Feature | Traditional FS | ZFS |
|---------|---------------|-----|
| **Snapshots** | Complex/Slow | Instant/Free |
| **Compression** | Manual | Transparent |
| **Checksums** | Rare | Always |
| **RAID** | Hardware | Software (better) |
| **Corruption Detection** | fsck (slow) | Automatic |
| **Recovery** | Limited | Snapshot rollback |

### ZFS Benefits Summary

**Data Integrity**: ✅ Checksums on everything  
**Space Efficiency**: ✅ Compression + Dedup  
**Snapshots**: ✅ Instant, free, unlimited  
**Performance**: ✅ ARC cache, prefetch  
**Simplicity**: ✅ Unified pool management

---

## 🛠️ PRODUCTION CONFIGURATIONS

### Typical Setup: Web Server
```toml
[dataset.web-content]
compression = "lz4"        # Fast compression
dedup = "off"              # Not needed for unique files
snapshots = "hourly"       # Automated snapshots
retention = "7 days"       # Keep week of history
```

### Typical Setup: Database
```toml
[dataset.database]
compression = "lz4"        # Helps with large data
recordsize = "16K"         # Match DB block size
dedup = "off"              # Rarely beneficial
snapshots = "before-upgrade"  # Manual snapshots
```

### Typical Setup: User Files
```toml
[dataset.user-files]
compression = "lz4"        # Good space savings
dedup = "on"               # Lots of duplicates
snapshots = "daily"        # Daily backups
retention = "30 days"      # Monthly history
```

### Typical Setup: VM Images
```toml
[dataset.vm-images]
compression = "lz4"        # VMs compress well
dedup = "on"               # High dedup ratio
snapshots = "before-start" # Before each boot
clones = "enabled"         # Fast VM creation
```

---

## 🔧 PERFORMANCE CONSIDERATIONS

### Compression

**LZ4** (Recommended):
- Speed: ~500 MB/s
- Ratio: 1.5-2.5x
- CPU: Minimal (<5%)

**GZIP** (High Ratio):
- Speed: ~50 MB/s
- Ratio: 2-4x
- CPU: Moderate (10-20%)

**ZStandard** (Balanced):
- Speed: ~200 MB/s
- Ratio: 2-3x
- CPU: Low (5-10%)

### Deduplication

**When to Use**:
- ✅ VM images (90% identical)
- ✅ Backup storage
- ✅ Similar files

**When NOT to Use**:
- ❌ Unique data (no benefit)
- ❌ High write workloads (overhead)
- ❌ Limited RAM (<32GB)

**RAM Requirements**:
- Dedup table: ~1GB per TB of unique data
- Recommend: 2GB per TB minimum

### Snapshots

**Impact**:
- Creation: <1 second
- Storage: Only changed blocks
- Performance: Negligible

**Best Practices**:
- Take before risky operations
- Automate with cron
- Prune old snapshots
- Monitor space usage

---

## 🆘 TROUBLESHOOTING

### "Snapshot creation failed"
**Cause**: Dataset may not support snapshots  
**Fix**:
```bash
# Check if ZFS backend is active
curl ${NESTGATE_URL}/api/v1/capabilities | grep zfs

# Verify dataset uses ZFS
curl ${NESTGATE_URL}/api/v1/datasets/your-dataset/info
```

### "Compression not working"
**Cause**: Already-compressed data or wrong setting  
**Fix**:
```bash
# Check current compression setting
curl ${NESTGATE_URL}/api/v1/datasets/your-dataset/properties | grep compression

# Try different algorithm
curl -X PUT ${NESTGATE_URL}/api/v1/datasets/your-dataset/properties \
  -d '{"compression": "zstd"}'
```

### "Dedup using too much RAM"
**Cause**: Large dataset with high dedup table  
**Fix**:
```bash
# Disable dedup on large datasets
curl -X PUT ${NESTGATE_URL}/api/v1/datasets/your-dataset/properties \
  -d '{"deduplication": "off"}'

# Or increase system RAM
```

---

## 📚 LEARN MORE

**ZFS Concepts**:
- **ZFS Architecture**: `../../../docs/architecture/UNIVERSAL_STORAGE_DESIGN.md`
- **Storage Backend**: `../../../code/crates/nestgate-zfs/`
- **ZFS Operations**: `../../../code/crates/nestgate-zfs/src/operations/`

**Best Practices**:
- **Production Config**: `../../../config/production-optimized.toml`
- **Performance Tuning**: `../../../docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Backup Strategies**: `../../../docs/guides/BACKUP_STRATEGIES.md`

**External Resources**:
- OpenZFS Documentation: https://openzfs.org/
- ZFS Admin Guide: https://docs.oracle.com/cd/E19253-01/819-5461/
- ZFS Best Practices: https://jrs-s.net/category/open-source/zfs/

---

## ⏭️ CONGRATULATIONS!

**You've completed Level 1: Isolated Instance!** 🎉

You now understand:
- ✅ Basic storage operations
- ✅ REST API usage
- ✅ Zero-knowledge architecture
- ✅ Health monitoring
- ✅ Advanced ZFS features

**Ready for Level 2: Ecosystem Integration** (`../../02_ecosystem_integration/`)
- See NestGate work with BearDog (crypto)
- See NestGate work with Songbird (orchestration)
- See NestGate work with ToadStool (compute)

---

**Status**: 🆕 New demo  
**Estimated time**: 5 minutes  
**Prerequisites**: Completed Demos 1.1-1.4, ZFS installed

**Key Takeaway**: ZFS features like snapshots and compression make NestGate enterprise-grade! 💾

---

*Demo 1.5 - ZFS Advanced Features*  
*Part of Level 1: Isolated Instance*  
*🎉 Level 1 Complete!*

