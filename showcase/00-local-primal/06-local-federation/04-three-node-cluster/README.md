# 🌟 Three-Node Cluster - Complete Federation

**Time**: 15 minutes  
**Complexity**: Advanced  
**Prerequisites**: Completed all previous federation demos

---

## 🎯 WHAT YOU'LL LEARN

- ✅ Full mesh topology (3 connections)
- ✅ Quorum-based consensus
- ✅ Distributed data consistency
- ✅ Production cluster operations
- ✅ Complete federation architecture

**After this demo**: You'll understand enterprise-grade clustering

---

## 🚀 QUICK START

```bash
./demo-cluster.sh
```

**Expected duration**: ~15 minutes  
**Expected result**: Fully operational 3-node cluster with data consistency

---

## 📋 WHAT HAPPENS

### **Phase 1: Cluster Formation**
```
Node 1 starts:
  ✅ Announced via mDNS
  Status: Standalone
  
Node 2 starts:
  ✅ Discovered Node 1
  ✅ Connected to Node 1
  Status: 2-node cluster
  
Node 3 starts:
  ✅ Discovered Node 1, Node 2
  ✅ Connected to Node 1
  ✅ Connected to Node 2
  Status: 3-node full mesh ✅
  
Topology:
     Node 1
     /    \
    /      \
Node 2 ---- Node 3

Connections: 3 (full mesh)
Formation time: 2.1 seconds
```

### **Phase 2: Distributed Write**
```
Client → Node 1: Write "hello.txt"

Node 1 (coordinator):
  1. Write locally ✅
  2. Replicate to Node 2 → ACK (8ms)
  3. Replicate to Node 3 → ACK (9ms)
  4. Confirm to client → Total: 18ms
  
Result:
  ✅ Node 1: Has "hello.txt"
  ✅ Node 2: Has "hello.txt"
  ✅ Node 3: Has "hello.txt"
  
Consistency: STRONG (all nodes synchronized)
```

### **Phase 3: Distributed Read**
```
Read "hello.txt" from each node:

Client → Node 1: GET "hello.txt"
  ✅ Found (2.1ms)
  Content: "Hello, NestGate Federation!"
  
Client → Node 2: GET "hello.txt"
  ✅ Found (2.3ms)
  Content: "Hello, NestGate Federation!"
  
Client → Node 3: GET "hello.txt"
  ✅ Found (2.2ms)
  Content: "Hello, NestGate Federation!"
  
Consistency verified: ✅ All nodes identical
```

### **Phase 4: Quorum Demonstration**
```
Scenario: Network partition

Partition 1 (Node 1 alone):
  Nodes: 1/3
  Quorum: NO (need 2/3)
  Mode: READ-ONLY ⚠️
  Writes: REJECTED
  
Partition 2 (Node 2 + Node 3):
  Nodes: 2/3
  Quorum: YES ✅
  Mode: READ-WRITE
  Writes: ACCEPTED
  
Write attempt to Node 1:
  ❌ REJECTED: "No quorum (1/3 < 2/3)"
  
Write to Node 2:
  ✅ ACCEPTED: "Quorum maintained (2/3 >= 2/3)"
  
Data consistency maintained! ✅
```

### **Phase 5: Cluster Operations**
```
Operations demonstrated:

1. Cluster status
   ✅ 3 nodes healthy
   ✅ Full mesh connected
   ✅ Quorum established
   
2. Node addition (scaling up)
   ✅ Node 4 joins automatically
   ✅ Mesh expands (6 connections now)
   ✅ Data synchronized
   
3. Node removal (scaling down)
   ✅ Node 4 leaves cleanly
   ✅ Mesh adjusts (3 connections)
   ✅ Data remains consistent
   
4. Rolling restart
   ✅ Restart nodes one by one
   ✅ Cluster stays operational
   ✅ Zero downtime
```

---

## 💡 FULL MESH TOPOLOGY

### **Why Full Mesh?**

**Hub-and-Spoke (Traditional)**:
```
     Hub
    / | \
   1  2  3
   
Pros:
  ✅ Simple
  
Cons:
  ❌ Single point of failure (hub)
  ❌ Bottleneck at hub
  ❌ Hub failure = total failure
```

**Full Mesh (NestGate)**:
```
   1---2
    \ /
     3
     
Pros:
  ✅ No single point of failure
  ✅ Direct peer communication
  ✅ Maximum resilience
  ✅ Distributed load
  
Cons:
  ⚠️  More connections (N*(N-1)/2)
  ⚠️  More complex (worth it!)
```

### **Connection Scaling**

| Nodes | Connections | Notes |
|-------|-------------|-------|
| 2 | 1 | Simple HA |
| 3 | 3 | Production HA ✅ |
| 4 | 6 | High availability |
| 5 | 10 | Maximum recommended |
| 10 | 45 | Scaling limit |

**Sweet spot**: 3-5 nodes for most use cases

---

## 🔬 QUORUM MECHANICS

### **What is Quorum?**

**Definition**: Minimum number of nodes required to make decisions

**Formula**: `⌈N/2⌉ + 1` where N = total nodes

**Examples**:
```
2 nodes: Quorum = 2 (all nodes!)
3 nodes: Quorum = 2 (majority)
4 nodes: Quorum = 3 (majority)
5 nodes: Quorum = 3 (majority)
```

### **Why Quorum?**

**Problem: Split-Brain**
```
Without quorum:
  3 nodes, network partition:
    Side A: Node 1 (thinks it's primary)
    Side B: Node 2+3 (thinks they're primary)
    
  Both sides accept writes → DATA DIVERGENCE! ❌
```

**Solution: Quorum**
```
With quorum (need 2/3):
  3 nodes, network partition:
    Side A: Node 1 (1/3 < 2/3 = NO QUORUM)
      → Read-only mode
      → Rejects writes
    
    Side B: Node 2+3 (2/3 >= 2/3 = HAS QUORUM)
      → Read-write mode
      → Accepts writes
      
  Only one side can write → CONSISTENCY! ✅
```

### **Quorum States**

```rust
enum ClusterState {
    Healthy {
        nodes: 3,
        quorum: true,
        mode: ReadWrite,
    },
    Degraded {
        nodes: 2,
        quorum: true,     // 2/3 still OK
        mode: ReadWrite,
    },
    Critical {
        nodes: 1,
        quorum: false,    // 1/3 not enough
        mode: ReadOnly,
    },
}
```

---

## 📊 CONSISTENCY GUARANTEES

### **Strong Consistency**

**What**: All nodes see the same data at the same time

**How NestGate achieves it**:
```
Write operation:
  1. Client sends write to Node 1
  2. Node 1 writes locally
  3. Node 1 replicates to quorum (2/3 nodes)
  4. Wait for quorum ACKs
  5. Only then confirm to client
  
If any node in quorum fails:
  → Retry or abort
  → Never confirm partial write
  
Result: Either all nodes have it, or none do ✅
```

**Trade-off**: Slightly higher latency (18ms vs 8ms for local)  
**Benefit**: Guaranteed consistency

### **Read Consistency**

```
Scenario 1: Read from any node
  Result: All nodes return same data ✅
  
Scenario 2: Read immediately after write
  Result: All nodes see new data ✅
  
Scenario 3: Concurrent reads
  Result: All return same value ✅
  
Consistency level: STRONG ✅
```

---

## 🎓 KEY CONCEPTS

### **1. CAP Theorem**

**CAP**: Can only pick 2 of 3
- **C**onsistency: All nodes see same data
- **A**vailability: System always responds
- **P**artition tolerance: Works despite network splits

**NestGate choice**: **CP** (Consistency + Partition tolerance)

**Why**:
```
Network partition happens:
  ✅ Consistency: Maintained via quorum
  ✅ Partition tolerance: System handles splits
  ⚠️  Availability: Minority partition read-only
  
Rationale: Data correctness > availability
Better to be read-only than wrong!
```

### **2. RAFT Consensus** (Simplified)

**Problem**: How do distributed nodes agree?

**RAFT approach**:
```
1. Leader election (if needed)
2. Leader receives write
3. Leader replicates to followers
4. Leader waits for quorum ACKs
5. Leader commits and confirms
```

**NestGate**: Simplified RAFT for storage clustering

### **3. Two-Phase Commit**

**Phase 1: Prepare**
```
Coordinator → All nodes: "Can you commit?"
All nodes → Coordinator: "Yes" or "No"

If all "Yes": Proceed to Phase 2
If any "No": Abort
```

**Phase 2: Commit**
```
Coordinator → All nodes: "Commit now!"
All nodes: Execute commit
Coordinator: Confirm to client
```

**NestGate uses**: Simplified 2PC for writes

---

## 📊 PERFORMANCE METRICS

### **Operation Latencies**

| Operation | Local | Cluster (3 nodes) | Overhead |
|-----------|-------|-------------------|----------|
| **Write** | 2ms | 18ms | 9x |
| **Read (local)** | 2ms | 2ms | 1x |
| **Read (remote)** | 2ms | 4ms | 2x |
| **Snapshot** | 1ms | 3ms | 3x |

**Key insight**: Writes slower (consistency), reads fast

### **Throughput**

```
Single node: 450 req/s

3-node cluster:
  Writes: 150 req/s per node = 450 req/s total
  Reads: 450 req/s per node = 1350 req/s total!
  
Read scaling: 3x ✅
Write scaling: 1x (consistency overhead)
```

---

## 💬 WHAT YOU'LL SEE

```
🌟 NestGate Three-Node Cluster Demo
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Phase 1: Cluster Formation
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Starting Node 1...
  ✅ Node 1: ONLINE (node-abc123...)
  Status: Standalone

Starting Node 2...
  ✅ Node 2: ONLINE (node-def456...)
  ✅ Node 2 → Node 1: Connected
  Status: 2-node cluster

Starting Node 3...
  ✅ Node 3: ONLINE (node-ghi789...)
  ✅ Node 3 → Node 1: Connected
  ✅ Node 3 → Node 2: Connected
  
Cluster topology:
     Node 1
     /    \
    /      \
Node 2 ---- Node 3

Status: FULL MESH ✅
Connections: 3
Quorum: 2/3 established
Formation time: 2.1s

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Phase 2: Distributed Write
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Client → Node 1: Write "hello.txt"

Node 1 (coordinator):
  [  0ms] Write locally
  [  3ms] ✅ Local write complete
  [  3ms] Replicate to Node 2
  [ 11ms] ✅ Node 2: ACK
  [ 11ms] Replicate to Node 3
  [ 20ms] ✅ Node 3: ACK
  [ 20ms] Quorum achieved (3/3)
  [ 20ms] Confirm to client
  
Total time: 20ms

Verification:
  ✅ Node 1: Has "hello.txt" (5 bytes)
  ✅ Node 2: Has "hello.txt" (5 bytes)
  ✅ Node 3: Has "hello.txt" (5 bytes)
  
Consistency: STRONG ✅

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Phase 3: Distributed Read
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Reading from all nodes...

Client → Node 1: GET "hello.txt"
  ✅ Found (2.1ms)
  Content: "Hello, NestGate Federation!"
  Checksum: a3f5...
  
Client → Node 2: GET "hello.txt"
  ✅ Found (2.3ms)
  Content: "Hello, NestGate Federation!"
  Checksum: a3f5...
  
Client → Node 3: GET "hello.txt"
  ✅ Found (2.2ms)
  Content: "Hello, NestGate Federation!"
  Checksum: a3f5...
  
Verification: ✅ All checksums match
Data consistency: VERIFIED ✅

... [continues with Phases 4-5]

✅ Demo Complete!
```

---

## 🔬 TRY IT YOURSELF

### Experiment 1: Scale to 4 Nodes
```bash
# Add 4th node dynamically
# Watch mesh expand to 6 connections
# Verify data replication
# Test quorum (now 3/4 required)
```

### Experiment 2: Chaos Testing
```bash
# Kill random nodes continuously
# Generate constant write load
# Verify zero data loss
# Measure availability percentage
```

### Experiment 3: Performance Profiling
```bash
# Measure write latency vs cluster size
# 2 nodes vs 3 nodes vs 4 nodes
# Plot latency distribution
# Find optimal cluster size
```

---

**Ready for production clustering?** Run `./demo-cluster.sh`!

🌟 **Three-node cluster: Enterprise-grade federation!** 🌟

