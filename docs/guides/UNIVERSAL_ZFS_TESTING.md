> **Historical**: This document was written in November 10, 2025. Current architecture
> and patterns may differ. See root-level docs and `specs/` for current specifications.

# **UNIVERSAL ZFS - TESTING GUIDE**

**Date**: November 10, 2025  
**Core Concept**: ZFS features on ANY storage backend  
**Status**: **READY TO TEST**

---

## **THE NESTGATE VALUE PROPOSITION**

### **Traditional Problem:**
```
Want ZFS features? You MUST have:
- ZFS kernel modules installed
- Root access for pool creation
- Dedicated disks/partitions
- Linux/FreeBSD/Solaris
- Complex setup and configuration
```

### **NestGate Solution:**
```
ZFS features on ANY storage backend!
No kernel modules needed
Works with regular filesystems (ext4, xfs, btrfs, ntfs)
Works with cloud storage (S3, Azure, GCS)
Works with network storage (NFS, SMB, iSCSI)
Works with memory (RAM disk, tmpfs)
Cross-platform (Linux, macOS, Windows, containers)
No root required, no special setup
```

---

## **HOW IT WORKS**

### **NestGate Universal Storage Abstraction**

```
┌─────────────────────────────────────────────┐
│         NestGate API Layer                  │
│  (ZFS semantics - snapshots, compression,   │
│   dedup, checksums, copy-on-write)          │
└─────────────────┬───────────────────────────┘
                  │
┌─────────────────┴───────────────────────────┐
│    Universal Storage Abstraction Layer      │
│  (Translates ZFS operations to backend)     │
└─────────────────┬───────────────────────────┘
                  │
        ┌─────────┼─────────┐
        ↓         ↓         ↓
   ┌────────┐ ┌──────┐ ┌────────┐
   │Native  │ │File  │ │Cloud   │
   │ZFS     │ │System│ │Storage │
   │(if avail)│ │(any!)│ │(S3/etc)│
   └────────┘ └──────┘ └────────┘
```

### **Feature Implementation on Non-ZFS**

**Snapshots:**
```
- Native ZFS: zfs snapshot
- Filesystem: Copy-on-write file copies + metadata
- Cloud: Versioned objects
- Result: Same snapshot semantics!
```

**Compression:**
```
- Native ZFS: Built-in LZ4/ZSTD
- Filesystem: Transparent compression layer
- Cloud: Pre-compressed objects
- Result: Same space savings!
```

**Deduplication:**
```
- Native ZFS: Block-level dedup
- Filesystem: Content-addressed storage
- Cloud: Object dedup via hashing
- Result: Same efficiency!
```

**Checksumming:**
```
- Native ZFS: SHA-256 checksums
- Filesystem: Extended attributes + verification
- Cloud: ETag/MD5 verification
- Result: Same data integrity!
```

---

## **TESTING WITHOUT NATIVE ZFS**

### **Test 1: Regular Filesystem Storage**

```bash
# No ZFS? No problem!

# Configure NestGate to use regular filesystem
nestgate storage configure --backend filesystem --path /home/data

# Create a "ZFS-style" dataset
curl -X POST http://localhost:9001/api/v1/datasets \
  -H 'Content-Type: application/json' \
  -d '{
    "name": "mydata",
    "compression": "lz4",
    "dedup": true,
    "checksum": "sha256"
  }'

# Write data (automatically compressed & checksummed)
curl -X PUT http://localhost:9001/api/v1/datasets/mydata/file.txt \
  --data-binary @large_file.txt

# Create snapshot
curl -X POST http://localhost:9001/api/v1/datasets/mydata/snapshots \
  -d '{"name": "snap1"}'

# Modify file
echo "changes" >> file.txt
curl -X PUT http://localhost:9001/api/v1/datasets/mydata/file.txt \
  --data-binary @file.txt

# Rollback to snapshot
curl -X POST http://localhost:9001/api/v1/datasets/mydata/rollback \
  -d '{"snapshot": "snap1"}'

# File reverted! ZFS semantics on ext4!
```

### **Test 2: Memory-Backed Storage**

```bash
# Use RAM as storage backend
nestgate storage configure --backend memory --size 1GB

# Create dataset in memory
curl -X POST http://localhost:9001/api/v1/datasets \
  -d '{"name": "cache", "backend": "memory"}'

# Ultra-fast operations (all in RAM)
# Still get snapshots, compression, checksums!
```

### **Test 3: Cloud Storage (S3-Compatible)**

```bash
# Use S3/MinIO/R2 as backend
nestgate storage configure \
  --backend s3 \
  --endpoint https://s3.amazonaws.com \
  --bucket my-zfs-bucket

# Create dataset in cloud
curl -X POST http://localhost:9001/api/v1/datasets \
  -d '{
    "name": "cloud-data",
    "backend": "s3",
    "compression": "zstd"
  }'

# Data stored in S3 with ZFS semantics!
# Snapshots = S3 versions
# Compression = Pre-compressed objects
# Checksums = ETags + verification
```

### **Test 4: Hybrid/Tiered Storage**

```bash
# Hot data in memory
# Warm data on filesystem  
# Cold data in cloud

nestgate storage configure --backend tiered \
  --hot memory:1GB \
  --warm filesystem:/data \
  --cold s3:my-bucket

# NestGate automatically migrates data between tiers!
# Recent/frequent = memory
# Active = filesystem
# Archive = S3
```

---

## **CAPABILITY MATRIX**

| Feature | Native ZFS | Filesystem | Memory | Cloud | Network |
|---------|------------|------------|--------|-------|---------|
| **Snapshots** | Native | COW | COW | Versions | COW |
| **Compression** | LZ4/ZSTD | Transparent | In-memory | Pre-compress | Transparent |
| **Dedup** | Block-level | Content-hash | Content-hash | Object-hash | Content-hash |
| **Checksums** | SHA-256 | XAttr + SHA | In-memory | ETag/MD5 | Verify |
| **COW** | Native | Emulated | Native | Emulated | Emulated |
| **Encryption** | Native | Layer | Layer | SSE | Layer |
| **Replication** | Send/Recv | Sync | N/A | S3 Repl | Rsync |

**Result: Same API, Same Features, Any Backend!**

---

## **PRACTICAL EXAMPLES**

### **Example 1: Dev Environment (No ZFS)**

```bash
# Developer laptop without ZFS
# Use local filesystem

nestgate storage configure --backend filesystem --path ~/.nestgate/data

# Get all ZFS features for development:
# - Snapshots before each git commit
# - Compression for large datasets
# - Dedup for repeated builds
# - Checksums for data integrity

# Hook into git:
git config core.hooksPath .githooks
cat > .githooks/pre-commit << 'EOF'
#!/bin/bash
# Snapshot before each commit
nestgate snapshot create --name "pre-commit-$(git rev-parse --short HEAD)"
EOF
```

### **Example 2: Docker Container**

```bash
# Container without ZFS kernel module
# Use tmpfs + filesystem

docker run -v /host/data:/data nestgate:latest \
  nestgate storage configure --backend filesystem --path /data

# Container gets ZFS features without host dependencies!
```

### **Example 3: Windows Development**

```bash
# Windows machine (no ZFS at all!)
# Use NTFS backend

nestgate.exe storage configure --backend filesystem --path C:\NestGate

# ZFS semantics on Windows NTFS!
# Snapshots via VSS/COW
# Compression via NTFS compression
# Checksums via extended attributes
```

### **Example 4: Kubernetes StatefulSet**

```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: nestgate
spec:
  serviceName: nestgate
  replicas: 3
  template:
    spec:
      containers:
      - name: nestgate
        image: nestgate:latest
        volumeMounts:
        - name: data
          mountPath: /data
  volumeClaimTemplates:
  - metadata:
      name: data
    spec:
      accessModes: [ "ReadWriteOnce" ]
      resources:
        requests:
          storage: 10Gi

# Each pod gets ZFS features on K8s PersistentVolumes!
# No ZFS kernel module needed in cluster!
```

---

## **KEY TESTING SCENARIOS**

### **Scenario 1: Snapshot & Rollback**

```bash
# Start with data
echo "version 1" > data.txt
nestgate put dataset1/data.txt < data.txt

# Snapshot
nestgate snapshot create dataset1@snap1

# Modify
echo "version 2" > data.txt
nestgate put dataset1/data.txt < data.txt

# Verify changed
nestgate get dataset1/data.txt
# Output: version 2

# Rollback
nestgate rollback dataset1@snap1

# Verify reverted
nestgate get dataset1/data.txt
# Output: version 1

Snapshots work without native ZFS!
```

### **Scenario 2: Compression**

```bash
# Create large repetitive file
dd if=/dev/zero of=zeros.dat bs=1M count=100

# Upload with compression
nestgate put --compress lz4 dataset1/zeros.dat < zeros.dat

# Check actual storage used
nestgate get --info dataset1/zeros.dat
# logical_size: 100MB
# actual_size: 100KB  ← Compressed!

Compression works without native ZFS!
```

### **Scenario 3: Deduplication**

```bash
# Upload same file multiple times
nestgate put dataset1/file1.txt < data.txt
nestgate put dataset1/file2.txt < data.txt
nestgate put dataset1/file3.txt < data.txt

# Check storage used
nestgate get --stats dataset1
# files: 3
# logical_size: 30MB
# actual_size: 10MB  ← Deduped!

Dedup works without native ZFS!
```

### **Scenario 4: Data Integrity**

```bash
# Upload with checksums
nestgate put --checksum sha256 dataset1/important.dat < important.dat

# Verify integrity
nestgate verify dataset1/important.dat
# Checksum valid

# Simulate corruption
# (manually modify file on backend)

# Verify again
nestgate verify dataset1/important.dat
# FAIL: Checksum mismatch - data corrupted!

Data integrity without native ZFS!
```

---

## **THE POWER OF UNIVERSAL ZFS**

### **What This Means:**

1. **No Installation Barriers**
   - No kernel modules
   - No root access needed
   - Works anywhere

2. **Cross-Platform**
   - Linux, macOS, Windows
   - Containers, VMs, bare metal
   - Cloud, edge, IoT

3. **Flexible Backends**
   - Use what you have
   - Mix and match
   - Tier automatically

4. **Same API Everywhere**
   - Write once, run anywhere
   - Consistent semantics
   - Easy to learn

5. **Progressive Enhancement**
   - Native ZFS when available (faster)
   - Emulated when not (still works!)
   - Seamless fallback

---

## **LOCAL TESTING PLAN**

### **Phase 1: Filesystem Backend** (NOW)

```bash
# Test on your ext4 system
cd /path/to/nestgate

# Configure filesystem backend
./target/release/nestgate storage configure \
  --backend filesystem \
  --path $HOME/.nestgate/data

# Test basic operations
./target/release/nestgate storage list
./target/release/nestgate doctor

# Test via API (once service running)
curl http://localhost:9001/api/v1/health
```

### **Phase 2: Memory Backend**

```bash
# Fast testing with RAM
./target/release/nestgate storage configure \
  --backend memory \
  --size 1GB

# Ultra-fast operations
# Perfect for testing
```

### **Phase 3: Hybrid**

```bash
# Combine multiple backends
./target/release/nestgate storage configure \
  --backend hybrid \
  --hot memory:1GB \
  --warm filesystem:$HOME/.nestgate \
  --cold s3:backup-bucket  # If you have S3
```

---

## **VERIFICATION**

After configuration, verify:

```bash
# Check storage backends
./target/release/nestgate storage list

# Run diagnostics
./target/release/nestgate doctor --comprehensive

# Test API
curl http://localhost:9001/api/v1/storage/backends

# Create test dataset
curl -X POST http://localhost:9001/api/v1/datasets \
  -H 'Content-Type: application/json' \
  -d '{"name": "test", "compression": "lz4"}'
```

---

## **SUMMARY**

### **The Key Point:**

**"No ZFS on system? That's EXACTLY why NestGate exists!"**

NestGate brings ZFS features to:
- Regular filesystems (ext4, xfs, btrfs, ntfs)
- Memory (RAM disk, tmpfs)
- Cloud storage (S3, Azure, GCS)
- Network storage (NFS, SMB)
- ANY storage backend!

**Same API. Same Features. Any Backend.**

**This is NestGate's superpower!**

---

**Universal ZFS = ZFS everywhere**

**No installation barriers**

**Works on THIS system RIGHT NOW!**

