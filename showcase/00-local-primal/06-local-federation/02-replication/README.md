# 📦 ZFS Replication - Data Federation

**Time**: 15 minutes  
**Complexity**: Advanced  
**Prerequisites**: Completed 01-two-node-mesh

---

## 🎯 WHAT YOU'LL LEARN

- ✅ ZFS send/receive replication
- ✅ Incremental replication (only changes)
- ✅ Deduplication during transfer
- ✅ Compression during transfer
- ✅ Real throughput numbers

**After this demo**: You'll understand NestGate's replication engine

---

## 🚀 QUICK START

```bash
./demo-zfs-replication.sh
```

**Expected duration**: ~15 minutes  
**Expected result**: Dataset replicated across 2 nodes with incremental updates

---

## 💡 WHAT IS ZFS REPLICATION?

### **Traditional File Copy**:
```
❌ Copy entire files every time
❌ No deduplication
❌ No compression
❌ Slow and wasteful
```

### **ZFS Send/Receive**:
```
✅ Block-level replication
✅ Only changed blocks sent (incremental)
✅ Automatic compression
✅ Automatic deduplication
✅ 10-100x faster than traditional
```

---

## 📋 WHAT HAPPENS

### **Phase 1: Initial Replication**
```
Node 1:
  Create dataset: pool/photos (500 MB)
  Take snapshot: pool/photos@initial
  
  zfs send pool/photos@initial | 
    ssh node2 zfs receive pool/photos
    
Transfer:
  Size: 500 MB
  Compressed: 125 MB (4:1 ratio)
  Throughput: 234 MB/s
  Time: 2.1 seconds
  
Node 2:
  Received: pool/photos@initial (500 MB)
  Status: Identical to Node 1 ✅
```

### **Phase 2: Incremental Update**
```
Node 1:
  Modify data: Add 50 MB of new photos
  Take snapshot: pool/photos@update1
  
  zfs send -i @initial @update1 pool/photos |
    ssh node2 zfs receive pool/photos
    
Transfer:
  Total dataset: 550 MB
  Changes only: 50 MB
  Compressed: 12 MB (4:1 ratio)
  Throughput: 245 MB/s
  Time: 0.2 seconds ⚡
  
Node 2:
  Applied incremental
  Status: Synchronized ✅
  
Efficiency: 91% bandwidth saved!
```

### **Phase 3: Second Incremental**
```
Node 1:
  Modify data: Add 100 MB more
  Take snapshot: pool/photos@update2
  
  zfs send -i @update1 @update2 pool/photos |
    ssh node2 zfs receive pool/photos
    
Transfer:
  Changes only: 100 MB
  Compressed: 25 MB (4:1 ratio)
  Time: 0.4 seconds ⚡
  
Node 2:
  Applied incremental
  Total now: 650 MB
  Status: Synchronized ✅
```

---

## 🔬 HOW IT WORKS

### **ZFS Send Stream Format**

**Block-Level Replication:**
```
Traditional (file-level):
  • Copy entire files
  • Re-send unchanged data
  • No deduplication
  
ZFS (block-level):
  • Copy only changed blocks
  • Reference unchanged blocks
  • Automatic deduplication
```

**Stream Structure:**
```
ZFS Send Stream:
  1. Header (metadata)
  2. BEGIN record
  3. Block records (data + checksums)
  4. Property records (dataset properties)
  5. END record
  
Each block:
  - Block pointer
  - Data (compressed)
  - Checksum (SHA-256)
  - Birth time (for dedup)
```

### **Incremental Algorithm**

**Step 1: Identify Changes**
```rust
// Find blocks that changed between snapshots
let changed_blocks = zfs_diff(
    snapshot_old,  // @initial
    snapshot_new   // @update1
);

// Only send changed blocks
for block in changed_blocks {
    stream.write(block);
}
```

**Step 2: Compression**
```
Block compressed with LZ4:
  • Fast (500+ MB/s compression)
  • Good ratio (2-4x typical)
  • Low CPU usage (<10%)
  
During transfer:
  • Already compressed by ZFS
  • No additional compression needed
  • Network sees compressed data
```

**Step 3: Deduplication**
```
ZFS dedup (on sender):
  • Calculate block hash
  • Check if block exists
  • If exists: Send reference only
  • If new: Send full block
  
Result:
  • Duplicate data sent once
  • Massive bandwidth savings
```

---

## 📊 PERFORMANCE EXPECTATIONS

### **Network Throughput**

| Network | Raw Bandwidth | ZFS Throughput | Notes |
|---------|---------------|----------------|-------|
| 1 Gbps | 125 MB/s | 110-120 MB/s | Network limited |
| 10 Gbps | 1250 MB/s | 900-1000 MB/s | CPU limited |
| 100 Gbps | 12500 MB/s | 4000-6000 MB/s | Disk limited |

**Overhead**: ~5% (checksumming, protocol)

### **Compression Ratios**

| Data Type | Typical Ratio | Notes |
|-----------|---------------|-------|
| **Text/Logs** | 5-10x | Highly compressible |
| **Source Code** | 4-6x | Very compressible |
| **Documents** | 3-5x | Good compression |
| **Photos (JPEG)** | 1.01-1.05x | Already compressed |
| **Videos** | 1.0-1.02x | Already compressed |
| **Databases** | 2-4x | Medium compression |

### **Incremental Efficiency**

```
Scenario: 1 TB dataset, 10 GB daily changes

Full replication:
  Transfer: 1 TB
  Time: ~2.5 hours (1 Gbps)
  Bandwidth: 1 TB
  
Incremental replication:
  Transfer: 10 GB
  Time: ~90 seconds (1 Gbps)
  Bandwidth: 10 GB
  Savings: 99%! ✅
```

---

## 🎓 KEY CONCEPTS

### **1. Copy-on-Write = Zero-Cost Snapshots**

**How snapshots work:**
```
Time 0: Create dataset
  ├─ Block A (data)
  ├─ Block B (data)
  └─ Block C (data)
  
Time 1: Take snapshot @snap1
  • No data copied
  • Just mark current state
  • Cost: ~1ms, ~1KB metadata
  
Time 2: Modify data
  ├─ Block A (old) ← snap1 points here
  ├─ Block A' (new) ← current points here
  ├─ Block B (unchanged) ← both point here
  └─ Block C (unchanged) ← both point here
  
Only changed blocks create new copies!
```

### **2. Incremental Sends = Minimal Transfer**

**First replication (full):**
```
Send all blocks: A, B, C, D, E
Transfer: 100%
```

**Second replication (incremental):**
```
Changed: Block C, Block E
Send only: C', E'
Transfer: 40% (60% saved!)
```

**Third replication (incremental):**
```
Changed: Block A
Send only: A''
Transfer: 20% (80% saved!)
```

### **3. Resumable Sends**

**If transfer fails:**
```
Traditional:
  ❌ Start over from beginning
  ❌ Waste all progress
  
ZFS:
  ✅ Resume from last successful block
  ✅ No wasted bandwidth
  ✅ Automatic retry
```

---

## 💬 WHAT YOU'LL SEE

```
📦 NestGate ZFS Replication Demo
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Phase 1: Initial Replication (Full)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Node 1: Creating dataset...
  ✅ pool/photos created (500 MB)
  ✅ Snapshot: pool/photos@initial

Node 1 → Node 2: Full replication...
  Dataset: pool/photos@initial
  Size: 500 MB
  [████████████████████████████] 500 MB
  
  Compressed: 125 MB (4:1 ratio)
  Throughput: 234 MB/s
  Time: 2.1 seconds ⚡
  
Node 2:
  ✅ Received: pool/photos@initial
  ✅ Size: 500 MB
  ✅ Checksum: VALID
  
Status: SYNCHRONIZED

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Phase 2: Incremental Replication #1
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Node 1: Modifying dataset...
  ✅ Added 50 MB of new data
  ✅ Snapshot: pool/photos@update1

Node 1 → Node 2: Incremental replication...
  Dataset: pool/photos
  From: @initial
  To: @update1
  Changes: 50 MB
  [████████████████████████████] 50 MB
  
  Compressed: 12 MB (4:1 ratio)
  Throughput: 245 MB/s
  Time: 0.2 seconds ⚡⚡
  
Node 2:
  ✅ Applied incremental
  ✅ New size: 550 MB
  ✅ Checksum: VALID
  
Efficiency: 91% bandwidth saved!
Status: SYNCHRONIZED

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ Demo Complete!

Summary:
  Full replication: 500 MB in 2.1s (234 MB/s)
  Incremental #1: 50 MB in 0.2s (245 MB/s)
  Total bandwidth saved: 91%
  
🎓 What you learned:
   ✅ ZFS send/receive replication
   ✅ Incremental efficiency (10-100x faster)
   ✅ Automatic compression (4x bandwidth savings)
   ✅ Block-level deduplication

⏭️  Next: Load balancing and failover
   cd ../03-load-balancing && ./demo-failover.sh
```

---

## 🔬 TRY IT YOURSELF

### Experiment 1: Different Compression
```bash
# Try different compression algorithms
zfs set compression=lz4 pool/photos    # Fast
zfs set compression=zstd pool/photos   # Best ratio
zfs set compression=gzip pool/photos   # Slow but good

# Test replication speed with each
```

### Experiment 2: Large Dataset
```bash
# Create 10 GB dataset
# Replicate
# Modify 100 MB
# Replicate incrementally
# Calculate efficiency
```

### Experiment 3: Network Simulation
```bash
# Limit bandwidth to 10 MB/s
tc qdisc add dev eth0 root tbf rate 10mbit

# Test replication
# Watch compression help!
```

---

**Ready to replicate?** Run `./demo-zfs-replication.sh`!

📦 **Replication: The backbone of federation!** 📦

